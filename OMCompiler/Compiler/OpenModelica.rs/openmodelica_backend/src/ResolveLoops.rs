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

use crate::AvlSetInt;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::Tearing;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type IntList = metamodelica::List<i32>;

pub(crate) fn resolveLoops(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    (eqSysts, shared, _) = List::mapFold2(
        &inDAE.eqs,
        &fnptr!(
            resolveLoops_main,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            i32
        ),
        inDAE.shared.clone(),
        1,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqSysts,
        shared: shared,
    });
    Ok(outDAE)
}

fn resolveLoops_main(
    mut inEqSys: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inSysIdx: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
) {
    let mut outEqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut outSysIdx: i32;
    (outEqSys, outSysIdx) = 'mc: {
        let __mc_input = inEqSys.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. } => {
                    let mut numSimpEqs: i32;
                    let mut numVars: i32;
                    let mut eqMapArr: metamodelica::Array<i32>;
                    let mut varMapArr: metamodelica::Array<i32>;
                    let mut nonLoopEqMark: metamodelica::Array<i32>;
                    let mut markLinEqVars: metamodelica::Array<i32>;
                    let mut eqMapping: metamodelica::List<i32>;
                    let mut partitions: metamodelica::List<metamodelica::List<i32>>;
                    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut simpVars: BackendDAE::Variables;
                    let mut simpEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut m_cut: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mT_cut: metamodelica::Array<metamodelica::List<i32>>;
                    let mut m_after: metamodelica::Array<metamodelica::List<i32>>;
                    let mut simpEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut simpVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let mut eqs = (*eqs).clone();
                    (m, _) = BackendDAEUtil::adjacencyMatrix(metamodelica::AsArg::as_arg(&syst), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(&inShared))?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        BackendDump::dumpBipartiteGraphEqSystem(syst.clone(), &inShared, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("whole System_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }))?;
                    }
                    markLinEqVars = arrayCreate(BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars)), -1);
                    (simpEqLst, eqMapping, _, _, markLinEqVars, _) = BackendEquation::traverseEquationArray(eqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::List<i32>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<metamodelica::List<i32>>)| getSimpleEquations(__a0, &__a1), (metamodelica::nil(), metamodelica::nil(), 1, vars.clone(), markLinEqVars.clone(), m.clone()))?;
                    eqMapArr = metamodelica::arrayFromVec(eqMapping.clone().into_iter().cloned().collect());
                    (simpVarLst, varMapArr) = getSimpleEquationVariables(markLinEqVars.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    simpEqs = BackendEquation::listEquation(&simpEqLst)?;
                    simpVars = BackendVariable::listVar1(&simpVarLst)?;
                    numSimpEqs = ((simpEqLst).len() as i32);
                    numVars = ((simpVarLst).len() as i32);
                    (m, mT) = BackendDAEUtil::adjacencyMatrixDispatch(&simpVars, simpEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(&inShared))?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        varAtts = List::threadMap(List::fill(false, numVars), List::fill(literal!(""), numVars), &fnptr!(Util::makeTuple, _, _))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs), List::fill(literal!(""), numSimpEqs), &fnptr!(Util::makeTuple, _, _))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m.clone(), varAtts.clone(), &eqAtts, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_simpEqs_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }))?;
                    }
                    partitions = partitionBipartiteGraph(m.clone(), mT.clone())?;
                    partitions = List::filterOnTrue(partitions.clone(), std::sync::Arc::new(move |__a0: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(List::hasSeveralElements(&__a0)) }))?;
                    m_cut = metamodelica::arrayFromVec(m.clone().borrow().clone());
                    mT_cut = metamodelica::arrayFromVec(mT.clone().borrow().clone());
                    (_, nonLoopEqMark) = resolveLoops_cutNodes(m_cut.clone(), mT_cut.clone())?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        varAtts = List::threadMap(List::fill(false, numVars), List::fill(literal!(""), numVars), &fnptr!(Util::makeTuple, _, _))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs), List::fill(literal!(""), numSimpEqs), &fnptr!(Util::makeTuple, _, _))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_cut.clone(), varAtts.clone(), &eqAtts, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_loops_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }))?;
                    }
                    eqs = resolveLoops_resolvePartitions(&partitions, m_cut.clone(), mT_cut.clone(), m.clone(), mT.clone(), eqMapArr.clone(), varMapArr.clone(), eqs.clone(), metamodelica::AsArg::as_arg(&vars), nonLoopEqMark.clone())?;
                    assign_field!(syst.orderedEqs = eqs.clone());
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        simpEqLst = BackendEquation::getList(eqMapping.clone(), eqs.clone())?;
                        simpEqs = BackendEquation::listEquation(&simpEqLst)?;
                        numSimpEqs = ((simpEqLst).len() as i32);
                        numVars = ((simpVarLst).len() as i32);
                        (m_after, _) = BackendDAEUtil::adjacencyMatrixDispatch(&simpVars, simpEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(&inShared))?;
                        varAtts = List::threadMap(List::fill(false, numVars), List::fill(literal!(""), numVars), &fnptr!(Util::makeTuple, _, _))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs), List::fill(literal!(""), numSimpEqs), &fnptr!(Util::makeTuple, _, _))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_after.clone(), varAtts.clone(), &eqAtts, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_after_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }))?;
                    }
                    syst = BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst));
                    Ok((syst.clone(), inSysIdx + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEqSys.clone(), inSysIdx + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEqSys, outShared, outSysIdx)
}

