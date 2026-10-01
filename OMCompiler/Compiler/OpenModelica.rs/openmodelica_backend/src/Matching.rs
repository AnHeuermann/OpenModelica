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

use crate::BackendDAEFunc;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::IndexReduction;
use crate::Sorting;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::Ceval;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// just a matching algorithm
// - PerfectMatching
// - RegularMatching
//
// =============================================================================
pub fn PerfectMatching(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut N: i32 = metamodelica::arrayLength(m.clone());
    ass1 = arrayCreate(N, -1);
    ass2 = arrayCreate(N, -1);
    let (__pa0, __pa1, true, _, _) = (ContinueMatching(m.clone(), N, N, ass1.clone(), ass2.clone(), true)?) else {
        return Err("pattern mismatch");
    };
    ass1 = metamodelica::Own::own(__pa0);
    ass2 = metamodelica::Own::own(__pa1);
    Ok((ass1, ass2))
}

pub(crate) fn RegularMatching(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut nVars: i32,
    mut nEqns: i32,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    bool,
    metamodelica::Array<bool>,
    metamodelica::Array<bool>,
)> {
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut perfectMatching: bool;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    ass1 = arrayCreate(nVars, -1);
    ass2 = arrayCreate(nEqns, -1);
    (ass1, ass2, perfectMatching, eMark, vMark) =
        ContinueMatching(m.clone(), nVars, nEqns, ass1.clone(), ass2.clone(), false)?;
    Ok((ass1, ass2, perfectMatching, eMark, vMark))
}

pub(crate) fn ContinueMatching(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut nVars: i32,
    mut nEqns: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut stopAtSingularity: bool,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    bool,
    metamodelica::Array<bool>,
    metamodelica::Array<bool>,
)> {
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut perfectMatching: bool = true;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    let mut i: i32;
    let mut j: i32;
    let mut eMarkIx: metamodelica::Array<i32>;
    let mut vMarkIx: metamodelica::Array<i32>;
    let mut eMarkN: i32 = 0;
    let mut vMarkN: i32 = 0;
    let mut eDead: i32 = 0;
    let mut vDead: i32 = 0;
    let mut success: bool;
    vMark = arrayCreate(nVars, false);
    eMark = arrayCreate(nEqns, false);
    vMarkIx = arrayCreate(nVars, 0);
    eMarkIx = arrayCreate(nEqns, 0);
    i = 1;
    while i <= nEqns {
        j = ({
            let __elt = (*metamodelica::index_checked(&ass2.borrow(), i)?).clone();
            __elt
        });
        if !(j > 0
            && ({
                let __elt = (*metamodelica::index_checked(&ass1.borrow(), j)?).clone();
                __elt
            }) == i)
        {
            (success, eMarkN, vMarkN) = BBPathFound(
                i,
                m.clone(),
                eMark.clone(),
                vMark.clone(),
                ass1.clone(),
                ass2.clone(),
                eMarkIx.clone(),
                vMarkIx.clone(),
                eDead,
                vDead,
            )?;
            if success {
                clearMarks(eMark.clone(), eMarkIx.clone(), eDead, eMarkN)?;
                clearMarks(vMark.clone(), vMarkIx.clone(), vDead, vMarkN)?;
            } else {
                perfectMatching = false;
                eDead = eMarkN;
                vDead = vMarkN;
                if stopAtSingularity {
                    return Ok((ass1, ass2, perfectMatching, eMark, vMark));
                }
            }
        }
        i = i + 1;
    }
    Ok((ass1, ass2, perfectMatching, eMark, vMark))
}

fn clearMarks(
    mut arr: metamodelica::Array<bool>,
    mut arrIx: metamodelica::Array<i32>,
    mut from: i32,
    mut to: i32,
) -> Result<()> {
    let __ab_arrIx = arrIx.borrow();
    for mut i in from + 1..=to {
        metamodelica::arrayUpdate(
            arr.clone(),
            (*metamodelica::index_checked(&__ab_arrIx, i)?).clone(),
            false,
        )?;
    }
    Ok(())
}

pub(crate) fn BBMatching(
    mut inSys: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outSys: metamodelica::Ref<BackendDAE::EqSystem> = inSys;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ) = inArg;
    let mut i: i32;
    let mut success: bool = true;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut j: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut eMark: metamodelica::Array<bool>;
    let mut vMark: metamodelica::Array<bool>;
    let mut eMarkIx: metamodelica::Array<i32>;
    let mut vMarkIx: metamodelica::Array<i32>;
    let mut eMarkN: i32 = 0;
    let mut vMarkN: i32 = 0;
    let mut mEqns: metamodelica::List<i32>;
    let __pa0 = ::match_deref::match_deref! { match &(outSys.m.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    nEqns = BackendDAEUtil::systemSize(&outSys)?;
    nVars = BackendVariable::daenumVariables(&outSys);
    ass2 = arrayCreate(nEqns, -1);
    ass1 = arrayCreate(nVars, -1);
    vMark = arrayCreate(nVars, false);
    eMark = arrayCreate(nEqns, false);
    vMarkIx = arrayCreate(nVars, 0);
    eMarkIx = arrayCreate(nEqns, 0);
    i = 1;
    while i <= nEqns && success {
        j = ({
            let __elt = (*metamodelica::index_checked(&ass2.borrow(), i)?).clone();
            __elt
        });
        if j > 0
            && ({
                let __elt = (*metamodelica::index_checked(&ass1.borrow(), j)?).clone();
                __elt
            }) == i
        {
            success = true;
        } else {
            clearArrayWithKnownSetIndexes(eMark.clone(), eMarkIx.clone(), eMarkN)?;
            clearArrayWithKnownSetIndexes(vMark.clone(), vMarkIx.clone(), vMarkN)?;
            (success, eMarkN, vMarkN) = BBPathFound(
                i,
                m.clone(),
                eMark.clone(),
                vMark.clone(),
                ass1.clone(),
                ass2.clone(),
                eMarkIx.clone(),
                vMarkIx.clone(),
                0,
                0,
            )?;
            if !(success) {
                mEqns = metamodelica::nil();
                for mut j in 1..=nEqns {
                    if ({
                        let __elt = (*metamodelica::index_checked(&eMark.borrow(), j)?).clone();
                        __elt
                    }) {
                        mEqns = metamodelica::cons(j, mEqns);
                    }
                }
                (_, i, outSys, outShared, ass1, ass2, outArg) =
                    sssHandler(list![mEqns], i, outSys, outShared, ass1.clone(), ass2.clone(), outArg)?;
                let __pa1 = ::match_deref::match_deref! { match &(outSys.m.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                m = metamodelica::Own::own(__pa1);
                success = true;
                i = i - 1;
            }
        }
        i = i + 1;
    }
    if success {
        outSys = BackendDAEUtil::setEqSystMatching(
            outSys,
            metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                ass1: ass1.clone(),
                ass2: ass2.clone(),
                comps: metamodelica::nil(),
            }),
        );
    } else {
        metamodelica::print(literal!("\nSingular System!!!\n"));
    }
    Ok((outSys, outShared, outArg))
}

fn BBPathFound(
    mut i: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut eMark: metamodelica::Array<bool>,
    mut vMark: metamodelica::Array<bool>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut eMarkIx: metamodelica::Array<i32>,
    mut vMarkIx: metamodelica::Array<i32>,
    mut eMarkN: i32,
    mut vMarkN: i32,
) -> Result<(bool, i32, i32)> {
    let __ab_m = m.borrow();
    let mut success: bool = false;
    let mut eMarkN: i32 = eMarkN;
    let mut vMarkN: i32 = vMarkN;
    let mut stack: metamodelica::List<(i32, i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut vars: metamodelica::List<i32> = metamodelica::nil();
    let mut eqn: i32 = i;
    let mut var: i32;
    let mut parent: i32;
    let mut enter: bool = true;
    let mut descended: bool;
    loop {
        if enter {
            enter = false;
            vars = metamodelica::nil();
            if !(metamodelica::arrayGet(eMark.clone(), eqn)?) {
                metamodelica::arrayUpdate(eMark.clone(), eqn, true)?;
                eMarkN = eMarkN + 1;
                metamodelica::arrayUpdate(eMarkIx.clone(), eMarkN, eqn)?;
                for mut j in &*(*metamodelica::index_checked(&__ab_m, eqn)?).clone() {
                    if j.clone() > 0
                        && ({
                            let __elt = (*metamodelica::index_checked(&ass1.borrow(), j.clone())?).clone();
                            __elt
                        }) <= 0
                    {
                        success = true;
                        metamodelica::arrayUpdate(ass1.clone(), j.clone(), eqn)?;
                        metamodelica::arrayUpdate(ass2.clone(), eqn, j.clone())?;
                        break;
                    }
                }
                if success {
                    for mut frame in &*stack {
                        (parent, var, _) = frame.clone();
                        metamodelica::arrayUpdate(ass1.clone(), var, parent)?;
                        metamodelica::arrayUpdate(ass2.clone(), parent, var)?;
                    }
                    return Ok((success, eMarkN, vMarkN));
                }
                vars = (*metamodelica::index_checked(&__ab_m, eqn)?).clone();
            }
        }
        descended = false;
        while !((vars).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vars) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            var = metamodelica::Own::own(__pa0);
            vars = metamodelica::Own::own(__pa1);
            if var > 0
                && !({
                    let __elt = (*metamodelica::index_checked(&vMark.borrow(), var)?).clone();
                    __elt
                })
            {
                metamodelica::arrayUpdate(vMark.clone(), var, true)?;
                vMarkN = vMarkN + 1;
                metamodelica::arrayUpdate(vMarkIx.clone(), vMarkN, var)?;
                stack = metamodelica::cons((eqn, var, vars.clone()), stack);
                eqn = ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), var)?).clone();
                    __elt
                });
                enter = true;
                descended = true;
                break;
            }
        }
        if !(descended) {
            if (stack).is_empty() {
                return Ok((success, eMarkN, vMarkN));
            }
            let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(stack) {
                Deref @ metamodelica::ListNode::Cons { head: (__pa2, _, __pa3), tail: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqn = metamodelica::Own::own(__pa2);
            vars = metamodelica::Own::own(__pa3);
            stack = metamodelica::Own::own(__pa4);
        }
    }
    Ok((success, eMarkN, vMarkN))
}

fn BBCheapMatching(
    mut nEqns: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let __ab_m = m.borrow();
    let mut i: i32 = 0;
    let mut j: i32;
    let mut success: bool = false;
    let mut vars: metamodelica::List<i32>;
    for mut i in 1..=nEqns {
        vars = (*metamodelica::index_checked(&__ab_m, i)?).clone();
        while !(success) && !((vars).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vars) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            j = metamodelica::Own::own(__pa0);
            vars = metamodelica::Own::own(__pa1);
            if j > 0
                && ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), j)?).clone();
                    __elt
                }) <= 0
            {
                success = true;
                metamodelica::arrayUpdate(ass1.clone(), j, i)?;
                metamodelica::arrayUpdate(ass2.clone(), i, j)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn invertMatching(mut inAss: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32>;
    let mut N: i32 = metamodelica::arrayLength(inAss.clone());
    let mut j: i32;
    outAss = arrayCreate(N, -1);
    for mut i in 1..=N {
        j = ({
            let __elt = (*metamodelica::index_checked(&inAss.borrow(), i)?).clone();
            __elt
        });
        if j > 0 {
            {
                let __cell0 = i;
                let __idx0 = ({
                    let __elt = (*metamodelica::index_checked(&inAss.borrow(), i)?).clone();
                    __elt
                });
                *metamodelica::index_mut_checked(&mut outAss.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
    }
    Ok(outAss)
}

// =============================================================================
// Matching Algorithms
//
// =============================================================================
pub(crate) fn DFSLH(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut emark: metamodelica::Array<i32>;
                    let mut vmark: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vmark = arrayCreate(nvars, -1);
                    emark = arrayCreate(neqns, -1);
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = DFSLH2(isyst.clone(), &ishared, nvars, neqns, 1, emark.clone(), vmark.clone(), vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, &inArg)?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec1.clone(), ass2: vec2.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.DFSLH failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn DFSLH2(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut nf: i32,
    mut i: i32,
    mut emark: metamodelica::Array<i32>,
    mut vmark: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut match_opts: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (outAssignments1, outAssignments2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = (isyst.clone(), match_opts);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let true = (intGe(i, nv)) else { return Err("pattern mismatch") };
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), i, i, emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone(), syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(_), mT: Some(_), .. }, _) => {
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    let mut i_1: i32;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut syst = (*syst).clone();
                    i_1 = i + 1;
                    let true = (intGt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), i)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    (ass1_2, ass2_2, syst, shared, arg) = DFSLH2(syst.clone(), ishared, nv, nf, i_1, emark.clone(), vmark.clone(), ass1.clone(), ass2.clone(), match_opts, sssHandler, inArg)?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    let mut i_1: i32;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut syst = (*syst).clone();
                    i_1 = i + 1;
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), i, i, emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    (ass1_2, ass2_2, syst, shared, arg) = DFSLH2(syst.clone(), ishared, nv, nf, i_1, emark.clone(), vmark.clone(), ass1_1.clone(), ass2_1.clone(), match_opts, sssHandler, inArg)?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    let mut ass1_3: metamodelica::Array<i32>;
                    let mut ass2_3: metamodelica::Array<i32>;
                    let mut emark1: metamodelica::Array<i32>;
                    let mut vmark1: metamodelica::Array<i32>;
                    let mut i_1: i32;
                    let mut nv_1: i32;
                    let mut nf_1: i32;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut meqns: metamodelica::List<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    meqns = getMarked(nf, i, emark.clone(), metamodelica::nil())?;
                    (_, i_1, syst, shared, ass1_1, ass2_1, arg) = sssHandler(list![meqns.clone()], i, isyst.clone(), ishared.clone(), ass1.clone(), ass2.clone(), inArg.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(&syst);
                    nf_1 = BackendEquation::equationArraySize(eqns.clone())?;
                    nv_1 = BackendVariable::varsSize(&(BackendVariable::daeVars(&syst)));
                    ass1_2 = assignmentsArrayExpand(ass1_1.clone(), nv_1, metamodelica::arrayLength(ass1_1.clone()), -1)?;
                    ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nf_1, metamodelica::arrayLength(ass2_1.clone()), -1)?;
                    vmark1 = assignmentsArrayExpand(vmark.clone(), nv_1, metamodelica::arrayLength(vmark.clone()), -1)?;
                    emark1 = assignmentsArrayExpand(emark.clone(), nf_1, metamodelica::arrayLength(emark.clone()), -1)?;
                    (ass1_3, ass2_3, syst, shared, arg1) = DFSLH2(syst.clone(), &shared, nv_1, nf_1, i_1, emark1.clone(), vmark1.clone(), ass1_2.clone(), ass2_2.clone(), match_opts, sssHandler, &arg)?;
                    Ok((ass1_3.clone(), ass2_3.clone(), syst.clone(), shared.clone(), arg1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut eqn_lst: metamodelica::List<i32>;
                    eqn_lst = getMarked(nf, i, emark.clone(), metamodelica::nil())?;
                    singularSystemError(list![eqn_lst.clone()], i, &isyst, ishared, ass1.clone(), ass2.clone(), inArg)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAssignments1, outAssignments2, osyst, oshared, outArg))
}

fn pathFound(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut i: i32,
    mut imark: i32,
    mut emark: metamodelica::Array<i32>,
    mut vmark: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            metamodelica::arrayUpdate(emark.clone(), i, imark)?;
            (ass1_1, ass2_1) = assignOneInEqn(m.clone(), mt.clone(), i, ass1.clone(), ass2.clone())?;
            Ok((ass1_1.clone(), ass2_1.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            (ass1_1, ass2_1) = forallUnmarkedVarsInEqn(
                m.clone(),
                mt.clone(),
                i,
                imark,
                emark.clone(),
                vmark.clone(),
                ass1.clone(),
                ass2.clone(),
            )?;
            Ok((ass1_1.clone(), ass2_1.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAssignments1, outAssignments2))
}

fn assignOneInEqn(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut i: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut vars: metamodelica::List<i32>;
    vars = BackendDAEUtil::varsInEqn(m.clone(), i)?;
    (outAssignments1, outAssignments2) = assignFirstUnassigned(i, &vars, ass1.clone(), ass2.clone())?;
    Ok((outAssignments1, outAssignments2))
}

fn assignFirstUnassigned(
    mut i: i32,
    mut inIntegerLst2: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = &**inIntegerLst2;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: _ } => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), v.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    ass1_1 = metamodelica::arrayUpdate(ass1.clone(), v.clone(), i)?;
                    ass2_1 = metamodelica::arrayUpdate(ass2.clone(), i, v.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: vs } => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    (ass1_1, ass2_1) = assignFirstUnassigned(i, metamodelica::AsArg::as_arg(&vs), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAssignments1, outAssignments2))
}

fn forallUnmarkedVarsInEqn(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut i: i32,
    mut imark: i32,
    mut emark: metamodelica::Array<i32>,
    mut vmark: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut vars_1: metamodelica::List<i32>;
    vars = BackendDAEUtil::varsInEqn(m.clone(), i)?;
    vars_1 = List::filter1OnTrue(
        vars,
        (std::sync::Arc::new(isNotVMarked)
            as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, metamodelica::Array<i32>)) -> Result<bool> + 'static>),
        (imark, vmark.clone()),
    )?;
    (outAssignments1, outAssignments2) = forallUnmarkedVarsInEqnBody(
        m.clone(),
        mt.clone(),
        i,
        imark,
        emark.clone(),
        vmark.clone(),
        &vars_1,
        ass1.clone(),
        ass2.clone(),
    )?;
    Ok((outAssignments1, outAssignments2))
}

fn isNotVMarked(mut i: i32, mut inTpl: (i32, metamodelica::Array<i32>)) -> Result<bool> {
    let mut outB: bool;
    let mut imark: i32;
    let mut vmark: metamodelica::Array<i32>;
    (imark, vmark) = inTpl;
    outB = !(intEq(
        imark,
        ({
            let __elt = (*metamodelica::index_checked(&vmark.borrow(), i)?).clone();
            __elt
        }),
    ));
    Ok(outB)
}

