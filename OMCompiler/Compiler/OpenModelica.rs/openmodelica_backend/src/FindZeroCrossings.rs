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

use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::SynchronousFeatures;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub type ZCArgType = (
    (
        BackendDAE::ZeroCrossingSet,
        BackendDAE::ZeroCrossingSet,
        BackendDAE::ZeroCrossingSet,
        i32,
    ),
    (i32, BackendDAE::Variables, BackendDAE::Variables),
    Option<metamodelica::List<BackendDAE::SimIterator>>,
);

pub type ForArgType = (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::ZeroCrossingSet,
        BackendDAE::ZeroCrossingSet,
        BackendDAE::ZeroCrossingSet,
        i32,
    ),
    (i32, BackendDAE::Variables, BackendDAE::Variables),
);

// =============================================================================
// section for preOptModule >>encapsulateWhenConditions<<
//
// This module encapsulates each when-condition in a boolean-variable
// $whenConditionsN and generates to each of these variables an equation
// $whenConditions = whenConditions
// =============================================================================
pub(crate) fn encapsulateWhenConditions(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut index: i32;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut vars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqns: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars_: BackendDAE::Variables;
    let mut eqns_: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut removedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    ht = HashTableExpToIndex::emptyHashTable();
    (systs, index, ht) = List::mapFold2(&systs, &encapsulateWhenConditions_EqSystem, 1, ht)?;
    (removedEqs, vars, eqns, index, _) = BackendEquation::traverseEquationArray(
        shared.removedEqs.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
            DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
            DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
            i32,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
        )| encapsulateWhenConditions_Equation(__a0, &__a1),
        (
            BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(shared.removedEqs.clone())),
            DoubleEnded::fromList(&(metamodelica::nil()))?,
            DoubleEnded::fromList(&(metamodelica::nil()))?,
            index,
            ht,
        ),
    )?;
    assign_field!(shared.removedEqs = removedEqs);
    eqns_ = BackendEquation::listEquation(&(DoubleEnded::toListNoCopyNoClear(eqns)))?;
    vars_ = BackendVariable::listVar(DoubleEnded::toListNoCopyNoClear(vars))?;
    syst = BackendDAEUtil::createEqSystem(
        vars_,
        eqns_,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION,
        BackendEquation::emptyEqns(),
    );
    systs = List::appendElt(syst, systs);
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs,
        shared: shared,
    });
    if index > 1 {
        outDAE = SynchronousFeatures::contPartitioning(&outDAE)?;
    }
    if Flags::isSet(Flags::DUMP_ENCAPSULATECONDITIONS.clone())? {
        BackendDump::dumpBackendDAE(
            &outDAE,
            &(literal!("DAE after PreOptModule >>encapsulateWhenConditions<<")),
        )?;
    }
    Ok(outDAE)
}

fn encapsulateWhenConditions_EqSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inIndex: i32,
    mut inHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    i32,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outIndex: i32 = 0;
    let mut outHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    outEqSystem = (::match_deref::match_deref! { match &(inEqSystem) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, .. } => {
            let mut removedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut varLst: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqnLst: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>;
            let mut syst = (*syst).clone();
            let mut orderedEqs = (*orderedEqs).clone();
            (orderedEqs, varLst, eqnLst, outIndex, outHT) = BackendEquation::traverseEquationArray(orderedEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>, DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>, i32, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))| encapsulateWhenConditions_Equation(__a0, &__a1), (BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(orderedEqs.clone())), DoubleEnded::fromList(&(metamodelica::nil()))?, DoubleEnded::fromList(&(metamodelica::nil()))?, inIndex, inHT))?;
            (removedEqs, varLst, eqnLst, outIndex, outHT) = BackendEquation::traverseEquationArray(syst.removedEqs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>, DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>, i32, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))| encapsulateWhenConditions_Equation(__a0, &__a1), (BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(syst.removedEqs.clone())), varLst, eqnLst, outIndex, outHT))?;
            assign_field!(
                syst.removedEqs = removedEqs,
                syst.orderedVars = BackendVariable::addVars(&(DoubleEnded::toListNoCopyNoClear(varLst)), orderedVars.clone())?,
                syst.orderedEqs = BackendEquation::addList(&(DoubleEnded::toListNoCopyNoClear(eqnLst)), orderedEqs.clone())?
            );
            BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEqSystem, outIndex, outHT))
}

fn encapsulateWhenConditions_Equation(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
    );
    (outEq, outTpl) = (::match_deref::match_deref! { match &((inEq.clone(), inTpl.clone())) {
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation, source, attr }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut whenEquation = (*whenEquation).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let mut ht = (*ht).clone();
            (whenEquation, vars1, eqns1, index, ht) = encapsulateWhenConditions_Equations(metamodelica::AsArg::as_arg(&whenEquation), metamodelica::AsArg::as_arg(&source), index.clone(), &(ht.clone()))?;
            DoubleEnded::push_list_back(vars.clone(), &vars1)?;
            DoubleEnded::push_list_back(eqns.clone(), &eqns1)?;
            eqn = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEquation.clone(), source: source.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            (eqn, (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { size: 0, alg: alg_, source, expand: crefExpand, attr }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqn2: metamodelica::Ref<BackendDAE::Equation>;
            let mut size: i32;
            let mut sizePre: i32;
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut allPreStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut allStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut alg_ = (*alg_).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let __arc1 = alg_.clone();
            let DAE::ALGORITHM_STMTS { statementLst: __pa0 } = &*__arc1;
            stmts = metamodelica::Own::own(__pa0);
            size = -(index.clone());
            allPreStmts = metamodelica::nil();
            allStmts = metamodelica::nil();
            for mut stmt in &*stmts.clone() {
                (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(&(list![stmt.clone()]), vars.clone(), index.clone())?;
                allPreStmts = listAppend(preStmts, allPreStmts);
                allStmts = listAppend(stmts.clone(), allStmts);
            }
            stmts = allStmts.reverse();
            sizePre = ((allPreStmts).len() as i32);
            size = size + index.clone() - sizePre;
            alg_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts });
            eqn = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            if sizePre > 0 {
                alg_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: allPreStmts });
                eqn2 = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: sizePre, alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
                DoubleEnded::push_front(eqns.clone(), eqn2);
            }
            (eqn, (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { size, alg: alg_, source, expand: crefExpand, attr }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut size = (*size).clone();
            let mut alg_ = (*alg_).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let __arc1 = alg_.clone();
            let DAE::ALGORITHM_STMTS { statementLst: __pa0 } = &*__arc1;
            stmts = metamodelica::Own::own(__pa0);
            size = size.clone() - index.clone();
            (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(&stmts, vars.clone(), index.clone())?;
            size = size.clone() + index.clone();
            stmts = listAppend(preStmts, stmts);
            alg_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts });
            eqn = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            (eqn, (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (_, (equationArray, vars, eqns, index, ht)) => {
            let mut equationArray = (*equationArray).clone();
            equationArray = BackendEquation::add(inEq.clone(), equationArray.clone())?;
            (inEq, (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn encapsulateWhenConditions_Equations(
    mut inWhenEquation: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inIndex: i32,
    mut inHT: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::WhenEquation>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    i32,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outWhenEquation: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outIndex: i32;
    let mut outHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    (outWhenEquation, outVars, outEqns, outIndex, outHT) = (::match_deref::match_deref! { match inWhenEquation {
        Deref @ BackendDAE::WhenEquation { condition, whenStmtLst, elsewhenPart: None } => {
            let mut index: i32;
            let mut whenEquation: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut condition = (*condition).clone();
            (condition, vars, eqns, index, ht) = encapsulateWhenConditions_Equations1(condition.clone(), inSource.clone(), inIndex, inHT.clone())?;
            whenEquation = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: condition.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: None });
            (whenEquation, vars, eqns, index, ht)
        },
        Deref @ BackendDAE::WhenEquation { condition, whenStmtLst, elsewhenPart: Some(elsewhenPart) } => {
            let mut index: i32;
            let mut whenEquation: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut condition = (*condition).clone();
            let mut elsewhenPart = (*elsewhenPart).clone();
            (elsewhenPart, vars1, eqns1, index, ht) = encapsulateWhenConditions_Equations(metamodelica::AsArg::as_arg(&elsewhenPart), inSource, inIndex, inHT)?;
            (condition, vars, eqns, index, ht) = encapsulateWhenConditions_Equations1(condition.clone(), inSource.clone(), index, ht)?;
            whenEquation = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: condition.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: Some(elsewhenPart.clone()) });
            vars1 = listAppend(vars, vars1);
            eqns1 = listAppend(eqns, eqns1);
            (whenEquation, vars1, eqns1, index, ht)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Equations")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outWhenEquation, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_Equations1(
    mut inCondition: metamodelica::Ref<DAE::Exp>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inIndex: i32,
    mut inHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    i32,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outCondition: metamodelica::Ref<DAE::Exp>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outIndex: i32;
    let mut outHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    (outCondition, outVars, outEqns, outIndex, outHT) = (::match_deref::match_deref! { match &(&*inCondition) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex, inHT.clone())
        },
        _ if (Expression::isConst(inCondition.clone())?) => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex, inHT.clone())
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar, array } => {
            let mut index: i32;
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut array = (*array).clone();
            (array, vars, eqns, index, ht) = encapsulateWhenConditions_EquationsWithArrayConditions(metamodelica::AsArg::as_arg(&array), inSource, inIndex, inHT.clone())?;
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: array.clone() }), vars, eqns, index, ht)
        },
        _ if (BaseHashTable::hasKey(inCondition.clone(), &inHT)?) => {
            let mut localIndex: i32;
            let mut crStr: ArcStr;
            let mut condition: metamodelica::Ref<DAE::Exp>;
            localIndex = BaseHashTable::get(inCondition.clone(), &inHT)?;
            crStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(localIndex)); ArcStr::from(__mm_s) };
            condition = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr, identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() });
            (condition, metamodelica::nil(), metamodelica::nil(), inIndex, inHT.clone())
        },
        _ => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut crStr: ArcStr;
            let mut condition: metamodelica::Ref<DAE::Exp>;
            let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            ht = BaseHashTable::add((inCondition.clone(), inIndex), inHT.clone())?;
            crStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex)); ArcStr::from(__mm_s) };
            var = metamodelica::Ref::new(BackendDAE::Var { varName: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool().clone()), true)?, tearingSelectOption: None, hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })), comment: Some(metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: Some(ExpressionBasics::printExpStr(inCondition.clone())?) })), connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false });
            var = BackendVariable::setVarFixed(var, true)?;
            eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: inCondition.clone(), source: inSource, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            condition = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr, identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() });
            (condition, list![var], list![eqn], inIndex + 1, ht)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCondition, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_EquationsWithArrayConditions(
    mut inConditionList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inIndex: i32,
    mut inHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    i32,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outConditionList: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut outIndex: i32 = inIndex;
    let mut outHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = inHT;
    let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    for mut condition in &**inConditionList {
        let mut condition = condition.clone();
        (condition, vars1, eqns1, outIndex, outHT) =
            encapsulateWhenConditions_Equations1(condition, inSource.clone(), outIndex, outHT)?;
        outVars = List::append_reverse(&vars1, outVars);
        outEqns = List::append_reverse(&eqns1, outEqns);
        outConditionList = metamodelica::cons(condition, outConditionList);
    }
    outVars = outVars.reverse();
    outEqns = outEqns.reverse();
    outConditionList = outConditionList.reverse();
    Ok((outConditionList, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_Algorithms(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut vars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
    mut inIndex: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    i32,
)> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outPreStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outIndex: i32;
    (outStmts, outPreStmts, outIndex) = (::match_deref::match_deref! { match inStmts {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil(), inIndex)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: condition, statementLst: stmts1, elseWhen: None, source, .. }, tail: rest } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut stmts_: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut index: i32;
            let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut initialCall: bool;
            let mut condition = (*condition).clone();
            (condition, vars1, preStmts, index) = encapsulateWhenConditions_Algorithms1(condition.clone(), source.clone(), inIndex)?;
            (conditions, initialCall) = BackendDAEUtil::getConditionList(condition.clone())?;
            DoubleEnded::push_list_front(vars.clone(), &vars1)?;
            if ((CheckModel::algorithmStatementListOutputs(metamodelica::AsArg::as_arg(&stmts1), openmodelica_frontend_types::DAE::Expand::EXPAND)?)).is_empty() {
                (stmts, preStmts2, index) = encapsulateWhenConditions_Algorithms(rest, vars, index)?;
                preStmts = listAppend(preStmts, preStmts2);
                stmts = metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions, initialCall: initialCall, statementLst: stmts1.clone(), elseWhen: None, source: source.clone() }), stmts);
            } else {
                (stmts, stmts_, index) = encapsulateWhenConditions_Algorithms(rest, vars, index)?;
                stmts_ = metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions, initialCall: initialCall, statementLst: stmts1.clone(), elseWhen: None, source: source.clone() }), stmts_);
                stmts = listAppend(stmts_, stmts);
            }
            (stmts, preStmts, index)
        },
        Deref @ metamodelica::ListNode::Cons { head: stmt @ Deref @ DAE::Statement::STMT_WHEN { exp: condition, statementLst: stmts1, elseWhen: Some(elseWhen), source, .. }, tail: rest } => {
            let mut stmt2: metamodelica::Ref<DAE::Statement>;
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut stmts_: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut elseWhenList: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut index: i32;
            let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut initialCall: bool;
            let mut condition = (*condition).clone();
            let mut elseWhen = (*elseWhen).clone();
            (condition, vars1, preStmts, index) = encapsulateWhenConditions_Algorithms1(condition.clone(), source.clone(), inIndex)?;
            (conditions, initialCall) = BackendDAEUtil::getConditionList(condition.clone())?;
            DoubleEnded::push_list_front(vars.clone(), &vars1)?;
            (elseWhenList, preStmts2, index) = encapsulateWhenConditions_Algorithms(&(list![elseWhen.clone()]), vars.clone(), index)?;
            if (elseWhenList).is_empty() {
                (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest, vars, inIndex)?;
                stmts_ = metamodelica::cons(stmt.clone(), listAppend(preStmts.clone(), stmts));
            } else {
                elseWhen = List::last(&elseWhenList)?;
                stmt2 = metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions, initialCall: initialCall, statementLst: stmts1.clone(), elseWhen: Some(elseWhen.clone()), source: source.clone() });
                if ((CheckModel::algorithmStatementListOutputs(&(list![stmt2.clone()]), openmodelica_frontend_types::DAE::Expand::EXPAND)?)).is_empty() {
                    preStmts2 = List::stripLast(elseWhenList)?;
                    preStmts = listAppend(preStmts, preStmts2);
                    (stmts, preStmts2, index) = encapsulateWhenConditions_Algorithms(rest, vars, index)?;
                    preStmts = listAppend(preStmts, preStmts2);
                    stmts_ = metamodelica::cons(stmt2, stmts);
                } else if ((elseWhenList).len() as i32) == 1 {
                    preStmts = listAppend(preStmts, preStmts2);
                    (stmts, stmts_, index) = encapsulateWhenConditions_Algorithms(rest, vars, index)?;
                    stmts_ = metamodelica::cons(stmt2, listAppend(stmts_, stmts));
                } else {
                    (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest, vars, inIndex)?;
                    stmts_ = listAppend(preStmts.clone(), stmts);
                }
            }
            (stmts_, preStmts, index)
        },
        Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut preStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut index: i32;
            (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest, vars, inIndex)?;
            stmts = listAppend(preStmts, stmts);
            (metamodelica::cons(stmt.clone(), stmts), metamodelica::nil(), index)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStmts, outPreStmts, outIndex))
}

