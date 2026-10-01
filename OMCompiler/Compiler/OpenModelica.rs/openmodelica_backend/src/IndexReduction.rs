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

use crate::AdjacencyMatrix;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::InlineArrayEquations;
use crate::Matching;
use crate::Sorting;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::HashTableCrIntToExp;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::HashTable2;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// Pantelides index reduction method .
// see:
// C Pantelides, The Consistent Initialization of Differential-Algebraic Systems, SIAM J. Sci. and Stat. Comput. Volume 9, Issue 2, pp. 213–231 (March 1988)
// Soares, R. de P.; Secchi, A. R.: Direct Initialisation and Solution of High-Index DAESystems. in Proceedings of the European Symbosium on Computer Aided Process Engineering - 15, Barcelona, Spain,
// =============================================================================
pub(crate) fn pantelidesIndexReduction(
    mut inEqns: metamodelica::List<metamodelica::List<i32>>,
    mut inActualEqn: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
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
    let mut changedEqns: metamodelica::List<i32>;
    let mut continueEqn: i32;
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oass1: metamodelica::Array<i32>;
    let mut oass2: metamodelica::Array<i32>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut markarr: metamodelica::Array<i32>;
    let mut size: i32;
    let mut newsize: i32;
    let mut eqns_1: metamodelica::List<metamodelica::List<i32>>;
    let mut unassignedStates: metamodelica::List<metamodelica::List<i32>>;
    let mut unassignedEqns: metamodelica::List<metamodelica::List<i32>>;
    Error::checkCancel()?;
    if (inEqns).is_empty() {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                "- IndexReduction.pantelidesIndexReduction called with empty list of equations!"
            )],
        )?;
        if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
            metamodelica::print(literal!("Index reduction done.\n"));
        }
        return Err("fail");
    }
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            metamodelica::print(literal!("\n\nIndex reduction:\n"));
        }
        ErrorExt::setCheckpoint(literal!("Pantelides"));
        (eqns_1, unassignedStates, unassignedEqns, _) = unwrap_break_err!(minimalStructurallySingularSystem(inEqns.clone(), inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), &inArg), '__try0);
        size = unwrap_break_err!(BackendDAEUtil::systemSize(&inSystem), '__try0);
        ErrorExt::delCheckpoint(literal!("Pantelides"));
        ErrorExt::setCheckpoint(literal!("Pantelides"));
        markarr = arrayCreate(size, -1);
        (osyst, oshared, oass1, oass2, outArg, _) = unwrap_break_err!(pantelidesIndexReduction1(&unassignedStates, &unassignedEqns, &inEqns, &eqns_1, inActualEqn, inSystem.clone(), inShared.clone(), inAssignments1.clone(), inAssignments2.clone(), 1, markarr.clone(), inArg.clone(), metamodelica::nil()), '__try0);
        ErrorExt::rollBack(literal!("Pantelides"));
        ErrorExt::setCheckpoint(literal!("Pantelides"));
        newsize = unwrap_break_err!(BackendDAEUtil::systemSize(&osyst), '__try0);
        changedEqns = if (newsize > size) {
            List::intRange2(size + 1, newsize)
        } else {
            metamodelica::nil()
        };
        (changedEqns, continueEqn) =
            unwrap_break_err!(getChangedEqnsAndLowest(newsize, oass2.clone(), changedEqns.clone(), size), '__try0);
        ErrorExt::delCheckpoint(literal!("Pantelides"));
        if unwrap_break_err!(Flags::isSet(Flags::OPT_DAE_DUMP.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqSystemShort(&osyst, &(literal!("pantelidesIndexReduction"))), '__try0);
            metamodelica::print(literal!("Index reduction done.\n"));
        }
        Ok::<_, &'static str>((
            changedEqns.clone(),
            continueEqn.clone(),
            eqns_1.clone(),
            markarr.clone(),
            newsize.clone(),
            oass1.clone(),
            oass2.clone(),
            oshared.clone(),
            osyst.clone(),
            outArg.clone(),
            size.clone(),
            unassignedEqns.clone(),
            unassignedStates.clone(),
        ))
    } {
        Ok((
            __try0_o0,
            __try0_o1,
            __try0_o2,
            __try0_o3,
            __try0_o4,
            __try0_o5,
            __try0_o6,
            __try0_o7,
            __try0_o8,
            __try0_o9,
            __try0_o10,
            __try0_o11,
            __try0_o12,
        )) => {
            changedEqns = __try0_o0;
            continueEqn = __try0_o1;
            eqns_1 = __try0_o2;
            markarr = __try0_o3;
            newsize = __try0_o4;
            oass1 = __try0_o5;
            oass2 = __try0_o6;
            oshared = __try0_o7;
            osyst = __try0_o8;
            outArg = __try0_o9;
            size = __try0_o10;
            unassignedEqns = __try0_o11;
            unassignedStates = __try0_o12;
        }
        Err(__try0_err) => {
            ErrorExt::delCheckpoint(literal!("Pantelides"));
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("- IndexReduction.pantelidesIndexReduction failed!")],
            )?;
            if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
                metamodelica::print(literal!("Index reduction done.\n"));
            }
            return Err(__try0_err);
        }
    }
    Ok((changedEqns, continueEqn, osyst, oshared, oass1, oass2, outArg))
}

pub(crate) fn failIfIndexReduction(
    mut inEqns: &metamodelica::List<metamodelica::List<i32>>,
    mut inActualEqn: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
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
    let mut changedEqns: metamodelica::List<i32> = metamodelica::nil();
    let mut inActualEqn: i32 = inActualEqn;
    let mut inSystem: metamodelica::Ref<BackendDAE::EqSystem> = inSystem;
    let mut inShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut inAssignments1: metamodelica::Array<i32> = inAssignments1;
    let mut inAssignments2: metamodelica::Array<i32> = inAssignments2;
    let mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ) = inArg;
    Error::addCompilerError(literal!(
        "Structurally singular system detected, but no index reduction method has been selected."
    ))?;
    return Err("fail");
    Ok((
        changedEqns,
        inActualEqn,
        inSystem,
        inShared,
        inAssignments1,
        inAssignments2,
        inArg,
    ))
}

