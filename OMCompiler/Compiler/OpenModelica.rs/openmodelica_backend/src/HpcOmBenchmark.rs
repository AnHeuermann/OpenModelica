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

use crate::HpcOmBenchmarkExt;
use openmodelica_util::System;

pub(crate) fn benchSystem() -> Result<((i32, i32), (i32, i32))> {
    let mut oTime: ((i32, i32), (i32, i32));
    let mut comCostM: i32;
    let mut comCostN: i32;
    let mut opCostM: i32;
    let mut opCostN: i32;
    let mut opCosts: metamodelica::List<i32>;
    let mut comCosts: metamodelica::List<i32>;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    opCosts = HpcOmBenchmarkExt::requiredTimeForOp()?;
    let true = (((opCosts).len() as i32) == 2) else {
        return Err("pattern mismatch");
    };
    opCostM = (opCosts).get(1)?;
    opCostN = (opCosts).get(2)?;
    s1 = intString(opCostM);
    s2 = intString(opCostN);
    comCosts = HpcOmBenchmarkExt::requiredTimeForComm()?;
    comCostM = (comCosts).get(1)?;
    comCostN = (comCosts).get(2)?;
    s1 = intString(comCostM);
    s2 = intString(comCostN);
    oTime = ((opCostM, opCostN), (comCostM, comCostN));
    Ok(oTime)
}

pub(crate) fn readCalcTimesFromFile(
    mut iFileNamePrefix: &ArcStr,
) -> Result<metamodelica::List<(i32, i32, metamodelica::Real)>> {
    let mut calcTimes: metamodelica::List<(i32, i32, metamodelica::Real)>;
    let mut fullFileName: ArcStr = arcstr::literal!("");
    let mut tmpCalcTimes: metamodelica::List<(i32, i32, metamodelica::Real)> = metamodelica::nil();
    calcTimes = 'mc: {
        let __mc_input = iFileNamePrefix.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut fullFileName: ArcStr = fullFileName.clone();
            let mut tmpCalcTimes: metamodelica::List<(i32, i32, metamodelica::Real)> = tmpCalcTimes.clone();
            fullFileName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iFileNamePrefix);
                __mm_s.push_str(&*literal!(".json"));
                ArcStr::from(__mm_s)
            };
            ::match_deref::match_deref! { match &(System::getFileModificationTime(fullFileName.clone())) {
                Some(_) => (),
                _ => return Err("pattern mismatch"),
            } };
            metamodelica::print(literal!("Using json-file\n"));
            tmpCalcTimes = readCalcTimesFromJson(fullFileName.clone())?;
            Ok((tmpCalcTimes.clone(), fullFileName.clone(), tmpCalcTimes.clone()))
        })() {
            fullFileName = __wb0;
            tmpCalcTimes = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut fullFileName: ArcStr = fullFileName.clone();
            let mut tmpCalcTimes: metamodelica::List<(i32, i32, metamodelica::Real)> = tmpCalcTimes.clone();
            fullFileName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*iFileNamePrefix);
                __mm_s.push_str(&*literal!(".xml"));
                ArcStr::from(__mm_s)
            };
            ::match_deref::match_deref! { match &(System::getFileModificationTime(fullFileName.clone())) {
                Some(_) => (),
                _ => return Err("pattern mismatch"),
            } };
            tmpCalcTimes = readCalcTimesFromXml(fullFileName.clone())?;
            Ok((tmpCalcTimes.clone(), fullFileName.clone(), tmpCalcTimes.clone()))
        })() {
            fullFileName = __wb0;
            tmpCalcTimes = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("readCalcTimesFromFile: No valid profiling-file found.\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(calcTimes)
}

fn readCalcTimesFromXml(mut fileName: ArcStr) -> Result<metamodelica::List<(i32, i32, metamodelica::Real)>> {
    let mut calcTimes: metamodelica::List<(i32, i32, metamodelica::Real)>;
    let mut tmpResult: metamodelica::List<metamodelica::Real>;
    tmpResult = HpcOmBenchmarkExt::readCalcTimesFromXml(fileName)?;
    calcTimes = expandCalcTimes(&tmpResult, &(metamodelica::nil()))?;
    Ok(calcTimes)
}

fn readCalcTimesFromJson(mut fileName: ArcStr) -> Result<metamodelica::List<(i32, i32, metamodelica::Real)>> {
    let mut calcTimes: metamodelica::List<(i32, i32, metamodelica::Real)>;
    let mut tmpResult: metamodelica::List<metamodelica::Real>;
    tmpResult = HpcOmBenchmarkExt::readCalcTimesFromJson(fileName)?;
    calcTimes = expandCalcTimes(&tmpResult, &(metamodelica::nil()))?;
    Ok(calcTimes)
}

fn expandCalcTimes(
    mut iList: &metamodelica::List<metamodelica::Real>,
    mut iTuples: &metamodelica::List<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::List<(i32, i32, metamodelica::Real)>> {
    let mut oTuples: metamodelica::List<(i32, i32, metamodelica::Real)>;
    let mut eqIdx: metamodelica::Real;
    let mut numOfCalcs: metamodelica::Real;
    let mut calcTimeSum: metamodelica::Real;
    let mut intNumOfCalcs: i32 = 0;
    let mut intEqIdx: i32 = 0;
    let mut rest: metamodelica::List<metamodelica::Real>;
    let mut tmpTuples: metamodelica::List<(i32, i32, metamodelica::Real)> = metamodelica::nil();
    oTuples = 'mc: {
        let __mc_input = &**iList;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: numOfCalcs, tail: Deref @ metamodelica::ListNode::Cons { head: calcTimeSum, tail: Deref @ metamodelica::ListNode::Cons { head: eqIdx, tail: rest } } } => {
                    let mut intEqIdx: i32 = intEqIdx.clone();
                    let mut intNumOfCalcs: i32 = intNumOfCalcs.clone();
                    let mut tmpTuples: metamodelica::List<(i32, i32, metamodelica::Real)> = tmpTuples.clone();
                    intNumOfCalcs = ((numOfCalcs.clone()).0.floor() as i32);
                    intEqIdx = ((eqIdx.clone()).0.floor() as i32);
                    tmpTuples = expandCalcTimes(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons((intEqIdx, intNumOfCalcs, calcTimeSum.clone()), iTuples.clone())))?;
                    Ok((tmpTuples.clone(), intEqIdx.clone(), intNumOfCalcs.clone(), tmpTuples.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            intEqIdx = __wb0;
            intNumOfCalcs = __wb1;
            tmpTuples = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iTuples.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("expandCalcTimes: Invalid number of list-entries\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oTuples)
}