fn forallUnmarkedVarsInEqnBody(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut i: i32,
    mut imark: i32,
    mut emark: metamodelica::Array<i32>,
    mut vmark: metamodelica::Array<i32>,
    mut inIntegerLst4: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    (outAssignments1, outAssignments2) = 'mc: {
        let __mc_input = &**inIntegerLst4;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: _ } => {
                    let mut assarg: i32;
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    metamodelica::arrayUpdate(vmark.clone(), v.clone(), imark)?;
                    assarg = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), v.clone())?).clone(); __elt});
                    (ass1_1, ass2_1) = pathFound(m.clone(), mt.clone(), assarg, imark, emark.clone(), vmark.clone(), ass1.clone(), ass2.clone())?;
                    ass1_2 = metamodelica::arrayUpdate(ass1_1.clone(), v.clone(), i)?;
                    ass2_2 = metamodelica::arrayUpdate(ass2_1.clone(), i, v.clone())?;
                    Ok((ass1_2.clone(), ass2_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: vs } => {
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    (ass1_1, ass2_1) = forallUnmarkedVarsInEqnBody(m.clone(), mt.clone(), i, imark, emark.clone(), vmark.clone(), metamodelica::AsArg::as_arg(&vs), ass1.clone(), ass2.clone())?;
                    Ok((ass1_1.clone(), ass2_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAssignments1, outAssignments2))
}

pub(crate) fn BFSB(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut parentcolum: metamodelica::Array<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    rowmarks = arrayCreate(nvars, -1);
                    parentcolum = arrayCreate(nvars, -1);
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = BFSB1(1, 1, nvars, neqns, m.clone(), mt.clone(), rowmarks.clone(), parentcolum.clone(), vec1.clone(), vec2.clone(), &isyst, &ishared, inMatchingOptions, sssHandler, &inArg)?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.BFSB failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn BFSB1(
    mut i: i32,
    mut rowmark: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut parentcolum: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(i, ne)) else {
                return Err("pattern mismatch");
            };
            Ok((
                ass1.clone(),
                ass2.clone(),
                isyst.clone(),
                ishared.clone(),
                inArg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut visitedcolums: metamodelica::List<i32>;
            let mut m1: metamodelica::Array<metamodelica::List<i32>>;
            let mut mt1: metamodelica::Array<metamodelica::List<i32>>;
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut i_1: i32;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass1_2: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut ass2_2: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut parentcolum1: metamodelica::Array<i32>;
            let false = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            visitedcolums = BFSBphase(
                list![i],
                rowmark,
                i,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                parentcolum.clone(),
                ass1.clone(),
                ass2.clone(),
                metamodelica::nil(),
                &(metamodelica::nil()),
            )?;
            let (__pa0, __pa3, __pa1, __pa2, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i, isyst.clone(), ishared.clone(), nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg.clone())?) {
                (_, __pa0, __pa3 @ Deref @ BackendDAE::EqSystem { m: Some(__pa1), mT: Some(__pa2), .. }, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i_1 = metamodelica::Own::own(__pa0);
            m1 = metamodelica::Own::own(__pa1);
            mt1 = metamodelica::Own::own(__pa2);
            syst = metamodelica::Own::own(__pa3);
            shared = metamodelica::Own::own(__pa4);
            nv_1 = metamodelica::Own::own(__pa5);
            ne_1 = metamodelica::Own::own(__pa6);
            ass1_1 = metamodelica::Own::own(__pa7);
            ass2_1 = metamodelica::Own::own(__pa8);
            arg = metamodelica::Own::own(__pa9);
            rowmarks1 =
                assignmentsArrayExpand(rowmarks.clone(), nv_1, metamodelica::arrayLength(rowmarks.clone()), -1)?;
            parentcolum1 = assignmentsArrayExpand(
                parentcolum.clone(),
                nv_1,
                metamodelica::arrayLength(parentcolum.clone()),
                -1,
            )?;
            (ass1_2, ass2_2, syst, shared, arg) = BFSB1(
                i_1,
                rowmark + 1,
                nv_1,
                ne_1,
                m1.clone(),
                mt1.clone(),
                rowmarks1.clone(),
                parentcolum1.clone(),
                ass1_1.clone(),
                ass2_1.clone(),
                &syst,
                &shared,
                inMatchingOptions,
                sssHandler,
                &arg,
            )?;
            Ok((
                ass1_2.clone(),
                ass2_2.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let true = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            (ass1_1, ass2_1, syst, shared, arg) = BFSB1(
                i + 1,
                rowmark,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                parentcolum.clone(),
                ass1.clone(),
                ass2.clone(),
                isyst,
                ishared,
                inMatchingOptions,
                sssHandler,
                inArg,
            )?;
            Ok((
                ass1_1.clone(),
                ass2_1.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("function BFSB1 failed in equation "));
                    __mm_s.push_str(&*intString(i));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn BFSBphase<'__b>(
    mut queue: metamodelica::List<i32>,
    mut rowmark: i32,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut parentcolum: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextQueue: metamodelica::List<i32>,
    mut inVisitedColums: &'__b metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue, nextQueue.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inVisitedColums.clone())
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                { (queue, rowmark, i, nv, ne, m, mT, rowmarks, parentcolum, ass1, ass2, nextQueue, inVisitedColums) = (nextQueue, rowmark, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), inVisitedColums); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _) => {
                let mut queue1: metamodelica::List<i32>;
                let mut rows: metamodelica::List<i32>;
                let mut b: bool;
                rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                (queue1, b) = BFSBtraverseRows(&rows, &nextQueue, rowmark, i, c.clone(), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone())?;
                return Ok(BFSBphase1(b, rest.clone(), rowmark, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone(), queue1, &(metamodelica::cons(c.clone(), inVisitedColums.clone())))?)
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BFSBphase failed in equation ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn BFSBphase1(
    mut inPathFound: bool,
    mut queue: metamodelica::List<i32>,
    mut rowmark: i32,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut parentcolum: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextQueue: metamodelica::List<i32>,
    mut inVisitedColums: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (match inPathFound {
        true => metamodelica::nil(),
        false => BFSBphase(
            queue,
            rowmark,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            parentcolum.clone(),
            ass1.clone(),
            ass2.clone(),
            nextQueue,
            inVisitedColums,
        )?,
        _ => {
            Error::addInternalError(
                literal!("function BFSBphase1 failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outVisitedColums)
}

fn BFSBtraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut queue: &metamodelica::List<i32>,
    mut rowmark: i32,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut parentcolum: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, bool)> {
    let mut outEqnqueue: metamodelica::List<i32>;
    let mut pathFound: bool;
    (outEqnqueue, pathFound) = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((queue.clone().reverse(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    BFSBreasign(i, c, parentcolum.clone(), r.clone(), ass1.clone(), ass2.clone())?;
                    Ok((metamodelica::nil(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut queue2: metamodelica::List<i32>;
                    let mut rc: i32;
                    let mut b: bool;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    queue1 = BFSBenque(queue.clone(), rowmark, c, rc, r.clone(), intLt(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), rowmark), rowmarks.clone(), parentcolum.clone())?;
                    (queue2, b) = BFSBtraverseRows(metamodelica::AsArg::as_arg(&rest), &queue1, rowmark, i, c, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), parentcolum.clone(), ass1.clone(), ass2.clone())?;
                    Ok((queue2.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function BFSBtraverseRows failed in equation ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEqnqueue, pathFound))
}

fn BFSBreasign(
    mut i: i32,
    mut c: i32,
    mut parentcolum: metamodelica::Array<i32>,
    mut l: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intEq(i, c)) else {
                return Err("pattern mismatch");
            };
            metamodelica::arrayUpdate(ass1.clone(), c, l)?;
            metamodelica::arrayUpdate(ass2.clone(), l, c)?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r: i32;
            r = ({
                let __elt = (*metamodelica::index_checked(&ass1.borrow(), c)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(ass1.clone(), c, l)?;
            metamodelica::arrayUpdate(ass2.clone(), l, c)?;
            BFSBreasign(
                i,
                ({
                    let __elt = (*metamodelica::index_checked(&parentcolum.borrow(), r)?).clone();
                    __elt
                }),
                parentcolum.clone(),
                r,
                ass1.clone(),
                ass2.clone(),
            )?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function BFSBreasign failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn BFSBenque(
    mut queue: metamodelica::List<i32>,
    mut rowmark: i32,
    mut c: i32,
    mut rc: i32,
    mut r: i32,
    mut visited: bool,
    mut rowmarks: metamodelica::Array<i32>,
    mut parentcolum: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outEqnqueue: metamodelica::List<i32>;
    outEqnqueue = (match visited {
        false => queue,
        true => {
            metamodelica::arrayUpdate(rowmarks.clone(), r, rowmark)?;
            metamodelica::arrayUpdate(parentcolum.clone(), r, c)?;
            metamodelica::cons(rc, queue)
        }
        _ => {
            Error::addInternalError(
                literal!("function BFSBenque failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outEqnqueue)
}

pub(crate) fn DFSB(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    rowmarks = arrayCreate(nvars, -1);
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = DFSB1(1, 1, nvars, neqns, m.clone(), mt.clone(), rowmarks.clone(), vec1.clone(), vec2.clone(), &isyst, &ishared, inMatchingOptions, sssHandler, &inArg)?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.BFSB failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn DFSB1(
    mut i: i32,
    mut rowmark: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(i, ne)) else {
                return Err("pattern mismatch");
            };
            Ok((
                ass1.clone(),
                ass2.clone(),
                isyst.clone(),
                ishared.clone(),
                inArg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut visitedcolums: metamodelica::List<i32>;
            let mut m1: metamodelica::Array<metamodelica::List<i32>>;
            let mut mt1: metamodelica::Array<metamodelica::List<i32>>;
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut i_1: i32;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass1_2: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut ass2_2: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let false = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            visitedcolums = DFSBphase(
                &(list![i]),
                rowmark,
                i,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                ass1.clone(),
                ass2.clone(),
                list![i],
            )?;
            let (__pa0, __pa3, __pa1, __pa2, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i, isyst.clone(), ishared.clone(), nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg.clone())?) {
                (_, __pa0, __pa3 @ Deref @ BackendDAE::EqSystem { m: Some(__pa1), mT: Some(__pa2), .. }, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i_1 = metamodelica::Own::own(__pa0);
            m1 = metamodelica::Own::own(__pa1);
            mt1 = metamodelica::Own::own(__pa2);
            syst = metamodelica::Own::own(__pa3);
            shared = metamodelica::Own::own(__pa4);
            nv_1 = metamodelica::Own::own(__pa5);
            ne_1 = metamodelica::Own::own(__pa6);
            ass1_1 = metamodelica::Own::own(__pa7);
            ass2_1 = metamodelica::Own::own(__pa8);
            arg = metamodelica::Own::own(__pa9);
            rowmarks1 =
                assignmentsArrayExpand(rowmarks.clone(), nv_1, metamodelica::arrayLength(rowmarks.clone()), -1)?;
            (ass1_2, ass2_2, syst, shared, arg) = DFSB1(
                i_1,
                rowmark + 1,
                nv_1,
                ne_1,
                m1.clone(),
                mt1.clone(),
                rowmarks1.clone(),
                ass1_1.clone(),
                ass2_1.clone(),
                &syst,
                &shared,
                inMatchingOptions,
                sssHandler,
                &arg,
            )?;
            Ok((
                ass1_2.clone(),
                ass2_2.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let true = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            (ass1_1, ass2_1, syst, shared, arg) = DFSB1(
                i + 1,
                rowmark,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                ass1.clone(),
                ass2.clone(),
                isyst,
                ishared,
                inMatchingOptions,
                sssHandler,
                inArg,
            )?;
            Ok((
                ass1_1.clone(),
                ass2_1.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("function DFSB1 failed in equation "));
                    __mm_s.push_str(&*intString(i));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn DFSBphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            inVisitedColums
        },
        _ => {
            let mut rows: metamodelica::List<i32>;
            rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            DFSBtraverseRows(&rows, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), &inVisitedColums)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function DFSBphase failed in equation ")); __mm_s.push_str(&*intString(c)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

fn DFSBtraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inVisitedColums.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    DFSBreasign(stack, r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut visitedColums: metamodelica::List<i32>;
                    let mut rc: i32;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    visitedColums = DFSBphase(&(metamodelica::cons(rc, stack.clone())), i, rc, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc, inVisitedColums.clone()))?;
                    Ok(DFSBtraverseRows1(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), visitedColums.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(DFSBtraverseRows(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), inVisitedColums))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function DFSBtraverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVisitedColums
}

fn DFSBtraverseRows1(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (::match_deref::match_deref! { match &(inVisitedColums.clone()) {
        Deref @ metamodelica::ListNode::Nil => inVisitedColums,
        _ => DFSBtraverseRows(rows, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), ass1.clone(), ass2.clone(), &inVisitedColums),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outVisitedColums
}

fn DFSBreasign(
    mut stack: &metamodelica::List<i32>,
    mut r: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
            let mut rc: i32;
            rc = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt});
            metamodelica::arrayUpdate(ass1.clone(), c.clone(), r)?;
            metamodelica::arrayUpdate(ass2.clone(), r, c.clone())?;
            DFSBreasign(rest, rc, ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn MC21A(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut lookahead: metamodelica::Array<i32>;
                    let mut syst = (*syst).clone();
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    rowmarks = arrayCreate(nvars, -1);
                    lookahead = arrayCreate(neqns, 0);
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), false)?;
                    (vec1, vec2, syst, shared, arg) = MC21A1(1, 1, nvars, neqns, m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), vec1.clone(), vec2.clone(), &isyst, &ishared, inMatchingOptions, sssHandler, &inArg)?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.MC21A failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn MC21A1(
    mut i: i32,
    mut rowmark: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(i, ne)) else {
                return Err("pattern mismatch");
            };
            Ok((
                ass1.clone(),
                ass2.clone(),
                isyst.clone(),
                ishared.clone(),
                inArg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut visitedcolums: metamodelica::List<i32>;
            let mut changedEqns: metamodelica::List<i32>;
            let mut m1: metamodelica::Array<metamodelica::List<i32>>;
            let mut mt1: metamodelica::Array<metamodelica::List<i32>>;
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut i_1: i32;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass1_2: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut ass2_2: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut lookahead1: metamodelica::Array<i32>;
            let false = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            visitedcolums = MC21Aphase(
                &(list![i]),
                rowmark,
                i,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                lookahead.clone(),
                ass1.clone(),
                ass2.clone(),
                list![i],
            )?;
            let (__pa0, __pa1, __pa4, __pa2, __pa3, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(reduceIndexifNecessary(visitedcolums.clone(), i, isyst.clone(), ishared.clone(), nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg.clone())?) {
                (__pa0, __pa1, __pa4 @ Deref @ BackendDAE::EqSystem { m: Some(__pa2), mT: Some(__pa3), .. }, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => return Err("pattern mismatch"),
            } };
            changedEqns = metamodelica::Own::own(__pa0);
            i_1 = metamodelica::Own::own(__pa1);
            m1 = metamodelica::Own::own(__pa2);
            mt1 = metamodelica::Own::own(__pa3);
            syst = metamodelica::Own::own(__pa4);
            shared = metamodelica::Own::own(__pa5);
            nv_1 = metamodelica::Own::own(__pa6);
            ne_1 = metamodelica::Own::own(__pa7);
            ass1_1 = metamodelica::Own::own(__pa8);
            ass2_1 = metamodelica::Own::own(__pa9);
            arg = metamodelica::Own::own(__pa10);
            (rowmarks1, lookahead1) = MC21A1fixArrays(
                &visitedcolums,
                nv_1,
                ne_1,
                rowmarks.clone(),
                lookahead.clone(),
                &changedEqns,
            )?;
            (ass1_2, ass2_2, syst, shared, arg) = MC21A1(
                i_1,
                rowmark + 1,
                nv_1,
                ne_1,
                m1.clone(),
                mt1.clone(),
                rowmarks1.clone(),
                lookahead1.clone(),
                ass1_1.clone(),
                ass2_1.clone(),
                &syst,
                &shared,
                inMatchingOptions,
                sssHandler,
                &arg,
            )?;
            Ok((
                ass1_2.clone(),
                ass2_2.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let true = (intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
                    __elt
                }),
                0,
            )) else {
                return Err("pattern mismatch");
            };
            (ass1_1, ass2_1, syst, shared, arg) = MC21A1(
                i + 1,
                rowmark,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                lookahead.clone(),
                ass1.clone(),
                ass2.clone(),
                isyst,
                ishared,
                inMatchingOptions,
                sssHandler,
                inArg,
            )?;
            Ok((
                ass1_1.clone(),
                ass2_1.clone(),
                syst.clone(),
                shared.clone(),
                arg.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("function MC21A1 failed in equation "));
                    __mm_s.push_str(&*intString(i));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn MC21A1fixArrays(
    mut meqns: &metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut changedEqns: &metamodelica::List<i32>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outlookahead: metamodelica::Array<i32>;
    (outrowmarks, outlookahead) = (::match_deref::match_deref! { match meqns {
        Deref @ metamodelica::ListNode::Nil => {
            (rowmarks.clone(), lookahead.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut memsize: i32;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut lookahead1: metamodelica::Array<i32>;
            memsize = metamodelica::arrayLength(rowmarks.clone());
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv, memsize, -1)?;
            lookahead1 = assignmentsArrayExpand(lookahead.clone(), ne, memsize, 0)?;
            MC21A1fixArray(changedEqns, lookahead1.clone())?;
            (rowmarks1.clone(), lookahead1.clone())
        },
        _ => {
            Error::addInternalError(literal!("function MC21A1fixArrays failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outrowmarks, outlookahead))
}

fn MC21A1fixArray(mut meqns: &metamodelica::List<i32>, mut arr: metamodelica::Array<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match meqns {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
            metamodelica::arrayUpdate(arr.clone(), e.clone(), 0)?;
            MC21A1fixArray(rest, arr.clone())?;
            ()
        },
        _ => {
            Error::addInternalError(literal!("function MC21A1fixArray failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn MC21Aphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            inVisitedColums
        },
        _ => {
            let mut rows: metamodelica::List<i32>;
            let mut b: bool;
            rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = (*metamodelica::index_checked(&lookahead.borrow(), c)?).clone(); __elt}), ((rows).len() as i32));
            MC21Achecklookahead(b, &rows, stack, i, c, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), &inVisitedColums)?
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function MC21Aphase failed in equation ")); __mm_s.push_str(&*intString(c)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVisitedColums)
}

fn MC21Achecklookahead(
    mut dolookahaed: bool,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (match dolookahaed {
        true => MC21AtraverseRowsUnmatched(
            rows,
            rows,
            stack,
            i,
            c,
            ((rows).len() as i32),
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
            inVisitedColums,
        )?,
        _ => MC21AtraverseRows(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
            inVisitedColums,
        ),
    });
    Ok(outVisitedColums)
}

fn MC21AtraverseRowsUnmatched(
    mut rows: &metamodelica::List<i32>,
    mut rows1: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c, l)?;
                    Ok(MC21AtraverseRows(rows1, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    DFSBreasign(stack, r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(MC21AtraverseRowsUnmatched(metamodelica::AsArg::as_arg(&rest), rows1, stack, i, c, l, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVisitedColums)
}

fn MC21AtraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inVisitedColums.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut visitedColums: metamodelica::List<i32>;
                    let mut rc: i32;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    visitedColums = MC21Aphase(&(metamodelica::cons(rc, stack.clone())), i, rc, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), metamodelica::cons(rc, inVisitedColums.clone()))?;
                    Ok(MC21AtraverseRows1(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), visitedColums.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(MC21AtraverseRows(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), inVisitedColums))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function MC21AtraverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVisitedColums
}

fn MC21AtraverseRows1(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inVisitedColums: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outVisitedColums: metamodelica::List<i32>;
    outVisitedColums = (::match_deref::match_deref! { match &(inVisitedColums.clone()) {
        Deref @ metamodelica::ListNode::Nil => inVisitedColums,
        _ => MC21AtraverseRows(rows, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), &inVisitedColums),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outVisitedColums
}

pub(crate) fn PF(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut lookahead: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    rowmarks = arrayCreate(nvars, -1);
                    lookahead = arrayCreate(neqns, 0);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = PF1(0, unmatched.clone(), rowmarks.clone(), lookahead.clone(), isyst.clone(), ishared.clone(), nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.PF failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn PF1<'__b>(
    mut i: i32,
    mut unmatched: metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((ass1.clone(), ass2.clone(), isyst, ishared, inArg))
            },
            (_, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
                let mut nv_1: i32;
                let mut ne_1: i32;
                let mut i_1: i32;
                let mut unmatched1: metamodelica::List<i32>;
                let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut ass1_1: metamodelica::Array<i32>;
                let mut ass1_2: metamodelica::Array<i32>;
                let mut ass2_1: metamodelica::Array<i32>;
                let mut ass2_2: metamodelica::Array<i32>;
                let mut rowmarks1: metamodelica::Array<i32>;
                let mut lookahead1: metamodelica::Array<i32>;
                (i_1, unmatched1) = PFaugmentmatching(i, &(unmatched.clone()), nv, ne, m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), ((unmatched).len() as i32), &(metamodelica::nil()))?;
                meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), &inArg)?;
                (unmatched1, rowmarks1, lookahead1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PF2(meqns, unmatched1, &(metamodelica::nil()), rowmarks.clone(), lookahead.clone(), isyst, ishared, nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg)?;
                { (i, unmatched, rowmarks, lookahead, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1 + 1, unmatched1, rowmarks1.clone(), lookahead1.clone(), syst, shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn PF2(
    mut meqns: metamodelica::List<metamodelica::List<i32>>,
    mut unmatched: metamodelica::List<i32>,
    mut changedEqns: &metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    i32,
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outunmatched: metamodelica::List<i32>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outlookahead: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (
        outunmatched,
        outrowmarks,
        outlookahead,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (unmatched, rowmarks.clone(), lookahead.clone(), nv, ne, ass1.clone(), ass2.clone(), isyst, ishared, inArg)
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut unmatched1: metamodelica::List<i32>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut lookahead1: metamodelica::Array<i32>;
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns, 0, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
            ne_1 = BackendDAEUtil::systemSize(&syst)?;
            nv_1 = BackendVariable::daenumVariables(&syst);
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1, ne, -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1, nv, -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1, nv, -1)?;
            lookahead1 = assignmentsArrayExpand(lookahead.clone(), ne_1, ne, 0)?;
            MC21A1fixArray(&unmatched1, lookahead1.clone())?;
            (unmatched1, rowmarks1.clone(), lookahead1.clone(), nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), syst, shared, arg)
        },
        (_, _) => {
            singularSystemError(meqns, 0, &isyst, &ishared, ass1.clone(), ass2.clone(), &inArg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        outunmatched,
        outrowmarks,
        outlookahead,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ))
}

fn PFaugmentmatching(
    mut i: i32,
    mut U: &metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut previousUnmatched: i32,
    mut unMatched: &metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut outI: i32;
    let mut outUnmatched: metamodelica::List<i32>;
    (outI, outUnmatched) = 'mc: {
        let __mc_input = &**U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let true = (intEq(previousUnmatched, ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    Ok((i, unMatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    (i_1, unmatched) = PFaugmentmatching(i + 1, unMatched, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), ((unMatched).len() as i32), &(metamodelica::nil()))?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    let true = (intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), -1)) else { return Err("pattern mismatch") };
                    (i_1, unmatched) = PFaugmentmatching(i, metamodelica::AsArg::as_arg(&rest), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched, unMatched)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    let mut b: bool;
                    b = PFphase(&(list![c.clone()]), i, c.clone(), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = List::consOnTrue(!(b), c.clone(), unMatched.clone());
                    (i_1, unmatched) = PFaugmentmatching(i, metamodelica::AsArg::as_arg(&rest), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched, &unmatched)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PFaugmentmatching failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outI, outUnmatched))
}

fn PFphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            false
        },
        _ => {
            let mut rows: metamodelica::List<i32>;
            let mut b: bool;
            rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = (*metamodelica::index_checked(&lookahead.borrow(), c)?).clone(); __elt}), ((rows).len() as i32));
            PFchecklookahead(b, &rows, stack, i, c, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function PFphase failed in equation ")); __mm_s.push_str(&*intString(c)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn PFchecklookahead(
    mut dolookahaed: bool,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut matched: bool;
    matched = (match dolookahaed {
        true => PFtraverseRowsUnmatched(
            rows,
            rows,
            stack,
            i,
            c,
            ((rows).len() as i32),
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
        )?,
        _ => PFtraverseRows(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
        ),
    });
    Ok(matched)
}

fn PFtraverseRowsUnmatched(
    mut rows: &metamodelica::List<i32>,
    mut rows1: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c, l)?;
                    Ok(PFtraverseRows(rows1, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    DFSBreasign(stack, r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(PFtraverseRowsUnmatched(metamodelica::AsArg::as_arg(&rest), rows1, stack, i, c, l, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(matched)
}

fn PFtraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> bool {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut rc: i32;
                    let mut b: bool;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    b = PFphase(&(metamodelica::cons(rc, stack.clone())), i, rc, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone())?;
                    Ok(PFtraverseRows1(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(PFtraverseRows(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PFtraverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    matched
}

fn PFtraverseRows1(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
) -> bool {
    let mut matched: bool;
    matched = (match inMatched {
        true => inMatched,
        _ => PFtraverseRows(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
        ),
    });
    matched
}

pub(crate) fn PFPlus(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut lookahead: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    rowmarks = arrayCreate(nvars, -1);
                    lookahead = arrayCreate(neqns, 0);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (_, vec1, vec2, syst, shared, arg) = PFPlus1(0, unmatched.clone(), rowmarks.clone(), lookahead.clone(), isyst.clone(), ishared.clone(), nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.PFPlus failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn PFPlus1<'__b>(
    mut i: i32,
    mut unmatched: metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((i, ass1.clone(), ass2.clone(), isyst, ishared, inArg))
            },
            (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
                let mut nv_1: i32;
                let mut ne_1: i32;
                let mut i_1: i32;
                let mut unmatched1: metamodelica::List<i32>;
                let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut ass1_1: metamodelica::Array<i32>;
                let mut ass1_2: metamodelica::Array<i32>;
                let mut ass2_1: metamodelica::Array<i32>;
                let mut ass2_2: metamodelica::Array<i32>;
                let mut rowmarks1: metamodelica::Array<i32>;
                let mut lookahead1: metamodelica::Array<i32>;
                let mut syst = (*syst).clone();
                (i_1, unmatched1) = PFPlusaugmentmatching(i, &(unmatched.clone()), nv, ne, m.clone(), mt.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), ((unmatched).len() as i32), &(metamodelica::nil()), false)?;
                meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), &inArg)?;
                (unmatched1, rowmarks1, lookahead1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PF2(meqns, unmatched1, &(metamodelica::nil()), rowmarks.clone(), lookahead.clone(), syst.clone(), ishared, nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg)?;
                { (i, unmatched, rowmarks, lookahead, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1 + 1, unmatched1, rowmarks1.clone(), lookahead1.clone(), syst.clone(), shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn PFPlusaugmentmatching(
    mut i: i32,
    mut U: &metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut previousUnmatched: i32,
    mut unMatched: &metamodelica::List<i32>,
    mut reverseRows: bool,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut outI: i32;
    let mut outUnMatched: metamodelica::List<i32>;
    (outI, outUnMatched) = 'mc: {
        let __mc_input = &**U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let true = (intEq(previousUnmatched, ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    Ok((i, unMatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    (i_1, unmatched) = PFPlusaugmentmatching(i + 1, unMatched, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), ((unMatched).len() as i32), &(metamodelica::nil()), reverseRows)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    let true = (intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), -1)) else { return Err("pattern mismatch") };
                    (i_1, unmatched) = PFPlusaugmentmatching(i, metamodelica::AsArg::as_arg(&rest), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched, unMatched, reverseRows)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    let mut b: bool;
                    b = PFPlusphase(&(list![c.clone()]), i, c.clone(), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows)?;
                    unmatched = List::consOnTrue(!(b), c.clone(), unMatched.clone());
                    (i_1, unmatched) = PFPlusaugmentmatching(i, metamodelica::AsArg::as_arg(&rest), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), previousUnmatched, &unmatched, !(reverseRows))?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PFPlusaugmentmatching failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outI, outUnMatched))
}

fn PFPlusphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut reverseRows: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match &((stack.clone(), reverseRows)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            false
        },
        (_, false) => {
            let mut rows: metamodelica::List<i32>;
            let mut b: bool;
            rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = (*metamodelica::index_checked(&lookahead.borrow(), c)?).clone(); __elt}), ((rows).len() as i32));
            PFPluschecklookahead(b, &rows, stack, i, c, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows)?
        },
        (_, true) => {
            let mut rows: metamodelica::List<i32>;
            let mut b: bool;
            rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            b = intLt(({let __elt = (*metamodelica::index_checked(&lookahead.borrow(), c)?).clone(); __elt}), ((rows).len() as i32));
            PFPluschecklookahead(b, &(rows.reverse()), stack, i, c, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows)?
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function PFPlusphase failed in equation ")); __mm_s.push_str(&*intString(c)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn PFPluschecklookahead(
    mut dolookahaed: bool,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut reverseRows: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = (match dolookahaed {
        true => PFPlustraverseRowsUnmatched(
            rows,
            rows,
            stack,
            i,
            c,
            ((rows).len() as i32),
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
            reverseRows,
        )?,
        _ => PFPlustraverseRows(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
            reverseRows,
        ),
    });
    Ok(matched)
}

fn PFPlustraverseRowsUnmatched(
    mut rows: &metamodelica::List<i32>,
    mut rows1: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut c: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut reverseRows: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::arrayUpdate(lookahead.clone(), c, l)?;
                    Ok(PFPlustraverseRows(rows1, stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    DFSBreasign(stack, r.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(PFPlustraverseRowsUnmatched(metamodelica::AsArg::as_arg(&rest), rows1, stack, i, c, l, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(matched)
}

fn PFPlustraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut reverseRows: bool,
) -> bool {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut rc: i32;
                    let mut b: bool;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    b = PFPlusphase(&(metamodelica::cons(rc, stack.clone())), i, rc, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows)?;
                    Ok(PFPlustraverseRows1(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), b, reverseRows))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(PFPlustraverseRows(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), lookahead.clone(), ass1.clone(), ass2.clone(), reverseRows))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PFPlustraverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    matched
}

fn PFPlustraverseRows1(
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut lookahead: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
    mut reverseRows: bool,
) -> bool {
    let mut matched: bool;
    matched = (match inMatched {
        true => inMatched,
        _ => PFPlustraverseRows(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            lookahead.clone(),
            ass1.clone(),
            ass2.clone(),
            reverseRows,
        ),
    });
    matched
}

pub(crate) fn HK(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut level: metamodelica::Array<i32>;
                    let mut collummarks: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    rowmarks = arrayCreate(nvars, -1);
                    collummarks = arrayCreate(neqns, -1);
                    level = arrayCreate(neqns, -1);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = HK1(0, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), isyst.clone(), ishared.clone(), nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.HK failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn HK1<'__b>(
    mut i: i32,
    mut unmatched: metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((ass1.clone(), ass2.clone(), isyst, ishared, inArg))
            },
            (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
                let mut nv_1: i32;
                let mut ne_1: i32;
                let mut i_1: i32;
                let mut unmatched1: metamodelica::List<i32>;
                let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut ass1_1: metamodelica::Array<i32>;
                let mut ass1_2: metamodelica::Array<i32>;
                let mut ass2_1: metamodelica::Array<i32>;
                let mut ass2_2: metamodelica::Array<i32>;
                let mut rowmarks1: metamodelica::Array<i32>;
                let mut collummarks1: metamodelica::Array<i32>;
                let mut level1: metamodelica::Array<i32>;
                let mut syst = (*syst).clone();
                (i_1, unmatched1) = HKphase(i, &(unmatched.clone()), nv, ne, m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ((unmatched).len() as i32), &(metamodelica::nil()))?;
                meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), &inArg)?;
                (unmatched1, rowmarks1, collummarks1, level1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = HK2(meqns, unmatched1, &(metamodelica::nil()), rowmarks.clone(), collummarks.clone(), level.clone(), syst.clone(), ishared, nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg)?;
                { (i, unmatched, rowmarks, collummarks, level, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1 + 1, unmatched1, rowmarks1.clone(), collummarks1.clone(), level1.clone(), syst.clone(), shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn HK2(
    mut meqns: metamodelica::List<metamodelica::List<i32>>,
    mut unmatched: metamodelica::List<i32>,
    mut changedEqns: &metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    i32,
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outunmatched: metamodelica::List<i32>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outcollummarks: metamodelica::Array<i32>;
    let mut outlevel: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (
        outunmatched,
        outrowmarks,
        outcollummarks,
        outlevel,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (unmatched, rowmarks.clone(), collummarks.clone(), level.clone(), nv, ne, ass1.clone(), ass2.clone(), isyst, ishared, inArg)
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut unmatched1: metamodelica::List<i32>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut collummarks1: metamodelica::Array<i32>;
            let mut level1: metamodelica::Array<i32>;
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns, 0, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
            ne_1 = BackendDAEUtil::systemSize(&syst)?;
            nv_1 = BackendVariable::daenumVariables(&syst);
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1, metamodelica::arrayLength(ass1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1, metamodelica::arrayLength(ass2.clone()), -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1, metamodelica::arrayLength(rowmarks.clone()), -1)?;
            collummarks1 = assignmentsArrayExpand(collummarks.clone(), ne_1, metamodelica::arrayLength(collummarks.clone()), -1)?;
            level1 = assignmentsArrayExpand(level.clone(), ne_1, metamodelica::arrayLength(level.clone()), -1)?;
            (unmatched1, rowmarks1.clone(), collummarks1.clone(), level1.clone(), nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), syst, shared, arg)
        },
        (_, _) => {
            singularSystemError(meqns, 0, &isyst, &ishared, ass1.clone(), ass2.clone(), &inArg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        outunmatched,
        outrowmarks,
        outcollummarks,
        outlevel,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ))
}

fn HKphase(
    mut i: i32,
    mut U: &metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut previousUnmatched: i32,
    mut unMatched: &metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut outI: i32;
    let mut outunMatched: metamodelica::List<i32>;
    (outI, outunMatched) = 'mc: {
        let __mc_input = &**U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let true = (intEq(previousUnmatched, ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    Ok((i, unMatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    (i_1, unmatched) = HKphase(i + 1, unMatched, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ((unMatched).len() as i32), &(metamodelica::nil()))?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut rows: metamodelica::List<(i32, i32)>;
                    let mut i_1: i32;
                    rows = HKBFS(U, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), i, level.clone(), None, ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    HKDFS(&rows, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    unmatched = HKgetUnmatched(U, ass1.clone(), &(metamodelica::nil()));
                    (i_1, unmatched) = HKphase(i, &(metamodelica::nil()), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), previousUnmatched, &unmatched)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function HKphase failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outI, outunMatched))
}

fn HKgetUnmatched(
    mut U: &metamodelica::List<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut inUnmatched: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outUnmatched: metamodelica::List<i32>;
    outUnmatched = 'mc: {
        let __mc_input = &**U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inUnmatched.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let true = (intGt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    Ok(HKgetUnmatched(metamodelica::AsArg::as_arg(&rest), ass1.clone(), inUnmatched))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    Ok(HKgetUnmatched(metamodelica::AsArg::as_arg(&rest), ass1.clone(), &(metamodelica::cons(c.clone(), inUnmatched.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outUnmatched
}

fn HKBFS<'__b>(
    mut colums: &'__b metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut i: i32,
    mut level: metamodelica::Array<i32>,
    mut lowestL: Option<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inRows: metamodelica::List<(i32, i32)>,
) -> Result<metamodelica::List<(i32, i32)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match colums {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inRows)
            },
            Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                let mut rows: metamodelica::List<(i32, i32)>;
                let mut ll: Option<i32>;
                (rows, ll) = HKBFSBphase(list![c.clone()], i, 0, lowestL, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows, metamodelica::nil())?;
                { (colums, nv, ne, m, mT, rowmarks, i, level, lowestL, ass1, ass2, inRows) = (rest, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), i, level.clone(), ll, ass1.clone(), ass2.clone(), rows); continue '__tco; }
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFS failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn HKBFSBphase(
    mut queue: metamodelica::List<i32>,
    mut i: i32,
    mut l: i32,
    mut lowestL: Option<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inRows: metamodelica::List<(i32, i32)>,
    mut queue1: metamodelica::List<i32>,
) -> Result<(metamodelica::List<(i32, i32)>, Option<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue, lowestL.clone(), queue1.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inRows, lowestL))
            },
            (Deref @ metamodelica::ListNode::Nil, Some(lowl), _) => {
                let mut rows: metamodelica::List<(i32, i32)>;
                let mut l_1: i32;
                let mut b: bool;
                let mut ll: Option<i32>;
                l_1 = l + 1;
                b = intGt(l_1, lowl.clone());
                return Ok(HKBFSBphase1(b, queue1, i, l_1, lowestL, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows, metamodelica::nil())?)
            },
            (Deref @ metamodelica::ListNode::Nil, None, _) => {
                let mut rows: metamodelica::List<(i32, i32)>;
                let mut ll: Option<i32>;
                { (queue, i, l, lowestL, nv, ne, m, mT, rowmarks, level, ass1, ass2, inRows, queue1) = (queue1, i, l + 1, lowestL, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows, metamodelica::nil()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _, _) => {
                let mut queue2: metamodelica::List<i32>;
                let mut cr: metamodelica::List<i32>;
                let mut rows: metamodelica::List<(i32, i32)>;
                let mut b: bool;
                let mut ll: Option<i32>;
                cr = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                metamodelica::arrayUpdate(level.clone(), c.clone(), l)?;
                (queue2, rows, b) = HKBFStraverseRows(&cr, &(metamodelica::nil()), i, l, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), &inRows, false)?;
                queue2 = listAppend(queue1, queue2);
                ll = if (b) {Some(l)} else {lowestL};
                { (queue, i, l, lowestL, nv, ne, m, mT, rowmarks, level, ass1, ass2, inRows, queue1) = (rest.clone(), i, l, ll, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), rows, queue2); continue '__tco; }
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFSBphase failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn HKBFSBphase1(
    mut inUnMaRowFound: bool,
    mut queue: metamodelica::List<i32>,
    mut i: i32,
    mut l: i32,
    mut lowestL: Option<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inRows: metamodelica::List<(i32, i32)>,
    mut queue1: metamodelica::List<i32>,
) -> Result<(metamodelica::List<(i32, i32)>, Option<i32>)> {
    let mut outRows: metamodelica::List<(i32, i32)>;
    let mut outlowestL: Option<i32>;
    (outRows, outlowestL) = (match inUnMaRowFound {
        true => (inRows, Some(l)),
        false => {
            let mut ll: Option<i32>;
            let mut rows: metamodelica::List<(i32, i32)>;
            (rows, ll) = HKBFSBphase(
                queue,
                i,
                l,
                lowestL,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                rowmarks.clone(),
                level.clone(),
                ass1.clone(),
                ass2.clone(),
                inRows,
                queue1,
            )?;
            (rows, ll)
        }
        _ => {
            Error::addInternalError(
                literal!("function HKBFSBphase1 failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok((outRows, outlowestL))
}

fn HKBFStraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut queue: &metamodelica::List<i32>,
    mut i: i32,
    mut l: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inRows: &metamodelica::List<(i32, i32)>,
    mut inunmarowFound: bool,
) -> Result<(metamodelica::List<i32>, metamodelica::List<(i32, i32)>, bool)> {
    let mut outEqnqueue: metamodelica::List<i32>;
    let mut outRows: metamodelica::List<(i32, i32)>;
    let mut unmarowFound: bool;
    (outEqnqueue, outRows, unmarowFound) = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((queue.clone().reverse(), inRows.clone(), inunmarowFound))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut rowstpl: metamodelica::List<(i32, i32)>;
                    let mut b: bool;
                    let false = (intLt(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    (queue1, rowstpl, b) = HKBFStraverseRows(metamodelica::AsArg::as_arg(&rest), queue, i, l, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows, inunmarowFound)?;
                    Ok((queue1.clone(), rowstpl.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut rowstpl: metamodelica::List<(i32, i32)>;
                    let mut b: bool;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    (queue1, rowstpl, b) = HKBFStraverseRows(metamodelica::AsArg::as_arg(&rest), queue, i, l, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), &(metamodelica::cons((r.clone(), l), inRows.clone())), true)?;
                    Ok((queue1.clone(), rowstpl.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut rowstpl: metamodelica::List<(i32, i32)>;
                    let mut rc: i32;
                    let mut b: bool;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    (queue1, rowstpl, b) = HKBFStraverseRows(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(rc, queue.clone())), i, l, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inRows, inunmarowFound)?;
                    Ok((queue1.clone(), rowstpl.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKBFStraverseRows failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEqnqueue, outRows, unmarowFound))
}

fn HKDFS<'__b>(
    mut unmatchedRows: &'__b metamodelica::List<(i32, i32)>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inUnmatchedRows: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match unmatchedRows {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inUnmatchedRows)
            },
            Deref @ metamodelica::ListNode::Cons { head: (r, l), tail: rest } => {
                let mut ur: metamodelica::List<i32>;
                let mut b: bool;
                b = HKDFSphase(&(list![r.clone()]), i, r.clone(), l.clone(), nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), false)?;
                ur = List::consOnTrue(!(b), r.clone(), inUnmatchedRows);
                { (unmatchedRows, i, nv, ne, m, mT, collummarks, level, ass1, ass2, inUnmatchedRows) = (rest, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ur); continue '__tco; }
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDFS failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn HKDFSphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut r: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            inMatched
        },
        _ => {
            let mut collums: metamodelica::List<i32>;
            collums = List::select(({let __elt = (*metamodelica::index_checked(&mT.borrow(), r)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            HKDFStraverseCollums(&collums, stack, i, l, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched)?
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDFSphase failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn HKDFStraverseCollums(
    mut collums: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**collums;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inMatched)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&level.borrow(), c.clone())?).clone(); __elt}), l)) else { return Err("pattern mismatch") };
                    Ok(HKDFStraverseCollums(metamodelica::AsArg::as_arg(&rest), stack, i, l, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: _ } => {
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&level.borrow(), c.clone())?).clone(); __elt}), l)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    HKDFSreasign(stack, c.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut r: i32;
                    let mut b: bool;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&level.borrow(), c.clone())?).clone(); __elt}), l)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    r = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt});
                    let false = (intLt(r, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(collummarks.clone(), c.clone(), i)?;
                    b = HKDFSphase(&(metamodelica::cons(r, stack.clone())), i, r, l - 1, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched)?;
                    Ok(HKDFStraverseCollums1(b, metamodelica::AsArg::as_arg(&rest), stack, i, l, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&level.borrow(), c.clone())?).clone(); __elt}), l)) else { return Err("pattern mismatch") };
                    let false = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    Ok(HKDFStraverseCollums(metamodelica::AsArg::as_arg(&rest), stack, i, l, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), inMatched)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function HKDFStraverseCollums failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(matched)
}

fn HKDFStraverseCollums1(
    mut inMatched: bool,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut l: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched {
        true => inMatched,
        _ => HKDFStraverseCollums(
            rows,
            stack,
            i,
            l,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            collummarks.clone(),
            level.clone(),
            ass1.clone(),
            ass2.clone(),
            inMatched,
        )?,
    });
    Ok(matched)
}

fn HKDFSreasign(
    mut stack: &metamodelica::List<i32>,
    mut c: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
            let mut cr: i32;
            cr = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
            metamodelica::arrayUpdate(ass1.clone(), c, r.clone())?;
            metamodelica::arrayUpdate(ass2.clone(), r.clone(), c)?;
            HKDFSreasign(rest, cr, ass1.clone(), ass2.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn HKDW(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut level: metamodelica::Array<i32>;
                    let mut collummarks: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    rowmarks = arrayCreate(nvars, -1);
                    collummarks = arrayCreate(neqns, -1);
                    level = arrayCreate(neqns, -1);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = HKDW1(0, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), isyst.clone(), ishared.clone(), nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.HKDW failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn HKDW1<'__b>(
    mut i: i32,
    mut unmatched: metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((ass1.clone(), ass2.clone(), isyst, ishared, inArg))
            },
            (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
                let mut nv_1: i32;
                let mut ne_1: i32;
                let mut i_1: i32;
                let mut unmatched1: metamodelica::List<i32>;
                let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut ass1_1: metamodelica::Array<i32>;
                let mut ass1_2: metamodelica::Array<i32>;
                let mut ass2_1: metamodelica::Array<i32>;
                let mut ass2_2: metamodelica::Array<i32>;
                let mut rowmarks1: metamodelica::Array<i32>;
                let mut collummarks1: metamodelica::Array<i32>;
                let mut level1: metamodelica::Array<i32>;
                let mut syst = (*syst).clone();
                (i_1, unmatched1) = HKDWphase(i, &(unmatched.clone()), nv, ne, m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ((unmatched).len() as i32), metamodelica::nil())?;
                meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), &inArg)?;
                (unmatched1, rowmarks1, collummarks1, level1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = HK2(meqns, unmatched1, &(metamodelica::nil()), rowmarks.clone(), collummarks.clone(), level.clone(), syst.clone(), ishared, nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg)?;
                { (i, unmatched, rowmarks, collummarks, level, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1 + 1, unmatched1, rowmarks1.clone(), collummarks1.clone(), level1.clone(), syst.clone(), shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn HKDWphase(
    mut i: i32,
    mut U: &metamodelica::List<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut previousUnmatched: i32,
    mut unMatched: metamodelica::List<i32>,
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut outI: i32;
    let mut outunMatched: metamodelica::List<i32>;
    (outI, outunMatched) = 'mc: {
        let __mc_input = &**U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let true = (intEq(previousUnmatched, ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    Ok((i, unMatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    (i_1, unmatched) = HKphase(i + 1, &(unMatched.clone()), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ((unMatched).len() as i32), &(metamodelica::nil()))?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut rows: metamodelica::List<(i32, i32)>;
                    let mut ur: metamodelica::List<i32>;
                    let mut i_1: i32;
                    rows = HKBFS(U, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), i, level.clone(), None, ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    ur = HKDFS(&rows, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    HKDWDFS(&ur, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = HKgetUnmatched(U, ass1.clone(), &(metamodelica::nil()));
                    (i_1, unmatched) = HKphase(i, &(metamodelica::nil()), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), previousUnmatched, &unmatched)?;
                    Ok((i_1, unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function HKDWphase failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outI, outunMatched))
}

fn HKDWDFS(
    mut unmatchedRows: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match unmatchedRows {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
            HKDWDFSphase(&(list![r.clone()]), i, r.clone(), nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), false)?;
            HKDWDFS(rest, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDWDFS failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn HKDWDFSphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut r: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            inMatched
        },
        _ => {
            let mut collums: metamodelica::List<i32>;
            collums = List::select(({let __elt = (*metamodelica::index_checked(&mT.borrow(), r)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            HKDWDFStraverseCollums(&collums, stack, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched)?
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function HKDWDFSphase failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matched)
}

fn HKDWDFStraverseCollums(
    mut collums: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatched: bool,
) -> Result<bool> {
    let mut matched: bool;
    matched = 'mc: {
        let __mc_input = &**collums;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inMatched)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    HKDFSreasign(stack, c.clone(), ass1.clone(), ass2.clone())?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut r: i32;
                    let mut b: bool;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    r = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt});
                    let false = (intLt(r, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(collummarks.clone(), c.clone(), i)?;
                    b = HKDWDFSphase(&(metamodelica::cons(r, stack.clone())), i, r, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched)?;
                    Ok(HKDWDFStraverseCollums1(b, metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let false = (intLt(({let __elt = (*metamodelica::index_checked(&collummarks.borrow(), c.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    Ok(HKDWDFStraverseCollums(metamodelica::AsArg::as_arg(&rest), stack, i, nv, ne, m.clone(), mT.clone(), collummarks.clone(), ass1.clone(), ass2.clone(), inMatched)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function HKDWDFStraverseCollums failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(matched)
}

fn HKDWDFStraverseCollums1(
    mut inMatched: bool,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut collummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut matched: bool;
    matched = (match inMatched {
        true => inMatched,
        _ => HKDWDFStraverseCollums(
            rows,
            stack,
            i,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            collummarks.clone(),
            ass1.clone(),
            ass2.clone(),
            inMatched,
        )?,
    });
    Ok(matched)
}

pub(crate) fn ABMP(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut level: metamodelica::Array<i32>;
                    let mut collummarks: metamodelica::Array<i32>;
                    let mut rlevel: metamodelica::Array<i32>;
                    let mut colptrs: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    rowmarks = arrayCreate(nvars, -1);
                    collummarks = arrayCreate(neqns, -1);
                    level = arrayCreate(neqns, -1);
                    rlevel = arrayCreate(nvars, nvars);
                    colptrs = arrayCreate(neqns, -1);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = ABMP1(1, unmatched.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), colptrs.clone(), isyst.clone(), ishared.clone(), nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.ABMP failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn ABMP1<'__b>(
    mut i: i32,
    mut unmatched: metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut rlevel: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((unmatched.clone(), isyst.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((ass1.clone(), ass2.clone(), isyst, ishared, inArg))
            },
            (_, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }) => {
                let mut nv_1: i32;
                let mut ne_1: i32;
                let mut i_1: i32;
                let mut lim: i32;
                let mut unmatched1: metamodelica::List<i32>;
                let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut ass1_1: metamodelica::Array<i32>;
                let mut ass1_2: metamodelica::Array<i32>;
                let mut ass2_1: metamodelica::Array<i32>;
                let mut ass2_2: metamodelica::Array<i32>;
                let mut rowmarks1: metamodelica::Array<i32>;
                let mut collummarks1: metamodelica::Array<i32>;
                let mut level1: metamodelica::Array<i32>;
                let mut rlevel1: metamodelica::Array<i32>;
                lim = ((metamodelica::OrderedFloat(0.1_f64) * (metamodelica::OrderedFloat((metamodelica::arrayLength(ass1.clone())) as f64)).sqrt()).0.floor() as i32);
                unmatched1 = ABMPphase(unmatched.clone(), i, nv, ne, m.clone(), mt.clone(), rowmarks.clone(), rlevel.clone(), colptrs.clone(), lim, ass1.clone(), ass2.clone())?;
                (i_1, unmatched1) = HKphase(i + 1, &(unmatched.clone()), nv, ne, m.clone(), mt.clone(), rowmarks.clone(), collummarks.clone(), level.clone(), ass1.clone(), ass2.clone(), ((unmatched).len() as i32), &(metamodelica::nil()))?;
                meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), &inArg)?;
                (unmatched1, rowmarks1, collummarks1, level1, rlevel1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = ABMP2(meqns, unmatched1, &(metamodelica::nil()), rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), isyst, ishared, nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg)?;
                { (i, unmatched, rowmarks, collummarks, level, rlevel, colptrs, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (i_1 + 1, unmatched1, rowmarks1.clone(), collummarks1.clone(), level1.clone(), rlevel1.clone(), colptrs.clone(), syst, shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn ABMP2(
    mut meqns: metamodelica::List<metamodelica::List<i32>>,
    mut unmatched: metamodelica::List<i32>,
    mut changedEqns: &metamodelica::List<i32>,
    mut rowmarks: metamodelica::Array<i32>,
    mut collummarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut rlevel: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    i32,
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outunmatched: metamodelica::List<i32>;
    let mut outrowmarks: metamodelica::Array<i32>;
    let mut outcollummarks: metamodelica::Array<i32>;
    let mut outlevel: metamodelica::Array<i32>;
    let mut outrlevel: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (
        outunmatched,
        outrowmarks,
        outcollummarks,
        outlevel,
        outrlevel,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (unmatched, rowmarks.clone(), collummarks.clone(), level.clone(), rlevel.clone(), nv, ne, ass1.clone(), ass2.clone(), isyst, ishared, inArg)
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut unmatched1: metamodelica::List<i32>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut rowmarks1: metamodelica::Array<i32>;
            let mut collummarks1: metamodelica::Array<i32>;
            let mut level1: metamodelica::Array<i32>;
            let mut rlevel1: metamodelica::Array<i32>;
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns, 0, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
            ne_1 = BackendDAEUtil::systemSize(&syst)?;
            nv_1 = BackendVariable::daenumVariables(&syst);
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1, metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1, metamodelica::arrayLength(ass2_1.clone()), -1)?;
            rowmarks1 = assignmentsArrayExpand(rowmarks.clone(), nv_1, metamodelica::arrayLength(rowmarks.clone()), -1)?;
            collummarks1 = assignmentsArrayExpand(collummarks.clone(), ne_1, metamodelica::arrayLength(collummarks.clone()), -1)?;
            rlevel1 = arrayCreate(metamodelica::arrayLength(ass2_1.clone()), metamodelica::arrayLength(ass2_1.clone()));
            level1 = assignmentsArrayExpand(level.clone(), ne_1, metamodelica::arrayLength(level.clone()), -1)?;
            (unmatched1, rowmarks1.clone(), collummarks1.clone(), level1.clone(), rlevel1.clone(), nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), syst, shared, arg)
        },
        (_, _) => {
            singularSystemError(meqns, 0, &isyst, &ishared, ass2.clone(), ass1.clone(), &inArg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        outunmatched,
        outrowmarks,
        outcollummarks,
        outlevel,
        outrlevel,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ))
}

fn ABMPphase(
    mut U: metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut lim: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut unMatched: metamodelica::List<i32>;
    unMatched = (::match_deref::match_deref! { match &(U.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => {
            let mut ur: metamodelica::List<i32>;
            ur = ABMPBFSphase(U.clone(), i, 0, lim, ((U).len() as i32), nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), metamodelica::nil())?;
            ABMPphase1(U, &ur, i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim, ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError(literal!("function ABMPphase failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(unMatched)
}

fn ABMPphase1(
    mut U: metamodelica::List<i32>,
    mut unmatchedRows: &metamodelica::List<i32>,
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut lim: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut unMatched: metamodelica::List<i32>;
    unMatched = (::match_deref::match_deref! { match unmatchedRows {
        Deref @ metamodelica::ListNode::Nil => {
            U
        },
        Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
            let mut unmatched: metamodelica::List<i32>;
            let mut L: i32;
            L = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r.clone())?).clone(); __elt});
            ABMPDFS(unmatchedRows, 0, L, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
            unmatched = HKgetUnmatched(&U, ass1.clone(), &(metamodelica::nil()));
            ABMPphase2(unmatched, i, L, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim, ass1.clone(), ass2.clone())?
        },
        _ => {
            Error::addInternalError(literal!("function ABMPphase1 failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(unMatched)
}

fn ABMPphase2(
    mut U: metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut lim: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut unMatched: metamodelica::List<i32>;
    unMatched = 'mc: {
        let __mc_input = &*U;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(U.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(50 * L, ((U).len() as i32))) else { return Err("pattern mismatch") };
                    Ok(U.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ABMPphase(U.clone(), i, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), colptrs.clone(), lim, ass1.clone(), ass2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ABMPphase2 failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(unMatched)
}

fn ABMPBFSphase(
    mut queue: metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut lim: i32,
    mut lim1: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextqueue: metamodelica::List<i32>,
    mut unMatched: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((queue, nextqueue.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(unMatched)
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                let mut l: i32;
                let mut b: bool;
                l = L + 2;
                b = intGt(l, lim) || intGt(50 * l, lim1);
                return Ok(ABMPBFSphase1(b, nextqueue, i, l, lim, lim1, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), metamodelica::nil(), unMatched)?)
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _) => {
                let mut rows: metamodelica::List<i32>;
                let mut queue1: metamodelica::List<i32>;
                let mut unmatched: metamodelica::List<i32>;
                rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), c.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                (queue1, unmatched) = ABMPBFStraverseRows(&rows, i, L, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), &nextqueue, &unMatched)?;
                { (queue, i, L, lim, lim1, nv, ne, m, mT, rowmarks, level, ass1, ass2, nextqueue, unMatched) = (rest.clone(), i, L, lim, lim1, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue1, unmatched); continue '__tco; }
            },
            _ => {
                Error::addInternalError(literal!("function ABMPBFSphase failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn ABMPBFSphase1(
    mut inStop: bool,
    mut queue: metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut lim: i32,
    mut lim1: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextqueue: metamodelica::List<i32>,
    mut unMatched: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outunMatched: metamodelica::List<i32>;
    outunMatched = (match inStop {
        true => unMatched,
        false => ABMPBFSphase(
            queue,
            i,
            L,
            lim,
            lim1,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            rowmarks.clone(),
            level.clone(),
            ass1.clone(),
            ass2.clone(),
            nextqueue,
            unMatched,
        )?,
        _ => {
            Error::addInternalError(
                literal!("function ABMPBFSphase1 failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outunMatched)
}

fn ABMPBFStraverseRows(
    mut rows: &metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarks: metamodelica::Array<i32>,
    mut level: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut queue: &metamodelica::List<i32>,
    mut unMatched: &metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut outEqnqueue: metamodelica::List<i32>;
    let mut outUnmatched: metamodelica::List<i32>;
    (outEqnqueue, outUnmatched) = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((queue.clone().reverse(), unMatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(level.clone(), r.clone(), L)?;
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    (queue1, unmatched) = ABMPBFStraverseRows(metamodelica::AsArg::as_arg(&rest), i, L, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue, &(metamodelica::cons(r.clone(), unMatched.clone())))?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    let mut rc: i32;
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                    let false = (intLt(rc, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(rowmarks.clone(), r.clone(), i)?;
                    (queue1, unmatched) = ABMPBFStraverseRows(metamodelica::AsArg::as_arg(&rest), i, L, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), &(metamodelica::cons(rc, queue.clone())), unMatched)?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut queue1: metamodelica::List<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&rowmarks.borrow(), r.clone())?).clone(); __elt}), i)) else { return Err("pattern mismatch") };
                    (queue1, unmatched) = ABMPBFStraverseRows(metamodelica::AsArg::as_arg(&rest), i, L, nv, ne, m.clone(), mT.clone(), rowmarks.clone(), level.clone(), ass1.clone(), ass2.clone(), queue, unMatched)?;
                    Ok((queue1.clone(), unmatched.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ABMPBFStraverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEqnqueue, outUnmatched))
}

fn ABMPDFS(
    mut unmatchedRows: &metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut unMatched: metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**unmatchedRows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intLt(i, ne)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut i_1: i32;
                    let mut b: bool;
                    metamodelica::arrayUpdate(colptrs.clone(), r.clone(), 0)?;
                    (i_1, b) = ABMPDFSphase(&(list![r.clone()]), i, r.clone(), nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    unmatched = List::consOnTrue(!(b), r.clone(), unMatched.clone());
                    ABMPDFS1(b, r.clone(), rest.clone(), unmatched.clone(), i_1, L, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ABMPBFS failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn ABMPDFS1(
    mut inMatched: bool,
    mut r: i32,
    mut unmatchedRows: metamodelica::List<i32>,
    mut unMatched: metamodelica::List<i32>,
    mut i: i32,
    mut L: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inMatched, &*unmatchedRows, &*unMatched);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _) => {
                    let true = (intGt(50 * L, ((unmatchedRows).len() as i32) + ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, Deref @ metamodelica::ListNode::Nil) => {
                    let false = (intGt(50 * L, ((unmatchedRows).len() as i32) + ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    ABMPDFS(&unmatchedRows, i, L, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::ListNode::Cons { head: r1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: r2, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut l: i32;
                    let false = (intGt(50 * L, ((unmatchedRows).len() as i32) + ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    let false = (intEq(L, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r1.clone())?).clone(); __elt}))) else { return Err("pattern mismatch") };
                    l = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r2.clone())?).clone(); __elt});
                    ABMPDFS(&(metamodelica::cons(r2.clone(), unmatchedRows.clone())), i, l, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::ListNode::Cons { head: r1, tail: _ }, _) => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut r2: i32;
                    let mut l: i32;
                    let false = (intGt(50 * L, ((unmatchedRows).len() as i32) + ((unMatched).len() as i32))) else { return Err("pattern mismatch") };
                    let false = (intEq(L, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r1.clone())?).clone(); __elt}))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unMatched.clone().reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r2 = metamodelica::Own::own(__pa0);
                    unmatched = metamodelica::Own::own(__pa1);
                    l = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r2)?).clone(); __elt});
                    unmatched = listAppend(unmatched.clone(), metamodelica::cons(r2, unmatchedRows.clone()));
                    ABMPDFS(&unmatchedRows, i, l, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: r1, tail: _ }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut l: i32;
                    let false = (intEq(L, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r1.clone())?).clone(); __elt}))) else { return Err("pattern mismatch") };
                    l = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r)?).clone(); __elt});
                    ABMPDFS(&unmatchedRows, i, l, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: r1, tail: _ }, _) => {
                    let mut unmatched: metamodelica::List<i32>;
                    let mut r2: i32;
                    let mut l: i32;
                    let false = (intEq(L, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r1.clone())?).clone(); __elt}))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unMatched.clone().reverse()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r2 = metamodelica::Own::own(__pa0);
                    unmatched = metamodelica::Own::own(__pa1);
                    l = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r2)?).clone(); __elt});
                    unmatched = listAppend(metamodelica::cons(r2, unmatched.clone()), unmatchedRows.clone());
                    ABMPDFS(&unmatched, i, l, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: r1, tail: _ }, _) => {
                    let true = (intEq(L, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r1.clone())?).clone(); __elt}))) else { return Err("pattern mismatch") };
                    ABMPDFS(&unmatchedRows, i, L, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone(), unMatched.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ABMPBFS1 failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn ABMPDFSphase(
    mut stack: &metamodelica::List<i32>,
    mut i: i32,
    mut r: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = (::match_deref::match_deref! { match stack {
        Deref @ metamodelica::ListNode::Nil => {
            (i, false)
        },
        _ => {
            let mut collums: metamodelica::List<i32>;
            let mut desL: i32;
            let mut i_1: i32;
            let mut b: bool;
            collums = List::select(({let __elt = (*metamodelica::index_checked(&mT.borrow(), r)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            collums = List::stripN(collums, ({let __elt = (*metamodelica::index_checked(&colptrs.borrow(), r)?).clone(); __elt}))?;
            desL = ({let __elt = (*metamodelica::index_checked(&level.borrow(), r)?).clone(); __elt}) - 2;
            (i_1, b) = ABMPDFStraverseCollums(&collums, 1, stack, r, i, desL, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
            (i_1, b)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ABMPDFSphase failed in phase ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outI, matched))
}

fn ABMPDFStraverseCollums(
    mut collums: &metamodelica::List<i32>,
    mut counter: i32,
    mut stack: &metamodelica::List<i32>,
    mut r: i32,
    mut i: i32,
    mut desL: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = 'mc: {
        let __mc_input = &**collums;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::arrayUpdate(level.clone(), r, ({let __elt = (*metamodelica::index_checked(&level.borrow(), r)?).clone(); __elt}) + 2)?;
                    metamodelica::arrayUpdate(colptrs.clone(), r, 0)?;
                    Ok((i + 1, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colptrs.clone(), r, counter)?;
                    HKDFSreasign(stack, c.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut rc: i32;
                    let mut i_1: i32;
                    let mut b: bool;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&level.borrow(), c.clone())?).clone(); __elt}), desL)) else { return Err("pattern mismatch") };
                    rc = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt});
                    let true = (intGt(rc, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(colptrs.clone(), r, counter)?;
                    (i_1, b) = ABMPDFSphase(&(metamodelica::cons(rc, stack.clone())), i, rc, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    (i_1, b) = ABMPDFStraverseCollums1(b, counter + 1, metamodelica::AsArg::as_arg(&rest), stack, r, i_1, desL, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i_1, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut i_1: i32;
                    let mut b: bool;
                    (i_1, b) = ABMPDFStraverseCollums(metamodelica::AsArg::as_arg(&rest), counter + 1, stack, r, i, desL, nv, ne, m.clone(), mT.clone(), level.clone(), colptrs.clone(), ass1.clone(), ass2.clone())?;
                    Ok((i_1, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ABMPDFSBtraverseCollums failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outI, matched))
}

fn ABMPDFStraverseCollums1(
    mut inMatched: bool,
    mut counter: i32,
    mut rows: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut r: i32,
    mut i: i32,
    mut desL: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut level: metamodelica::Array<i32>,
    mut colptrs: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(i32, bool)> {
    let mut outI: i32;
    let mut matched: bool;
    (outI, matched) = (match (inMatched, i) {
        (true, mut i_1) => (i_1, true),
        _ => {
            let mut i_1: i32;
            let mut b: bool;
            (i_1, b) = ABMPDFStraverseCollums(
                rows,
                counter,
                stack,
                r,
                i,
                desL,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                level.clone(),
                colptrs.clone(),
                ass1.clone(),
                ass2.clone(),
            )?;
            (i_1, b)
        }
    });
    Ok((outI, matched))
}

pub(crate) fn PR_FIFO_FAIR(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. } => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut l_label: metamodelica::Array<i32>;
                    let mut r_label: metamodelica::Array<i32>;
                    let mut unmatched: metamodelica::List<i32>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let true = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let true = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
                    l_label = arrayCreate(neqns, -1);
                    r_label = arrayCreate(nvars, -1);
                    unmatched = cheapmatchingalgorithm(nvars, neqns, m.clone(), mt.clone(), vec1.clone(), vec2.clone(), true)?;
                    (vec1, vec2, syst, shared, arg) = PR_FIFO_FAIR1(&unmatched, l_label.clone(), r_label.clone(), isyst.clone(), &ishared, nvars, neqns, vec1.clone(), vec2.clone(), inMatchingOptions, sssHandler, &inArg)?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), shared.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nvars: i32;
                    let mut neqns: i32;
                    let mut vec1: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    neqns = BackendDAEUtil::systemSize(&isyst)?;
                    nvars = BackendVariable::daenumVariables(&isyst);
                    let false = (intGt(nvars, 0)) else { return Err("pattern mismatch") };
                    let false = (intGt(neqns, 0)) else { return Err("pattern mismatch") };
                    vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                    syst = BackendDAEUtil::setEqSystMatching(isyst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec2.clone(), ass2: vec1.clone(), comps: metamodelica::nil() }));
                    Ok((syst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("- Matching.PR_FIFO_FAIR failed\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn PR_FIFO_FAIR1(
    mut unmatched: &metamodelica::List<i32>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (outAss1, outAss2, osyst, oshared, outArg) = 'mc: {
        let __mc_input = (&**unmatched, isyst.clone(), inArg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok((ass1.clone(), ass2.clone(), isyst.clone(), ishared.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, syst @ Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut nv_1: i32;
                    let mut ne_1: i32;
                    let mut unmatched1: metamodelica::List<i32>;
                    let mut meqns: metamodelica::List<metamodelica::List<i32>>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    let mut l_label1: metamodelica::Array<i32>;
                    let mut r_label1: metamodelica::Array<i32>;
                    let mut syst = (*syst).clone();
                    PR_Global_Relabel(l_label.clone(), r_label.clone(), nv, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone())?;
                    PR_FIFO_FAIRphase(0, unmatched, nv + ne, -1, nv, ne, m.clone(), mt.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    unmatched1 = getUnassigned(ne, ass1.clone(), metamodelica::nil())?;
                    meqns = getEqnsforIndexReduction(&unmatched1, ne, m.clone(), mt.clone(), ass1.clone(), ass2.clone(), inArg)?;
                    (unmatched1, l_label1, r_label1, nv_1, ne_1, ass1_1, ass2_1, syst, shared, arg) = PR_FIFO_FAIR2(meqns.clone(), unmatched1.clone(), &(metamodelica::nil()), l_label.clone(), r_label.clone(), syst.clone(), ishared.clone(), nv, ne, ass1.clone(), ass2.clone(), inMatchingOptions, sssHandler, inArg.clone())?;
                    (ass1_2, ass2_2, syst, shared, arg1) = PR_FIFO_FAIR1(&unmatched1, l_label1.clone(), r_label1.clone(), syst.clone(), &shared, nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, &arg)?;
                    Ok((ass1_2.clone(), ass2_2.clone(), syst.clone(), shared.clone(), arg1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (_, _, _, mapIncRowEqn, _)) => {
                    let mut unmatched1: metamodelica::List<i32>;
                    let mut eqn_str: ArcStr;
                    let mut var_str: ArcStr;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut info: SourceInfo;
                    unmatched1 = List::map1r(unmatched.clone(), &arrayGet, mapIncRowEqn.clone())?;
                    unmatched1 = List::uniqueIntN(&unmatched1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
                    eqn_str = BackendDump::dumpMarkedEqns(&isyst, unmatched1.clone())?;
                    unmatched1 = getUnassigned(nv, ass2.clone(), metamodelica::nil())?;
                    var_str = BackendDump::dumpMarkedVars(&isyst, unmatched1.clone())?;
                    source = BackendEquation::markedEquationSource(&isyst, (unmatched1).head().cloned()?)?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Error::addSourceMessage(&(Error::STRUCT_SINGULAR_SYSTEM.clone()), list![eqn_str.clone(), var_str.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAss1, outAss2, osyst, oshared, outArg))
}

fn PR_FIFO_FAIR2(
    mut meqns: metamodelica::List<metamodelica::List<i32>>,
    mut unmatched: metamodelica::List<i32>,
    mut changedEqns: &metamodelica::List<i32>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    i32,
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outunmatched: metamodelica::List<i32>;
    let mut outl_label: metamodelica::Array<i32>;
    let mut outr_label: metamodelica::Array<i32>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (
        outunmatched,
        outl_label,
        outr_label,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (unmatched, l_label.clone(), r_label.clone(), nv, ne, ass1.clone(), ass2.clone(), isyst, ishared, inArg)
        },
        (_, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut unmatched1: metamodelica::List<i32>;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut l_label1: metamodelica::Array<i32>;
            let mut r_label1: metamodelica::Array<i32>;
            (unmatched1, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns, 0, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
            ne_1 = BackendDAEUtil::systemSize(&syst)?;
            nv_1 = BackendVariable::daenumVariables(&syst);
            ass1_1 = assignmentsArrayExpand(ass1_1.clone(), ne_1, metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_1 = assignmentsArrayExpand(ass2_1.clone(), nv_1, metamodelica::arrayLength(ass2_1.clone()), -1)?;
            l_label1 = assignmentsArrayExpand(l_label.clone(), ne_1, metamodelica::arrayLength(l_label.clone()), -1)?;
            r_label1 = assignmentsArrayExpand(r_label.clone(), nv_1, metamodelica::arrayLength(r_label.clone()), -1)?;
            (unmatched1, l_label1.clone(), r_label1.clone(), nv_1, ne_1, ass1_1.clone(), ass2_1.clone(), syst, shared, arg)
        },
        (_, _) => {
            singularSystemError(meqns, 0, &isyst, &ishared, ass1.clone(), ass2.clone(), &inArg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        outunmatched,
        outl_label,
        outr_label,
        nvars,
        neqns,
        outAss1,
        outAss2,
        osyst,
        oshared,
        outArg,
    ))
}

fn PR_Global_Relabel(
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let mut queue: metamodelica::List<i32>;
    let mut max: i32;
    max = nv + ne;
    PR_Global_Relabel_init_l_label(1, ne, max, l_label.clone())?;
    queue = PR_Global_Relabel_init_r_label(1, nv, max, r_label.clone(), ass2.clone(), &(metamodelica::nil()))?;
    PR_Global_Relabel1(
        &queue,
        l_label.clone(),
        r_label.clone(),
        max,
        nv,
        ne,
        m.clone(),
        mT.clone(),
        ass1.clone(),
        ass2.clone(),
        &(metamodelica::nil()),
    )?;
    Ok(())
}

fn PR_Global_Relabel_init_l_label(
    mut i: i32,
    mut ne: i32,
    mut max: i32,
    mut l_label: metamodelica::Array<i32>,
) -> Result<()> {
    if !(intGt(i, ne)) {
        metamodelica::arrayUpdate(l_label.clone(), i, max)?;
        PR_Global_Relabel_init_l_label(i + 1, ne, max, l_label.clone())?;
    }
    Ok(())
}

fn PR_Global_Relabel_init_r_label(
    mut i: i32,
    mut nv: i32,
    mut max: i32,
    mut r_label: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inQueue: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outQueue: metamodelica::List<i32>;
    outQueue = 'mc: {
        let __mc_input = &**inQueue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(i, nv)) else { return Err("pattern mismatch") };
                    Ok(inQueue.clone().reverse())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intGt(i, nv)) else { return Err("pattern mismatch") };
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), i)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(r_label.clone(), i, 0)?;
                    Ok(PR_Global_Relabel_init_r_label(i + 1, nv, max, r_label.clone(), ass2.clone(), &(metamodelica::cons(i, inQueue.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::arrayUpdate(r_label.clone(), i, max)?;
                    Ok(PR_Global_Relabel_init_r_label(i + 1, nv, max, r_label.clone(), ass2.clone(), inQueue)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outQueue)
}

fn PR_Global_Relabel1(
    mut queue: &metamodelica::List<i32>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut max: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextqueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**queue, &**nextqueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    PR_Global_Relabel1(&(nextqueue.clone().reverse()), l_label.clone(), r_label.clone(), max, nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: r, tail: rest }, _) => {
                    let mut collums: metamodelica::List<i32>;
                    let mut queue1: metamodelica::List<i32>;
                    collums = List::select(({let __elt = (*metamodelica::index_checked(&mT.borrow(), r.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    queue1 = PR_Global_Relabel_traverseCollums(&collums, max, r.clone(), l_label.clone(), r_label.clone(), nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), nextqueue);
                    PR_Global_Relabel1(metamodelica::AsArg::as_arg(&rest), l_label.clone(), r_label.clone(), max, nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), &queue1)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn PR_Global_Relabel_traverseCollums(
    mut collums: &metamodelica::List<i32>,
    mut max: i32,
    mut r: i32,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextqueue: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outQueue: metamodelica::List<i32>;
    outQueue = 'mc: {
        let __mc_input = &**collums;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(nextqueue.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut rc: i32;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&l_label.borrow(), c.clone())?).clone(); __elt}), max)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(l_label.clone(), c.clone(), ({let __elt = (*metamodelica::index_checked(&r_label.borrow(), r)?).clone(); __elt}) + 1)?;
                    rc = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt});
                    let true = (intGt(rc, -1)) else { return Err("pattern mismatch") };
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&r_label.borrow(), rc)?).clone(); __elt}), max)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(r_label.clone(), rc, ({let __elt = (*metamodelica::index_checked(&l_label.borrow(), c.clone())?).clone(); __elt}) + 1)?;
                    Ok(PR_Global_Relabel_traverseCollums(metamodelica::AsArg::as_arg(&rest), max, r, l_label.clone(), r_label.clone(), nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), &(metamodelica::cons(rc, nextqueue.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(PR_Global_Relabel_traverseCollums(metamodelica::AsArg::as_arg(&rest), max, r, l_label.clone(), r_label.clone(), nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), nextqueue))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PR_Global_Relabel_traverseCollums failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outQueue
}

fn PR_FIFO_FAIRphase(
    mut relabels: i32,
    mut U: &metamodelica::List<i32>,
    mut max: i32,
    mut min_vertex: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut nextqueue: &metamodelica::List<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**U, &**nextqueue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    PR_FIFO_FAIRphase(relabels, nextqueue, max, min_vertex, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), &(metamodelica::nil()))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (intEq(relabels, max)) else { return Err("pattern mismatch") };
                    PR_Global_Relabel(l_label.clone(), r_label.clone(), nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
                    PR_FIFO_FAIRphase(0, U, max, min_vertex, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), nextqueue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: c, tail: rest }, _) => {
                    let mut queue: metamodelica::List<i32>;
                    let mut min_label: i32;
                    let mut rlcount: i32;
                    let mut minvertex: i32;
                    (rlcount, min_label, minvertex) = PR_FIFO_FAIRphase1(intLt(({let __elt = (*metamodelica::index_checked(&l_label.borrow(), c.clone())?).clone(); __elt}), max), relabels + 1, c.clone(), min_vertex, max, max, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone())?;
                    queue = PR_FIFO_FAIRrelabel(c.clone(), minvertex, min_label, max, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), nextqueue.clone());
                    PR_FIFO_FAIRphase(rlcount, metamodelica::AsArg::as_arg(&rest), max, minvertex, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone(), &queue)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn PR_FIFO_FAIRphase1(
    mut b: bool,
    mut relabels: i32,
    mut max_vertex: i32,
    mut min_vertec: i32,
    mut min_label: i32,
    mut max: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(i32, i32, i32)> {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = (match b {
        true => {
            let mut rel: i32;
            let mut minlab: i32;
            let mut minvert: i32;
            let mut tmp: i32;
            tmp = intMod(
                ({
                    let __elt = (*metamodelica::index_checked(&l_label.borrow(), max_vertex)?).clone();
                    __elt
                }),
                4,
            );
            (rel, minlab, minvert) = PR_FIFO_FAIRphase2(
                intEq(tmp, 1),
                relabels,
                max_vertex,
                min_vertec,
                min_label,
                max,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                l_label.clone(),
                r_label.clone(),
                ass1.clone(),
                ass2.clone(),
            )?;
            (rel, minlab, minvert)
        }
        _ => (relabels, min_label, min_vertec),
    });
    Ok((outRelabels, outMinLabels, outMinVertex))
}

fn PR_FIFO_FAIRphase2(
    mut b: bool,
    mut relabels: i32,
    mut max_vertex: i32,
    mut min_vertec: i32,
    mut min_label: i32,
    mut max: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(i32, i32, i32)> {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = (match b {
        true => {
            let mut rows: metamodelica::List<i32>;
            let mut rel: i32;
            let mut minlab: i32;
            let mut minvert: i32;
            rows = List::select(
                ({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), max_vertex)?).clone();
                    __elt
                }),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            (rel, minlab, minvert) = PR_FIFO_FAIRphase_traverseRows(
                &rows,
                relabels,
                max_vertex,
                min_vertec,
                min_label,
                max,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                l_label.clone(),
                r_label.clone(),
                ass1.clone(),
                ass2.clone(),
            );
            (rel, minlab, minvert)
        }
        _ => {
            let mut rows: metamodelica::List<i32>;
            let mut rel: i32;
            let mut minlab: i32;
            let mut minvert: i32;
            rows = List::select(
                ({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), max_vertex)?).clone();
                    __elt
                }),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            rows = rows.reverse();
            (rel, minlab, minvert) = PR_FIFO_FAIRphase_traverseRows(
                &rows,
                relabels,
                max_vertex,
                min_vertec,
                min_label,
                max,
                nv,
                ne,
                m.clone(),
                mT.clone(),
                l_label.clone(),
                r_label.clone(),
                ass1.clone(),
                ass2.clone(),
            );
            (rel, minlab, minvert)
        }
    });
    Ok((outRelabels, outMinLabels, outMinVertex))
}

fn PR_FIFO_FAIRphase_traverseRows(
    mut rows: &metamodelica::List<i32>,
    mut relabels: i32,
    mut max_vertex: i32,
    mut min_vertex: i32,
    mut min_label: i32,
    mut max: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> (i32, i32, i32) {
    let mut outRelabels: i32;
    let mut outMinLabels: i32;
    let mut outMinVertex: i32;
    (outRelabels, outMinLabels, outMinVertex) = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((relabels, min_label, min_vertex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let mut minlabel: i32;
                    let mut minvertex: i32;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&r_label.borrow(), r.clone())?).clone(); __elt}), min_label)) else { return Err("pattern mismatch") };
                    minlabel = ({let __elt = (*metamodelica::index_checked(&r_label.borrow(), r.clone())?).clone(); __elt});
                    minvertex = r.clone();
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&r_label.borrow(), minvertex)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&l_label.borrow(), max_vertex)?).clone(); __elt}) - 1)) else { return Err("pattern mismatch") };
                    Ok((relabels - 1, minlabel, minvertex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut minlabel: i32;
                    let mut minvertex: i32;
                    let mut rel: i32;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&r_label.borrow(), r.clone())?).clone(); __elt}), min_label)) else { return Err("pattern mismatch") };
                    minlabel = ({let __elt = (*metamodelica::index_checked(&r_label.borrow(), r.clone())?).clone(); __elt});
                    minvertex = r.clone();
                    let false = (intEq(({let __elt = (*metamodelica::index_checked(&r_label.borrow(), minvertex)?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&l_label.borrow(), max_vertex)?).clone(); __elt}) - 1)) else { return Err("pattern mismatch") };
                    (rel, minlabel, minvertex) = PR_FIFO_FAIRphase_traverseRows(metamodelica::AsArg::as_arg(&rest), relabels, max_vertex, minvertex, minlabel, max, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone());
                    Ok((rel, minlabel, minvertex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut minlabel: i32;
                    let mut minvertex: i32;
                    let mut rel: i32;
                    (rel, minlabel, minvertex) = PR_FIFO_FAIRphase_traverseRows(metamodelica::AsArg::as_arg(&rest), relabels, max_vertex, min_vertex, min_label, max, nv, ne, m.clone(), mT.clone(), l_label.clone(), r_label.clone(), ass1.clone(), ass2.clone());
                    Ok((rel, minlabel, minvertex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function PR_FIFO_FAIRphase_traverseRows failed"), metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outRelabels, outMinLabels, outMinVertex)
}

fn PR_FIFO_FAIRrelabel(
    mut max_vertex: i32,
    mut min_vertex: i32,
    mut min_label: i32,
    mut max: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut l_label: metamodelica::Array<i32>,
    mut r_label: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inQueue: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outQueue: metamodelica::List<i32>;
    outQueue = 'mc: {
        let __mc_input = &*inQueue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intLt(min_label, max)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), min_vertex)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(ass2.clone(), min_vertex, max_vertex)?;
                    metamodelica::arrayUpdate(ass1.clone(), max_vertex, min_vertex)?;
                    metamodelica::arrayUpdate(r_label.clone(), min_vertex, min_label + 2)?;
                    Ok(inQueue.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut next_vertex: i32;
                    let true = (intLt(min_label, max)) else { return Err("pattern mismatch") };
                    let false = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), min_vertex)?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    next_vertex = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), min_vertex)?).clone(); __elt});
                    metamodelica::arrayUpdate(ass2.clone(), min_vertex, max_vertex)?;
                    metamodelica::arrayUpdate(ass1.clone(), max_vertex, min_vertex)?;
                    metamodelica::arrayUpdate(ass1.clone(), next_vertex, -1)?;
                    metamodelica::arrayUpdate(l_label.clone(), max_vertex, min_label + 1)?;
                    metamodelica::arrayUpdate(r_label.clone(), min_vertex, min_label + 2)?;
                    Ok(metamodelica::cons(next_vertex, inQueue.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inQueue.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outQueue
}

// =============================================================================
// cheap matching implementations
//
// =============================================================================
fn cheapmatchingalgorithm(
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut intRangeUsed: bool,
) -> Result<metamodelica::List<i32>> {
    let mut outUnMatched: metamodelica::List<i32>;
    outUnMatched = cheapmatchingalgorithm1(
        Config::getCheapMatchingAlgorithm()?,
        nv,
        ne,
        m.clone(),
        mT.clone(),
        ass1.clone(),
        ass2.clone(),
        intRangeUsed,
    )?;
    Ok(outUnMatched)
}

fn cheapmatchingalgorithm1(
    mut algorithmid: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut intRangeUsed: bool,
) -> Result<metamodelica::List<i32>> {
    let mut outUnMatched: metamodelica::List<i32>;
    outUnMatched = (match (algorithmid, intRangeUsed) {
        (1, _) => cheapmatching(
            1,
            nv,
            ne,
            m.clone(),
            mT.clone(),
            ass1.clone(),
            ass2.clone(),
            &(metamodelica::nil()),
        )?,
        (3, _) => ks_rand_cheapmatching(nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?,
        (_, true) => getUnassigned(ne, ass1.clone(), metamodelica::nil())?,
        _ => metamodelica::nil(),
    });
    Ok(outUnMatched)
}

fn cheapmatching(
    mut i: i32,
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inUnMatched: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outUnMatched: metamodelica::List<i32>;
    outUnMatched = 'mc: {
        let __mc_input = &**inUnMatched;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intGt(i, ne)) else { return Err("pattern mismatch") };
                    Ok(inUnMatched.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut rows: metamodelica::List<i32>;
                    rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), i)?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    cheapmatching1(&rows, i, ass1.clone(), ass2.clone())?;
                    Ok(cheapmatching(i + 1, nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), inUnMatched)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cheapmatching(i + 1, nv, ne, m.clone(), mT.clone(), ass1.clone(), ass2.clone(), &(metamodelica::cons(i, inUnMatched.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function cheapmatching failed in equation ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Matching.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outUnMatched)
}

fn cheapmatching1(
    mut rows: &metamodelica::List<i32>,
    mut c: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: _ } => {
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(ass1.clone(), c, r.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), r.clone(), c)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    cheapmatching1(metamodelica::AsArg::as_arg(&rest), c, ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn ks_rand_cheapmatching(
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outUnMatched: metamodelica::List<i32>;
    let mut onecolums: metamodelica::List<i32>;
    let mut onerows: metamodelica::List<i32>;
    let mut col_degrees: metamodelica::Array<i32>;
    let mut row_degrees: metamodelica::Array<i32>;
    let mut randarr: metamodelica::Array<i32>;
    col_degrees = arrayCreate(ne, 0);
    row_degrees = arrayCreate(ne, 0);
    onerows = getOneRows(ne, mT.clone(), row_degrees.clone(), metamodelica::nil())?;
    onecolums = getOneRows(nv, m.clone(), col_degrees.clone(), metamodelica::nil())?;
    randarr = Array::createIntRange(ne);
    setrandArray(ne, randarr.clone())?;
    ks_rand_cheapmatching1(
        1,
        ne,
        &onecolums,
        &onerows,
        col_degrees.clone(),
        row_degrees.clone(),
        randarr.clone(),
        m.clone(),
        mT.clone(),
        ass1.clone(),
        ass2.clone(),
    )?;
    outUnMatched = getUnassigned(ne, ass1.clone(), metamodelica::nil())?;
    Ok(outUnMatched)
}

fn ks_rand_cheapmatching1(
    mut i: i32,
    mut ne: i32,
    mut onecolums: &metamodelica::List<i32>,
    mut onerows: &metamodelica::List<i32>,
    mut col_degrees: metamodelica::Array<i32>,
    mut row_degrees: metamodelica::Array<i32>,
    mut randarr: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let mut onecolums1: metamodelica::List<i32>;
    let mut onerows1: metamodelica::List<i32>;
    let mut c: i32;
    let mut b: bool;
    if intLe(i, ne) {
        ks_rand_match(
            onerows,
            onecolums,
            row_degrees.clone(),
            col_degrees.clone(),
            mT.clone(),
            m.clone(),
            ass2.clone(),
            ass1.clone(),
        )?;
        c = ({
            let __elt = (*metamodelica::index_checked(&randarr.borrow(), i)?).clone();
            __elt
        });
        b = intLt(
            ({
                let __elt = (*metamodelica::index_checked(&ass1.borrow(), c)?).clone();
                __elt
            }),
            0,
        ) && intGt(
            ({
                let __elt = (*metamodelica::index_checked(&col_degrees.borrow(), c)?).clone();
                __elt
            }),
            0,
        );
        (onecolums1, onerows1) = ks_rand_cheapmatching2(
            b,
            c,
            col_degrees.clone(),
            row_degrees.clone(),
            randarr.clone(),
            m.clone(),
            mT.clone(),
            ass1.clone(),
            ass2.clone(),
        )?;
        ks_rand_cheapmatching1(
            i + 1,
            ne,
            &onecolums1,
            &onerows1,
            col_degrees.clone(),
            row_degrees.clone(),
            randarr.clone(),
            m.clone(),
            mT.clone(),
            ass1.clone(),
            ass2.clone(),
        )?;
    }
    Ok(())
}

fn ks_rand_cheapmatching2(
    mut b: bool,
    mut c: i32,
    mut col_degrees: metamodelica::Array<i32>,
    mut row_degrees: metamodelica::Array<i32>,
    mut randarr: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let __ab_m = m.borrow();
    let __ab_mT = mT.borrow();
    let mut onecolums: metamodelica::List<i32>;
    let mut onerows: metamodelica::List<i32>;
    (onecolums, onerows) = (match b {
        true => {
            let mut clst: metamodelica::List<i32>;
            let mut rlst: metamodelica::List<i32>;
            let mut lst: metamodelica::List<i32>;
            let mut e_id: i32;
            let mut r: i32;
            e_id = ((realMod(
                System::realRand(),
                intReal(
                    ({
                        let __elt = (*metamodelica::index_checked(&col_degrees.borrow(), c)?).clone();
                        __elt
                    }),
                ),
            ))
            .0
            .floor() as i32);
            lst = List::select(
                (*metamodelica::index_checked(&__ab_m, c)?).clone(),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            (rlst, r) = ks_rand_cheapmatching3(
                e_id,
                &lst,
                row_degrees.clone(),
                c,
                ass1.clone(),
                ass2.clone(),
                &(metamodelica::nil()),
                0,
            );
            lst = List::select(
                (*metamodelica::index_checked(&__ab_mT, r)?).clone(),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            clst = ks_rand_cheapmatching4(
                &lst,
                ({
                    let __elt = (*metamodelica::index_checked(&row_degrees.borrow(), r)?).clone();
                    __elt
                }),
                col_degrees.clone(),
                ass1.clone(),
                &(metamodelica::nil()),
            );
            (clst, rlst)
        }
        _ => (metamodelica::nil(), metamodelica::nil()),
    });
    Ok((onecolums, onerows))
}

fn ks_rand_cheapmatching3(
    mut e_id: i32,
    mut rows: &metamodelica::List<i32>,
    mut row_degrees: metamodelica::Array<i32>,
    mut c: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut onerows: &metamodelica::List<i32>,
    mut inR: i32,
) -> (metamodelica::List<i32>, i32) {
    let mut outonerows: metamodelica::List<i32>;
    let mut outR: i32;
    (outonerows, outR) = 'mc: {
        let __mc_input = &**rows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((onerows.clone(), inR))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut stack: metamodelica::List<i32>;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    let true = (intEq(e_id, 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(ass1.clone(), c, r.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), r.clone(), c)?;
                    stack = ks_rand_match_degree(metamodelica::AsArg::as_arg(&rest), row_degrees.clone(), ass2.clone(), onerows);
                    Ok((stack.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut stack: metamodelica::List<i32>;
                    let mut statck1: metamodelica::List<i32>;
                    let mut r_1: i32;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(row_degrees.clone(), r.clone(), ({let __elt = (*metamodelica::index_checked(&row_degrees.borrow(), r.clone())?).clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = (*metamodelica::index_checked(&row_degrees.borrow(), r.clone())?).clone(); __elt}), 1), r.clone(), onerows.clone());
                    (statck1, r_1) = ks_rand_cheapmatching3(e_id - 1, metamodelica::AsArg::as_arg(&rest), row_degrees.clone(), c, ass1.clone(), ass2.clone(), &stack, r.clone());
                    Ok((statck1.clone(), r_1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                    let mut statck1: metamodelica::List<i32>;
                    let mut r_1: i32;
                    (statck1, r_1) = ks_rand_cheapmatching3(e_id - 1, metamodelica::AsArg::as_arg(&rest), row_degrees.clone(), c, ass1.clone(), ass2.clone(), onerows, r.clone());
                    Ok((statck1.clone(), r_1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outonerows, outR)
}

fn ks_rand_cheapmatching4(
    mut cols: &metamodelica::List<i32>,
    mut count: i32,
    mut col_degrees: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut inStack: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outStack: metamodelica::List<i32>;
    outStack = 'mc: {
        let __mc_input = &**cols;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inStack.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (intGt(count, 0)) else { return Err("pattern mismatch") };
                    Ok(inStack.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
                    let mut stack: metamodelica::List<i32>;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), c.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(col_degrees.clone(), c.clone(), ({let __elt = (*metamodelica::index_checked(&col_degrees.borrow(), c.clone())?).clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = (*metamodelica::index_checked(&col_degrees.borrow(), c.clone())?).clone(); __elt}), 1), c.clone(), inStack.clone());
                    Ok(ks_rand_cheapmatching4(metamodelica::AsArg::as_arg(&rest), count - 1, col_degrees.clone(), ass1.clone(), &stack))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_cheapmatching4(metamodelica::AsArg::as_arg(&rest), count, col_degrees.clone(), ass1.clone(), inStack))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStack
}

fn getOneRows(
    mut n: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut degrees: metamodelica::Array<i32>,
    mut inOneRows: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        match n {
            0 => return Ok(inOneRows.reverse()),
            _ => {
                let mut lst: metamodelica::List<i32>;
                let mut onerows: metamodelica::List<i32>;
                let mut l: i32;
                lst = List::select(
                    ({
                        let __elt = (*metamodelica::index_checked(&m.borrow(), n)?).clone();
                        __elt
                    }),
                    (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
                )?;
                l = ((lst).len() as i32);
                metamodelica::arrayUpdate(degrees.clone(), n, l)?;
                onerows = List::consOnTrue(intEq(l, 1), n, inOneRows);
                {
                    (n, m, degrees, inOneRows) = (n - 1, m.clone(), degrees.clone(), onerows);
                    continue '__tco;
                }
            }
        }
    }
}

fn setrandArray(mut n: i32, mut randarr: metamodelica::Array<i32>) -> Result<()> {
    let () = (match n {
        0 => (),
        _ => {
            let mut z: i32;
            let mut tmp: i32;
            z = ((realMod(System::realRand(), intReal(n))).0.floor() as i32) + 1;
            tmp = ({
                let __elt = (*metamodelica::index_checked(&randarr.borrow(), n)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(
                randarr.clone(),
                n,
                ({
                    let __elt = (*metamodelica::index_checked(&randarr.borrow(), z)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(randarr.clone(), z, tmp)?;
            setrandArray(n - 1, randarr.clone())?;
            ()
        }
    });
    Ok(())
}

fn ks_rand_match(
    mut stack1: &metamodelica::List<i32>,
    mut stack2: &metamodelica::List<i32>,
    mut degrees1: metamodelica::Array<i32>,
    mut degrees2: metamodelica::Array<i32>,
    mut m1: metamodelica::Array<metamodelica::List<i32>>,
    mut m2: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**stack1, &**stack2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut lst: metamodelica::List<i32>;
                    let mut stack: metamodelica::List<i32>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&degrees1.borrow(), e.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), e.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    lst = List::select(({let __elt = (*metamodelica::index_checked(&m1.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), &lst, metamodelica::AsArg::as_arg(&rest), degrees1.clone(), degrees2.clone(), m2.clone(), ass1.clone(), ass2.clone());
                    ks_rand_match(&stack, &(metamodelica::nil()), degrees1.clone(), degrees2.clone(), m1.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    ks_rand_match(metamodelica::AsArg::as_arg(&rest), &(metamodelica::nil()), degrees1.clone(), degrees2.clone(), m1.clone(), m2.clone(), ass1.clone(), ass2.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }) => {
                    let mut lst: metamodelica::List<i32>;
                    let mut stack: metamodelica::List<i32>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&degrees2.borrow(), e.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    lst = List::select(({let __elt = (*metamodelica::index_checked(&m2.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), &lst, metamodelica::AsArg::as_arg(&rest), degrees2.clone(), degrees1.clone(), m1.clone(), ass2.clone(), ass1.clone());
                    ks_rand_match(&stack, &(metamodelica::nil()), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    ks_rand_match(metamodelica::AsArg::as_arg(&rest), &(metamodelica::nil()), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, _) => {
                    let mut lst: metamodelica::List<i32>;
                    let mut stack: metamodelica::List<i32>;
                    let true = (intEq(({let __elt = (*metamodelica::index_checked(&degrees1.borrow(), e.clone())?).clone(); __elt}), 1)) else { return Err("pattern mismatch") };
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), e.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    lst = List::select(({let __elt = (*metamodelica::index_checked(&m1.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    stack = ks_rand_match1(e.clone(), &lst, metamodelica::AsArg::as_arg(&rest), degrees1.clone(), degrees2.clone(), m2.clone(), ass1.clone(), ass2.clone());
                    ks_rand_match(stack2, &stack, degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    ks_rand_match(stack2, metamodelica::AsArg::as_arg(&rest), degrees2.clone(), degrees1.clone(), m2.clone(), m1.clone(), ass2.clone(), ass1.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn ks_rand_match1(
    mut i: i32,
    mut entries: &metamodelica::List<i32>,
    mut stack: &metamodelica::List<i32>,
    mut degrees1: metamodelica::Array<i32>,
    mut degrees2: metamodelica::Array<i32>,
    mut adjacency: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> metamodelica::List<i32> {
    let mut outStack: metamodelica::List<i32>;
    outStack = 'mc: {
        let __mc_input = &**entries;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(stack.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
                    let mut lst: metamodelica::List<i32>;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), e.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    lst = List::select(({let __elt = (*metamodelica::index_checked(&adjacency.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                    metamodelica::arrayUpdate(ass1.clone(), i, e.clone())?;
                    metamodelica::arrayUpdate(ass2.clone(), e.clone(), i)?;
                    Ok(ks_rand_match_degree(&lst, degrees1.clone(), ass1.clone(), stack))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_match1(i, metamodelica::AsArg::as_arg(&rest), stack, degrees1.clone(), degrees2.clone(), adjacency.clone(), ass1.clone(), ass2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStack
}

fn ks_rand_match_degree(
    mut entries: &metamodelica::List<i32>,
    mut degrees: metamodelica::Array<i32>,
    mut ass: metamodelica::Array<i32>,
    mut inStack: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outStack: metamodelica::List<i32>;
    outStack = 'mc: {
        let __mc_input = &**entries;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inStack.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                    let mut stack: metamodelica::List<i32>;
                    let true = (intLt(({let __elt = (*metamodelica::index_checked(&ass.borrow(), e.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(degrees.clone(), e.clone(), ({let __elt = (*metamodelica::index_checked(&degrees.borrow(), e.clone())?).clone(); __elt}) - 1)?;
                    stack = List::consOnTrue(intEq(({let __elt = (*metamodelica::index_checked(&degrees.borrow(), e.clone())?).clone(); __elt}), 1), e.clone(), inStack.clone());
                    Ok(ks_rand_match_degree(metamodelica::AsArg::as_arg(&rest), degrees.clone(), ass.clone(), &stack))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(ks_rand_match_degree(metamodelica::AsArg::as_arg(&rest), degrees.clone(), ass.clone(), inStack))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStack
}

// =============================================================================
// C-Implementation Stuff from
// Kamer Kaya, Johannes Langguth and Bora Ucar
// see: http://bmi.osu.edu/~kamer/index.html
// =============================================================================
pub(crate) fn DFSBExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                1,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.DFSBExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn BFSBExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                2,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.BFSBExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn MC21AExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                3,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.MC21AExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn PFExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                4,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.PFExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn PFPlusExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut nvars: i32;
    let mut neqns: i32;
    neqns = BackendDAEUtil::systemSize(&isyst)?;
    nvars = BackendVariable::daenumVariables(&isyst);
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !(intGt(nvars, 0) && intGt(neqns, 0)) {
                return Err("guard");
            }
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                5,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if !(!(intGt(nvars, 0)) && !(intGt(neqns, 0))) {
                return Err("guard");
            }
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.PFPlusExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn HKExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                6,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.HKExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn HKDWExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                7,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.HKDWExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn ABMPExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                8,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.ABMPExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

pub(crate) fn PR_FIFO_FAIRExternal(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut clearMatching: bool,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (osyst, oshared, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let true = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2) = getAssignment(clearMatching, nvars, neqns, &isyst);
            let true = (if (!(clearMatching)) {
                BackendDAEEXT::setAssignment(neqns, nvars, vec1.clone(), vec2.clone())
            } else {
                true
            }) else {
                return Err("pattern mismatch");
            };
            (vec1, vec2, syst, shared, arg) = matchingExternal(
                metamodelica::nil(),
                false,
                10,
                Config::getCheapMatchingAlgorithm()?,
                if (clearMatching) { 1 } else { 0 },
                isyst.clone(),
                ishared.clone(),
                nvars,
                neqns,
                vec1.clone(),
                vec2.clone(),
                inMatchingOptions,
                sssHandler,
                inArg.clone(),
            )?;
            syst = BackendDAEUtil::setEqSystMatching(
                syst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), shared.clone(), arg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nvars: i32;
            let mut neqns: i32;
            let mut vec1: metamodelica::Array<i32>;
            let mut vec2: metamodelica::Array<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            neqns = BackendDAEUtil::systemSize(&isyst)?;
            nvars = BackendVariable::daenumVariables(&isyst);
            let false = (intGt(nvars, 0)) else {
                return Err("pattern mismatch");
            };
            let false = (intGt(neqns, 0)) else {
                return Err("pattern mismatch");
            };
            vec1 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            vec2 = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
            syst = BackendDAEUtil::setEqSystMatching(
                isyst.clone(),
                metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: vec2.clone(),
                    ass2: vec1.clone(),
                    comps: metamodelica::nil(),
                }),
            );
            Ok((syst.clone(), ishared.clone(), inArg.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("- Matching.PR_FIFO_FAIRExternal failed\n"))?;
            }
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, outArg))
}

fn matchingExternal<'__b>(
    mut meqns: metamodelica::List<metamodelica::List<i32>>,
    mut internalCall: bool,
    mut algIndx: i32,
    mut cheapMatching: i32,
    mut clearMatching: i32,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &'__b dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    '__tco: loop {
        Error::checkCancel()?;
        ({
            let mut changed: bool = false;
            ::match_deref::match_deref! { match &((meqns.clone(), internalCall, isyst.clone(), inMatchingOptions)) {
                (Deref @ metamodelica::ListNode::Nil, true, _, _) => {
                    return Ok((ass1.clone(), ass2.clone(), isyst, ishared, inArg))
                },
                (Deref @ metamodelica::ListNode::Nil, false, Deref @ BackendDAE::EqSystem { m: Some(m), mT: Some(mt), .. }, _) => {
                    let mut m1: metamodelica::Array<metamodelica::List<i32>>;
                    let mut m1t: metamodelica::Array<metamodelica::List<i32>>;
                    let mut unmatched_eqs: metamodelica::List<i32>;
                    let mut meqns1: metamodelica::List<metamodelica::List<i32>>;
                    let mut comps: metamodelica::List<metamodelica::List<i32>>;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut m = (*m).clone();
                    let mut mt = (*mt).clone();
                    matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
                    BackendDAEEXT::matching(nv, ne, algIndx, cheapMatching, metamodelica::OrderedFloat(1.0_f64), clearMatching);
                    BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
                    (ass1_1, ass2_1) = (ass1.clone(), ass2.clone());
                    syst = isyst.clone();
                    if !(Flags::getConfigBool(Flags::NO_ASSC.clone())?) && BackendDAEUtil::hasIndexTypeSolvableAndUnprocessedScalar(&syst) && BackendDAEUtil::doIndexReduction(inMatchingOptions) {
                        syst = BackendDAEUtil::setAnalyticalToStructuralProcessed(syst, true)?;
                        (_, m1, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(isyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(&ishared))?;
                        comps = Sorting::Tarjan(m1.clone(), ass2_1.clone(), metamodelica::arrayLength(ass2_1.clone()))?;
                        for mut comp in &*comps {
                            (ass1_1, ass2_1, syst, changed) = BackendDAEUtil::analyticalToStructuralSingularity(metamodelica::AsArg::as_arg(&comp), ass1_1.clone(), ass2_1.clone(), syst, changed, false)?;
                        }
                        if changed {
                            BackendDAEEXT::setAssignment(nv, ne, ass1_1.clone(), ass2_1.clone());
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
                                Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            m = metamodelica::Own::own(__pa0);
                            mt = metamodelica::Own::own(__pa1);
                            matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
                            BackendDAEEXT::matching(nv, ne, algIndx, cheapMatching, metamodelica::OrderedFloat(1.0_f64), 0);
                            BackendDAEEXT::getAssignment(ass1_1.clone(), ass2_1.clone())?;
                        }
                    }
                    unmatched_eqs = getUnassigned(ne, ass1_1.clone(), metamodelica::nil())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? && Flags::isSet(Flags::GRAPHML.clone())? {
                        BackendDump::dumpBipartiteGraphEqSystem(isyst, &ishared, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BeforMatching_")); __mm_s.push_str(&*intString(metamodelica::arrayLength(m.clone()))); __mm_s.push_str(&*literal!("_unmatched ")); __mm_s.push_str(&*intString(((unmatched_eqs).len() as i32))); ArcStr::from(__mm_s) }))?;
                    }
                    if Flags::isSet(Flags::BLT_DUMP.clone())? && !((unmatched_eqs).is_empty()) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("unmatched equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(unmatched_eqs.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    m1 = metamodelica::arrayFromVec(m.clone().borrow().clone());
                    m1t = metamodelica::arrayFromVec(mt.clone().borrow().clone());
                    (m1, m1t) = removeEdgesForNoDerivativeFunctionInputs(m1.clone(), m1t.clone(), &syst, &ishared)?;
                    (m1, m1t) = removeEdgesToDiscreteEquations(m1.clone(), m1t.clone(), &syst, &ishared)?;
                    meqns1 = getEqnsforIndexReduction(&unmatched_eqs, ne, m1.clone(), m1t.clone(), ass1_1.clone(), ass2_1.clone(), &inArg)?;
                    if !((meqns1).is_empty()) {
                        (syst, meqns1, ass1_1, ass2_1) = sanityCheckArtificialStates(syst, ishared.clone(), nv, ne, meqns1, ass1_1.clone(), ass2_1.clone(), algIndx, cheapMatching, clearMatching, &inArg)?;
                    } else {
                        (syst, meqns1, ass1_1, ass2_1) = (syst, meqns1, ass1_1.clone(), ass2_1.clone());
                    }
                    if Flags::isSet(Flags::BLT_DUMP.clone())? && !((meqns1).is_empty()) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Index Reduction neccessary!\n")); __mm_s.push_str(&*literal!("MSS subsets:\n ")); __mm_s.push_str(&*stringDelimitList(List::map(meqns1.clone(), &Util::intLstString)?, literal!("\n "))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    { (meqns, internalCall, algIndx, cheapMatching, clearMatching, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (meqns1, true, algIndx, -1, 0, syst, ishared, nv, ne, ass1_1.clone(), ass2_1.clone(), inMatchingOptions, sssHandler, inArg); continue '__tco; }
                },
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _, _, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
                    let mut nv_1: i32;
                    let mut ne_1: i32;
                    let mut memsize: i32;
                    let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut arg1: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut ass1_1: metamodelica::Array<i32>;
                    let mut ass1_2: metamodelica::Array<i32>;
                    let mut ass1_3: metamodelica::Array<i32>;
                    let mut ass2_1: metamodelica::Array<i32>;
                    let mut ass2_2: metamodelica::Array<i32>;
                    let mut ass2_3: metamodelica::Array<i32>;
                    memsize = metamodelica::arrayLength(ass1.clone());
                    (_, _, syst, shared, ass2_1, ass1_1, arg) = sssHandler(meqns, 0, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
                    ne_1 = BackendDAEUtil::systemSize(&syst)?;
                    nv_1 = BackendVariable::daenumVariables(&syst);
                    ass1_2 = assignmentsArrayExpand(ass1_1.clone(), ne_1, memsize, -1)?;
                    ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nv_1, memsize, -1)?;
                    let true = (BackendDAEEXT::setAssignment(ne_1, nv_1, ass1_2.clone(), ass2_2.clone())) else { return Err("pattern mismatch") };
                    { (meqns, internalCall, algIndx, cheapMatching, clearMatching, isyst, ishared, nv, ne, ass1, ass2, inMatchingOptions, sssHandler, inArg) = (metamodelica::nil(), false, algIndx, cheapMatching, clearMatching, syst, shared, nv_1, ne_1, ass1_2.clone(), ass2_2.clone(), inMatchingOptions, sssHandler, arg); continue '__tco; }
                },
                _ => {
                    singularSystemError(meqns, 0, &isyst, &ishared, ass1.clone(), ass2.clone(), &inArg)?;
                    return Ok(return Err("fail"))
                },
                _ => return Err("match: no arm matched"),
            } }
        })
    }
}

fn sanityCheckArtificialStates(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut eqns: metamodelica::List<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut algIndx: i32,
    mut cheapMatching: i32,
    mut clearMatching: i32,
    mut arg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut eqns: metamodelica::List<metamodelica::List<i32>> = eqns;
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut eqns_1: metamodelica::List<metamodelica::List<i32>>;
    let mut unassignedStates: metamodelica::List<metamodelica::List<i32>>;
    let mut flat_unassignedStates: metamodelica::List<i32>;
    let mut flat_eqns: metamodelica::List<i32>;
    let mut unmatched1: metamodelica::List<i32>;
    let mut scalarToArrayMap: metamodelica::Array<i32>;
    let mut artificialStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut undiffable_artificial: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut equations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut eqn_forms: metamodelica::List<(
        metamodelica::Ref<BackendDAE::Equation>,
        Option<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    )> = metamodelica::nil();
    let mut residual_exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut opt_exps: Option<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut state_array: metamodelica::Array<metamodelica::Ref<BackendDAE::Var>>;
    let mut failed_eqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut state_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, i32>>;
    let mut eqn_states: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
    let mut visited_states: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        13,
    );
    let mut visited_eqns: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        13,
    );
    let mut arrayIdx: i32;
    let mut n_states: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut form_eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut residualExp: metamodelica::Ref<DAE::Exp>;
    let mut diffExp: metamodelica::Ref<DAE::Exp>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut m1: metamodelica::Array<metamodelica::List<i32>>;
    let mut m1t: metamodelica::Array<metamodelica::List<i32>>;
    let mut msg: ArcStr;
    if let Ok((__pa0, __pa1, _, _)) = IndexReduction::minimalStructurallySingularSystem(
        eqns.clone(),
        syst.clone(),
        shared.clone(),
        ass2.clone(),
        ass1.clone(),
        arg,
    ) {
        eqns_1 = metamodelica::Own::own(__pa0);
        unassignedStates = metamodelica::Own::own(__pa1);
    } else {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            singularSystemError(eqns.clone(), 0, &syst, &shared, ass1.clone(), ass2.clone(), arg)?;
        }
        return Err("fail");
    }
    flat_unassignedStates = List::flatten(unassignedStates)?;
    for mut state in &*flat_unassignedStates {
        if UnorderedSet::add(state.clone(), visited_states.clone())? {
            var = BackendVariable::getVarAt(&syst.orderedVars, state.clone())?;
            if BackendVariable::isArtificialState(&var) {
                artificialStates = metamodelica::cons(var, artificialStates);
            }
        }
    }
    if (artificialStates).is_empty() {
        return Ok((syst, eqns, ass1, ass2));
    }
    flat_eqns = List::flatten(eqns_1)?;
    let __pa2 = ::match_deref::match_deref! { match &(syst.mapping.clone()) {
        Some((_, __pa2, _, _, _)) => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    scalarToArrayMap = metamodelica::Own::own(__pa2);
    for mut scalar_eqn in &*flat_eqns {
        if '__try3: {
            arrayIdx = ({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&scalarToArrayMap.borrow(), scalar_eqn.clone()), '__try3)).clone(); __elt});
            if unwrap_break_err!(UnorderedSet::add(arrayIdx, visited_eqns.clone()), '__try3) {
                equations = metamodelica::cons(unwrap_break_err!(BackendEquation::get(syst.orderedEqs.clone(), arrayIdx), '__try3), equations.clone());
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    state_array = metamodelica::arrayFromVec(artificialStates.into_iter().cloned().collect());
    n_states = metamodelica::arrayLength(state_array.clone());
    failed_eqns = arrayCreate(n_states, metamodelica::nil());
    state_map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::hashComponentRef(&__a0)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        Util::nextPrime(n_states),
    );
    for mut i in 1..=n_states {
        UnorderedMap::add(
            BackendVariable::varCref(
                &({
                    let __elt = (*metamodelica::index_checked(&state_array.borrow(), i)?).clone();
                    __elt
                }),
            ),
            i,
            state_map.clone(),
        )?;
    }
    for mut eqn in &*equations {
        Error::checkCancel()?;
        match '__try4: {
            residual_exps = metamodelica::nil();
            for mut res in &*unwrap_break_err!(BackendEquation::equationToScalarResidualForm(eqn.clone(), &shared.functionTree), '__try4)
            {
                let __pa5 = ::match_deref::match_deref! { match &(res.clone()) {
                    Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa5, .. } => __pa5.clone(),
                    _ => break '__try4 Err::<_, _>("pattern mismatch"),
                } };
                residualExp = metamodelica::Own::own(__pa5);
                residual_exps = metamodelica::cons(residualExp.clone(), residual_exps.clone());
            }
            eqn_forms = metamodelica::cons((eqn.clone(), Some(residual_exps.clone().reverse())), eqn_forms.clone());
            Ok::<_, &'static str>((eqn_forms.clone(),))
        } {
            Ok((__try4_o0,)) => {
                eqn_forms = __try4_o0;
            }
            Err(_) => {
                eqn_forms = metamodelica::cons((eqn.clone(), None), eqn_forms.clone());
            }
        }
    }
    for mut form in &*eqn_forms.reverse() {
        (form_eqn, opt_exps) = form.clone();
        if (opt_exps).is_none() {
            for mut i in 1..=n_states {
                {
                    let __cell6 = metamodelica::cons(
                        form_eqn.clone(),
                        ({
                            let __elt = (*metamodelica::index_checked(&failed_eqns.borrow(), i)?).clone();
                            __elt
                        }),
                    );
                    let __idx6 = i;
                    *metamodelica::index_mut_checked(&mut failed_eqns.clone().borrow_mut(), __idx6)? = __cell6;
                }
            }
        } else {
            let __pa7 = ::match_deref::match_deref! { match &(opt_exps) {
                Some(__pa7) => __pa7.clone(),
                _ => return Err("pattern mismatch"),
            } };
            residual_exps = metamodelica::Own::own(__pa7);
            eqn_states = UnorderedSet::new(
                std::sync::Arc::new(fnptr!(Util::id, _)),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                13,
            );
            for mut res_exp in &*residual_exps {
                Expression::traverseExpTopDown(
                    res_exp.clone(),
                    &({
                        let __pe_b2 = state_map.clone();
                        let __pe_b3 = eqn_states.clone();
                        move |__pe_a0, __pe_a1| {
                            collectArtificialStates(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                        }
                    }),
                    false,
                )?;
            }
            for mut i in &*UnorderedSet::toList(eqn_states) {
                Error::checkCancel()?;
                cr = BackendVariable::varCref(
                    &({
                        let __elt = (*metamodelica::index_checked(&state_array.borrow(), i.clone())?).clone();
                        __elt
                    }),
                );
                if '__try8: {
                    for mut res_exp in &*residual_exps {
                        if unwrap_break_err!(Expression::expHasCref(res_exp.clone(), cr.clone()), '__try8) {
                            (diffExp, _, _) = unwrap_break_err!(Inline::forceInlineExp(res_exp.clone(), (Some(shared.functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), &Ceval::cevalSimpleWithFunctionTreeReturnExp), '__try8);
                            diffExp = unwrap_break_err!(Expression::replaceDerOpInExp(diffExp.clone()), '__try8);
                            unwrap_break_err!(Differentiate::differentiateExpSolve(diffExp.clone(), cr.clone(), Some(shared.functionTree.clone())), '__try8);
                            let false = (unwrap_break_err!(Expression::expHasCrefInSmoothZero(diffExp.clone(), cr.clone()), '__try8)) else { break '__try8 Err::<_, _>("pattern mismatch") };
                        }
                    }
                    Ok::<(), &'static str>(())
                }.is_err() {
                    {
                        let __cell9 = metamodelica::cons(form_eqn.clone(), ({let __elt = (*metamodelica::index_checked(&failed_eqns.borrow(), i.clone())?).clone(); __elt}));
                        let __idx9 = i.clone();
                        *metamodelica::index_mut_checked(&mut failed_eqns.clone().borrow_mut(), __idx9)? = __cell9;
                    }
                }
            }
        }
    }
    for mut i in 1..=n_states {
        if !(({
            let __elt = (*metamodelica::index_checked(&failed_eqns.borrow(), i)?).clone();
            __elt
        })
        .is_empty())
        {
            var = ({
                let __elt = (*metamodelica::index_checked(&state_array.borrow(), i)?).clone();
                __elt
            });
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                let __range10 = &*({
                    let __elt = (*metamodelica::index_checked(&failed_eqns.borrow(), i)?).clone();
                    __elt
                })
                .reverse();
                for mut eqn in __range10 {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("### The Equation ### \n"));
                        __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eqn))?);
                        __mm_s.push_str(&*literal!(
                            "\n\n--- could not be differentiated for artificial variable ---\n "
                        ));
                        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
                        __mm_s.push_str(&*literal!(".\n\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            undiffable_artificial = metamodelica::cons(
                BackendVariable::setVarKind(var, openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?,
                undiffable_artificial,
            );
        }
    }
    if !((undiffable_artificial).is_empty()) {
        assign_field!(syst.orderedVars = BackendVariable::addVars(&undiffable_artificial, syst.orderedVars.clone())?);
        (syst, _, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
            syst,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(shared.functionTree.clone()),
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        if (syst.m).is_some() && (syst.mT).is_some() {
            let __pa11 = ::match_deref::match_deref! { match &(syst.m.clone()) {
                Some(__pa11) => __pa11.clone(),
                _ => return Err("pattern mismatch"),
            } };
            m = metamodelica::Own::own(__pa11);
            let __pa12 = ::match_deref::match_deref! { match &(syst.mT.clone()) {
                Some(__pa12) => __pa12.clone(),
                _ => return Err("pattern mismatch"),
            } };
            mt = metamodelica::Own::own(__pa12);
            matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
            BackendDAEEXT::matching(
                nv,
                ne,
                algIndx,
                cheapMatching,
                metamodelica::OrderedFloat(1.0_f64),
                clearMatching,
            );
            BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
            unmatched1 = getUnassigned(ne, ass1.clone(), metamodelica::nil())?;
            m1 = metamodelica::arrayFromVec(m.clone().borrow().clone());
            m1t = metamodelica::arrayFromVec(mt.clone().borrow().clone());
            (m1, m1t) = removeEdgesForNoDerivativeFunctionInputs(m1.clone(), m1t.clone(), &syst, &shared)?;
            (m1, m1t) = removeEdgesToDiscreteEquations(m1.clone(), m1t.clone(), &syst, &shared)?;
            eqns = getEqnsforIndexReduction(
                &unmatched1,
                ne,
                m1.clone(),
                m1t.clone(),
                ass1.clone(),
                ass2.clone(),
                arg,
            )?;
        }
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "----------------------------- INFO -----------------------------\n"
                ));
                __mm_s.push_str(&*literal!(
                    " Artificial states are those which do not naturally appear\n"
                ));
                __mm_s.push_str(&*literal!(
                    " differentiated in the system of DAEs, but have been forced\n"
                ));
                __mm_s.push_str(&*literal!(
                    " to be states with 'StateSelect.always' or 'StateSelect.prefer'.\n"
                ));
                __mm_s.push_str(&*literal!(
                    " The ones mentioned above will be treated as if they had \n"
                ));
                __mm_s.push_str(&*literal!("'StateSelect.default'.\n"));
                __mm_s.push_str(&*literal!(
                    "----------------------------------------------------------------\n\n"
                ));
                ArcStr::from(__mm_s)
            });
        }
        msg = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BackendDump::varListStringShort(
                &undiffable_artificial,
                &(literal!("They will be treated as if they had stateSelect=StateSelect.default")),
            )?);
            __mm_s.push_str(&*literal!("Please use -d=bltdump for more information.\n"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(Error::STATE_STATESELECT_PREFER_REVERT.clone(), list![msg])?;
    }
    Ok((syst, eqns, ass1, ass2))
}

fn collectArtificialStates(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut arg: bool,
    mut stateMap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, i32>>,
    mut states: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = exp.clone();
    let mut cont: bool;
    let mut outArg: bool = arg;
    let mut index: i32;
    cont = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => false,
        Deref @ DAE::Exp::CREF { componentRef: __exp_componentRef, .. } => {
            let () = (match UnorderedMap::get(__exp_componentRef.clone(), stateMap)? {
        Some(mut __esc_index) => {
            index = __esc_index.clone();
            UnorderedSet::add(index, states)?;
            ()
        },
        _ => (),
    });
            true
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outArg))
}

fn removeEdgesToDiscreteEquations(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut sys: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut mOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut mtOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut isDiscrete: bool;
    let mut idx: i32;
    let mut idx2: i32;
    let mut size: i32;
    let mut varIdx: i32 = 0;
    let mut varIdxs: metamodelica::List<i32>;
    let mut row: metamodelica::List<i32>;
    let mut eqIdxs: metamodelica::List<i32>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqIdxArray: metamodelica::Array<metamodelica::List<i32>>;
    vars = sys.orderedVars.clone();
    eqs = sys.orderedEqs.clone();
    idx = 1;
    idx2 = 0;
    eqIdxArray = arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), metamodelica::nil());
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        size = BackendEquation::equationSize(&(BackendEquation::get(eqs.clone(), idx)?))?;
        eqIdxs = List::map1(List::intRange(size), &fnptr!(intAdd, i32, i32), idx2)?;
        metamodelica::arrayUpdate(eqIdxArray.clone(), idx, eqIdxs)?;
        idx = idx + 1;
        idx2 = size + idx2;
    }
    idx = 1;
    for mut eq in &*BackendEquation::equationList(eqs)? {
        Error::checkCancel()?;
        isDiscrete = BackendEquation::isWhenEquationOrDiscreteAlgorithm(metamodelica::AsArg::as_arg(&eq), &vars);
        if isDiscrete {
            varLst = BackendEquation::equationVars(eq.clone(), vars.clone())?;
            varIdxs = BackendVariable::getVarIndexFromVars(&varLst, &vars);
            eqIdxs = ({
                let __elt = (*metamodelica::index_checked(&eqIdxArray.borrow(), idx)?).clone();
                __elt
            });
            for mut e in &*eqIdxs {
                row = ({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone();
                    __elt
                });
                row = UnorderedSet::difference_list(
                    row,
                    varIdxs.clone(),
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?;
                metamodelica::arrayUpdate(m.clone(), e.clone(), row)?;
            }
            for mut varIdx in &*varIdxs {
                let mut varIdx = varIdx.clone();
                row = ({
                    let __elt = (*metamodelica::index_checked(&mt.borrow(), varIdx)?).clone();
                    __elt
                });
                row = UnorderedSet::difference_list(
                    row,
                    eqIdxs.clone(),
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?;
                metamodelica::arrayUpdate(mt.clone(), varIdx, row)?;
            }
        }
        idx = idx + 1;
    }
    mOut = m.clone();
    mtOut = mt.clone();
    Ok((mOut, mtOut))
}

fn removeEdgesForNoDerivativeFunctionInputs(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut sys: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut mOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut mtOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut hasNoDerAnno: bool;
    let mut idx: i32;
    let mut varIdx: i32 = 0;
    let mut varIdxs: metamodelica::List<i32>;
    let mut row: metamodelica::List<i32>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut noDerInputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    vars = sys.orderedVars.clone();
    eqs = sys.orderedEqs.clone();
    functionTree = shared.functionTree.clone();
    idx = 1;
    for mut eq in &*BackendEquation::equationList(eqs)? {
        Error::checkCancel()?;
        (hasNoDerAnno, noDerInputs) = BackendDAEUtil::isFuncCallWithNoDerAnnotation(eq.clone(), functionTree.clone())?;
        if hasNoDerAnno {
            (_, varIdxs) = BackendVariable::getVarLst(&noDerInputs, &vars);
            row = ({
                let __elt = (*metamodelica::index_checked(&m.borrow(), idx)?).clone();
                __elt
            });
            row = UnorderedSet::difference_list(
                row,
                varIdxs.clone(),
                std::sync::Arc::new(fnptr!(Util::id, _)),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            metamodelica::arrayUpdate(m.clone(), idx, row)?;
            for mut varIdx in &*varIdxs {
                let mut varIdx = varIdx.clone();
                row = metamodelica::arrayGet(mt.clone(), varIdx)?;
                (row, _) = List::deleteMemberOnTrue(idx, row, &fnptr!(intEq, i32, i32))?;
                metamodelica::arrayUpdate(mt.clone(), varIdx, row)?;
            }
        }
        idx = idx + 1;
    }
    mOut = m.clone();
    mtOut = mt.clone();
    Ok((mOut, mtOut))
}

fn countadjacencyMatrixEntries(mut n: i32, mut m: metamodelica::Array<metamodelica::List<i32>>) -> Result<i32> {
    let __ab_m = m.borrow();
    let mut outCount: i32 = 0;
    for mut i in 1..=n {
        for mut e in &*(*metamodelica::index_checked(&__ab_m, i)?).clone() {
            if intGt(e.clone(), 0) {
                outCount = outCount + 1;
            }
        }
    }
    Ok(outCount)
}

pub fn matchingExternalsetAdjacencyMatrix(
    mut nv: i32,
    mut ne: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut nz: i32;
    nz = countadjacencyMatrixEntries(ne, m.clone())?;
    BackendDAEEXT::setAdjacencyMatrix(nv, ne, nz, m.clone());
    Ok(())
}

// =============================================================================
// Util Functions
//
// =============================================================================
pub(crate) fn reachableEquations(
    mut eqn: i32,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_ass2 = ass2.borrow();
    let __ab_mT = mT.borrow();
    let mut outEqNodes: metamodelica::List<i32>;
    let mut var: i32;
    var = (*metamodelica::index_checked(&__ab_ass2, eqn)?).clone();
    outEqNodes = if (var > 0) {
        ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut e in ((*metamodelica::index_checked(&__ab_mT, var)?).clone())
                .into_iter()
                .cloned()
            {
                if !(e.clone() > 0 && e.clone() != eqn) {
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
    Ok(outEqNodes)
}

pub(crate) fn incomingEquations(
    mut eqn: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_ass1 = ass1.borrow();
    let __ab_m = m.borrow();
    let mut outEqNodes: metamodelica::List<i32>;
    outEqNodes = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut var in ((*metamodelica::index_checked(&__ab_m, eqn)?).clone())
            .into_iter()
            .cloned()
        {
            if !(var.clone() > 0
                && (*metamodelica::index_checked(&__ab_ass1, var.clone())?).clone() != eqn
                && (*metamodelica::index_checked(&__ab_ass1, var.clone())?).clone() > 0)
            {
                continue;
            }
            let __x = (*metamodelica::index_checked(&__ab_ass1, var.clone())?).clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEqNodes)
}

pub(crate) fn isAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = intGt((*metamodelica::index_checked(&__ab_ass, intAbs(i))?).clone(), 0);
    Ok(b)
}

pub(crate) fn isUnAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> Result<bool> {
    let __ab_ass = ass.borrow();
    let mut b: bool;
    b = intLt((*metamodelica::index_checked(&__ab_ass, intAbs(i))?).clone(), 1);
    Ok(b)
}

pub(crate) fn getMarked(
    mut ne: i32,
    mut mark: i32,
    mut markArr: metamodelica::Array<i32>,
    mut iMarked: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        match ne {
            0 => return Ok(iMarked),
            _ => {
                let mut marked: metamodelica::List<i32>;
                marked = List::consOnTrue(
                    intEq(
                        ({
                            let __elt = (*metamodelica::index_checked(&markArr.borrow(), ne)?).clone();
                            __elt
                        }),
                        mark,
                    ),
                    ne,
                    iMarked,
                );
                {
                    (ne, mark, markArr, iMarked) = (ne - 1, mark, markArr.clone(), marked);
                    continue '__tco;
                }
            }
        }
    }
}

pub(crate) fn getUnassigned(
    mut ne: i32,
    mut ass: metamodelica::Array<i32>,
    mut inUnassigned: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        match ne {
            0 => return Ok(inUnassigned),
            _ => {
                let mut unassigned: metamodelica::List<i32>;
                unassigned = List::consOnTrue(
                    intLt(
                        ({
                            let __elt = (*metamodelica::index_checked(&ass.borrow(), ne)?).clone();
                            __elt
                        }),
                        1,
                    ),
                    ne,
                    inUnassigned,
                );
                {
                    (ne, ass, inUnassigned) = (ne - 1, ass.clone(), unassigned);
                    continue '__tco;
                }
            }
        }
    }
}

pub(crate) fn anyUnassigned(mut ne: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    '__tco: loop {
        match ne {
            0 => return Ok(false),
            _ if (intLt(
                ({
                    let __elt = (*metamodelica::index_checked(&ass.borrow(), ne)?).clone();
                    __elt
                }),
                1,
            )) =>
            {
                return Ok(true);
            }
            _ => {
                (ne, ass) = (ne - 1, ass.clone());
                continue '__tco;
            }
        }
    }
}

pub(crate) fn getAssignedArray(mut ass: metamodelica::Array<i32>) -> Result<metamodelica::Array<bool>> {
    let mut outIsAssigned: metamodelica::Array<bool>;
    let mut N: i32 = metamodelica::arrayLength(ass.clone());
    outIsAssigned = arrayCreate(N, false);
    for mut i in 1..=N {
        if ({
            let __elt = (*metamodelica::index_checked(&ass.borrow(), i)?).clone();
            __elt
        }) > 0
        {
            metamodelica::arrayUpdate(outIsAssigned.clone(), i, true)?;
        }
    }
    Ok(outIsAssigned)
}

pub(crate) fn getAssigned(
    mut ne: i32,
    mut ass: metamodelica::Array<i32>,
    mut inAssigned: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        match ne {
            0 => return Ok(inAssigned),
            _ => {
                let mut assigned: metamodelica::List<i32>;
                assigned = List::consOnTrue(
                    intGt(
                        ({
                            let __elt = (*metamodelica::index_checked(&ass.borrow(), ne)?).clone();
                            __elt
                        }),
                        0,
                    ),
                    ne,
                    inAssigned,
                );
                {
                    (ne, ass, inAssigned) = (ne - 1, ass.clone(), assigned);
                    continue '__tco;
                }
            }
        }
    }
}

pub(crate) fn getEqnsforIndexReduction(
    mut U: &metamodelica::List<i32>,
    mut neqns: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut eqns: metamodelica::List<metamodelica::List<i32>>;
    eqns = (::match_deref::match_deref! { match &((&**U, inArg)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (_, (_, _, mapEqnIncRow, mapIncRowEqn, _)) => {
            let mut lengthU: i32;
            let mut colummarks: metamodelica::Array<i32>;
            let mut subsets: metamodelica::Array<metamodelica::List<i32>>;
            colummarks = arrayCreate(neqns, -1);
            lengthU = ((U).len() as i32);
            subsets = arrayCreate(lengthU, metamodelica::nil());
            subsets = getEqnsforIndexReduction1(U, m.clone(), mT.clone(), 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), subsets.clone())?;
            removeEmptySubsets(1, lengthU, subsets.clone(), metamodelica::nil())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn removeEmptySubsets(
    mut index: i32,
    mut length: i32,
    mut subsets: metamodelica::Array<metamodelica::List<i32>>,
    mut iAcc: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iAcc.clone()) {
            _ if (intLe(index, length)) => {
                let mut eqns: metamodelica::List<i32>;
                let mut acc: metamodelica::List<metamodelica::List<i32>>;
                eqns = ({let __elt = (*metamodelica::index_checked(&subsets.borrow(), index)?).clone(); __elt});
                acc = appendNonEmpty(eqns, iAcc);
                { (index, length, subsets, iAcc) = (index + 1, length, subsets.clone(), acc); continue '__tco; }
            },
            _ => {
                return Ok(iAcc)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn appendNonEmpty(
    mut eqns: metamodelica::List<i32>,
    mut iAcc: metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<metamodelica::List<i32>>;
    oAcc = (::match_deref::match_deref! { match &(eqns.clone()) {
        Deref @ metamodelica::ListNode::Nil => iAcc,
        _ => metamodelica::cons(eqns, iAcc),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oAcc
}

fn getEqnsforIndexReduction1<'__b>(
    mut U: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubsets: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match U {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inSubsets.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } if (!(intGt(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), 0))) => {
                let mut eqns: metamodelica::List<i32>;
                let mut e1: i32;
                Error::checkCancel()?;
                e1 = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), e.clone())?).clone(); __elt});
                eqns = ({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), e1)?).clone(); __elt});
                List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), mark, colummarks.clone())?;
                eqns = getEqnsforIndexReductionphase(&(eqns.clone()), m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), eqns)?;
                Array::appendToElement(mark, eqns, inSubsets.clone())?;
                { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets) = (rest, m.clone(), mT.clone(), mark + 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone()); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getEqnsforIndexReductionphase<'__b>(
    mut elst: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubsets: metamodelica::Array<metamodelica::List<i32>>,
    mut inEqns: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match elst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inEqns)
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                let mut rows: metamodelica::List<i32>;
                let mut eqns: metamodelica::List<i32>;
                rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                eqns = getEqnsforIndexReductiontraverseRows(rows, metamodelica::nil(), m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), inEqns)?;
                { (elst, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets, inEqns) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), eqns); continue '__tco; }
            },
            _ => {
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getEqnsforIndexReductiontraverseRows(
    mut rows: metamodelica::List<i32>,
    mut nextColums: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubsets: metamodelica::Array<metamodelica::List<i32>>,
    mut inEqns: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((rows, nextColums.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inEqns)
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(getEqnsforIndexReductionphase(&nextColums, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), inEqns)?)
            },
            (Deref @ metamodelica::ListNode::Cons { head: r, tail: rest }, _) => {
                let mut queue: metamodelica::List<i32>;
                let mut nextqueue: metamodelica::List<i32>;
                let mut eqns: metamodelica::List<i32>;
                let mut rc: i32;
                let mut e: i32;
                rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                if List::exist1(&({let __elt = (*metamodelica::index_checked(&mT.borrow(), r.clone())?).clone(); __elt}), &fnptr!(intEq, i32, i32), rc)? && intGt(rc, 0) && !(intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), rc)?).clone(); __elt}), mark)) {
                    if intGt(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), rc)?).clone(); __elt}), 0) {
                        mergeSubsets(mark, ({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), rc)?).clone(); __elt}), inSubsets.clone(), colummarks.clone())?;
                        nextqueue = nextColums;
                        queue = inEqns;
                    } else {
                        e = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), rc)?).clone(); __elt});
                        eqns = ({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), e)?).clone(); __elt});
                        List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), mark, colummarks.clone())?;
                        nextqueue = listAppend(nextColums, eqns.clone());
                        queue = listAppend(inEqns, eqns);
                    }
                } else {
                    nextqueue = nextColums;
                    queue = inEqns;
                }
                { (rows, nextColums, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubsets, inEqns) = (rest.clone(), nextqueue, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubsets.clone(), queue); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn mergeSubsets(
    mut mark: i32,
    mut markColum: i32,
    mut inSubsets: metamodelica::Array<metamodelica::List<i32>>,
    mut colummarks: metamodelica::Array<i32>,
) -> Result<()> {
    let mut eqns: metamodelica::List<i32>;
    eqns = ({
        let __elt = (*metamodelica::index_checked(&inSubsets.borrow(), markColum)?).clone();
        __elt
    });
    Array::appendToElement(mark, eqns.clone(), inSubsets.clone())?;
    metamodelica::arrayUpdate(inSubsets.clone(), markColum, metamodelica::nil())?;
    List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), mark, colummarks.clone())?;
    Ok(())
}

fn reduceIndexifNecessary(
    mut meqns: metamodelica::List<i32>,
    mut actualEqn: i32,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut nv: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut sssHandler: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::List<i32>>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::List<i32>,
        i32,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<i32>,
    i32,
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
)> {
    let mut outchangedEqns: metamodelica::List<i32>;
    let mut continueEqn: i32;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut nvars: i32;
    let mut neqns: i32;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    (
        outchangedEqns,
        continueEqn,
        osyst,
        oshared,
        nvars,
        neqns,
        outAss1,
        outAss2,
        outArg,
    ) = (::match_deref::match_deref! { match &((meqns.clone(), inMatchingOptions)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (metamodelica::nil(), actualEqn + 1, isyst, ishared, nv, ne, ass1.clone(), ass2.clone(), inArg)
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, (BackendDAE::IndexReduction::INDEX_REDUCTION { .. }, _)) => {
            let mut nv_1: i32;
            let mut ne_1: i32;
            let mut i_1: i32;
            let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut ass1_1: metamodelica::Array<i32>;
            let mut ass1_2: metamodelica::Array<i32>;
            let mut ass2_1: metamodelica::Array<i32>;
            let mut ass2_2: metamodelica::Array<i32>;
            let mut changedEqns: metamodelica::List<i32>;
            (changedEqns, i_1, syst, shared, ass2_1, ass1_1, arg) = sssHandler(list![meqns], actualEqn, isyst, ishared, ass2.clone(), ass1.clone(), inArg)?;
            ne_1 = BackendDAEUtil::systemSize(&syst)?;
            nv_1 = BackendVariable::daenumVariables(&syst);
            ass1_2 = assignmentsArrayExpand(ass1_1.clone(), ne_1, metamodelica::arrayLength(ass1_1.clone()), -1)?;
            ass2_2 = assignmentsArrayExpand(ass2_1.clone(), nv_1, metamodelica::arrayLength(ass2_1.clone()), -1)?;
            (changedEqns, i_1, syst, shared, nv_1, ne_1, ass1_2.clone(), ass2_2.clone(), arg)
        },
        (_, _) => {
            singularSystemError(list![meqns], actualEqn, &isyst, &ishared, ass1.clone(), ass2.clone(), &inArg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((
        outchangedEqns,
        continueEqn,
        osyst,
        oshared,
        nvars,
        neqns,
        outAss1,
        outAss2,
        outArg,
    ))
}

fn assignmentsArrayExpand(
    mut ass: metamodelica::Array<i32>,
    mut needed: i32,
    mut memsize: i32,
    mut default: i32,
) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32>;
    outAss = 'mc: {
        let __mc_input = default;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(memsize, needed)) else {
                return Err("pattern mismatch");
            };
            Ok(ass.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(memsize, needed)) else {
                return Err("pattern mismatch");
            };
            Ok(Array::expand(needed - memsize, ass.clone(), default)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function assignmentsArrayExpand failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAss)
}

fn assignmentsArrayBooleanExpand(
    mut ass: metamodelica::Array<bool>,
    mut needed: i32,
    mut memsize: i32,
    mut default: bool,
) -> Result<metamodelica::Array<bool>> {
    let mut outAss: metamodelica::Array<bool>;
    outAss = 'mc: {
        let __mc_input = default;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intGt(memsize, needed)) else {
                return Err("pattern mismatch");
            };
            Ok(ass.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (intGt(memsize, needed)) else {
                return Err("pattern mismatch");
            };
            Ok(Array::expand(needed - memsize, ass.clone(), default)?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function assignmentsArrayExpand failed"),
                metamodelica::sourceInfo!("BackEnd/Matching.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAss)
}

fn checkAssignment(
    mut indx: i32,
    mut ne: i32,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut inUnassigned: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        let mut unassigned: metamodelica::List<i32>;
        if intGt(indx, ne) {
            return Ok(inUnassigned);
        } else {
            unassigned = List::consOnTrue(
                intLt(
                    ({
                        let __elt = (*metamodelica::index_checked(&ass1.borrow(), indx)?).clone();
                        __elt
                    }),
                    0,
                ),
                indx,
                inUnassigned,
            );
            {
                (indx, ne, ass1, ass2, inUnassigned) = (indx + 1, ne, ass1.clone(), ass2.clone(), unassigned);
                continue '__tco;
            }
        }
    }
}

fn getAssignment(
    mut clearMatching: bool,
    mut nVars: i32,
    mut nEqns: i32,
    mut iSyst: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> (metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    (ass1, ass2) = (::match_deref::match_deref! { match &((clearMatching, &**iSyst)) {
        (false, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __esc_ass1, ass2: __esc_ass2, .. }, .. }) if (intGe(nVars, metamodelica::arrayLength(__esc_ass1.clone())) && intGe(nEqns, metamodelica::arrayLength(__esc_ass2.clone()))) => {
            ass1 = (*__esc_ass1).clone();
            ass2 = (*__esc_ass2).clone();
            (ass2.clone(), ass1.clone())
        },
        _ => {
            ass2 = arrayCreate(nEqns, -1);
            ass1 = arrayCreate(nVars, -1);
            (ass2.clone(), ass1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (ass1, ass2)
}

// =============================================================================
// tests
//
// =============================================================================
pub(crate) fn testMatchingAlgorithms(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
) -> Result<()> {
    let mut t: metamodelica::Real;
    let mut nv: i32;
    let mut ne: i32;
    let mut cheapID: i32;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vec1: metamodelica::Array<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut matchingAlgorithms: metamodelica::List<(ArcStr, BackendDAEFunc::matchingAlgorithmFunc)>;
    let mut extmatchingAlgorithms: metamodelica::List<(ArcStr, i32)>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    ne = BackendDAEUtil::systemSize(&isyst)?;
    nv = BackendVariable::daenumVariables(&isyst);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Systemsize: "));
        __mm_s.push_str(&*intString(ne));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    matchingAlgorithms = list![
        (
            literal!("OMCNew:   "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| DFSLH(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("BFSB:     "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| BFSB(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("DFSB:     "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| DFSB(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("MC21A:    "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| MC21A(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("PF:       "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| PF(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("PFPlus:   "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| PFPlus(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("HK:       "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| HK(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("HKDW:     "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| HKDW(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("ABMP:     "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| ABMP(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        ),
        (
            literal!("PR:       "),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                      __a1: metamodelica::Ref<BackendDAE::Shared>,
                      __a2: bool,
                      __a3: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                      __a4: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::List<metamodelica::List<i32>>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::List<i32>,
                            i32,
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::Array<i32>,
                            metamodelica::Array<i32>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >,
                      __a5: (
                    BackendDAE::StateOrder,
                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<i32>,
                    i32
                )| PR_FIFO_FAIR(__a0, __a1, __a2, __a3, metamodelica::arc_ref(&__a4), __a5)
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            bool,
                            (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::List<metamodelica::List<i32>>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        ),
                                    ) -> Result<(
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Ref<BackendDAE::EqSystem>,
                                        metamodelica::Ref<BackendDAE::Shared>,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<i32>,
                                        (
                                            BackendDAE::StateOrder,
                                            metamodelica::Array<
                                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                            >,
                                            metamodelica::Array<metamodelica::List<i32>>,
                                            metamodelica::Array<i32>,
                                            i32
                                        )
                                    )> + 'static,
                            >,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::EqSystem>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            (
                                BackendDAE::StateOrder,
                                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<i32>,
                                i32
                            )
                        )> + 'static,
                >)
        )
    ];
    syst = randSortSystem(isyst, ishared)?;
    testMatchingAlgorithms1(&matchingAlgorithms, &syst, ishared, inMatchingOptions);
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    (_, m, _) = BackendDAEUtil::getAdjacencyMatrixfromOption(
        syst,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(ishared),
    )?;
    matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
    cheapID = 3;
    t = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("SetMEXT:     "));
        __mm_s.push_str(&*realString(t));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    extmatchingAlgorithms = list![
        (literal!("DFSEXT:   "), 1),
        (literal!("BFSEXT:   "), 2),
        (literal!("MC21AEXT: "), 3),
        (literal!("PFEXT:    "), 4),
        (literal!("PFPlusEXT:"), 5),
        (literal!("HKEXT:    "), 6),
        (literal!("HKDWEXT   "), 7),
        (literal!("ABMPEXT   "), 8),
        (literal!("PREXT:    "), 10)
    ];
    testExternMatchingAlgorithms1(&extmatchingAlgorithms, cheapID, nv, ne);
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    vec1 = arrayCreate(ne, -1);
    vec2 = arrayCreate(nv, -1);
    BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("GetAssEXT:   "));
        __mm_s.push_str(&*realString(t));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
    Ok(())
}

pub(crate) fn testMatchingAlgorithms1(
    mut matchingAlgorithms: &metamodelica::List<(
        ArcStr,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<BackendDAE::EqSystem>,
                    metamodelica::Ref<BackendDAE::Shared>,
                    bool,
                    (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
                    Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::List<metamodelica::List<i32>>,
                                i32,
                                metamodelica::Ref<BackendDAE::EqSystem>,
                                metamodelica::Ref<BackendDAE::Shared>,
                                metamodelica::Array<i32>,
                                metamodelica::Array<i32>,
                                (
                                    BackendDAE::StateOrder,
                                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                    metamodelica::Array<i32>,
                                    i32,
                                ),
                            ) -> Result<(
                                metamodelica::List<i32>,
                                i32,
                                metamodelica::Ref<BackendDAE::EqSystem>,
                                metamodelica::Ref<BackendDAE::Shared>,
                                metamodelica::Array<i32>,
                                metamodelica::Array<i32>,
                                (
                                    BackendDAE::StateOrder,
                                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                    metamodelica::Array<i32>,
                                    i32,
                                ),
                            )> + 'static,
                    >,
                    (
                        BackendDAE::StateOrder,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<i32>,
                        i32,
                    ),
                ) -> Result<(
                    metamodelica::Ref<BackendDAE::EqSystem>,
                    metamodelica::Ref<BackendDAE::Shared>,
                    (
                        BackendDAE::StateOrder,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<i32>,
                        i32,
                    ),
                )> + 'static,
        >,
    )>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
) -> () {
    let () = 'mc: {
        let __mc_input = &**matchingAlgorithms;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r#str, matchingAlgorithm), tail: rest } => {
                    let mut t: metamodelica::Real;
                    System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
                    testMatchingAlgorithm(10, &*(matchingAlgorithm.clone()), isyst, ishared, inMatchingOptions)?;
                    t = System::realtimeTock(ClockIndexes::RT_PROFILER0.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*realString(realDiv(t, metamodelica::OrderedFloat(10.0_f64)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    testMatchingAlgorithms1(metamodelica::AsArg::as_arg(&rest), isyst, ishared, inMatchingOptions);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (r#str, _), tail: rest } => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("failed!\n")); ArcStr::from(__mm_s) });
                    testMatchingAlgorithms1(metamodelica::AsArg::as_arg(&rest), isyst, ishared, inMatchingOptions);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn testMatchingAlgorithm(
    mut index: i32,
    mut matchingAlgorithm: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
        (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::List<metamodelica::List<i32>>,
                    i32,
                    metamodelica::Ref<BackendDAE::EqSystem>,
                    metamodelica::Ref<BackendDAE::Shared>,
                    metamodelica::Array<i32>,
                    metamodelica::Array<i32>,
                    (
                        BackendDAE::StateOrder,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<i32>,
                        i32,
                    ),
                ) -> Result<(
                    metamodelica::List<i32>,
                    i32,
                    metamodelica::Ref<BackendDAE::EqSystem>,
                    metamodelica::Ref<BackendDAE::Shared>,
                    metamodelica::Array<i32>,
                    metamodelica::Array<i32>,
                    (
                        BackendDAE::StateOrder,
                        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                        metamodelica::Array<metamodelica::List<i32>>,
                        metamodelica::Array<i32>,
                        i32,
                    ),
                )> + 'static,
        >,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    ) -> Result<(
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        (
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        ),
    )>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
) -> Result<()> {
    let () = (match index {
        0 => (),
        _ => {
            let mut arg: (
                BackendDAE::StateOrder,
                metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Array<i32>,
                i32,
            );
            arg = IndexReduction::getStructurallySingularSystemHandlerArg(
                isyst,
                ishared,
                metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
            )?;
            matchingAlgorithm(
                isyst.clone(),
                ishared.clone(),
                true,
                inMatchingOptions,
                (std::sync::Arc::new(IndexReduction::pantelidesIndexReduction)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::List<metamodelica::List<i32>>,
                                i32,
                                metamodelica::Ref<BackendDAE::EqSystem>,
                                metamodelica::Ref<BackendDAE::Shared>,
                                metamodelica::Array<i32>,
                                metamodelica::Array<i32>,
                                (
                                    BackendDAE::StateOrder,
                                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                    metamodelica::Array<i32>,
                                    i32,
                                ),
                            ) -> Result<(
                                metamodelica::List<i32>,
                                i32,
                                metamodelica::Ref<BackendDAE::EqSystem>,
                                metamodelica::Ref<BackendDAE::Shared>,
                                metamodelica::Array<i32>,
                                metamodelica::Array<i32>,
                                (
                                    BackendDAE::StateOrder,
                                    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                    metamodelica::Array<i32>,
                                    i32,
                                ),
                            )> + 'static,
                    >),
                arg,
            )?;
            testMatchingAlgorithm(index - 1, matchingAlgorithm, isyst, ishared, inMatchingOptions)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn testExternMatchingAlgorithms1(
    mut matchingAlgorithms: &metamodelica::List<(ArcStr, i32)>,
    mut cheapId: i32,
    mut nv: i32,
    mut ne: i32,
) -> () {
    let mut r#str: ArcStr;
    let mut matchingAlgorithm: i32;
    let mut t: metamodelica::Real;
    for mut alg in &**matchingAlgorithms {
        (r#str, matchingAlgorithm) = alg.clone();
        if '__try0: {
            unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_PROFILER0.clone()), '__try0);
            testExternMatchingAlgorithm(10, matchingAlgorithm, cheapId, nv, ne);
            t = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_PROFILER0.clone()), '__try0);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*realString(realDiv(t, metamodelica::OrderedFloat(10.0_f64))));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok::<(), &'static str>(())
        }
        .is_err()
        {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("failed!\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    ()
}

pub(crate) fn testExternMatchingAlgorithm(
    mut index: i32,
    mut matchingAlgorithm: i32,
    mut cheapId: i32,
    mut nv: i32,
    mut ne: i32,
) -> () {
    let () = (match index {
        0 => (),
        _ => {
            BackendDAEEXT::matching(
                nv,
                ne,
                matchingAlgorithm,
                cheapId,
                metamodelica::OrderedFloat(1.0_f64),
                1,
            );
            testExternMatchingAlgorithm(index - 1, matchingAlgorithm, cheapId, nv, ne);
            ()
        }
    });
    ()
}

fn randSortSystem(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut ne: i32;
            let mut nv: i32;
            let mut randarr: metamodelica::Array<i32>;
            let mut randarr1: metamodelica::Array<i32>;
            let mut syst = (*syst).clone();
            ne = BackendDAEUtil::systemSize(&isyst)?;
            nv = BackendVariable::daenumVariables(&isyst);
            randarr = Array::createIntRange(ne);
            setrandArray(ne, randarr.clone())?;
            randarr1 = Array::createIntRange(nv);
            setrandArray(nv, randarr1.clone())?;
            assign_field!(
                syst.orderedEqs = randSortSystem1(ne, 0, randarr.clone(), eqns.clone(), BackendEquation::listEquation(&(metamodelica::nil()))?, &BackendEquation::get, &BackendEquation::add)?,
                syst.orderedVars = randSortSystem1(nv, 0, randarr1.clone(), vars.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), &BackendVariable::addVar)?
            );
            (syst, _, _) = BackendDAEUtil::getAdjacencyMatrix(BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(ishared))?;
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn randSortSystem1<
    '__b,
    Type_a: Clone + 'static + metamodelica::gc::MMTrace,
    Type_b: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut index: i32,
    mut offset: i32,
    mut randarr: metamodelica::Array<i32>,
    mut oldTypeA: Type_a,
    mut newTypeA: Type_a,
    mut get: &'__b dyn ::std::ops::Fn(Type_a, i32) -> Result<Type_b>,
    mut set: &'__b dyn ::std::ops::Fn(Type_b, Type_a) -> Result<Type_a>,
) -> Result<Type_a> {
    pub type getFunc<Type_a: Clone + 'static, Type_b: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a, i32) -> Result<Type_b> + 'static>;

    pub type setFunc<Type_b: Clone + 'static, Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_a) -> Result<Type_a> + 'static>;

    '__tco: loop {
        match index {
            0 => return Ok(newTypeA),
            _ => {
                let mut tb: Type_b;
                let mut ta: Type_a;
                tb = get(
                    oldTypeA.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&randarr.borrow(), index)?).clone();
                        __elt
                    }) + offset,
                )?;
                ta = set(tb, newTypeA)?;
                {
                    (index, offset, randarr, oldTypeA, newTypeA, get, set) =
                        (index - 1, offset, randarr.clone(), oldTypeA, ta, get, set);
                    continue '__tco;
                }
            }
        }
    }
}

fn singularSystemError(
    mut eqns: metamodelica::List<metamodelica::List<i32>>,
    mut actualEqn: i32,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<()> {
    let mut n: i32;
    let mut unmatched: metamodelica::List<i32>;
    let mut unmatched1: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut eqn_str: ArcStr;
    let mut var_str: ArcStr;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut info: SourceInfo;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    (_, _, _, mapIncRowEqn, _) = inArg.clone();
    n = BackendDAEUtil::systemSize(isyst)?;
    unmatched = List::flatten(eqns)?;
    unmatched1 = List::map1r(unmatched.clone(), &arrayGet, mapIncRowEqn.clone())?;
    unmatched1 = List::uniqueIntN(&unmatched1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
    eqn_str = BackendDump::dumpMarkedEqns(isyst, unmatched1.clone())?;
    vars = getUnassigned(n, inAssignments2.clone(), metamodelica::nil())?;
    vars = List::fold1(&unmatched, &getAssignedVars, inAssignments1.clone(), vars)?;
    var_str = BackendDump::dumpMarkedVars(isyst, vars)?;
    source = BackendEquation::markedEquationSource(isyst, (unmatched1).head().cloned()?)?;
    info = ElementSource::getElementSourceFileInfo(source);
    Error::addSourceMessage(&(Error::STRUCT_SINGULAR_SYSTEM.clone()), list![eqn_str, var_str], &info)?;
    Ok(())
}

fn getAssignedVars(
    mut e: i32,
    mut ass: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_ass = ass.borrow();
    let mut oAcc: metamodelica::List<i32>;
    let mut i: i32;
    let mut b: bool;
    i = (*metamodelica::index_checked(&__ab_ass, e)?).clone();
    b = intGt(i, 0);
    oAcc = List::consOnTrue(b, i, iAcc);
    Ok(oAcc)
}

fn clearArrayWithKnownSetIndexes(
    mut arr: metamodelica::Array<bool>,
    mut arrIx: metamodelica::Array<i32>,
    mut n: i32,
) -> Result<()> {
    let debug: bool = false;
    if metamodelica::OrderedFloat((n) as f64)
        > metamodelica::OrderedFloat(0.3_f64)
            * metamodelica::OrderedFloat((metamodelica::arrayLength(arr.clone())) as f64)
    {
        for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
            metamodelica::Dangerous::arrayUpdateNoBoundsChecking(arr.clone(), i, false);
        }
    } else {
        let true = (n <= metamodelica::arrayLength(arrIx.clone())) else {
            return Err("pattern mismatch");
        };
        for mut i in 1..=n {
            metamodelica::arrayUpdate(
                arr.clone(),
                metamodelica::Dangerous::arrayGetNoBoundsChecking(arrIx.clone(), i),
                false,
            )?;
        }
    }
    if debug {
        for mut e in 1..=metamodelica::arrayLength(arr.clone()) {
            Error::assertion(
                !(metamodelica::arrayGet(arr.clone(), e)?),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("clearArrayWithKnownSetIndexes failed: "));
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", e)));
                    __mm_s.push_str(&*literal!(" n="));
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", n)));
                    __mm_s.push_str(&*literal!(" ixs="));
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<_> = metamodelica::nil();
                            for mut i in (1..=n).into_iter() {
                                let __x = ArcStr::from(::std::format!(
                                    "{}",
                                    metamodelica::arrayGet(arrIx.clone(), i.clone())?
                                ));
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(","),
                    ));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("BackEnd/Matching.mo")),
            )?;
        }
    }
    Ok(())
}