fn getChangedEqnsAndLowest(
    mut index: i32,
    mut ass2: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
    mut iLowest: i32,
) -> Result<(metamodelica::List<i32>, i32)> {
    let __ab_ass2 = ass2.borrow();
    let mut oAcc: metamodelica::List<i32> = iAcc;
    let mut oLowest: i32 = iLowest;
    for mut i in ({
        let __s = index;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        oAcc = List::consOnTrue(
            intLt((*metamodelica::index_checked(&__ab_ass2, i)?).clone(), 1),
            i,
            oAcc,
        );
        oLowest = i;
    }
    Ok((oAcc, oLowest))
}

fn pantelidesIndexReduction1<'__b>(
    mut unassignedStates: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut unassignedEqns: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut alleqns: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut iEqns: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut actualEqn: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
    mut mark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut iNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<(
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
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (unassignedStates, unassignedEqns, alleqns, iEqns) {
            (_, _, _, Deref @ metamodelica::ListNode::Nil) => {
                let mut ass1: metamodelica::Array<i32>;
                let mut ass2: metamodelica::Array<i32>;
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                (syst, shared, ass1, ass2, arg) = handleundifferntiableMSSLst(iNotDiffableMSS, inSystem, inShared, inAssignments1.clone(), inAssignments2.clone(), inArg)?;
                return Ok((syst, shared, ass1.clone(), ass2.clone(), arg, metamodelica::nil()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: states, tail: statelst }, Deref @ metamodelica::ListNode::Cons { head: ueqns, tail: ueqnsrest }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnsrest }, Deref @ metamodelica::ListNode::Cons { head: eqns_1, tail: eqnsrest_1 }) => {
                let mut ass1: metamodelica::Array<i32>;
                let mut ass2: metamodelica::Array<i32>;
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut notDiffableMSS: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>)>;
                (syst, shared, ass1, ass2, arg, notDiffableMSS) = pantelidesIndexReductionMSS(states.clone(), ueqns.clone(), metamodelica::AsArg::as_arg(&eqns), eqns_1.clone(), actualEqn, inSystem, inShared, inAssignments1.clone(), inAssignments2.clone(), mark, markarr.clone(), &inArg, iNotDiffableMSS)?;
                { (unassignedStates, unassignedEqns, alleqns, iEqns, actualEqn, inSystem, inShared, inAssignments1, inAssignments2, mark, markarr, inArg, iNotDiffableMSS) = (statelst, ueqnsrest, eqnsrest, eqnsrest_1, actualEqn, syst, shared, ass1.clone(), ass2.clone(), mark, markarr.clone(), arg, notDiffableMSS); continue '__tco; }
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("- IndexReduction.pantelidesIndexReduction1 failed! Use -d=bltdump to get more information.")])?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn pantelidesIndexReductionMSS(
    mut unassignedStates: metamodelica::List<i32>,
    mut unassignedEqns: metamodelica::List<i32>,
    mut alleqns: &metamodelica::List<i32>,
    mut MSSSeqs: metamodelica::List<i32>,
    mut actualEqn: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
    mut mark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut iNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<(
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
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outAssignments1: metamodelica::Array<i32>;
    let mut outAssignments2: metamodelica::Array<i32>;
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut oNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>;
    (
        osyst,
        oshared,
        outAssignments1,
        outAssignments2,
        outArg,
        oNotDiffableMSS,
    ) = 'mc: {
        let __mc_input = (&*MSSSeqs, &*inSystem, inArg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqnsarray, .. }, (so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn, noofeqns)) => {
                    let mut MSSSeqs1: metamodelica::List<i32>;
                    let mut orgEqnsLst1: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut eqnstpl: metamodelica::List<(i32, Option<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::Ref<BackendDAE::Equation>)>;
                    let mut notDiffableMSS: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>, metamodelica::List<i32>)>;
                    let mut mapEqnIncRow = (*mapEqnIncRow).clone();
                    let mut mapIncRowEqn = (*mapIncRowEqn).clone();
                    MSSSeqs1 = List::map1r(MSSSeqs.clone(), &arrayGet, mapIncRowEqn.clone())?;
                    MSSSeqs1 = List::uniqueIntN(&MSSSeqs1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
                    MSSSeqs1 = List::select1(MSSSeqs1.clone(), (std::sync::Arc::new(fnptr!(intLe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), noofeqns.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##############--MSSS--##############\n")); __mm_s.push_str(&*literal!("Indices of constraint equations: ")); ArcStr::from(__mm_s) });
                        BackendDump::debuglst(&MSSSeqs1, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
                        metamodelica::print(literal!("\n"));
                    }
                    (eqnstpl, shared) = differentiateEqnsLst(MSSSeqs1.clone(), vars.clone(), eqnsarray.clone(), inShared.clone())?;
                    (syst, shared, ass1, ass2, orgEqnsLst1, mapEqnIncRow, mapIncRowEqn, notDiffableMSS) = differentiateEqns(&eqnstpl, MSSSeqs1.clone(), unassignedStates.clone(), unassignedEqns.clone(), inSystem.clone(), shared.clone(), inAssignments1.clone(), inAssignments2.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), iNotDiffableMSS.clone())?;
                    Ok((syst.clone(), shared.clone(), ass1.clone(), ass2.clone(), (so.clone(), orgEqnsLst1.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), noofeqns.clone()), notDiffableMSS.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("- IndexReduction.pantelidesIndexReductionMSS failed! Use -d=bltdump to get more information.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        osyst,
        oshared,
        outAssignments1,
        outAssignments2,
        outArg,
        oNotDiffableMSS,
    ))
}

fn eqnstplDebugString(
    mut tpl: (
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    ),
) -> Result<ArcStr> {
    let mut s: ArcStr;
    if (Util::tuple32(tpl.clone())).is_some() {
        s = literal!("");
    } else {
        s = BackendDump::equationString(&(Util::getOption(Util::tuple32(tpl.clone()))?))?;
    }
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Original Eq "));
        __mm_s.push_str(&*intString(Util::tuple31(tpl.clone())));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!("\n\t-->"));
        __mm_s.push_str(&*BackendDump::equationString(&(Util::tuple33(tpl)))?);
        __mm_s.push_str(&*literal!(""));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

pub(crate) fn minimalStructurallySingularSystem(
    mut inEqnsLst: metamodelica::List<metamodelica::List<i32>>,
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<i32>,
)> {
    let mut outEqnsLst: metamodelica::List<metamodelica::List<i32>>;
    let mut outStateIndxs: metamodelica::List<metamodelica::List<i32>>;
    let mut outunassignedEqns: metamodelica::List<metamodelica::List<i32>>;
    let mut discEqns: metamodelica::List<i32>;
    let mut unassignedEqns: metamodelica::List<i32>;
    let mut eqnslst: metamodelica::List<i32>;
    let mut stateindxs: metamodelica::List<i32>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut statemark: metamodelica::Array<i32>;
    let mut size: i32;
    let mut b: bool;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, m: Some(__pa2), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    m = metamodelica::Own::own(__pa2);
    size = BackendVariable::varsSize(&vars);
    statemark = arrayCreate(size, -1);
    unassignedEqns = List::flatten(inEqnsLst.clone())?;
    stateindxs = List::fold2(
        &unassignedEqns,
        &statesInEquations,
        (m.clone(), statemark.clone(), 0),
        inAssignments1.clone(),
        metamodelica::nil(),
    )?;
    (unassignedEqns, eqnslst, discEqns) = List::fold3(
        &unassignedEqns,
        &move |__a0: i32,
               __a1: BackendDAE::Variables,
               __a2: metamodelica::Array<i32>,
               __a3: metamodelica::Array<metamodelica::List<i32>>,
               __a4: (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        )| unassignedContinuesEqns(__a0, __a1, __a2, __a3, &__a4),
        vars.clone(),
        inAssignments2.clone(),
        m.clone(),
        (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
    )?;
    b = intGe(((stateindxs).len() as i32), ((unassignedEqns).len() as i32));
    singularSystemError(
        b,
        stateindxs,
        &unassignedEqns,
        eqnslst,
        syst.clone(),
        shared.clone(),
        inAssignments1.clone(),
        inAssignments2.clone(),
        inArg,
    )?;
    (outEqnsLst, outStateIndxs, outunassignedEqns, discEqns) = minimalStructurallySingularSystemMSS(
        &inEqnsLst,
        syst,
        shared,
        inAssignments1.clone(),
        inAssignments2.clone(),
        inArg,
        statemark.clone(),
        1,
        m.clone(),
        vars,
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    Ok((outEqnsLst, outStateIndxs, outunassignedEqns, discEqns))
}

fn minimalStructurallySingularSystemMSS(
    mut inEqnsLst: &metamodelica::List<metamodelica::List<i32>>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAssignments1: metamodelica::Array<i32>,
    mut inAssignments2: metamodelica::Array<i32>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut statemark: metamodelica::Array<i32>,
    mut mark: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: BackendDAE::Variables,
    mut inEqnsLstAcc: metamodelica::List<metamodelica::List<i32>>,
    mut inStateIndxsAcc: metamodelica::List<metamodelica::List<i32>>,
    mut inUnassEqnsAcc: metamodelica::List<metamodelica::List<i32>>,
    mut inDiscEqnsAcc: metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<i32>,
)> {
    let mut outEqnsLst: metamodelica::List<metamodelica::List<i32>> = inEqnsLstAcc;
    let mut outStateIndxs: metamodelica::List<metamodelica::List<i32>> = inStateIndxsAcc;
    let mut outUnassEqnsAcc: metamodelica::List<metamodelica::List<i32>> = inUnassEqnsAcc;
    let mut outDiscEqns: metamodelica::List<i32> = inDiscEqnsAcc;
    let mut markIdx: i32 = mark;
    let mut unassignedEqns: metamodelica::List<i32>;
    let mut eqnsLst: metamodelica::List<i32>;
    let mut stateIndxs: metamodelica::List<i32>;
    let mut b: bool;
    for mut ilst in &**inEqnsLst {
        (unassignedEqns, eqnsLst, outDiscEqns) = List::fold3(
            metamodelica::AsArg::as_arg(&ilst),
            &move |__a0: i32,
                   __a1: BackendDAE::Variables,
                   __a2: metamodelica::Array<i32>,
                   __a3: metamodelica::Array<metamodelica::List<i32>>,
                   __a4: (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            )| unassignedContinuesEqns(__a0, __a1, __a2, __a3, &__a4),
            vars.clone(),
            inAssignments2.clone(),
            m.clone(),
            (metamodelica::nil(), metamodelica::nil(), outDiscEqns),
        )?;
        stateIndxs = List::fold2(
            metamodelica::AsArg::as_arg(&ilst),
            &statesInEquations,
            (m.clone(), statemark.clone(), markIdx),
            inAssignments1.clone(),
            metamodelica::nil(),
        )?;
        b = intGe(((stateIndxs).len() as i32), ((unassignedEqns).len() as i32));
        singularSystemError(
            b,
            stateIndxs.clone(),
            &unassignedEqns,
            eqnsLst.clone(),
            inSystem.clone(),
            inShared.clone(),
            inAssignments1.clone(),
            inAssignments2.clone(),
            inArg,
        )?;
        outEqnsLst = metamodelica::cons(eqnsLst, outEqnsLst);
        outStateIndxs = metamodelica::cons(stateIndxs, outStateIndxs);
        outUnassEqnsAcc = metamodelica::cons(unassignedEqns, outUnassEqnsAcc);
        markIdx = markIdx + 1;
    }
    Ok((outEqnsLst, outStateIndxs, outUnassEqnsAcc, outDiscEqns))
}

fn singularSystemError(
    mut b: bool,
    mut unassignedStates: metamodelica::List<i32>,
    mut unassignedEqns: &metamodelica::List<i32>,
    mut eqns: metamodelica::List<i32>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
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
    let () = (::match_deref::match_deref! { match &((b, eqns.clone(), inArg.clone())) {
        (true, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => {
            ()
        },
        (_, Deref @ metamodelica::ListNode::Nil, (_, _, _, mapIncRowEqn, _)) => {
            let mut eqns1: metamodelica::List<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                metamodelica::print(literal!("Reduce Index failed! Found empty set of continuous equations.\nmarked equations:\n"));
            }
            eqns1 = List::map1r(eqns, &arrayGet, mapIncRowEqn.clone())?;
            eqns1 = List::uniqueIntN(&eqns1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                metamodelica::print(BackendDump::dumpMarkedEqns(&inSystem, eqns1)?);
            }
            syst = BackendDAEUtil::setEqSystMatching(inSystem, metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: inAssignments1.clone(), ass2: inAssignments2.clone(), comps: metamodelica::nil() }));
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printBackendDAE(&(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: list![syst], shared: inShared })))?;
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("IndexReduction.pantelidesIndexReduction failed! Found empty set of continuous equations. Use -d=bltdump to get more information.")])?;
            return Err("fail")
        },
        (false, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, (_, _, _, mapIncRowEqn, _)) => {
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqns1: metamodelica::List<i32>;
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                metamodelica::print(literal!("Reduce Index failed! System is structurally singular and cannot be handled because the number of unassigned continuous equations is larger than the number of states.\nmarked equations:\n"));
                BackendDump::debuglst(&eqns, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
            }
            eqns1 = List::map1r(eqns, &arrayGet, mapIncRowEqn.clone())?;
            eqns1 = List::uniqueIntN(&eqns1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                metamodelica::print(BackendDump::dumpMarkedEqns(&inSystem, eqns1)?);
                metamodelica::print(literal!("\n\nunassigned states:\n"));
            }
            varlst = List::map1r(unassignedStates, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), BackendVariable::daeVars(&inSystem))?;
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printVarList(&varlst)?;
            }
            syst = BackendDAEUtil::setEqSystMatching(inSystem, metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: inAssignments1.clone(), ass2: inAssignments2.clone(), comps: metamodelica::nil() }));
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                BackendDump::printBackendDAE(&(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: list![syst], shared: inShared })))?;
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("IndexReduction.pantelidesIndexReduction failed! System is structurally singular and cannot be handled because the number of unassigned equations is larger than the number of states. Use -d=bltdump to get more information.")])?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn unassignedContinuesEqns(
    mut eindx: i32,
    mut vars: BackendDAE::Variables,
    mut ass2: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inFold: &(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let __ab_ass2 = ass2.borrow();
    let __ab_m = m.borrow();
    let mut outFold: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    outFold = 'mc: {
        let __mc_input = inFold;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (unassignedEqns, eqnsLst, discEqns) => {
                    let mut vindx: i32;
                    let mut varlst: metamodelica::List<i32>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut b: bool;
                    let mut ba: bool;
                    let mut unassignedEqns = (*unassignedEqns).clone();
                    let mut eqnsLst = (*eqnsLst).clone();
                    let mut discEqns = (*discEqns).clone();
                    vindx = (*metamodelica::index_checked(&__ab_ass2, eindx)?).clone();
                    ba = intLt(vindx, 1);
                    varlst = (*metamodelica::index_checked(&__ab_m, eindx)?).clone();
                    varlst = List::map(varlst.clone(), &fnptr!(intAbs, i32))?;
                    vlst = List::map1r(varlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    b = List::all(&vlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isVarDiscrete(&__a0)) })?;
                    eqnsLst = List::consOnTrue(!(b), eindx, eqnsLst.clone());
                    unassignedEqns = List::consOnTrue(ba && !(b), eindx, unassignedEqns.clone());
                    discEqns = List::consOnTrue(b, eindx, discEqns.clone());
                    Ok((unassignedEqns.clone(), eqnsLst.clone(), discEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (unassignedEqns, eqnsLst, discEqns) => {
                    let mut vindx: i32;
                    vindx = (*metamodelica::index_checked(&__ab_ass2, eindx)?).clone();
                    let false = (intGt(vindx, 0)) else { return Err("pattern mismatch") };
                    Ok((metamodelica::cons(eindx, unassignedEqns.clone()), metamodelica::cons(eindx, eqnsLst.clone()), discEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outFold)
}

fn statesInEquations(
    mut eindx: i32,
    mut inTpl: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut ass1: metamodelica::Array<i32>,
    mut inStateLst: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outStateLst: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut statemark: metamodelica::Array<i32>;
    let mut mark: i32;
    (m, statemark, mark) = inTpl;
    vars = List::removeOnTrue(
        0,
        &fnptr!(intLt, i32, i32),
        ({
            let __elt = (*metamodelica::index_checked(&m.borrow(), eindx)?).clone();
            __elt
        }),
    )?;
    vars = List::map(vars, &fnptr!(intAbs, i32))?;
    vars = List::removeOnTrue((statemark.clone(), mark), &isMarked, vars)?;
    List::fold1(&vars, &markTrue, mark, statemark.clone())?;
    outStateLst = listAppend(inStateLst, vars);
    Ok(outStateLst)
}

fn isMarked(mut ass: (metamodelica::Array<i32>, i32), mut indx: i32) -> Result<bool> {
    let mut b: bool;
    let mut arr: metamodelica::Array<i32>;
    let mut mark: i32;
    (arr, mark) = ass;
    b = intEq(
        ({
            let __elt = (*metamodelica::index_checked(&arr.borrow(), intAbs(indx))?).clone();
            __elt
        }),
        mark,
    );
    Ok(b)
}

fn markTrue(mut indx: i32, mut mark: i32, mut arr: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut arr: metamodelica::Array<i32> = arr;
    metamodelica::arrayUpdate(arr.clone(), intAbs(indx), mark)?;
    Ok(arr)
}

fn differentiateEqns(
    mut inEqnsTpl: &metamodelica::List<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    mut MSSSeqs: metamodelica::List<i32>,
    mut unassignedStates: metamodelica::List<i32>,
    mut unassignedEqns: metamodelica::List<i32>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAss1: metamodelica::Array<i32>,
    mut inAss2: metamodelica::Array<i32>,
    mut inOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut imapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut imapIncRowEqn: metamodelica::Array<i32>,
    mut iNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outAss1: metamodelica::Array<i32>;
    let mut outAss2: metamodelica::Array<i32>;
    let mut outOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut omapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut omapIncRowEqn: metamodelica::Array<i32>;
    let mut oNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut eqns_1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut v: BackendDAE::Variables;
    let mut v1: BackendDAE::Variables;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut numEqs: i32;
    let mut numEqs1: i32;
    let mut changedVars: metamodelica::List<i32>;
    let mut eqnslst: metamodelica::List<i32>;
    let mut eqnslst1: metamodelica::List<i32>;
    let mut assEqs: metamodelica::List<i32>;
    if (inEqnsTpl).is_empty() {
        osyst = inSystem;
        oshared = inShared;
        outAss1 = inAss1.clone();
        outAss2 = inAss2.clone();
        outOrgEqnsLst = inOrgEqnsLst.clone();
        omapEqnIncRow = imapEqnIncRow.clone();
        omapIncRowEqn = imapIncRowEqn.clone();
        oNotDiffableMSS = metamodelica::cons((MSSSeqs, unassignedStates, unassignedEqns), iNotDiffableMSS);
    } else {
        syst = inSystem;
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, m: Some(__pa2), mT: Some(__pa3), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        v = metamodelica::Own::own(__pa0);
        eqns = metamodelica::Own::own(__pa1);
        m = metamodelica::Own::own(__pa2);
        mt = metamodelica::Own::own(__pa3);
        numEqs = BackendEquation::getNumberOfEquations(eqns.clone());
        (v1, eqns_1, changedVars, outOrgEqnsLst) = replaceDifferentiatedEqns(
            inEqnsTpl,
            v,
            eqns,
            mt.clone(),
            imapIncRowEqn.clone(),
            metamodelica::nil(),
            inOrgEqnsLst.clone(),
        )?;
        numEqs1 = BackendEquation::getNumberOfEquations(eqns_1.clone());
        eqnslst = if (intGt(numEqs1, numEqs)) {
            List::intRange2(numEqs + 1, numEqs1)
        } else {
            metamodelica::nil()
        };
        assEqs = List::map1r(changedVars.clone(), &arrayGet, inAss1.clone())?;
        assEqs = List::select1(
            assEqs,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            0,
        )?;
        outAss2 = List::fold1r(&assEqs, &*(Arc::new(arrayUpdate.clone())), -1, inAss2.clone())?;
        outAss1 = List::fold1r(&changedVars, &*(Arc::new(arrayUpdate.clone())), -1, inAss1.clone())?;
        eqnslst1 = collectVarEqns(
            &changedVars,
            mt.clone(),
            metamodelica::arrayLength(mt.clone()),
            metamodelica::arrayLength(m.clone()),
        )?;
        assign_field!(syst.orderedVars = v1, syst.orderedEqs = eqns_1);
        eqnslst1 = List::map1r(eqnslst1, &arrayGet, imapIncRowEqn.clone())?;
        eqnslst1 = List::uniqueIntN(&(listAppend(MSSSeqs, eqnslst1)), numEqs1)?;
        eqnslst = listAppend(eqnslst1, eqnslst);
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            metamodelica::print(literal!("Update Adjacency Matrix: "));
            BackendDump::debuglst(&eqnslst, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
            metamodelica::print(literal!("\n"));
        }
        funcs = BackendDAEUtil::getFunctions(&inShared);
        (syst, omapEqnIncRow, omapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(
            syst,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(funcs),
            eqnslst,
            imapEqnIncRow.clone(),
            imapIncRowEqn.clone(),
            BackendDAEUtil::isInitializationDAE(&inShared),
        )?;
        osyst = syst;
        oshared = inShared;
        oNotDiffableMSS = iNotDiffableMSS;
    }
    Ok((
        osyst,
        oshared,
        outAss1,
        outAss2,
        outOrgEqnsLst,
        omapEqnIncRow,
        omapIncRowEqn,
        oNotDiffableMSS,
    ))
}

fn collectVarEqns(
    mut varIdcsIn: &metamodelica::List<i32>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut numVars: i32,
    mut numEqs: i32,
) -> Result<metamodelica::List<i32>> {
    let __ab_mT = mT.borrow();
    let mut eqIdcsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut eqIdcs: metamodelica::List<i32>;
    for mut varIdx in &**varIdcsIn {
        let mut varIdx = varIdx.clone();
        if intLt(varIdx, numVars) {
            eqIdcs = List::map(
                (*metamodelica::index_checked(&__ab_mT, varIdx)?).clone(),
                &fnptr!(intAbs, i32),
            )?;
            eqIdcsOut = listAppend(eqIdcs, eqIdcsOut);
        }
    }
    eqIdcsOut = List::uniqueIntN(&eqIdcsOut, numEqs)?;
    Ok(eqIdcsOut)
}

fn searchDerivativesExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (metamodelica::List<i32>, BackendDAE::Variables),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<i32>, BackendDAE::Variables),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (metamodelica::List<i32>, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &tpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (ilst, vars)) => {
                    let mut i1lst: metamodelica::List<i32>;
                    let mut ilst = (*ilst).clone();
                    (_, i1lst) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    ilst = List::fold1(&i1lst, &move |__a0: _, __a1: _, __a2: _| List::removeOnTrue(__a0, metamodelica::arc_ref(&__a1), __a2), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), ilst.clone())?;
                    Ok((e.clone(), (ilst.clone(), vars.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn differentiateEqnsLst(
    mut inEqns: metamodelica::List<i32>,
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outEqnTpl: metamodelica::List<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    let mut oShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut e: i32;
    let mut eqs: metamodelica::List<i32>;
    let mut eqTplOpt: Option<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    outEqnTpl = metamodelica::nil();
    oShared = inShared.clone();
    eqs = inEqns;
    while !((eqs).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        eqs = metamodelica::Own::own(__pa1);
        (eqTplOpt, oShared) = differentiateEqnsLst1(e, vars.clone(), eqns.clone(), oShared)?;
        if (eqTplOpt).is_some() {
            outEqnTpl = metamodelica::cons(Util::getOption(eqTplOpt)?, outEqnTpl);
        } else {
            outEqnTpl = metamodelica::nil();
            oShared = inShared;
            return Ok((outEqnTpl, oShared));
        }
    }
    Ok((outEqnTpl, oShared))
}

fn differentiateEqnsLst1(
    mut eqIdx: i32,
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    Option<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut oEqTpl: Option<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut diffEqn: Option<metamodelica::Ref<BackendDAE::Equation>>;
    eqn = BackendEquation::get(eqns, eqIdx)?;
    if BackendEquation::isDifferentiated(eqn.clone())? {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            BackendDump::debugStrEqnStr(
                &(literal!("Skip already differentiated equation\n")),
                &eqn,
                &(literal!("\n")),
            )?;
        }
        oEqTpl = Some((eqIdx, None, eqn));
        oshared = inShared;
    } else {
        (diffEqn, oshared) = Differentiate::differentiateEquationTime(&eqn, vars, inShared.clone())?;
        eqn = BackendEquation::markDifferentiated(eqn)?;
        if (diffEqn).is_some() {
            oEqTpl = Some((eqIdx, diffEqn, eqn));
        } else {
            oEqTpl = None;
            oshared = inShared;
        }
    }
    Ok((oEqTpl, oshared))
}

fn replaceDifferentiatedEqns(
    mut inEqnTplLst: &metamodelica::List<(
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    )>,
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut imapIncRowEqn: metamodelica::Array<i32>,
    mut inChangedVars: metamodelica::List<i32>,
    mut inOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<i32>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outChangedVars: metamodelica::List<i32>;
    let mut outOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqIdx: i32;
    let mut eqOrig: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqDiff: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqTpl: (
        i32,
        Option<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Equation>,
    ) = (0, None, metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION));
    outVars = vars;
    outEqns = eqns;
    outChangedVars = inChangedVars;
    outOrgEqns = inOrgEqns.clone();
    for mut eqTpl in &**inEqnTplLst {
        let mut eqTpl = eqTpl.clone();
        if (Util::tuple32(eqTpl.clone())).is_some() {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eqTpl) {
                (__pa0, Some(__pa1), __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqIdx = metamodelica::Own::own(__pa0);
            eqDiff = metamodelica::Own::own(__pa1);
            eqOrig = metamodelica::Own::own(__pa2);
            (eqDiff, _) = BackendEquation::traverseExpsOfEquation(
                eqDiff,
                (std::sync::Arc::new(replaceStateOrderExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                BackendDAE::Variables,
                            )
                                -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)>
                            + 'static,
                    >),
                outVars.clone(),
            )?;
            let (__pa3, (_, (__pa4, __pa5, __pa6, _, _, _))) = BackendEquation::traverseExpsOfEquation(
                eqDiff,
                (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
                (
                    (std::sync::Arc::new(changeDerVariablesToStatesFinder)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    (
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<metamodelica::List<i32>>,
                                    ),
                                ) -> Result<(
                                    metamodelica::Ref<DAE::Exp>,
                                    (
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                        metamodelica::List<i32>,
                                        i32,
                                        metamodelica::Array<i32>,
                                        metamodelica::Array<metamodelica::List<i32>>,
                                    ),
                                )> + 'static,
                        >),
                    (
                        outVars,
                        outEqns,
                        outChangedVars,
                        eqIdx,
                        imapIncRowEqn.clone(),
                        mt.clone(),
                    ),
                ),
            )?;
            eqDiff = metamodelica::Own::own(__pa3);
            outVars = metamodelica::Own::own(__pa4);
            outEqns = metamodelica::Own::own(__pa5);
            outChangedVars = metamodelica::Own::own(__pa6);
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                debugdifferentiateEqns(&((eqOrig.clone(), eqDiff.clone(), eqIdx)))?;
            }
            outEqns = BackendEquation::setAtIndex(outEqns, eqIdx, eqDiff)?;
            outOrgEqns = addOrgEqn(eqIdx, eqOrig, outOrgEqns.clone())?;
        }
    }
    Ok((outVars, outEqns, outChangedVars, outOrgEqns))
}

fn replaceStateOrderExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut vars: BackendDAE::Variables;
    (e, vars) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            replaceStateOrderExpFinder,
            metamodelica::Ref<DAE::Exp>,
            BackendDAE::Variables
        ),
        inVars,
    )?;
    Ok((e, vars))
}

fn replaceStateOrderExpFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVars: BackendDAE::Variables,
) -> (metamodelica::Ref<DAE::Exp>, bool, BackendDAE::Variables) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outVars: BackendDAE::Variables;
    (outExp, cont, outVars) = 'mc: {
        let __mc_input = (inExp, inVars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(__pa0), .. }, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dcr = metamodelica::Own::own(__pa0);
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((e.clone(), false, vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: index }, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, vars) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                    let true = (intEq(index.clone(), 2)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(__pa0), .. }, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dcr = metamodelica::Own::own(__pa0);
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e.clone()], attr: attr.clone() }), false, vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr }, vars) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(__pa0), .. }, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dcr = metamodelica::Own::own(__pa0);
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e.clone()], attr: attr.clone() }), false, vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, vars) => {
                    Ok((e.clone(), true, vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outVars)
}

fn statesWithUnusedDerivative(
    mut state: i32,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut iAcc: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let __ab_mt = mt.borrow();
    let mut oAcc: metamodelica::List<i32>;
    oAcc = 'mc: {
        let __mc_input = state;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (List::all(
                &(*metamodelica::index_checked(&__ab_mt, state)?),
                &({
                    let __pe_b1 = 0;
                    move |__pe_a0| Ok(intLt(__pe_a0, __pe_b1.clone()))
                }),
            )?) else {
                return Err("pattern mismatch");
            };
            Ok(metamodelica::cons(state, iAcc.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iAcc.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn isStateonIndex(mut index: i32, mut vars: &BackendDAE::Variables) -> Result<bool> {
    let mut b: bool;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    v = BackendVariable::getVarAt(vars, index)?;
    b = BackendVariable::isStateVar(&v);
    Ok(b)
}

fn handleundifferntiableMSSLst(
    mut iNotDiffableMSS: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inAss1: metamodelica::Array<i32>,
    mut inAss2: metamodelica::Array<i32>,
    mut iArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
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
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iNotDiffableMSS, inSystem.clone(), iArg.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok((inSystem, inShared, inAss1.clone(), inAss2.clone(), iArg))
            },
            (Deref @ metamodelica::ListNode::Cons { head: (eqns, unassignedStates, unassignedEqns), tail: notDiffableMSS }, Deref @ BackendDAE::EqSystem { orderedVars: v, mT: Some(mt), .. }, (so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn, noofeqns)) => {
                let mut ilst: metamodelica::List<i32>;
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut arg: (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32);
                let mut ass1: metamodelica::Array<i32>;
                let mut ass2: metamodelica::Array<i32>;
                let mut so = (*so).clone();
                let mut orgEqnsLst = (*orgEqnsLst).clone();
                let mut mapEqnIncRow = (*mapEqnIncRow).clone();
                let mut mapIncRowEqn = (*mapIncRowEqn).clone();
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    metamodelica::print(literal!("not differentiable minimal singular subset:\n"));
                    metamodelica::print(literal!("unassignedEqns:\n"));
                    BackendDump::debuglst(metamodelica::AsArg::as_arg(&unassignedEqns), &fnptr!(intString, i32), &(literal!(", ")), &(literal!("\n")))?;
                    metamodelica::print(literal!("unassignedStates:\n"));
                    BackendDump::debuglst(metamodelica::AsArg::as_arg(&unassignedStates), &fnptr!(intString, i32), &(literal!(", ")), &(literal!("\n")))?;
                }
                ilst = List::fold1(metamodelica::AsArg::as_arg(&unassignedStates), &fnptr!(statesWithUnusedDerivative, i32, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<i32>), mt.clone(), metamodelica::nil())?;
                ilst = List::select1(ilst, (std::sync::Arc::new(move |__a0: i32, __a1: BackendDAE::Variables| isStateonIndex(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<bool> + 'static>), v.clone())?;
                let (_, (__pa0, _)) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::getInitialEqnsFromShared(&inShared), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(searchDerivativesExp, metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables))> + 'static>), (ilst, v.clone())))?;
                ilst = metamodelica::Own::own(__pa0);
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    metamodelica::print(literal!("states without used derivative:\n"));
                    BackendDump::debuglst(&ilst, &fnptr!(intString, i32), &(literal!(", ")), &(literal!("\n")))?;
                }
                (syst, shared, ass1, ass2, so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn) = handleundifferntiableMSS(intLe(((ilst).len() as i32), ((unassignedEqns).len() as i32)), &(ilst), metamodelica::AsArg::as_arg(&eqns), metamodelica::AsArg::as_arg(&unassignedStates), metamodelica::AsArg::as_arg(&unassignedEqns), inSystem, &inShared, inAss1.clone(), inAss2.clone(), metamodelica::AsArg::as_arg(&so), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
                { (iNotDiffableMSS, inSystem, inShared, inAss1, inAss2, iArg) = (notDiffableMSS.clone(), syst, shared, ass1.clone(), ass2.clone(), (so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), noofeqns.clone())); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn handleundifferntiableMSS(
    mut b: bool,
    mut statesWithUnusedDer: &metamodelica::List<i32>,
    mut inEqns: &metamodelica::List<i32>,
    mut unassignedStates: &metamodelica::List<i32>,
    mut unassignedEqns: &metamodelica::List<i32>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inAss1: metamodelica::Array<i32>,
    mut inAss2: metamodelica::Array<i32>,
    mut inStateOrd: &BackendDAE::StateOrder,
    mut inOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut imapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut imapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    BackendDAE::StateOrder,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> =
        <metamodelica::Ref<BackendDAE::Shared> as ::std::default::Default>::default();
    let mut outAss1: metamodelica::Array<i32> = Default::default();
    let mut outAss2: metamodelica::Array<i32> = Default::default();
    let mut outStateOrd: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    let mut outOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> =
        Default::default();
    let mut omapEqnIncRow: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut omapIncRowEqn: metamodelica::Array<i32> = Default::default();
    (
        osyst,
        oshared,
        outAss1,
        outAss2,
        outStateOrd,
        outOrgEqnsLst,
        omapEqnIncRow,
        omapIncRowEqn,
    ) = 'mc: {
        let __mc_input = (b, &**statesWithUnusedDer, inSystem.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, syst @ Deref @ BackendDAE::EqSystem { m: Some(_), mT: Some(_), .. }) => {
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut eqnslst: metamodelica::List<i32>;
                    let mut eqnslst1: metamodelica::List<i32>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut syst = (*syst).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::fold1(inEqns, &move |__a0: i32, __a1: BackendDAE::Variables, __a2: (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::List<i32>, BackendVarTransform::VariableReplacements)| replaceFinalVars(__a0, __a1, &__a2), BackendVariable::daeGlobalKnownVars(inShared), (syst.orderedEqs.clone(), metamodelica::nil(), BackendVarTransform::emptyReplacements()))?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqns = metamodelica::Own::own(__pa0);
                    eqnslst = metamodelica::Own::own(__pa1);
                    assign_field!(syst.orderedEqs = eqns.clone());
                    eqnslst1 = List::flatten(List::map1r(eqnslst.clone(), &arrayGet, imapEqnIncRow.clone())?)?;
                    ilst = List::map1r(eqnslst1.clone(), &arrayGet, inAss2.clone())?;
                    ilst = List::select1(ilst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    ass2 = List::fold1r(&eqnslst1, &*(Arc::new(arrayUpdate.clone())), -1, inAss2.clone())?;
                    ass1 = List::fold1r(&ilst, &*(Arc::new(arrayUpdate.clone())), -1, inAss1.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Replaced final Parameter in Eqns\n"));
                        metamodelica::print(literal!("Update Adjacency Matrix: "));
                        BackendDump::debuglst(&eqnslst, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared);
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared))?;
                    Ok((syst.clone(), inShared.clone(), ass1.clone(), ass2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, syst @ Deref @ BackendDAE::EqSystem { orderedVars: v, m: Some(m), mT: Some(mt), .. }) => {
                    let mut eqnslst1: metamodelica::List<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut syst = (*syst).clone();
                    varlst = List::map1r(statesWithUnusedDer.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), v.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Change varKind to algebraic for\n"));
                        BackendDump::printVarList(&varlst)?;
                    }
                    varlst = BackendVariable::setVarsKind(varlst.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    assign_field!(syst.orderedVars = BackendVariable::addVars(&varlst, syst.orderedVars.clone())?);
                    eqnslst1 = collectVarEqns(statesWithUnusedDer, mt.clone(), metamodelica::arrayLength(mt.clone()), metamodelica::arrayLength(m.clone()))?;
                    eqnslst1 = List::map1r(eqnslst1.clone(), &arrayGet, imapIncRowEqn.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Update Adjacency Matrix: "));
                        BackendDump::debuglst(&eqnslst1, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared);
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst1.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared))?;
                    Ok((syst.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, Deref @ metamodelica::ListNode::Cons { head: i, tail: ilst }, syst @ Deref @ BackendDAE::EqSystem { orderedVars: v, m: Some(m), mT: Some(mt), .. }) => {
                    let mut eqnslst1: metamodelica::List<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut syst = (*syst).clone();
                    var = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&v), i.clone())?;
                    varlst = list![var.clone()];
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Change varKind to algebraic for\n"));
                        BackendDump::printVarList(&varlst)?;
                    }
                    varlst = BackendVariable::setVarsKind(varlst.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    assign_field!(syst.orderedVars = BackendVariable::addVars(&varlst, v.clone())?);
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        varlst = List::map1r(ilst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), v.clone())?;
                        metamodelica::print(literal!("Other Candidates are\n"));
                        BackendDump::printVarList(&varlst)?;
                    }
                    eqnslst1 = collectVarEqns(&(list![i.clone()]), mt.clone(), metamodelica::arrayLength(mt.clone()), metamodelica::arrayLength(m.clone()))?;
                    eqnslst1 = List::map1r(eqnslst1.clone(), &arrayGet, imapIncRowEqn.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Update Adjacency Matrix: "));
                        BackendDump::debuglst(&eqnslst1, &fnptr!(intString, i32), &(literal!(" ")), &(literal!("\n")))?;
                    }
                    funcs = BackendDAEUtil::getFunctions(inShared);
                    (syst, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::updateAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), eqnslst1.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone(), BackendDAEUtil::isInitializationDAE(inShared))?;
                    Ok((syst.clone(), inShared.clone(), inAss1.clone(), inAss2.clone(), inStateOrd.clone(), inOrgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, Deref @ BackendDAE::EqSystem { orderedVars: v, m: Some(_), mT: Some(mt), .. }) => {
                    let mut ilst: metamodelica::List<i32>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut omapEqnIncRow: metamodelica::Array<metamodelica::List<i32>> = omapEqnIncRow.clone();
                    let mut omapIncRowEqn: metamodelica::Array<i32> = omapIncRowEqn.clone();
                    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = oshared.clone();
                    let mut outAss1: metamodelica::Array<i32> = outAss1.clone();
                    let mut outAss2: metamodelica::Array<i32> = outAss2.clone();
                    let mut outOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> = outOrgEqnsLst.clone();
                    let mut outStateOrd: BackendDAE::StateOrder = outStateOrd.clone();
                    ilst = Matching::getUnassigned(BackendVariable::varsSize(metamodelica::AsArg::as_arg(&v)), inAss1.clone(), metamodelica::nil())?;
                    ilst = List::fold1(&ilst, &fnptr!(statesWithUnusedDerivative, i32, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<i32>), mt.clone(), metamodelica::nil())?;
                    varlst = List::map1r(ilst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), v.clone())?;
                    let (_, (__pa0, _)) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::getInitialEqnsFromShared(inShared), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(searchDerivativesExp, metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<i32>, BackendDAE::Variables))> + 'static>), (ilst.clone(), v.clone())))?;
                    ilst = metamodelica::Own::own(__pa0);
                    ::match_deref::match_deref! { match &(ilst.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All unassignedStates without Derivative: ")); __mm_s.push_str(&*stringDelimitList(List::map(ilst.clone(), &fnptr!(intString, i32))?, literal!(", "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        BackendDump::printVarList(&varlst)?;
                    }
                    (syst, oshared, outAss1, outAss2, outStateOrd, outOrgEqnsLst, omapEqnIncRow, omapIncRowEqn) = handleundifferntiableMSS(intLe(((ilst).len() as i32), ((unassignedEqns).len() as i32)), &(ilst.clone()), inEqns, unassignedStates, unassignedEqns, inSystem.clone(), inShared, inAss1.clone(), inAss2.clone(), inStateOrd, inOrgEqnsLst.clone(), imapEqnIncRow.clone(), imapIncRowEqn.clone())?;
                    Ok(((syst.clone(), oshared.clone(), outAss1.clone(), outAss2.clone(), outStateOrd.clone(), outOrgEqnsLst.clone(), omapEqnIncRow.clone(), omapIncRowEqn.clone()), omapEqnIncRow.clone(), omapIncRowEqn.clone(), oshared.clone(), outAss1.clone(), outAss2.clone(), outOrgEqnsLst.clone(), outStateOrd.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            omapEqnIncRow = __wb0;
            omapIncRowEqn = __wb1;
            oshared = __wb2;
            outAss1 = __wb3;
            outAss2 = __wb4;
            outOrgEqnsLst = __wb5;
            outStateOrd = __wb6;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ BackendDAE::EqSystem { orderedVars: v, m: Some(_), mT: Some(_), .. }) => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    varlst = List::map1r(unassignedStates.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), v.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("unassignedStates\n"));
                        BackendDump::printVarList(&varlst)?;
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
    Ok((
        osyst,
        oshared,
        outAss1,
        outAss2,
        outStateOrd,
        outOrgEqnsLst,
        omapEqnIncRow,
        omapIncRowEqn,
    ))
}

fn replaceFinalVars(
    mut e: i32,
    mut vars: BackendDAE::Variables,
    mut inTpl: &(
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::List<i32>,
        BackendVarTransform::VariableReplacements,
    ),
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<i32>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut outTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::List<i32>,
        BackendVarTransform::VariableReplacements,
    );
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut changedEqns: metamodelica::List<i32>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut b: bool;
    let mut repl: BackendVarTransform::VariableReplacements;
    (eqns, changedEqns, repl) = inTpl.clone();
    eqn = BackendEquation::get(eqns.clone(), e)?;
    let (__pa0, (_, __pa1, __pa2)) = BackendEquation::traverseExpsOfEquation(
        eqn,
        (std::sync::Arc::new(replaceFinalVarsEqn)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
                    )> + 'static,
            >),
        (vars, false, repl),
    )?;
    eqn = metamodelica::Own::own(__pa0);
    b = metamodelica::Own::own(__pa1);
    repl = metamodelica::Own::own(__pa2);
    eqns = if (b) {
        BackendEquation::setAtIndex(eqns, e, eqn)?
    } else {
        eqns
    };
    changedEqns = List::consOnTrue(b, e, changedEqns);
    outTpl = (eqns, changedEqns, repl);
    Ok(outTpl)
}

fn replaceFinalVarsEqn(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    let mut b: bool;
    let (__pa0, ref __pa2 @ (_, ref __pa1, _)) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(
            replaceFinalVarsExp,
            metamodelica::Ref<DAE::Exp>,
            (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)
        ),
        inTpl,
    )?;
    e = metamodelica::Own::own(__pa0);
    b = metamodelica::Own::own(__pa1);
    tpl = metamodelica::Own::own(__pa2);
    (e, _) = ExpressionSimplify::condsimplify(b, e)?;
    Ok((e, tpl))
}

fn replaceFinalVarsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, repl)) => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut repl = (*repl).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (__pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    vlst = metamodelica::Own::own(__pa0);
                    let (__pa1, true) = (List::fold20(&vlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: BackendVarTransform::VariableReplacements, __a2: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(replaceFinalVarsGetExp(&__a0, __a1, __a2)) }, repl.clone(), false)?) else { return Err("pattern mismatch") };
                    repl = metamodelica::Own::own(__pa1);
                    let __pa2 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None)) {
                        (__pa2, true) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2 = metamodelica::Own::own(__pa2);
                    Ok((e2.clone(), (vars.clone(), true, repl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn replaceFinalVarsGetExp(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut b: bool,
) -> (BackendVarTransform::VariableReplacements, bool) {
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut b: bool = b;
    (repl, b) = 'mc: {
        let __mc_input = &**inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, bindExp: Some(exp), .. } => {
                    if !((BackendVariable::isFinalVar(inVar))) { return Err("guard") }
                    let mut repl1: BackendVarTransform::VariableReplacements;
                    repl1 = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), exp.clone(), None)?;
                    Ok((repl1.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, bindExp: None, values, .. } => {
                    if !((BackendVariable::isFinalVar(inVar))) { return Err("guard") }
                    let mut repl1: BackendVarTransform::VariableReplacements;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = DAEUtil::getStartAttrFail(values.clone())?;
                    repl1 = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), exp.clone(), None)?;
                    Ok((repl1.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((repl.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (repl, b)
}

pub(crate) fn getStructurallySingularSystemHandlerArg(
    mut inSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(
    BackendDAE::StateOrder,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    i32,
)> {
    let mut outArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    let mut dht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let mut so: BackendDAE::StateOrder;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut count: i32;
    if metamodelica::stringEq(&(Config::getIndexReductionMethod()?), &(literal!("uode"))) {
        so = openmodelica_backend_types::BackendDAE::StateOrder::NOSTATEORDER;
    } else {
        count = ((metamodelica::real_div_checked(
            metamodelica::OrderedFloat((8) as f64),
            metamodelica::OrderedFloat((3) as f64),
        )? * metamodelica::OrderedFloat(
            (BackendVariable::getNumStateVarFromVariables(inSystem.orderedVars.clone())?) as f64,
        ))
        .0
        .floor() as i32);
        if count == 0 {
            so = openmodelica_backend_types::BackendDAE::StateOrder::NOSTATEORDER;
        } else {
            ht = HashTableCG::emptyHashTableSized(count);
            dht = HashTable3::emptyHashTableSized(count);
            so = BackendDAE::StateOrder::STATEORDER {
                hashTable: ht,
                invHashTable: dht,
            };
        }
    }
    eqns = BackendEquation::getEqnsFromEqSystem(inSystem);
    outArg = (
        so,
        arrayCreate(BackendEquation::getNumberOfEquations(eqns.clone()), metamodelica::nil()),
        mapEqnIncRow.clone(),
        mapIncRowEqn.clone(),
        BackendEquation::getNumberOfEquations(eqns),
    );
    Ok(outArg)
}

// =============================================================================
// No State deselection Method.
// use the index 1/0 system as it is
// =============================================================================
pub(crate) fn noStateDeselection(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inArgs: &metamodelica::List<
        Option<(
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        )>,
    >,
) -> metamodelica::Ref<BackendDAE::BackendDAE> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = inDAE;
    outDAE
}

// =============================================================================
// dynamic state selection method
// see
// - Mattsson, S.E.; Söderlind, G.: A new technique for solving high-index differential-algebraic equations using dummy derivatives, Computer-Aided Control System Design, 1992. (CACSD),1992 IEEE Symposium on , pp.218-224, 17-19 Mar 1992
// - Mattsson, S.E.; Olsson, H; Elmqviste, H. Dynamic Selection of States in Dymola. In: Proceedings of the Modelica Workshop 2000, Lund, Sweden, Modelica Association, 23-24 Oct. 2000.
// - Mattsson, S.; Söderlind, G.: Index reduction in differential-Algebraic equations using dummy derivatives, SIAM J. Sci. Comput. 14, 677-692, 1993.
// =============================================================================
pub(crate) fn dynamicStateSelection(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inArgs: metamodelica::List<
        Option<(
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        )>,
    >,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    ht = HashTableCrIntToExp::emptyHashTable();
    (systs, shared, ht) = dynamicStateSelection_mapEqsystem(&systs, shared, inArgs, 1, ht)?;
    if intGt(BaseHashTable::hashTableCurrentSize(&ht), 0) {
        (systs, shared) = List::map1Fold(&systs, &replaceDummyDerivatives, ht, shared)?;
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs,
        shared: shared,
    });
    Ok(outDAE)
}

fn dynamicStateSelection_mapEqsystem(
    mut isysts: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut iargs: metamodelica::List<
        Option<(
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        )>,
    >,
    mut setIndex: i32,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut osysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    ) = iHt;
    let mut syst_: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oarg: Option<(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    )>;
    let mut arg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut args: metamodelica::List<
        Option<(
            BackendDAE::StateOrder,
            metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
            metamodelica::Array<metamodelica::List<i32>>,
            metamodelica::Array<i32>,
            i32,
        )>,
    > = iargs;
    let mut index: i32 = setIndex;
    for mut syst in &**isysts {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        oarg = metamodelica::Own::own(__pa0);
        args = metamodelica::Own::own(__pa1);
        if (oarg).is_some() {
            let __pa2 = ::match_deref::match_deref! { match &(oarg) {
                Some(__pa2) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            arg = metamodelica::Own::own(__pa2);
            (syst_, oshared, oHt, index) = dynamicStateSelectionWork(syst.clone(), oshared, &arg, oHt, index)?;
            osysts = metamodelica::cons(syst_, osysts);
        } else {
            osysts = metamodelica::cons(syst.clone(), osysts);
        }
    }
    osysts = metamodelica::Dangerous::listReverseInPlace(osysts);
    Ok((osysts, oshared, oHt))
}

fn dynamicStateSelectionWork(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inArg: &(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSetIndex: i32,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    let mut oSetIndex: i32 = iSetIndex;
    let mut so: BackendDAE::StateOrder;
    let mut orgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut numFreeStates: i32;
    let mut numOrgEqs: i32;
    (so, orgEqnsLst, mapEqnIncRow, mapIncRowEqn, _) = inArg.clone();
    if Array::all(orgEqnsLst.clone(), &fnptr!(listEmpty, _))? {
        osyst = inSystem;
        oshared = inShared;
        oHt = iHt;
    } else {
        match '__try0: {
            let __arc2 = inSystem.clone();
            let BackendDAE::EQSYSTEM { orderedVars: __pa1, .. } = &*__arc2;
            vars = metamodelica::Own::own(__pa1);
            let __arc4 = inShared.clone();
            let BackendDAE::SHARED {
                functionTree: __pa3, ..
            } = &*__arc4;
            funcs = metamodelica::Own::own(__pa3);
            orgEqnsLst = unwrap_break_err!(inlineOrgEqns(orgEqnsLst.clone(), &((Some(funcs.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE]))), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::BLT_DUMP.clone()), '__try0) {
                metamodelica::print(literal!(
                    "########################### STATE SELECTION ###########################\n"
                ));
            }
            numFreeStates = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(fnptr!(countStateCandidates, metamodelica::Ref<BackendDAE::Var>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, i32) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> + 'static>), 0), '__try0);
            numOrgEqs = unwrap_break_err!(countOrgEqns(orgEqnsLst.clone(), 0), '__try0);
            (osyst, oshared, oHt, oSetIndex) = unwrap_break_err!(selectStates(numFreeStates, numOrgEqs, inSystem.clone(), inShared.clone(), so.clone(), orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), iHt.clone(), iSetIndex), '__try0);
            Ok::<_, &'static str>((
                funcs.clone(),
                numFreeStates.clone(),
                numOrgEqs.clone(),
                oHt.clone(),
                oSetIndex.clone(),
                orgEqnsLst.clone(),
                oshared.clone(),
                osyst.clone(),
                vars.clone(),
            ))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
                funcs = __try0_o0;
                numFreeStates = __try0_o1;
                numOrgEqs = __try0_o2;
                oHt = __try0_o3;
                oSetIndex = __try0_o4;
                orgEqnsLst = __try0_o5;
                oshared = __try0_o6;
                osyst = __try0_o7;
                vars = __try0_o8;
            }
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![literal!("- IndexReduction.dynamicStateSelectionWork failed!")],
                )?;
                return Err(__try0_err);
            }
        }
    }
    Ok((osyst, oshared, oHt, oSetIndex))
}

fn countStateCandidates(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inCount: i32,
) -> (metamodelica::Ref<BackendDAE::Var>, i32) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outCount: i32;
    outCount = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            b = BackendVariable::varStateSelectAlways(&inVar);
            statecount = if (!(b)) {inCount + 1} else {inCount};
            statecount
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            b = BackendVariable::varStateSelectAlways(&inVar);
            statecount = if (b) {inCount + 1} else {inCount};
            statecount
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            statecount = diffcount.clone() + inCount;
            b = BackendVariable::varStateSelectAlways(&inVar);
            statecount = if (b) {statecount - 1} else {statecount};
            statecount
        },
        _ => {
            inCount
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outCount)
}