fn resolveLoops_resolvePartitions(
    mut partitionsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut m_uncut: metamodelica::Array<metamodelica::List<i32>>,
    mut mT_uncut: metamodelica::Array<metamodelica::List<i32>>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
    mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut daeVars: &BackendDAE::Variables,
    mut nonLoopEqMark: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut daeEqsOut: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    daeEqsOut = (::match_deref::match_deref! { match partitionsIn {
        Deref @ metamodelica::ListNode::Cons { head: partition, tail: rest } => {
            let mut optStructureMapping: Option<(metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<metamodelica::List<i32>>)>;
            let mut eqCrossLst: metamodelica::List<i32>;
            let mut varCrossLst: metamodelica::List<i32>;
            let mut mapIndices: metamodelica::List<i32>;
            let mut loops: metamodelica::List<metamodelica::List<i32>>;
            let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut map: metamodelica::Array<metamodelica::List<i32>>;
            let mut partition = (*partition).clone();
            partition = List::filter1OnTrue(partition.clone(), (std::sync::Arc::new(arrayIsZeroAt) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), nonLoopEqMark.clone())?;
            if (partition).is_empty() {
                eqs = resolveLoops_resolvePartitions(rest, mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVars, nonLoopEqMark.clone())?;
            } else {
                (loops, eqCrossLst, varCrossLst, optStructureMapping) = resolveLoops_findLoops(&(list![partition.clone()]), mIn.clone(), mTIn.clone(), false);
                if (optStructureMapping).is_some() {
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(optStructureMapping) {
                        Some((__pa0, __pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    mapIndices = metamodelica::Own::own(__pa0);
                    map = metamodelica::Own::own(__pa1);
                    loops = metamodelica::Own::own(__pa2);
                    loops = List::filter1OnTrueAndUpdate(loops, &move |__a0: metamodelica::List<i32>, __a1: TripleLoopInfo| evaluateTripleLoop(&__a0, &__a1), &move |__a0: metamodelica::List<i32>, __a1: TripleLoopInfo| updateTripleLoop(__a0, &__a1), tripleLoopInfo(m_uncut.clone(), metamodelica::arrayLength(mT_uncut.clone()), mapIndices, map.clone())?)?;
                } else {
                    loops = List::filterOnFalse(loops, &fnptr!(listEmpty, _))?;
                    loops = List::filter1OnTrue(loops, (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: (metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<i32>)| evaluateLoop(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, (metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<i32>)) -> Result<bool> + 'static>), (m_uncut.clone(), mT_uncut.clone(), eqCrossLst.clone()))?;
                }
                (eqs, _) = resolveLoops_resolveAndReplace(loops, &eqCrossLst, &varCrossLst, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVars, metamodelica::nil())?;
                eqs = resolveLoops_resolvePartitions(rest, mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), eqs, daeVars, nonLoopEqMark.clone())?;
            }
            eqs
        },
        Deref @ metamodelica::ListNode::Nil => {
            daeEqs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(daeEqsOut)
}

fn resolveLoops_cutNodes(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut deadEndVarsMark: metamodelica::Array<i32> = Default::default();
    let mut deadEndEqsMark: metamodelica::Array<i32> = Default::default();
    (deadEndVarsMark, deadEndEqsMark) = 'mc: {
        let __mc_input = mTIn.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut numVars: i32;
            let mut numEqs: i32;
            let mut idx: i32 = 0;
            let mut loopVars: metamodelica::List<i32>;
            let mut loopEqs: metamodelica::List<i32>;
            let mut nonLoopVars: metamodelica::List<i32>;
            let mut deadEndEqsMark: metamodelica::Array<i32> = deadEndEqsMark.clone();
            let mut deadEndVarsMark: metamodelica::Array<i32> = deadEndVarsMark.clone();
            numVars = metamodelica::arrayLength(mTIn.clone());
            numEqs = metamodelica::arrayLength(mIn.clone());
            nonLoopVars = List::filter2OnTrue(
                List::intRange(numVars),
                (std::sync::Arc::new(arrayEntryLengthIs)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(i32, metamodelica::Array<metamodelica::List<i32>>, i32) -> Result<bool>
                            + 'static,
                    >),
                mTIn.clone(),
                1,
            )?;
            deadEndVarsMark = arrayCreate(numVars, 0);
            deadEndEqsMark = arrayCreate(numVars, 0);
            for mut idx in &*nonLoopVars {
                let mut idx = idx.clone();
                metamodelica::arrayUpdate(deadEndVarsMark.clone(), idx, 1)?;
            }
            for mut idx in &*nonLoopVars {
                let mut idx = idx.clone();
                markDeadEndsInBipartiteGraph(
                    idx,
                    mIn.clone(),
                    mTIn.clone(),
                    deadEndEqsMark.clone(),
                    deadEndVarsMark.clone(),
                )?;
            }
            idx = 1;
            while idx <= numVars {
                if metamodelica::arrayGet(deadEndVarsMark.clone(), idx)? == 1 {
                    metamodelica::arrayUpdate(mTIn.clone(), idx, metamodelica::nil())?;
                } else {
                    loopEqs = metamodelica::arrayGet(mTIn.clone(), idx)?;
                    loopEqs = List::filter1OnTrue(
                        loopEqs.clone(),
                        (std::sync::Arc::new(arrayIsZeroAt)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static,
                            >),
                        deadEndEqsMark.clone(),
                    )?;
                    metamodelica::arrayUpdate(mTIn.clone(), idx, loopEqs.clone())?;
                }
                idx = idx + 1;
            }
            idx = 1;
            while idx <= numEqs {
                if metamodelica::arrayGet(deadEndEqsMark.clone(), idx)? == 1 {
                    metamodelica::arrayUpdate(mIn.clone(), idx, metamodelica::nil())?;
                } else {
                    loopVars = metamodelica::arrayGet(mIn.clone(), idx)?;
                    loopVars = List::filter1OnTrue(
                        loopVars.clone(),
                        (std::sync::Arc::new(arrayIsZeroAt)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static,
                            >),
                        deadEndVarsMark.clone(),
                    )?;
                    metamodelica::arrayUpdate(mIn.clone(), idx, loopVars.clone())?;
                }
                idx = idx + 1;
            }
            Ok((
                (deadEndVarsMark.clone(), deadEndEqsMark.clone()),
                deadEndEqsMark.clone(),
                deadEndVarsMark.clone(),
            ))
        })() {
            deadEndEqsMark = __wb0;
            deadEndVarsMark = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function resolveLoops_cutNodes failed"),
                metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((deadEndVarsMark, deadEndEqsMark))
}

fn arrayEntryLengthIs(
    mut idx: i32,
    mut arr: metamodelica::Array<metamodelica::List<i32>>,
    mut len: i32,
) -> Result<bool> {
    let mut eqLen: bool;
    let mut entry: metamodelica::List<i32>;
    let mut len1: i32;
    entry = metamodelica::arrayGet(arr.clone(), idx)?;
    len1 = ((entry).len() as i32);
    eqLen = intEq(len, len1);
    Ok(eqLen)
}

fn getSimpleEquations(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<i32>,
        i32,
        BackendDAE::Variables,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<i32>,
        i32,
        BackendDAE::Variables,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<i32>,
        i32,
        BackendDAE::Variables,
        metamodelica::Array<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
    );
    let mut isSimple: bool;
    let mut idx: i32;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut vars: BackendDAE::Variables;
    let mut markLinEqVars: metamodelica::Array<i32>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut idxMap: metamodelica::List<i32>;
    (eqLst, idxMap, idx, vars, markLinEqVars, m) = inTpl.clone();
    if BackendEquation::isEquation(&inEq) && !(eqIsConst(&inEq)) {
        let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(
            inEq,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (bool, BackendDAE::Variables)| {
                    isAddOrSubExp(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (bool, BackendDAE::Variables),
                        )
                            -> Result<(metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables))>
                        + 'static,
                >),
            (true, vars.clone()),
        )?;
        eq = metamodelica::Own::own(__pa0);
        isSimple = metamodelica::Own::own(__pa1);
        if isSimple {
            eqLst = metamodelica::cons(eq, eqLst);
            idxMap = metamodelica::cons(idx, idxMap);
            let __range2 = &*({
                let __elt = (*metamodelica::index_checked(&m.borrow(), idx)?).clone();
                __elt
            });
            for mut varIdx in __range2 {
                metamodelica::arrayUpdate(markLinEqVars.clone(), intAbs(varIdx.clone()), 1)?;
            }
        }
    }
    outTpl = (eqLst, idxMap, idx + 1, vars, markLinEqVars.clone(), m.clone());
    Ok((outEq, outTpl))
}

fn getSimpleEquationVariables(
    mut markLinEqVars: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::Array<i32>,
)> {
    let mut simpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut varMapArr: metamodelica::Array<i32>;
    let mut varIdx: i32 = 0;
    let mut varMap: metamodelica::List<i32>;
    varMap = metamodelica::nil();
    for mut varIdx in 1..=metamodelica::arrayLength(markLinEqVars.clone()) {
        if ({
            let __elt = (*metamodelica::index_checked(&markLinEqVars.borrow(), varIdx)?).clone();
            __elt
        }) > 0
        {
            varMap = metamodelica::cons(varIdx, varMap);
            simpVars = metamodelica::cons(BackendVariable::getVarAt(vars, varIdx)?, simpVars);
        }
    }
    varMapArr = metamodelica::arrayFromVec(varMap.into_iter().cloned().collect());
    Ok((simpVars, varMapArr))
}

pub(crate) fn resolveLoops_findLoops(
    mut partitionsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut findExactlyOneLoop: bool,
) -> (
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    Option<(
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::List<i32>>,
    )>,
) {
    let mut loopsOut: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut crossEqsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut crossVarsOut: metamodelica::List<i32> = metamodelica::nil();
    let mut optStructureMapping: Option<(
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::List<i32>>,
    )> = None;
    let mut loops: metamodelica::List<metamodelica::List<i32>>;
    let mut eqVars: metamodelica::List<metamodelica::List<i32>>;
    let mut eqCrossLst: metamodelica::List<i32>;
    let mut varCrossLst: metamodelica::List<i32>;
    let mut partitionVars: metamodelica::List<i32>;
    let mut set: metamodelica::Ref<AvlSetInt::Tree>;
    for mut partition in &**partitionsIn {
        match '__try0: {
            eqVars = unwrap_break_err!(List::map1(partition.clone(), &Array::getIndexFirst, mIn.clone()), '__try0);
            set = crate::AvlSetInt::Tree::interned_EMPTY();
            for mut vars in &*eqVars {
                set = unwrap_break_err!(AvlSetInt::addList(set.clone(), metamodelica::AsArg::as_arg(&vars)), '__try0);
            }
            partitionVars = AvlSetInt::listKeys(&set, metamodelica::nil());
            eqCrossLst = unwrap_break_err!(List::fold2(metamodelica::AsArg::as_arg(&partition), &gatherCrossNodes, mIn.clone(), mTIn.clone(), metamodelica::nil()), '__try0);
            varCrossLst = unwrap_break_err!(List::fold2(&partitionVars, &gatherCrossNodes, mTIn.clone(), mIn.clone(), metamodelica::nil()), '__try0);
            (loops, optStructureMapping) = unwrap_break_err!(resolveLoops_findLoops2(partition.clone(), eqCrossLst.clone(), varCrossLst.clone(), mIn.clone(), mTIn.clone(), findExactlyOneLoop), '__try0);
            if if (findExactlyOneLoop) {
                !((loops).is_empty()) && !((loopsOut).is_empty())
            } else {
                false
            } {
                break '__try0 Err::<_, _>("fail");
            }
            loopsOut = listAppend(loops.clone(), loopsOut.clone());
            if true
            /* isPresent not implemented in Rust */
            {
                crossEqsOut = listAppend(eqCrossLst.clone(), crossEqsOut.clone());
            }
            if true
            /* isPresent not implemented in Rust */
            {
                crossVarsOut = listAppend(varCrossLst.clone(), crossVarsOut.clone());
            }
            Ok::<_, &'static str>((
                eqCrossLst.clone(),
                eqVars.clone(),
                loops.clone(),
                loopsOut.clone(),
                optStructureMapping.clone(),
                partitionVars.clone(),
                set.clone(),
                varCrossLst.clone(),
            ))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
                eqCrossLst = __try0_o0;
                eqVars = __try0_o1;
                loops = __try0_o2;
                loopsOut = __try0_o3;
                optStructureMapping = __try0_o4;
                partitionVars = __try0_o5;
                set = __try0_o6;
                varCrossLst = __try0_o7;
            }
            Err(_) => {
                return (loopsOut, crossEqsOut, crossVarsOut, optStructureMapping);
            }
        }
    }
    (loopsOut, crossEqsOut, crossVarsOut, optStructureMapping)
}

fn resolveLoops_findLoops2(
    mut eqsIn: metamodelica::List<i32>,
    mut eqCrossLstIn: metamodelica::List<i32>,
    mut varCrossLstIn: metamodelica::List<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut findExactlyOneLoop: bool,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    Option<(
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::List<i32>>,
    )>,
)> {
    let mut loopsOut: metamodelica::List<metamodelica::List<i32>>;
    let mut structureMapping: Option<(
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<metamodelica::List<i32>>,
    )>;
    (loopsOut, structureMapping) = (::match_deref::match_deref! { match &((eqCrossLstIn.clone(), varCrossLstIn.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Nil) => {
            let mut isNoSingleLoop: bool;
            let mut eqCrossLst: metamodelica::List<i32>;
            let mut subLoop: metamodelica::List<i32>;
            let mut mapIndices: metamodelica::List<i32>;
            let mut paths: metamodelica::List<metamodelica::List<i32>>;
            let mut allPaths: metamodelica::List<metamodelica::List<i32>>;
            let mut simpleLoops: metamodelica::List<metamodelica::List<i32>>;
            let mut tripleLoops: metamodelica::List<metamodelica::List<i32>>;
            let mut paths0: metamodelica::List<metamodelica::List<i32>>;
            let mut paths1: metamodelica::List<metamodelica::List<i32>>;
            let mut loopConnectors: metamodelica::List<metamodelica::List<i32>>;
            let mut connectedPaths: metamodelica::List<metamodelica::List<i32>>;
            let mut minAdj: metamodelica::Array<metamodelica::List<i32>>;
            let mut map: metamodelica::Array<metamodelica::List<i32>>;
            let mut mapping: (metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>);
            let mut optTripleMapping: Option<(metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<metamodelica::List<i32>>)>;
            allPaths = getPathTillNextCrossEq(eqCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqCrossLstIn.clone(), metamodelica::nil(), metamodelica::nil())?;
            allPaths = List::sort(allPaths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths1 = List::fold1(&(allPaths.clone()), &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<metamodelica::List<i32>>, __a2: metamodelica::List<metamodelica::List<i32>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getReverseDoubles(__a0, &__a1, __a2)) }, allPaths, metamodelica::nil())?;
            simpleLoops = getDoubles(paths1.clone(), metamodelica::nil())?;
            (_, paths, _) = List::intersection1OnTrue(paths1.clone(), simpleLoops.clone(), &intLstIsEqual)?;
            if (simpleLoops).is_empty() {
                (eqCrossLst, paths1, mapping, minAdj) = findEqualPathStructure(eqCrossLstIn, paths1)?;
                (mapIndices, map) = mapping;
                (tripleLoops, paths0) = getTriples(&eqCrossLst, minAdj.clone())?;
                optTripleMapping = Some((mapIndices, map.clone(), tripleLoops));
            } else {
                optTripleMapping = None;
                paths0 = List::sort(paths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                (connectedPaths, loopConnectors) = connect2PathsToLoops(paths0, metamodelica::nil(), metamodelica::nil())?;
                loopConnectors = List::filter1OnTrue(loopConnectors, (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<metamodelica::List<i32>>| connectsLoops(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>) -> Result<bool> + 'static>), simpleLoops.clone())?;
                simpleLoops = listAppend(simpleLoops, loopConnectors);
                subLoop = connectPathsToOneLoop(&simpleLoops, &(metamodelica::nil()));
                isNoSingleLoop = (subLoop).is_empty();
                simpleLoops = if (isNoSingleLoop) {simpleLoops} else {list![subLoop]};
                paths0 = listAppend(simpleLoops, connectedPaths);
                paths0 = sortPathsAsChain(paths0);
                if findExactlyOneLoop {
                    if !((paths0).is_empty()) {
                        ::match_deref::match_deref! { match &(paths0.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => (),
                            _ => return Err("pattern mismatch"),
                        } };
                    }
                }
            }
            (paths0, optTripleMapping)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
            let mut paths: metamodelica::List<metamodelica::List<i32>>;
            let mut paths0: metamodelica::List<metamodelica::List<i32>>;
            let mut paths1: metamodelica::List<metamodelica::List<i32>>;
            let mut closedPaths: metamodelica::List<metamodelica::List<i32>>;
            paths = getPathTillNextCrossEq(varCrossLstIn.clone(), mTIn.clone(), mIn.clone(), varCrossLstIn, metamodelica::nil(), metamodelica::nil())?;
            paths = List::sort(paths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths = paths.reverse();
            (paths0, paths1) = List::extract1OnTrue(&(paths.clone()), &move |__a0: metamodelica::List<i32>, __a1: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(listLengthIs(&__a0, __a1)) }, (((List::last(&paths)?)).len() as i32))?;
            paths1 = if ((paths1).is_empty()) {paths0.clone()} else {paths1};
            closedPaths = List::map1(paths1, &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<metamodelica::List<i32>>| closePathDirectly(__a0, &__a1), paths0)?;
            closedPaths = List::fold1(&(closedPaths.clone()), &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<metamodelica::List<i32>>, __a2: metamodelica::List<metamodelica::List<i32>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getReverseDoubles(__a0, &__a1, __a2)) }, closedPaths, metamodelica::nil())?;
            closedPaths = List::map(closedPaths, &move |__a0: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(List::unique(&__a0)) })?;
            closedPaths = List::map1(closedPaths, &getEqNodesForVarLoop, mTIn.clone())?;
            if findExactlyOneLoop {
                if !((closedPaths).is_empty()) {
                    ::match_deref::match_deref! { match &(closedPaths.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                }
            }
            (closedPaths, None)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut subLoop: metamodelica::List<i32>;
            subLoop = eqsIn.clone();
            for mut e in &*eqsIn {
                if (({let __elt = (*metamodelica::index_checked(&mIn.borrow(), e.clone())?).clone(); __elt})).is_empty() {
                    subLoop = metamodelica::nil();
                    break;
                }
            }
            (list![subLoop], None)
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
            let mut paths: metamodelica::List<metamodelica::List<i32>>;
            let mut eqCrossSet: metamodelica::Ref<AvlSetInt::Tree>;
            for mut i in 1..=metamodelica::arrayLength(mIn.clone()) {
                metamodelica::arrayUpdate(mIn.clone(), i, List::heapSortIntList(({let __elt = (*metamodelica::index_checked(&mIn.borrow(), i)?).clone(); __elt}))?)?;
            }
            for mut i in 1..=metamodelica::arrayLength(mTIn.clone()) {
                metamodelica::arrayUpdate(mTIn.clone(), i, List::heapSortIntList(({let __elt = (*metamodelica::index_checked(&mTIn.borrow(), i)?).clone(); __elt}))?)?;
            }
            eqCrossSet = AvlSetInt::addList(crate::AvlSetInt::Tree::interned_EMPTY(), &eqCrossLstIn)?;
            paths = getShortPathsBetweenEqCrossNodes(&(AvlSetInt::listKeysReverse(&eqCrossSet, metamodelica::nil())), eqCrossSet, mIn.clone(), mTIn.clone(), metamodelica::nil(), findExactlyOneLoop)?;
            (paths, None)
        },
        _ => {
            Error::addInternalError(literal!("function resolveLoops_findLoops2 failed"), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((loopsOut, structureMapping))
}

fn findEqualPathStructure(
    mut crossNodes: metamodelica::List<i32>,
    mut uniquePaths: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::List<i32>>,
    (metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>),
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut crossNodes: metamodelica::List<i32> = crossNodes;
    let mut uniquePaths: metamodelica::List<metamodelica::List<i32>> = uniquePaths;
    let mut mapping: (metamodelica::List<i32>, metamodelica::Array<metamodelica::List<i32>>);
    let mut minAdj: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIndices: metamodelica::List<i32>;
    let mut map: metamodelica::Array<metamodelica::List<i32>>;
    minAdj = getMinimalAdjacencyMatrix(crossNodes.clone(), &uniquePaths)?;
    (minAdj, uniquePaths, mapIndices, map, crossNodes) = removeEqualPaths(
        crossNodes.clone(),
        minAdj.clone(),
        uniquePaths,
        metamodelica::nil(),
        arrayCreate(
            ({
                let mut __acc: Option<i32> = None;
                for mut cn in (crossNodes).into_iter().cloned() {
                    let __x = cn.clone();
                    __acc = Some(match __acc {
                        None => __x,
                        Some(__cur) => {
                            if __x > __cur {
                                __x
                            } else {
                                __cur
                            }
                        }
                    });
                }
                __acc.unwrap_or((-i32::MAX))
            }),
            metamodelica::nil(),
        ),
    )?;
    mapping = (mapIndices, map.clone());
    Ok((crossNodes, uniquePaths, mapping, minAdj))
}

fn getMinimalAdjacencyMatrix(
    mut crossNodes: metamodelica::List<i32>,
    mut uniquePaths: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut minAdj: metamodelica::Array<metamodelica::List<i32>>;
    minAdj = arrayCreate(
        ({
            let mut __acc: Option<i32> = None;
            for mut cn in (crossNodes.clone()).into_iter().cloned() {
                let __x = cn.clone();
                __acc = Some(match __acc {
                    None => __x,
                    Some(__cur) => {
                        if __x > __cur {
                            __x
                        } else {
                            __cur
                        }
                    }
                });
            }
            __acc.unwrap_or((-i32::MAX))
        }),
        metamodelica::nil(),
    );
    for mut path in &**uniquePaths {
        let _ = (::match_deref::match_deref! { match &(path.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: a, tail: Deref @ metamodelica::ListNode::Cons { head: b, tail: Deref @ metamodelica::ListNode::Nil } } => {
                minAdj = Array::consToElement(a.clone(), b.clone(), minAdj.clone())?;
                minAdj = Array::consToElement(b.clone(), a.clone(), minAdj.clone())?;
                0
            },
            _ => {
                1
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    for mut cn in &*crossNodes {
        metamodelica::arrayUpdate(
            minAdj.clone(),
            cn.clone(),
            List::sort(
                metamodelica::arrayGet(minAdj.clone(), cn.clone())?,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
        )?;
    }
    Ok(minAdj)
}

fn removeEqualPaths(
    mut crossNodes: metamodelica::List<i32>,
    mut minAdj: metamodelica::Array<metamodelica::List<i32>>,
    mut uniquePaths: metamodelica::List<metamodelica::List<i32>>,
    mut mapIndices: metamodelica::List<i32>,
    mut map: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::List<i32>,
)> {
    let mut minAdj: metamodelica::Array<metamodelica::List<i32>> = minAdj;
    let mut uniquePaths: metamodelica::List<metamodelica::List<i32>> = uniquePaths;
    let mut mapIndices: metamodelica::List<i32> = mapIndices;
    let mut map: metamodelica::Array<metamodelica::List<i32>> = map;
    let mut accCrossNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut groups: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::List<i32>, i32>>;
    let mut groupOf: metamodelica::Array<i32> = arrayCreate(metamodelica::arrayLength(minAdj.clone()), 0);
    let mut merged: metamodelica::Array<bool> = arrayCreate(metamodelica::arrayLength(minAdj.clone()), false);
    let mut collected: metamodelica::Array<bool> = arrayCreate(metamodelica::arrayLength(minAdj.clone()), false);
    let mut cn1: i32;
    let mut numGroups: i32 = 0;
    let mut numMerged: i32 = 0;
    let mut row: metamodelica::List<i32>;
    let mut nodes: metamodelica::List<i32> = crossNodes.clone();
    let mut rest: metamodelica::List<i32>;
    let mut assigned: metamodelica::List<i32>;
    let mut unassigned: metamodelica::List<i32>;
    groups = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(hashIntList(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<i32>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(HpcOmTaskGraph::equalLists(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::List<i32>, metamodelica::List<i32>) -> Result<bool> + 'static,
            >),
        1,
    );
    for mut node in &*crossNodes {
        row = metamodelica::arrayGet(minAdj.clone(), node.clone())?;
        if !(UnorderedMap::contains(row.clone(), groups.clone())?) {
            numGroups = numGroups + 1;
            UnorderedMap::add(row.clone(), numGroups, groups.clone())?;
        }
        metamodelica::arrayUpdate(
            groupOf.clone(),
            node.clone(),
            UnorderedMap::getOrFail(row, groups.clone())?,
        )?;
    }
    while !((nodes).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cn1 = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !({
            let __elt = (*metamodelica::index_checked(&collected.borrow(), cn1)?).clone();
            __elt
        }) {
            metamodelica::arrayUpdate(collected.clone(), cn1, true)?;
            accCrossNodes = metamodelica::cons(cn1, accCrossNodes);
        }
        assigned = metamodelica::nil();
        unassigned = metamodelica::nil();
        for mut cn2 in &*rest {
            if ({
                let __elt = (*metamodelica::index_checked(&groupOf.borrow(), cn2.clone())?).clone();
                __elt
            }) == ({
                let __elt = (*metamodelica::index_checked(&groupOf.borrow(), cn1)?).clone();
                __elt
            }) {
                assigned = metamodelica::cons(cn2.clone(), assigned);
                metamodelica::arrayUpdate(minAdj.clone(), cn2.clone(), metamodelica::nil())?;
                metamodelica::arrayUpdate(merged.clone(), cn2.clone(), true)?;
                numMerged = numMerged + 1;
            } else {
                unassigned = metamodelica::cons(cn2.clone(), unassigned);
                if !({
                    let __elt = (*metamodelica::index_checked(&collected.borrow(), cn2.clone())?).clone();
                    __elt
                }) {
                    metamodelica::arrayUpdate(collected.clone(), cn2.clone(), true)?;
                    accCrossNodes = metamodelica::cons(cn2.clone(), accCrossNodes);
                }
            }
        }
        if !((assigned).is_empty()) {
            mapIndices = metamodelica::cons(cn1, mapIndices);
            map = Array::appendToElement(cn1, assigned, map.clone())?;
        }
        nodes = unassigned;
    }
    uniquePaths = ({
        let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
        for mut path in (uniquePaths).into_iter().cloned() {
            if !(!(pathContainsMerged(&(path.clone()), merged.clone())?)) {
                continue;
            }
            let __x = path.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if intMod(numMerged, 2) == 1 {
        uniquePaths = uniquePaths.reverse();
    }
    Ok((minAdj, uniquePaths, mapIndices, map, accCrossNodes))
}

fn hashIntList(mut lst: &metamodelica::List<i32>) -> i32 {
    let mut hash: i32 = 17;
    for mut i in &**lst {
        hash = intMod(hash * 31 + i.clone(), 65599);
    }
    hash
}

fn pathContainsMerged(mut path: &metamodelica::List<i32>, mut merged: metamodelica::Array<bool>) -> Result<bool> {
    let mut c: bool = false;
    for mut n in &**path {
        if n.clone() <= metamodelica::arrayLength(merged.clone())
            && ({
                let __elt = (*metamodelica::index_checked(&merged.borrow(), n.clone())?).clone();
                __elt
            })
        {
            c = true;
            return Ok(c);
        }
    }
    Ok(c)
}

fn listContains(mut lst: &metamodelica::List<i32>, mut int: i32) -> bool {
    let mut res: bool = false;
    for mut i in &**lst {
        if intEq(i.clone(), int) {
            res = true;
            return res;
        }
    }
    res
}

fn hasSameIntSortedExcept(
    mut inList1: metamodelica::List<i32>,
    mut inList2: metamodelica::List<i32>,
    mut excl: i32,
) -> Result<bool> {
    let mut rv: bool = false;
    let mut i1: i32;
    let mut i2: i32;
    let mut l1: metamodelica::List<i32> = inList1.clone();
    let mut l2: metamodelica::List<i32> = inList2.clone();
    if (inList1).is_empty() || (inList2).is_empty() {
        return Ok(rv);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l1) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    i1 = metamodelica::Own::own(__pa0);
    l1 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(l2) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    i2 = metamodelica::Own::own(__pa2);
    l2 = metamodelica::Own::own(__pa3);
    loop {
        if i1 > i2 {
            if (l2).is_empty() {
                return Ok(rv);
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(l2) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i2 = metamodelica::Own::own(__pa4);
            l2 = metamodelica::Own::own(__pa5);
        } else if i1 < i2 {
            if (l1).is_empty() {
                return Ok(rv);
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(l1) {
                Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i1 = metamodelica::Own::own(__pa6);
            l1 = metamodelica::Own::own(__pa7);
        } else {
            if i1 != excl {
                rv = true;
                return Ok(rv);
            }
            if (l1).is_empty() || (l2).is_empty() {
                return Ok(rv);
            }
            let (__pa8, __pa9) = ::match_deref::match_deref! { match &(l1) {
                Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i1 = metamodelica::Own::own(__pa8);
            l1 = metamodelica::Own::own(__pa9);
            let (__pa10, __pa11) = ::match_deref::match_deref! { match &(l2) {
                Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
                _ => return Err("pattern mismatch"),
            } };
            i2 = metamodelica::Own::own(__pa10);
            l2 = metamodelica::Own::own(__pa11);
        }
    }
    Ok(rv)
}

fn getShortPathsBetweenEqCrossNodes(
    mut eqCrossLstIn: &metamodelica::List<i32>,
    mut eqCrossSet: metamodelica::Ref<AvlSetInt::Tree>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut pathsIn: metamodelica::List<metamodelica::List<i32>>,
    mut findExactlyOneLoop: bool,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut pathsOut: metamodelica::List<metamodelica::List<i32>> = pathsIn;
    let mut hub: i32;
    let mut adjVars: metamodelica::List<i32>;
    let mut adjEqs: metamodelica::List<i32>;
    let mut newPath: metamodelica::List<i32>;
    let mut paths: metamodelica::List<metamodelica::List<i32>>;
    for mut crossEq in &**eqCrossLstIn {
        paths = metamodelica::nil();
        adjVars = metamodelica::arrayGet(mIn.clone(), crossEq.clone())?;
        hub = longestRow(adjVars.clone(), mTIn.clone())?;
        for mut adjVar in &*adjVars {
            adjEqs = if (adjVar.clone() == hub) {
                eqsSharingVia(&adjVars, hub, mIn.clone(), mTIn.clone())?
            } else {
                metamodelica::arrayGet(mTIn.clone(), adjVar.clone())?
            };
            for mut adjEq in &*adjEqs {
                if if (adjEq.clone() > crossEq.clone()) {
                    !(AvlSetInt::hasKey(eqCrossSet.clone(), adjEq.clone())?)
                } else {
                    true
                } {
                    continue;
                }
                if hasSameIntSortedExcept(
                    adjVars.clone(),
                    metamodelica::arrayGet(mIn.clone(), adjEq.clone())?,
                    adjVar.clone(),
                )? {
                    newPath = metamodelica::cons(adjEq.clone(), list![crossEq.clone()]);
                    paths = List::unionElt(newPath, paths);
                    if if (findExactlyOneLoop) {
                        !((pathsOut).is_empty())
                    } else {
                        false
                    } {
                        return Err("fail");
                    }
                }
            }
        }
        pathsOut = listAppend(paths, pathsOut);
    }
    Ok(pathsOut)
}

fn longestRow(mut vars: metamodelica::List<i32>, mut mT: metamodelica::Array<metamodelica::List<i32>>) -> Result<i32> {
    let mut var: i32 = 0;
    let mut rows: metamodelica::List<(i32, metamodelica::List<i32>)> = ({
        let mut __acc: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = (v, metamodelica::arrayGet(mT.clone(), v)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut left: metamodelica::List<(i32, metamodelica::List<i32>)>;
    let mut v: i32;
    let mut row: metamodelica::List<i32>;
    while !((rows).is_empty()) {
        (var, _) = (rows).head().cloned()?;
        if ((rows).rest()?).is_empty() {
            return Ok(var);
        }
        left = metamodelica::nil();
        for mut r in &*rows {
            (v, row) = r.clone();
            if !((row).is_empty()) {
                left = metamodelica::cons((v, (row).rest()?), left);
            }
        }
        rows = left.reverse();
    }
    Ok(var)
}

fn eqsSharingVia(
    mut vars: &metamodelica::List<i32>,
    mut var: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut eqs: metamodelica::List<i32> = metamodelica::nil();
    for mut v in &**vars {
        if v.clone() != var {
            for mut eq in &*metamodelica::arrayGet(mT.clone(), v.clone())? {
                if sortedListContains(&(metamodelica::arrayGet(m.clone(), eq.clone())?), var) {
                    eqs = metamodelica::cons(eq.clone(), eqs);
                }
            }
        }
    }
    eqs = List::sortedUnique(
        List::sort(
            eqs,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        &fnptr!(intEq, i32, i32),
    )?;
    Ok(eqs)
}

fn sortedListContains(mut lst: &metamodelica::List<i32>, mut x: i32) -> bool {
    let mut found: bool = false;
    for mut i in &**lst {
        if i.clone() >= x {
            found = i.clone() == x;
            return found;
        }
    }
    found
}

fn connectsLoops(
    mut path: &metamodelica::List<i32>,
    mut allLoops: metamodelica::List<metamodelica::List<i32>>,
) -> Result<bool> {
    let mut connected: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut startNode: i32;
    let mut endNode: i32;
    let mut loops1: metamodelica::List<metamodelica::List<i32>>;
    let mut loops2: metamodelica::List<metamodelica::List<i32>>;
    startNode = (path).head().cloned()?;
    endNode = List::last(path)?;
    loops1 = List::filter1OnTrue(
        allLoops.clone(),
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
        startNode,
    )?;
    loops2 = List::filter1OnTrue(
        allLoops.clone(),
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
        startNode,
    )?;
    b1 = !((loops1).is_empty()) || !((loops2).is_empty());
    loops1 = List::filter1OnTrue(
        allLoops.clone(),
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
        endNode,
    )?;
    loops2 = List::filter1OnTrue(
        allLoops,
        (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
        endNode,
    )?;
    b2 = !((loops1).is_empty()) || !((loops2).is_empty());
    connected = b1 && b2;
    Ok(connected)
}

fn connectPathsToOneLoop(
    mut allPathsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut loopIn: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut loopOut: metamodelica::List<i32>;
    loopOut = 'mc: {
        let __mc_input = (&**allPathsIn, &**loopIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: startNode, tail: path }) => {
                    let mut endNode: i32;
                    endNode = List::last(metamodelica::AsArg::as_arg(&path))?;
                    let true = (intEq(startNode.clone(), endNode)) else { return Err("pattern mismatch") };
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: startNode, tail: _ }) => {
                    let mut path: metamodelica::List<i32>;
                    let mut nextPath: metamodelica::List<i32>;
                    let mut rest: metamodelica::List<metamodelica::List<i32>>;
                    let mut nextPaths1: metamodelica::List<metamodelica::List<i32>>;
                    let mut nextPaths2: metamodelica::List<metamodelica::List<i32>>;
                    nextPaths1 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = listAppend(nextPaths1.clone(), nextPaths2.clone());
                    nextPath = (nextPaths2).head().cloned()?;
                    (rest, _) = List::deleteMemberOnTrue(nextPath.clone(), allPathsIn.clone(), &({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2) }))?;
                    (nextPath, _) = List::deleteMemberOnTrue(startNode.clone(), nextPath.clone(), &fnptr!(intEq, i32, i32))?;
                    path = listAppend(nextPath.clone(), loopIn.clone());
                    path = connectPathsToOneLoop(&rest, &path);
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: path, tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut startNode: i32;
                    let mut nextPath: metamodelica::List<i32>;
                    let mut restPath: metamodelica::List<i32>;
                    let mut nextPaths1: metamodelica::List<metamodelica::List<i32>>;
                    let mut nextPaths2: metamodelica::List<metamodelica::List<i32>>;
                    let mut path = (*path).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(path.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    startNode = metamodelica::Own::own(__pa0);
                    restPath = metamodelica::Own::own(__pa1);
                    nextPaths1 = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), startNode)?;
                    nextPaths2 = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), startNode)?;
                    nextPaths2 = listAppend(nextPaths1.clone(), nextPaths2.clone());
                    nextPath = (nextPaths2).head().cloned()?;
                    (rest, _) = List::deleteMemberOnTrue(nextPath.clone(), rest.clone(), &({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2) }))?;
                    path = listAppend(nextPath.clone(), restPath.clone());
                    path = connectPathsToOneLoop(metamodelica::AsArg::as_arg(&rest), metamodelica::AsArg::as_arg(&path));
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    loopOut
}

fn resolveLoops_resolveAndReplace<'__b>(
    mut loopsIn: metamodelica::List<metamodelica::List<i32>>,
    mut eqCrossLstIn: &'__b metamodelica::List<i32>,
    mut varCrossLstIn: &'__b metamodelica::List<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
    mut daeEqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut daeVarsIn: &'__b BackendDAE::Variables,
    mut replEqsIn: metamodelica::List<i32>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<i32>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((loopsIn, eqCrossLstIn.clone(), varCrossLstIn.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok((daeEqsIn, replEqsIn))
            },
            (Deref @ metamodelica::ListNode::Cons { head: loop1, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: crossEqs }, Deref @ metamodelica::ListNode::Nil) => {
                let mut pos: i32;
                let mut eqs: metamodelica::List<i32>;
                let mut vars: metamodelica::List<i32>;
                let mut replEqs: metamodelica::List<i32>;
                let mut loopVars: metamodelica::List<i32>;
                let mut adjVars: metamodelica::List<i32>;
                let mut m_row: metamodelica::List<i32>;
                let mut eqVars: metamodelica::List<metamodelica::List<i32>>;
                let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                let mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut loop1 = (*loop1).clone();
                let mut rest = (*rest).clone();
                let mut crossEqs = (*crossEqs).clone();
                loop1 = List::unique(metamodelica::AsArg::as_arg(&loop1));
                (resolvedEq, m_row) = resolveClosedLoop(metamodelica::AsArg::as_arg(&loop1), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn)?;
                (crossEqs, eqs, _) = List::intersection1OnTrue(loop1.clone(), eqCrossLstIn.clone(), &fnptr!(intEq, i32, i32))?;
                replEqs = List::intersectionOnTrue(&replEqsIn, metamodelica::AsArg::as_arg(&loop1), &fnptr!(intEq, i32, i32))?;
                if !((eqs).is_empty()) {
                    pos = (eqs).head().cloned()?;
                } else if !((replEqs).is_empty()) {
                    pos = (replEqs).head().cloned()?;
                } else if !((crossEqs).is_empty()) {
                    pos = (crossEqs).head().cloned()?;
                } else {
                    pos = -1;
                }
                (eqs, _) = List::deleteMemberOnTrue(pos, loop1.clone(), &fnptr!(intEq, i32, i32))?;
                eqVars = List::map1(loop1.clone(), &Array::getIndexFirst, mIn.clone())?;
                vars = List::flatten(eqVars)?;
                loopVars = doubleEntriesInLst(&vars);
                (_, adjVars, _) = List::intersection1OnTrue(vars, loopVars.clone(), &fnptr!(intEq, i32, i32))?;
                List::map2_0(&loopVars, &Array::updateIndexFirst, metamodelica::nil(), mTIn.clone())?;
                List::map2_0(&adjVars, &arrayGetDeleteInLst, loop1.clone(), mTIn.clone())?;
                List::map2_0(&adjVars, &arrayGetAppendLst, list![pos], mTIn.clone())?;
                List::map2_0(metamodelica::AsArg::as_arg(&loop1), &Array::updateIndexFirst, metamodelica::nil(), mIn.clone())?;
                metamodelica::arrayUpdate(mIn.clone(), pos, adjVars)?;
                rest = List::map2(rest.clone(), &replaceContractedNodes, pos, eqs)?;
                rest = List::unique(metamodelica::AsArg::as_arg(&rest));
                replEqs = metamodelica::cons(pos, replEqsIn);
                metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
                pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
                daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
                { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: loop1, tail: rest }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: crossVars }) => {
                let mut pos: i32;
                let mut eqs: metamodelica::List<i32>;
                let mut vars: metamodelica::List<i32>;
                let mut replEqs: metamodelica::List<i32>;
                let mut loopVars: metamodelica::List<i32>;
                let mut adjVars: metamodelica::List<i32>;
                let mut m_row: metamodelica::List<i32>;
                let mut eqVars: metamodelica::List<metamodelica::List<i32>>;
                let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                let mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut loop1 = (*loop1).clone();
                let mut rest = (*rest).clone();
                let mut crossVars = (*crossVars).clone();
                loop1 = List::unique(metamodelica::AsArg::as_arg(&loop1));
                (resolvedEq, m_row) = resolveClosedLoop(metamodelica::AsArg::as_arg(&loop1), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn)?;
                (replEqs, _, eqs) = List::intersection1OnTrue(replEqsIn.clone(), loop1.clone(), &fnptr!(intEq, i32, i32))?;
                eqs = priorizeEqsWithVarCrosses(&eqs, mIn.clone(), varCrossLstIn)?;
                pos = if (!((replEqs).is_empty())) {(replEqs).head().cloned()?} else {-1};
                pos = if (!((eqs).is_empty())) {(eqs).head().cloned()?} else {pos};
                (eqs, _) = List::deleteMemberOnTrue(pos, loop1.clone(), &fnptr!(intEq, i32, i32))?;
                eqVars = List::map1(loop1.clone(), &Array::getIndexFirst, mIn.clone())?;
                vars = List::flatten(eqVars)?;
                loopVars = doubleEntriesInLst(&vars);
                (crossVars, loopVars, _) = List::intersection1OnTrue(loopVars, varCrossLstIn.clone(), &fnptr!(intEq, i32, i32))?;
                (_, adjVars, _) = List::intersection1OnTrue(vars, loopVars.clone(), &fnptr!(intEq, i32, i32))?;
                adjVars = listAppend(crossVars.clone(), adjVars);
                adjVars = List::unique(&adjVars);
                List::map2_0(&loopVars, &Array::updateIndexFirst, metamodelica::nil(), mTIn.clone())?;
                List::map2_0(&adjVars, &arrayGetDeleteInLst, loop1.clone(), mTIn.clone())?;
                List::map2_0(&adjVars, &arrayGetAppendLst, list![pos], mTIn.clone())?;
                List::map2_0(metamodelica::AsArg::as_arg(&loop1), &Array::updateIndexFirst, metamodelica::nil(), mIn.clone())?;
                metamodelica::arrayUpdate(mIn.clone(), pos, adjVars)?;
                rest = List::map2(rest.clone(), &replaceContractedNodes, pos, eqs)?;
                rest = List::unique(metamodelica::AsArg::as_arg(&rest));
                replEqs = metamodelica::cons(pos, replEqsIn);
                metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
                pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
                daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
                { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: loop1, tail: rest }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                let mut pos: i32;
                let mut vars: metamodelica::List<i32>;
                let mut crossEqs: metamodelica::List<i32>;
                let mut replEqs: metamodelica::List<i32>;
                let mut m_row: metamodelica::List<i32>;
                let mut eqVars: metamodelica::List<metamodelica::List<i32>>;
                let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                let mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut loop1 = (*loop1).clone();
                loop1 = List::unique(metamodelica::AsArg::as_arg(&loop1));
                (resolvedEq, m_row) = resolveClosedLoop(metamodelica::AsArg::as_arg(&loop1), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn)?;
                (_, crossEqs, _) = List::intersection1OnTrue(loop1.clone(), replEqsIn.clone(), &fnptr!(intEq, i32, i32))?;
                let __pa0 = ::match_deref::match_deref! { match &(crossEqs) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                pos = metamodelica::Own::own(__pa0);
                eqVars = List::map1(loop1.clone(), &Array::getIndexFirst, mIn.clone())?;
                vars = List::flatten(eqVars)?;
                List::map2_0(metamodelica::AsArg::as_arg(&loop1), &Array::updateIndexFirst, metamodelica::nil(), mIn.clone())?;
                List::map2_0(&vars, &Array::updateIndexFirst, metamodelica::nil(), mTIn.clone())?;
                replEqs = metamodelica::cons(pos, replEqsIn);
                metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
                pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
                daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
                { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: loop1, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                let mut pos: i32;
                let mut eq1: i32;
                let mut eq2: i32;
                let mut replEqs: metamodelica::List<i32>;
                let mut m_row: metamodelica::List<i32>;
                let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                let mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut loop1 = (*loop1).clone();
                loop1 = List::unique(metamodelica::AsArg::as_arg(&loop1));
                let true = (((loop1).len() as i32) == 2) else { return Err("pattern mismatch") };
                (resolvedEq, m_row) = resolveClosedLoop(metamodelica::AsArg::as_arg(&loop1), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn)?;
                if eqIsConst(&resolvedEq) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loop1.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq1 = metamodelica::Own::own(__pa0);
                    eq2 = metamodelica::Own::own(__pa1);
                    if (((BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq1)?)?, daeVarsIn.clone())?)).len() as i32) >= (((BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq2)?)?, daeVarsIn.clone())?)).len() as i32) {
                        pos = eq1;
                    } else {
                        pos = eq2;
                    }
                    replEqs = metamodelica::cons(pos, replEqsIn);
                    metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
                    pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
                    daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
                } else {
                    replEqs = replEqsIn;
                    daeEqs = daeEqsIn;
                }
                { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn eqIsConst(mut eq: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match eq {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { .. }, scalar: Deref @ DAE::Exp::CREF { .. }, .. } => true,
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::RCONST { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn arrayIsZeroAt(mut pos: i32, mut arr: metamodelica::Array<i32>) -> Result<bool> {
    let __ab_arr = arr.borrow();
    let mut isZero: bool;
    isZero = intEq(0, (*metamodelica::index_checked(&__ab_arr, pos)?).clone());
    Ok(isZero)
}

fn markDeadEndsInBipartiteGraph(
    mut varIdx: i32,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut deadEndEqs: metamodelica::Array<i32>,
    mut deadEndVars: metamodelica::Array<i32>,
) -> Result<()> {
    let mut eqIdx: i32;
    let mut var: i32 = varIdx;
    let mut adjEqs: metamodelica::List<i32>;
    let mut adjVars: metamodelica::List<i32>;
    let mut walk: bool = true;
    while walk {
        walk = false;
        adjEqs = metamodelica::arrayGet(mTIn.clone(), var)?;
        adjEqs = List::filter1OnTrue(
            adjEqs,
            (std::sync::Arc::new(arrayIsZeroAt)
                as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>),
            deadEndEqs.clone(),
        )?;
        if ((adjEqs).len() as i32) == 1 {
            eqIdx = (adjEqs).head().cloned()?;
            metamodelica::arrayUpdate(deadEndVars.clone(), var, 1)?;
            adjVars = metamodelica::arrayGet(mIn.clone(), eqIdx)?;
            adjVars = List::filter1OnTrue(
                adjVars,
                (std::sync::Arc::new(arrayIsZeroAt)
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>),
                deadEndVars.clone(),
            )?;
            if ((adjVars).len() as i32) == 1 {
                metamodelica::arrayUpdate(deadEndEqs.clone(), eqIdx, 1)?;
                var = (adjVars).head().cloned()?;
                walk = true;
            }
        }
    }
    Ok(())
}

fn arrayGetDeleteInLst(
    mut idx: i32,
    mut delEntries: metamodelica::List<i32>,
    mut arrIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut entry: metamodelica::List<i32>;
    entry = metamodelica::arrayGet(arrIn.clone(), idx)?;
    (_, entry, _) = List::intersection1OnTrue(entry, delEntries, &fnptr!(intEq, i32, i32))?;
    metamodelica::arrayUpdate(arrIn.clone(), idx, entry)?;
    Ok(())
}

fn arrayGetAppendLst(
    mut idx: i32,
    mut appLst: metamodelica::List<i32>,
    mut arrIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut entry: metamodelica::List<i32>;
    entry = metamodelica::arrayGet(arrIn.clone(), idx)?;
    metamodelica::arrayUpdate(arrIn.clone(), idx, listAppend(entry, appLst))?;
    Ok(())
}

fn getReverseDoubles(
    mut elem: metamodelica::List<i32>,
    mut elemLst: &metamodelica::List<metamodelica::List<i32>>,
    mut foldLstIn: metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut foldLstOut: metamodelica::List<metamodelica::List<i32>>;
    foldLstOut = 'mc: {
        let __mc_input = &*foldLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elemR: metamodelica::List<i32>;
                    let mut foldLst: metamodelica::List<metamodelica::List<i32>>;
                    elemR = elem.clone().reverse();
                    elemR = List::getMember(elemR.clone(), elemLst)?;
                    (foldLst, _) = List::deleteMemberOnTrue(elem.clone(), foldLstIn.clone(), &({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2) }))?;
                    Ok(metamodelica::cons(elemR.clone(), foldLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(foldLstIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    foldLstOut
}

fn getDoubles(
    mut elemLstIn: metamodelica::List<metamodelica::List<i32>>,
    mut lstIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut lstOut: metamodelica::List<metamodelica::List<i32>> = lstIn;
    let mut elem: metamodelica::List<i32>;
    let mut elemLst: metamodelica::List<metamodelica::List<i32>> = elemLstIn;
    while !((elemLst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elemLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        elem = metamodelica::Own::own(__pa0);
        elemLst = metamodelica::Own::own(__pa1);
        if listMember(elem.clone(), elemLst.clone()) {
            lstOut = metamodelica::cons(elem, lstOut);
        }
    }
    Ok(lstOut)
}

fn getTriples(
    mut crossNodes: &metamodelica::List<i32>,
    mut minAdj: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut tripleLoops: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut allPaths: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut path1: metamodelica::List<i32>;
    let mut path2: metamodelica::List<i32>;
    let mut path3: metamodelica::List<i32>;
    for mut c0 in &**crossNodes {
        path1 = metamodelica::arrayGet(minAdj.clone(), c0.clone())?;
        for mut c1 in &*path1 {
            if intGt(c1.clone(), c0.clone()) {
                path2 = metamodelica::arrayGet(minAdj.clone(), c1.clone())?;
                for mut c2 in &*path2 {
                    if intGt(c2.clone(), c1.clone()) {
                        path3 = metamodelica::arrayGet(minAdj.clone(), c2.clone())?;
                        if listContains(&path3, c0.clone()) {
                            tripleLoops = metamodelica::cons(list![c0.clone(), c1.clone(), c2.clone()], tripleLoops);
                            allPaths = metamodelica::cons(list![c1.clone(), c2.clone()], allPaths);
                            allPaths = metamodelica::cons(list![c0.clone(), c2.clone()], allPaths);
                            allPaths = metamodelica::cons(list![c0.clone(), c1.clone()], allPaths);
                        }
                    }
                }
            }
        }
    }
    Ok((tripleLoops, allPaths))
}

fn getEqNodesForVarLoop(
    mut varIdcs: metamodelica::List<i32>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut eqIdcs: metamodelica::List<i32>;
    let mut varEqLst: metamodelica::List<metamodelica::List<i32>>;
    let mut eqLst: metamodelica::List<i32>;
    varEqLst = List::map1(varIdcs, &Array::getIndexFirst, mTIn.clone())?;
    eqLst = List::flatten(varEqLst)?;
    eqIdcs = doubleEntriesInLst(&eqLst);
    Ok(eqIdcs)
}

fn resolveClosedLoop(
    mut loopIn: &metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
    mut daeEqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut daeVarsIn: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<i32>)> {
    let mut eqOut: metamodelica::Ref<BackendDAE::Equation>;
    let mut m_row: metamodelica::List<i32>;
    let mut startEqIdx: i32;
    let mut startEqDaeIdx: i32;
    let mut loop1: metamodelica::List<i32>;
    let mut restLoop: metamodelica::List<i32>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*loopIn)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    startEqIdx = metamodelica::Own::own(__pa0);
    restLoop = metamodelica::Own::own(__pa1);
    startEqDaeIdx = metamodelica::arrayGet(eqMap.clone(), startEqIdx)?;
    loop1 = sortLoop(restLoop, m.clone(), mT.clone(), list![startEqIdx])?;
    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? && ((loop1).len() as i32) > 1 {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("solve the loop: "));
            __mm_s.push_str(&*List::toString(
                loop1.clone(),
                &fnptr!(intString, i32),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    eq = BackendEquation::get(daeEqsIn.clone(), startEqDaeIdx)?;
    (eqOut, m_row) = resolveClosedLoop2(
        eq,
        loop1,
        m.clone(),
        metamodelica::arrayGet(m.clone(), startEqIdx)?,
        eqMap.clone(),
        varMap.clone(),
        daeEqsIn,
        daeVarsIn,
    )?;
    Ok((eqOut, m_row))
}

fn resolveClosedLoop2<'__b>(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut loopIn: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut m_row: metamodelica::List<i32>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
    mut daeEqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut daeVarsIn: &'__b BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(loopIn) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
                return Ok((eq, m_row))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: eqIdx2, tail: restLoop } } => {
                let mut algSign: bool;
                let mut adjVars: metamodelica::List<i32>;
                let mut adjVars1: metamodelica::List<i32>;
                let mut adjVars2: metamodelica::List<i32>;
                let mut posVars: metamodelica::List<i32>;
                let mut negVars: metamodelica::List<i32>;
                let mut nonUnitVars: metamodelica::List<i32>;
                let mut adjCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut eq2: metamodelica::Ref<BackendDAE::Equation>;
                let mut eq3: metamodelica::Ref<BackendDAE::Equation>;
                let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                let mut replacements: BackendVarTransform::VariableReplacements;
                eq2 = BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eqIdx2.clone())?)?;
                adjVars1 = m_row;
                adjVars2 = metamodelica::arrayGet(m.clone(), eqIdx2.clone())?;
                (adjVars, adjVars1, adjVars2) = List::intersection1OnTrue(adjVars1, adjVars2, &fnptr!(intEq, i32, i32))?;
                (adjVars, nonUnitVars) = List::splitOnTrue(&adjVars, &({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varIsUnitCoeff(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3, &__pe_b4) }))?;
                (posVars, negVars) = List::splitOnTrue(&adjVars, &({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varSign(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3, &__pe_b4) }))?;
                algSign = ((posVars).len() as i32) > ((negVars).len() as i32);
                adjCrefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
            for mut idx in (if (algSign) {posVars.clone()} else {negVars.clone()}).into_iter().cloned() {
                let __x = crefFromIndex(idx.clone(), varMap.clone(), daeVarsIn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                m_row = List::flatten(list![adjVars1, adjVars2, nonUnitVars, if (algSign) {negVars} else {posVars}])?;
                replacements = BackendVarTransform::emptyReplacementsSized(((adjCrefs).len() as i32));
                replacements = BackendVarTransform::addReplacements(replacements, &(adjCrefs.clone()), &(({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut c in (adjCrefs).into_iter().cloned() {
                let __x = Expression::createZeroExpression(ComponentReference::crefTypeFull(&(c.clone()))?)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), None)?;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eq.clone(), eq2.clone()], &replacements, None)?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } }, _) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                resolvedEq = metamodelica::Own::own(__pa0);
                eq3 = metamodelica::Own::own(__pa1);
                resolvedEq = sumUp2Equations(algSign, &resolvedEq, &eq3)?;
                if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("From eqs \n")); __mm_s.push_str(&*BackendDump::equationString(&eq)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BackendDump::equationString(&eq2)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("resolved the eq \n")); __mm_s.push_str(&*BackendDump::equationString(&resolvedEq)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                }
                { (eq, loopIn, m, m_row, eqMap, varMap, daeEqsIn, daeVarsIn) = (resolvedEq, metamodelica::cons(eqIdx2.clone(), restLoop.clone()), m.clone(), m_row, eqMap.clone(), varMap.clone(), daeEqsIn, daeVarsIn); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn crefFromIndex(
    mut varIdx: i32,
    mut varMap: metamodelica::Array<i32>,
    mut daeVarsIn: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut daeVarIdx: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    daeVarIdx = metamodelica::arrayGet(varMap.clone(), varIdx)?;
    var = BackendVariable::getVarAt(daeVarsIn, daeVarIdx)?;
    cref = BackendVariable::varCref(&var);
    Ok(cref)
}

fn varSign(
    mut index: i32,
    mut varMap: metamodelica::Array<i32>,
    mut daeVarsIn: &BackendDAE::Variables,
    mut eq1: &metamodelica::Ref<BackendDAE::Equation>,
    mut eq2: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<bool> {
    let mut algSign: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = crefFromIndex(index, varMap.clone(), daeVarsIn)?;
    algSign = CRefIsPosOnRHS(&cref, eq1)? != CRefIsPosOnRHS(&cref, eq2)?;
    Ok(algSign)
}

fn varIsUnitCoeff(
    mut index: i32,
    mut varMap: metamodelica::Array<i32>,
    mut daeVarsIn: &BackendDAE::Variables,
    mut eq1: &metamodelica::Ref<BackendDAE::Equation>,
    mut eq2: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<bool> {
    let mut isUnit: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef> = crefFromIndex(index, varMap.clone(), daeVarsIn)?;
    isUnit = crefHasUnitCoeff(&cref, eq1)? && crefHasUnitCoeff(&cref, eq2)?;
    Ok(isUnit)
}

fn crefHasUnitCoeff(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<bool> {
    let mut isUnit: bool;
    isUnit = (match &**eq {
        BackendDAE::Equation::EQUATION {
            exp: e1, scalar: e2, ..
        } => crefUnitCoeffInExp(e1, cref)? && crefUnitCoeffInExp(e2, cref)?,
        _ => true,
    });
    Ok(isUnit)
}

fn crefUnitCoeffInExp<'__b>(
    mut exp: &'__b metamodelica::Ref<DAE::Exp>,
    mut cref: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match exp {
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                return Ok(crefUnitCoeffInExp(e1, cref)? && crefUnitCoeffInExp(e2, cref)?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                return Ok(crefUnitCoeffInExp(e1, cref)? && crefUnitCoeffInExp(e2, cref)?)
            },
            Deref @ DAE::Exp::UNARY { exp: e1, .. } => {
                { (exp, cref) = (e1, cref); continue '__tco; }
            },
            Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: c, .. }, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                return Ok(!(ComponentReferenceBasics::crefEqualNoStringCompare(cref, metamodelica::AsArg::as_arg(&c))?) || Expression::isOne(e2) || Expression::isConstMinusOne(e2))
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: c, .. } } => {
                return Ok(!(ComponentReferenceBasics::crefEqualNoStringCompare(cref, metamodelica::AsArg::as_arg(&c))?) || Expression::isOne(e1) || Expression::isConstMinusOne(e1))
            },
            _ => {
                return Ok(true)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn sortLoop(
    mut loopIn: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut sortLoopIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((loopIn.clone(), sortLoopIn.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(sortLoopIn.reverse())
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: start, tail: _ }) => {
                let mut next: i32;
                let mut rest: metamodelica::List<i32>;
                let mut vars: metamodelica::List<i32>;
                let mut eqs: metamodelica::List<i32>;
                let mut varEqs: metamodelica::List<metamodelica::List<i32>>;
                vars = metamodelica::arrayGet(m.clone(), start.clone())?;
                varEqs = List::map1(vars, &Array::getIndexFirst, mT.clone())?;
                eqs = List::flatten(varEqs)?;
                eqs = List::unique(&eqs);
                eqs = List::intersectionOnTrue(&eqs, &loopIn, &fnptr!(intEq, i32, i32))?;
                if (eqs).is_empty() {
                    next = (loopIn).head().cloned()?;
                } else {
                    next = (eqs).head().cloned()?;
                }
                (rest, _) = List::deleteMemberOnTrue(next, loopIn, &fnptr!(intEq, i32, i32))?;
                { (loopIn, m, mT, sortLoopIn) = (rest, m.clone(), mT.clone(), metamodelica::cons(next, sortLoopIn)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn closePathDirectly(
    mut pathIn: metamodelica::List<i32>,
    mut pathLstIn: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut pathOut: metamodelica::List<i32>;
    pathOut = 'mc: {
        let __mc_input = &**pathLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32;
                    let mut endNode: i32;
                    startNode = (pathIn).head().cloned()?;
                    endNode = List::last(&pathIn)?;
                    let true = (intEq(startNode, endNode)) else { return Err("pattern mismatch") };
                    Ok(pathIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut closed: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut path: metamodelica::List<i32>;
                    let __pa0 = ::match_deref::match_deref! { match &(pathIn.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    startNode = metamodelica::Own::own(__pa0);
                    endNode = List::last(&pathIn)?;
                    path = findPathByEnds(pathLstIn, startNode, endNode)?;
                    closed = !((path).is_empty());
                    path = if (closed) {path.clone()} else {metamodelica::nil()};
                    path = listAppend(pathIn.clone(), path.clone());
                    path = List::unique(&path);
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function ResolveLoops.closePathDirectly failed"), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(pathOut)
}

fn findPathByEnds(
    mut pathLstIn: &metamodelica::List<metamodelica::List<i32>>,
    mut startNodeIn: i32,
    mut endNodeIn: i32,
) -> Result<metamodelica::List<i32>> {
    let mut pathOut: metamodelica::List<i32>;
    pathOut = 'mc: {
        let __mc_input = &**pathLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: path, tail: pathLst } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut path = (*path).clone();
                    startNode = (path).head().cloned()?;
                    b1 = intEq(startNode, endNodeIn);
                    endNode = List::last(metamodelica::AsArg::as_arg(&path))?;
                    b2 = intEq(endNode, startNodeIn);
                    path = if (!(b1 && b2)) {findPathByEnds(metamodelica::AsArg::as_arg(&pathLst), startNodeIn, endNodeIn)?} else {path.clone()};
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
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
                _ => {
                    Error::addInternalError(literal!("function ResolveLoops.findPathByEnds failed"), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(pathOut)
}

fn countDoubleEntriesInLst(
    mut lstIn: &metamodelica::List<i32>,
    mut checkLst: metamodelica::List<i32>,
    mut dupLst: metamodelica::List<i32>,
) -> (i32, metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut num: i32 = 0;
    let mut checkLst: metamodelica::List<i32> = checkLst;
    let mut dupLst: metamodelica::List<i32> = dupLst;
    for mut elem in &**lstIn {
        if listMember(elem.clone(), checkLst.clone()) {
            num = num + 1;
            if !(listMember(elem.clone(), dupLst.clone())) {
                dupLst = metamodelica::cons(elem.clone(), dupLst);
            }
        } else {
            checkLst = metamodelica::cons(elem.clone(), checkLst);
        }
    }
    (num, checkLst, dupLst)
}

fn countDoubleEntriesInLstLst(
    mut lstIn: &metamodelica::List<metamodelica::List<i32>>,
    mut checkLst: metamodelica::List<i32>,
    mut dupLst: metamodelica::List<i32>,
) -> (i32, metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut num: i32 = 0;
    let mut checkLst: metamodelica::List<i32> = checkLst;
    let mut dupLst: metamodelica::List<i32> = dupLst;
    for mut lst in &**lstIn {
        for mut elem in &*lst.clone() {
            if listMember(elem.clone(), checkLst.clone()) {
                num = num + 1;
                if !(listMember(elem.clone(), dupLst.clone())) {
                    dupLst = metamodelica::cons(elem.clone(), dupLst);
                }
            } else {
                checkLst = metamodelica::cons(elem.clone(), checkLst);
            }
        }
    }
    (num, checkLst, dupLst)
}

fn doubleEntriesInLst(mut lstIn: &metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut doubleLst: metamodelica::List<i32> = metamodelica::nil();
    let mut checkLst: metamodelica::List<i32> = metamodelica::nil();
    for mut i in &**lstIn {
        if listMember(i.clone(), checkLst.clone()) {
            doubleLst = metamodelica::cons(i.clone(), doubleLst);
        } else {
            checkLst = metamodelica::cons(i.clone(), checkLst);
        }
    }
    doubleLst
}

fn getPathTillNextCrossEq(
    mut checkEqCrossNodes: metamodelica::List<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut allEqCrossNodes: metamodelica::List<i32>,
    mut unfinPathsIn: metamodelica::List<metamodelica::List<i32>>,
    mut eqPathsIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut eqPathsOut: metamodelica::List<metamodelica::List<i32>> = eqPathsIn;
    let mut crossEq: i32;
    let mut lastEq: i32;
    let mut prevEq: i32;
    let mut adjVars: metamodelica::List<i32>;
    let mut nextEqs: metamodelica::List<i32>;
    let mut endEqs: metamodelica::List<i32>;
    let mut unfinEqs: metamodelica::List<i32>;
    let mut crossNodes: metamodelica::List<i32> = checkEqCrossNodes;
    let mut pathStart: metamodelica::List<i32>;
    let mut paths: metamodelica::List<metamodelica::List<i32>>;
    let mut adjEqs: metamodelica::List<metamodelica::List<i32>>;
    let mut unfinPaths: metamodelica::List<metamodelica::List<i32>> = unfinPathsIn;
    loop {
        if !((unfinPaths).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unfinPaths) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            pathStart = metamodelica::Own::own(__pa0);
            unfinPaths = metamodelica::Own::own(__pa1);
            lastEq = (pathStart).head().cloned()?;
            prevEq = List::second(&pathStart)?;
            adjVars = metamodelica::arrayGet(mIn.clone(), lastEq)?;
            adjEqs = List::map1(adjVars, &Array::getIndexFirst, mTIn.clone())?;
            adjEqs = ({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut eq in (adjEqs).into_iter().cloned() {
                    let __x = (List::deleteMemberOnTrue(lastEq, eq.clone(), &fnptr!(intEq, i32, i32))?).0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            adjEqs = List::filterOnFalse(adjEqs, &fnptr!(listEmpty, _))?;
            nextEqs = List::map(adjEqs, &listHead)?;
            (nextEqs, _) = List::deleteMemberOnTrue(prevEq, nextEqs, &fnptr!(intEq, i32, i32))?;
            (endEqs, unfinEqs, _) =
                List::intersection1OnTrue(nextEqs, allEqCrossNodes.clone(), &fnptr!(intEq, i32, i32))?;
            paths = List::map1(endEqs, &fnptr!(cons1, i32, metamodelica::List<i32>), pathStart.clone())?;
            eqPathsOut = listAppend(paths, eqPathsOut);
            paths = List::map1(unfinEqs, &fnptr!(cons1, i32, metamodelica::List<i32>), pathStart)?;
            unfinPaths = listAppend(paths, unfinPaths);
        } else if !((crossNodes).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(crossNodes) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            crossEq = metamodelica::Own::own(__pa2);
            crossNodes = metamodelica::Own::own(__pa3);
            adjVars = metamodelica::arrayGet(mIn.clone(), crossEq)?;
            adjEqs = List::map1(adjVars, &Array::getIndexFirst, mTIn.clone())?;
            adjEqs = ({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut eq in (adjEqs).into_iter().cloned() {
                    let __x = (List::deleteMemberOnTrue(crossEq, eq.clone(), &fnptr!(intEq, i32, i32))?).0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            adjEqs = List::filterOnFalse(adjEqs, &fnptr!(listEmpty, _))?;
            nextEqs = List::flatten(adjEqs)?;
            (endEqs, unfinEqs, _) =
                List::intersection1OnTrue(nextEqs, allEqCrossNodes.clone(), &fnptr!(intEq, i32, i32))?;
            paths = List::map1(endEqs, &fnptr!(cons1, i32, metamodelica::List<i32>), list![crossEq])?;
            eqPathsOut = listAppend(paths, eqPathsOut);
            paths = List::map1(unfinEqs, &fnptr!(cons1, i32, metamodelica::List<i32>), list![crossEq])?;
            unfinPaths = listAppend(paths, unfinPaths);
        } else {
            return Ok(eqPathsOut);
        }
    }
    Ok(eqPathsOut)
}

fn cons1(mut elem: i32, mut lst: metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut outLst: metamodelica::List<i32>;
    outLst = metamodelica::cons(elem, lst);
    outLst
}

fn replaceContractedNodes(
    mut pathIn: metamodelica::List<i32>,
    mut nodeIn: i32,
    mut replNodes: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut pathOut: metamodelica::List<i32>;
    pathOut = List::map2(
        pathIn,
        &move |__a0: i32, __a1: i32, __a2: metamodelica::List<i32>| replaceContractedNodes2(__a0, __a1, &__a2),
        nodeIn,
        replNodes,
    )?;
    Ok(pathOut)
}

fn replaceContractedNodes2(mut entryIn: i32, mut nodeIn: i32, mut replNodes: &metamodelica::List<i32>) -> Result<i32> {
    let mut entryOut: i32;
    let mut repl: bool;
    repl = List::isMemberOnTrue(entryIn, replNodes, &fnptr!(intEq, i32, i32))?;
    entryOut = if (repl) { nodeIn } else { entryIn };
    Ok(entryOut)
}

fn priorizeEqsWithVarCrosses(
    mut eqsIn: &metamodelica::List<i32>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut varCrossLst: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut eqsOut: metamodelica::List<i32>;
    let mut priorities: metamodelica::Array<metamodelica::List<i32>>;
    priorities = arrayCreate(3, metamodelica::nil());
    for mut eq in &**eqsIn {
        priorizeEqsWithVarCrosses2(eq.clone(), mIn.clone(), varCrossLst, priorities.clone())?;
    }
    eqsOut = List::flatten(
        priorities
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?;
    Ok(eqsOut)
}

fn priorizeEqsWithVarCrosses2(
    mut eq: i32,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut varCrossLst: &metamodelica::List<i32>,
    mut priorities: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut eqVars: metamodelica::List<i32>;
    let mut crossVars: metamodelica::List<i32>;
    eqVars = metamodelica::arrayGet(mIn.clone(), eq)?;
    crossVars = List::intersectionOnTrue(&eqVars, varCrossLst, &fnptr!(intEq, i32, i32))?;
    if (crossVars).is_empty() {
        arrayGetAppendLst(1, list![eq], priorities.clone())?;
    } else if List::hasOneElement(&crossVars) {
        arrayGetAppendLst(2, list![eq], priorities.clone())?;
    } else {
        arrayGetAppendLst(3, list![eq], priorities.clone())?;
    }
    Ok(())
}

fn evaluateLoop(
    mut loopIn: metamodelica::List<i32>,
    mut tplIn: &(
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::List<i32>,
    ),
) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool;
    let mut r2: bool;
    let mut numInLoop: i32;
    let mut numOutLoop: i32;
    let mut eqCrossLst: metamodelica::List<i32>;
    let mut chk: metamodelica::List<i32> = metamodelica::nil();
    let mut dup: metamodelica::List<i32> = metamodelica::nil();
    let mut eqVars: metamodelica::List<metamodelica::List<i32>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (m, _, eqCrossLst) = tplIn.clone();
        eqVars = List::map1(loopIn, &Array::getIndexFirst, m.clone())?;
        (numInLoop, chk, dup) = countDoubleEntriesInLstLst(&eqVars, chk, dup);
        numOutLoop = ((chk).len() as i32) - ((dup).len() as i32);
        r1 = intGe(numInLoop, numOutLoop - 1) && intLe(numInLoop, 6);
        r2 = intGe(numInLoop, numOutLoop - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {
            r1
        } else {
            false
        };
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {
            r2
        } else {
            r1
        };
    }
    Ok(resolve)
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TripleLoopInfo {
    pub m: metamodelica::Array<metamodelica::List<i32>>,
    pub mapIndices: metamodelica::List<i32>,
    pub map: metamodelica::Array<metamodelica::List<i32>>,
    /// occurrences of each variable in the merged nodes' rows
    pub count: metamodelica::Array<i32>,
    pub entries: i32,
    pub distinct: i32,
    /// of those rows
    pub singles: i32,
}

impl metamodelica::gc::MMTrace for TripleLoopInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.m, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mapIndices, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.map, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.count, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entries, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.distinct, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.singles, __mmv)?;
        Ok(())
    }
}
impl Default for TripleLoopInfo {
    fn default() -> Self {
        Self {
            m: Default::default(),
            mapIndices: Default::default(),
            map: Default::default(),
            count: Default::default(),
            entries: Default::default(),
            distinct: Default::default(),
            singles: Default::default(),
        }
    }
}

pub type TRIPLE_LOOP_INFO = TripleLoopInfo;

fn tripleLoopInfo(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut numVars: i32,
    mut mapIndices: metamodelica::List<i32>,
    mut map: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<TripleLoopInfo> {
    let mut info: TripleLoopInfo;
    let mut count: metamodelica::Array<i32> = arrayCreate(numVars, 0);
    let mut entries: i32 = 0;
    let mut distinct: i32 = 0;
    let mut singles: i32 = 0;
    for mut i in &*mapIndices {
        for mut j in &*metamodelica::arrayGet(map.clone(), i.clone())? {
            (entries, distinct, singles) = countRow(
                &(metamodelica::arrayGet(m.clone(), j.clone())?),
                count.clone(),
                entries,
                distinct,
                singles,
                1,
            )?;
        }
    }
    info = TripleLoopInfo {
        m: m.clone(),
        mapIndices: mapIndices,
        map: map.clone(),
        count: count.clone(),
        entries: entries,
        distinct: distinct,
        singles: singles,
    };
    Ok(info)
}

fn countRow(
    mut row: &metamodelica::List<i32>,
    mut count: metamodelica::Array<i32>,
    mut entries: i32,
    mut distinct: i32,
    mut singles: i32,
    mut delta: i32,
) -> Result<(i32, i32, i32)> {
    let mut entries: i32 = entries;
    let mut distinct: i32 = distinct;
    let mut singles: i32 = singles;
    let mut c: i32;
    for mut v in &**row {
        c = ({
            let __elt = (*metamodelica::index_checked(&count.borrow(), v.clone())?).clone();
            __elt
        }) + delta;
        metamodelica::arrayUpdate(count.clone(), v.clone(), c)?;
        entries = entries + delta;
        if delta > 0 {
            if c == 1 {
                distinct = distinct + 1;
                singles = singles + 1;
            } else if c == 2 {
                singles = singles - 1;
            }
        } else {
            if c == 0 {
                distinct = distinct - 1;
                singles = singles - 1;
            } else if c == 1 {
                singles = singles + 1;
            }
        }
    }
    Ok((entries, distinct, singles))
}

fn evaluateTripleLoop(mut loopIn: &metamodelica::List<i32>, mut info: &TripleLoopInfo) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool;
    let mut r2: bool;
    let mut entries: i32;
    let mut distinct: i32;
    let mut singles: i32;
    let mut numInLoop: i32;
    let mut numOutLoop: i32;
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (entries, distinct, singles) = (info.entries.clone(), info.distinct.clone(), info.singles.clone());
        for mut j in &**loopIn {
            (entries, distinct, singles) = countRow(
                &(metamodelica::arrayGet(info.m.clone(), j.clone())?),
                info.count.clone(),
                entries,
                distinct,
                singles,
                1,
            )?;
        }
        for mut j in &**loopIn {
            countRow(
                &(metamodelica::arrayGet(info.m.clone(), j.clone())?),
                info.count.clone(),
                0,
                0,
                0,
                -1,
            )?;
        }
        numInLoop = entries - distinct;
        numOutLoop = singles;
        r1 = intGe(numInLoop, numOutLoop - 1) && intLe(numInLoop, 10);
        r2 = intGe(numInLoop, numOutLoop - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {
            r1
        } else {
            false
        };
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {
            r2
        } else {
            r1
        };
    }
    Ok(resolve)
}

fn updateTripleLoop(
    mut loopFull: metamodelica::List<i32>,
    mut info: &TripleLoopInfo,
) -> Result<metamodelica::List<i32>> {
    let mut loopFull: metamodelica::List<i32> = loopFull;
    for mut i in &*info.mapIndices.clone() {
        loopFull = listAppend(metamodelica::arrayGet(info.map.clone(), i.clone())?, loopFull);
    }
    Ok(loopFull)
}

fn sumUp2Equations(
    mut sumUp: bool,
    mut eq1: &metamodelica::Ref<BackendDAE::Equation>,
    mut eq2: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut eqOut: metamodelica::Ref<BackendDAE::Equation>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let mut exp3: metamodelica::Ref<DAE::Exp>;
    let mut exp4: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eq1)) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*eq2)) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa2, scalar: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp3 = metamodelica::Own::own(__pa2);
    exp4 = metamodelica::Own::own(__pa3);
    exp1 = sumUp2Expressions(sumUp, exp1, exp3)?;
    exp2 = sumUp2Expressions(sumUp, exp2, exp4)?;
    exp2 = sumUp2Expressions(false, exp2, exp1)?;
    (exp2, _) = ExpressionSimplify::simplify(exp2)?;
    exp1 = Expression::createZeroExpression(Expression::r#typeof(exp2.clone())?)?;
    eqOut = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
        exp: exp1,
        scalar: exp2,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
    });
    eqOut = simplifyZeroAssignment(eqOut);
    Ok(eqOut)
}

fn simplifyZeroAssignment(mut eIn: metamodelica::Ref<BackendDAE::Equation>) -> metamodelica::Ref<BackendDAE::Equation> {
    let mut eOut: metamodelica::Ref<BackendDAE::Equation>;
    eOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { real: __rlit_0 }, scalar: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, exp: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            eIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eOut
}

fn CRefIsPosOnRHS(
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
    mut eqIn: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<bool> {
    let mut isPos: bool;
    isPos = 'mc: {
        let __mc_input = &**eqIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
                    let mut exists1: bool;
                    let mut sign1: bool;
                    let mut sign2: bool;
                    (exists1, sign1) = expIsCref(metamodelica::AsArg::as_arg(&e1), crefIn)?;
                    (_, sign2) = expIsCref(metamodelica::AsArg::as_arg(&e2), crefIn)?;
                    sign1 = if (exists1) {!(sign1)} else {sign2};
                    Ok(sign1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("add a case to CRefIsPosOnRHS")); __mm_s.push_str(&*BackendDump::equationString(eqIn)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(isPos)
}

fn expIsCref(
    mut expIn: &metamodelica::Ref<DAE::Exp>,
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(bool, bool)> {
    let mut isInExp: bool;
    let mut algSign: bool;
    (isInExp, algSign) = (::match_deref::match_deref! { match expIn {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            let mut sameCref: bool;
            sameCref = ComponentReferenceBasics::crefEqualNoStringCompare(crefIn, cref)?;
            (sameCref, true)
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 } => {
            let mut sign: bool;
            let mut sign1: bool;
            let mut sign2: bool;
            let mut exists: bool;
            let mut exists1: bool;
            let mut exists2: bool;
            (exists1, sign1) = expIsCref(exp1, crefIn)?;
            (exists2, sign2) = expIsCref(exp2, crefIn)?;
            sign2 = boolNot(sign2);
            exists = boolOr(exists1, exists2);
            sign = exists1 && sign1;
            sign = if (exists2) {sign2} else {sign};
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 } => {
            let mut sign: bool;
            let mut sign1: bool;
            let mut sign2: bool;
            let mut exists: bool;
            let mut exists1: bool;
            let mut exists2: bool;
            (exists1, sign1) = expIsCref(exp1, crefIn)?;
            (exists2, sign2) = expIsCref(exp2, crefIn)?;
            exists = boolOr(exists1, exists2);
            sign = exists1 && sign1;
            sign = if (exists2) {sign2} else {sign};
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, _) = expIsCref(metamodelica::AsArg::as_arg(&exp1), crefIn)?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { .. } } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, _) = expIsCref(metamodelica::AsArg::as_arg(&exp1), crefIn)?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists, sign)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: exp1 } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, sign) = expIsCref(exp1, crefIn)?;
            sign = boolNot(sign);
            (exists, sign)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (false, false)
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (false, false)
        },
        _ => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("add a case to expIsCref:")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expIn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            (false, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((isInExp, algSign))
}

fn listLengthIs(mut lst: &metamodelica::List<i32>, mut value: i32) -> bool {
    let mut bOut: bool;
    bOut = intEq(((lst).len() as i32), value);
    bOut
}

pub(crate) fn partitionBipartiteGraph(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut partitions: metamodelica::List<metamodelica::List<i32>>;
    let mut numEqs: i32;
    let mut numVars: i32;
    let mut markEqs: metamodelica::Array<i32>;
    let mut markVars: metamodelica::Array<i32>;
    numEqs = metamodelica::arrayLength(m.clone());
    numVars = metamodelica::arrayLength(mT.clone());
    if numEqs == 0 || numVars == 0 {
        partitions = list![metamodelica::nil()];
    } else {
        markEqs = arrayCreate(numEqs, -1);
        markVars = arrayCreate(numVars, -1);
        (_, partitions) = colorNodePartitions(
            m.clone(),
            mT.clone(),
            list![1],
            markEqs.clone(),
            markVars.clone(),
            1,
            metamodelica::nil(),
            1,
        )?;
    }
    Ok(partitions)
}

fn colorNodePartitions(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut checkNextIn: metamodelica::List<i32>,
    mut markEqs: metamodelica::Array<i32>,
    mut markVars: metamodelica::Array<i32>,
    mut currNumberIn: i32,
    mut partitionsIn: metamodelica::List<metamodelica::List<i32>>,
    mut nextIndex: i32,
) -> Result<(i32, metamodelica::List<metamodelica::List<i32>>)> {
    '__tco: loop {
        let mut eq: i32;
        let mut next_index: i32;
        let mut rest: metamodelica::List<i32>;
        let mut vars: metamodelica::List<i32>;
        let mut eqs: metamodelica::List<i32>;
        let mut part: metamodelica::List<i32>;
        let mut restPart: metamodelica::List<metamodelica::List<i32>>;
        let mut partitions: metamodelica::List<metamodelica::List<i32>>;
        ::match_deref::match_deref! { match &(checkNextIn) {
            Deref @ metamodelica::ListNode::Cons { head: 0, tail: Deref @ metamodelica::ListNode::Nil } => return Ok((currNumberIn - 1, partitionsIn)),
            Deref @ metamodelica::ListNode::Cons { head: __esc_eq, tail: __esc_rest } => {
                eq = (*__esc_eq).clone();
                rest = (*__esc_rest).clone();
                if arrayGetIsNotPositive(eq.clone(), markEqs.clone())? {
                    metamodelica::arrayUpdate(markEqs.clone(), eq.clone(), currNumberIn)?;
                    if (partitionsIn).is_empty() {
                        partitions = list![list![eq.clone()]];
                    } else {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partitionsIn) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        part = metamodelica::Own::own(__pa0);
                        restPart = metamodelica::Own::own(__pa1);
                        part = metamodelica::cons(eq.clone(), part);
                        partitions = metamodelica::cons(part, restPart);
                    }
                    vars = metamodelica::arrayGet(m.clone(), eq.clone())?;
                    let true = (!((vars).is_empty())) else { return Err("pattern mismatch") };
                    vars = List::filter1OnTrue(vars, (std::sync::Arc::new(arrayGetIsNotPositive) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markVars.clone())?;
                    List::map2_0(&vars, &Array::updateIndexFirst, currNumberIn, markVars.clone())?;
                    eqs = List::fold1(&vars, &getArrayEntryAndAppend, mT.clone(), metamodelica::nil())?;
                    eqs = List::filter1OnTrue(eqs, (std::sync::Arc::new(arrayGetIsNegative) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markEqs.clone())?;
                    List::map2_0(&eqs, &Array::updateIndexFirst, 0, markEqs.clone())?;
                    rest = listAppend(rest.clone(), eqs);
                } else {
                    partitions = partitionsIn;
                }
                { (m, mT, checkNextIn, markEqs, markVars, currNumberIn, partitionsIn, nextIndex) = (m.clone(), mT.clone(), rest.clone(), markEqs.clone(), markVars.clone(), currNumberIn, partitions, nextIndex); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                eq = 0;
                next_index = nextIndex;
                for mut i in nextIndex..=metamodelica::arrayLength(markEqs.clone()) {
                    if ({let __elt = (*metamodelica::index_checked(&markEqs.borrow(), i)?).clone(); __elt}) == -1 {
                        eq = i;
                        next_index = i + 1;
                        break;
                    }
                }
                { (m, mT, checkNextIn, markEqs, markVars, currNumberIn, partitionsIn, nextIndex) = (m.clone(), mT.clone(), list![eq], markEqs.clone(), markVars.clone(), currNumberIn + 1, metamodelica::cons(metamodelica::nil(), partitionsIn), next_index); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn arrayGetIsNotPositive(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx)? <= 0;
    Ok(isNonZero)
}

fn arrayGetIsNegative(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx)? < 0;
    Ok(isNonZero)
}

fn getArrayEntryAndAppend(
    mut entry: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut lstIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    let mut lst: metamodelica::List<i32>;
    lst = metamodelica::arrayGet(m.clone(), entry)?;
    lstOut = listAppend(lst, lstIn);
    Ok(lstOut)
}

fn gatherCrossNodes(
    mut idx: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut lstIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    let mut isCross: bool;
    let mut num: i32;
    let mut row: metamodelica::List<i32>;
    row = metamodelica::arrayGet(m.clone(), idx)?;
    num = ((row).len() as i32);
    isCross = intGt(num, 2);
    lstOut = if (isCross) {
        metamodelica::cons(idx, lstIn)
    } else {
        lstIn
    };
    Ok(lstOut)
}

fn isAddOrSubExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inTuple: &(bool, BackendDAE::Variables),
) -> Result<(metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (bool, BackendDAE::Variables);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((&**inExp, inTuple)) {
        (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (true, vars)) => {
            let mut b: bool;
            b = Expression::subscriptConstants(&(ComponentReferenceBasics::crefSubs(metamodelica::AsArg::as_arg(&cref))?));
            (inExp.clone(), (b, vars.clone()))
        },
        (Deref @ DAE::Exp::UNARY { exp: exp1, .. }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(metamodelica::AsArg::as_arg(&exp1), &((true, vars.clone())))?;
            b = metamodelica::Own::own(__pa0);
            (inExp.clone(), (b, vars.clone()))
        },
        (Deref @ DAE::Exp::RCONST { .. }, (true, vars)) => {
            (inExp.clone(), (true, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(metamodelica::AsArg::as_arg(&exp1), &((true, vars.clone())))?;
            b = metamodelica::Own::own(__pa0);
            let (_, (__pa1, _)) = isAddOrSubExp(metamodelica::AsArg::as_arg(&exp2), &((b, vars.clone())))?;
            b = metamodelica::Own::own(__pa1);
            (inExp.clone(), (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(metamodelica::AsArg::as_arg(&exp1), &((true, vars.clone())))?;
            b = metamodelica::Own::own(__pa0);
            let (_, (__pa1, _)) = isAddOrSubExp(metamodelica::AsArg::as_arg(&exp2), &((b, vars.clone())))?;
            b = metamodelica::Own::own(__pa1);
            (inExp.clone(), (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, operator: DAE::Operator::MUL { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            b = BackendVariable::isState(cref.clone(), metamodelica::AsArg::as_arg(&vars)) && Expression::isConst(exp2.clone())?;
            (inExp.clone(), (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: cref, .. } }, (true, vars)) => {
            let mut b: bool;
            b = Expression::isConst(exp1.clone())? && BackendVariable::isState(cref.clone(), metamodelica::AsArg::as_arg(&vars));
            (inExp.clone(), (b, vars.clone()))
        },
        _ => {
            (inExp.clone(), (false, Util::tuple22(inTuple.clone())))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn sumUp2Expressions(
    mut sumUp: bool,
    mut exp1: metamodelica::Ref<DAE::Exp>,
    mut exp2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut op: DAE::Operator;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = DAE::T_REAL_DEFAULT().clone();
    op = if (sumUp) {
        DAE::Operator::ADD { ty: ty }
    } else {
        DAE::Operator::SUB { ty: ty }
    };
    expOut = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: exp1,
        operator: op,
        exp2: exp2,
    });
    (expOut, _) = ExpressionSimplify::simplify(expOut)?;
    Ok(expOut)
}

fn intLstIsEqual(mut lst1: metamodelica::List<i32>, mut lst2: metamodelica::List<i32>) -> Result<bool> {
    let mut bOut: bool;
    bOut = List::isEqualOnTrue(lst1, lst2, &fnptr!(intEq, i32, i32))?;
    Ok(bOut)
}

fn sortPathsAsChain(
    mut pathsIn: metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut pathsOut: metamodelica::List<metamodelica::List<i32>>;
    pathsOut = 'mc: {
        let __mc_input = &*pathsIn;
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
                _ => {
                    let mut pathLst: metamodelica::List<metamodelica::List<i32>>;
                    pathLst = sortPathsAsChain1(&pathsIn, 0, 0, &(metamodelica::nil()))?;
                    Ok(pathLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(pathsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    pathsOut
}

fn sortPathsAsChain1(
    mut pathsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut firstNode: i32,
    mut lastNode: i32,
    mut sortedPathsIn: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut sortedPathsOut: metamodelica::List<metamodelica::List<i32>>;
    sortedPathsOut = 'mc: {
        let __mc_input = (&**pathsIn, firstNode, lastNode, &**sortedPathsIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok(sortedPathsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (-1), (-1), _) => {
                    Ok(sortedPathsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: path, tail: rest }, _, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut sortedPaths: metamodelica::List<metamodelica::List<i32>>;
                    startNode = (path).head().cloned()?;
                    endNode = List::last(metamodelica::AsArg::as_arg(&path))?;
                    sortedPaths = sortPathsAsChain1(metamodelica::AsArg::as_arg(&rest), startNode, endNode, &(list![path.clone()]))?;
                    Ok(sortedPaths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut endNode: i32;
                    let mut path: metamodelica::List<i32>;
                    let mut rest: metamodelica::List<metamodelica::List<i32>>;
                    let mut paths1: metamodelica::List<metamodelica::List<i32>>;
                    let mut paths2: metamodelica::List<metamodelica::List<i32>>;
                    let mut allPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut sortedPaths: metamodelica::List<metamodelica::List<i32>>;
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), lastNode)?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), lastNode)?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = ((allPaths).is_empty()) else { return Err("pattern mismatch") };
                    path = (allPaths).head().cloned()?;
                    endNode = if (!((allPaths).is_empty())) {List::last(&path)?} else {-1};
                    endNode = if (!((paths2).is_empty())) {(path).head().cloned()?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), &({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2) }))?;
                    sortedPaths = listAppend(sortedPathsIn.clone(), list![path.clone()]);
                    sortedPaths = sortPathsAsChain1(&rest, firstNode, endNode, &sortedPaths)?;
                    Ok(sortedPaths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut startNode: i32;
                    let mut path: metamodelica::List<i32>;
                    let mut rest: metamodelica::List<metamodelica::List<i32>>;
                    let mut paths1: metamodelica::List<metamodelica::List<i32>>;
                    let mut paths2: metamodelica::List<metamodelica::List<i32>>;
                    let mut allPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut sortedPaths: metamodelica::List<metamodelica::List<i32>>;
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), firstNode)?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>), firstNode)?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = ((allPaths).is_empty()) else { return Err("pattern mismatch") };
                    path = (allPaths).head().cloned()?;
                    startNode = if (!((allPaths).is_empty())) {List::last(&path)?} else {-1};
                    startNode = if (!((paths2).is_empty())) {(path).head().cloned()?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), &({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, &*__pe_b2) }))?;
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    sortedPaths = sortPathsAsChain1(&rest, startNode, lastNode, &sortedPaths)?;
                    Ok(sortedPaths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32;
                    let mut path: metamodelica::List<i32>;
                    let mut rest: metamodelica::List<metamodelica::List<i32>>;
                    let mut sortedPaths: metamodelica::List<metamodelica::List<i32>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*pathsIn)) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    rest = metamodelica::Own::own(__pa1);
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    startNode = (path).head().cloned()?;
                    sortedPaths = sortPathsAsChain1(&rest, startNode, lastNode, &sortedPaths)?;
                    Ok(sortedPaths.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(sortedPathsOut)
}

fn firstInListIsEqual(mut lstIn: &metamodelica::List<i32>, mut value: i32) -> Result<bool> {
    let mut isEq: bool;
    let mut first: i32;
    first = (lstIn).head().cloned()?;
    isEq = intEq(first, value);
    Ok(isEq)
}

fn lastInListIsEqual(mut lstIn: &metamodelica::List<i32>, mut value: i32) -> Result<bool> {
    let mut isEq: bool;
    let mut last: i32;
    last = List::last(lstIn)?;
    isEq = intEq(last, value);
    Ok(isEq)
}

fn connect2PathsToLoops(
    mut pathsIn: metamodelica::List<metamodelica::List<i32>>,
    mut loopsIn: metamodelica::List<metamodelica::List<i32>>,
    mut restPathsIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut pathsOut: metamodelica::List<metamodelica::List<i32>> = loopsIn;
    let mut restPathsOut: metamodelica::List<metamodelica::List<i32>> = restPathsIn;
    let mut closedALoop: bool;
    let mut startNode: i32;
    let mut endNode: i32;
    let mut path: metamodelica::List<i32>;
    let mut rest: metamodelica::List<metamodelica::List<i32>> = pathsIn.clone();
    let mut endPaths: metamodelica::List<metamodelica::List<i32>>;
    let mut startPaths: metamodelica::List<metamodelica::List<i32>>;
    let mut newLoops: metamodelica::List<metamodelica::List<i32>>;
    if (pathsIn).is_empty() {
        pathsOut = metamodelica::nil();
        restPathsOut = metamodelica::nil();
        return Ok((pathsOut, restPathsOut));
    }
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        path = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        startNode = (path).head().cloned()?;
        endNode = List::last(&path)?;
        if intEq(startNode, endNode) {
            pathsOut = metamodelica::cons(path, pathsOut);
        } else if (rest).is_empty() {
            restPathsOut = metamodelica::cons(path, restPathsOut);
        } else {
            startPaths = List::filter1OnTrue(
                rest.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
                startNode,
            )?;
            startPaths = List::filter1OnTrue(
                startPaths,
                (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
                endNode,
            )?;
            endPaths = List::filter1OnTrue(
                rest.clone(),
                (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| firstInListIsEqual(&__a0, __a1))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
                endNode,
            )?;
            endPaths = List::filter1OnTrue(
                endPaths,
                (std::sync::Arc::new(move |__a0: metamodelica::List<i32>, __a1: i32| lastInListIsEqual(&__a0, __a1))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<i32>, i32) -> Result<bool> + 'static>),
                startNode,
            )?;
            endPaths = listAppend(startPaths, endPaths);
            closedALoop = !((endPaths).is_empty());
            newLoops = if (closedALoop) {
                connectPaths(&path, endPaths)?
            } else {
                metamodelica::nil()
            };
            if !(closedALoop) {
                restPathsOut = metamodelica::cons(path, restPathsOut);
            }
            pathsOut = listAppend(newLoops, pathsOut);
        }
    }
    Ok((pathsOut, restPathsOut))
}

fn connectPaths(
    mut pathIn: &metamodelica::List<i32>,
    mut closingPaths: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut loopsOut: metamodelica::List<metamodelica::List<i32>>;
    let mut path: metamodelica::List<i32>;
    let __pa0 = ::match_deref::match_deref! { match &((*pathIn)) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    path = List::stripLast(path)?;
    loopsOut = List::map1(
        closingPaths,
        &fnptr!(listAppend, metamodelica::List<i32>, metamodelica::List<i32>),
        path,
    )?;
    Ok(loopsOut)
}

//____________________________________________________
//reshuffle systems of equations, not yet finished
//____________________________________________________
pub(crate) fn reshuffling_post(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    if Flags::isSet(Flags::RESHUFFLE_POST.clone())? {
        eqSystems = List::map1(inDAE.eqs.clone(), &reshuffling_post0, inDAE.shared.clone())?;
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqSystems,
            shared: inDAE.shared.clone(),
        });
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn reshuffling_post0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    osyst = List::fold1(
        &comps,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: metamodelica::Ref<BackendDAE::EqSystem>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(reshuffling_post1(&__a0, &__a1, __a2)) },
        shared,
        isyst,
    )?;
    Ok(osyst)
}

fn reshuffling_post1(
    mut compIn: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut systIn: metamodelica::Ref<BackendDAE::EqSystem>,
) -> metamodelica::Ref<BackendDAE::EqSystem> {
    let mut systOut: metamodelica::Ref<BackendDAE::EqSystem>;
    systOut = 'mc: {
        let __mc_input = &**compIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcs, vars: vIdcs, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: ojac }, jacType: jacType @ BackendDAE::JacobianType::JAC_LINEAR { .. }, .. } => {
                    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
                    (eqSys, _) = reshuffling_post2(eqIdcs.clone(), vIdcs.clone(), systIn.clone(), shared, ojac.clone(), jacType.clone())?;
                    Ok(eqSys.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(systIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    systOut
}

fn reshuffling_post2(
    mut eqIdcs: metamodelica::List<i32>,
    mut varIdcs: metamodelica::List<i32>,
    mut dae: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut ojac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut jacType: BackendDAE::JacobianType,
) -> Result<(metamodelica::Ref<BackendDAE::EqSystem>, bool)> {
    let mut daeOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outRunMatching: bool;
    let mut size: i32;
    let mut resEqs: metamodelica::List<metamodelica::List<i32>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut ass1Sys: metamodelica::Array<i32>;
    let mut ass2Sys: metamodelica::Array<i32>;
    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut daeEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut daeVars: BackendDAE::Variables;
    let mut subSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqsInLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    size = ((varIdcs).len() as i32);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa2, ass2: __pa3, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    daeVars = metamodelica::Own::own(__pa0);
    daeEqs = metamodelica::Own::own(__pa1);
    ass1Sys = metamodelica::Own::own(__pa2);
    ass2Sys = metamodelica::Own::own(__pa3);
    funcs = BackendDAEUtil::getFunctions(shared);
    eqLst = BackendEquation::getList(eqIdcs.clone(), daeEqs.clone())?;
    eqs = BackendEquation::listEquation(&eqLst)?;
    varLst = List::map1r(
        varIdcs.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        daeVars,
    )?;
    vars = BackendVariable::listVar1(&varLst)?;
    subSys = BackendDAEUtil::createEqSystem(
        vars.clone(),
        eqs.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (me, meT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&subSys, shared, false)?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        subSys,
        openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
        Some(BackendDAEUtil::getFunctions(shared)),
        BackendDAEUtil::isInitializationDAE(shared),
    )?;
    ass1 = arrayCreate(size, -1);
    ass2 = arrayCreate(size, -1);
    varAtts = List::threadMap(
        List::fill(false, ((varLst).len() as i32)),
        List::map(eqIdcs.clone(), &fnptr!(intString, i32))?,
        &fnptr!(Util::makeTuple, _, _),
    )?;
    eqAtts = List::threadMap(
        List::fill(false, ((eqLst).len() as i32)),
        List::map(varIdcs, &fnptr!(intString, i32))?,
        &fnptr!(Util::makeTuple, _, _),
    )?;
    BackendDump::dumpBipartiteGraphStrongComponent2(
        vars,
        eqs,
        m.clone(),
        varAtts,
        &eqAtts,
        &(literal!("shuffle_pre")),
    )?;
    resEqs = reshuffling_post3_selectShuffleEqs(me.clone(), meT.clone());
    eqsInLst = reshuffling_post4_resolveAndReplace(&resEqs, &eqLst, &varLst, me.clone(), meT.clone())?;
    daeEqs = List::threadFold(&eqIdcs, eqsInLst, &BackendEquation::setAtIndexFirst, daeEqs)?;
    daeOut = BackendDAEUtil::setEqSystEqs(dae, daeEqs);
    daeOut = BackendDAEUtil::setEqSystMatching(
        daeOut,
        metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ass1Sys.clone(),
            ass2: ass2Sys.clone(),
            comps: metamodelica::nil(),
        }),
    );
    (daeOut, _, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        daeOut,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        Some(funcs),
        BackendDAEUtil::isInitializationDAE(shared),
    )?;
    outRunMatching = true;
    Ok((daeOut, outRunMatching))
}

fn reshuffling_post3_selectShuffleEqs(
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut resolveEqs: metamodelica::List<metamodelica::List<i32>>;
    resolveEqs = 'mc: {
        let __mc_input = meT.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut bArr: metamodelica::Array<bool>;
            let mut suitableEqs: metamodelica::List<i32>;
            let mut eqPairs: metamodelica::List<metamodelica::List<i32>>;
            bArr = Array::map1(me.clone(), &chooseEquation, meT.clone())?;
            (_, suitableEqs) = List::filter1OnTrueSync(
                &(bArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
                &fnptr!(boolEq, bool, bool),
                true,
                List::intRange(metamodelica::arrayLength(me.clone())),
            )?;
            eqPairs = List::map2(suitableEqs.clone(), &getEqPairs, me.clone(), meT.clone())?;
            eqPairs = List::filterOnTrue(
                eqPairs.clone(),
                std::sync::Arc::new(move |__a0: _| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(List::hasSeveralElements(&__a0))
                }),
            )?;
            Ok(eqPairs.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("reshuffling_post3_selectShuffleEqs failed!\n"));
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    resolveEqs
}

fn reshuffling_post4_resolveAndReplace(
    mut resolveEqLst: &metamodelica::List<metamodelica::List<i32>>,
    mut unassEqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut unassVarsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut unassEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    unassEqsOut = 'mc: {
        let __mc_input = &**resolveEqLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(unassEqsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: resolveEqs, tail: rest } => {
                    let mut maxNum: i32;
                    let mut replEqIdx: i32;
                    let mut numOfAdjVars: metamodelica::List<i32>;
                    let mut unassEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut resolvedEq: metamodelica::Ref<BackendDAE::Equation>;
                    resolvedEq = resolveEquations(None, metamodelica::AsArg::as_arg(&resolveEqs), me.clone(), meT.clone(), unassEqsIn, unassVarsIn)?;
                    numOfAdjVars = List::map(List::map1(resolveEqs.clone(), &Array::getIndexFirst, me.clone())?, &fnptr!(listLength, _))?;
                    maxNum = List::fold(&(numOfAdjVars.clone()), &fnptr!(intMax, i32, i32), (numOfAdjVars).head().cloned()?)?;
                    replEqIdx = (resolveEqs).get(List::position(maxNum, &numOfAdjVars)?)?;
                    unassEqs = List::replaceAt(resolvedEq.clone(), replEqIdx, unassEqsIn.clone())?;
                    Ok(reshuffling_post4_resolveAndReplace(metamodelica::AsArg::as_arg(&rest), &unassEqs, unassVarsIn, me.clone(), meT.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("reshuffling_post4_resolveAndReplace failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(unassEqsOut)
}

fn getEqPairs(
    mut eq: i32,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut eqs: metamodelica::List<i32>;
    vars = List::map(metamodelica::arrayGet(me.clone(), eq)?, &fnptr!(Util::tuple31, _))?;
    eqs = List::map(
        List::flatten(List::map1(vars, &Array::getIndexFirst, meT.clone())?)?,
        &fnptr!(Util::tuple31, _),
    )?;
    eqs = getDoublicates(&eqs)?;
    lstOut = List::consOnTrue(!(listMember(eq, eqs.clone())), eq, eqs);
    Ok(lstOut)
}

fn chooseEquation(
    mut row: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<bool> {
    let mut chooseThis: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut vars: metamodelica::List<i32>;
    let mut eqs: metamodelica::List<i32>;
    let mut numEqs: metamodelica::List<i32>;
    let mut eqLst: metamodelica::List<metamodelica::List<i32>>;
    vars = List::map(row.clone(), &fnptr!(Util::tuple31, _))?;
    b1 = intEq(((row).len() as i32), 2);
    eqLst = List::mapList(
        List::map1(vars, &Array::getIndexFirst, meT.clone())?,
        &fnptr!(Util::tuple31, _),
    )?;
    numEqs = List::map(eqLst.clone(), &fnptr!(listLength, _))?;
    b3 = List::applyAndFold1(&numEqs, &fnptr!(boolOr, bool, bool), &fnptr!(intEq, i32, i32), 2, false)?;
    eqs = List::flatten(eqLst)?;
    b2 = intEq(((eqs).len() as i32), ((List::unique(&eqs)).len() as i32) + 2);
    b1 = b1 && b2 && b3;
    chooseThis = b1 && List::applyAndFold(&row, &fnptr!(boolAnd, bool, bool), &isSolvable, true)?;
    Ok(chooseThis)
}

fn getDoublicates(mut lstIn: &metamodelica::List<i32>) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    let mut max: i32;
    let mut arr: metamodelica::Array<i32>;
    max = List::fold(lstIn, &fnptr!(intMax, i32, i32), (lstIn).head().cloned()?)?;
    arr = arrayCreate(max, -1);
    List::map1_0(lstIn, &getDoublicates2, arr.clone())?;
    (_, lstOut) = List::filter1OnTrueSync(
        &(arr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        &fnptr!(intGe, i32, i32),
        1,
        List::intRange(metamodelica::arrayLength(arr.clone())),
    )?;
    Ok(lstOut)
}

fn getDoublicates2(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<()> {
    let mut entry: i32;
    entry = metamodelica::arrayGet(arr.clone(), idx)?;
    metamodelica::arrayUpdate(arr.clone(), idx, entry + 1)?;
    Ok(())
}

fn isSolvable(
    mut entry: (
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    ),
) -> Result<bool> {
    let mut solvable: bool;
    solvable = !(Tearing::unsolvable(&(list![entry]))?);
    Ok(solvable)
}

pub(crate) fn resolveEquations(
    mut eq: Option<metamodelica::Ref<BackendDAE::Equation>>,
    mut loopIn: &metamodelica::List<i32>,
    mut me: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut meT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut eqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut eqOut: metamodelica::Ref<BackendDAE::Equation>;
    eqOut = 'mc: {
        let __mc_input = (eq, &**loopIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(eq1), Deref @ metamodelica::ListNode::Nil) => {
                    Ok(eq1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, Deref @ metamodelica::ListNode::Cons { head: startEq, tail: rest }) => {
                    let mut nextEq: i32;
                    let mut sharedVar: i32;
                    let mut vars1: metamodelica::List<i32>;
                    let mut vars2: metamodelica::List<i32>;
                    let mut numEqs: metamodelica::List<i32>;
                    let mut eq1: metamodelica::Ref<BackendDAE::Equation>;
                    let mut eq2: metamodelica::Ref<BackendDAE::Equation>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut attr: BackendDAE::EquationAttributes;
                    let mut lhs1: metamodelica::Ref<DAE::Exp>;
                    let mut lhs2: metamodelica::Ref<DAE::Exp>;
                    let mut rhs1: metamodelica::Ref<DAE::Exp>;
                    let mut rhs2: metamodelica::Ref<DAE::Exp>;
                    let mut varExp: metamodelica::Ref<DAE::Exp>;
                    let mut eqExp: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    nextEq = metamodelica::Own::own(__pa0);
                    rest = metamodelica::Own::own(__pa1);
                    vars1 = List::map(metamodelica::arrayGet(me.clone(), startEq.clone())?, &fnptr!(Util::tuple31, _))?;
                    vars2 = List::map(metamodelica::arrayGet(me.clone(), nextEq)?, &fnptr!(Util::tuple31, _))?;
                    vars1 = List::intersectionOnTrue(&vars1, &vars2, &fnptr!(intEq, i32, i32))?;
                    numEqs = List::map(List::map1(vars1.clone(), &Array::getIndexFirst, meT.clone())?, &fnptr!(listLength, _))?;
                    (_, vars1) = List::filter1OnTrueSync(&numEqs, &fnptr!(intEq, i32, i32), 2, vars1.clone())?;
                    sharedVar = (vars1).head().cloned()?;
                    eq1 = (eqsIn).get(startEq.clone())?;
                    eq2 = (eqsIn).get(nextEq)?;
                    var = (varsIn).get(sharedVar)?;
                    varExp = Expression::crefExp(BackendVariable::varCref(&var))?;
                    let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(eq1.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa2, scalar: __pa3, source: __pa4, attr: __pa5 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs1 = metamodelica::Own::own(__pa2);
                    rhs1 = metamodelica::Own::own(__pa3);
                    source = metamodelica::Own::own(__pa4);
                    attr = metamodelica::Own::own(__pa5);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(eq2.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa6, scalar: __pa7, .. } => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs2 = metamodelica::Own::own(__pa6);
                    rhs2 = metamodelica::Own::own(__pa7);
                    (eqExp, _) = ExpressionSolve::solve(lhs1.clone(), rhs1.clone(), varExp.clone(), None)?;
                    (lhs2, _) = Expression::replaceExp(lhs2.clone(), varExp.clone(), eqExp.clone())?;
                    (rhs2, _) = Expression::replaceExp(rhs2.clone(), varExp.clone(), eqExp.clone())?;
                    (lhs2, _) = ExpressionSimplify::simplify(lhs2.clone())?;
                    (rhs2, _) = ExpressionSimplify::simplify(rhs2.clone())?;
                    eq2 = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs2.clone(), scalar: rhs2.clone(), source: source.clone(), attr: attr });
                    Ok(resolveEquations(Some(eq2.clone()), metamodelica::AsArg::as_arg(&rest), me.clone(), meT.clone(), eqsIn, varsIn)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("resolveEquations failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(eqOut)
}

// =============================================================================
// section for postOptModule >solveLinearSystem<<
//
// solve linear system of equations (A x = b)
// =============================================================================
pub(crate) fn solveLinearSystem(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut maxSize: i32 = Flags::getConfigInt(Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone())?;
    let mut b: bool = 1 < maxSize;
    if b {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(&inDAE, &solveLinearSystem0, (false, 1, maxSize))?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn solveLinearSystem0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (bool, i32, i32),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (bool, i32, i32),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outTpl: (bool, i32, i32);
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    (osyst, outShared, outTpl) = solveLinearSystem1(isyst, inShared, &comps, inTpl)?;
    Ok((osyst, outShared, outTpl))
}

fn solveLinearSystem1(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inTpl: (bool, i32, i32),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    (bool, i32, i32),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut outTpl: (bool, i32, i32);
    let mut b: bool;
    let mut runMatching: bool;
    let mut ii: metamodelica::List<i32> = metamodelica::nil();
    let mut offset: i32;
    let mut maxSize: i32;
    (runMatching, offset, maxSize) = inTpl;
    for mut comp in &**inComps {
        (osyst, oshared, b, ii, offset) =
            solveLinearSystem2(osyst, oshared, metamodelica::AsArg::as_arg(&comp), ii, offset, maxSize);
        runMatching = runMatching || b;
    }
    outTpl = (runMatching, offset, maxSize);
    if runMatching {
        osyst = (::match_deref::match_deref! { match &(osyst) {
            syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
                let mut syst = (*syst).clone();
                let mut eqns = (*eqns).clone();
                eqns = List::fold(&ii, &BackendEquation::delete, eqns.clone())?;
                assign_field!(
                    syst.orderedVars = BackendVariable::listVar1(&(BackendVariable::varList(metamodelica::AsArg::as_arg(&vars))?))?,
                    syst.orderedEqs = BackendEquation::listEquation(&(BackendEquation::equationList(eqns.clone())?))?
                );
                BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok((osyst, oshared, outTpl))
}

fn solveLinearSystem2(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut ii: metamodelica::List<i32>,
    mut offset: i32,
    mut maxSize: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    metamodelica::List<i32>,
    i32,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outRunMatching: bool;
    let mut oi: metamodelica::List<i32>;
    let mut offset_: i32;
    (osyst, oshared, outRunMatching, oi, offset_) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), &**comp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }) => {
                    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut toffset: i32;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone())?;
                    var_lst = List::map1r(vindx.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    let true = (((var_lst).len() as i32) <= maxSize) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(List::splitOnTrue(&var_lst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0)) })?) {
                        (Deref @ metamodelica::ListNode::Nil, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (syst, shared, toffset) = solveLinearSystem3(syst.clone(), shared.clone(), &eqn_lst, metamodelica::AsArg::as_arg(&eindex), var_lst.clone(), metamodelica::AsArg::as_arg(&vindx), jac.clone(), offset)?;
                    Ok((syst.clone(), shared.clone(), true, listAppend(eindex.clone(), ii.clone()), toffset))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone(), false, ii.clone(), offset))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared, outRunMatching, oi, offset_)
}

fn solveLinearSystem3(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut eqn_lst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut eqn_indxs: &metamodelica::List<i32>,
    mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut var_indxs: &metamodelica::List<i32>,
    mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut offset: i32,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut offset_: i32;
    (osyst, oshared, offset_) = (::match_deref::match_deref! { match &((inSyst, ishared)) {
        (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
            let mut beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut names: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut n: i32;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            let mut shared = (*shared).clone();
            (beqs, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqn_lst)?, BackendVariable::listVar1(&var_lst)?, Some(funcs.clone()))?;
            beqs = beqs.reverse();
            n = ((beqs).len() as i32);
            names = List::map(var_lst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::varCref(&__a0)) })?;
            (eqns, vars, n, shared) = solveLinearSystem4(&beqs, jac, names, var_lst, n, eqns.clone(), vars.clone(), offset, shared.clone())?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqns.clone()
            );
            syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None);
            (syst.clone(), shared.clone(), n)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared, offset_))
}

fn solveLinearSystem4(
    mut b_lst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    mut cr_x: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut n: i32,
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ivars: BackendDAE::Variables,
    mut offset: i32,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    i32,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = ieqns;
    let mut ovars: BackendDAE::Variables = ivars;
    let mut offset_: i32 = offset + 1;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut R: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut Qb: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut b: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n * n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut ax: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut scaled_x: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut scaleA: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut a: metamodelica::Ref<DAE::Exp>;
    let mut m: i32;
    let mut ii: i32;
    let mut jj: i32;
    let mut mm: i32;
    let mut x_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = List::map(cr_x.clone(), &Expression::crefExp)?;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = var_lst;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut jac_: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)> = jac.clone();
    mm = ((jac).len() as i32);
    for mut i in 1..=mm {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(jac_) {
            Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa2, .. }), tail: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        jj = metamodelica::Own::own(__pa0);
        ii = metamodelica::Own::own(__pa1);
        a = metamodelica::Own::own(__pa2);
        jac_ = metamodelica::Own::own(__pa3);
        m = ii + (jj - 1) * n;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            a,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$A$"));
                __mm_s.push_str(&*intString(m));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(A.clone(), m, a)?;
    }
    for mut i in 1..=n {
        m = (i - 1) * n;
        a = Expression::makeSum1(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut j in (1..=n).into_iter() {
                    let __x = Expression::makeAbs(metamodelica::arrayGet(A.clone(), m + j.clone())?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            false,
        )?;
        metamodelica::arrayUpdate(scaleA.clone(), i, a)?;
    }
    for mut i in 1..=n {
        m = (i - 1) * n;
        for mut j in 1..=n {
            a = metamodelica::arrayGet(A.clone(), j + m)?;
            if !(Expression::isZero(&a)?) {
                a = Expression::expDiv(a, metamodelica::arrayGet(scaleA.clone(), j)?)?;
                (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                    a,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("QR$sA$"));
                        __mm_s.push_str(&*intString(i + (j - 1) * n));
                        ArcStr::from(__mm_s)
                    }),
                    offset,
                    oeqns,
                    ovars,
                    oshared,
                    false,
                )?;
                metamodelica::arrayUpdate(A.clone(), j + m, a)?;
            }
        }
    }
    m = 1;
    for mut b_ in &**b_lst {
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            b_.clone(),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$b$"));
                __mm_s.push_str(&*intString(m));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(b.clone(), m, a)?;
        m = m + 1;
    }
    m = 1;
    for mut xx in &*x_lst {
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(vars) {
            Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        var = metamodelica::Own::own(__pa5);
        vars = metamodelica::Own::own(__pa6);
        if BackendVariable::isStateVar(&var) {
            metamodelica::arrayUpdate(ax.clone(), m, Expression::expDer(xx.clone()))?;
        } else {
            metamodelica::arrayUpdate(ax.clone(), m, xx.clone())?;
        }
        m = m + 1;
    }
    for mut i in 1..=n {
        a = Expression::expMul(
            metamodelica::arrayGet(ax.clone(), i)?,
            metamodelica::arrayGet(scaleA.clone(), i)?,
        )?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            a,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$sx$"));
                __mm_s.push_str(&*intString(i));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(scaled_x.clone(), i, a)?;
    }
    (R, Qb, oeqns, ovars, oshared) =
        qrDecompositionHouseholder(A.clone(), n, b.clone(), oeqns, ovars, offset, oshared)?;
    for mut i in ({
        let __s = n;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        m = (i - 1) * n;
        a = Expression::makeSum1(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut j in (i..=n).into_iter() {
                    let __x = Expression::expMul(
                        metamodelica::arrayGet(R.clone(), m + j.clone())?,
                        metamodelica::arrayGet(scaled_x.clone(), j.clone())?,
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            false,
        )?;
        eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: a,
            scalar: metamodelica::arrayGet(Qb.clone(), i)?,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
        });
        eqn = BackendEquation::solveEquation(eqn, metamodelica::arrayGet(scaled_x.clone(), i)?, None)?;
        oeqns = BackendEquation::add(eqn, oeqns)?;
    }
    Ok((oeqns, ovars, offset_, oshared))
}

fn qrDecompositionHouseholder(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut n: i32,
    mut ib: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ivars: BackendDAE::Variables,
    mut offset: i32,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut R: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = A.clone();
    let mut b: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = ib;
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = ieqns;
    let mut ovars: BackendDAE::Variables = ivars;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut cA: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut alpha: metamodelica::Ref<DAE::Exp>;
    let mut y1: metamodelica::Ref<DAE::Exp>;
    let mut h: metamodelica::Ref<DAE::Exp>;
    let mut h2: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut m: i32;
    let mut nn: i32 = n - 1;
    let mut idxVars: i32 = 1;
    let mut shift: i32;
    for mut iter in 1..=nn {
        m = n - iter + 1;
        qrGet_cA(A.clone(), iter, 1, n, v.clone())?;
        y1 = metamodelica::arrayGet(v.clone(), 1)?;
        alpha = qrCalc_alpha(v.clone(), y1.clone(), m)?;
        (alpha, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            alpha,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$a$"));
                __mm_s.push_str(&*intString(iter));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        e = Expression::expAdd(y1.clone(), alpha.clone())?;
        (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            e,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$y1$"));
                __mm_s.push_str(&*intString(iter));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(v.clone(), 1, e)?;
        h = Expression::expAdd(y1, alpha.clone())?;
        h = Expression::expMul(alpha.clone(), h)?;
        h = Expression::negate(h)?;
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            h,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$h$"));
                __mm_s.push_str(&*intString(iter));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        shift = (iter - 1) * n + iter;
        metamodelica::arrayUpdate(R.clone(), shift, Expression::negate(alpha)?)?;
        for mut j in 2..=m {
            metamodelica::arrayUpdate(
                R.clone(),
                shift + (j - 1) * n,
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
            )?;
        }
        for mut col in 2..=m {
            qrGet_cA(A.clone(), iter, col, n, cA.clone())?;
            h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
            h2 = Expression::expDiv(h2, h.clone())?;
            (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                h2,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("QR$h2$"));
                    __mm_s.push_str(&*intString(idxVars));
                    ArcStr::from(__mm_s)
                }),
                offset,
                oeqns,
                ovars,
                oshared,
                false,
            )?;
            idxVars = idxVars + 1;
            for mut j in 1..=m {
                e1 = metamodelica::arrayGet(cA.clone(), j)?;
                e2 = metamodelica::arrayGet(v.clone(), j)?;
                e = Expression::expAdd(e1, Expression::expMul(h2.clone(), e2)?)?;
                (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                    e,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("QR$R$"));
                        __mm_s.push_str(&*intString(idxVars));
                        ArcStr::from(__mm_s)
                    }),
                    offset,
                    oeqns,
                    ovars,
                    oshared,
                    false,
                )?;
                idxVars = idxVars + 1;
                metamodelica::arrayUpdate(A.clone(), shift + (j - 1) * n + col - 1, e)?;
            }
        }
        for mut j in 1..=m {
            metamodelica::arrayUpdate(cA.clone(), j, metamodelica::arrayGet(b.clone(), iter - 1 + j)?)?;
        }
        h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
        h2 = Expression::expDiv(h2, h)?;
        (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            h2,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$b_$"));
                __mm_s.push_str(&*intString(idxVars));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        idxVars = idxVars + 1;
        for mut j in 1..=m {
            e1 = metamodelica::arrayGet(cA.clone(), j)?;
            e2 = metamodelica::arrayGet(v.clone(), j)?;
            e = Expression::expAdd(e1, Expression::expMul(h2.clone(), e2)?)?;
            e = Expression::expand(&e)?;
            e = ExpressionSimplify::simplify2(e, true, true)?;
            (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                e,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("QR$b$"));
                    __mm_s.push_str(&*intString(idxVars));
                    ArcStr::from(__mm_s)
                }),
                offset,
                oeqns,
                ovars,
                oshared,
                false,
            )?;
            idxVars = idxVars + 1;
            metamodelica::arrayUpdate(b.clone(), iter - 1 + j, e)?;
        }
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrGet_cA(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut iter: i32,
    mut j: i32,
    mut n: i32,
    mut cA: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
) -> Result<()> {
    let mut shift: i32 = (iter - 1) * n + iter + j - 1;
    let mut m: i32 = n - iter + 1;
    for mut i in 1..=m {
        metamodelica::arrayUpdate(cA.clone(), i, metamodelica::arrayGet(A.clone(), shift + (i - 1) * n)?)?;
    }
    for mut i in m + 1..=n {
        metamodelica::arrayUpdate(
            cA.clone(),
            i,
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            }),
        )?;
    }
    Ok(())
}

fn qrCalc_alpha(
    mut y: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut y1: metamodelica::Ref<DAE::Exp>,
    mut m: i32,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut alpha: metamodelica::Ref<DAE::Exp>;
    let mut sgn_y1: metamodelica::Ref<DAE::Exp> = Expression::makeSign(y1.clone());
    let mut norm_y: metamodelica::Ref<DAE::Exp> = Expression::lenVec(y.clone())?;
    norm_y = Expression::makeSum1(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut j in (1..=m).into_iter() {
                let __x = Expression::expPow(
                    metamodelica::arrayGet(y.clone(), j.clone())?,
                    metamodelica::Ref::new(DAE::Exp::RCONST {
                        real: metamodelica::OrderedFloat(2.0_f64),
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        false,
    )?;
    norm_y = Expression::makePureBuiltinCall(literal!("sqrt"), list![norm_y], DAE::T_REAL_DEFAULT().clone());
    alpha = Expression::expMul(sgn_y1, norm_y)?;
    Ok(alpha)
}

fn qrDecomposition(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut n: i32,
    mut ib: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ivars: BackendDAE::Variables,
    mut offset: i32,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut R: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n * n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut b: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = ieqns;
    let mut ovars: BackendDAE::Variables = ivars;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut Q: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n * n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut u: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        n,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut x: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut y: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut a: metamodelica::Ref<DAE::Exp>;
    let mut kk: i32 = 1;
    let mut m: i32 = n - 1;
    let mut nn: i32;
    v = qrDecomposition1(A.clone(), n, kk)?;
    (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(
        v.clone(),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("QR$NOM$"));
            __mm_s.push_str(&*intString(kk));
            ArcStr::from(__mm_s)
        }),
        offset,
        oeqns,
        ovars,
        ishared,
    )?;
    for mut j in 1..=n {
        (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j)?)?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            a,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$Q$"));
                __mm_s.push_str(&*intString(kk + (j - 1) * n));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(Q.clone(), kk + (j - 1) * n, a)?;
    }
    for mut k in 1..=m {
        v = qrDecomposition1(A.clone(), n, k + 1)?;
        for mut j in 1..=k {
            u = qrDecomposition1(Q.clone(), n, j)?;
            (v, oeqns, ovars, oshared) = gramSchmidtProcessHelper(
                v.clone(),
                u.clone(),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("QR$W$"));
                    __mm_s.push_str(&*intString(kk));
                    __mm_s.push_str(&*literal!("$"));
                    __mm_s.push_str(&*intString(kk));
                    ArcStr::from(__mm_s)
                }),
                offset,
                oeqns,
                ovars,
                oshared,
            )?;
            kk = kk + 1;
        }
        (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(
            v.clone(),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$NOM$"));
                __mm_s.push_str(&*intString(k + 1));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
        )?;
        for mut j in 1..=n {
            nn = k + 1 + (j - 1) * n;
            (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j)?)?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                a,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("QR$Q$"));
                    __mm_s.push_str(&*intString(nn));
                    ArcStr::from(__mm_s)
                }),
                offset,
                oeqns,
                ovars,
                oshared,
                false,
            )?;
            metamodelica::arrayUpdate(Q.clone(), nn, a)?;
        }
    }
    for mut i in 1..=n {
        x = qrDecomposition1(Q.clone(), n, i)?;
        m = (i - 1) * n;
        for mut j in i..=n {
            y = qrDecomposition1(A.clone(), n, j)?;
            a = Expression::makeScalarProduct(x.clone(), y.clone())?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
                a,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("QR$R$"));
                    __mm_s.push_str(&*intString(m + j));
                    ArcStr::from(__mm_s)
                }),
                offset,
                oeqns,
                ovars,
                oshared,
                false,
            )?;
            metamodelica::arrayUpdate(R.clone(), m + j, a)?;
        }
    }
    for mut i in 1..=n {
        x = qrDecomposition1(Q.clone(), n, i)?;
        a = Expression::makeScalarProduct(x.clone(), ib.clone())?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            a,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("QR$Qb$"));
                __mm_s.push_str(&*intString(i));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(b.clone(), i, a)?;
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrDecomposition1(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut sizeA: i32,
    mut i: i32,
) -> Result<metamodelica::Array<metamodelica::Ref<DAE::Exp>>> {
    let mut column: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        sizeA,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    for mut j in 1..=sizeA {
        metamodelica::arrayUpdate(
            column.clone(),
            j,
            metamodelica::arrayGet(A.clone(), i + (j - 1) * sizeA)?,
        )?;
    }
    Ok(column)
}

fn qrDecomposition2(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut sizeA: i32,
    mut i: i32,
) -> Result<metamodelica::Array<metamodelica::Ref<DAE::Exp>>> {
    let mut row: metamodelica::Array<metamodelica::Ref<DAE::Exp>> = arrayCreate(
        sizeA,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    let mut k: i32 = i - 1;
    for mut j in 1..=sizeA {
        metamodelica::arrayUpdate(row.clone(), j, metamodelica::arrayGet(A.clone(), j + k * sizeA)?)?;
    }
    Ok(row)
}

fn qrDecomposition3(
    mut A: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut sizeA: i32,
    mut isMat: bool,
    mut s: &ArcStr,
) -> Result<()> {
    let mut n: i32 = sizeA;
    let mut m: i32 = if (isMat) { sizeA } else { 1 };
    metamodelica::print(literal!("\n"));
    for mut i in 1..=n {
        metamodelica::print(literal!("\n"));
        for mut j in 1..=m {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(i));
                __mm_s.push_str(&*literal!(","));
                __mm_s.push_str(&*intString(j));
                __mm_s.push_str(&*literal!(") = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(metamodelica::arrayGet(
                    A.clone(),
                    (i - 1) * m + j,
                )?)?);
                __mm_s.push_str(&*literal!("\t"));
                ArcStr::from(__mm_s)
            });
        }
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn gramSchmidtProcessHelper(
    mut w: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut u: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut name: &ArcStr,
    mut offset: i32,
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ivars: BackendDAE::Variables,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut ovars: BackendDAE::Variables;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut h: metamodelica::Ref<DAE::Exp> = Expression::makeScalarProduct(w.clone(), u.clone())?;
    let mut n: i32 = metamodelica::arrayLength(w.clone());
    (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
        h,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!("_h"));
            ArcStr::from(__mm_s)
        }),
        offset,
        ieqns,
        ivars,
        ishared,
        false,
    )?;
    v = Array::map1(u.clone(), &Expression::expMul, h)?;
    v = Expression::subVec(w.clone(), v.clone())?;
    for mut i in 1..=n {
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(
            metamodelica::arrayGet(v.clone(), i)?,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*intString(i));
                ArcStr::from(__mm_s)
            }),
            offset,
            oeqns,
            ovars,
            oshared,
            false,
        )?;
        metamodelica::arrayUpdate(v.clone(), i, h)?;
    }
    Ok((v, oeqns, ovars, oshared))
}
