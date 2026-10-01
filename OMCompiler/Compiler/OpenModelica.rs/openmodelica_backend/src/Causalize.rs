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
use crate::BackendDAEFunc;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpGraphML;
use crate::Matching;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub type DAEHandler = (
    BackendDAEFunc::StructurallySingularSystemHandlerFunc,
    ArcStr,
    BackendDAEFunc::stateDeselectionFunc,
    ArcStr,
);

/* ****************************************
Singular System check
*****************************************/
pub(crate) fn singularSystemCheck(
    mut nvars: i32,
    mut neqns: i32,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inMatchingOptions: (BackendDAE::IndexReduction, BackendDAE::EquationConstraints),
    mut matchingAlgorithm: &(
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
        ArcStr,
    ),
    mut arg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    outSyst = 'mc: {
        let __mc_input = inMatchingOptions;
        if let Ok(__v) = (|| -> Result<_> {
            let (_, BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(singularSystemCheck1(
                nvars,
                neqns,
                isyst.clone(),
                openmodelica_backend_types::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED,
                matchingAlgorithm,
                arg.clone(),
                ishared.clone(),
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, BackendDAE::EquationConstraints::EXACT { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (intEq(nvars, neqns)) else {
                return Err("pattern mismatch");
            };
            Ok(singularSystemCheck1(
                nvars,
                neqns,
                isyst.clone(),
                openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
                matchingAlgorithm,
                arg.clone(),
                ishared.clone(),
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, BackendDAE::EquationConstraints::EXACT { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut esize_str: ArcStr;
            let mut vsize_str: ArcStr;
            let true = (intGt(nvars, neqns)) else {
                return Err("pattern mismatch");
            };
            esize_str = intString(neqns);
            vsize_str = intString(nvars);
            Error::addMessage(
                Error::UNDERDET_EQN_SYSTEM.clone(),
                list![esize_str.clone(), vsize_str.clone()],
            )?;
            BackendDAEUtil::checkAdjacencyMatrixSolvability(
                isyst.clone(),
                ishared.functionTree.clone(),
                BackendDAEUtil::isInitializationDAE(&ishared),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut esize_str: ArcStr;
            let mut vsize_str: ArcStr;
            let true = (intLt(nvars, neqns)) else {
                return Err("pattern mismatch");
            };
            esize_str = intString(neqns);
            vsize_str = intString(nvars);
            Error::addMessage(
                Error::OVERDET_EQN_SYSTEM.clone(),
                list![esize_str.clone(), vsize_str.clone()],
            )?;
            BackendDAEUtil::checkAdjacencyMatrixSolvability(
                isyst.clone(),
                ishared.functionTree.clone(),
                BackendDAEUtil::isInitializationDAE(&ishared),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- Causalize.singularSystemCheck failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSyst)
}

//protected import BackendDAETransform;
fn singularSystemCheck1(
    mut nVars: i32,
    mut nEqns: i32,
    mut iSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut eqnConstr: BackendDAE::EquationConstraints,
    mut matchingAlgorithm: &(
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
        ArcStr,
    ),
    mut arg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem> = iSyst.clone();
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matchingFunc: BackendDAEFunc::matchingAlgorithmFunc;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut indexType: BackendDAE::IndexType;
    let mut scalar: bool;
    let mut processed: bool;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(iSyst.clone()) {
        Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), mapping: Some((__pa2, __pa3, __pa4, __pa5, __pa6)), .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    mT = metamodelica::Own::own(__pa1);
    mapEqnIncRow = metamodelica::Own::own(__pa2);
    mapIncRowEqn = metamodelica::Own::own(__pa3);
    indexType = metamodelica::Own::own(__pa4);
    scalar = metamodelica::Own::own(__pa5);
    processed = metamodelica::Own::own(__pa6);
    (matchingFunc, _) = matchingAlgorithm.clone();
    m = AdjacencyMatrix::absAdjacencyMatrix(m.clone())?;
    mT = AdjacencyMatrix::absAdjacencyMatrix(mT.clone())?;
    syst = BackendDAEUtil::setEqSystMatrices(
        iSyst,
        Some(m.clone()),
        Some(mT.clone()),
        Some((
            mapEqnIncRow.clone(),
            mapIncRowEqn.clone(),
            openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE,
            scalar,
            processed,
        )),
    );
    assign_field!(syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING());
    let (__pa9, __pa7, __pa8) = ::match_deref::match_deref! { match &(matchingFunc(syst, iShared, true, (openmodelica_backend_types::BackendDAE::IndexReduction::INDEX_REDUCTION, eqnConstr), (std::sync::Arc::new(foundSingularSystem) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::List<i32>>, i32, metamodelica::Ref<BackendDAE::EqSystem>, metamodelica::Ref<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::List<i32>, i32, metamodelica::Ref<BackendDAE::EqSystem>, metamodelica::Ref<BackendDAE::Shared>, metamodelica::Array<i32>, metamodelica::Array<i32>, (BackendDAE::StateOrder, metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>, i32))> + 'static>), arg)?) {
        (__pa9 @ Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa7, ass2: __pa8, .. }, .. }, _, _) => (__pa9.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ass1 = metamodelica::Own::own(__pa7);
    ass2 = metamodelica::Own::own(__pa8);
    syst = metamodelica::Own::own(__pa9);
    assign_field!(
        outSyst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ass1.clone(),
            ass2: ass2.clone(),
            comps: metamodelica::nil()
        })
    );
    (_, ass1, ass2) = BackendVariable::traverseBackendDAEVars(
        outSyst.orderedVars.clone(),
        (std::sync::Arc::new(freeStateAssignments)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
                    )> + 'static,
            >),
        (1, ass1.clone(), ass2.clone()),
    )?;
    Ok(outSyst)
}

fn freeStateAssignments(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (i32, metamodelica::Array<i32>, metamodelica::Array<i32>),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    (outVar, outTpl) = (::match_deref::match_deref! { match &((inVar, inTpl)) {
        (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, (index, ass1, ass2)) => {
            let mut e: i32;
            let mut ass1 = (*ass1).clone();
            let mut ass2 = (*ass2).clone();
            e = ({let __elt = (*metamodelica::index_checked(&ass1.borrow(), index.clone())?).clone(); __elt});
            ass1 = metamodelica::arrayUpdate(ass1.clone(), index.clone(), -1)?;
            ass2 = metamodelica::arrayUpdate(ass2.clone(), e, -1)?;
            (var.clone(), (index.clone() + 1, ass1.clone(), ass2.clone()))
        },
        (var, (index, ass1, ass2)) => {
            (var.clone(), (index.clone() + 1, ass1.clone(), ass2.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outTpl))
}

fn foundSingularSystem(
    mut eqns: metamodelica::List<metamodelica::List<i32>>,
    mut actualEqn: i32,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
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
    let mut actualEqn: i32 = actualEqn;
    let mut isyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst;
    let mut ishared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut inAssignments1: metamodelica::Array<i32> = inAssignments1;
    let mut inAssignments2: metamodelica::Array<i32> = inAssignments2;
    let mut inArg: (
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ) = inArg;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut n: i32;
    let mut unmatched: metamodelica::List<i32>;
    let mut unmatched1: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut info: SourceInfo;
    let mut eqn_str: ArcStr;
    let mut var_str: ArcStr;
    if !((eqns).is_empty()) {
        (_, _, _, mapIncRowEqn, _) = inArg.clone();
        n = BackendDAEUtil::systemSize(&isyst)?;
        unmatched = List::flatten(eqns)?;
        unmatched1 = List::map1r(unmatched.clone(), &arrayGet, mapIncRowEqn.clone())?;
        unmatched1 = List::uniqueIntN(&unmatched1, metamodelica::arrayLength(mapIncRowEqn.clone()))?;
        eqn_str = BackendDump::dumpMarkedEqns(
            &isyst,
            List::sort(
                unmatched1.clone(),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
        )?;
        vars = Matching::getUnassigned(n, inAssignments2.clone(), metamodelica::nil())?;
        vars = List::fold1(&unmatched, &getAssignedVars, inAssignments1.clone(), vars)?;
        var_str = BackendDump::dumpMarkedVars(
            &isyst,
            List::sort(
                vars,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
        )?;
        source = BackendEquation::markedEquationSource(&isyst, (unmatched1).head().cloned()?)?;
        info = ElementSource::getElementSourceFileInfo(source);
        Error::addSourceMessage(
            &(if (BackendDAEUtil::isInitializationDAE(&ishared)) {
                Error::STRUCTURAL_SINGULAR_INITIAL_SYSTEM.clone()
            } else {
                Error::STRUCT_SINGULAR_SYSTEM.clone()
            }),
            list![eqn_str, var_str],
            &info,
        )?;
        return Err("fail");
    }
    Ok((
        changedEqns,
        actualEqn,
        isyst,
        ishared,
        inAssignments1,
        inAssignments2,
        inArg,
    ))
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