fn countStateCandidatesWithNever(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inCount: i32,
) -> (metamodelica::Ref<BackendDAE::Var>, i32) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outCount: i32;
    outCount = (::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            b = BackendVariable::varStateSelectNever(&inVar);
            statecount = if (b) {inCount + 1} else {inCount};
            statecount
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            b = BackendVariable::varStateSelectNever(&inVar);
            statecount = if (!(b)) {inCount + 1} else {inCount};
            statecount
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, .. }, .. } => {
            let mut statecount: i32;
            let mut b: bool;
            statecount = diffcount.clone() + inCount;
            b = BackendVariable::varStateSelectNever(&inVar);
            statecount = if (!(b)) {statecount - 1} else {statecount};
            statecount
        },
        _ => {
            inCount
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outCount)
}

fn countOrgEqns(
    mut inOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iCount: i32,
) -> Result<i32> {
    let mut oCount: i32 = iCount;
    let mut orgeqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut size: i32;
    let mut numEqs: i32;
    let mut e: i32 = 0;
    numEqs = metamodelica::arrayLength(inOrgEqns.clone());
    for mut e in 1..=numEqs {
        orgeqns = metamodelica::arrayGet(inOrgEqns.clone(), e)?;
        size = BackendEquation::equationLstSize(&orgeqns)?;
        oCount = oCount + size;
    }
    Ok(oCount)
}

fn inlineOrgEqns(
    mut inOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inA: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut outOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut orgeqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut e: i32 = 0;
    let mut numEqs: i32;
    outOrgEqns = inOrgEqns.clone();
    numEqs = metamodelica::arrayLength(inOrgEqns.clone());
    for mut e in 1..=numEqs {
        orgeqns = metamodelica::arrayGet(inOrgEqns.clone(), e)?;
        (orgeqns, _) = BackendInline::inlineEqs(&orgeqns, inA, metamodelica::nil(), false);
        metamodelica::arrayUpdate(outOrgEqns.clone(), e, orgeqns)?;
    }
    Ok(outOrgEqns)
}

fn replaceDerStatesStatesExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inOrder: BackendDAE::StateOrder,
) -> (metamodelica::Ref<DAE::Exp>, BackendDAE::StateOrder) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outOrder: BackendDAE::StateOrder;
    (outExp, outOrder) = 'mc: {
        let __mc_input = (&*inExp, inOrder.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, so) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                    dcr = getStateOrder(cr.clone(), so.clone())?;
                    e = Expression::crefExp(dcr.clone())?;
                    Ok((e.clone(), so.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inOrder.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outOrder)
}

fn highestOrderDerivatives(
    mut v: BackendDAE::Variables,
    mut iSo: BackendDAE::StateOrder,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendDAE::StateOrder,
)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut oSo: BackendDAE::StateOrder;
    (oSo, _, outVars) = BackendVariable::traverseBackendDAEVars(
        v.clone(),
        (std::sync::Arc::new(fnptr!(
            traversinghighestOrderDerivativesFinder,
            metamodelica::Ref<BackendDAE::Var>,
            (
                BackendDAE::StateOrder,
                BackendDAE::Variables,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::StateOrder,
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::StateOrder,
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                        ),
                    )> + 'static,
            >),
        (iSo, v, metamodelica::nil()),
    )?;
    Ok((outVars, oSo))
}

fn traversinghighestOrderDerivativesFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        BackendDAE::StateOrder,
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::StateOrder,
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::StateOrder,
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: None, .. }, .. }, (so, vars, varlst)) => {
                    Ok((v.clone(), (so.clone(), vars.clone(), metamodelica::cons(v.clone(), varlst.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::STATE { derName: Some(dcr), .. }, .. }, (so, vars, varlst)) => {
                    let mut b: bool;
                    let mut so = (*so).clone();
                    let mut varlst = (*varlst).clone();
                    b = BackendVariable::isState(dcr.clone(), metamodelica::AsArg::as_arg(&vars));
                    varlst = List::consOnTrue(!(b), v.clone(), varlst.clone());
                    so = addStateOrder(cr.clone(), dcr.clone(), so.clone())?;
                    Ok((v.clone(), (so.clone(), vars.clone(), varlst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn getVar(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    (v, _) = BackendVariable::getVarSingle(cr, vars)?;
    Ok(v)
}

/// Level,nStates,nStateCandidates,nUnassignedEquations,StateCandidates,ConstraintEqns,OtherVars,OtherEqns
pub type StateSets = metamodelica::List<(
    i32,
    i32,
    i32,
    i32,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)>;

fn reduceStateSets(
    mut iTplLst: &StateSets,
    mut idummyStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut odummyStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if !((iTplLst).is_empty()) {
        odummyStates = reduceStateSets2(iTplLst)?;
    } else {
        odummyStates = idummyStates;
    }
    Ok(odummyStates)
}

fn reduceStateSets2(mut iTplLst: &StateSets) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut dummyStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut tpl: (
        i32,
        i32,
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ) = (
        0,
        0,
        0,
        0,
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    );
    let mut rang: i32;
    let mut nStateCandidates: i32;
    let mut nUnassignedEquations: i32;
    let mut stateCandidates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    for mut tpl in &**iTplLst {
        let mut tpl = tpl.clone();
        (_, _, nStateCandidates, nUnassignedEquations, stateCandidates, _, _, _) = tpl;
        rang = nStateCandidates - nUnassignedEquations;
        (_, stateCandidates) = List::split(stateCandidates, rang)?;
        dummyStates = listAppend(stateCandidates, dummyStates);
    }
    Ok(dummyStates)
}

fn addStateSets(
    mut iTplLst: &StateSets,
    mut iSetIndex: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(i32, metamodelica::Ref<BackendDAE::EqSystem>)> {
    let mut oSetIndex: i32 = iSetIndex;
    let mut oSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    (oSetIndex, oSystem) = (::match_deref::match_deref! { match iTplLst {
        Deref @ metamodelica::ListNode::Nil => {
            (iSetIndex, inSystem)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut syst = inSystem.clone();
            let mut setIndex: i32;
            let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut vars: BackendDAE::Variables;
            let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
            (setIndex, vars, eqs, stateSets) = generateStateSets(iTplLst, iSetIndex, syst.orderedVars.clone(), syst.orderedEqs.clone(), syst.stateSets.clone())?;
            assign_field!(
                syst.orderedVars = vars,
                syst.orderedEqs = eqs,
                syst.stateSets = stateSets
            );
            (setIndex, syst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oSetIndex, oSystem))
}

fn generateStateSets(
    mut iTplLst: &StateSets,
    mut iSetIndex: i32,
    mut iVars: BackendDAE::Variables,
    mut iEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iStateSets: metamodelica::List<BackendDAE::StateSet>,
) -> Result<(
    i32,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<BackendDAE::StateSet>,
)> {
    let mut oSetIndex: i32 = iSetIndex;
    let mut oVars: BackendDAE::Variables = iVars;
    let mut oEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = iEqns;
    let mut oStateSets: metamodelica::List<BackendDAE::StateSet> = iStateSets;
    let mut tpl: (
        i32,
        i32,
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ) = (
        0,
        0,
        0,
        0,
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    );
    let mut setVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut aVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varJ: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut otherVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut stateCandidates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut crset: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crA: metamodelica::Ref<DAE::ComponentRef>;
    let mut crJ: metamodelica::Ref<DAE::ComponentRef>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut tyExpCrStates: metamodelica::Ref<DAE::Type>;
    let mut rang: i32;
    let mut nStateCandidates: i32;
    let mut nUnassignedEquations: i32;
    let mut level: i32;
    let mut recordSize: Option<i32>;
    let mut expcrA: metamodelica::Ref<DAE::Exp>;
    let mut mulAstates: metamodelica::Ref<DAE::Exp>;
    let mut mulAdstates: metamodelica::Ref<DAE::Exp>;
    let mut expset: metamodelica::Ref<DAE::Exp>;
    let mut expderset: metamodelica::Ref<DAE::Exp>;
    let mut expsetstart: metamodelica::Ref<DAE::Exp>;
    let mut expcrstates: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expcrdstates: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expcrset: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expcrdset: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expcrstatesstart: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut crstates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut op: DAE::Operator;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut deqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut cEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut b: bool;
    for mut tpl in &**iTplLst {
        let mut tpl = tpl.clone();
        (
            level,
            _,
            nStateCandidates,
            nUnassignedEquations,
            stateCandidates,
            cEqnsLst,
            otherVars,
            oEqnLst,
        ) = tpl;
        rang = nStateCandidates - nUnassignedEquations;
        b = intGt(rang, 1);
        (_, crset, setVars, crA, aVars, tp, crJ, varJ) =
            getSetVars(oSetIndex, rang, nStateCandidates, nUnassignedEquations, level)?;
        expcrstates = List::map(stateCandidates.clone(), &move |__a0: metamodelica::Ref<
            BackendDAE::Var,
        >| BackendVariable::varExp(&__a0))?;
        crstates = List::map(stateCandidates.clone(), &move |__a0: metamodelica::Ref<
            BackendDAE::Var,
        >|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        })?;
        expcrstatesstart = List::map(crstates, &makeStartExp)?;
        expcrdstates = List::map(expcrstates.clone(), &makeder)?;
        expcrset = List::map(crset.clone(), &Expression::crefExp)?;
        expcrdset = List::map(expcrset.clone(), &makeder)?;
        expcrA = Expression::crefExp(crA.clone())?;
        expcrA = metamodelica::Ref::new(DAE::Exp::CAST { ty: tp, exp: expcrA });
        tyExpCrStates = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_REAL_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                integer: nStateCandidates
            })],
        });
        op = if (b) {
            DAE::Operator::MUL_MATRIX_PRODUCT {
                ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                    dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: rang })],
                }),
            }
        } else {
            DAE::Operator::MUL_SCALAR_PRODUCT {
                ty: DAE::T_REAL_DEFAULT().clone(),
            }
        };
        mulAstates = metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: expcrA.clone(),
            operator: op.clone(),
            exp2: metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: tyExpCrStates.clone(),
                scalar: true,
                array: expcrstates,
            }),
        });
        (mulAstates, _) = Expression::extendArrExp(mulAstates, false);
        mulAdstates = metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: expcrA.clone(),
            operator: op.clone(),
            exp2: metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: tyExpCrStates.clone(),
                scalar: true,
                array: expcrdstates,
            }),
        });
        (mulAdstates, _) = Expression::extendArrExp(mulAdstates, false);
        expset = if (b) {
            metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                    dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: rang })],
                }),
                scalar: true,
                array: expcrset,
            })
        } else {
            (expcrset).head().cloned()?
        };
        expderset = if (b) {
            metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                    dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: rang })],
                }),
                scalar: true,
                array: expcrdset,
            })
        } else {
            (expcrdset).head().cloned()?
        };
        source = metamodelica::Ref::new(DAE::ElementSource {
            info: SourceInfo {
                fileName: literal!("stateselection"),
                isReadOnly: false,
                lineNumberStart: 0,
                columnNumberStart: 0,
                lineNumberEnd: 0,
                columnNumberEnd: 0,
                lastModification: metamodelica::OrderedFloat(0.0_f64),
            },
            partOfLst: metamodelica::nil(),
            instance: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
            connectEquationOptLst: metamodelica::nil(),
            typeLst: metamodelica::nil(),
            operations: metamodelica::nil(),
            comment: metamodelica::nil(),
        });
        tp = ComponentReference::crefTypeFull(&crA)?;
        tp = DAEUtil::expTypeElementType(&tp);
        if DAEUtil::expTypeComplex(&tp) {
            recordSize = Some(Expression::sizeOf(&tp));
        } else {
            recordSize = None;
        }
        eqn = if (b) {
            metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION {
                dimSize: list![rang],
                left: expset,
                right: mulAstates,
                source: source,
                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                recordSize: recordSize.clone(),
            })
        } else {
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: expset,
                scalar: mulAstates,
                source: source,
                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
            })
        };
        deqn = if (b) {
            metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION {
                dimSize: list![rang],
                left: expderset,
                right: mulAdstates,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                recordSize: recordSize,
            })
        } else {
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: expderset,
                scalar: mulAdstates,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
            })
        };
        expsetstart = metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: expcrA,
            operator: op,
            exp2: metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: tyExpCrStates,
                scalar: true,
                array: expcrstatesstart,
            }),
        });
        (expsetstart, _) = Expression::extendArrExp(expsetstart, false);
        (setVars, _) = List::map2Fold(&setVars, &setStartExp, expsetstart, rang, 1, metamodelica::nil())?;
        oVars = BackendVariable::addVars(&setVars, oVars)?;
        oEqns = BackendEquation::add(eqn, oEqns)?;
        oEqns = BackendEquation::add(deqn, oEqns)?;
        stateCandidates = List::map1(
            stateCandidates,
            &BackendVariable::setVarKind,
            openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE,
        )?;
        otherVars = List::map1(
            otherVars,
            &BackendVariable::setVarKind,
            openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE,
        )?;
        oStateSets = metamodelica::cons(
            BackendDAE::StateSet {
                index: oSetIndex,
                rang: rang,
                state: crset,
                crA: crA,
                varA: aVars,
                statescandidates: stateCandidates,
                ovars: otherVars,
                eqns: cEqnsLst,
                oeqns: oEqnLst,
                crJ: crJ,
                varJ: varJ,
                jacobian: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
            },
            oStateSets,
        );
        oSetIndex = oSetIndex + 1;
    }
    if Flags::isSet(Flags::BLT_DUMP.clone())? {
        BackendDump::dumpStateSets(&oStateSets, &(literal!("Generated StateSets:")))?;
    }
    Ok((oSetIndex, oVars, oEqns, oStateSets))
}

pub(crate) fn makeStartExp(mut inCref: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = Expression::crefExp(ComponentReference::crefPrefixStart(inCref))?;
    Ok(outExp)
}

fn setStartExp(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut startExp: metamodelica::Ref<DAE::Exp>,
    mut size: i32,
    mut iIndex: i32,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oIndex: i32 = iIndex;
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = if (intGt(size, 1)) {
        Expression::makeASUB(
            startExp,
            list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: iIndex })],
        )?
    } else {
        startExp
    };
    (e, _) = ExpressionSimplify::simplify(e)?;
    outVar = BackendVariable::setVarStartValue(inVar, e)?;
    oIndex = iIndex + 1;
    Ok((outVar, oIndex))
}