fn encapsulateWhenConditions_Algorithms1(
    mut inCondition: metamodelica::Ref<DAE::Exp>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inIndex: i32,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    i32,
)> {
    let mut outCondition: metamodelica::Ref<DAE::Exp>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outIndex: i32;
    (outCondition, outVars, outStmts, outIndex) = (::match_deref::match_deref! { match &(&*inCondition) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex)
        },
        _ if (Expression::isConst(inCondition.clone())?) => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex)
        },
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: condition, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut stmt: metamodelica::Ref<DAE::Statement>;
            let mut crStr: ArcStr;
            let mut condition = (*condition).clone();
            crStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex)); ArcStr::from(__mm_s) };
            var = metamodelica::Ref::new(BackendDAE::Var { varName: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool().clone()), true)?, tearingSelectOption: None, hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })), comment: Some(metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: Some(ExpressionBasics::printExpStr(inCondition.clone())?) })), connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false });
            var = BackendVariable::setVarFixed(var, true)?;
            stmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_BOOL_DEFAULT().clone(), exp1: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() }), exp: condition.clone(), source: inSource });
            condition = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr, identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() });
            (condition.clone(), list![var], list![stmt], inIndex + 1)
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar, array } => {
            let mut index: i32;
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut array = (*array).clone();
            (array, vars, stmts, index) = encapsulateWhenConditions_AlgorithmsWithArrayConditions(metamodelica::AsArg::as_arg(&array), inSource, inIndex)?;
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: array.clone() }), vars, stmts, index)
        },
        _ => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut stmt: metamodelica::Ref<DAE::Statement>;
            let mut crStr: ArcStr;
            let mut condition: metamodelica::Ref<DAE::Exp>;
            crStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex)); ArcStr::from(__mm_s) };
            var = metamodelica::Ref::new(BackendDAE::Var { varName: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool().clone()), true)?, tearingSelectOption: None, hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })), comment: Some(metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: Some(ExpressionBasics::printExpStr(inCondition.clone())?) })), connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false });
            var = BackendVariable::setVarFixed(var, true)?;
            stmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_BOOL_DEFAULT().clone(), exp1: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() }), exp: inCondition.clone(), source: inSource });
            condition = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: crStr, identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() });
            (condition, list![var], list![stmt], inIndex + 1)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms1")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms1")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCondition, outVars, outStmts, outIndex))
}

fn encapsulateWhenConditions_AlgorithmsWithArrayConditions(
    mut inConditionList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inIndex: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    i32,
)> {
    let mut outConditionList: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut outIndex: i32 = inIndex;
    let mut vars1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut stmt1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    for mut condition in &**inConditionList {
        let mut condition = condition.clone();
        (condition, vars1, stmt1, outIndex) =
            encapsulateWhenConditions_Algorithms1(condition, inSource.clone(), outIndex)?;
        outVars = List::append_reverse(&vars1, outVars);
        outStmts = List::append_reverse(&stmt1, outStmts);
        outConditionList = metamodelica::cons(condition, outConditionList);
    }
    outVars = outVars.reverse();
    outStmts = outStmts.reverse();
    outConditionList = outConditionList.reverse();
    Ok((outConditionList, outVars, outStmts, outIndex))
}

// =============================================================================
// section for zero crossings
//
// This section contains all the functions to find zero crossings inside
// BackendDAE.
// =============================================================================
pub fn findZeroCrossings(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &findZeroCrossings1)?;
    Ok(outDAE)
}

fn findZeroCrossings1(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem> = inSyst.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let __arc3 = inSyst.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        matching: __pa2,
        ..
    } = &*__arc3;
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    matching = metamodelica::Own::own(__pa2);
    (outSyst, outShared) = (match BackendDAEUtil::getSubClock(&inSyst, &inShared)? {
        Some(BackendDAE::SubClock::SUBCLOCK { solver: mut solver, .. })
            if (!metamodelica::stringEq(&(BackendDump::optionString(solver.clone())), &(literal!("External")))) =>
        {
            (inSyst, inShared)
        }
        _ => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut eqns1: metamodelica::Ref<
                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
            >;
            let mut einfo: BackendDAE::EventInfo;
            let mut eqs_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqs_lst1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut timeEvents: metamodelica::List<BackendDAE::TimeEvent>;
            let mut zero_crossings: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relations: BackendDAE::ZeroCrossingSet;
            let mut countMathFunctions: i32;
            let __arc2 = inShared.clone();
            let BackendDAE::SHARED {
                globalKnownVars: __pa0,
                eventInfo: __pa1,
                ..
            } = &*__arc2;
            globalKnownVars = metamodelica::Own::own(__pa0);
            einfo = metamodelica::Own::own(__pa1);
            let BackendDAE::EVENT_INFO {
                timeEvents: __pa3,
                zeroCrossings: __pa4,
                samples: __pa5,
                relations: __pa6,
                numberMathEvents: __pa7,
            } = einfo;
            timeEvents = metamodelica::Own::own(__pa3);
            zero_crossings = metamodelica::Own::own(__pa4);
            sampleLst = metamodelica::Own::own(__pa5);
            relations = metamodelica::Own::own(__pa6);
            countMathFunctions = metamodelica::Own::own(__pa7);
            eqs_lst = BackendEquation::equationList(eqns)?;
            (zero_crossings, eqs_lst1, countMathFunctions, relations, sampleLst) = findZeroCrossings2(
                &vars,
                &globalKnownVars,
                &eqs_lst,
                0,
                countMathFunctions,
                zero_crossings,
                relations,
                sampleLst,
                metamodelica::nil(),
            )?;
            eqs_lst1 = eqs_lst1.reverse();
            eqns1 = BackendEquation::listEquation(&eqs_lst1)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("findZeroCrossings1 number of relations: "));
                    __mm_s.push_str(&*intString(ZeroCrossings::count(&relations)));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("findZeroCrossings1 sample index: "));
                    __mm_s.push_str(&*intString(ZeroCrossings::length(&sampleLst)));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            if '__try8: {
                let (__pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(matching.clone()) {
                    Deref @ BackendDAE::Matching::MATCHING { comps: __pa9, ass1: __pa10, ass2: __pa11 } => (__pa9.clone(), __pa10.clone(), __pa11.clone()),
                    _ => break '__try8 Err::<_, _>("pattern mismatch"),
                } };
                comps = metamodelica::Own::own(__pa9);
                ass1 = metamodelica::Own::own(__pa10);
                ass2 = metamodelica::Own::own(__pa11);
                comps = findZeroCrossingsinJacobians(&comps, zero_crossings.clone(), relations.clone(), sampleLst.clone(), &vars, &globalKnownVars);
                assign_field!(
                    outSyst.orderedEqs = eqns1.clone(),
                    outSyst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() })
                );
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            einfo = BackendDAE::EventInfo {
                timeEvents: timeEvents,
                zeroCrossings: zero_crossings,
                relations: relations,
                samples: sampleLst,
                numberMathEvents: countMathFunctions,
            };
            (outSyst, BackendDAEUtil::setSharedEventInfo(inShared, einfo))
        }
    });
    Ok((outSyst, outShared))
}

