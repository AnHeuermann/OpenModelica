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

use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_frontend_dump::HashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::MMath;

pub(crate) fn check(
    mut tms: &metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    mut ist: UnitAbsyn::InstStore,
) -> UnitAbsyn::InstStore {
    let mut outSt: UnitAbsyn::InstStore;
    outSt = 'mc: {
        let __mc_input = (&**tms, ist);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, st) => {
                    Ok(st.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    Ok(UnitAbsyn::InstStore::INSTSTORE { store: st1.clone(), ht: ht.clone(), checkResult: Some(crate::UnitAbsyn::UnitCheckResult::CONSISTENT) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: tm1, tail: rest1 }, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st: UnitAbsyn::InstStore;
                    let (UnitAbsyn::CONSISTENT { .. }, _, __pa0) = (checkTerm(metamodelica::AsArg::as_arg(&tm1), st1.clone())?) else { return Err("pattern mismatch") };
                    st2 = metamodelica::Own::own(__pa0);
                    st = check(metamodelica::AsArg::as_arg(&rest1), UnitAbsyn::InstStore::INSTSTORE { store: st2.clone(), ht: ht.clone(), checkResult: Some(crate::UnitAbsyn::UnitCheckResult::CONSISTENT) });
                    Ok(st.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: tm1, tail: _ }, UnitAbsyn::InstStore::INSTSTORE { store: st1, ht, checkResult: _ }) => {
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let (UnitAbsyn::INCONSISTENT { u1: __pa0, u2: __pa1 }, _, _) = (checkTerm(metamodelica::AsArg::as_arg(&tm1), st1.clone())?) else { return Err("pattern mismatch") };
                    su1 = metamodelica::Own::own(__pa0);
                    su2 = metamodelica::Own::own(__pa1);
                    s1 = UnitAbsynBuilder::printTermsStr(list![tm1.clone()])?;
                    s2 = UnitAbsynBuilder::unit2str(&(UnitAbsyn::Unit::SPECIFIED { specified: su1.clone() }))?;
                    s3 = UnitAbsynBuilder::unit2str(&(UnitAbsyn::Unit::SPECIFIED { specified: su2.clone() }))?;
                    Error::addMessage(Error::INCONSISTENT_UNITS.clone(), list![s1.clone(), s2.clone(), s3.clone()])?;
                    Ok(UnitAbsyn::InstStore::INSTSTORE { store: st1.clone(), ht: ht.clone(), checkResult: Some(UnitAbsyn::UnitCheckResult::INCONSISTENT { u1: su1.clone(), u2: su2.clone() }) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::check() failed\n"))?;
                    metamodelica::print(literal!("check failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outSt
}

pub(crate) fn isComplete(mut st: &UnitAbsyn::Store) -> Result<(bool, UnitAbsyn::Store)> {
    let mut complete: bool;
    let mut stout: UnitAbsyn::Store;
    (complete, stout) = (match st.clone() {
        UnitAbsyn::Store {
            storeVector: mut vector,
            numElts: mut indx,
        } => {
            let mut lst: metamodelica::List<Option<UnitAbsyn::Unit>>;
            let mut comp: bool;
            let mut st2: UnitAbsyn::Store;
            lst = vector
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>();
            (comp, st2) = completeCheck(
                &lst,
                1,
                UnitAbsyn::Store {
                    storeVector: vector.clone(),
                    numElts: indx.clone(),
                },
            )?;
            (comp, st2)
        }
    });
    Ok((complete, stout))
}

fn completeCheck(
    mut ilst: &metamodelica::List<Option<UnitAbsyn::Unit>>,
    mut indx: i32,
    mut st: UnitAbsyn::Store,
) -> Result<(bool, UnitAbsyn::Store)> {
    let mut isComplete: bool;
    let mut stout: UnitAbsyn::Store;
    (isComplete, stout) = 'mc: {
        let __mc_input = (&**ilst, st);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, st2) => {
                    Ok((true, st2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Some(_), tail: lst }, st2) => {
                    let mut u2: UnitAbsyn::Unit;
                    let mut comp1: bool;
                    let mut st3: UnitAbsyn::Store;
                    (u2, st3) = normalize(indx, st2.clone())?;
                    let false = (unitHasUnknown(&u2)?) else { return Err("pattern mismatch") };
                    (comp1, _) = completeCheck(metamodelica::AsArg::as_arg(&lst), indx + 1, st3.clone())?;
                    Ok((comp1, st3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Some(_), tail: _ }, st2) => {
                    let mut u2: UnitAbsyn::Unit;
                    (u2, _) = normalize(indx, st2.clone())?;
                    let true = (unitHasUnknown(&u2)?) else { return Err("pattern mismatch") };
                    Ok((false, st2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: None, tail: _ }, st2) => {
                    Ok((true, st2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((isComplete, stout))
}

pub(crate) fn checkTerm(
    mut tm: &metamodelica::Ref<UnitAbsyn::UnitTerm>,
    mut st: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::SpecUnit, UnitAbsyn::Store)> {
    let mut result: UnitAbsyn::UnitCheckResult;
    let mut outUnit: UnitAbsyn::SpecUnit;
    let mut outSt: UnitAbsyn::Store;
    (result, outUnit, outSt) = 'mc: {
        let __mc_input = (&**tm, st);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::ADD { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut st4: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut res2: UnitAbsyn::UnitCheckResult;
                    let mut res3: UnitAbsyn::UnitCheckResult;
                    let mut res4: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    (res2, su2, st3) = checkTerm(metamodelica::AsArg::as_arg(&ut2), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::SUB { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut st4: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut res2: UnitAbsyn::UnitCheckResult;
                    let mut res3: UnitAbsyn::UnitCheckResult;
                    let mut res4: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    (res2, su2, st3) = checkTerm(metamodelica::AsArg::as_arg(&ut2), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::MUL { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut res2: UnitAbsyn::UnitCheckResult;
                    let mut res4: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    let mut su3: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    (res2, su2, st3) = checkTerm(metamodelica::AsArg::as_arg(&ut2), st2.clone())?;
                    su3 = mulSpecUnit(&su1, &su2)?;
                    res4 = chooseResult(res1.clone(), res2.clone(), crate::UnitAbsyn::UnitCheckResult::CONSISTENT)?;
                    Ok((res4.clone(), su3.clone(), st3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::DIV { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut res2: UnitAbsyn::UnitCheckResult;
                    let mut res4: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    let mut su3: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    (res2, su2, st3) = checkTerm(metamodelica::AsArg::as_arg(&ut2), st2.clone())?;
                    su3 = divSpecUnit(&su1, &su2)?;
                    res4 = chooseResult(res1.clone(), res2.clone(), crate::UnitAbsyn::UnitCheckResult::CONSISTENT)?;
                    Ok((res4.clone(), su3.clone(), st3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::EQN { ut1, ut2, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut st4: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut res2: UnitAbsyn::UnitCheckResult;
                    let mut res3: UnitAbsyn::UnitCheckResult;
                    let mut res4: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    (res2, su2, st3) = checkTerm(metamodelica::AsArg::as_arg(&ut2), st2.clone())?;
                    (res3, st4) = unify(su1.clone(), su2.clone(), st3.clone())?;
                    res4 = chooseResult(res1.clone(), res2.clone(), res3.clone())?;
                    Ok((res4.clone(), su1.clone(), st4.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::LOC { loc, origExp: _ }, st1) => {
                    let UnitAbsyn::UNSPECIFIED { .. } = (UnitAbsynBuilder::find(loc.clone(), metamodelica::AsArg::as_arg(&st1))?) else { return Err("pattern mismatch") };
                    Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, UnitAbsyn::SpecUnit { typeParameters: metamodelica::cons((MMath::Rational { nom: 1, denom: 1 }, UnitAbsyn::TypeParameter { name: literal!(""), indx: loc.clone() }), metamodelica::nil()), units: metamodelica::nil() }, st1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::LOC { loc, origExp: _ }, st1) => {
                    let mut su1: UnitAbsyn::SpecUnit;
                    let UnitAbsyn::SPECIFIED { specified: __pa0 } = (UnitAbsynBuilder::find(loc.clone(), metamodelica::AsArg::as_arg(&st1))?) else { return Err("pattern mismatch") };
                    su1 = metamodelica::Own::own(__pa0);
                    Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, su1.clone(), st1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ UnitAbsyn::UnitTerm::POW { ut1, exponent: expo1, origExp: _ }, st1) => {
                    let mut st2: UnitAbsyn::Store;
                    let mut res1: UnitAbsyn::UnitCheckResult;
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    (res1, su1, st2) = checkTerm(metamodelica::AsArg::as_arg(&ut1), st1.clone())?;
                    su2 = powSpecUnit(&su1, expo1.clone())?;
                    Ok((res1.clone(), su2.clone(), st2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::checkTerm() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((result, outUnit, outSt))
}

fn chooseResult(
    mut res1: UnitAbsyn::UnitCheckResult,
    mut res2: UnitAbsyn::UnitCheckResult,
    mut res3: UnitAbsyn::UnitCheckResult,
) -> Result<UnitAbsyn::UnitCheckResult> {
    let mut resout: UnitAbsyn::UnitCheckResult;
    let mut incon: UnitAbsyn::UnitCheckResult;
    resout = (match (res1, res2, res3) {
        (
            UnitAbsyn::UnitCheckResult::CONSISTENT { .. },
            UnitAbsyn::UnitCheckResult::CONSISTENT { .. },
            UnitAbsyn::UnitCheckResult::CONSISTENT { .. },
        ) => crate::UnitAbsyn::UnitCheckResult::CONSISTENT,
        (
            UnitAbsyn::UnitCheckResult::CONSISTENT { .. },
            UnitAbsyn::UnitCheckResult::CONSISTENT { .. },
            mut __esc_incon,
        ) => {
            incon = __esc_incon.clone();
            incon
        }
        (UnitAbsyn::UnitCheckResult::CONSISTENT { .. }, mut __esc_incon, _) => {
            incon = __esc_incon.clone();
            incon
        }
        (mut __esc_incon, _, _) => {
            incon = __esc_incon.clone();
            incon
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::chooseResult() failed\n"))?;
            return Err("fail");
        }
    });
    Ok(resout)
}

fn unify(
    mut insu1: UnitAbsyn::SpecUnit,
    mut insu2: UnitAbsyn::SpecUnit,
    mut st: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::Store)> {
    let mut outresult: UnitAbsyn::UnitCheckResult;
    let mut outSt: UnitAbsyn::Store;
    let mut su1: UnitAbsyn::SpecUnit;
    let mut su2: UnitAbsyn::SpecUnit;
    let mut st1: UnitAbsyn::Store;
    let mut st2: UnitAbsyn::Store;
    let (UnitAbsyn::SPECIFIED { specified: __pa0 }, __pa1) =
        (normalizeOnUnit(&(UnitAbsyn::Unit::SPECIFIED { specified: insu1 }), st)?)
    else {
        return Err("pattern mismatch");
    };
    su1 = metamodelica::Own::own(__pa0);
    st1 = metamodelica::Own::own(__pa1);
    let (UnitAbsyn::SPECIFIED { specified: __pa2 }, __pa3) =
        (normalizeOnUnit(&(UnitAbsyn::Unit::SPECIFIED { specified: insu2 }), st1)?)
    else {
        return Err("pattern mismatch");
    };
    su2 = metamodelica::Own::own(__pa2);
    st2 = metamodelica::Own::own(__pa3);
    (outresult, outSt) = unifyunits(su1, su2, st2)?;
    Ok((outresult, outSt))
}

fn isSpecUnitEq(mut insu1: UnitAbsyn::SpecUnit, mut insu2: UnitAbsyn::SpecUnit) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((insu1, insu2)) {
            (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Nil }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Nil }) => {
                return true
            },
            (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Nil }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: 0, denom: _ }, tail: rest1 } }) => {
                let mut r1: bool;
                { (insu1, insu2) = (UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: metamodelica::nil() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() }); continue '__tco; }
            },
            (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: 0, denom: _ }, tail: rest1 } }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Nil }) => {
                let mut r1: bool;
                { (insu1, insu2) = (UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: metamodelica::nil() }); continue '__tco; }
            },
            (UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: i1a, denom: i1b }, tail: rest1 } }, UnitAbsyn::SpecUnit { typeParameters: _, units: Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: i2a, denom: i2b }, tail: rest2 } }) if (intEq(i1a.clone(), i2a.clone()) && intEq(i1b.clone(), i2b.clone())) => {
                let mut r1: bool;
                { (insu1, insu2) = (UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest1.clone() }, UnitAbsyn::SpecUnit { typeParameters: metamodelica::nil(), units: rest2.clone() }); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn unifyunits(
    mut insu1: UnitAbsyn::SpecUnit,
    mut insu2: UnitAbsyn::SpecUnit,
    mut st: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::UnitCheckResult, UnitAbsyn::Store)> {
    let mut outresult: UnitAbsyn::UnitCheckResult;
    let mut outSt: UnitAbsyn::Store;
    (outresult, outSt) = 'mc: {
        let __mc_input = (insu1, insu2, st);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let false = (hasUnknown(&(su1.clone()))?) else {
                return Err("pattern mismatch");
            };
            let false = (hasUnknown(&(su2.clone()))?) else {
                return Err("pattern mismatch");
            };
            let true = (isSpecUnitEq(su1.clone(), su2.clone())) else {
                return Err("pattern mismatch");
            };
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st1.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let false = (hasUnknown(&(su1.clone()))?) else {
                return Err("pattern mismatch");
            };
            let false = (hasUnknown(&(su2.clone()))?) else {
                return Err("pattern mismatch");
            };
            Ok((
                UnitAbsyn::UnitCheckResult::INCONSISTENT {
                    u1: su1.clone(),
                    u2: su2.clone(),
                },
                st1.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su1, mut su2, mut st1) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut su3: UnitAbsyn::SpecUnit;
            let mut su4: UnitAbsyn::SpecUnit;
            let mut st2: UnitAbsyn::Store;
            let mut loc1: i32;
            su3 = divSpecUnit(&(su2.clone()), &(su1.clone()))?;
            (loc1, su4) = getUnknown(&su3)?;
            st2 = UnitAbsynBuilder::update(
                UnitAbsyn::Unit::SPECIFIED { specified: su4.clone() },
                loc1,
                &(st1.clone()),
            )?;
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st2.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, mut st1) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((crate::UnitAbsyn::UnitCheckResult::CONSISTENT, st1.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outresult, outSt))
}

pub(crate) fn newDimlessSpecUnit() -> Result<UnitAbsyn::SpecUnit> {
    let mut su: UnitAbsyn::SpecUnit;
    let UnitAbsyn::SPECIFIED { specified: __pa0 } = (UnitAbsynBuilder::str2unit(literal!("1"), None)?) else {
        return Err("pattern mismatch");
    };
    su = metamodelica::Own::own(__pa0);
    Ok(su)
}

pub(crate) fn getUnknown(mut suin: &UnitAbsyn::SpecUnit) -> Result<(i32, UnitAbsyn::SpecUnit)> {
    let mut loc: i32;
    let mut suout: UnitAbsyn::SpecUnit;
    (loc, suout) = 'mc: {
        let __mc_input = suin;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::ListNode::Cons { head: (expo1, UnitAbsyn::TypeParameter { name: _, indx: loc1 }), tail: rest1 }, units: unitvec1 } => {
                    let mut su1: UnitAbsyn::SpecUnit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    let mut expo2: MMath::Rational;
                    su1 = divSpecUnit(&(newDimlessSpecUnit()?), &(UnitAbsyn::SpecUnit { typeParameters: rest1.clone(), units: unitvec1.clone() }))?;
                    expo2 = MMath::divRational(MMath::Rational { nom: 1, denom: 1 }, expo1.clone())?;
                    su2 = powSpecUnit(&su1, expo2)?;
                    Ok((loc1.clone(), su2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::getUnknown() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((loc, suout))
}

pub(crate) fn hasUnknown(mut su: &UnitAbsyn::SpecUnit) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(su) {
        UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::ListNode::Nil, units: _ } => false,
        UnitAbsyn::SpecUnit { typeParameters: _, units: _ } => true,
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("UnitChecker::hasUnknown() failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn unitHasUnknown(mut u: &UnitAbsyn::Unit) -> Result<bool> {
    let mut res: bool;
    res = (match u.clone() {
        UnitAbsyn::Unit::SPECIFIED { specified: mut su } => {
            let mut unk: bool;
            unk = hasUnknown(metamodelica::AsArg::as_arg(&su))?;
            unk
        }
        _ => true,
    });
    Ok(res)
}

pub(crate) fn mulSpecUnit(mut u1: &UnitAbsyn::SpecUnit, mut u2: &UnitAbsyn::SpecUnit) -> Result<UnitAbsyn::SpecUnit> {
    let mut u: UnitAbsyn::SpecUnit;
    u = 'mc: {
        let __mc_input = (u1.clone(), u2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (
                UnitAbsyn::SpecUnit {
                    typeParameters: ref tparams1,
                    units: ref units1,
                },
                UnitAbsyn::SpecUnit {
                    typeParameters: ref tparams2,
                    units: ref units2,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut tparams3: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut tparams4: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut units: metamodelica::List<MMath::Rational>;
            tparams3 = listAppend(tparams1.clone(), tparams2.clone());
            tparams4 = normalizeParamsExponents(&tparams3)?;
            units = mulUnitVec(&(units1.clone()), &(units2.clone()))?;
            Ok(UnitAbsyn::SpecUnit {
                typeParameters: tparams4.clone(),
                units: units.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::mulSpecUnit() failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(u)
}

pub(crate) fn mulUnitVec(
    mut inunitvec1: &metamodelica::List<MMath::Rational>,
    mut inunitvec2: &metamodelica::List<MMath::Rational>,
) -> Result<metamodelica::List<MMath::Rational>> {
    let mut outunitvec: metamodelica::List<MMath::Rational>;
    outunitvec = 'mc: {
        let __mc_input = (&**inunitvec1, &**inunitvec2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: expo2, tail: rest2 }) => {
                    let mut expo3: MMath::Rational;
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    expo3 = MMath::addRational(expo1.clone(), expo2.clone())?;
                    rest3 = mulUnitVec(metamodelica::AsArg::as_arg(&rest1), metamodelica::AsArg::as_arg(&rest2))?;
                    Ok(metamodelica::cons(expo3, rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    rest3 = mulUnitVec(metamodelica::AsArg::as_arg(&rest1), &(metamodelica::nil()))?;
                    Ok(metamodelica::cons(expo1.clone(), rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }) => {
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    rest3 = mulUnitVec(&(metamodelica::nil()), metamodelica::AsArg::as_arg(&rest1))?;
                    Ok(metamodelica::cons(expo1.clone(), rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::powUnitVec() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outunitvec)
}

pub(crate) fn divSpecUnit(mut u1: &UnitAbsyn::SpecUnit, mut u2: &UnitAbsyn::SpecUnit) -> Result<UnitAbsyn::SpecUnit> {
    let mut u: UnitAbsyn::SpecUnit;
    u = 'mc: {
        let __mc_input = (u1.clone(), u2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (
                UnitAbsyn::SpecUnit {
                    typeParameters: ref tparams1,
                    units: ref units1,
                },
                UnitAbsyn::SpecUnit {
                    typeParameters: ref tparams2,
                    units: ref units2,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut tparams3: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut tparams4: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut tparams5: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut units: metamodelica::List<MMath::Rational>;
            tparams3 = negParamList(&(tparams2.clone()), metamodelica::nil())?;
            tparams4 = listAppend(tparams1.clone(), tparams3.clone());
            tparams5 = normalizeParamsExponents(&tparams4)?;
            units = divUnitVec(&(units1.clone()), &(units2.clone()))?;
            Ok(UnitAbsyn::SpecUnit {
                typeParameters: tparams5.clone(),
                units: units.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::divSpecUnit() failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(u)
}

pub(crate) fn divUnitVec(
    mut inunitvec1: &metamodelica::List<MMath::Rational>,
    mut inunitvec2: &metamodelica::List<MMath::Rational>,
) -> Result<metamodelica::List<MMath::Rational>> {
    let mut outunitvec: metamodelica::List<MMath::Rational>;
    outunitvec = 'mc: {
        let __mc_input = (&**inunitvec1, &**inunitvec2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: expo2, tail: rest2 }) => {
                    let mut expo3: MMath::Rational;
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    expo3 = MMath::subRational(expo1.clone(), expo2.clone())?;
                    rest3 = divUnitVec(metamodelica::AsArg::as_arg(&rest1), metamodelica::AsArg::as_arg(&rest2))?;
                    Ok(metamodelica::cons(expo3, rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    rest3 = divUnitVec(metamodelica::AsArg::as_arg(&rest1), &(metamodelica::nil()))?;
                    Ok(metamodelica::cons(expo1.clone(), rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }) => {
                    let mut expo2: MMath::Rational;
                    let mut rest3: metamodelica::List<MMath::Rational>;
                    expo2 = MMath::subRational(MMath::Rational { nom: 0, denom: 1 }, expo1.clone())?;
                    rest3 = divUnitVec(&(metamodelica::nil()), metamodelica::AsArg::as_arg(&rest1))?;
                    Ok(metamodelica::cons(expo2, rest3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::powUnitVec() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outunitvec)
}

pub(crate) fn powSpecUnit(mut suin: &UnitAbsyn::SpecUnit, mut expo: MMath::Rational) -> Result<UnitAbsyn::SpecUnit> {
    let mut uout: UnitAbsyn::SpecUnit;
    uout = 'mc: {
        let __mc_input = suin.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::SpecUnit {
                typeParameters: ref params1,
                units: ref unitvec1,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut params2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut unitvec2: metamodelica::List<MMath::Rational>;
            params2 = powUnitParams(&(params1.clone()), expo)?;
            unitvec2 = powUnitVec(&(unitvec1.clone()), expo)?;
            Ok(UnitAbsyn::SpecUnit {
                typeParameters: params2.clone(),
                units: unitvec2.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::powSpecUnit() failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(uout)
}

pub(crate) fn powUnitParams(
    mut inparams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    mut expo: MMath::Rational,
) -> Result<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> {
    let mut outparams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    outparams = 'mc: {
        let __mc_input = (&**inparams, expo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (expo1, param), tail: rest1 }, expo2) => {
                    let mut expo3: MMath::Rational;
                    let mut rest2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    expo3 = MMath::multRational(expo1.clone(), expo2.clone())?;
                    rest2 = powUnitParams(metamodelica::AsArg::as_arg(&rest1), expo2.clone())?;
                    Ok(metamodelica::cons((expo3, param.clone()), rest2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::powUnitParams() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outparams)
}

pub(crate) fn powUnitVec(
    mut inunitvec: &metamodelica::List<MMath::Rational>,
    mut expo: MMath::Rational,
) -> Result<metamodelica::List<MMath::Rational>> {
    let mut outunitvec: metamodelica::List<MMath::Rational>;
    outunitvec = 'mc: {
        let __mc_input = (&**inunitvec, expo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: expo1, tail: rest1 }, expo2) => {
                    let mut expo3: MMath::Rational;
                    let mut rest2: metamodelica::List<MMath::Rational>;
                    expo3 = MMath::multRational(expo1.clone(), expo2.clone())?;
                    rest2 = powUnitVec(metamodelica::AsArg::as_arg(&rest1), expo2.clone())?;
                    Ok(metamodelica::cons(expo3, rest2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::powUnitVec() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outunitvec)
}

fn negParamList(
    mut ine: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    mut ac: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
) -> Result<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> {
    let mut oute: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    oute = 'mc: {
        let __mc_input = (&**ine, ac);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ac2) => {
                    Ok(ac2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name, indx }), tail: rest }, ac2) => {
                    let mut qr: MMath::Rational;
                    let mut pres: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    qr = MMath::multRational(MMath::Rational { nom: -1, denom: 1 }, MMath::Rational { nom: i1.clone(), denom: i2.clone() })?;
                    pres = negParamList(metamodelica::AsArg::as_arg(&rest), metamodelica::cons((qr, UnitAbsyn::TypeParameter { name: name.clone(), indx: indx.clone() }), ac2.clone()))?;
                    Ok(pres.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::negParamList() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oute)
}

pub(crate) fn normalize(mut loc: i32, mut st: UnitAbsyn::Store) -> Result<(UnitAbsyn::Unit, UnitAbsyn::Store)> {
    let mut unit: UnitAbsyn::Unit;
    let mut outSt: UnitAbsyn::Store;
    let mut u1: UnitAbsyn::Unit;
    let mut u2: UnitAbsyn::Unit;
    let mut st2: UnitAbsyn::Store;
    u1 = UnitAbsynBuilder::find(loc, &st)?;
    (u2, st2) = normalizeOnUnit(&u1, st)?;
    outSt = UnitAbsynBuilder::update(u2.clone(), loc, &st2)?;
    unit = u2;
    Ok((unit, outSt))
}

pub(crate) fn normalizeOnUnit(
    mut u: &UnitAbsyn::Unit,
    mut st: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::Unit, UnitAbsyn::Store)> {
    let mut unit: UnitAbsyn::Unit;
    let mut outSt: UnitAbsyn::Store;
    (unit, outSt) = 'mc: {
        let __mc_input = u.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Unit::UNSPECIFIED { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((crate::UnitAbsyn::Unit::UNSPECIFIED, st.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Unit::SPECIFIED {
                specified:
                    UnitAbsyn::SpecUnit {
                        typeParameters: ref params1,
                        units: ref unitvec1,
                    },
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut params2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut params3: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut unitvec2: metamodelica::List<MMath::Rational>;
            let mut st2: UnitAbsyn::Store;
            let (
                UnitAbsyn::SPECUNIT {
                    typeParameters: __pa0,
                    units: __pa1,
                },
                __pa2,
            ) = normalizeParamsValues(
                &(params1.clone()),
                &(UnitAbsyn::SpecUnit {
                    typeParameters: metamodelica::nil(),
                    units: unitvec1.clone(),
                }),
                &st,
            )?;
            params2 = metamodelica::Own::own(__pa0);
            unitvec2 = metamodelica::Own::own(__pa1);
            st2 = metamodelica::Own::own(__pa2);
            params3 = normalizeParamsExponents(&params2)?;
            Ok((
                UnitAbsyn::Unit::SPECIFIED {
                    specified: UnitAbsyn::SpecUnit {
                        typeParameters: params3.clone(),
                        units: unitvec2.clone(),
                    },
                },
                st2.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::normalizeOnUnit() failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((unit, outSt))
}

fn normalizeParamsExponents(
    mut inparams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
) -> Result<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> {
    let mut outparams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    outparams = 'mc: {
        let __mc_input = &**inparams;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (expo1, UnitAbsyn::TypeParameter { name, indx: loc1 }), tail: rest1 } => {
                    let mut rest2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    let mut rest3: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    let mut expo2: MMath::Rational;
                    let mut expo3: MMath::Rational;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getParam(metamodelica::AsArg::as_arg(&rest1), loc1.clone())?) {
                        (true, __pa0, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    expo2 = metamodelica::Own::own(__pa0);
                    rest2 = metamodelica::Own::own(__pa1);
                    expo3 = MMath::addRational(expo1.clone(), expo2)?;
                    rest3 = normalizeParamsExponents(&(metamodelica::cons((expo3, UnitAbsyn::TypeParameter { name: name.clone(), indx: loc1.clone() }), rest2.clone())))?;
                    Ok(rest3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (MMath::Rational { nom: 0, denom: 1 }, _), tail: rest1 } => {
                    let mut rest2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    rest2 = normalizeParamsExponents(metamodelica::AsArg::as_arg(&rest1))?;
                    Ok(rest2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: param, tail: rest1 } => {
                    let mut rest2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
                    rest2 = normalizeParamsExponents(metamodelica::AsArg::as_arg(&rest1))?;
                    Ok(metamodelica::cons(param.clone(), rest2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::normalizeParamsExponents() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outparams)
}

fn getParam(
    mut inparams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    mut loc: i32,
) -> Result<(
    bool,
    MMath::Rational,
    metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
)> {
    let mut found: bool;
    let mut outexpo: MMath::Rational;
    let mut outparams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    (found, outexpo, outparams) = (::match_deref::match_deref! { match inparams {
        Deref @ metamodelica::ListNode::Nil => {
            (false, MMath::Rational { nom: 1, denom: 1 }, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: (expo, UnitAbsyn::TypeParameter { name: _, indx: loc2 }), tail: rest } if (intEq(loc2.clone(), loc)) => {
            (true, expo.clone(), rest.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: param, tail: rest } => {
            let mut rest2: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut expo: MMath::Rational;
            let mut found2: bool;
            (found2, expo, rest2) = getParam(rest, loc)?;
            (found2, expo, metamodelica::cons(param.clone(), rest2))
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("UnitChecker::getParam() failed\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((found, outexpo, outparams))
}

fn normalizeParamsValues(
    mut inparams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    mut suin: &UnitAbsyn::SpecUnit,
    mut st: &UnitAbsyn::Store,
) -> Result<(UnitAbsyn::SpecUnit, UnitAbsyn::Store)> {
    let mut uout: UnitAbsyn::SpecUnit;
    let mut outSt: UnitAbsyn::Store;
    (uout, outSt) = 'mc: {
        let __mc_input = &**inparams;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((suin.clone(), st.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (expo, UnitAbsyn::TypeParameter { name, indx: loc }), tail: rest } => {
                    let mut st2: UnitAbsyn::Store;
                    let mut st3: UnitAbsyn::Store;
                    let mut u2: UnitAbsyn::Unit;
                    let mut su2: UnitAbsyn::SpecUnit;
                    let mut su3: UnitAbsyn::SpecUnit;
                    (u2, st2) = normalize(loc.clone(), st.clone())?;
                    su2 = mulSpecUnitWithNorm(suin.clone(), &u2, name.clone(), loc.clone(), expo.clone())?;
                    (su3, st3) = normalizeParamsValues(metamodelica::AsArg::as_arg(&rest), &su2, &st2)?;
                    Ok((su3.clone(), st3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("UnitChecker::normalizeParamsValues() failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((uout, outSt))
}

fn mulSpecUnitWithNorm(
    mut suin: UnitAbsyn::SpecUnit,
    mut normunit: &UnitAbsyn::Unit,
    mut name: ArcStr,
    mut loc: i32,
    mut expo: MMath::Rational,
) -> Result<UnitAbsyn::SpecUnit> {
    let mut suout: UnitAbsyn::SpecUnit;
    suout = 'mc: {
        let __mc_input = (suin, normunit.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (
                UnitAbsyn::SpecUnit {
                    typeParameters: ref params,
                    units: ref unitvec,
                },
                UnitAbsyn::Unit::UNSPECIFIED { .. },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok(UnitAbsyn::SpecUnit {
                typeParameters: metamodelica::cons(
                    (
                        expo,
                        UnitAbsyn::TypeParameter {
                            name: name.clone(),
                            indx: loc,
                        },
                    ),
                    params.clone(),
                ),
                units: unitvec.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut su2, UnitAbsyn::Unit::SPECIFIED { specified: mut sunorm }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut su3: UnitAbsyn::SpecUnit;
            let mut su4: UnitAbsyn::SpecUnit;
            su3 = powSpecUnit(&(sunorm.clone()), expo)?;
            su4 = mulSpecUnit(&(su2.clone()), &su3)?;
            Ok(su4.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("UnitChecker::mulSpecUnitWithNorm() failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(suout)
}

pub(crate) fn printSpecUnit(mut text: ArcStr, mut su: UnitAbsyn::SpecUnit) -> Result<()> {
    let () = (match (text, su.clone()) {
        (
            mut r#str,
            UnitAbsyn::SpecUnit {
                typeParameters: ref params,
                units: _,
            },
        ) => {
            metamodelica::print(r#str);
            metamodelica::print(literal!(" \""));
            metamodelica::print(UnitAbsynBuilder::unit2str(
                &(UnitAbsyn::Unit::SPECIFIED { specified: su }),
            )?);
            metamodelica::print(literal!("\" {"));
            printSpecUnitParams(metamodelica::AsArg::as_arg(&params))?;
            metamodelica::print(literal!("}\n"));
            ()
        }
    });
    Ok(())
}

pub(crate) fn printSpecUnitParams(
    mut params: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match params {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name, indx: loc }), tail: rest } => {
            metamodelica::print(literal!("(\""));
            metamodelica::print(name.clone());
            metamodelica::print(literal!("\","));
            metamodelica::print(intString(loc.clone()));
            metamodelica::print(literal!(")^("));
            metamodelica::print(intString(i1.clone()));
            metamodelica::print(literal!("/"));
            metamodelica::print(intString(i2.clone()));
            metamodelica::print(literal!("),"));
            printSpecUnitParams(rest)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn testUnitOp() -> () {
    metamodelica::print(literal!("test"));
    ()
}

pub(crate) fn printResult(mut res: &UnitAbsyn::UnitCheckResult) -> Result<()> {
    let () = (match res.clone() {
        UnitAbsyn::UnitCheckResult::CONSISTENT { .. } => {
            metamodelica::print(literal!("\n---\nThe system of units is consistent.\n---\n"));
            ()
        }
        UnitAbsyn::UnitCheckResult::INCONSISTENT { u1: mut u1, u2: mut u2 } => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            metamodelica::print(literal!("\n---\nThe system of units is inconsistent. \""));
            str1 = UnitAbsynBuilder::unit2str(&(UnitAbsyn::Unit::SPECIFIED { specified: u1.clone() }))?;
            metamodelica::print(str1);
            metamodelica::print(literal!("\" != \""));
            str2 = UnitAbsynBuilder::unit2str(&(UnitAbsyn::Unit::SPECIFIED { specified: u2.clone() }))?;
            metamodelica::print(str2);
            metamodelica::print(literal!("\"\n---\n"));
            ()
        }
    });
    Ok(())
}