fn selectStates(
    mut nfreeStates: i32,
    mut nOrgEqns: i32,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut iSo: BackendDAE::StateOrder,
    mut orgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iMapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut iMapIncRowEqn: metamodelica::Array<i32>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSetIndex: i32,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    let mut oSetIndex: i32 = iSetIndex;
    (osyst, oshared, oHt, oSetIndex) = 'mc: {
        let __mc_input = &*inSystem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, .. }, .. } => {
                    if !((intEq(nfreeStates, nOrgEqns))) { return Err("guard") }
                    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut ne: i32;
                    let mut nv: i32;
                    let mut ass1 = (*ass1).clone();
                    let mut ass2 = (*ass2).clone();
                    eqnslst = List::flatten(orgEqnsLst.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?;
                    syst = BackendEquation::equationsAddDAE(&eqnslst, inSystem.clone())?;
                    (syst, ht) = addAllDummyStates(syst.clone(), iSo.clone(), iHt.clone())?;
                    funcs = BackendDAEUtil::getFunctions(&inShared);
                    (syst, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(&inShared))?;
                    ass1 = Array::expand(nfreeStates, ass1.clone(), -1)?;
                    ass2 = Array::expand(nOrgEqns, ass2.clone(), -1)?;
                    nv = BackendVariable::varsSize(&(BackendVariable::daeVars(&syst)));
                    ne = BackendDAEUtil::systemSize(&syst)?;
                    let true = (BackendDAEEXT::setAssignment(ne, nv, ass2.clone(), ass1.clone())) else { return Err("pattern mismatch") };
                    Matching::matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
                    BackendDAEEXT::matching(nv, ne, 5, -1, metamodelica::OrderedFloat(0.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    syst = BackendDAEUtil::setEqSystMatching(syst.clone(), metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }));
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::dumpEquationList(&eqnslst, &(literal!("No state selection needed for following equations:")))?;
                    }
                    Ok((syst.clone(), inShared.clone(), ht.clone(), iSetIndex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut setIndex: i32;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut hov: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut so: BackendDAE::StateOrder;
                    ErrorExt::setCheckpoint(literal!("DynamicStateSelection"));
                    (hov, so) = highestOrderDerivatives(BackendVariable::daeVars(&inSystem), iSo.clone())?;
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::dumpStateOrder(&so)?;
                    }
                    funcs = BackendDAEUtil::getFunctions(&inShared);
                    syst = replaceHigherDerivatives(inSystem.clone())?;
                    (syst, _, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(&inShared))?;
                    (syst, shared, ht, setIndex) = selectStatesWork(1, hov.clone(), syst.clone(), &inShared, &so, orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), iHt.clone(), iSetIndex)?;
                    ErrorExt::rollBack(literal!("DynamicStateSelection"));
                    Ok((syst.clone(), shared.clone(), ht.clone(), setIndex))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    ErrorExt::delCheckpoint(literal!("DynamicStateSelection"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osyst, oshared, oHt, oSetIndex))
}

fn selectStatesWork<'__b>(
    mut level: i32,
    mut iHov: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &'__b metamodelica::Ref<BackendDAE::Shared>,
    mut so: &'__b BackendDAE::StateOrder,
    mut iOrgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iMapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut iMapIncRowEqn: metamodelica::Array<i32>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSetIndex: i32,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inSystem.clone()) {
            _ if (Array::all(iOrgEqnsLst.clone(), &fnptr!(listEmpty, _))?) => {
                return Ok((inSystem, inShared.clone(), iHt, iSetIndex))
            },
            Deref @ BackendDAE::EqSystem { orderedVars: vars, matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, .. }, .. } => {
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnslst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut dummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut lov: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut hov: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                let mut mapIncRowEqn: metamodelica::Array<i32>;
                let mut nfreeStates: i32;
                let mut neqns: i32;
                let mut setIndex: i32;
                let mut ne: i32;
                let mut ne1: i32;
                let mut nv: i32;
                let mut nv1: i32;
                let mut stateSets: StateSets;
                let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                let mut m: metamodelica::Array<metamodelica::List<i32>>;
                let mut orgEqnsLst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                let mut repl: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                let mut ass1 = (*ass1).clone();
                let mut ass2 = (*ass2).clone();
                (eqnslst1, orgEqnsLst) = removeFirstOrgEqns(iOrgEqnsLst.clone())?;
                (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(&eqnslst1, (std::sync::Arc::new(replaceFinalVarsEqn) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool, BackendVarTransform::VariableReplacements))> + 'static>), (BackendVariable::daeGlobalKnownVars(inShared), false, BackendVarTransform::emptyReplacements()))?;
                (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(&eqnslst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(replaceDerStatesStatesExp, metamodelica::Ref<DAE::Exp>, BackendDAE::StateOrder)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, BackendDAE::StateOrder) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::StateOrder)> + 'static>), so.clone()))?;
                funcs = BackendDAEUtil::getFunctions(inShared);
                (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(&eqnslst, (std::sync::Arc::new(forceInlinEqn) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<AvlTreePathFunction::Tree>) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<AvlTreePathFunction::Tree>)> + 'static>), funcs.clone())?;
                (eqnslst, _) = InlineArrayEquations::getScalarArrayEqns(&eqnslst);
                (hov, ht) = List::map1Fold(&iHov, &fnptr!(getLevelStates, metamodelica::Ref<BackendDAE::Var>, i32, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))), level, HashTableCrIntToExp::emptyHashTable())?;
                (eqnslst, _) = BackendEquation::traverseExpsOfEquationList(&eqnslst, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), ht.clone()))?;
                (eqnslst1, _) = BackendEquation::traverseExpsOfEquationList(&eqnslst1, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), ht))?;
                varlst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut var in (hov.clone()).into_iter().cloned() {
                if !(BackendVariable::notVarStateSelectAlways(&(var.clone()), level)) { continue; }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                neqns = BackendEquation::equationLstSizeKeepAlgorithmAsOne(&eqnslst)?;
                nfreeStates = ((varlst).len() as i32);
                (dummyVars, stateSets) = selectStatesWork1(nfreeStates, varlst, neqns, &eqnslst, level, &inSystem, inShared, so, iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), &hov, &(metamodelica::nil()), &(metamodelica::nil()))?;
                lov = List::fold3(&iHov, &fnptr!(getlowerOrderDerivatives, metamodelica::Ref<BackendDAE::Var>, i32, BackendDAE::StateOrder, BackendDAE::Variables, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>), level, so.clone(), vars.clone(), metamodelica::nil())?;
                repl = HashTable2::emptyHashTable();
                (dummyVars, repl) = removeFirstOrderDerivatives(&dummyVars, metamodelica::AsArg::as_arg(&vars), so, repl)?;
                nv = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars));
                ne = BackendDAEUtil::systemSize(&inSystem)?;
                syst = BackendEquation::equationsAddDAE(&eqnslst1, inSystem)?;
                if metamodelica::stringEq(&(Flags::getConfigString(Flags::INDEX_REDUCTION_METHOD.clone())?), &(literal!("dummyDerivatives"))) && neqns < nfreeStates {
                    dummyVars = reduceStateSets(&stateSets, dummyVars)?;
                    stateSets = metamodelica::nil();
                }
                (setIndex, syst) = addStateSets(&stateSets, iSetIndex, syst)?;
                (syst, ht) = addDummyStates(&dummyVars, level, repl, syst, iHt)?;
                List::fold1(&iHov, &fnptr!(fixDerivativeIndex, metamodelica::Ref<BackendDAE::Var>, i32, BackendDAE::Variables), level, BackendVariable::daeVars(&syst))?;
                (syst, m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
                nv1 = BackendVariable::varsSize(&(BackendVariable::daeVars(&syst)));
                ne1 = BackendDAEUtil::systemSize(&syst)?;
                ass1 = Array::expand(nv1 - nv, ass1.clone(), -1)?;
                ass2 = Array::expand(ne1 - ne, ass2.clone(), -1)?;
                let true = (BackendDAEEXT::setAssignment(ne1, nv1, ass2.clone(), ass1.clone())) else { return Err("pattern mismatch") };
                Matching::matchingExternalsetAdjacencyMatrix(nv1, ne1, m.clone())?;
                BackendDAEEXT::matching(nv1, ne1, 5, -1, metamodelica::OrderedFloat(0.0_f64), 0);
                BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                syst = BackendDAEUtil::setEqSystMatching(syst, metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: metamodelica::nil() }));
                { (level, iHov, inSystem, inShared, so, iOrgEqnsLst, iMapEqnIncRow, iMapIncRowEqn, iHt, iSetIndex) = (level + 1, lov, syst, inShared, so, orgEqnsLst.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), ht, setIndex); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn removeFirstOrderDerivatives(
    mut iDummyVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iVars: &BackendDAE::Variables,
    mut so: &BackendDAE::StateOrder,
    mut iRepl: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oDummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut oRepl: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTable2::FuncHashCref,
            HashTable2::FuncCrefEqual,
            HashTable2::FuncCrefStr,
            HashTable2::FuncExpStr,
        ),
    ) = iRepl;
    for mut var in &**iDummyVars {
        (oDummyVars, oRepl) = (::match_deref::match_deref! { match &(var.clone()) {
            Deref @ BackendDAE::Var { varName: dcr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: cr, .. }, varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } if (!(intEq(System::strncmp(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?, arcstr::literal!(DAE::derivativeNamePrefix), 4), 0))) => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                exp = Expression::crefExp(cr.clone())?;
                exp = Expression::makePureBuiltinCall(literal!("der"), list![exp.clone()], Expression::r#typeof(exp)?);
                oRepl = BaseHashTable::add((dcr.clone(), exp), oRepl)?;
                (oDummyVars, oRepl)
            },
            _ => {
                (metamodelica::cons(var.clone(), oDummyVars), oRepl)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok((oDummyVars, oRepl))
}

fn getlowerOrderDerivatives(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut level: i32,
    mut so: BackendDAE::StateOrder,
    mut vars: BackendDAE::Variables,
    mut iVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Var>> {
    let mut oVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    oVars = 'mc: {
        let __mc_input = &*inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: dcr, varKind: BackendDAE::VarKind::STATE { index: diffindx, .. }, .. } => {
                    if !((intEq(diffindx.clone(), 1))) { return Err("guard") }
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    crlst = getDerStateOrder(dcr.clone(), so.clone())?;
                    vlst = List::map1(crlst.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: BackendDAE::Variables| getVar(&__a0, &__a1), vars.clone())?;
                    Ok(listAppend(vlst.clone(), iVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffindx, .. }, .. } => {
                    Ok(List::consOnTrue(intGt(diffindx.clone(), level), inVar.clone(), iVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oVars
}

fn fixDerivativeIndex(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut level: i32,
    mut iVars: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut oVars: BackendDAE::Variables;
    oVars = 'mc: {
        let __mc_input = &*inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffindx, derName, natural }, .. } => {
                    let mut vars: BackendDAE::Variables;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut diffindx = (*diffindx).clone();
                    let true = (intGt(diffindx.clone(), level)) else { return Err("pattern mismatch") };
                    diffindx = diffindx.clone() - level;
                    v = BackendVariable::setVarKind(inVar.clone(), BackendDAE::VarKind::STATE { index: diffindx.clone(), derName: derName.clone(), natural: natural.clone() })?;
                    vars = BackendVariable::addVar(v.clone(), iVars.clone())?;
                    Ok(vars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oVars
}

fn selectStatesWork1<'__b>(
    mut nfreeStates: i32,
    mut statecandidates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut neqns: i32,
    mut eqnslst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut level: i32,
    mut inSystem: &'__b metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &'__b metamodelica::Ref<BackendDAE::Shared>,
    mut so: &'__b BackendDAE::StateOrder,
    mut iMapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut iMapIncRowEqn: metamodelica::Array<i32>,
    mut iHov: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inDummyVars: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iStateSets: &'__b StateSets,
) -> Result<(metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, StateSets)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inSystem {
            _ if (intEq(nfreeStates, neqns)) => {
                return Ok((statecandidates, iStateSets.clone()))
            },
            Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, m: Some(m), mT: Some(mT), matching: Deref @ BackendDAE::Matching::MATCHING { ass1, ass2, .. }, .. } if (intGt(nfreeStates, 1) && !(intGt(neqns, nfreeStates))) => {
                let mut dummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut stateVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                let mut mapIncRowEqn: metamodelica::Array<i32>;
                let mut nv: i32;
                let mut nv1: i32;
                let mut ne: i32;
                let mut ne1: i32;
                let mut neqnarr: i32;
                let mut hovvars: BackendDAE::Variables;
                let mut eqns1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut me: metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>;
                let mut meT: metamodelica::Array<metamodelica::List<(i32, BackendDAE::Solvability, metamodelica::List<metamodelica::Ref<DAE::Constraint>>)>>;
                let mut stateSets: StateSets;
                let mut indexmap: metamodelica::Array<i32>;
                let mut invindexmap: metamodelica::Array<i32>;
                let mut vec1: metamodelica::Array<i32>;
                let mut vec2: metamodelica::Array<i32>;
                let mut ilst: metamodelica::List<i32>;
                let mut unassigned: metamodelica::List<i32>;
                let mut m1: metamodelica::Array<metamodelica::List<i32>>;
                let mut mT1: metamodelica::Array<metamodelica::List<i32>>;
                let mut comps: metamodelica::List<metamodelica::List<i32>>;
                let mut eqnslst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut states: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
                let mut dstates: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
                let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                let mut vars = (*vars).clone();
                let mut eqns = (*eqns).clone();
                let mut m = (*m).clone();
                let mut mT = (*mT).clone();
                hovvars = BackendVariable::listVar1(&statecandidates)?;
                eqns1 = BackendEquation::listEquation(eqnslst)?;
                syst = BackendDAEUtil::createEqSystem(hovvars.clone(), eqns1, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                (me, meT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst, inShared, false)?;
                m1 = adjacencyMatrixfromEnhancedStrict(me.clone(), hovvars.clone())?;
                mT1 = AdjacencyMatrix::transposeAdjacencyMatrix(m1.clone(), nfreeStates)?;
                hovvars = sortStateCandidatesVars(&hovvars, &(BackendVariable::daeVars(inSystem)), Some(mT1.clone()))?;
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########## Try static state selection ##########\n")); __mm_s.push_str(&*literal!("Try to select dummy vars with natural matching (newer)\n")); __mm_s.push_str(&*literal!("Select ")); __mm_s.push_str(&*intString(((eqnslst).len() as i32))); __mm_s.push_str(&*literal!(" dummy states from ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(&hovvars))); __mm_s.push_str(&*literal!(" candidates.\n")); ArcStr::from(__mm_s) });
                    BackendDump::dumpVariables(&hovvars, &(literal!("Highest order derivatives (state candidates):")))?;
                    BackendDump::dumpEquationList(eqnslst, &(literal!("Constraint equations:")))?;
                }
                nv = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars));
                ne = BackendEquation::equationArraySize(eqns.clone())?;
                neqnarr = BackendEquation::getNumberOfEquations(eqns.clone());
                ne1 = ne + neqns;
                indexmap = arrayCreate(nfreeStates + nv, -1);
                invindexmap = arrayCreate(nfreeStates, -1);
                nv1 = nv + nfreeStates;
                let (__pa0, (__pa1, __pa2, _, _, _, _)) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, metamodelica::List<i32>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(getStateIndexes(__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, metamodelica::List<i32>)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Array<i32>, metamodelica::Array<i32>, i32, i32, BackendDAE::Variables, metamodelica::List<i32>))> + 'static>), (indexmap.clone(), invindexmap.clone(), 1, nv, hovvars.clone(), metamodelica::nil()))?;
                vars = metamodelica::Own::own(__pa0);
                indexmap = metamodelica::Own::own(__pa1);
                invindexmap = metamodelica::Own::own(__pa2);
                m1 = arrayCreate(ne1, metamodelica::nil());
                mT1 = arrayCreate(nv1, metamodelica::nil());
                mapEqnIncRow = Array::expand(neqns, iMapEqnIncRow.clone(), metamodelica::nil())?;
                mapIncRowEqn = Array::expand(neqns, iMapIncRowEqn.clone(), -1)?;
                getAdjacencyMatrixSelectStates(ne, m1.clone(), mT1.clone(), m.clone(), indexmap.clone())?;
                funcs = BackendDAEUtil::getFunctions(inShared);
                getAdjacencyMatrixLevelEquations(eqnslst, metamodelica::AsArg::as_arg(&vars), neqnarr, ne, m1.clone(), mT1.clone(), m.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), indexmap.clone(), funcs.clone(), BackendDAEUtil::isInitializationDAE(inShared))?;
                vec1 = Array::expand(nfreeStates, ass1.clone(), -1)?;
                vec2 = Array::expand(neqns, ass2.clone(), -1)?;
                let true = (BackendDAEEXT::setAssignment(nv1, ne1, vec1.clone(), vec2.clone())) else { return Err("pattern mismatch") };
                Matching::matchingExternalsetAdjacencyMatrix(ne1, nv1, mT1.clone())?;
                BackendDAEEXT::matching(ne1, nv1, 3, -1, metamodelica::OrderedFloat(0.0_f64), 0);
                BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
                comps = Sorting::TarjanTransposed(mT1.clone(), vec2.clone())?;
                comps = List::select1(comps, (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(selectBlock(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), ne)?;
                ilst = List::fold1(&comps, &getCompsExtraEquations, ne, metamodelica::nil())?;
                ilst = List::map1r(ilst, &arrayGet, iMapIncRowEqn.clone())?;
                ilst = List::uniqueIntN(&ilst, ne)?;
                eqnslst1 = BackendEquation::getList(ilst, eqns.clone())?;
                ilst = List::fold2(&comps, &getCompsExtraVars, nv, vec2.clone(), metamodelica::nil())?;
                vlst = List::map1r(ilst, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                eqns = BackendEquation::listEquation(eqnslst)?;
                eqns = BackendEquation::addList(&eqnslst1, eqns.clone())?;
                vars = BackendVariable::listVar1(&vlst)?;
                vars = BackendVariable::addVars(&(BackendVariable::varList(&hovvars)?), vars.clone())?;
                syst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                (me, meT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst, inShared, false)?;
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    BackendDump::dumpAdjacencyMatrixEnhanced(me.clone())?;
                    metamodelica::print(literal!("\n"));
                    BackendDump::dumpAdjacencyMatrixTEnhanced(meT.clone())?;
                }
                m = adjacencyMatrixfromEnhancedStrict(me.clone(), vars.clone())?;
                nv = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars));
                ne = BackendEquation::equationArraySize(eqns.clone())?;
                mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv)?;
                Matching::matchingExternalsetAdjacencyMatrix(ne, nv, mT.clone())?;
                BackendDAEEXT::matching(ne, nv, 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
                vec1 = arrayCreate(nv, -1);
                vec2 = arrayCreate(ne, -1);
                BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
                (dstates, states, vec1, vec2) = forceStateSelectNever(vec1.clone(), vec2.clone(), vars.clone(), eqns.clone(), me.clone(), inShared, so.clone())?;
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    metamodelica::print(literal!("\n"));
                    BackendDump::dumpMatchingVars(vec1.clone())?;
                    metamodelica::print(literal!("\n"));
                    BackendDump::dumpMatchingEqns(vec2.clone())?;
                }
                (dstates, _) = checkAssignment(1, nv, vec1.clone(), metamodelica::AsArg::as_arg(&vars))?;
                dummyVars = List::map1r(List::map(dstates, &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                stateVars = List::map1r(List::map(states, &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                dummyVars = List::select(dummyVars, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>))?;
                unassigned = Matching::getUnassigned(ne, vec2.clone(), metamodelica::nil())?;
                Matching::getAssigned(ne, vec2.clone(), metamodelica::nil())?;
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    if (unassigned).is_empty() {
                        metamodelica::print(literal!("Perfect Matching, no dynamic index reduction needed! There are no unassigned equations.\n\n"));
                        if Flags::isSet(Flags::BLT_DUMP.clone())? {
                            BackendDump::dumpVarList(&dummyVars, &(literal!("Selected dummy states:")))?;
                            BackendDump::dumpVarList(&stateVars, &(literal!("Selected continuous states:")))?;
                        }
                    } else {
                        metamodelica::print(literal!("No perfect matching possible, dynamic index reduction needed.\n"));
                        unassigned = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut unassigned_eq in (unassigned).into_iter().cloned() {
                let __x = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), unassigned_eq.clone())?).clone(); __elt});
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                        BackendDump::dumpEquationList(&(BackendEquation::getEquationArraySubsetLst(eqns.clone(), &unassigned)?), &(literal!("Unassigned equations:")))?;
                        BackendDump::dumpVarList(&dummyVars, &(literal!("Statically selected dummy states:")))?;
                        metamodelica::print(literal!("\n"));
                    }
                }
                syst = BackendDAEUtil::setEqSystMatching(syst, metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: vec1.clone(), ass2: vec2.clone(), comps: metamodelica::nil() }));
                (syst, m, mT, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst, openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(funcs), BackendDAEUtil::isInitializationDAE(inShared))?;
                comps = partitionSystem(m.clone(), mT.clone())?;
                (vlst, _, stateSets) = processComps4New(&comps, nv, ne, vars.clone(), eqns.clone(), m.clone(), mT.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), vec2.clone(), vec1.clone(), level, inShared, iStateSets.clone())?;
                vlst = List::select(vlst, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>))?;
                return Ok((listAppend(dummyVars, vlst), stateSets))
            },
            _ if (intGt(neqns, nfreeStates)) => {
                let mut dummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut nv: i32;
                let mut hovvars: BackendDAE::Variables;
                let mut stateSets: StateSets;
                let mut msg: ArcStr;
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    hovvars = BackendVariable::listVar1(&statecandidates)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########## Try static state selection ##########\n")); __mm_s.push_str(&*literal!("Try to select dummy vars with natural matching (newer)\n")); __mm_s.push_str(&*literal!("Select ")); __mm_s.push_str(&*intString(((eqnslst).len() as i32))); __mm_s.push_str(&*literal!(" dummy states from ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(&hovvars))); __mm_s.push_str(&*literal!(" andidates.\n")); ArcStr::from(__mm_s) });
                    BackendDump::dumpVariables(&hovvars, &(literal!("Highest order derivatives (state candidates):")))?;
                    BackendDump::dumpEquationList(eqnslst, &(literal!("Constraint equations:")))?;
                }
                msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It is not possible to select continuous time states because Number of Equations ")); __mm_s.push_str(&*intString(neqns)); __mm_s.push_str(&*literal!(" greater than number of States ")); __mm_s.push_str(&*intString(nfreeStates)); __mm_s.push_str(&*literal!(" to select from.")); ArcStr::from(__mm_s) };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg])?;
                nv = ((iHov).len() as i32);
                if !(intGe(nv, neqns)) {
                    return Err("fail");
                }
                { (nfreeStates, statecandidates, neqns, eqnslst, level, inSystem, inShared, so, iMapEqnIncRow, iMapIncRowEqn, iHov, inDummyVars, iStateSets) = (nv, iHov.clone(), neqns, eqnslst, level, inSystem, inShared, so, iMapEqnIncRow.clone(), iMapIncRowEqn.clone(), iHov, inDummyVars, iStateSets); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn forceStateSelectNever(
    mut vec_old1: metamodelica::Array<i32>,
    mut vec_old2: metamodelica::Array<i32>,
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut so: BackendDAE::StateOrder,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
)> {
    let mut dummyStates: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut states: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut vec1: metamodelica::Array<i32> = vec_old1.clone();
    let mut vec2: metamodelica::Array<i32> = vec_old2;
    let mut nv: i32;
    let mut nv2: i32;
    let mut ne: i32;
    let mut never_i: i32;
    let mut eq_i: i32;
    let mut old_i: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let mut vec_res1: metamodelica::Array<i32>;
    let mut vec_res2: metamodelica::Array<i32>;
    let mut neverVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut neverVarsArray: BackendDAE::Variables;
    let mut neverIdx: metamodelica::List<i32> = metamodelica::nil();
    let mut syst2: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut me2: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut tplLst: metamodelica::List<(i32, i32)>;
    let mut msg: ArcStr;
    nv = BackendVariable::varsSize(&vars);
    ne = BackendEquation::equationArraySize(eqns.clone())?;
    let BackendDAE::STATEORDER {
        invHashTable: __pa0, ..
    } = (so.clone())
    else {
        return Err("pattern mismatch");
    };
    ht = metamodelica::Own::own(__pa0);
    (dummyStates, states) = checkAssignment(1, nv, vec_old1.clone(), &vars)?;
    for mut state in &*states {
        var = BackendVariable::getVarAt(&vars, Util::tuple22(state.clone()))?;
        if BackendVariable::varStateSelectNever(&var) && !(BaseHashTable::hasKey(BackendVariable::varCref(&var), &ht)?)
        {
            neverVars = metamodelica::cons(var, neverVars);
            neverIdx = metamodelica::cons(Util::tuple22(state.clone()), neverIdx);
        }
    }
    if !((neverVars).is_empty()) {
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            BackendDump::dumpVarList(
                &neverVars,
                &(literal!("StateSelect.never variables that will tried to be forced as dummys")),
            )?;
        }
        if Matching::anyUnassigned(ne, vec2.clone())? {
            m = adjacencyMatrixfromEnhanced(me.clone(), vars.clone(), so.clone())?;
            mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv)?;
            BackendDAEEXT::setAssignment(ne, nv, vec2.clone(), vec1.clone());
            Matching::matchingExternalsetAdjacencyMatrix(ne, nv, mT.clone())?;
            BackendDAEEXT::matching(ne, nv, 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
            BackendDAEEXT::getAssignment(vec1.clone(), vec2.clone())?;
            (dummyStates, states) = checkAssignment(1, nv, vec1.clone(), &vars)?;
            neverVars = metamodelica::nil();
            neverIdx = metamodelica::nil();
            for mut state in &*states {
                var = BackendVariable::getVarAt(&vars, Util::tuple22(state.clone()))?;
                if BackendVariable::varStateSelectNever(&var)
                    && !(BaseHashTable::hasKey(BackendVariable::varCref(&var), &ht)?)
                {
                    neverVars = metamodelica::cons(var, neverVars);
                    neverIdx = metamodelica::cons(Util::tuple22(state.clone()), neverIdx);
                }
            }
        }
        if !((neverVars).is_empty()) {
            neverVarsArray = BackendVariable::listVar1(&neverVars)?;
            nv2 = BackendVariable::varsSize(&neverVarsArray);
            syst2 = BackendDAEUtil::createEqSystem(
                neverVarsArray.clone(),
                eqns,
                metamodelica::nil(),
                openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                BackendEquation::emptyEqns(),
            );
            (me2, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst2, inShared, false)?;
            m = adjacencyMatrixfromEnhancedPartial(me2.clone(), vars.clone(), neverVarsArray, vec2.clone(), so)?;
            if !(AdjacencyMatrix::isEmpty(m.clone())) {
                mT = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), nv2)?;
                vec_res1 = arrayCreate(nv2, -1);
                vec_res2 = arrayCreate(ne, -1);
                BackendDAEEXT::setAssignment(ne, nv2, vec_res2.clone(), vec_res1.clone());
                Matching::matchingExternalsetAdjacencyMatrix(ne, nv2, mT.clone())?;
                BackendDAEEXT::matching(ne, nv2, 3, -1, metamodelica::OrderedFloat(1.0_f64), 1);
                BackendDAEEXT::getAssignment(vec_res1.clone(), vec_res2.clone())?;
                tplLst = List::zip(neverIdx.clone(), List::intRange(((neverIdx).len() as i32)));
                for mut tpl in &*tplLst {
                    (never_i, eq_i) = tpl.clone();
                    {
                        let __cell1 = ({
                            let __elt = (*metamodelica::index_checked(&vec_res1.borrow(), eq_i)?).clone();
                            __elt
                        });
                        let __idx1 = never_i;
                        *metamodelica::index_mut_checked(&mut vec1.clone().borrow_mut(), __idx1)? = __cell1;
                    }
                    old_i = ({
                        let __elt = (*metamodelica::index_checked(
                            &vec2.borrow(),
                            ({
                                let __elt = (*metamodelica::index_checked(&vec_res1.borrow(), eq_i)?).clone();
                                __elt
                            }),
                        )?)
                        .clone();
                        __elt
                    });
                    {
                        let __cell2 = never_i;
                        let __idx2 = ({
                            let __elt = (*metamodelica::index_checked(&vec_res1.borrow(), eq_i)?).clone();
                            __elt
                        });
                        *metamodelica::index_mut_checked(&mut vec2.clone().borrow_mut(), __idx2)? = __cell2;
                    }
                    if !(intEq(old_i, -1)) {
                        {
                            let __cell3 = -1;
                            let __idx3 = old_i;
                            *metamodelica::index_mut_checked(&mut vec1.clone().borrow_mut(), __idx3)? = __cell3;
                        }
                    }
                }
            }
            neverVars = metamodelica::nil();
            neverIdx = metamodelica::nil();
            for mut state in &*states {
                var = BackendVariable::getVarAt(&vars, Util::tuple22(state.clone()))?;
                if BackendVariable::varStateSelectNever(&var)
                    && BackendVariable::isNaturalState(&var)
                    && !(BaseHashTable::hasKey(BackendVariable::varCref(&var), &ht)?)
                {
                    neverVars = metamodelica::cons(var, neverVars);
                    neverIdx = metamodelica::cons(Util::tuple22(state.clone()), neverIdx);
                }
            }
            if !((neverVars).is_empty()) {
                msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*BackendDump::varListStringShort(&neverVars, &(literal!("")))?);
                    __mm_s.push_str(&*literal!("They could not be forced to be statically selected as dummys, this could lead to errors during simulation, please use -d=bltdump for more information.\n"));
                    ArcStr::from(__mm_s)
                };
                Error::addMessage(Error::STATE_STATESELECT_NEVER_FORCED.clone(), list![msg])?;
            }
        }
        if Flags::isSet(Flags::BLT_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n###################################\n"));
                __mm_s.push_str(&*literal!("INCLUDES FORCED STATESELECT.NEVER()\n"));
                __mm_s.push_str(&*literal!("###################################\n"));
                ArcStr::from(__mm_s)
            });
        }
        (dummyStates, states) = checkAssignment(1, nv, vec1.clone(), &vars)?;
    }
    Ok((dummyStates, states, vec1, vec2))
}