fn findZeroCrossings2<'__b>(
    mut inVariables1: &'__b BackendDAE::Variables,
    mut globalKnownVars: &'__b BackendDAE::Variables,
    mut inEquationLst2: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqnCount: i32,
    mut inNumberOfMathFunctions: i32,
    mut inZeroCrossingLst: BackendDAE::ZeroCrossingSet,
    mut inRelationsLst: BackendDAE::ZeroCrossingSet,
    mut inSamplesLst: BackendDAE::ZeroCrossingSet,
    mut inEquationLstAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::ZeroCrossingSet,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    i32,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inEquationLst2 {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inZeroCrossingLst, inEquationLstAccum, inNumberOfMathFunctions, inRelationsLst, inSamplesLst))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source: source_, expand, attr: eqAttr }, tail: xs } => {
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                eq_count = inEqnCount + 1;
                let (__pa0, (_, _, _, (__pa1, __pa2, __pa3, __pa4), _)) = traverseStmtsExps(metamodelica::AsArg::as_arg(&stmts), (metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("$$$") }), metamodelica::nil(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), (inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions), (eq_count, inVariables1.clone(), globalKnownVars.clone())), globalKnownVars)?;
                stmts_1 = metamodelica::Own::own(__pa0);
                res = metamodelica::Own::own(__pa1);
                relationsLst = metamodelica::Own::own(__pa2);
                sampleLst = metamodelica::Own::own(__pa3);
                countMathFunctions = metamodelica::Own::own(__pa4);
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts_1 }), source: source_.clone(), expand: expand.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: weqn, source: source_, attr: eqAttr }, tail: xs } => {
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut weqn = (*weqn).clone();
                eq_count = inEqnCount + 1;
                (weqn, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossingsWhenEqns(metamodelica::AsArg::as_arg(&weqn), &inZeroCrossingLst, &inRelationsLst, &inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1, globalKnownVars)?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weqn.clone(), source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source: source_, attr: eqAttr }, tail: xs } => {
                let mut zcs1: BackendDAE::ZeroCrossingSet;
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eres1: metamodelica::Ref<DAE::Exp>;
                let mut eres2: metamodelica::Ref<DAE::Exp>;
                eq_count = inEqnCount + 1;
                (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1, relationsLst, sampleLst, countMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: eres1, scalar: eres2, source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr: eqAttr }, tail: xs } => {
                let mut zcs1: BackendDAE::ZeroCrossingSet;
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eres1: metamodelica::Ref<DAE::Exp>;
                let mut eres2: metamodelica::Ref<DAE::Exp>;
                eq_count = inEqnCount + 1;
                (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1, relationsLst, sampleLst, countMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: eres1, right: eres2, source: source.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimsize, left: e1, right: e2, source, attr: eqAttr, recordSize }, tail: xs } => {
                let mut zcs1: BackendDAE::ZeroCrossingSet;
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eres1: metamodelica::Ref<DAE::Exp>;
                let mut eres2: metamodelica::Ref<DAE::Exp>;
                eq_count = inEqnCount + 1;
                (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1, relationsLst, sampleLst, countMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimsize.clone(), left: eres1, right: eres2, source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e1, source: source_, attr: eqAttr }, tail: xs } => {
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eres1: metamodelica::Ref<DAE::Exp>;
                (eres1, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions, inEqnCount, -1, inVariables1.clone(), globalKnownVars.clone())?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: eres1, source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, inEqnCount, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source: source_, attr: eqAttr }, tail: xs } => {
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eres1: metamodelica::Ref<DAE::Exp>;
                eq_count = inEqnCount + 1;
                (eres1, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst, inRelationsLst, inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1.clone(), globalKnownVars.clone())?;
                eqnsAccum = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: eres1, source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ BackendDAE::Equation::IF_EQUATION { .. }, tail: xs } => {
                let mut res: BackendDAE::ZeroCrossingSet;
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut e = (*e).clone();
                eq_count = inEqnCount + 1;
                (e, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossingsIfEqns(metamodelica::AsArg::as_arg(&e), &inZeroCrossingLst, &inRelationsLst, &inSamplesLst, inNumberOfMathFunctions, eq_count, -1, inVariables1, globalKnownVars)?;
                eqnsAccum = metamodelica::cons(e.clone(), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, countMathFunctions, res, relationsLst, sampleLst, eqnsAccum); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: xs } => {
                let mut res1: BackendDAE::ZeroCrossingSet;
                let mut sampleLst: BackendDAE::ZeroCrossingSet;
                let mut relationsLst: BackendDAE::ZeroCrossingSet;
                let mut eq_count: i32;
                let mut countMathFunctions: i32;
                let mut eq_reslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnsAccum: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                eq_count = inEqnCount + 1;
                eqnsAccum = metamodelica::cons(e.clone(), inEquationLstAccum);
                { (inVariables1, globalKnownVars, inEquationLst2, inEqnCount, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, inEquationLstAccum) = (inVariables1, globalKnownVars, xs, eq_count, inNumberOfMathFunctions, inZeroCrossingLst, inRelationsLst, inSamplesLst, eqnsAccum); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findZeroCrossingsWhenEqns(
    mut inWhenEqn: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut inZeroCrossings: &BackendDAE::ZeroCrossingSet,
    mut inrelationsinZC: &BackendDAE::ZeroCrossingSet,
    mut inSamplesLst: &BackendDAE::ZeroCrossingSet,
    mut incountMathFunctions: i32,
    mut counteq: i32,
    mut countwc: i32,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<BackendDAE::WhenEquation>,
    i32,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
)> {
    let mut oWhenEqn: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut outCountMathFunctions: i32;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: BackendDAE::ZeroCrossingSet;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    (
        oWhenEqn,
        outCountMathFunctions,
        outZeroCrossings,
        outrelationsinZC,
        outSamplesLst,
    ) = (match &**inWhenEqn {
        BackendDAE::WhenEquation {
            condition: cond,
            whenStmtLst,
            elsewhenPart: oweelse,
        } => {
            let mut we: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: BackendDAE::ZeroCrossingSet;
            let mut countMathFunctions: i32;
            let mut cond = (*cond).clone();
            let mut oweelse = (*oweelse).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                BackendDump::debugStrExpStr(
                    &(literal!("processed when condition: ")),
                    cond.clone(),
                    &(literal!("\n")),
                )?;
            }
            (cond, countMathFunctions, zc, relations, samples) = findZeroCrossings3(
                cond.clone(),
                inZeroCrossings.clone(),
                inrelationsinZC.clone(),
                inSamplesLst.clone(),
                incountMathFunctions,
                counteq,
                countwc,
                vars.clone(),
                globalKnownVars.clone(),
            )?;
            if (oweelse).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oweelse.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                we = metamodelica::Own::own(__pa0);
                (we, countMathFunctions, zc, relations, samples) = findZeroCrossingsWhenEqns(
                    &we,
                    &zc,
                    &relations,
                    &samples,
                    countMathFunctions,
                    counteq,
                    countwc,
                    vars,
                    globalKnownVars,
                )?;
                oweelse = Some(we);
            } else {
                oweelse = None;
            }
            (
                metamodelica::Ref::new(BackendDAE::WhenEquation {
                    condition: cond.clone(),
                    whenStmtLst: whenStmtLst.clone(),
                    elsewhenPart: oweelse.clone(),
                }),
                countMathFunctions,
                zc,
                relations,
                samples,
            )
        }
    });
    Ok((
        oWhenEqn,
        outCountMathFunctions,
        outZeroCrossings,
        outrelationsinZC,
        outSamplesLst,
    ))
}

fn findZeroCrossingsIfEqns(
    mut inIfEqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut inZeroCrossings: &BackendDAE::ZeroCrossingSet,
    mut inrelationsinZC: &BackendDAE::ZeroCrossingSet,
    mut inSamplesLst: &BackendDAE::ZeroCrossingSet,
    mut incountMathFunctions: i32,
    mut counteq: i32,
    mut countwc: i32,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    i32,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
)> {
    let mut outIfEqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut outCountMathFunctions: i32;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: BackendDAE::ZeroCrossingSet;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    (
        outIfEqn,
        outCountMathFunctions,
        outZeroCrossings,
        outrelationsinZC,
        outSamplesLst,
    ) = (::match_deref::match_deref! { match inIfEqn {
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: Deref @ metamodelica::ListNode::Nil, eqnstrue: Deref @ metamodelica::ListNode::Nil, eqnsfalse: elseeqns, source: source_, attr: eqAttr } => {
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: BackendDAE::ZeroCrossingSet;
            let mut countMathFunctions: i32;
            let mut elseeqns = (*elseeqns).clone();
            (zc, elseeqns, countMathFunctions, relations, samples) = findZeroCrossings2(vars, globalKnownVars, metamodelica::AsArg::as_arg(&elseeqns), counteq, incountMathFunctions, inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), metamodelica::nil())?;
            elseeqns = elseeqns.clone().reverse();
            (metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: metamodelica::nil(), eqnstrue: metamodelica::nil(), eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() }), countMathFunctions, zc, relations, samples)
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: Deref @ metamodelica::ListNode::Cons { head: condition, tail: restconditions }, eqnstrue: Deref @ metamodelica::ListNode::Cons { head: eqnstrue, tail: resteqns }, eqnsfalse: elseeqns, source: source_, attr: eqAttr } => {
            let mut conditions: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ifeqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqnsTrueLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: BackendDAE::ZeroCrossingSet;
            let mut countMathFunctions: i32;
            let mut condition = (*condition).clone();
            let mut eqnstrue = (*eqnstrue).clone();
            let mut elseeqns = (*elseeqns).clone();
            let mut source_ = (*source_).clone();
            (condition, countMathFunctions, zc, relations, samples) = findZeroCrossings3(condition.clone(), inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), incountMathFunctions, counteq, countwc, vars.clone(), globalKnownVars.clone())?;
            (zc, eqnstrue, countMathFunctions, relations, samples) = findZeroCrossings2(vars, globalKnownVars, metamodelica::AsArg::as_arg(&eqnstrue), counteq, countMathFunctions, zc, relations, samples, metamodelica::nil())?;
            eqnstrue = eqnstrue.clone().reverse();
            ifeqn = metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: restconditions.clone(), eqnstrue: resteqns.clone(), eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() });
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(findZeroCrossingsIfEqns(&ifeqn, &zc, &relations, &samples, countMathFunctions, counteq, countwc, vars, globalKnownVars)?) {
                (Deref @ BackendDAE::Equation::IF_EQUATION { conditions: __pa0, eqnstrue: __pa1, eqnsfalse: __pa2, source: __pa3, .. }, __pa4, __pa5, __pa6, __pa7) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            conditions = metamodelica::Own::own(__pa0);
            eqnsTrueLst = metamodelica::Own::own(__pa1);
            elseeqns = metamodelica::Own::own(__pa2);
            source_ = metamodelica::Own::own(__pa3);
            countMathFunctions = metamodelica::Own::own(__pa4);
            zc = metamodelica::Own::own(__pa5);
            relations = metamodelica::Own::own(__pa6);
            samples = metamodelica::Own::own(__pa7);
            conditions = metamodelica::cons(condition.clone(), conditions);
            eqnsTrueLst = metamodelica::cons(eqnstrue.clone(), eqnsTrueLst);
            (metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions, eqnstrue: eqnsTrueLst, eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() }), countMathFunctions, zc, relations, samples)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((
        outIfEqn,
        outCountMathFunctions,
        outZeroCrossings,
        outrelationsinZC,
        outSamplesLst,
    ))
}