fn selectBlock<'__b>(mut comp: &'__b metamodelica::List<i32>, mut ne: i32) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match comp {
        Deref @ metamodelica::ListNode::Nil => {
            false
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
            b = if (intLe(c.clone(), ne)) {selectBlock(rest, ne)} else {true};
            b
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn getCompsExtraEquations(
    mut comp: metamodelica::List<i32>,
    mut neqns: i32,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32>;
    let mut eqns: metamodelica::List<i32>;
    eqns = List::select1(
        comp,
        (std::sync::Arc::new(fnptr!(intLe, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        neqns,
    )?;
    oAcc = listAppend(eqns, iAcc);
    Ok(oAcc)
}

fn getCompsExtraVars(
    mut comp: metamodelica::List<i32>,
    mut nvars: i32,
    mut ass2: metamodelica::Array<i32>,
    mut iAcc: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oAcc: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    vars = List::map1r(comp, &arrayGet, ass2.clone())?;
    vars = List::select1(
        vars,
        (std::sync::Arc::new(fnptr!(intLe, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        nvars,
    )?;
    vars = List::select1(
        vars,
        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        0,
    )?;
    oAcc = listAppend(vars, iAcc);
    Ok(oAcc)
}

fn dumpBlock(
    mut comp: metamodelica::List<i32>,
    mut iMapIncRowEqn: metamodelica::Array<i32>,
    mut nvars: i32,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<()> {
    let mut eqns: metamodelica::List<i32>;
    let mut ilst: metamodelica::List<i32>;
    let mut ilst1: metamodelica::List<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut invindexmap: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*syst)) {
        Deref @ BackendDAE::EqSystem { m: Some(__pa0), matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa1, ass2: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    invindexmap = metamodelica::Own::own(__pa1);
    ass2 = metamodelica::Own::own(__pa2);
    eqns = List::map1r(comp.clone(), &arrayGet, iMapIncRowEqn.clone())?;
    eqns = List::uniqueIntN(&eqns, BackendDAEUtil::equationArraySizeDAE(syst))?;
    ilst = List::map1r(comp, &arrayGet, ass2.clone())?;
    (ilst1, ilst) = List::split1OnTrue(&ilst, &fnptr!(intGt, i32, i32), nvars)?;
    ilst1 = List::map1(ilst1, &fnptr!(intSub, i32, i32), nvars)?;
    ilst1 = List::map1r(ilst1, &arrayGet, invindexmap.clone())?;
    ilst1 = listAppend(ilst, ilst1);
    metamodelica::print(literal!("##########################\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*BackendDump::dumpMarkedVars(syst, ilst1)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(BackendDump::dumpMarkedEqns(syst, eqns)?);
    Ok(())
}

fn getStateIndexes(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        i32,
        i32,
        BackendDAE::Variables,
        metamodelica::List<i32>,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        i32,
        i32,
        BackendDAE::Variables,
        metamodelica::List<i32>,
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        i32,
        i32,
        BackendDAE::Variables,
        metamodelica::List<i32>,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (&*inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::STATE { .. }, .. }, (stateindexs, invmap, indx, nv, hov, derstatesindexs)) => {
                    let mut s: i32;
                    let mut newindx: i32;
                    (_, s) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&hov))?;
                    newindx = nv.clone() + s;
                    metamodelica::arrayUpdate(stateindexs.clone(), indx.clone(), newindx)?;
                    metamodelica::arrayUpdate(invmap.clone(), s, indx.clone())?;
                    Ok((inVar.clone(), (stateindexs.clone(), invmap.clone(), indx.clone() + 1, nv.clone(), hov.clone(), metamodelica::cons(indx.clone(), derstatesindexs.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (stateindexs, invmap, indx, nv, hov, derstatesindexs)) => {
                    Ok((inVar.clone(), (stateindexs.clone(), invmap.clone(), indx.clone() + 1, nv.clone(), hov.clone(), derstatesindexs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn getAdjacencyMatrixSelectStates(
    mut nEqns: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mo: metamodelica::Array<metamodelica::List<i32>>,
    mut stateindexs: metamodelica::Array<i32>,
) -> Result<()> {
    let __ab_mo = mo.borrow();
    let mut row: metamodelica::List<i32>;
    let mut negrow: metamodelica::List<i32>;
    for mut i in ({
        let __s = nEqns;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        row = (*metamodelica::index_checked(&__ab_mo, i)?).clone();
        row = List::map1(row, &replaceStateIndex, stateindexs.clone())?;
        metamodelica::arrayUpdate(m.clone(), i, row.clone())?;
        (row, negrow) = List::split1OnTrue(&row, &fnptr!(intGt, i32, i32), 0)?;
        List::fold1(&row, &Array::consToElement, i, mT.clone())?;
        row = List::map(negrow, &fnptr!(intAbs, i32))?;
        List::fold1(&row, &Array::consToElement, -(i), mT.clone())?;
    }
    Ok(())
}

fn replaceStateIndex(mut iR: i32, mut stateindexs: metamodelica::Array<i32>) -> Result<i32> {
    let __ab_stateindexs = stateindexs.borrow();
    let mut oR: i32;
    let mut s: i32;
    let mut r: i32;
    oR = iR;
    if !(intGt(iR, 0)) {
        r = intAbs(iR);
        s = (*metamodelica::index_checked(&__ab_stateindexs, r)?).clone();
        if intGt(s, 0) {
            oR = s;
        }
    }
    Ok(oR)
}

fn getAdjacencyMatrixLevelEquations(
    mut iEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut vars: &BackendDAE::Variables,
    mut index: i32,
    mut sindex: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut om: metamodelica::Array<metamodelica::List<i32>>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut stateindexs: metamodelica::Array<i32>,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut isInitial: bool,
) -> Result<()> {
    let mut row: metamodelica::List<i32>;
    let mut rowindxs: metamodelica::List<i32>;
    let mut negrow: metamodelica::List<i32>;
    let mut i1: i32;
    let mut rowSize: i32;
    let mut size: i32;
    let mut idx: i32 = index;
    let mut sidx: i32 = sindex;
    for mut e in &**iEqns {
        (row, size) = BackendDAEUtil::adjacencyRow(
            metamodelica::AsArg::as_arg(&e),
            vars,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(functionTree.clone()),
            &(metamodelica::nil()),
            isInitial,
        )?;
        row = BackendDAEUtil::uniqueRow(row)?;
        rowSize = sidx + size;
        i1 = idx + 1;
        rowindxs = List::intRange2(sidx + 1, rowSize);
        List::fold1r(&rowindxs, &*(Arc::new(arrayUpdate.clone())), i1, mapIncRowEqn.clone())?;
        metamodelica::arrayUpdate(mapEqnIncRow.clone(), i1, rowindxs.clone())?;
        row = List::map1(row, &replaceStateIndex, stateindexs.clone())?;
        List::fold1r(&rowindxs, &*(Arc::new(arrayUpdate.clone())), row.clone(), m.clone())?;
        (row, negrow) = List::split1OnTrue(&row, &fnptr!(intGt, i32, i32), 0)?;
        List::fold1(&row, &Array::appendToElement, rowindxs.clone(), mT.clone())?;
        row = List::map(negrow, &fnptr!(intAbs, i32))?;
        rowindxs = List::map(rowindxs, &fnptr!(intNeg, i32))?;
        List::fold1(&row, &Array::appendToElement, rowindxs, mT.clone())?;
        idx = i1;
        sidx = rowSize;
    }
    Ok(())
}

fn partitionSystem(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut systs: metamodelica::List<metamodelica::List<i32>>;
    let mut rowmarkarr: metamodelica::Array<i32>;
    let mut collmarkarr: metamodelica::Array<i32>;
    let mut nsystems: i32;
    let mut neqns: i32;
    let mut systsarr: metamodelica::Array<metamodelica::List<i32>>;
    neqns = metamodelica::arrayLength(m.clone());
    rowmarkarr = arrayCreate(neqns, 0);
    collmarkarr = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
    nsystems = partitionSystem1(neqns, m.clone(), mT.clone(), rowmarkarr.clone(), collmarkarr.clone(), 1)?;
    systsarr = arrayCreate(nsystems, metamodelica::nil());
    systsarr = partitionSystemSplitt(neqns, rowmarkarr.clone(), systsarr.clone())?;
    systs = systsarr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    Ok(systs)
}

fn partitionSystem1(
    mut index: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarkarr: metamodelica::Array<i32>,
    mut collmarkarr: metamodelica::Array<i32>,
    mut iNSystems: i32,
) -> Result<i32> {
    let mut oNSystems: i32 = iNSystems;
    let mut rows: metamodelica::List<i32>;
    for mut i in ({
        let __s = index;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if !(intGt(
            ({
                let __elt = (*metamodelica::index_checked(&rowmarkarr.borrow(), i)?).clone();
                __elt
            }),
            0,
        )) {
            metamodelica::arrayUpdate(rowmarkarr.clone(), i, oNSystems)?;
            rows = List::select(
                ({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), i)?).clone();
                    __elt
                }),
                (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
            )?;
            oNSystems = partitionSystemstraverseRows(
                rows,
                metamodelica::nil(),
                m.clone(),
                mT.clone(),
                rowmarkarr.clone(),
                collmarkarr.clone(),
                oNSystems,
            )?;
        }
    }
    oNSystems = oNSystems - 1;
    Ok(oNSystems)
}

fn partitionSystemstraverseRows(
    mut iRows: metamodelica::List<i32>,
    mut iQueue: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rowmarkarr: metamodelica::Array<i32>,
    mut collmarkarr: metamodelica::Array<i32>,
    mut iNSystems: i32,
) -> Result<i32> {
    let __ab_mT = mT.borrow();
    let mut oNSystems: i32 = 0;
    let mut rows: metamodelica::List<i32> = iRows;
    let mut queue: metamodelica::List<i32> = iQueue;
    let mut colls: metamodelica::List<i32>;
    let mut r: i32;
    loop {
        if (rows).is_empty() {
            if (queue).is_empty() {
                oNSystems = iNSystems + 1;
                return Ok(oNSystems);
            }
            rows = queue;
            queue = metamodelica::nil();
        } else {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rows) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            rows = metamodelica::Own::own(__pa1);
            if !(intGt(
                ({
                    let __elt = (*metamodelica::index_checked(&collmarkarr.borrow(), r)?).clone();
                    __elt
                }),
                0,
            )) {
                metamodelica::arrayUpdate(collmarkarr.clone(), r, iNSystems)?;
                colls = List::select(
                    (*metamodelica::index_checked(&__ab_mT, r)?).clone(),
                    (std::sync::Arc::new(fnptr!(Util::intPositive, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
                )?;
                colls = List::select1r(
                    colls,
                    (std::sync::Arc::new(Matching::isUnAssigned)
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>),
                    rowmarkarr.clone(),
                )?;
                List::fold1(&colls, &markTrue, iNSystems, rowmarkarr.clone())?;
                queue = listAppend(
                    List::select1r(
                        List::flatten(List::map1r(colls, &arrayGet, m.clone())?)?,
                        (std::sync::Arc::new(Matching::isUnAssigned)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static,
                            >),
                        collmarkarr.clone(),
                    )?,
                    queue,
                );
            }
        }
    }
    Ok(oNSystems)
}

fn partitionSystemSplitt(
    mut index: i32,
    mut rowmarkarr: metamodelica::Array<i32>,
    mut systsarr: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let __ab_rowmarkarr = rowmarkarr.borrow();
    let mut osystsarr: metamodelica::Array<metamodelica::List<i32>> = systsarr;
    for mut i in ({
        let __s = index;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        osystsarr = Array::consToElement(
            (*metamodelica::index_checked(&__ab_rowmarkarr, i)?).clone(),
            i,
            osystsarr.clone(),
        )?;
    }
    Ok(osystsarr)
}

fn processComps4New(
    mut iSets: &metamodelica::List<metamodelica::List<i32>>,
    mut inVarSize: i32,
    mut inEqnsSize: i32,
    mut iVars: BackendDAE::Variables,
    mut iEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut inMT: metamodelica::Array<metamodelica::List<i32>>,
    mut inMapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut inMapIncRowEqn: metamodelica::Array<i32>,
    mut vec1: metamodelica::Array<i32>,
    mut vec2: metamodelica::Array<i32>,
    mut level: i32,
    mut iShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut iStateSets: StateSets,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    StateSets,
)> {
    let mut outDummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outDummyStates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut oStateSets: StateSets = iStateSets;
    let mut mapIncRowEqn1: metamodelica::Array<i32>;
    let mut ass1arr: metamodelica::Array<i32>;
    let mut dummyStates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqns1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = iEqns;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut seteqns: metamodelica::List<i32> = metamodelica::nil();
    let mut unassigned: metamodelica::List<i32>;
    let mut assigned: metamodelica::List<i32>;
    let mut set: metamodelica::List<i32>;
    let mut statevars: metamodelica::List<i32>;
    let mut ass1: metamodelica::List<i32>;
    let mut ass2: metamodelica::List<i32>;
    let mut assigend1: metamodelica::List<i32>;
    let mut range: metamodelica::List<i32>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut flag: metamodelica::Array<bool>;
    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut states1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut dstates1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>;
    let mut nstatevars: i32;
    let mut nassigned: i32;
    let mut nunassigned: i32;
    let mut nass1arr: i32;
    let mut n: i32;
    let mut nv: i32;
    let mut ne: i32;
    match '__try0: {
        for mut seteqns in &**iSets {
            let mut seteqns = seteqns.clone();
            if !(((unwrap_break_err!(List::select1r(seteqns.clone(), (std::sync::Arc::new(Matching::isUnAssigned) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone()), '__try0))).is_empty()) {
                unassigned = unwrap_break_err!(List::select1r(seteqns.clone(), (std::sync::Arc::new(Matching::isUnAssigned) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone()), '__try0);
                n = metamodelica::arrayLength(inM.clone());
                set = unwrap_break_err!(getEqnsforDynamicStateSelection(&unassigned, n, inM.clone(), inMT.clone(), vec1.clone(), vec2.clone(), inMapEqnIncRow.clone(), inMapIncRowEqn.clone()), '__try0);
                assigned = unwrap_break_err!(List::select1r(set.clone(), (std::sync::Arc::new(Matching::isAssigned) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), vec1.clone()), '__try0);
                flag = arrayCreate(inVarSize, true);
                (statevars, _) = unwrap_break_err!(List::fold3(&set, &getSetStates, flag.clone(), inM.clone(), vec2.clone(), (metamodelica::nil(), metamodelica::nil())), '__try0);
                nstatevars = ((statevars).len() as i32);
                ass1 = List::consN(nstatevars, -1, metamodelica::nil());
                nunassigned = ((unassigned).len() as i32);
                ass2 = List::consN(nunassigned, -1, metamodelica::nil());
                varlst = unwrap_break_err!(List::map1r(statevars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iVars.clone()), '__try0);
                assigend1 = unwrap_break_err!(List::map1r(unassigned.clone(), &arrayGet, inMapIncRowEqn.clone()), '__try0);
                n = metamodelica::arrayLength(inMapIncRowEqn.clone());
                assigend1 = unwrap_break_err!(List::uniqueIntN(&assigend1, n), '__try0);
                eqnlst = unwrap_break_err!(BackendEquation::getList(assigend1.clone(), eqns1.clone()), '__try0);
                eqns1 = unwrap_break_err!(List::fold(&assigend1, &BackendEquation::delete, eqns1.clone()), '__try0);
                nassigned = ((assigned).len() as i32);
                flag = arrayCreate(inEqnsSize, true);
                (eqnlst, varlst, ass1, ass2, eqns1) = unwrap_break_err!(getSetSystem(&assigned, inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), &iVars, eqns1.clone(), flag.clone(), nassigned, eqnlst.clone(), varlst.clone(), ass1.clone(), ass2.clone()), '__try0);
                eqns = unwrap_break_err!(BackendEquation::listEquation(&eqnlst), '__try0);
                vars = unwrap_break_err!(BackendVariable::listVar1(&varlst), '__try0);
                syst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                (_, _, _, mapIncRowEqn1) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst, iShared, false), '__try0);
                ass1arr = metamodelica::arrayFromVec(ass1.clone().into_iter().cloned().collect());
                nass1arr = metamodelica::arrayLength(ass1arr.clone());
                (dstates1, states1) = unwrap_break_err!(checkAssignment(1, nass1arr, ass1arr.clone(), &vars), '__try0);
                assigend1 = if (!((assigned).is_empty())) {List::intRange2(1, nassigned)} else {metamodelica::nil()};
                nunassigned = nassigned + nunassigned;
                nassigned = nassigned + 1;
                range = List::intRange2(nassigned, nunassigned);
                nv = BackendVariable::varsSize(&vars);
                ne = unwrap_break_err!(BackendEquation::equationArraySize(eqns.clone()), '__try0);
                (varlst, oStateSets) = unwrap_break_err!(selectDummyDerivatives2new(dstates1.clone(), states1.clone(), range.clone(), assigend1.clone(), vars.clone(), nv, eqns.clone(), ne, mapIncRowEqn1.clone(), level, oStateSets.clone()), '__try0);
                dummyStates = unwrap_break_err!(List::map(varlst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) }), '__try0);
                outDummyStates = List::append_reverse(&dummyStates, outDummyStates.clone());
                outDummyVars = listAppend(varlst.clone(), outDummyVars.clone());
            }
        }
        outDummyStates = metamodelica::Dangerous::listReverseInPlace(outDummyStates.clone());
        Ok::<_, &'static str>((outDummyStates.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outDummyStates = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("- IndexReduction.processComps4New failed!")],
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outDummyVars, outDummyStates, oStateSets))
}

fn forceInlinEqn(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFuncs: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    funcs = inFuncs;
    (e, _, _) = Inline::forceInlineExp(
        inExp,
        (
            Some(funcs.clone()),
            list![
                openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
                openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE
            ],
        ),
        DAE::emptyElementSource().clone(),
        &Ceval::cevalSimpleWithFunctionTreeReturnExp,
    )?;
    Ok((e, funcs))
}

fn getSetSystem<'__b>(
    mut iEqns: &'__b metamodelica::List<i32>,
    mut inMapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut inMapIncRowEqn: metamodelica::Array<i32>,
    mut vec1: metamodelica::Array<i32>,
    mut iVars: &'__b BackendDAE::Variables,
    mut iEqnsArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut flag: metamodelica::Array<bool>,
    mut n: i32,
    mut iEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iAss1: metamodelica::List<i32>,
    mut iAss2: metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut oEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut oAss1: metamodelica::List<i32>;
    let mut oAss2: metamodelica::List<i32>;
    let mut oEqnsArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    (oEqnsLst, oVarsLst, oAss1, oAss2, oEqnsArr) = (::match_deref::match_deref! { match iEqns {
        Deref @ metamodelica::ListNode::Nil => {
            (iEqnsLst, iVarsLst, iAss1, iAss2, iEqnsArr)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } if (({let __elt = (*metamodelica::index_checked(&flag.borrow(), e.clone())?).clone(); __elt}) && intGt(({let __elt = (*metamodelica::index_checked(&vec1.borrow(), e.clone())?).clone(); __elt}), 0)) => {
            let mut e1: i32;
            let mut eqns: metamodelica::List<i32>;
            let mut vindx: metamodelica::List<i32>;
            let mut ass: metamodelica::List<i32>;
            let mut ass1: metamodelica::List<i32>;
            let mut ass2: metamodelica::List<i32>;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            e1 = ({let __elt = (*metamodelica::index_checked(&inMapIncRowEqn.borrow(), e.clone())?).clone(); __elt});
            eqn = BackendEquation::get(iEqnsArr.clone(), e1)?;
            eqnarr = BackendEquation::delete(e1, iEqnsArr)?;
            eqns = ({let __elt = (*metamodelica::index_checked(&inMapEqnIncRow.borrow(), e1)?).clone(); __elt});
            List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), false, flag.clone())?;
            vindx = List::map1r(eqns.clone(), &arrayGet, vec1.clone())?;
            varlst = listAppend(List::map1r(vindx, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iVars.clone())?, iVarsLst);
            ass = List::intRange2(n - ((eqns).len() as i32) + 1, n);
            ass1 = listAppend(ass.clone(), iAss1);
            ass2 = listAppend(ass, iAss2);
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr) = getSetSystem(rest, inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), iVars, eqnarr, flag.clone(), n - ((eqns).len() as i32), metamodelica::cons(eqn, iEqnsLst), varlst, ass1, ass2)?;
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr)
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
            let mut ass1: metamodelica::List<i32>;
            let mut ass2: metamodelica::List<i32>;
            let mut eqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr) = getSetSystem(rest, inMapEqnIncRow.clone(), inMapIncRowEqn.clone(), vec1.clone(), iVars, iEqnsArr, flag.clone(), n, iEqnsLst, iVarsLst, iAss1, iAss2)?;
            (oEqnsLst, oVarsLst, ass1, ass2, eqnarr)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oEqnsLst, oVarsLst, oAss1, oAss2, oEqnsArr))
}

fn getSetStates(
    mut e: i32,
    mut flag: metamodelica::Array<bool>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut vec2: metamodelica::Array<i32>,
    mut iStates: (metamodelica::List<i32>, metamodelica::List<i32>),
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut oStates: (metamodelica::List<i32>, metamodelica::List<i32>);
    oStates = List::fold3(
        &({
            let __elt = (*metamodelica::index_checked(&inM.borrow(), e)?).clone();
            __elt
        }),
        &move |__a0: i32,
               __a1: metamodelica::Array<bool>,
               __a2: metamodelica::Array<metamodelica::List<i32>>,
               __a3: metamodelica::Array<i32>,
               __a4: (metamodelica::List<i32>, metamodelica::List<i32>)| {
            getSetEqnStates(__a0, __a1, __a2, __a3, &__a4)
        },
        flag.clone(),
        inM.clone(),
        vec2.clone(),
        iStates,
    )?;
    Ok(oStates)
}

fn getSetEqnStates(
    mut v: i32,
    mut flag: metamodelica::Array<bool>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut vec2: metamodelica::Array<i32>,
    mut iStates: &(metamodelica::List<i32>, metamodelica::List<i32>),
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let __ab_vec2 = vec2.borrow();
    let mut oStates: (metamodelica::List<i32>, metamodelica::List<i32>);
    let mut states: metamodelica::List<i32>;
    let mut dstates: metamodelica::List<i32>;
    (states, dstates) = iStates.clone();
    states = List::consOnTrue(
        intLt((*metamodelica::index_checked(&__ab_vec2, v)?).clone(), 1)
            && ({
                let __elt = (*metamodelica::index_checked(&flag.borrow(), v)?).clone();
                __elt
            }),
        v,
        states,
    );
    dstates = List::consOnTrue(
        intGt((*metamodelica::index_checked(&__ab_vec2, v)?).clone(), 0)
            && ({
                let __elt = (*metamodelica::index_checked(&flag.borrow(), v)?).clone();
                __elt
            }),
        v,
        dstates,
    );
    metamodelica::arrayUpdate(flag.clone(), v, false)?;
    oStates = (states, dstates);
    Ok(oStates)
}

fn getEqnsforDynamicStateSelection(
    mut U: &metamodelica::List<i32>,
    mut neqns: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut eqns: metamodelica::List<i32>;
    eqns = (::match_deref::match_deref! { match U {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => {
            let mut colummarks: metamodelica::Array<i32>;
            colummarks = arrayCreate(neqns, 0);
            getEqnsforDynamicStateSelection1(U, m.clone(), mT.clone(), 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), metamodelica::nil())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn getEqnsforDynamicStateSelection1<'__b>(
    mut U: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubset: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match U {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inSubset)
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } if (intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt}), 0)) => {
                let mut eqns: metamodelica::List<i32>;
                let mut set: metamodelica::List<i32>;
                let mut e1: i32;
                e1 = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), e.clone())?).clone(); __elt});
                eqns = ({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), e1)?).clone(); __elt});
                List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), mark, colummarks.clone())?;
                (set, _) = getEqnsforDynamicStateSelectionPhase(&eqns, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset, false)?;
                { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset) = (rest, m.clone(), mT.clone(), mark + 1, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (U, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getEqnsforDynamicStateSelectionPhase<'__b>(
    mut elst: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubset: metamodelica::List<i32>,
    mut iFound: bool,
) -> Result<(metamodelica::List<i32>, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match elst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inSubset, iFound))
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                let mut rows: metamodelica::List<i32>;
                let mut set: metamodelica::List<i32>;
                let mut found: bool;
                rows = List::select(({let __elt = (*metamodelica::index_checked(&m.borrow(), e.clone())?).clone(); __elt}), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
                rows = List::removeOnTrue(({let __elt = (*metamodelica::index_checked(&ass1.borrow(), e.clone())?).clone(); __elt}), &fnptr!(intEq, i32, i32), rows)?;
                (set, found) = getEqnsforDynamicStateSelectionRows(&rows, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset, false)?;
                set = List::consOnTrue(found, e.clone(), set);
                metamodelica::arrayUpdate(colummarks.clone(), e.clone(), if (found) {mark} else {({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), e.clone())?).clone(); __elt})})?;
                { (elst, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset, iFound) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set, found || iFound); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getEqnsforDynamicStateSelectionRows<'__b>(
    mut rows: &'__b metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut mark: i32,
    mut colummarks: metamodelica::Array<i32>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut inSubset: metamodelica::List<i32>,
    mut iFound: bool,
) -> Result<(metamodelica::List<i32>, bool)> {
    '__tco: loop {
        ::match_deref::match_deref! { match rows {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inSubset, iFound))
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } if (!(intGt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0))) => {
                let mut set: metamodelica::List<i32>;
                let mut b: bool;
                { (rows, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset, iFound) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset, true); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } if (intGt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0) && intEq(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}))?).clone(); __elt}), 0)) => {
                let mut set: metamodelica::List<i32>;
                let mut eqns: metamodelica::List<i32>;
                let mut rc: i32;
                let mut e: i32;
                let mut b: bool;
                rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                e = ({let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), rc)?).clone(); __elt});
                eqns = ({let __elt = (*metamodelica::index_checked(&mapEqnIncRow.borrow(), e)?).clone(); __elt});
                List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), if (iFound) {mark} else {-(mark)}, colummarks.clone())?;
                (set, b) = getEqnsforDynamicStateSelectionPhase(&eqns, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset, false)?;
                eqns = if (b && !(iFound)) {eqns} else {metamodelica::nil()};
                List::fold1r(&eqns, &*(Arc::new(arrayUpdate.clone())), mark, colummarks.clone())?;
                { (rows, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset, iFound) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), set, b || iFound); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } if (intGt(({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt}), 0)) => {
                let mut set: metamodelica::List<i32>;
                let mut rc: i32;
                let mut b: bool;
                rc = ({let __elt = (*metamodelica::index_checked(&ass2.borrow(), r.clone())?).clone(); __elt});
                b = intGt(({let __elt = (*metamodelica::index_checked(&colummarks.borrow(), rc)?).clone(); __elt}), 0);
                { (rows, m, mT, mark, colummarks, ass1, ass2, mapEqnIncRow, mapIncRowEqn, inSubset, iFound) = (rest, m.clone(), mT.clone(), mark, colummarks.clone(), ass1.clone(), ass2.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone(), inSubset, b || iFound); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn removeFirstOrgEqns(
    mut inOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut outEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut outOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut orgeqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut e: i32 = 0;
    let mut numEqs: i32;
    outOrgEqns = inOrgEqns.clone();
    numEqs = metamodelica::arrayLength(inOrgEqns.clone());
    for mut e in 1..=numEqs {
        orgeqns = metamodelica::arrayGet(outOrgEqns.clone(), e)?;
        if !((orgeqns).is_empty()) {
            (outEqnsLst, orgeqns) = (::match_deref::match_deref! { match &(orgeqns) {
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: Deref @ metamodelica::ListNode::Nil } => {
                    (metamodelica::cons(eqn.clone(), outEqnsLst), metamodelica::nil())
                },
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns } => {
                    (metamodelica::cons(eqn.clone(), outEqnsLst), eqns.clone())
                },
                _ => return Err("match: no arm matched"),
            } });
            metamodelica::arrayUpdate(outOrgEqns.clone(), e, orgeqns)?;
        }
    }
    Ok((outEqnsLst, outOrgEqns))
}

fn sortStateCandidatesVars(
    mut inVars: &BackendDAE::Variables,
    mut allVars: &BackendDAE::Variables,
    mut m: Option<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<BackendDAE::Variables> {
    let mut outStates: BackendDAE::Variables;
    let mut varsize: i32;
    let mut varIndices: metamodelica::List<i32>;
    let mut prioTuples: metamodelica::List<(i32, metamodelica::Real)>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut varCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut prio1: metamodelica::Real;
    let mut prio2: metamodelica::Real;
    let mut prio: metamodelica::Array<metamodelica::Real>;
    let mut index: metamodelica::Array<i32>;
    let mut idx: i32 = 0;
    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    varsize = BackendVariable::varsSize(inVars);
    index = arrayCreate(varsize, -1);
    prio = arrayCreate(varsize, metamodelica::OrderedFloat(-1.0_f64));
    for mut idx in 1..=varsize {
        v = BackendVariable::getVarAt(inVars, idx)?;
        (prio1, prio2) = varStateSelectPrio(&v, allVars, idx, m.clone())?;
        {
            let __cell0 = prio1 + prio2;
            let __idx0 = idx;
            *metamodelica::index_mut_checked(&mut prio.clone().borrow_mut(), __idx0)? = __cell0;
        }
        {
            let __cell1 = idx;
            let __idx1 = idx;
            *metamodelica::index_mut_checked(&mut index.clone().borrow_mut(), __idx1)? = __cell1;
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            varCref = BackendVariable::varCref(&v);
            BackendDump::debugStrCrefStrRealStrRealStrRealStr(
                &(literal!("Calc Prio for ")),
                &varCref,
                &(literal!("\n Prio StateSelect : ")),
                prio1,
                &(literal!("\n Prio Heuristik : ")),
                prio2,
                &(literal!("\n ### Prio Result : ")),
                ({
                    let __elt = (*metamodelica::index_checked(&prio.borrow(), idx)?).clone();
                    __elt
                }),
                &(literal!("\n")),
            )?;
        }
    }
    prioTuples = ({
        let mut __acc: metamodelica::List<(i32, metamodelica::Real)> = metamodelica::nil();
        for mut idx in ({
            let __s = varsize;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        })
        .into_iter()
        {
            let __x = (
                ({
                    let __elt = (*metamodelica::index_checked(&index.borrow(), idx)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&prio.borrow(), idx)?).clone();
                    __elt
                }),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    prioTuples = List::sort(
        prioTuples,
        (std::sync::Arc::new(fnptr!(
            sortprioTuples,
            (i32, metamodelica::Real),
            (i32, metamodelica::Real)
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn((i32, metamodelica::Real), (i32, metamodelica::Real)) -> Result<bool> + 'static,
            >),
    )?;
    varIndices = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut elem in (prioTuples).into_iter().cloned() {
            let __x = Util::tuple21(elem.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    vlst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut idx in (varIndices).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(inVars, idx)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outStates = BackendVariable::listVar1(&vlst)?;
    Ok(outStates)
}

fn sortprioTuples(mut inTpl1: (i32, metamodelica::Real), mut inTpl2: (i32, metamodelica::Real)) -> bool {
    let mut b: bool;
    b = Util::tuple22(inTpl1) > Util::tuple22(inTpl2);
    b
}

fn varStateSelectPrio(
    mut v: &metamodelica::Ref<BackendDAE::Var>,
    mut vars: &BackendDAE::Variables,
    mut index: i32,
    mut m: Option<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<(metamodelica::Real, metamodelica::Real)> {
    let mut prio_att: metamodelica::Real;
    let mut prio_heu: metamodelica::Real;
    prio_att = varStateSelectPrioAttribute(v);
    prio_heu = varStateSelectHeuristicPrio(v, vars, index, m)?;
    Ok((prio_att, prio_heu))
}

fn varStateSelectHeuristicPrio(
    mut v: &metamodelica::Ref<BackendDAE::Var>,
    mut vars: &BackendDAE::Variables,
    mut index: i32,
    mut m: Option<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real;
    let mut prio1: metamodelica::Real;
    let mut prio2: metamodelica::Real;
    let mut prio3: metamodelica::Real;
    let mut prio4: metamodelica::Real;
    let mut prio5: metamodelica::Real;
    let mut bstart: bool;
    let mut bfixed: bool;
    bstart = (BackendVariable::varStartValueOption(v)).is_some();
    bfixed = BackendVariable::varFixed(v);
    if bstart && bfixed {
        prio1 = metamodelica::OrderedFloat(0.5_f64);
        prio2 = metamodelica::OrderedFloat(0.5_f64);
    } else if bfixed {
        prio1 = metamodelica::OrderedFloat(0.1_f64);
        prio2 = metamodelica::OrderedFloat(0.5_f64);
    } else if bstart {
        prio1 = metamodelica::OrderedFloat(0.1_f64);
        prio2 = metamodelica::OrderedFloat(0.0_f64);
    } else {
        prio1 = metamodelica::OrderedFloat(0.0_f64);
        prio2 = metamodelica::OrderedFloat(0.0_f64);
    }
    prio3 = varStateSelectHeuristicPrio3(v);
    prio4 = varStateSelectHeuristicPrio4(v, vars);
    prio5 = varStateSelectHeuristicPrio5(v, index, m)?;
    prio = prio1 + prio2 + prio3 + prio4 + prio5;
    printVarListtateSelectHeuristicPrio(prio1, prio2, prio3, prio4, prio5)?;
    Ok(prio)
}

fn printVarListtateSelectHeuristicPrio(
    mut Prio1: metamodelica::Real,
    mut Prio2: metamodelica::Real,
    mut Prio3: metamodelica::Real,
    mut Prio4: metamodelica::Real,
    mut Prio5: metamodelica::Real,
) -> Result<()> {
    if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prio 1 : "));
            __mm_s.push_str(&*realString(Prio1));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prio 2 : "));
            __mm_s.push_str(&*realString(Prio2));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prio 3 : "));
            __mm_s.push_str(&*realString(Prio3));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prio 4 : "));
            __mm_s.push_str(&*realString(Prio4));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Prio 5 : "));
            __mm_s.push_str(&*realString(Prio5));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn varStateSelectHeuristicPrio5(
    mut v: &metamodelica::Ref<BackendDAE::Var>,
    mut index: i32,
    mut om: Option<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<metamodelica::Real> {
    let mut prio: metamodelica::Real;
    prio = (match om {
        None => metamodelica::OrderedFloat(0.0_f64),
        Some(mut m) => {
            let mut row: metamodelica::List<i32>;
            let mut n: metamodelica::Real;
            row = ({
                let __elt = (*metamodelica::index_checked(&m.borrow(), index)?).clone();
                __elt
            });
            n = intReal(metamodelica::arrayLength(m.clone())) + metamodelica::OrderedFloat(1.0_f64);
            n = metamodelica::real_div_checked(intReal(((row).len() as i32)), n)?;
            metamodelica::OrderedFloat(0.3_f64) * n
        }
    });
    Ok(prio)
}

fn varStateSelectHeuristicPrio4(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut vars: &BackendDAE::Variables,
) -> metamodelica::Real {
    let mut prio: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    prio = 'mc: {
        let __mc_input = &**inVar;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(cr), .. }, .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut b: bool;
                    let mut prio: metamodelica::Real = prio.clone();
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), vars)?;
                    b = BackendVariable::isDummyStateVar(&v);
                    prio = if (b) {metamodelica::OrderedFloat(0.0_f64)} else {metamodelica::OrderedFloat(0.55_f64)};
                    Ok((prio, prio.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            prio = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::OrderedFloat(0.0_f64))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    prio
}

fn varStateSelectHeuristicPrio3(mut v: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Real {
    let mut prio: metamodelica::Real;
    prio = 'mc: {
        let __mc_input = &**v;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, .. } => {
                    if !((stringEq(&(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?), &arcstr::literal!(DAE::derivativeNamePrefix)))) { return Err("guard") }
                    Ok(metamodelica::OrderedFloat(-5.0_f64))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::OrderedFloat(0.0_f64))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    prio
}

fn varStateSelectPrioAttribute(mut v: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Real {
    let mut prio: metamodelica::Real;
    let mut ss: DAE::StateSelect;
    ss = BackendVariable::varStateSelect(v);
    prio = (match ss {
        DAE::StateSelect::NEVER { .. } => {
            if (BackendVariable::isArtificialState(v)) {
                metamodelica::OrderedFloat(-15.0_f64)
            } else {
                metamodelica::OrderedFloat(-20.0_f64)
            }
        }
        DAE::StateSelect::AVOID { .. } => metamodelica::OrderedFloat(-1.5_f64),
        DAE::StateSelect::DEFAULT { .. } => metamodelica::OrderedFloat(0.0_f64),
        DAE::StateSelect::PREFER { .. } => metamodelica::OrderedFloat(1.5_f64),
        DAE::StateSelect::ALWAYS { .. } => metamodelica::OrderedFloat(20.0_f64),
    });
    prio
}

fn selectDummyDerivatives2new(
    mut dstates: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
    mut states: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
    mut unassignedEqns: metamodelica::List<i32>,
    mut assignedEqns: metamodelica::List<i32>,
    mut vars: BackendDAE::Variables,
    mut varSize: i32,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eqnsSize: i32,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut level: i32,
    mut iStateSets: StateSets,
) -> Result<(metamodelica::List<metamodelica::Ref<BackendDAE::Var>>, StateSets)> {
    let mut outDummyVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut oStateSets: StateSets;
    (outDummyVars, oStateSets) = 'mc: {
        let __mc_input = &*dstates;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((intEq(((dstates).len() as i32), eqnsSize))) { return Err("guard") }
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Select as States(1):\n"));
                        BackendDump::debuglst(&states, &dumpStates, &(literal!("\n")), &(literal!("\n")))?;
                        metamodelica::print(literal!("Select as dummyStates(1):\n"));
                        BackendDump::debuglst(&dstates, &dumpStates, &(literal!("\n")), &(literal!("\n")))?;
                    }
                    Ok((metamodelica::nil(), iStateSets.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut statecandidates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut ovarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut unassignedEqnsSize: i32;
                    let mut size: i32;
                    let mut rang: i32;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut oeqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut unassignedEqns1: metamodelica::List<i32>;
                    let mut assignedEqns1: metamodelica::List<i32>;
                    unassignedEqnsSize = ((unassignedEqns).len() as i32);
                    size = ((states).len() as i32);
                    rang = size - unassignedEqnsSize;
                    let true = (intGt(rang, 0)) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        BackendDump::debugStrIntStrIntStr(&(literal!("Select ")), rang, &(literal!(" from ")), size, &(literal!(" States\n")))?;
                        BackendDump::debuglst(&states, &dumpStates, &(literal!("\n")), &(literal!("\n")))?;
                        metamodelica::print(literal!("Select as dummyStates(2):\n"));
                        BackendDump::debuglst(&dstates, &dumpStates, &(literal!("\n")), &(literal!("\n")))?;
                    }
                    statecandidates = List::map1r(List::map(states.clone(), &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    unassignedEqns1 = List::uniqueIntN(&(List::map1r(unassignedEqns.clone(), &arrayGet, mapIncRowEqn.clone())?), eqnsSize)?;
                    eqnlst = BackendEquation::getList(unassignedEqns1.clone(), eqns.clone())?;
                    ovarlst = List::map1r(List::map(dstates.clone(), &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    assignedEqns1 = List::uniqueIntN(&(List::map1r(assignedEqns.clone(), &arrayGet, mapIncRowEqn.clone())?), eqnsSize)?;
                    oeqnlst = BackendEquation::getList(assignedEqns1.clone(), eqns.clone())?;
                    varlst = List::map1r(List::map(states.clone(), &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    Ok((varlst.clone(), metamodelica::cons((level, rang, size, unassignedEqnsSize, statecandidates.clone(), eqnlst.clone(), ovarlst.clone(), oeqnlst.clone()), iStateSets.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut unassignedEqnsSize: i32;
                    let mut size: i32;
                    let mut rang: i32;
                    unassignedEqnsSize = ((unassignedEqns).len() as i32);
                    size = ((states).len() as i32);
                    rang = size - unassignedEqnsSize;
                    if intLt(rang, 0) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Selection of DummyDerivatives failed due to negative system rank of ")); __mm_s.push_str(&*intString(rang)); __mm_s.push_str(&*literal!("!\n           There are ")); __mm_s.push_str(&*intString(unassignedEqnsSize)); __mm_s.push_str(&*literal!(" unassigned equations and ")); __mm_s.push_str(&*intString(size)); __mm_s.push_str(&*literal!(" potential states.\n")); ArcStr::from(__mm_s) }])?;
                    }
                    let true = (intEq(rang, 0)) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::BLT_DUMP.clone())? {
                        metamodelica::print(literal!("Select as dummyStates(3):\n"));
                        BackendDump::debuglst(&states, &dumpStates, &(literal!("\n")), &(literal!("\n")))?;
                    }
                    varlst = List::map1r(List::map(states.clone(), &fnptr!(Util::tuple22, _))?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    Ok((varlst.clone(), iStateSets.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("- IndexReduction.selectDummyDerivatives2new failed!")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outDummyVars, oStateSets))
}

pub(crate) fn makeder(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    tp = Expression::r#typeof(inExp.clone())?;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
        expLst: list![inExp],
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: tp,
            tuple_: false,
            builtin: true,
            isImpure: false,
            isFunctionPointerCall: false,
            inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: DAE::NoReturn::RETURNS.clone(),
        }),
    });
    Ok(outExp)
}

fn adjacencyMatrixfromEnhancedStrict(
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut vars: BackendDAE::Variables,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    m = Array::map1(
        me.clone(),
        &move |__a0: metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
               __a1: BackendDAE::Variables| adjacencyMatrixElementfromEnhancedStrict(&__a0, __a1),
        vars,
    )?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhancedStrict(
    mut iRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut vars: BackendDAE::Variables,
) -> Result<metamodelica::List<i32>> {
    let mut oRow: metamodelica::List<i32>;
    oRow = List::fold1(
        iRow,
        &move |__a0: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        ),
               __a1: BackendDAE::Variables,
               __a2: metamodelica::List<i32>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(adjacencyMatrixElementElementfromEnhancedStrict(&__a0, &__a1, __a2))
        },
        vars,
        metamodelica::nil(),
    )?;
    oRow = List::map(oRow, &fnptr!(intAbs, i32))?;
    oRow = oRow.reverse();
    Ok(oRow)
}

fn adjacencyMatrixElementElementfromEnhancedStrict(
    mut inTpl: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
    mut vars: &BackendDAE::Variables,
    mut iRow: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut oRow: metamodelica::List<i32>;
    oRow = (::match_deref::match_deref! { match &(inTpl) {
        (i, BackendDAE::Solvability::SOLVABILITY_SOLVED { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        _ => {
            iRow
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oRow
}

fn adjacencyMatrixfromEnhanced(
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut vars: BackendDAE::Variables,
    mut so: BackendDAE::StateOrder,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    m = Array::map1(
        me.clone(),
        &move |__a0: metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
               __a1: (BackendDAE::Variables, BackendDAE::StateOrder)| {
            adjacencyMatrixElementfromEnhanced(&__a0, __a1)
        },
        (vars, so),
    )?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhanced(
    mut iRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut tpl: (BackendDAE::Variables, BackendDAE::StateOrder),
) -> Result<metamodelica::List<i32>> {
    let mut oRow: metamodelica::List<i32>;
    oRow = List::fold1(
        iRow,
        &move |__a0: (
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        ),
               __a1: (BackendDAE::Variables, BackendDAE::StateOrder),
               __a2: metamodelica::List<i32>| adjacencyMatrixElementElementfromEnhanced(&__a0, &__a1, __a2),
        tpl,
        metamodelica::nil(),
    )?;
    oRow = List::map(oRow, &fnptr!(intAbs, i32))?;
    oRow = oRow.reverse();
    Ok(oRow)
}

fn adjacencyMatrixfromEnhancedPartial(
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut vars: BackendDAE::Variables,
    mut neverVars: BackendDAE::Variables,
    mut ass: metamodelica::Array<i32>,
    mut so: BackendDAE::StateOrder,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    m = Array::map1Ind(
        me.clone(),
        &move |__a0: metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
               __a1: i32,
               __a2: (
            BackendDAE::Variables,
            BackendDAE::Variables,
            metamodelica::Array<i32>,
            BackendDAE::StateOrder,
        )| adjacencyMatrixElementfromEnhancedPartial(&__a0, __a1, &__a2),
        (vars, neverVars, ass.clone(), so),
    )?;
    Ok(m)
}

fn adjacencyMatrixElementfromEnhancedPartial(
    mut iRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut index: i32,
    mut varsAssTpl: &(
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Array<i32>,
        BackendDAE::StateOrder,
    ),
) -> Result<metamodelica::List<i32>> {
    let mut oRow: metamodelica::List<i32>;
    let mut vars: BackendDAE::Variables;
    let mut neverVars: BackendDAE::Variables;
    let mut ass: metamodelica::Array<i32>;
    let mut so: BackendDAE::StateOrder;
    (vars, neverVars, ass, so) = varsAssTpl.clone();
    if intEq(
        ({
            let __elt = (*metamodelica::index_checked(&ass.borrow(), index)?).clone();
            __elt
        }),
        -1,
    ) || !(BackendVariable::varStateSelectNever(
        &(BackendVariable::getVarAt(
            &vars,
            ({
                let __elt = (*metamodelica::index_checked(&ass.borrow(), index)?).clone();
                __elt
            }),
        )?),
    )) {
        oRow = List::fold1(
            iRow,
            &move |__a0: (
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            ),
                   __a1: (BackendDAE::Variables, BackendDAE::StateOrder),
                   __a2: metamodelica::List<i32>| {
                adjacencyMatrixElementElementfromEnhanced(&__a0, &__a1, __a2)
            },
            (neverVars, so),
            metamodelica::nil(),
        )?;
        oRow = List::map(oRow, &fnptr!(intAbs, i32))?;
        oRow = oRow.reverse();
    } else {
        oRow = metamodelica::nil();
    }
    Ok(oRow)
}

fn adjacencyMatrixElementElementfromEnhanced(
    mut inTpl: &(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
    mut tpl: &(BackendDAE::Variables, BackendDAE::StateOrder),
    mut iRow: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oRow: metamodelica::List<i32>;
    oRow = (::match_deref::match_deref! { match &(inTpl) {
        (i, BackendDAE::Solvability::SOLVABILITY_SOLVED { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_CONST { .. }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: true }, _) => {
            metamodelica::cons(i.clone(), iRow)
        },
        (i, BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: false }, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl, iRow)?
        },
        (i, BackendDAE::Solvability::SOLVABILITY_LINEAR { b: true }, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl, iRow)?
        },
        (i, BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. }, _) => {
            adjacencyMatrixElementElementfromEnhanced_1(i.clone(), tpl, iRow)?
        },
        _ => {
            iRow
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRow)
}

fn adjacencyMatrixElementElementfromEnhanced_1(
    mut i: i32,
    mut tpl: &(BackendDAE::Variables, BackendDAE::StateOrder),
    mut iRow: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oRow: metamodelica::List<i32>;
    let mut vars: BackendDAE::Variables;
    let mut so: BackendDAE::StateOrder;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut b: bool;
    let (
        __pa0,
        ref __pa2 @ BackendDAE::STATEORDER {
            invHashTable: ref __pa1,
            ..
        },
    ) = (tpl.clone())
    else {
        return Err("pattern mismatch");
    };
    vars = metamodelica::Own::own(__pa0);
    ht = metamodelica::Own::own(__pa1);
    so = metamodelica::Own::own(__pa2);
    v = BackendVariable::getVarAt(&vars, intAbs(i))?;
    b = BackendVariable::varStateSelectNever(&v) && !(BaseHashTable::hasKey(BackendVariable::varCref(&v), &ht)?);
    oRow = List::consOnTrue(b, i, iRow);
    Ok(oRow)
}

fn checkAssignment(
    mut index: i32,
    mut len: i32,
    mut ass: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
)> {
    let __ab_ass = ass.borrow();
    let mut outAssigned: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)> = metamodelica::nil();
    let mut outUnassigned: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)> = metamodelica::nil();
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    for mut indx in index..=len {
        let __arc1 = BackendVariable::getVarAt(vars, indx)?;
        let BackendDAE::VAR { varName: __pa0, .. } = &*__arc1;
        cr = metamodelica::Own::own(__pa0);
        if intGt((*metamodelica::index_checked(&__ab_ass, indx)?).clone(), 0) {
            outAssigned = metamodelica::cons((cr, indx), outAssigned);
        } else {
            outUnassigned = metamodelica::cons((cr, indx), outUnassigned);
        }
    }
    Ok((outAssigned, outUnassigned))
}

fn getLevelStates(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut level: i32,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    (outVar, oHt) = 'mc: {
        let __mc_input = &*inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, natural }, varDirection: dir, varParallelism: prl, varType: tp, arryDim: dim, source, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, encrypted, .. } => {
                    if !((intGt(diffcount.clone(), 1))) { return Err("guard") }
                    let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut odattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut dattr: metamodelica::Ref<DAE::VariableAttributes>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut n: i32;
                    n = diffcount.clone() - level;
                    let true = (intGt(n, 0)) else { return Err("pattern mismatch") };
                    cr = Util::foldcallN(n, &fnptr!(ComponentReference::crefPrefixDer, metamodelica::Ref<DAE::ComponentRef>), name.clone())?;
                    e = Expression::crefExp(cr.clone())?;
                    ht = BaseHashTable::add(((name.clone(), n), e.clone()), iHt.clone())?;
                    dattr = BackendVariable::getVariableAttributefromType(metamodelica::AsArg::as_arg(&tp))?;
                    odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })))?;
                    var = metamodelica::Ref::new(BackendDAE::Var { varName: cr.clone(), varKind: BackendDAE::VarKind::STATE { index: 1, derName: None, natural: natural.clone() }, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() });
                    Ok((var.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffcount, derName, natural }, .. } => {
                    if !((intGt(diffcount.clone(), 1))) { return Err("guard") }
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    var = BackendVariable::setVarKind(inVar.clone(), BackendDAE::VarKind::STATE { index: 1, derName: derName.clone(), natural: natural.clone() })?;
                    Ok((var.clone(), iHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), iHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, oHt)
}

fn replaceHigherDerivatives(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = inSystem;
    let mut vars: BackendDAE::Variables;
    let mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    let mut dummyvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut addassign: metamodelica::List<(i32, i32)>;
    let mut nv1: i32;
    let mut nv: i32;
    ht = HashTableCrIntToExp::emptyHashTable();
    nv = BackendVariable::varsSize(&osyst.orderedVars);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(osyst.matching.clone()) {
        Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ass1 = metamodelica::Own::own(__pa0);
    ass2 = metamodelica::Own::own(__pa1);
    let (__pa2, (_, _, __pa3, __pa4, __pa5, __pa6)) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        osyst.orderedVars.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>,
                  __a1: (
                BackendDAE::Variables,
                i32,
                i32,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                (
                    metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<
                            Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                        >,
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                ) -> Result<bool>
                                + 'static,
                        >,
                        Arc<
                            dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    ),
                ),
            )|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(makeHigherStatesRepl(__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            i32,
                            i32,
                            metamodelica::List<(i32, i32)>,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            i32,
                            i32,
                            metamodelica::List<(i32, i32)>,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ),
                    )> + 'static,
            >),
        (
            osyst.orderedVars.clone(),
            1,
            nv,
            metamodelica::nil(),
            metamodelica::nil(),
            ht,
        ),
    )?;
    vars = metamodelica::Own::own(__pa2);
    nv1 = metamodelica::Own::own(__pa3);
    addassign = metamodelica::Own::own(__pa4);
    dummyvars = metamodelica::Own::own(__pa5);
    ht = metamodelica::Own::own(__pa6);
    dummyvars = dummyvars.reverse();
    vars = BackendVariable::addVars(&dummyvars, vars)?;
    let (__asg7_0, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        vars,
        (std::sync::Arc::new(replaceDummyDerivativesVar)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    )> + 'static,
            >),
        ht.clone(),
    )?;
    assign_field!(osyst.orderedVars = __asg7_0.clone());
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        osyst.orderedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(replaceDummyDerivativesExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        )> + 'static,
                >),
            ht,
        ),
    )?;
    ass1 = Array::expand(nv1 - nv, ass1.clone(), -1)?;
    List::map2_0(&addassign, &setHigerDerivativeAssignment, ass1.clone(), ass2.clone())?;
    assign_field!(
        osyst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ass1.clone(),
            ass2: ass2.clone(),
            comps: metamodelica::nil()
        })
    );
    Ok(osyst)
}

fn setHigerDerivativeAssignment(
    mut inTpl: (i32, i32),
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<()> {
    let mut i: i32;
    let mut j: i32;
    let mut e: i32;
    (i, j) = inTpl;
    e = ({
        let __elt = (*metamodelica::index_checked(&ass1.borrow(), i)?).clone();
        __elt
    });
    metamodelica::arrayUpdate(ass1.clone(), i, -1)?;
    metamodelica::arrayUpdate(ass1.clone(), j, e)?;
    metamodelica::arrayUpdate(ass2.clone(), e, j)?;
    Ok(())
}

fn makeHigherStatesRepl(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(
        BackendDAE::Variables,
        i32,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        i32,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oTpl: (
        BackendDAE::Variables,
        i32,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableCrIntToExp::FuncHashCref,
                HashTableCrIntToExp::FuncCrefEqual,
                HashTableCrIntToExp::FuncCrefStr,
                HashTableCrIntToExp::FuncExpStr,
            ),
        ),
    );
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, .. }, .. }, (vars, i, j, addassign, varlst, ht)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut j = (*j).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    let true = (intGt(diffcount.clone(), 1)) else { return Err("pattern mismatch") };
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    (varlst, ht, j) = makeHigherStatesRepl1(diffcount.clone() - 2, 2, metamodelica::AsArg::as_arg(&name), cr.clone(), metamodelica::AsArg::as_arg(&var), metamodelica::AsArg::as_arg(&vars), varlst.clone(), ht.clone(), j.clone())?;
                    Ok((var.clone(), (vars.clone(), i.clone() + 1, j.clone(), metamodelica::cons((i.clone(), j.clone()), addassign.clone()), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var, (vars, i, j, addassign, varlst, ht)) => {
                    Ok((var.clone(), (vars.clone(), i.clone() + 1, j.clone(), addassign.clone(), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, oTpl)
}

fn makeHigherStatesRepl1<'__b>(
    mut diffCount: i32,
    mut diffedCount: i32,
    mut iOrigName: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut iName: metamodelica::Ref<DAE::ComponentRef>,
    mut inVar: &'__b metamodelica::Ref<BackendDAE::Var>,
    mut vars: &'__b BackendDAE::Variables,
    mut iVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iN: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    '__tco: loop {
        match &**inVar {
            BackendDAE::Var {
                varName: name,
                varDirection: dir,
                varParallelism: prl,
                varType: tp,
                arryDim: dim,
                source,
                tearingSelectOption: ts,
                hideResult,
                comment,
                connectorType: ct,
                innerOuter: io,
                encrypted,
                ..
            } if (intGt(diffCount, -1)) => {
                let mut ht: (
                    metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<
                            Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                        >,
                    ),
                    i32,
                    (
                        HashTableCrIntToExp::FuncHashCref,
                        HashTableCrIntToExp::FuncCrefEqual,
                        HashTableCrIntToExp::FuncCrefStr,
                        HashTableCrIntToExp::FuncExpStr,
                    ),
                );
                let mut kind: BackendDAE::VarKind;
                let mut odattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                let mut dattr: metamodelica::Ref<DAE::VariableAttributes>;
                let mut var: metamodelica::Ref<BackendDAE::Var>;
                let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut n: i32;
                let mut name = (*name).clone();
                name = ComponentReference::crefPrefixDer(iName);
                e = Expression::crefExp(name.clone())?;
                ht = BaseHashTable::add(((iOrigName.clone(), diffedCount), e), iHt)?;
                dattr = BackendVariable::getVariableAttributefromType(tp)?;
                odattr = DAEUtil::setFixedAttr(
                    Some(dattr),
                    Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })),
                )?;
                odattr = DAEUtil::setProtectedAttr(odattr, DAEUtil::getProtectedAttr(inVar.values.clone()))?;
                kind = if (intGt(diffCount, 0)) {
                    BackendDAE::VarKind::STATE {
                        index: diffCount,
                        derName: None,
                        natural: true,
                    }
                } else {
                    openmodelica_backend_types::BackendDAE::VarKind::DUMMY_DER
                };
                var = metamodelica::Ref::new(BackendDAE::Var {
                    varName: name.clone(),
                    varKind: kind,
                    varDirection: dir.clone(),
                    varParallelism: prl.clone(),
                    varType: tp.clone(),
                    bindExp: None,
                    tplExp: None,
                    arryDim: dim.clone(),
                    source: source.clone(),
                    values: odattr,
                    tearingSelectOption: ts.clone(),
                    hideResult: hideResult.clone(),
                    comment: comment.clone(),
                    connectorType: ct.clone(),
                    innerOuter: io.clone(),
                    unreplaceable: false,
                    initNonlinear: false,
                    encrypted: encrypted.clone(),
                });
                {
                    (diffCount, diffedCount, iOrigName, iName, inVar, vars, iVarLst, iHt, iN) = (
                        diffCount - 1,
                        diffedCount + 1,
                        iOrigName,
                        name.clone(),
                        inVar,
                        vars,
                        metamodelica::cons(var, iVarLst),
                        ht,
                        iN + 1,
                    );
                    continue '__tco;
                }
            }
            _ => return Ok((iVarLst, iHt, iN)),
        }
    }
}

fn addAllDummyStates(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut so: BackendDAE::StateOrder,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = inSystem;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    let mut vars: BackendDAE::Variables;
    let mut dummvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let (__pa0, (_, _, __pa1, __pa2)) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        osyst.orderedVars.clone(),
        (std::sync::Arc::new(fnptr!(
            makeAllDummyVarandDummyDerivativeRepl,
            metamodelica::Ref<BackendDAE::Var>,
            (
                BackendDAE::Variables,
                BackendDAE::StateOrder,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                (
                    metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<
                            Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                        >
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                    (metamodelica::Ref<DAE::ComponentRef>, i32),
                                ) -> Result<bool>
                                + 'static,
                        >,
                        Arc<
                            dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>
                    )
                )
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            BackendDAE::StateOrder,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            BackendDAE::StateOrder,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ),
                    )> + 'static,
            >),
        (osyst.orderedVars.clone(), so, metamodelica::nil(), iHt),
    )?;
    vars = metamodelica::Own::own(__pa0);
    dummvars = metamodelica::Own::own(__pa1);
    oHt = metamodelica::Own::own(__pa2);
    vars = BackendVariable::addVars(&dummvars, vars)?;
    let (__asg3_0, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        vars,
        (std::sync::Arc::new(replaceDummyDerivativesVar)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    )> + 'static,
            >),
        oHt.clone(),
    )?;
    assign_field!(osyst.orderedVars = __asg3_0.clone());
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        osyst.orderedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(replaceDummyDerivativesExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        )> + 'static,
                >),
            oHt.clone(),
        ),
    )?;
    Ok((osyst, oHt))
}

fn makeAllDummyVarandDummyDerivativeRepl(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        BackendDAE::Variables,
        BackendDAE::StateOrder,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        BackendDAE::StateOrder,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oTpl: (
        BackendDAE::Variables,
        BackendDAE::StateOrder,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableCrIntToExp::FuncHashCref,
                HashTableCrIntToExp::FuncCrefEqual,
                HashTableCrIntToExp::FuncCrefStr,
                HashTableCrIntToExp::FuncExpStr,
            ),
        ),
    );
    (outVar, oTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: diffcount, .. }, values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. }), .. }, _) => {
                    if !((intEq(diffcount.clone(), 1))) { return Err("guard") }
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(cr), natural, .. }, values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. }), .. }, _) => {
                    let mut var = (*var).clone();
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: Some(cr.clone()), natural: natural.clone() })?;
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, natural }, values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. }), .. }, (vars, so, varlst, ht)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut var = (*var).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(diffcount.clone() - 1, 2, metamodelica::AsArg::as_arg(&name), cr.clone(), metamodelica::AsArg::as_arg(&var), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&so), varlst.clone(), ht.clone())?;
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: natural.clone() })?;
                    Ok((var.clone(), (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(1, 1, metamodelica::AsArg::as_arg(&name), name.clone(), metamodelica::AsArg::as_arg(&var), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&so), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }), (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { index: diffcount, derName: None, .. }, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(diffcount.clone(), 1, metamodelica::AsArg::as_arg(&name), name.clone(), metamodelica::AsArg::as_arg(&var), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&so), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }), (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::VARIABLE { .. }, varDirection: dir, varParallelism: prl, varType: tp, bindExp: bind, tplExp, arryDim: dim, source, values: attr, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, .. }, (vars, so, varlst, ht)) => {
                    if !((BackendVariable::varStateSelectPrefer(metamodelica::AsArg::as_arg(&var)))) { return Err("guard") }
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut source = (*source).clone();
                    let mut varlst = (*varlst).clone();
                    let mut ht = (*ht).clone();
                    (varlst, ht) = makeAllDummyVarandDummyDerivativeRepl1(1, 1, metamodelica::AsArg::as_arg(&name), name.clone(), metamodelica::AsArg::as_arg(&var), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&so), varlst.clone(), ht.clone())?;
                    cr = ComponentReference::crefPrefixDer(name.clone());
                    source = ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::NEW_DUMMY_DER { chosen: cr.clone(), candidates: metamodelica::nil() }))?;
                    Ok((metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: tplExp.clone(), arryDim: dim.clone(), source: source.clone(), values: attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: var.encrypted.clone() }), (vars.clone(), so.clone(), varlst.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, oTpl)
}