fn findZeroCrossingsinJacobians(
    mut inStrongComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut zeroCrossingLst: BackendDAE::ZeroCrossingSet,
    mut relationsLst: BackendDAE::ZeroCrossingSet,
    mut samplesLst: BackendDAE::ZeroCrossingSet,
    mut allVariables: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> {
    let mut strongComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
    let mut outComponent: metamodelica::Ref<BackendDAE::StrongComponent>;
    for mut component in &**inStrongComponents {
        outComponent = 'mc: {
            let __mc_input = component.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fullJacobian }, .. } => {
                        let mut comp = (*comp).clone();
                        let mut fullJacobian = (*fullJacobian).clone();
                        fullJacobian = replaceZCExpinFullJacobian(fullJacobian.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables, globalKnownVars)?;
                        assign_variant_field!(comp => BackendDAE::StrongComponent::EQUATIONSYSTEM; jac = metamodelica::Ref::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fullJacobian.clone() }));
                        Ok(comp.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: jacobian @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(symJacobian), sparsePattern, coloring, nonlinearPattern }, .. } => {
                        let mut comp = (*comp).clone();
                        let mut symJacobian = (*symJacobian).clone();
                        symJacobian = replaceZCExpinSymJacobian(&(symJacobian.clone()), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables, globalKnownVars)?;
                        assign_variant_field!(comp => BackendDAE::StrongComponent::EQUATIONSYSTEM; jac = metamodelica::Ref::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(symJacobian.clone()), sparsePattern: sparsePattern.clone(), coloring: coloring.clone(), nonlinearPattern: nonlinearPattern.clone() }));
                        Ok(comp.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    comp @ Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: tearingSet @ BackendDAE::TearingSet { jac: jacobian @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(symJacobian), sparsePattern, coloring, nonlinearPattern }, .. }, .. } => {
                        let mut comp = (*comp).clone();
                        let mut tearingSet = (*tearingSet).clone();
                        let mut symJacobian = (*symJacobian).clone();
                        symJacobian = replaceZCExpinSymJacobian(&(symJacobian.clone()), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables, globalKnownVars)?;
                        tearingSet.jac = metamodelica::Ref::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(symJacobian.clone()), sparsePattern: sparsePattern.clone(), coloring: coloring.clone(), nonlinearPattern: nonlinearPattern.clone() });
                        assign_variant_field!(comp => BackendDAE::StrongComponent::TORNSYSTEM; strictTearingSet = tearingSet.clone());
                        Ok(comp.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(component.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        strongComponents = metamodelica::cons(outComponent, strongComponents);
    }
    strongComponents = strongComponents.reverse();
    strongComponents
}

fn replaceZCExpinFullJacobian(
    mut fullJac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>,
    mut zeroCrossingLst: BackendDAE::ZeroCrossingSet,
    mut relationsLst: BackendDAE::ZeroCrossingSet,
    mut samplesLst: BackendDAE::ZeroCrossingSet,
    mut allVariables: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>> {
    let mut outFullJac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>;
    let mut jac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut outJac: metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)> = metamodelica::nil();
    let mut i: i32;
    let mut j: i32;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut element: (i32, i32, metamodelica::Ref<BackendDAE::Equation>) =
        (0, 0, metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION));
    jac = fullJac.ok_or("pattern mismatch")?;
    for mut element in &*jac {
        let mut element = element.clone();
        (i, j, eqn) = element;
        let __pa0 = ::match_deref::match_deref! { match &(findZeroCrossings2(allVariables, globalKnownVars, &(list![eqn]), 0, 0, zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), metamodelica::nil())?) {
            (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _, _, _) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        eqn = metamodelica::Own::own(__pa0);
        outJac = metamodelica::cons((i, j, eqn), outJac);
    }
    outJac = outJac.reverse();
    outFullJac = Some(outJac);
    Ok(outFullJac)
}

fn replaceZCExpinSymJacobian(
    mut symJac: &(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
    mut zeroCrossingLst: BackendDAE::ZeroCrossingSet,
    mut relationsLst: BackendDAE::ZeroCrossingSet,
    mut samplesLst: BackendDAE::ZeroCrossingSet,
    mut allVariables: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    ArcStr,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outSymJac: (
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    let mut jacBDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut name: ArcStr;
    let mut seedVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tmpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut resultVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut depCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (jacBDAE, name, seedVars, tmpVars, resultVars, depCrefs) = symJac.clone();
    jacBDAE = replaceZeroCrossingsJacBackend(
        &jacBDAE,
        zeroCrossingLst,
        relationsLst,
        samplesLst,
        allVariables,
        globalKnownVars,
    )?;
    outSymJac = (jacBDAE, name, seedVars, tmpVars, resultVars, depCrefs);
    Ok(outSymJac)
}

fn replaceZeroCrossingsJacBackend(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut zeroCrossingLst: BackendDAE::ZeroCrossingSet,
    mut relationsLst: BackendDAE::ZeroCrossingSet,
    mut samplesLst: BackendDAE::ZeroCrossingSet,
    mut allVariables: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut outEqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut eqs_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    for mut system in &*eqs {
        let mut system = system.clone();
        eqs_lst = BackendEquation::equationList(system.orderedEqs.clone())?;
        (_, eqs_lst, _, _, _) = findZeroCrossings2(
            allVariables,
            globalKnownVars,
            &eqs_lst,
            0,
            0,
            zeroCrossingLst.clone(),
            relationsLst.clone(),
            samplesLst.clone(),
            metamodelica::nil(),
        )?;
        eqns = BackendEquation::listEquation(&(eqs_lst.reverse()))?;
        assign_field!(system.orderedEqs = eqns);
        let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(system.matching.clone()) {
            Deref @ BackendDAE::Matching::MATCHING { comps: __pa3, ass1: __pa4, ass2: __pa5 } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        comps = metamodelica::Own::own(__pa3);
        ass1 = metamodelica::Own::own(__pa4);
        ass2 = metamodelica::Own::own(__pa5);
        comps = findZeroCrossingsinJacobians(
            &comps,
            zeroCrossingLst.clone(),
            relationsLst.clone(),
            samplesLst.clone(),
            allVariables,
            globalKnownVars,
        );
        matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            comps: comps,
            ass1: ass1.clone(),
            ass2: ass2.clone(),
        });
        assign_field!(system.matching = matching);
        outEqs = metamodelica::cons(system, outEqs);
    }
    outEqs = outEqs.reverse();
    outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: outEqs,
        shared: shared,
    });
    Ok(outBackendDAE)
}

fn findZeroCrossings3(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut inZeroCrossings: BackendDAE::ZeroCrossingSet,
    mut inrelationsinZC: BackendDAE::ZeroCrossingSet,
    mut inSamplesLst: BackendDAE::ZeroCrossingSet,
    mut incountMathFunctions: i32,
    mut counteq: i32,
    mut countwc: i32,
    mut vars: BackendDAE::Variables,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    i32,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
    BackendDAE::ZeroCrossingSet,
)> {
    let mut eres: metamodelica::Ref<DAE::Exp>;
    let mut outCountMathFunctions: i32;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: BackendDAE::ZeroCrossingSet;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    if Flags::isSet(Flags::RELIDX.clone())? {
        BackendDump::debugStrExpStr(&(literal!("start: ")), e.clone(), &(literal!("\n")))?;
    }
    let (__pa0, ((__pa1, __pa2, __pa3, __pa4), _, _)) = Expression::traverseExpTopDown(
        e,
        &collectZC,
        (
            (inZeroCrossings, inrelationsinZC, inSamplesLst, incountMathFunctions),
            (counteq, vars, globalKnownVars),
            None,
        ),
    )?;
    eres = metamodelica::Own::own(__pa0);
    outZeroCrossings = metamodelica::Own::own(__pa1);
    outrelationsinZC = metamodelica::Own::own(__pa2);
    outSamplesLst = metamodelica::Own::own(__pa3);
    outCountMathFunctions = metamodelica::Own::own(__pa4);
    Ok((
        eres,
        outCountMathFunctions,
        outZeroCrossings,
        outrelationsinZC,
        outSamplesLst,
    ))
}

fn operatorEventsSupported() -> Result<bool> {
    let mut supported: bool;
    let mut target: ArcStr = Config::simCodeTarget()?;
    supported = metamodelica::stringEq(&target, &(literal!("C")))
        || metamodelica::stringEq(&target, &(literal!("wasm-jit")))
        || metamodelica::stringEq(&target, &(literal!("wasm")));
    Ok(supported)
}

fn collectZC(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: ZCArgType,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, ZCArgType)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: ZCArgType;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone(), operatorEventsSupported()?)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, ((_, _, samples, _), (eq_count, _, _), iters), _) => {
            let mut zc: BackendDAE::ZeroCrossing;
            zc = createZeroCrossing(inExp.clone(), list![eq_count.clone()], iters.clone());
            mergeZeroCrossings(zc, metamodelica::AsArg::as_arg(&samples))?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sample index: ")); __mm_s.push_str(&*intString(ZeroCrossings::length(metamodelica::AsArg::as_arg(&samples)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (inExp, true, inTpl)
        },
        (__esc_outExp @ Deref @ DAE::Exp::REDUCTION { .. }, ((zeroCrossings, relations, samples, numMathFunctions), tp1, _), _) => {
            outExp = (*__esc_outExp).clone();
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut iters: Option<metamodelica::List<BackendDAE::SimIterator>>;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC searching in: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            iters = Some(({
        let mut __acc: metamodelica::List<BackendDAE::SimIterator> = metamodelica::nil();
        for mut iter in (var_field!((*outExp).iterators, DAE::Exp::REDUCTION).clone()).into_iter().cloned() {
            let __x = createIterator(&(iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            let (__pa0, ((__pa1, __pa2, __pa3, __pa4), __pa5, _)) = Expression::traverseExpTopDown(var_field!((*outExp).expr, DAE::Exp::REDUCTION).clone(), &collectZC, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters))?;
            e = metamodelica::Own::own(__pa0);
            zeroCrossings = metamodelica::Own::own(__pa1);
            relations = metamodelica::Own::own(__pa2);
            samples = metamodelica::Own::own(__pa3);
            numMathFunctions = metamodelica::Own::own(__pa4);
            tp1 = metamodelica::Own::own(__pa5);
            assign_variant_field!(outExp => DAE::Exp::REDUCTION; expr = e);
            (outExp.clone(), false, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), None))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: index, tail: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: delay, tail: Deref @ metamodelica::ListNode::Cons { head: delayMax, tail: Deref @ metamodelica::ListNode::Nil } } } }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), true) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut eres1: metamodelica::Ref<DAE::Exp>;
            let mut itmp: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut e = (*e).clone();
            let mut delay = (*delay).clone();
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut iters = (*iters).clone();
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(e.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e = metamodelica::Own::own(__pa0);
            relations = metamodelica::Own::own(__pa1);
            samples = metamodelica::Own::own(__pa2);
            numMathFunctions = metamodelica::Own::own(__pa3);
            tp1 = metamodelica::Own::own(__pa4);
            iters = metamodelica::Own::own(__pa5);
            let (__pa6, ((_, __pa7, __pa8, __pa9), __pa10, __pa11)) = Expression::traverseExpTopDown(delay.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            delay = metamodelica::Own::own(__pa6);
            relations = metamodelica::Own::own(__pa7);
            samples = metamodelica::Own::own(__pa8);
            numMathFunctions = metamodelica::Own::own(__pa9);
            tp1 = metamodelica::Own::own(__pa10);
            iters = metamodelica::Own::own(__pa11);
            eres1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delayZeroCrossing") }), expLst: list![index.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)) }), delay.clone()], attr: attr.clone() });
            e_1 = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: eres1.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), optionExpisASUB: None });
            zc = createZeroCrossing(eres1, list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndex(e_1, relations.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa12, __pa13) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?) {
                (Deref @ DAE::Exp::RELATION { index: __pa12, .. }, __pa13, _) => (__pa12.clone(), __pa13.clone()),
                _ => return Err("pattern mismatch"),
            } };
            itmp = metamodelica::Own::own(__pa12);
            zeroCrossings = metamodelica::Own::own(__pa13);
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres)?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delay") }), expLst: list![index.clone(), e.clone(), delay.clone(), delayMax.clone()], attr: attr.clone() }), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: Deref @ metamodelica::ListNode::Cons { head: index, tail: Deref @ metamodelica::ListNode::Cons { head: in0, tail: Deref @ metamodelica::ListNode::Cons { head: in1, tail: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: dir, tail: Deref @ metamodelica::ListNode::Cons { head: initPnts, tail: Deref @ metamodelica::ListNode::Cons { head: initVals, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), true) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut eres1: metamodelica::Ref<DAE::Exp>;
            let mut itmp: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut in0 = (*in0).clone();
            let mut in1 = (*in1).clone();
            let mut x = (*x).clone();
            let mut dir = (*dir).clone();
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut eq_count = (*eq_count).clone();
            let mut iters = (*iters).clone();
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(in0.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            in0 = metamodelica::Own::own(__pa0);
            relations = metamodelica::Own::own(__pa1);
            samples = metamodelica::Own::own(__pa2);
            numMathFunctions = metamodelica::Own::own(__pa3);
            tp1 = metamodelica::Own::own(__pa4);
            iters = metamodelica::Own::own(__pa5);
            let (__pa6, ((_, __pa7, __pa8, __pa9), __pa10, __pa11)) = Expression::traverseExpTopDown(in1.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            in1 = metamodelica::Own::own(__pa6);
            relations = metamodelica::Own::own(__pa7);
            samples = metamodelica::Own::own(__pa8);
            numMathFunctions = metamodelica::Own::own(__pa9);
            tp1 = metamodelica::Own::own(__pa10);
            iters = metamodelica::Own::own(__pa11);
            let (__pa12, ((_, __pa13, __pa14, __pa15), __pa16, __pa17)) = Expression::traverseExpTopDown(x.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            x = metamodelica::Own::own(__pa12);
            relations = metamodelica::Own::own(__pa13);
            samples = metamodelica::Own::own(__pa14);
            numMathFunctions = metamodelica::Own::own(__pa15);
            tp1 = metamodelica::Own::own(__pa16);
            iters = metamodelica::Own::own(__pa17);
            let (__pa18, ((_, __pa19, __pa20, __pa21), ref __pa23 @ (ref __pa22, _, _), __pa24)) = Expression::traverseExpTopDown(dir.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            dir = metamodelica::Own::own(__pa18);
            relations = metamodelica::Own::own(__pa19);
            samples = metamodelica::Own::own(__pa20);
            numMathFunctions = metamodelica::Own::own(__pa21);
            eq_count = metamodelica::Own::own(__pa22);
            tp1 = metamodelica::Own::own(__pa23);
            iters = metamodelica::Own::own(__pa24);
            eres1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistributionZeroCrossing") }), expLst: list![index.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)) }), x.clone(), dir.clone()], attr: attr.clone() });
            e_1 = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: eres1.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), optionExpisASUB: None });
            zc = createZeroCrossing(eres1, list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndex(e_1, relations.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa25, __pa26) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?) {
                (Deref @ DAE::Exp::RELATION { index: __pa25, .. }, __pa26, _) => (__pa25.clone(), __pa26.clone()),
                _ => return Err("pattern mismatch"),
            } };
            itmp = metamodelica::Own::own(__pa25);
            zeroCrossings = metamodelica::Own::own(__pa26);
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres)?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistribution") }), expLst: list![index.clone(), in0.clone(), in1.clone(), x.clone(), dir.clone(), initPnts.clone(), initVals.clone()], attr: attr.clone() }), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete LUNARY: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (inExp, true, inTpl)
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete LBINARY: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (inExp, true, inTpl)
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, ((zeroCrossings, relations, _, _), _, iters), _) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eq_count: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ZCArgType;
            let mut empty: bool;
            let mut e1 = (*e1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            let (__pa0, ref __pa2 @ (_, (ref __pa1, _, _), _)) = Expression::traverseExpTopDown(e1.clone(), &collectZC, inTpl.clone())?;
            e1 = metamodelica::Own::own(__pa0);
            eq_count = metamodelica::Own::own(__pa1);
            tpl = metamodelica::Own::own(__pa2);
            e_1 = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count], iters.clone());
            empty = !(ZeroCrossings::contains(metamodelica::AsArg::as_arg(&zeroCrossings), zc.clone())?);
            if empty {
                ZeroCrossings::add(metamodelica::AsArg::as_arg(&zeroCrossings), zc)?;
            }
            if Flags::isSet(Flags::RELIDX.clone())? {
                BackendDump::debugExpStr(e_1.clone(), &(literal!("\n")))?;
            }
            (e_1, false, if (empty) {tpl} else {inTpl})
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, ((zeroCrossings, relations, samples, numMathFunctions), tp1, iters), _) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut eq_count: i32;
            let mut oldNumRelations: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut empty: bool;
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut iters = (*iters).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LBINARY: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations))))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                BackendDump::debugExpStr(inExp.clone(), &(literal!("\n")))?;
            }
            oldNumRelations = ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations));
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(e1.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e_1 = metamodelica::Own::own(__pa0);
            relations = metamodelica::Own::own(__pa1);
            samples = metamodelica::Own::own(__pa2);
            numMathFunctions = metamodelica::Own::own(__pa3);
            tp1 = metamodelica::Own::own(__pa4);
            iters = metamodelica::Own::own(__pa5);
            let (__pa6, ((_, __pa7, __pa8, __pa9), ref __pa11 @ (ref __pa10, _, _), __pa12)) = Expression::traverseExpTopDown(e2.clone(), &collectZC, ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e_2 = metamodelica::Own::own(__pa6);
            relations = metamodelica::Own::own(__pa7);
            samples = metamodelica::Own::own(__pa8);
            numMathFunctions = metamodelica::Own::own(__pa9);
            eq_count = metamodelica::Own::own(__pa10);
            tp1 = metamodelica::Own::own(__pa11);
            iters = metamodelica::Own::own(__pa12);
            if intGt(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), oldNumRelations) {
                e_1 = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e_1, operator: op.clone(), exp2: e_2 });
                zc = createZeroCrossing(e_1.clone(), list![eq_count], iters.clone());
                empty = !(ZeroCrossings::contains(metamodelica::AsArg::as_arg(&zeroCrossings), zc.clone())?);
                cont = false;
                if empty {
                    ZeroCrossings::add(metamodelica::AsArg::as_arg(&zeroCrossings), zc)?;
                }
                if Flags::isSet(Flags::RELIDX.clone())? {
                    BackendDump::dumpZeroCrossingList(&(ZeroCrossings::toList(metamodelica::AsArg::as_arg(&zeroCrossings))), &(literal!("LBINARY")))?;
                }
            } else {
                empty = true;
                cont = true;
            }
            (if (cont) {inExp} else {e_1}, cont, if (!(cont) && empty) {((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone())} else {inTpl})
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete RELATION: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (inExp, true, inTpl)
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut itmp: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC (2): ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numRelations: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), optionExpisASUB: None });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndex(e_1, relations.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), zc)?) {
                (Deref @ DAE::Exp::RELATION { index: __pa0, .. }, __pa1, _) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            itmp = metamodelica::Own::own(__pa0);
            zeroCrossings = metamodelica::Own::own(__pa1);
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("integer") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("floor") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ceil") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr: attr @ Deref @ DAE::CallAttributes { .. } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("mod") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            numMathFunctions = numMathFunctions.clone() + 1;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr: attr @ Deref @ DAE::CallAttributes { ty, .. } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            e_2 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: eres.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }) });
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (e_2, true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        _ => {
            (inExp, true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn collectZCAlgsFor(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: ForArgType,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, ForArgType)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: ForArgType;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _) => {
            (inExp.clone(), false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _) => {
            (inExp.clone(), false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, (_, _, _, (_, _, samples, _), (alg_indx, _, _))) => {
            let mut eqs: metamodelica::List<i32>;
            let mut zc: BackendDAE::ZeroCrossing;
            eqs = list![alg_indx.clone()];
            zc = createZeroCrossing(inExp.clone(), eqs, None);
            ZeroCrossings::add(metamodelica::AsArg::as_arg(&samples), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sample index algotihm: ")); __mm_s.push_str(&*intString(alg_indx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (inExp.clone(), true, inTpl)
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl)
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, (iterator, _, Deref @ DAE::Exp::RANGE { .. }, (zeroCrossings, relations, _, _), _)) if (Expression::expContains(&inExp, metamodelica::AsArg::as_arg(&iterator))?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut zc_lst: metamodelica::List<BackendDAE::ZeroCrossing>;
            let mut alg_indx: i32;
            let mut tpl: ForArgType;
            let mut e1 = (*e1).clone();
            let mut iterator = (*iterator).clone();
            let mut relations = (*relations).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY with Iterator: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            let (__pa0, ref __pa5 @ (ref __pa1, ref __pa2, _, (_, ref __pa3, _, _), (ref __pa4, _, _))) = Expression::traverseExpTopDown(e1.clone(), &collectZCAlgsFor, inTpl)?;
            e1 = metamodelica::Own::own(__pa0);
            iterator = metamodelica::Own::own(__pa1);
            inExpLst = metamodelica::Own::own(__pa2);
            relations = metamodelica::Own::own(__pa3);
            alg_indx = metamodelica::Own::own(__pa4);
            tpl = metamodelica::Own::own(__pa5);
            e_1 = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            (explst, _) = replaceIteratorWithStaticValues(&e_1, metamodelica::AsArg::as_arg(&iterator), &inExpLst, ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))?;
            zc_lst = createZeroCrossings(explst, list![alg_indx])?;
            ZeroCrossings::add_list(metamodelica::AsArg::as_arg(&zeroCrossings), &zc_lst)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print(literal!("collectZCAlgsFor LUNARY with Iterator result zc: "));
                BackendDump::debugExpStr(e_1.clone(), &(literal!("\n")))?;
            }
            (e_1, false, tpl)
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, (_, _, _, (zeroCrossings, relations, _, _), _)) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut alg_indx: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ForArgType;
            let mut e1 = (*e1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            let (__pa0, ref __pa2 @ (_, _, _, _, (ref __pa1, _, _))) = Expression::traverseExpTopDown(e1.clone(), &collectZCAlgsFor, inTpl)?;
            e1 = metamodelica::Own::own(__pa0);
            alg_indx = metamodelica::Own::own(__pa1);
            tpl = metamodelica::Own::own(__pa2);
            e_1 = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx], None);
            ZeroCrossings::add(metamodelica::AsArg::as_arg(&zeroCrossings), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print(literal!("collectZCAlgsFor LUNARY result zc: "));
                BackendDump::debugExpStr(e_1.clone(), &(literal!("\n")))?;
            }
            (e_1, false, tpl)
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl)
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, (iterator, inExpLst, range, (zeroCrossings, relations, samples, numMathFunctions), tp1)) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut zc_lst: metamodelica::List<BackendDAE::ZeroCrossing>;
            let mut alg_indx: i32;
            let mut oldNumRelations: i32;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ForArgType;
            let mut tp2: (BackendDAE::ZeroCrossingSet, BackendDAE::ZeroCrossingSet, BackendDAE::ZeroCrossingSet, i32);
            let mut inExpLst = (*inExpLst).clone();
            let mut range = (*range).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LBINARY: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                BackendDump::debugExpStr(inExp.clone(), &(literal!("\n")))?;
            }
            oldNumRelations = ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations));
            let (__pa0, (_, __pa1, __pa2, __pa3, __pa4)) = Expression::traverseExpTopDown(e1.clone(), &collectZCAlgsFor, (iterator.clone(), inExpLst.clone(), range.clone(), (ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))?;
            e_1 = metamodelica::Own::own(__pa0);
            inExpLst = metamodelica::Own::own(__pa1);
            range = metamodelica::Own::own(__pa2);
            tp2 = metamodelica::Own::own(__pa3);
            tp1 = metamodelica::Own::own(__pa4);
            let (__pa5, (_, __pa6, __pa7, (_, __pa8, __pa9, __pa10), ref __pa12 @ (ref __pa11, _, _))) = Expression::traverseExpTopDown(e2.clone(), &collectZCAlgsFor, (iterator.clone(), inExpLst.clone(), range.clone(), tp2, tp1.clone()))?;
            e_2 = metamodelica::Own::own(__pa5);
            inExpLst = metamodelica::Own::own(__pa6);
            range = metamodelica::Own::own(__pa7);
            relations = metamodelica::Own::own(__pa8);
            samples = metamodelica::Own::own(__pa9);
            numMathFunctions = metamodelica::Own::own(__pa10);
            alg_indx = metamodelica::Own::own(__pa11);
            tp1 = metamodelica::Own::own(__pa12);
            if intGt(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), oldNumRelations) {
                e_1 = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e_1, operator: op.clone(), exp2: e_2 });
                if Expression::expContains(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&iterator))? || Expression::expContains(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&iterator))? {
                    (explst, _) = replaceIteratorWithStaticValues(&e_1, metamodelica::AsArg::as_arg(&iterator), metamodelica::AsArg::as_arg(&inExpLst), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))?;
                    zc_lst = createZeroCrossings(explst, list![alg_indx])?;
                    ZeroCrossings::add_list(metamodelica::AsArg::as_arg(&zeroCrossings), &zc_lst)?;
                    if Flags::isSet(Flags::RELIDX.clone())? {
                        BackendDump::dumpZeroCrossingList(&(ZeroCrossings::toList(metamodelica::AsArg::as_arg(&zeroCrossings))), &(literal!("collectZCAlgsFor LBINARY1 result zc")))?;
                    }
                } else {
                    zc = createZeroCrossing(e_1.clone(), list![alg_indx], None);
                    if !(ZeroCrossings::contains(metamodelica::AsArg::as_arg(&zeroCrossings), zc.clone())?) {
                        ZeroCrossings::add(metamodelica::AsArg::as_arg(&zeroCrossings), zc)?;
                    }
                    if Flags::isSet(Flags::RELIDX.clone())? {
                        BackendDump::dumpZeroCrossingList(&(ZeroCrossings::toList(metamodelica::AsArg::as_arg(&zeroCrossings))), &(literal!("collectZCAlgsFor LBINARY2 result zc")))?;
                    }
                }
                cont = false;
                tpl = (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone());
            } else {
                e_1 = inExp.clone();
                cont = true;
                tpl = inTpl;
            }
            (e_1, cont, tpl)
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl)
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, (iterator, inExpLst, range @ Deref @ DAE::Exp::RANGE { start: startvalue, step: stepvalueopt, .. }, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, globalKnownVars))) if (if (Flags::isSet(Flags::EVENTS.clone())?) {if (Expression::expContains(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&iterator))?) {true} else {Expression::expContains(metamodelica::AsArg::as_arg(&e2), metamodelica::AsArg::as_arg(&iterator))?}} else {false}) => {
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut zcLstNew: metamodelica::List<BackendDAE::ZeroCrossing>;
            let mut itmp: i32;
            let mut stepvalue: metamodelica::Ref<DAE::Exp>;
            let mut istart: i32;
            let mut istep: i32;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of relations: ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            stepvalue = stepvalueopt.clone().unwrap_or(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }));
            istart = BackendDAEUtil::expInt(startvalue.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?;
            istep = BackendDAEUtil::expInt(stepvalue, metamodelica::AsArg::as_arg(&globalKnownVars))?;
            eres = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), optionExpisASUB: Some((iterator.clone(), istart, istep)) });
            (explst, itmp) = replaceIteratorWithStaticValues(&inExp, metamodelica::AsArg::as_arg(&iterator), metamodelica::AsArg::as_arg(&inExpLst), ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of new zc (1): ")); __mm_s.push_str(&*intString(((explst).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            zcLstNew = createZeroCrossings(explst, list![alg_indx.clone()])?;
            ZeroCrossings::push_list(metamodelica::AsArg::as_arg(&relations), &zcLstNew)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of new zc (2): ")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            itmp = ((zcLstNew).len() as i32);
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" itmp: ")); __mm_s.push_str(&*intString(itmp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            ZeroCrossings::add_list(metamodelica::AsArg::as_arg(&zeroCrossings), &zcLstNew)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZCAlgsFor result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index:")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, (iterator, inExpLst, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            eres = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)), optionExpisASUB: None });
            zc = createZeroCrossing(eres.clone(), list![alg_indx.clone()], None);
            ZeroCrossings::push(metamodelica::AsArg::as_arg(&relations), zc.clone())?;
            ZeroCrossings::add(metamodelica::AsArg::as_arg(&zeroCrossings), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZCAlgsFor result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index:")); __mm_s.push_str(&*intString(ZeroCrossings::count(metamodelica::AsArg::as_arg(&relations)))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("integer") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("floor") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ceil") }), expLst: list![e1.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr: attr @ Deref @ DAE::CallAttributes { .. } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("mod") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (eres, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, attr: attr @ Deref @ DAE::CallAttributes { ty, .. } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut e_2: metamodelica::Ref<DAE::Exp>;
            let mut eres: metamodelica::Ref<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            e_1 = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("div") }), expLst: list![e1.clone(), e2.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1, zeroCrossings.clone(), numMathFunctions.clone(), zc)?;
            e_2 = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: eres.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }) });
            if Flags::isSet(Flags::RELIDX.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (e_2, true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn replaceIteratorWithStaticValues(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inIterator: &metamodelica::Ref<DAE::Exp>,
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inIndex: i32,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, i32)> {
    let mut outZeroCrossings: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outIndex: i32;
    (outZeroCrossings, outIndex) = (::match_deref::match_deref! { match (inExp, inExpLst) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            (metamodelica::nil(), inIndex)
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut index: i32;
            e_1 = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: inIndex, optionExpisASUB: None });
            (res1, _) = Expression::replaceExpTpl(e_1, (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp, inIterator, rest, inIndex + 1)?;
            res2 = metamodelica::cons(res1, res2);
            (res2, index)
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut index: i32;
            e_1 = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            (res1, _) = Expression::replaceExpTpl(e_1, (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp, inIterator, rest, inIndex + 1)?;
            res2 = metamodelica::cons(res1, res2);
            (res2, index)
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }) => {
            let mut res1: metamodelica::Ref<DAE::Exp>;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut index: i32;
            e_1 = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            (res1, _) = Expression::replaceExpTpl(e_1, (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp, inIterator, rest, inIndex + 1)?;
            res2 = metamodelica::cons(res1, res2);
            (res2, index)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.replaceIteratorWithStaticValues")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outZeroCrossings, outIndex))
}

fn zcIndex(
    mut relation: metamodelica::Ref<DAE::Exp>,
    mut zeroCrossings: BackendDAE::ZeroCrossingSet,
    mut index: i32,
    mut zc: BackendDAE::ZeroCrossing,
) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::ZeroCrossingSet, i32)> {
    let mut relation: metamodelica::Ref<DAE::Exp> = relation;
    let mut zeroCrossings: BackendDAE::ZeroCrossingSet = zeroCrossings;
    let mut index: i32 = index;
    if ZeroCrossings::contains(&zeroCrossings, zc.clone())? {
        let BackendDAE::ZERO_CROSSING { relation_: __pa0, .. } = ZeroCrossings::get(&zeroCrossings, zc)?;
        relation = metamodelica::Own::own(__pa0);
        return Ok((relation, zeroCrossings, index));
    }
    (relation, index) = (::match_deref::match_deref! { match &(relation.clone()) {
        Deref @ DAE::Exp::RELATION { .. } => {
            ZeroCrossings::add(&zeroCrossings, zc)?;
            (relation, index + 1)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            ZeroCrossings::add(&zeroCrossings, zc)?;
            (relation, index + 1)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. } => {
            ZeroCrossings::add(&zeroCrossings, zc)?;
            (relation, index + 2)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.zcIndex")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(relation)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((relation, zeroCrossings, index))
}

fn mergeZeroCrossings(mut newZc: BackendDAE::ZeroCrossing, mut zcs: &BackendDAE::ZeroCrossingSet) -> Result<()> {
    if !(ZeroCrossings::contains(zcs, newZc.clone())?) {
        ZeroCrossings::add(zcs, newZc)?;
    } else {
        DoubleEnded::mapNoCopy_1(zcs.zc.clone(), &mergeZeroCrossingIfEqual, newZc)?;
    }
    Ok(())
}

fn mergeZeroCrossingIfEqual(
    mut zc1: BackendDAE::ZeroCrossing,
    mut zc2: BackendDAE::ZeroCrossing,
) -> Result<BackendDAE::ZeroCrossing> {
    let mut zc: BackendDAE::ZeroCrossing;
    zc = if (ZeroCrossings::equals(&zc1, &zc2)?) {
        mergeZeroCrossing(zc1, zc2)?
    } else {
        zc1
    };
    Ok(zc)
}

fn mergeZeroCrossing(
    mut inZeroCrossing1: BackendDAE::ZeroCrossing,
    mut inZeroCrossing2: BackendDAE::ZeroCrossing,
) -> Result<BackendDAE::ZeroCrossing> {
    let mut outZeroCrossing: BackendDAE::ZeroCrossing;
    let mut eq: metamodelica::List<i32>;
    let mut eq1: metamodelica::List<i32>;
    let mut eq2: metamodelica::List<i32>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    let BackendDAE::ZERO_CROSSING {
        relation_: __pa0,
        occurEquLst: __pa1,
        ..
    } = inZeroCrossing1;
    e1 = metamodelica::Own::own(__pa0);
    eq1 = metamodelica::Own::own(__pa1);
    let BackendDAE::ZERO_CROSSING {
        relation_: __pa2,
        occurEquLst: __pa3,
        ..
    } = inZeroCrossing2;
    e2 = metamodelica::Own::own(__pa2);
    eq2 = metamodelica::Own::own(__pa3);
    res = getMinZeroCrossings(&e1, &e2)?;
    eq = List::union(&eq1, &eq2);
    outZeroCrossing = BackendDAE::ZeroCrossing {
        index: 0,
        relation_: res,
        occurEquLst: eq,
        iter: None,
    };
    Ok(outZeroCrossing)
}

fn getMinZeroCrossings(
    mut inZCexp1: &metamodelica::Ref<DAE::Exp>,
    mut inZCexp2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outMinZC: metamodelica::Ref<DAE::Exp>;
    outMinZC = (::match_deref::match_deref! { match (inZCexp1, inZCexp2) {
        (Deref @ DAE::Exp::RELATION { index: index1, .. }, Deref @ DAE::Exp::RELATION { index: index2, .. }) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = if (index1.clone() < index2.clone()) {inZCexp1.clone()} else {inZCexp2.clone()};
            res
        },
        (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, Deref @ DAE::Exp::LUNARY { exp: e2, .. }) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = getMinZeroCrossings(e1, e2)?;
            metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: res })
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, Deref @ DAE::Exp::LBINARY { exp1: e3, exp2: e4, .. }) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            let mut res2: metamodelica::Ref<DAE::Exp>;
            res = getMinZeroCrossings(e1, e2)?;
            res2 = getMinZeroCrossings(e3, e4)?;
            metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: res, operator: op.clone(), exp2: res2 })
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, _) => {
            inZCexp1.clone()
        },
        (_, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }) => {
            inZCexp2.clone()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.getMinZeroCrossings")); __mm_s.push_str(&*literal!(" failed for {")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inZCexp1.clone())?); __mm_s.push_str(&*literal!("} and {")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inZCexp2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMinZC)
}

fn traverseStmtsExps(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inExtraArg: ForArgType,
    mut inKnvars: &BackendDAE::Variables,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, ForArgType)> {
    let mut slist: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut extraArg: ForArgType = inExtraArg;
    let mut e_1: metamodelica::Ref<DAE::Exp>;
    let mut e_2: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut iteratorExp: metamodelica::Ref<DAE::Exp>;
    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut iteratorexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut x: metamodelica::Ref<DAE::Statement>;
    let mut ew: metamodelica::Ref<DAE::Statement>;
    let mut ew_1: metamodelica::Ref<DAE::Statement>;
    let mut b1: bool;
    let mut id1: ArcStr;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut algElse: metamodelica::Ref<DAE::Else>;
    let mut loopPrlVars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    let mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut initialCall: bool;
    let mut sub_iters: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    )>;
    for mut stmt in &**inStmts {
        let mut stmt = stmt.clone();
        (stmt, extraArg) = (::match_deref::match_deref! { match &(stmt.clone()) {
            Deref @ DAE::Statement::STMT_ASSIGN { type_: __esc_tp, exp1: __esc_e2, exp: __esc_e, source: __esc_source } => {
                tp = (*__esc_tp).clone();
                e2 = (*__esc_e2).clone();
                e = (*__esc_e).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (e_2, extraArg) = Expression::traverseExpTopDown(e2.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1, source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: __esc_tp, expExpLst: __esc_expl1, exp: __esc_e, source: __esc_source } => {
                tp = (*__esc_tp).clone();
                expl1 = (*__esc_expl1).clone();
                e = (*__esc_e).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (expl2, extraArg) = Expression::traverseExpListTopDown(expl1.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2, exp: e_1, source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: __esc_tp, lhs: __esc_e2, exp: __esc_e, source: __esc_source } => {
                tp = (*__esc_tp).clone();
                e2 = (*__esc_e2).clone();
                e = (*__esc_e).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (e_2, _, extraArg) = collectZCAlgsFor(e2.clone(), extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_2.clone(), exp: e_1, source: source.clone() }), extraArg)
            },
            __esc_x @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: __esc_tp, lhs: __esc_e2, exp: __esc_e, source: __esc_source } => {
                x = (*__esc_x).clone();
                tp = (*__esc_tp).clone();
                e2 = (*__esc_e2).clone();
                e = (*__esc_e).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                if '__try0: {
                    (e_2, _, _) = unwrap_break_err!(collectZCAlgsFor(e2.clone(), extraArg.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }.is_ok() { return Err("failure(): body succeeded") }
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                metamodelica::print(DAEDump::ppStatementStr(x.clone()));
                metamodelica::print(literal!("Warning, not allowed to set the componentRef to a expression in FindZeroCrossings.traverseStmtsExps for ZeroCrosssing\n"));
                return Err("fail")
            },
            Deref @ DAE::Statement::STMT_IF { exp: __esc_e, statementLst: __esc_stmts, else_: __esc_algElse, source: __esc_source } => {
                e = (*__esc_e).clone();
                stmts = (*__esc_stmts).clone();
                algElse = (*__esc_algElse).clone();
                source = (*__esc_source).clone();
                (algElse, extraArg) = traverseStmtsElseExps(metamodelica::AsArg::as_arg(&algElse), &extraArg, inKnvars)?;
                (stmts2, extraArg) = traverseStmtsExps(metamodelica::AsArg::as_arg(&stmts), extraArg, inKnvars)?;
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e_1, statementLst: stmts2, else_: algElse.clone(), source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_FOR { type_: __esc_tp, iterIsArray: __esc_b1, iter: __esc_id1, range: __esc_e, statementLst: __esc_stmts, source: __esc_source, sub_iters: __esc_sub_iters } => {
                tp = (*__esc_tp).clone();
                b1 = (*__esc_b1).clone();
                id1 = (*__esc_id1).clone();
                e = (*__esc_e).clone();
                stmts = (*__esc_stmts).clone();
                source = (*__esc_source).clone();
                sub_iters = (*__esc_sub_iters).clone();
                cr = ComponentReferenceBasics::makeCrefIdent(id1.clone(), tp.clone(), metamodelica::nil());
                iteratorExp = Expression::crefExp(cr)?;
                iteratorexps = BackendDAEUtil::extendRange(metamodelica::AsArg::as_arg(&e), inKnvars)?;
                (stmts2, extraArg) = traverseStmtsForExps(iteratorExp, iteratorexps, e.clone(), stmts.clone(), inKnvars, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e.clone(), statementLst: stmts2, source: source.clone(), sub_iters: sub_iters.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_PARFOR { type_: __esc_tp, iterIsArray: __esc_b1, iter: __esc_id1, range: __esc_e, statementLst: __esc_stmts, loopPrlVars: __esc_loopPrlVars, source: __esc_source } => {
                tp = (*__esc_tp).clone();
                b1 = (*__esc_b1).clone();
                id1 = (*__esc_id1).clone();
                e = (*__esc_e).clone();
                stmts = (*__esc_stmts).clone();
                loopPrlVars = (*__esc_loopPrlVars).clone();
                source = (*__esc_source).clone();
                cr = ComponentReferenceBasics::makeCrefIdent(id1.clone(), tp.clone(), metamodelica::nil());
                iteratorExp = Expression::crefExp(cr)?;
                iteratorexps = BackendDAEUtil::extendRange(metamodelica::AsArg::as_arg(&e), inKnvars)?;
                (stmts2, extraArg) = traverseStmtsForExps(iteratorExp, iteratorexps, e.clone(), stmts.clone(), inKnvars, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: id1.clone(), range: e.clone(), statementLst: stmts2, loopPrlVars: loopPrlVars.clone(), source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_WHILE { exp: __esc_e, statementLst: __esc_stmts, source: __esc_source } => {
                e = (*__esc_e).clone();
                stmts = (*__esc_stmts).clone();
                source = (*__esc_source).clone();
                (stmts2, extraArg) = traverseStmtsExps(metamodelica::AsArg::as_arg(&stmts), extraArg, inKnvars)?;
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e_1, statementLst: stmts2, source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_WHEN { exp: __esc_e, conditions: __esc_conditions, initialCall: __esc_initialCall, statementLst: __esc_stmts, elseWhen: None, source: __esc_source } => {
                e = (*__esc_e).clone();
                conditions = (*__esc_conditions).clone();
                initialCall = (*__esc_initialCall).clone();
                stmts = (*__esc_stmts).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1, conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: None, source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_WHEN { exp: __esc_e, conditions: __esc_conditions, initialCall: __esc_initialCall, statementLst: __esc_stmts, elseWhen: Some(__esc_ew), source: __esc_source } => {
                e = (*__esc_e).clone();
                conditions = (*__esc_conditions).clone();
                initialCall = (*__esc_initialCall).clone();
                stmts = (*__esc_stmts).clone();
                ew = (*__esc_ew).clone();
                source = (*__esc_source).clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseStmtsExps(&(list![ew.clone()]), extraArg, inKnvars)?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                ew_1 = metamodelica::Own::own(__pa0);
                extraArg = metamodelica::Own::own(__pa1);
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1, conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: Some(ew_1), source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_ASSERT { .. } => (stmt, extraArg),
            Deref @ DAE::Statement::STMT_TERMINATE { .. } => (stmt, extraArg),
            Deref @ DAE::Statement::STMT_REINIT { .. } => (stmt, extraArg),
            Deref @ DAE::Statement::STMT_NORETCALL { exp: __esc_e, source: __esc_source } => {
                e = (*__esc_e).clone();
                source = (*__esc_source).clone();
                (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1, source: source.clone() }), extraArg)
            },
            Deref @ DAE::Statement::STMT_RETURN { .. } => (stmt, extraArg),
            Deref @ DAE::Statement::STMT_BREAK { .. } => (stmt, extraArg),
            Deref @ DAE::Statement::STMT_FAILURE { body: __esc_stmts, source: __esc_source } => {
                stmts = (*__esc_stmts).clone();
                source = (*__esc_source).clone();
                (stmts2, extraArg) = traverseStmtsExps(metamodelica::AsArg::as_arg(&stmts), extraArg, inKnvars)?;
                (metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: stmts2, source: source.clone() }), extraArg)
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.traverseStmtsExps")); __mm_s.push_str(&*literal!(" failed: ")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        slist = metamodelica::cons(stmt, slist);
    }
    slist = metamodelica::Dangerous::listReverseInPlace(slist);
    Ok((slist, extraArg))
}

fn traverseStmtsElseExps(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut inExtraArg: &ForArgType,
    mut inKnvars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Else>, ForArgType)> {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut outTplStmtTypeA: ForArgType;
    (outElse, outTplStmtTypeA) = (match &**inElse {
        DAE::Else::NOELSE { .. } => (
            openmodelica_frontend_types::DAE::Else::interned_NOELSE(),
            inExtraArg.clone(),
        ),
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: st,
            else_: el,
        } => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut el_1: metamodelica::Ref<DAE::Else>;
            let mut extraArg: ForArgType;
            (el_1, extraArg) = traverseStmtsElseExps(el, inExtraArg, inKnvars)?;
            (st_1, extraArg) = traverseStmtsExps(st, extraArg, inKnvars)?;
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), &collectZCAlgsFor, extraArg)?;
            (
                metamodelica::Ref::new(DAE::Else::ELSEIF {
                    exp: e_1,
                    statementLst: st_1,
                    else_: el_1,
                }),
                extraArg,
            )
        }
        DAE::Else::ELSE { statementLst: st } => {
            let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut extraArg: ForArgType;
            (st_1, extraArg) = traverseStmtsExps(st, inExtraArg.clone(), inKnvars)?;
            (metamodelica::Ref::new(DAE::Else::ELSE { statementLst: st_1 }), extraArg)
        }
    });
    Ok((outElse, outTplStmtTypeA))
}

fn traverseStmtsForExps(
    mut inIteratorExp: metamodelica::Ref<DAE::Exp>,
    mut inExplst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inRange: metamodelica::Ref<DAE::Exp>,
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inKnvars: &BackendDAE::Variables,
    mut inExtraArg: ForArgType,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Statement>>, ForArgType)> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut outTpl: ForArgType;
    (outStatements, outTpl) = (::match_deref::match_deref! { match &((inExplst.clone(), inExtraArg.clone())) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (inStmts, inExtraArg)
        },
        (_, (_, _, _, tpl2, tpl3)) => {
            let mut statementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut extraArg: ForArgType;
            (statementLst, extraArg) = traverseStmtsExps(&inStmts, (inIteratorExp, inExplst, inRange, tpl2.clone(), tpl3.clone()), inKnvars)?;
            (statementLst, extraArg)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.traverseStmtsForExps")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/FindZeroCrossings.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStatements, outTpl))
}

pub fn setOperatorZeroCrossingIndices(
    mut inZeroCrossings: &metamodelica::List<BackendDAE::ZeroCrossing>,
) -> Result<metamodelica::List<BackendDAE::ZeroCrossing>> {
    let mut outZeroCrossings: metamodelica::List<BackendDAE::ZeroCrossing> = metamodelica::nil();
    let mut relation_: metamodelica::Ref<DAE::Exp>;
    for mut zc in &**inZeroCrossings {
        let mut zc = zc.clone();
        (relation_, _) = Expression::traverseExpBottomUp(
            zc.relation_.clone(),
            &fnptr!(setOperatorIndex, metamodelica::Ref<DAE::Exp>, i32),
            zc.index.clone(),
        )?;
        zc.relation_ = relation_;
        outZeroCrossings = metamodelica::cons(zc, outZeroCrossings);
    }
    outZeroCrossings = outZeroCrossings.reverse();
    Ok(outZeroCrossings)
}

fn setOperatorIndex(mut inExp: metamodelica::Ref<DAE::Exp>, mut inIndex: i32) -> (metamodelica::Ref<DAE::Exp>, i32) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outIndex: i32 = inIndex;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delayZeroCrossing" }, expLst: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: delay, tail: Deref @ metamodelica::ListNode::Nil } } }, attr } => {
            metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delayZeroCrossing") }), expLst: list![expr.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: inIndex }), delay.clone()], attr: attr.clone() })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistributionZeroCrossing" }, expLst: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: dir, tail: Deref @ metamodelica::ListNode::Nil } } } }, attr } => {
            metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistributionZeroCrossing") }), expLst: list![expr.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: inIndex }), x.clone(), dir.clone()], attr: attr.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outIndex)
}

fn createZeroCrossings(
    mut inExpExpLst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inOccurEquLst: metamodelica::List<i32>,
) -> Result<metamodelica::List<BackendDAE::ZeroCrossing>> {
    let mut outZeroCrossingLst: metamodelica::List<BackendDAE::ZeroCrossing>;
    outZeroCrossingLst = List::map1(
        inExpExpLst1,
        &({
            let __pe_b2 = None;
            move |__pe_a0, __pe_a1| Ok(createZeroCrossing(__pe_a0, __pe_a1, __pe_b2.clone()))
        }),
        inOccurEquLst,
    )?;
    Ok(outZeroCrossingLst)
}

fn createZeroCrossing(
    mut inRelation: metamodelica::Ref<DAE::Exp>,
    mut inOccurEquLst: metamodelica::List<i32>,
    mut iters: Option<metamodelica::List<BackendDAE::SimIterator>>,
) -> BackendDAE::ZeroCrossing {
    let mut outZeroCrossing: BackendDAE::ZeroCrossing;
    outZeroCrossing = (::match_deref::match_deref! { match &(inOccurEquLst.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: (-1), tail: Deref @ metamodelica::ListNode::Nil } => BackendDAE::ZeroCrossing { index: 0, relation_: inRelation, occurEquLst: metamodelica::nil(), iter: iters },
        _ => BackendDAE::ZeroCrossing { index: 0, relation_: inRelation, occurEquLst: inOccurEquLst, iter: iters },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outZeroCrossing
}

fn createIterator(mut red_iter: &metamodelica::Ref<DAE::ReductionIterator>) -> Result<BackendDAE::SimIterator> {
    let mut iter: BackendDAE::SimIterator;
    iter = (::match_deref::match_deref! { match &(red_iter.exp.clone()) {
        exp @ Deref @ DAE::Exp::RANGE { .. } => {
            let mut step: metamodelica::Ref<DAE::Exp>;
            let mut size: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut non_resizable_size: i32;
            ty = Expression::r#typeof(var_field!((**exp).start, DAE::Exp::RANGE).clone())?;
            step = var_field!((**exp).step, DAE::Exp::RANGE).clone().unwrap_or(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }));
            size = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: var_field!((**exp).stop, DAE::Exp::RANGE).clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: var_field!((**exp).start, DAE::Exp::RANGE).clone() });
            size = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: size, operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: step.clone() });
            size = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: size, operator: DAE::Operator::ADD { ty: ty }, exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }) });
            (size, _) = ExpressionSimplify::simplify(size)?;
            match '__try0: {
                non_resizable_size = unwrap_break_err!(Expression::getEvaluatedConstInteger(&size), '__try0);
                Ok::<_, &'static str>((non_resizable_size.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    non_resizable_size = __try0_o0;
                }
                Err(_) => {
                    non_resizable_size = 0;
                }
            }
            BackendDAE::SimIterator::SIM_ITERATOR_RANGE { name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: red_iter.id.clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), start: var_field!((**exp).start, DAE::Exp::RANGE).clone(), step: step, stop: var_field!((**exp).stop, DAE::Exp::RANGE).clone(), size: size, non_resizable_size: non_resizable_size, sub_iter: metamodelica::nil() }
        },
        exp @ Deref @ DAE::Exp::ARRAY { .. } => {
            BackendDAE::SimIterator::SIM_ITERATOR_LIST { name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: red_iter.id.clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), lst: ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (var_field!((**exp).array, DAE::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = DAEUtil::getInteger(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), size: ((var_field!((**exp).array, DAE::Exp::ARRAY)).len() as i32), sub_iter: metamodelica::nil() }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.createIterator")); __mm_s.push_str(&*literal!(" failed for expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(red_iter.exp.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iter)
}