fn makeAllDummyVarandDummyDerivativeRepl1<'__b>(
    mut diffCount: i32,
    mut diffedCount: i32,
    mut iOrigName: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut iName: metamodelica::Ref<DAE::ComponentRef>,
    mut inVar: &'__b metamodelica::Ref<BackendDAE::Var>,
    mut vars: &'__b BackendDAE::Variables,
    mut so: &'__b BackendDAE::StateOrder,
    mut iVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((diffCount, inVar.clone())) {
            (0, _) => {
                return Ok((iVarLst, iHt))
            },
            (_, Deref @ BackendDAE::Var { varName: name, varDirection: dir, varParallelism: prl, varType: tp, arryDim: dim, source, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, encrypted, .. }) => {
                let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
                let mut odattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                let mut dattr: metamodelica::Ref<DAE::VariableAttributes>;
                let mut var: metamodelica::Ref<BackendDAE::Var>;
                let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut name = (*name).clone();
                name = ComponentReference::crefPrefixDer(iName);
                e = Expression::crefExp(name.clone())?;
                ht = BaseHashTable::add(((iOrigName.clone(), diffedCount), e), iHt)?;
                dattr = BackendVariable::getVariableAttributefromType(metamodelica::AsArg::as_arg(&tp))?;
                odattr = DAEUtil::setFixedAttr(Some(dattr), Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })))?;
                odattr = DAEUtil::setProtectedAttr(odattr, DAEUtil::getProtectedAttr(inVar.values.clone()))?;
                var = metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DUMMY_DER, varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr, tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() });
                { (diffCount, diffedCount, iOrigName, iName, inVar, vars, so, iVarLst, iHt) = (diffCount - 1, diffedCount + 1, iOrigName, name.clone(), inVar, vars, so, metamodelica::cons(var, iVarLst), ht); continue '__tco; }
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("IndexReduction.makeAllDummyVarandDummyDerivativeRepl1 failed!")])?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addDummyStates(
    mut dummyStates: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut level: i32,
    mut repl: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oHt: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    );
    (osyst, oHt) = (::match_deref::match_deref! { match dummyStates {
        Deref @ metamodelica::ListNode::Nil => {
            (inSystem, iHt)
        },
        _ => {
            let mut syst = inSystem.clone();
            let mut ht: (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableCrIntToExp::FuncHashCref, HashTableCrIntToExp::FuncCrefEqual, HashTableCrIntToExp::FuncCrefStr, HashTableCrIntToExp::FuncExpStr));
            let mut vars: BackendDAE::Variables;
            (vars, ht) = List::fold1(dummyStates, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: i32, __a2: (BackendDAE::Variables, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))| makeDummyVarandDummyDerivative(__a0, __a1, &__a2), level, (syst.orderedVars.clone(), iHt))?;
            let (__asg0_0, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars, (std::sync::Arc::new(replaceDummyDerivativesVar) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), ht.clone())?;
            assign_field!(syst.orderedVars = __asg0_0.clone());
            BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(replaceDummyDerivativesExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>, (i32, i32, metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32), (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), ht.clone()))?;
            BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(replaceFirstOrderDerivativesExp, metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>)))> + 'static>), repl))?;
            (syst, ht)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oHt))
}

fn makeDummyVarandDummyDerivative(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut level: i32,
    mut inTpl: &(
        BackendDAE::Variables,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    BackendDAE::Variables,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oTpl: (
        BackendDAE::Variables,
        (
            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableCrIntToExp::FuncHashCref,
                HashTableCrIntToExp::FuncCrefEqual,
                HashTableCrIntToExp::FuncCrefStr,
                HashTableCrIntToExp::FuncExpStr,
            ),
        ),
    );
    oTpl = 'mc: {
        let __mc_input = (&*inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { index: diffindex, .. }, varDirection: dir, varParallelism: prl, varType: tp, arryDim: dim, source, tearingSelectOption: ts, hideResult, comment, connectorType: ct, innerOuter: io, encrypted: e, .. }, (vars, ht)) => {
                    let mut dummyderName: metamodelica::Ref<DAE::ComponentRef>;
                    let mut odattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut dattr: metamodelica::Ref<DAE::VariableAttributes>;
                    let mut dummy_state: metamodelica::Ref<BackendDAE::Var>;
                    let mut dummy_derstate: metamodelica::Ref<BackendDAE::Var>;
                    let mut dn: i32;
                    let mut kind: BackendDAE::VarKind;
                    let mut name = (*name).clone();
                    let mut diffindex = (*diffindex).clone();
                    let mut vars = (*vars).clone();
                    let mut ht = (*ht).clone();
                    dn = intMax(diffindex.clone() - level, 0);
                    (name, dummyderName) = crefPrefixDerN(dn, name.clone());
                    dattr = BackendVariable::getVariableAttributefromType(metamodelica::AsArg::as_arg(&tp))?;
                    odattr = DAEUtil::setFixedAttr(Some(dattr.clone()), Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })))?;
                    odattr = DAEUtil::setProtectedAttr(odattr.clone(), DAEUtil::getProtectedAttr(inVar.values.clone()))?;
                    dummy_derstate = metamodelica::Ref::new(BackendDAE::Var { varName: dummyderName.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DUMMY_DER, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: e.clone() });
                    kind = if (intEq(dn, 0)) {openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE} else {openmodelica_backend_types::BackendDAE::VarKind::DUMMY_DER};
                    dummy_state = metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: kind.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: None, tplExp: None, arryDim: dim.clone(), source: source.clone(), values: odattr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: io.clone(), unreplaceable: false, initNonlinear: false, encrypted: e.clone() });
                    dummy_state = if (intEq(dn, 0)) {inVar.clone()} else {dummy_state.clone()};
                    dummy_state = BackendVariable::setVarKind(dummy_state.clone(), kind.clone())?;
                    vars = BackendVariable::addVar(dummy_derstate.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(dummy_state.clone(), vars.clone())?;
                    diffindex = dn + 1;
                    ht = BaseHashTable::add(((name.clone(), diffindex.clone()), Expression::crefExp(dummyderName.clone())?), ht.clone())?;
                    Ok((vars.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IndexReduction.makeDummyVarandDummyDerivative failed ")); __mm_s.push_str(&*BackendDump::varString(&inVar)?); __mm_s.push_str(&*literal!("!")); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oTpl)
}

fn crefPrefixDerN(
    mut n: i32,
    mut iName: metamodelica::Ref<DAE::ComponentRef>,
) -> (
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
) {
    '__tco: loop {
        match n {
            0 if (!(intGt(n, 0))) => {
                let mut dername: metamodelica::Ref<DAE::ComponentRef>;
                dername = ComponentReference::crefPrefixDer(iName.clone());
                return (iName, dername);
            }
            _ => {
                let mut name: metamodelica::Ref<DAE::ComponentRef>;
                let mut dername: metamodelica::Ref<DAE::ComponentRef>;
                dername = ComponentReference::crefPrefixDer(iName);
                {
                    (n, iName) = (n - 1, dername);
                    continue '__tco;
                }
            }
        }
    }
}

fn replaceFirstOrderDerivativesExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTable2::FuncHashCref,
            HashTable2::FuncCrefEqual,
            HashTable2::FuncCrefStr,
            HashTable2::FuncExpStr,
        ),
    );
    (outExp, ht) = 'mc: {
        let __mc_input = (&*inExp, iht.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, ht) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = BaseHashTable::get(cr.clone(), &(ht.clone()))?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), iht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, ht)
}

fn replaceDummyDerivativesExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    ) = ht;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut i: i32;
    let mut e: metamodelica::Ref<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: __esc_i }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            cr = (*__esc_cr).clone();
            i = (*__esc_i).clone();
            match '__try0: {
                e = unwrap_break_err!(BaseHashTable::get((cr.clone(), i.clone()), &ht), '__try0);
                Ok::<_, &'static str>((e.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    e = __try0_o0;
                }
                Err(_) => {
                    warnDummyDerivativeFailed(exp.clone())?;
                    e = exp.clone();
                }
            }
            e
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            cr = (*__esc_cr).clone();
            match '__try0: {
                e = unwrap_break_err!(BaseHashTable::get((cr.clone(), 1), &ht), '__try0);
                Ok::<_, &'static str>((e.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    e = __try0_o0;
                }
                Err(_) => {
                    e = exp.clone();
                }
            }
            e
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. } => {
            warnDummyDerivativeFailed(exp.clone())?;
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, ht))
}

fn warnDummyDerivativeFailed(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    Error::addMessage(
        Error::COMPILER_WARNING.clone(),
        list![{
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("IndexReduction.replaceDummyDerivativesExp failed for "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(exp)?);
            __mm_s.push_str(&*literal!("!"));
            ArcStr::from(__mm_s)
        }],
    )?;
    Ok(())
}

fn replaceDummyDerivatives(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem> = inSyst;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    BackendVariable::traverseBackendDAEVarsWithUpdate(
        outShared.aliasVars.clone(),
        (std::sync::Arc::new(replaceDummyDerivativesVar)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    )> + 'static,
            >),
        ht.clone(),
    )?;
    BackendVariable::traverseBackendDAEVarsWithUpdate(
        outShared.globalKnownVars.clone(),
        (std::sync::Arc::new(replaceDummyDerivativesVar)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<
                                    Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>,
                                >,
                            ),
                            i32,
                            (
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                        ) -> Result<bool>
                                        + 'static,
                                >,
                                Arc<
                                    dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    )> + 'static,
            >),
        ht.clone(),
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        outShared.initialEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(replaceDummyDerivativesExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        )> + 'static,
                >),
            ht.clone(),
        ),
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        outSyst.removedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(replaceDummyDerivativesExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        )> + 'static,
                >),
            ht.clone(),
        ),
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        outShared.removedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(replaceDummyDerivativesExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<
                                    metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>,
                                >,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            metamodelica::Ref<DAE::Exp>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                (metamodelica::Ref<DAE::ComponentRef>, i32),
                                            )
                                                -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                        )> + 'static,
                >),
            ht,
        ),
    )?;
    Ok((outSyst, outShared))
}

fn replaceDummyDerivativesVar(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                        (metamodelica::Ref<DAE::ComponentRef>, i32),
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn((metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut ht: (
        metamodelica::Array<metamodelica::List<((metamodelica::Ref<DAE::ComponentRef>, i32), i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<((metamodelica::Ref<DAE::ComponentRef>, i32), metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableCrIntToExp::FuncHashCref,
            HashTableCrIntToExp::FuncCrefEqual,
            HashTableCrIntToExp::FuncCrefStr,
            HashTableCrIntToExp::FuncExpStr,
        ),
    ) = ht;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            (e1, _) = Expression::traverseExpBottomUp(e.clone(), &replaceDummyDerivativesExp, ht.clone())?;
            if !(referenceEq(&*(e.clone()),&*(&*e1))) {
                var = BackendVariable::setBindExp(var, Some(e1));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (attr, _) = BackendDAEUtil::traverseBackendDAEVarAttr(var.values.clone(), &replaceDummyDerivativesExp, ht.clone())?;
    if !(match (&(attr), &(var.values)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) {
        var = BackendVariable::setVarAttributes(var, attr);
    }
    Ok((var, ht))
}

pub(crate) fn splitEqnsinConstraintAndOther(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqnsLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outCEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outOEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut ne: i32;
    let mut nv: i32;
    let mut vec1: metamodelica::Array<i32>;
    let mut vec2: metamodelica::Array<i32>;
    let mut unassigned: metamodelica::List<i32>;
    let mut assigned: metamodelica::List<i32>;
    vars = BackendVariable::listVar1(inVarLst)?;
    (eqnslst, _) = InlineArrayEquations::getScalarArrayEqns(inEqnsLst);
    eqns = BackendEquation::listEquation(&eqnslst)?;
    syst = BackendDAEUtil::createEqSystem(
        vars.clone(),
        eqns.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (me, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst, shared, false)?;
    m = adjacencyMatrixfromEnhancedStrict(me.clone(), vars.clone())?;
    nv = BackendVariable::varsSize(&vars);
    ne = BackendEquation::equationArraySize(eqns.clone())?;
    vec1 = arrayCreate(nv, -1);
    vec2 = arrayCreate(ne, -1);
    Matching::matchingExternalsetAdjacencyMatrix(nv, ne, m.clone())?;
    BackendDAEEXT::matching(nv, ne, 5, -1, metamodelica::OrderedFloat(1.0_f64), 1);
    BackendDAEEXT::getAssignment(vec2.clone(), vec1.clone())?;
    unassigned = Matching::getUnassigned(ne, vec2.clone(), metamodelica::nil())?;
    assigned = Matching::getAssigned(ne, vec2.clone(), metamodelica::nil())?;
    unassigned = List::map1r(unassigned, &arrayGet, mapIncRowEqn.clone())?;
    unassigned = List::uniqueIntN(&unassigned, ne)?;
    outCEqnsLst = BackendEquation::getList(unassigned, eqns.clone())?;
    assigned = List::map1r(assigned, &arrayGet, mapIncRowEqn.clone())?;
    assigned = List::uniqueIntN(&assigned, ne)?;
    outOEqnsLst = BackendEquation::getList(assigned, eqns)?;
    Ok((outCEqnsLst, outOEqnsLst))
}

fn changeDerVariablesToStatesFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::List<i32>,
        i32,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::List<i32>,
        i32,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::List<i32>,
        i32,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    );
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: metamodelica::List<i32>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
            (vars, ilst) = algebraicState(&varlst, &changedVars, vars.clone(), ilst.clone())?;
            (e.clone(), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: metamodelica::List<i32>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
            (vars, ilst) = increaseDifferentiation(varlst, changedVars, 2, vars.clone(), ilst.clone())?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![e.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 })], attr: DAE::callAttrBuiltinReal().clone() }), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: index }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, (vars, eqns, ilst, eindx, mapIncRowEqn, mt)) => {
            let mut changedVars: metamodelica::List<i32>;
            let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut vars = (*vars).clone();
            let mut ilst = (*ilst).clone();
            (varlst, changedVars) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
            (vars, ilst) = increaseDifferentiation(varlst, changedVars, index.clone(), vars.clone(), ilst.clone())?;
            (e.clone(), (vars.clone(), eqns.clone(), ilst.clone(), eindx.clone(), mapIncRowEqn.clone(), mt.clone()))
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn algebraicState<'__b>(
    mut inVarLst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inIndxLst: &'__b metamodelica::List<i32>,
    mut inVars: BackendDAE::Variables,
    mut iChangedVars: metamodelica::List<i32>,
) -> Result<(BackendDAE::Variables, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inVarLst, inIndxLst) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inVars, iChangedVars))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: vlst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: ilst }) => {
                let mut changedVars: metamodelica::List<i32>;
                let mut vars: BackendDAE::Variables;
                { (inVarLst, inIndxLst, inVars, iChangedVars) = (vlst, ilst, inVars, iChangedVars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: v, tail: vlst }, Deref @ metamodelica::ListNode::Cons { head: index, tail: ilst }) => {
                let mut changedVars: metamodelica::List<i32>;
                let mut vars: BackendDAE::Variables;
                let mut v = (*v).clone();
                v = BackendVariable::setVarKind(v.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false })?;
                if !(BackendVariable::varHasStateSelect(metamodelica::AsArg::as_arg(&v))) {
                    v = BackendVariable::setVarStateSelect(v.clone(), openmodelica_frontend_types::DAE::StateSelect::NEVER)?;
                }
                vars = BackendVariable::addVar(v.clone(), inVars)?;
                { (inVarLst, inIndxLst, inVars, iChangedVars) = (vlst, ilst, vars, metamodelica::cons(index.clone(), iChangedVars)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn increaseDifferentiation(
    mut inVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iVarIndxs: metamodelica::List<i32>,
    mut counter: i32,
    mut inVars: BackendDAE::Variables,
    mut iChangedVars: metamodelica::List<i32>,
) -> Result<(BackendDAE::Variables, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVarLst.clone(), iVarIndxs)) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((inVars, iChangedVars))
            },
            (Deref @ metamodelica::ListNode::Cons { head: var @ Deref @ BackendDAE::Var { .. }, tail: vlst }, Deref @ metamodelica::ListNode::Cons { head: i, tail: ilst }) => {
                let mut dcr: Option<metamodelica::Ref<DAE::ComponentRef>>;
                let mut vars: BackendDAE::Variables;
                let mut diffcounter: i32;
                let mut b: bool;
                let mut natural: bool;
                let mut changedVars: metamodelica::List<i32>;
                let mut var = (*var).clone();
                let mut ilst = (*ilst).clone();
                if BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&var)) {
                    let BackendDAE::STATE { index: __pa0, derName: __pa1, natural: __pa2 } = (var.varKind.clone()) else { return Err("pattern mismatch") };
                    diffcounter = metamodelica::Own::own(__pa0);
                    dcr = metamodelica::Own::own(__pa1);
                    natural = metamodelica::Own::own(__pa2);
                } else {
                    (diffcounter, dcr, natural) = (0, None, false);
                }
                b = intGt(counter, diffcounter);
                diffcounter = if (b) {counter} else {diffcounter};
                var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: diffcounter, derName: dcr, natural: natural })?;
                vars = if (b) {BackendVariable::addVar(var.clone(), inVars)?} else {inVars};
                changedVars = List::consOnTrue(b, i.clone(), iChangedVars);
                { (inVarLst, iVarIndxs, counter, inVars, iChangedVars) = (vlst.clone(), ilst.clone(), counter, vars, changedVars); continue '__tco; }
            },
            _ => {
                metamodelica::print(literal!("IndexReduction.increaseDifferentiation failt because of wrong input:\n"));
                BackendDump::printVar(&((inVarLst).head().cloned()?))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn debugdifferentiateEqns(
    mut inTpl: &(
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::Ref<BackendDAE::Equation>,
        i32,
    ),
) -> Result<()> {
    let mut a: metamodelica::Ref<BackendDAE::Equation>;
    let mut b: metamodelica::Ref<BackendDAE::Equation>;
    let mut idx: i32;
    (a, b, idx) = inTpl.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("------------------"));
        __mm_s.push_str(&*intString(idx));
        __mm_s.push_str(&*literal!("------------------\n"));
        __mm_s.push_str(&*literal!("Constraint equation to be differentiated:\n"));
        __mm_s.push_str(&*BackendDump::equationString(&a)?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("Differentiated equation:\n"));
        __mm_s.push_str(&*BackendDump::equationString(&b)?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn getSetVars(
    mut index: i32,
    mut setsize: i32,
    mut nCandidates: i32,
    mut nCEqns: i32,
    mut level: i32,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut crstates: metamodelica::Ref<DAE::ComponentRef>;
    let mut crset: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut oSetVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ocrA: metamodelica::Ref<DAE::ComponentRef>;
    let mut oAVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut realtp: metamodelica::Ref<DAE::Type>;
    let mut ocrJ: metamodelica::Ref<DAE::ComponentRef>;
    let mut oJVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut set: metamodelica::Ref<DAE::ComponentRef>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    set = ComponentReferenceBasics::makeCrefIdent(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$STATESET"));
            __mm_s.push_str(&*intString(index));
            ArcStr::from(__mm_s)
        },
        DAE::T_COMPLEX_DEFAULT().clone(),
        metamodelica::nil(),
    );
    tp = if (intGt(setsize, 1)) {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_REAL_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: setsize })],
        })
    } else {
        DAE::T_REAL_DEFAULT().clone()
    };
    crstates = ComponentReference::joinCrefs(
        &set,
        ComponentReferenceBasics::makeCrefIdent(literal!("x"), tp.clone(), metamodelica::nil()),
    )?;
    oSetVars = BackendVariable::generateArrayVar(
        crstates.clone(),
        BackendDAE::VarKind::STATE {
            index: 1,
            derName: None,
            natural: false,
        },
        tp,
        None,
    )?;
    oSetVars = List::map1(oSetVars, &BackendVariable::setVarFixed, false)?;
    crset = List::map(
        oSetVars.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    tp = if (intGt(setsize, 1)) {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
            dims: list![
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: setsize }),
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates })
            ],
        })
    } else {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                integer: nCandidates
            })],
        })
    };
    realtp = if (intGt(setsize, 1)) {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_REAL_DEFAULT().clone(),
            dims: list![
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: setsize }),
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nCandidates })
            ],
        })
    } else {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_REAL_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                integer: nCandidates
            })],
        })
    };
    ocrA = ComponentReference::joinCrefs(
        &set,
        ComponentReferenceBasics::makeCrefIdent(literal!("A"), tp.clone(), metamodelica::nil()),
    )?;
    oAVars = BackendVariable::generateArrayVar(
        ocrA.clone(),
        openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        tp,
        None,
    )?;
    oAVars = List::map1(oAVars, &BackendVariable::setVarFixed, true)?;
    oAVars = List::map1(
        oAVars,
        &BackendVariable::setVarStartValue,
        metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
    )?;
    oAVars = setSetAStart(&oAVars, 1, 1, nCandidates, metamodelica::nil())?;
    tp = if (intGt(nCEqns, 1)) {
        metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_REAL_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nCEqns })],
        })
    } else {
        DAE::T_REAL_DEFAULT().clone()
    };
    ocrJ = ComponentReference::joinCrefs(
        &set,
        ComponentReferenceBasics::makeCrefIdent(literal!("J"), tp.clone(), metamodelica::nil()),
    )?;
    oJVars = BackendVariable::generateArrayVar(
        ocrJ.clone(),
        openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        tp,
        None,
    )?;
    oJVars = List::map1(oJVars, &BackendVariable::setVarFixed, false)?;
    Ok((crstates, crset, oSetVars, ocrA, oAVars, realtp, ocrJ, oJVars))
}

fn setSetAStart<'__b>(
    mut iVars: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut n: i32,
    mut r: i32,
    mut nCandidates: i32,
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match iVars {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iAcc.reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
                let mut n1: i32;
                let mut r1: i32;
                let mut start: i32;
                let mut v = (*v).clone();
                start = if (intEq(n, r)) {1} else {0};
                v = BackendVariable::setVarStartValue(v.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: start }))?;
                n1 = if (intEq(n, nCandidates)) {1} else {n + 1};
                r1 = if (intEq(n, nCandidates)) {r + 1} else {r};
                { (iVars, n, r, nCandidates, iAcc) = (rest, n1, r1, nCandidates, metamodelica::cons(v.clone(), iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

// =============================================================================
// set the derivative information to the states
// use equations der(s) = v and set s:STATE(derivativeName=v)
// =============================================================================
pub(crate) fn findStateOrder(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    systs = List::map(systs, &findStateOrderWork)?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs,
        shared: shared,
    });
    Ok(outDAE)
}

fn findStateOrderWork(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem> = inSystem.clone();
    assign_field!(
        outSystem.orderedVars = BackendEquation::traverseEquationArray(
            inSystem.orderedEqs.clone(),
            &fnptr!(
                traverseFindStateOrder,
                metamodelica::Ref<BackendDAE::Equation>,
                BackendDAE::Variables
            ),
            inSystem.orderedVars.clone()
        )?
    );
    Ok(outSystem)
}

fn traverseFindStateOrder(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inVars: BackendDAE::Variables,
) -> (metamodelica::Ref<BackendDAE::Equation>, BackendDAE::Variables) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outVars: BackendDAE::Variables;
    (outEq, outVars) = 'mc: {
        let __mc_input = (inEq.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, v) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut dvlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut v = (*v).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::derivativeEquation(metamodelica::AsArg::as_arg(&e))?) {
                        (__pa0, __pa1, _, _, false) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = metamodelica::Own::own(__pa0);
                    dcr = metamodelica::Own::own(__pa1);
                    (vlst, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&v))?;
                    (dvlst, _) = BackendVariable::getVar(dcr.clone(), metamodelica::AsArg::as_arg(&v))?;
                    v = addStateOrderFinder(&vlst, &dvlst, v.clone())?;
                    Ok((e.clone(), v.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), inVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, outVars)
}

fn addStateOrderFinder<'__b>(
    mut iVlst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iDerVlst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    '__tco: loop {
        ::match_deref::match_deref! { match (iVlst, iDerVlst) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inVars)
            },
            (Deref @ metamodelica::ListNode::Cons { head: var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: vlst }, Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: dcr, .. }, tail: dvlst }) => {
                let mut vars: BackendDAE::Variables;
                let mut var = (*var).clone();
                var = BackendVariable::setStateDerivative(var.clone(), Some(dcr.clone()))?;
                vars = BackendVariable::addVar(var.clone(), inVars)?;
                { (iVlst, iDerVlst, inVars) = (vlst, dvlst, vars); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: var, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: dvar, tail: _ }) => {
                let mut msg: ArcStr;
                msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IndexReduction.addStateOrderFinder failed for ")); __mm_s.push_str(&*BackendDump::varString(metamodelica::AsArg::as_arg(&var))?); __mm_s.push_str(&*literal!(" with derivative ")); __mm_s.push_str(&*BackendDump::varString(metamodelica::AsArg::as_arg(&dvar))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg])?;
                return Ok(return Err("fail"))
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("IndexReduction.addStateOrderFinder failed!")])?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpStates(mut state: (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(Util::tuple22(state.clone())));
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(Util::tuple21(state)),
        )?);
        ArcStr::from(__mm_s)
    };
    Ok(outStr)
}

/* *****************************************
DAEHandler stuff
*****************************************/
fn addStateOrder(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut dcr: metamodelica::Ref<DAE::ComponentRef>,
    mut inStateOrder: BackendDAE::StateOrder,
) -> Result<BackendDAE::StateOrder> {
    let mut outStateOrder: BackendDAE::StateOrder;
    outStateOrder = 'mc: {
        let __mc_input = inStateOrder.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::StateOrder::STATEORDER {
                hashTable: mut ht,
                invHashTable: mut dht,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut ht1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTableCG::FuncHashCref,
                    HashTableCG::FuncCrefEqual,
                    HashTableCG::FuncCrefStr,
                    HashTableCG::FuncExpStr,
                ),
            );
            let mut dht1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTable3::FuncHashCref,
                    HashTable3::FuncCrefEqual,
                    HashTable3::FuncCrefStr,
                    HashTable3::FuncExpStr,
                ),
            );
            ht1 = BaseHashTable::add((cr.clone(), dcr.clone()), ht.clone())?;
            if '__try0: {
                unwrap_break_err!(getDerStateOrder(dcr.clone(), inStateOrder.clone()), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            dht1 = BaseHashTable::add((dcr.clone(), list![cr.clone()]), dht.clone())?;
            Ok(BackendDAE::StateOrder::STATEORDER {
                hashTable: ht1.clone(),
                invHashTable: dht1.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::StateOrder::STATEORDER {
                hashTable: mut ht,
                invHashTable: mut dht,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut ht1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTableCG::FuncHashCref,
                    HashTableCG::FuncCrefEqual,
                    HashTableCG::FuncCrefStr,
                    HashTableCG::FuncExpStr,
                ),
            );
            let mut dht1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTable3::FuncHashCref,
                    HashTable3::FuncCrefEqual,
                    HashTable3::FuncCrefStr,
                    HashTable3::FuncExpStr,
                ),
            );
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            ht1 = BaseHashTable::add((cr.clone(), dcr.clone()), ht.clone())?;
            crlst = getDerStateOrder(dcr.clone(), inStateOrder.clone())?;
            dht1 = BaseHashTable::add(
                (dcr.clone(), metamodelica::cons(cr.clone(), crlst.clone())),
                dht.clone(),
            )?;
            Ok(BackendDAE::StateOrder::STATEORDER {
                hashTable: ht1.clone(),
                invHashTable: dht1.clone(),
            })
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStateOrder)
}

fn getStateOrder(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut inStateOrder: BackendDAE::StateOrder,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    let BackendDAE::STATEORDER { hashTable: __pa0, .. } = (inStateOrder) else {
        return Err("pattern mismatch");
    };
    ht = metamodelica::Own::own(__pa0);
    dcr = BaseHashTable::get(cr, &ht)?;
    Ok(dcr)
}

fn getDerStateOrder(
    mut dcr: metamodelica::Ref<DAE::ComponentRef>,
    mut inStateOrder: BackendDAE::StateOrder,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let BackendDAE::STATEORDER {
        invHashTable: __pa0, ..
    } = (inStateOrder)
    else {
        return Err("pattern mismatch");
    };
    dht = metamodelica::Own::own(__pa0);
    crlst = BaseHashTable::get(dcr, &dht)?;
    Ok(crlst)
}

fn addOrgEqn(
    mut e: i32,
    mut inEqn: metamodelica::Ref<BackendDAE::Equation>,
    mut inOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut outOrgEqns: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outOrgEqns = inOrgEqns.clone();
    eqs = metamodelica::arrayGet(inOrgEqns.clone(), e)?;
    eqs = metamodelica::cons(inEqn, eqs);
    metamodelica::arrayUpdate(outOrgEqns.clone(), e, eqs)?;
    Ok(outOrgEqns)
}
