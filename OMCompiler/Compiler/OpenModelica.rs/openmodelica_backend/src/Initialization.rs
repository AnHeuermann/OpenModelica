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

use crate::BackendDAECreate;
use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::IndexReduction;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use crate::SynchronousFeatures;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all public functions
//
// These are functions that can be used to access the initialization.
// =============================================================================
pub(crate) fn solveInitialSystem(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::BackendDAE>,
)> {
    let mut outInitDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut outInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>;
    let mut outRemovedInitialEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut outSimDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut initdae: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut initdae0: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut initsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut initsyst0: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut reeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut reeqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut initVars: BackendDAE::Variables;
    let mut vars: BackendDAE::Variables;
    let mut fixvars: BackendDAE::Variables;
    let mut useHomotopy: bool;
    let mut datarecon: bool = false;
    let mut enabledModules: metamodelica::List<ArcStr>;
    let mut disabledModules: metamodelica::List<ArcStr>;
    let mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut removedEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut dumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outAllPrimaryParameters: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut allPrimaryParameters: metamodelica::Ref<AvlSetCR::Tree>;
    match '__try0: {
        dae = unwrap_break_err!(inlineWhenForInitialization(inDAE.clone()), '__try0);
        unwrap_break_err!(execStat(&(literal!("inlineWhenForInitialization (initialization)"))), '__try0);
        (dae, initVars, outAllPrimaryParameters, outGlobalKnownVars) =
            unwrap_break_err!(selectInitializationVariablesDAE(dae.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpVarList(&outAllPrimaryParameters, &(literal!("selected all primary parameters"))), '__try0);
        }
        unwrap_break_err!(execStat(&(literal!("selectInitializationVariablesDAE (initialization)"))), '__try0);
        hs = unwrap_break_err!(collectPreVariables(&dae), '__try0);
        unwrap_break_err!(execStat(&(literal!("collectPreVariables (initialization)"))), '__try0);
        vars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        fixvars = unwrap_break_err!(BackendVariable::listVar(outAllPrimaryParameters.clone()), '__try0);
        eqns = BackendEquation::emptyEqnsSized(
            BackendVariable::varsSize(&dae.shared.aliasVars)
                + BackendVariable::varsSize(&dae.shared.globalKnownVars)
                + BackendVariable::varsSize(&dae.shared.localKnownVars)
                + BackendEquation::getNumberOfEquations(dae.shared.initialEqs.clone())
                + 2 * unwrap_break_err!(BackendDAEUtil::daeSize(&dae), '__try0),
        );
        reeqns = BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(dae.shared.removedEqs.clone()));
        allPrimaryParameters = openmodelica_frontend_dump::AvlSetCR::Tree::interned_EMPTY();
        for mut v in &*outAllPrimaryParameters {
            allPrimaryParameters = unwrap_break_err!(AvlSetCR::add(allPrimaryParameters.clone(), &(BackendVariable::varCref(metamodelica::AsArg::as_arg(&v)))), '__try0);
        }
        if (inDAE.shared.dataReconciliationData).is_some() {
            datarecon = true;
        }
        (vars, fixvars, eqns, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.aliasVars.clone(), (std::sync::Arc::new(fnptr!(introducePreVarsForAliasVariables, metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.globalKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), metamodelica::Ref<AvlSetCR::Tree>, bool)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), metamodelica::Ref<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon)), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.localKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), metamodelica::Ref<AvlSetCR::Tree>, bool)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), metamodelica::Ref<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon)), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.initialEqs.clone(), &collectInitialEqns, (eqns.clone(), reeqns.clone())), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.removedEqs.clone(), &collectInitialEqns, (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat(&(literal!("collectInitialEqns (initialization)"))), '__try0);
        (vars, fixvars, eqns, reeqns) = unwrap_break_err!(collectInitialVarsEqnsSystem(&dae.eqs, vars.clone(), fixvars.clone(), eqns.clone(), reeqns.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(collectInitialBindings) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>, metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat(&(literal!("collectInitialBindings (initialization)"))), '__try0);
        eqnsLst = unwrap_break_err!(BackendEquation::equationList(eqns.clone()), '__try0);
        reeqnsLst = unwrap_break_err!(BackendEquation::equationList(reeqns.clone()), '__try0);
        (_, eqnsLst, reeqnsLst, _) = unwrap_break_err!(BackendDAECreate::patchRecordBindings(&(metamodelica::nil()), &(metamodelica::nil()), unwrap_break_err!(BackendVariable::varList(&dae.shared.globalKnownVars), '__try0), eqnsLst.clone(), reeqnsLst.clone(), metamodelica::nil()), '__try0);
        eqns = unwrap_break_err!(BackendEquation::listEquation(&eqnsLst), '__try0);
        reeqns = unwrap_break_err!(BackendEquation::listEquation(&reeqnsLst), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::NF_SCALARIZE.clone()), '__try0) {
            vars = unwrap_break_err!(BackendVariable::scalarizeVariables(vars.clone()), '__try0);
            initVars = unwrap_break_err!(BackendVariable::scalarizeVariables(initVars.clone()), '__try0);
        }
        useHomotopy = unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(eqns.clone(), (std::sync::Arc::new(simplifyInitialFunctions) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(execStat(&(literal!("simplifyInitialFunctions (initialization)"))), '__try0);
        vars = unwrap_break_err!(BackendVariable::rehashVariables(vars.clone()), '__try0);
        fixvars = unwrap_break_err!(BackendVariable::rehashVariables(fixvars.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::INITIALSYSTEM, dae.shared.info.clone(), dae.shared.cache.clone(), dae.shared.graph.clone()), '__try0);
        shared = BackendDAEUtil::setSharedRemovedEqns(shared.clone(), BackendEquation::emptyEqns());
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), fixvars.clone());
        shared = BackendDAEUtil::setSharedOptimica(
            shared.clone(),
            dae.shared.constraints.clone(),
            dae.shared.classAttrs.clone(),
        );
        shared = BackendDAEUtil::setSharedFunctionTree(shared.clone(), dae.shared.functionTree.clone());
        unwrap_break_err!(execStat(&(literal!("setup shared object (initialization)"))), '__try0);
        initsyst = BackendDAEUtil::createEqSystem(
            vars.clone(),
            eqns.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        initsyst = BackendDAEUtil::setEqSystRemovedEqns(initsyst.clone(), reeqns.clone());
        if useHomotopy {
            initsyst0 = BackendDAEUtil::copyEqSystem(&initsyst);
            enabledModules = if (unwrap_break_err!(Config::adaptiveHomotopy(), '__try0)) {
                list![literal!("inlineHomotopy"), literal!("generateHomotopyComponents")]
            } else {
                metamodelica::nil()
            };
            disabledModules = metamodelica::nil();
        } else {
            initsyst0 = initsyst.clone();
            enabledModules = metamodelica::nil();
            disabledModules = list![literal!("inlineHomotopy"), literal!("generateHomotopyComponents")];
        }
        (initdae, dumpVars, outRemovedInitialEquations) = unwrap_break_err!(createInitialDAEFromSystem(initsyst.clone(), shared.clone(), initVars.clone(), &enabledModules, &disabledModules, outGlobalKnownVars.clone(), false), '__try0);
        (outSimDAE, _) = unwrap_break_err!(BackendVariable::traverseBackendDAE(outSimDAE.clone(), (std::sync::Arc::new(updateFixedAttribute) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)> + 'static>), unwrap_break_err!(BackendVariable::listVar(dumpVars.clone()), '__try0)), '__try0);
        if useHomotopy && unwrap_break_err!(Config::globalHomotopy(), '__try0) {
            initsyst0 = unwrap_break_err!(replaceHomotopyWithSimplifiedEqs(initsyst0.clone()), '__try0);
            initdae0 = metamodelica::Ref::new(BackendDAE::BackendDAE {
                eqs: list![initsyst0.clone()],
                shared: shared.clone(),
            });
            initdae0 = BackendDAEUtil::setFunctionTree(&initdae0, BackendDAEUtil::getFunctions(&initdae.shared));
            (initdae0, _, removedEqns) = unwrap_break_err!(createInitialDAEFromSystem(initsyst0.clone(), shared.clone(), initVars.clone(), &(metamodelica::nil()), &(list![literal!("inlineHomotopy"), literal!("generateHomotopyComponents")]), outGlobalKnownVars.clone(), true), '__try0);
            outRemovedInitialEquations = listAppend(removedEqns.clone(), outRemovedInitialEquations.clone());
            assign_field!(
                initdae0.shared = BackendDAEUtil::setSharedGlobalKnownVars(
                    initdae0.shared.clone(),
                    BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())
                )
            );
            outInitDAE_lambda0 = Some(initdae0.clone());
            initdae = BackendDAEUtil::setFunctionTree(&initdae, BackendDAEUtil::getFunctions(&initdae0.shared));
        } else {
            outInitDAE_lambda0 = None;
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_EQNINORDER.clone()), '__try0)
            && unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0)
        {
            unwrap_break_err!(BackendDump::dumpEqnsSolved(&initdae, &(literal!("initial system: eqns in order"))), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::ITERATION_VARS.clone()), '__try0) {
            unwrap_break_err!(BackendDAEOptimize::listAllIterationVariables(&initdae), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone()), '__try0)
            || unwrap_break_err!(Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone()), '__try0)
            || unwrap_break_err!(Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone()), '__try0)
        {
            unwrap_break_err!(BackendDump::dumpCompShort(&initdae), '__try0);
        }
        outInitDAE = initdae.clone();
        Ok::<_, &'static str>((
            allPrimaryParameters.clone(),
            dae.clone(),
            disabledModules.clone(),
            dumpVars.clone(),
            enabledModules.clone(),
            eqns.clone(),
            eqnsLst.clone(),
            fixvars.clone(),
            hs.clone(),
            initVars.clone(),
            initdae.clone(),
            initsyst.clone(),
            initsyst0.clone(),
            outAllPrimaryParameters.clone(),
            outGlobalKnownVars.clone(),
            outInitDAE.clone(),
            outInitDAE_lambda0.clone(),
            outRemovedInitialEquations.clone(),
            outSimDAE.clone(),
            reeqns.clone(),
            reeqnsLst.clone(),
            shared.clone(),
            useHomotopy.clone(),
            vars.clone(),
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
            __try0_o13,
            __try0_o14,
            __try0_o15,
            __try0_o16,
            __try0_o17,
            __try0_o18,
            __try0_o19,
            __try0_o20,
            __try0_o21,
            __try0_o22,
            __try0_o23,
        )) => {
            allPrimaryParameters = __try0_o0;
            dae = __try0_o1;
            disabledModules = __try0_o2;
            dumpVars = __try0_o3;
            enabledModules = __try0_o4;
            eqns = __try0_o5;
            eqnsLst = __try0_o6;
            fixvars = __try0_o7;
            hs = __try0_o8;
            initVars = __try0_o9;
            initdae = __try0_o10;
            initsyst = __try0_o11;
            initsyst0 = __try0_o12;
            outAllPrimaryParameters = __try0_o13;
            outGlobalKnownVars = __try0_o14;
            outInitDAE = __try0_o15;
            outInitDAE_lambda0 = __try0_o16;
            outRemovedInitialEquations = __try0_o17;
            outSimDAE = __try0_o18;
            reeqns = __try0_o19;
            reeqnsLst = __try0_o20;
            shared = __try0_o21;
            useHomotopy = __try0_o22;
            vars = __try0_o23;
        }
        Err(__try0_err) => {
            Error::addCompilerError(literal!("No system for the symbolic initialization was generated"))?;
            return Err(__try0_err);
        }
    }
    Ok((
        outInitDAE,
        outInitDAE_lambda0,
        outRemovedInitialEquations,
        outGlobalKnownVars,
        outSimDAE,
    ))
}

pub(crate) fn createInitialDAEFromSystem(
    mut inInitsyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut initVars: BackendDAE::Variables,
    mut enabledModules: &metamodelica::List<ArcStr>,
    mut disabledModules: &metamodelica::List<ArcStr>,
    mut globalKnownVars: BackendDAE::Variables,
    mut isLambda0: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut initdae: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut dumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut removedEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut systemStr: ArcStr = if (isLambda0) {
        literal!("initialization_lambda0")
    } else {
        literal!("initialization")
    };
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut initsyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut dumpVars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut initOptModules: metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>;
    let mut daeHandler: (
        BackendDAEFunc::StructurallySingularSystemHandlerFunc,
        ArcStr,
        BackendDAEFunc::stateDeselectionFunc,
        ArcStr,
    );
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut b1: bool;
    let mut b2: bool;
    let mut msg: ArcStr;
    (initsyst, dumpVars) = preBalanceInitialSystem(inInitsyst, &initVars, isLambda0)?;
    execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("preBalanceInitialSystem ("));
            __mm_s.push_str(&*systemStr);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    initdae = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![initsyst.clone()],
        shared: shared.clone(),
    });
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(
            &initdae,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("created "));
                __mm_s.push_str(&*systemStr);
                __mm_s.push_str(&*literal!(" system"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    if Flags::isSet(Flags::PARTITION_INITIALIZATION.clone())? {
        (systs, shared) =
            BackendDAEOptimize::partitionIndependentBlocksHelper(initsyst, shared, Error::getNumErrorMessages(), true)?;
        initdae = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: systs,
            shared: shared,
        });
        execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("partitionIndependentBlocks ("));
                __mm_s.push_str(&*systemStr);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(
            &initdae,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("partitioned "));
                __mm_s.push_str(&*systemStr);
                __mm_s.push_str(&*literal!(" system"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    (initdae, dumpVars2, removedEqns) = analyzeInitialSystem(
        &initdae,
        initVars,
        (std::sync::Arc::new(balanceInitialSystem)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::EqSystem>,
                        metamodelica::Ref<BackendDAE::Shared>,
                        i32,
                        BackendDAE::Variables,
                        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
                        DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::EqSystem>,
                        metamodelica::Ref<BackendDAE::Shared>,
                        i32,
                    )> + 'static,
            >),
    )?;
    dumpVars = listAppend(dumpVars, dumpVars2);
    execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("analyzeInitialSystem ("));
            __mm_s.push_str(&*systemStr);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(&initdae, &systemStr)?;
    }
    initdae = BackendDAEUtil::mapEqSystem(&initdae, &solveInitialSystemEqSystem)?;
    execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("solveInitialSystemEqSystem ("));
            __mm_s.push_str(&*systemStr);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    initdae = BackendDAEUtil::transformBackendDAE(
        &initdae,
        Some((
            openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION,
            openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT,
        )),
        None,
        None,
    )?;
    execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("matching and sorting (n="));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(&initdae)?)));
            __mm_s.push_str(&*literal!(") ("));
            __mm_s.push_str(&*systemStr);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    initdae = BackendDAEOptimize::addInitialStmtsToAlgorithms(&initdae, true)?;
    initdae = BackendDAEUtil::setDAEGlobalKnownVars(&initdae, globalKnownVars);
    initOptModules = BackendDAEUtil::getInitOptModules(None, enabledModules, disabledModules)?;
    matchingAlgorithm = BackendDAEUtil::getMatchingAlgorithm(None)?;
    daeHandler = BackendDAEUtil::getIndexReductionMethod(Some(literal!("none")))?;
    initdae = BackendDAEUtil::postOptimizeDAE(initdae, &initOptModules, &matchingAlgorithm, &daeHandler)?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(
            &initdae,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("solved "));
                __mm_s.push_str(&*systemStr);
                ArcStr::from(__mm_s)
            }),
        )?;
        if Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone())? {
            BackendDump::graphvizBackendDAE(&initdae, literal!("dumpinitialsystem"))?;
        }
    }
    assign_field!(
        initdae.shared = BackendDAEUtil::setSharedGlobalKnownVars(
            initdae.shared.clone(),
            BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())
        )
    );
    b1 = !((dumpVars).is_empty());
    b2 = !((removedEqns).is_empty());
    msg = literal!(
        "For more information set -d=initialization. In OMEdit Tools->Options->Simulation->Show additional information from the initialization process, in OMNotebook call setCommandLineOptions(\"-d=initialization\")"
    );
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        if b1 {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Assuming fixed start value for the following "));
                __mm_s.push_str(&*intString(((dumpVars).len() as i32)));
                __mm_s.push_str(&*literal!(" variables:\n"));
                __mm_s.push_str(&*warnAboutVars2(dumpVars.clone())?);
                ArcStr::from(__mm_s)
            })?;
        }
        if b2 {
            Error::addMessage(
                Error::INITIALIZATION_OVER_SPECIFIED.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("The following "));
                    __mm_s.push_str(&*intString(((removedEqns).len() as i32)));
                    __mm_s.push_str(&*literal!(
                        " initial equations are redundant, so they are removed from the "
                    ));
                    __mm_s.push_str(&*systemStr);
                    __mm_s.push_str(&*literal!(" system:\n"));
                    __mm_s.push_str(&*warnAboutEqns2(&removedEqns)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
        }
    } else {
        if b1 {
            Error::addMessage(Error::INITIALIZATION_NOT_FULLY_SPECIFIED.clone(), list![msg.clone()])?;
        }
        if b2 {
            Error::addMessage(Error::INITIALIZATION_OVER_SPECIFIED.clone(), list![msg])?;
        }
    }
    Ok((initdae, dumpVars, removedEqns))
}

// =============================================================================
// section for helper functions of solveInitialSystem
//
// =============================================================================
fn solveInitialSystemEqSystem(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut nVars: i32;
    let mut nEqns: i32;
    nEqns = BackendDAEUtil::systemSize(&isyst)?;
    nVars = BackendVariable::varsSize(&(BackendVariable::daeVars(&isyst)));
    if intGt(nEqns, nVars) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "It was not possible to solve the over-determined initial system ("
                ));
                __mm_s.push_str(&*intString(nEqns));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(nVars));
                __mm_s.push_str(&*literal!(" variables)"));
                ArcStr::from(__mm_s)
            })?;
            BackendDump::dumpEqSystem(
                isyst.clone(),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "It was not possible to solve the over-determined initial system ("
                    ));
                    __mm_s.push_str(&*intString(nEqns));
                    __mm_s.push_str(&*literal!(" equations and "));
                    __mm_s.push_str(&*intString(nVars));
                    __mm_s.push_str(&*literal!(" variables)"));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        return Err("fail");
    }
    if intLt(nEqns, nVars) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "It was not possible to solve the under-determined initial system ("
                ));
                __mm_s.push_str(&*intString(nEqns));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(nVars));
                __mm_s.push_str(&*literal!(" variables)"));
                ArcStr::from(__mm_s)
            })?;
            BackendDump::dumpEqSystem(
                isyst,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "It was not possible to solve the under-determined initial system ("
                    ));
                    __mm_s.push_str(&*intString(nEqns));
                    __mm_s.push_str(&*literal!(" equations and "));
                    __mm_s.push_str(&*intString(nVars));
                    __mm_s.push_str(&*literal!(" variables)"));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        return Err("fail");
    }
    Ok((osyst, outShared))
}

// =============================================================================
// section for inlining when-clauses
//
// This section contains all the helper functions to replace all when-clauses
// from a given BackendDAE to get the initial equation system.
// =============================================================================
fn inlineWhenForInitialization(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut clockEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut leftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = HashSet::emptyHashSet();
    assign_field!(outDAE.eqs = List::map(inDAE.eqs.clone(), &inlineWhenForInitializationSystem)?);
    (eqnlst, _) = BackendEquation::traverseEquationArray(
        inDAE.shared.removedEqs.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
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
                ),
            ),
        )| inlineWhenForInitializationEquation(__a0, &__a1),
        (metamodelica::nil(), leftCrs),
    )?;
    clockEqnsLst = BackendEquation::traverseEquationArray(
        inDAE.shared.removedEqs.clone(),
        &fnptr!(
            SynchronousFeatures::getBoolClockWhenClauses,
            metamodelica::Ref<BackendDAE::Equation>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>
        ),
        metamodelica::nil(),
    )?;
    eqnlst = listAppend(clockEqnsLst, eqnlst);
    assign_field!(
        outDAE.shared =
            BackendDAEUtil::setSharedRemovedEqns(outDAE.shared.clone(), BackendEquation::listEquation(&eqnlst)?)
    );
    Ok(outDAE)
}

fn inlineWhenForInitializationSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut leftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = HashSet::emptyHashSet();
    let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (eqnlst, leftCrs) = BackendEquation::traverseEquationArray(
        inEqSystem.orderedEqs.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
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
                ),
            ),
        )| inlineWhenForInitializationEquation(__a0, &__a1),
        (metamodelica::nil(), leftCrs),
    )?;
    crefLst = BaseHashSet::hashSetList(&leftCrs)?;
    eqnlst = generateInactiveWhenEquationForInitialization(&crefLst, DAE::emptyElementSource().clone(), eqnlst)?;
    outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem, BackendEquation::listEquation(&eqnlst)?);
    outEqSystem = BackendDAEUtil::clearEqSyst(&outEqSystem);
    Ok(outEqSystem)
}

fn inlineWhenForInitializationEquation(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
    );
    let mut eqAttr: BackendDAE::EquationAttributes;
    let mut weqn: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut alg: metamodelica::Ref<DAE::Algorithm>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut crefExpand: DAE::Expand;
    let mut leftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut size: i32;
    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut accEq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (accEq, leftCrs) = inTpl.clone();
    outTpl = (match &*inEq {
        BackendDAE::Equation::WHEN_EQUATION {
            whenEquation: __esc_weqn,
            source: __esc_source,
            attr: __esc_eqAttr,
            ..
        } => {
            weqn = (*__esc_weqn).clone();
            source = (*__esc_source).clone();
            eqAttr = (*__esc_eqAttr).clone();
            (leftCrs, eqns) = inlineWhenForInitializationWhenEquation(
                metamodelica::AsArg::as_arg(&weqn),
                source.clone(),
                eqAttr.clone(),
                accEq,
                leftCrs,
            )?;
            (eqns, leftCrs)
        }
        BackendDAE::Equation::ALGORITHM {
            alg: __esc_alg,
            source: __esc_source,
            expand: __esc_crefExpand,
            ..
        } => {
            alg = (*__esc_alg).clone();
            source = (*__esc_source).clone();
            crefExpand = (*__esc_crefExpand).clone();
            let __arc1 = alg.clone();
            let DAE::ALGORITHM_STMTS { statementLst: __pa0 } = &*__arc1;
            stmts = metamodelica::Own::own(__pa0);
            (stmts, leftCrs) = inlineWhenForInitializationWhenAlgorithm(&stmts, metamodelica::nil(), leftCrs)?;
            alg = metamodelica::Ref::new(DAE::Algorithm {
                statementLst: stmts.clone(),
            });
            size = ((CheckModel::checkAndGetAlgorithmOutputs(
                metamodelica::AsArg::as_arg(&alg),
                metamodelica::AsArg::as_arg(&source),
                crefExpand.clone(),
            )?)
            .len() as i32);
            eqns = List::consOnTrue(
                !((stmts).is_empty()),
                metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM {
                    size: size,
                    alg: alg.clone(),
                    source: source.clone(),
                    expand: crefExpand.clone(),
                    attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                }),
                accEq,
            );
            (eqns, leftCrs)
        }
        _ => (metamodelica::cons(inEq, accEq), leftCrs),
    });
    Ok((outEq, outTpl))
}

fn inlineWhenForInitializationWhenEquation(
    mut inWEqn: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttr: BackendDAE::EquationAttributes,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inLeftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outLeftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = inLeftCrs;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut eLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut whenStmtLst: metamodelica::List<BackendDAE::WhenOperator>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut active: bool;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    outEqns = (match &**inWEqn {
        BackendDAE::WhenEquation {
            condition: __esc_condition,
            whenStmtLst: __esc_whenStmtLst,
            ..
        } => {
            condition = (*__esc_condition).clone();
            whenStmtLst = (*__esc_whenStmtLst).clone();
            active = Expression::containsInitialCall(metamodelica::AsArg::as_arg(&condition))?;
            for mut stmt in &*whenStmtLst.clone() {
                let () = (::match_deref::match_deref! { match &(stmt.clone()) {
                    BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, right: __esc_e, .. } => {
                        cr = (*__esc_cr).clone();
                        e = (*__esc_e).clone();
                        if active {
                            lhs = Expression::crefExp(cr.clone())?;
                            eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr)?;
                            outEqns = metamodelica::cons(eqn, outEqns);
                        } else {
                            outLeftCrs = List::fold(&(ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?), &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), outLeftCrs)?;
                        }
                        ()
                    },
                    BackendDAE::WhenOperator::ASSIGN { left: __esc_lhs @ Deref @ DAE::Exp::TUPLE { PR: __esc_eLst }, right: __esc_e, .. } => {
                        lhs = (*__esc_lhs).clone();
                        eLst = (*__esc_eLst).clone();
                        e = (*__esc_e).clone();
                        if active {
                            eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr)?;
                            outEqns = metamodelica::cons(eqn, outEqns);
                        } else {
                            crefLst = List::flatten(List::map(eLst.clone(), &Expression::getAllCrefs)?)?;
                            for mut cr in &*crefLst {
                                let mut cr = cr.clone();
                                outLeftCrs = List::fold(&(ComponentReference::expandCref(&cr, true)?), &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), outLeftCrs)?;
                            }
                        }
                        ()
                    },
                    BackendDAE::WhenOperator::NORETCALL { exp: __esc_e, source: __esc_source } => {
                        e = (*__esc_e).clone();
                        source = (*__esc_source).clone();
                        if active {
                            eqn = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] }), source: inSource.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            outEqns = metamodelica::cons(eqn, outEqns);
                        }
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            outEqns
        }
        _ => outEqns,
    });
    Ok((outLeftCrs, outEqns))
}

fn inlineWhenForInitializationWhenAlgorithm<'__b>(
    mut inStmts: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inLeftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStmts {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inAcc.reverse(), inLeftCrs))
            },
            Deref @ metamodelica::ListNode::Cons { head: stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, tail: rest } => {
                let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut leftCrs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                (stmts, leftCrs) = inlineWhenForInitializationWhenStmt(stmt.clone(), inLeftCrs, &inAcc)?;
                { (inStmts, inAcc, inLeftCrs) = (rest, stmts, leftCrs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest } => {
                let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut leftCrs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                { (inStmts, inAcc, inLeftCrs) = (rest, metamodelica::cons(stmt.clone(), inAcc), inLeftCrs); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn inlineWhenForInitializationWhenStmt<'__b>(
    mut inWhenStatement: metamodelica::Ref<DAE::Statement>,
    mut inLeftCrs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut inAcc: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inWhenStatement) {
            Deref @ DAE::Statement::STMT_WHEN { exp: condition, statementLst: stmts, .. } if (Expression::containsInitialCall(metamodelica::AsArg::as_arg(&condition))?) => {
                let mut stmts = (*stmts).clone();
                stmts = List::foldr(metamodelica::AsArg::as_arg(&stmts), &fnptr!(List::consr, _, _), inAcc.clone())?;
                return Ok((stmts.clone(), inLeftCrs))
            },
            Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: None, .. } => {
                let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut leftCrs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                crefLst = CheckModel::algorithmStatementListOutputs(metamodelica::AsArg::as_arg(&stmts), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
                leftCrs = List::fold(&crefLst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), inLeftCrs)?;
                return Ok((inAcc.clone(), leftCrs))
            },
            Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: Some(stmt), .. } => {
                let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut leftCrs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut stmts = (*stmts).clone();
                crefLst = CheckModel::algorithmStatementListOutputs(metamodelica::AsArg::as_arg(&stmts), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
                leftCrs = List::fold(&crefLst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), inLeftCrs)?;
                { (inWhenStatement, inLeftCrs, inAcc) = (stmt.clone(), leftCrs, inAcc); continue '__tco; }
            },
            _ => {
                Error::addInternalError(literal!("function inlineWhenForInitializationWhenStmt failed"), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn generateInactiveWhenEquationForInitialization(
    mut inCrLst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut identType: metamodelica::Ref<DAE::Type>;
    let mut crefExp: metamodelica::Ref<DAE::Exp>;
    let mut crefPreExp: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    for mut cr in &**inCrLst {
        identType = ComponentReference::crefTypeConsiderSubs(metamodelica::AsArg::as_arg(&cr))?;
        crefExp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: cr.clone(),
            ty: identType.clone(),
        });
        crefPreExp = Expression::makePureBuiltinCall(literal!("pre"), list![crefExp.clone()], identType);
        eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: crefExp,
            scalar: crefPreExp,
            source: inSource.clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        });
        outEqns = metamodelica::cons(eqn, outEqns);
    }
    Ok(outEqns)
}

// =============================================================================
// section for collecting all variables, of which the left limit is also used.
//
// collect all pre variables in time equations
// =============================================================================
fn collectPreVariables(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outHS = List::fold(
        &inDAE.eqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        )| collectPreVariablesEqSystem(&__a0, __a1),
        HashSet::emptyHashSet(),
    )?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        inDAE.shared.initialEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(collectPreVariablesTraverseExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            outHS,
        ),
    )?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        inDAE.shared.removedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(collectPreVariablesTraverseExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            outHS,
        ),
    )?;
    Ok(outHS)
}

pub(crate) fn collectPreVariablesEqSystem(
    mut inSyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        inSyst.orderedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(collectPreVariablesTraverseExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            inHS,
        ),
    )?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        inSyst.removedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(collectPreVariablesTraverseExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            outHS,
        ),
    )?;
    Ok(outHS)
}

pub(crate) fn collectPreVariablesTraverseExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outHS = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, &collectPreVariablesTraverseExp2, inHS)?;
            outHS
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, &collectPreVariablesTraverseExp2, inHS)?;
            outHS
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, &collectPreVariablesTraverseExp2, inHS)?;
            outHS
        },
        _ => inHS,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHS))
}

fn collectPreVariablesTraverseExp2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outHS: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    outHS = (match &*inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
            outHS = List::fold(&crefs, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), inHS)?;
            outHS
        }
        _ => inHS,
    });
    Ok((outExp, outHS))
}

fn warnAboutVars2(mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut strs: metamodelica::List<ArcStr>;
    let mut len: i32;
    let mut size: i32;
    if (vars).is_empty() {
        outString = literal!("");
        return Ok(outString);
    }
    strs = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (vars).into_iter().cloned() {
            let __x = BackendDump::varString(&(v.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    len = ((strs).len() as i32);
    size = ({
        let mut __acc: i32 = 0;
        for mut s in (strs.clone()).into_iter().cloned() {
            let __x = ((s).len() as i32);
            __acc += __x;
        }
        __acc
    }) + len * 10;
    outString = warnAboutVars2Work(&strs, literal!("         "), literal!("\n"), size)?;
    Ok(outString)
}

fn warnAboutVars2Work(
    mut strs: &metamodelica::List<ArcStr>,
    mut prefix: ArcStr,
    mut suffix: ArcStr,
    mut size: i32,
) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut sb: System::StringAllocator = System::StringAllocator(size)?;
    let mut i: i32 = 0;
    for mut r#str in &**strs {
        System::stringAllocatorStringCopy(sb.clone(), prefix.clone(), i);
        i = i + ((prefix).len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), r#str.clone(), i);
        i = i + ((r#str).len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), suffix.clone(), i);
        i = i + ((suffix).len() as i32);
    }
    s = System::stringAllocatorResult(sb, s);
    Ok(s)
}

fn warnAboutEqns2(mut inEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inEqns {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: eq, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut crStr: ArcStr;
            crStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?); ArcStr::from(__mm_s) };
            crStr
        },
        Deref @ metamodelica::ListNode::Cons { head: eq, tail: eqns } => {
            let mut crStr: ArcStr;
            let mut r#str: ArcStr;
            crStr = BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*crStr); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*warnAboutEqns2(eqns)?); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

// =============================================================================
// section for selecting initialization variables
//
//   - unfixed state
//   - secondary parameter
//   - unfixed discrete -> pre(vd)
// =============================================================================
fn selectInitializationVariablesDAE(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendDAE::Variables,
)> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    let mut outInitVars: BackendDAE::Variables;
    let mut outAllPrimaryParameters: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outGlobalKnownVars: BackendDAE::Variables = dae.shared.globalKnownVars.clone();
    let mut otherVariables: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut globalKnownVarsEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut globalKnownVarsSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut comps: metamodelica::List<metamodelica::List<i32>>;
    let mut flatComps: metamodelica::List<i32>;
    let mut nGlobalKnownVars: i32;
    let mut secondary: metamodelica::Array<i32>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut bindExp: metamodelica::Ref<DAE::Exp>;
    let mut primary: BackendDAE::Variables;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut globalKnownVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    for mut var in &*BackendVariable::varList(&dae.shared.globalKnownVars)? {
        let mut var = var.clone();
        if BackendVariable::isInput(&var)
            && !(Expression::isConstValue(&(BackendVariable::varStartValue(&var)?))?)
            && !(Types::isArray(&(BackendVariable::varType(&var))))
        {
            bindExp = BackendVariable::varStartValue(&var)?;
            (v, _) = BackendVariable::getVarSingle(&(Expression::expCref(&bindExp)?), &dae.shared.globalKnownVars)?;
            var = BackendVariable::setVarStartValueOption(var, v.bindExp.clone())?;
        }
        globalKnownVarList = metamodelica::cons(var, globalKnownVarList);
    }
    dae = BackendDAEUtil::setDAEGlobalKnownVars(&dae, BackendVariable::listVar(globalKnownVarList)?);
    globalKnownVars = BackendVariable::listVar(BackendVariable::varList(&dae.shared.globalKnownVars)?)?;
    outInitVars = selectInitializationVariables(&dae.eqs)?;
    outInitVars = BackendVariable::traverseBackendDAEVars(
        dae.shared.globalKnownVars.clone(),
        (std::sync::Arc::new(fnptr!(
            selectInitializationVariables2,
            metamodelica::Ref<BackendDAE::Var>,
            BackendDAE::Variables
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendDAE::Variables,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                    + 'static,
            >),
        outInitVars,
    )?;
    outInitVars = BackendVariable::traverseBackendDAEVars(
        dae.shared.aliasVars.clone(),
        (std::sync::Arc::new(fnptr!(
            selectInitializationVariables2,
            metamodelica::Ref<BackendDAE::Var>,
            BackendDAE::Variables
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendDAE::Variables,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                    + 'static,
            >),
        outInitVars,
    )?;
    globalKnownVars = BackendVariable::traverseBackendDAEVars(
        dae.shared.externalObjects.clone(),
        (std::sync::Arc::new(addExtObjToGlobalKnownVars)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendDAE::Variables,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                    + 'static,
            >),
        globalKnownVars,
    )?;
    nGlobalKnownVars = BackendVariable::varsSize(&globalKnownVars);
    otherVariables = BackendVariable::emptyVarsSized(nGlobalKnownVars);
    globalKnownVarsEqns = BackendEquation::emptyEqnsSized(nGlobalKnownVars);
    globalKnownVarsEqns = BackendVariable::traverseBackendDAEVars(
        globalKnownVars.clone(),
        (std::sync::Arc::new(createGlobalKnownVarsEquations)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
                    )> + 'static,
            >),
        globalKnownVarsEqns,
    )?;
    if nGlobalKnownVars > 0 {
        globalKnownVarsSystem = BackendDAEUtil::createEqSystem(
            globalKnownVars.clone(),
            globalKnownVarsEqns,
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        (m, mT) = BackendDAEUtil::adjacencyMatrix(
            &globalKnownVarsSystem,
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(&dae.shared),
        )?;
        (ass1, ass2) = Matching::PerfectMatching(m.clone())?;
        comps = Sorting::Tarjan(m.clone(), ass1.clone(), metamodelica::arrayLength(ass1.clone()))?;
        comps = mapListIndices(comps, ass2.clone())?;
        flatComps = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut comp in (comps).into_iter().cloned() {
                let __x = flattenParamComp(&(comp.clone()), &globalKnownVars)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        secondary = arrayCreate(nGlobalKnownVars, 0);
        secondary = selectSecondaryParameters(&flatComps, &globalKnownVars, mT.clone(), secondary.clone())?;
        primary = BackendVariable::emptyVarsSized(nGlobalKnownVars);
        for mut i in &*flatComps {
            v = BackendVariable::getVarAt(&globalKnownVars, i.clone())?;
            bindExp = BackendVariable::varBindExpStartValueNoFail(&v)?;
            crefs = Expression::extractCrefsFromExp(bindExp.clone())?;
            let () = (::match_deref::match_deref! { match &(v.clone()) {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } if (0 == ({let __elt = (*metamodelica::index_checked(&secondary.borrow(), i.clone())?).clone(); __elt}) && allPrimary(&crefs, &primary)?) => {
                    outAllPrimaryParameters = metamodelica::cons(v.clone(), outAllPrimaryParameters);
                    primary = BackendVariable::addVar(v.clone(), primary)?;
                    ()
                },
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, bindExp: Some(__esc_bindExp), .. } if (0 == ({let __elt = (*metamodelica::index_checked(&secondary.borrow(), i.clone())?).clone(); __elt}) && allPrimary(&crefs, &primary)?) => {
                    bindExp = (*__esc_bindExp).clone();
                    outAllPrimaryParameters = metamodelica::cons(v.clone(), outAllPrimaryParameters);
                    v = BackendVariable::setVarFixed(v, true)?;
                    outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars)?;
                    primary = BackendVariable::addVar(v.clone(), primary)?;
                    ()
                },
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
                    otherVariables = BackendVariable::addVar(v.clone(), otherVariables)?;
                    v = BackendVariable::setVarFixed(v, false)?;
                    outInitVars = BackendVariable::addVar(v.clone(), outInitVars)?;
                    outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars)?;
                    ()
                },
                _ if (BackendVariable::isVarAlg(&v) && 0 == ({let __elt = (*metamodelica::index_checked(&secondary.borrow(), i.clone())?).clone(); __elt}) && allPrimary(&crefs, &primary)?) => {
                    otherVariables = BackendVariable::addVar(v.clone(), otherVariables)?;
                    v = BackendVariable::setVarFixed(v, true)?;
                    v = BackendVariable::setVarFinal(v, true)?;
                    outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars)?;
                    primary = BackendVariable::addVar(v.clone(), primary)?;
                    ()
                },
                _ if (BackendVariable::isVarAlg(&v)) => {
                    otherVariables = BackendVariable::addVar(v.clone(), otherVariables)?;
                    v = BackendVariable::setVarFixed(v, false)?;
                    outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars)?;
                    ()
                },
                _ => {
                    otherVariables = BackendVariable::addVar(v.clone(), otherVariables)?;
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        GCExt::free(secondary.clone());
        outAllPrimaryParameters = outAllPrimaryParameters.reverse();
        dae = BackendDAEUtil::setDAEGlobalKnownVars(&dae, otherVariables);
    }
    Ok((dae, outInitVars, outAllPrimaryParameters, outGlobalKnownVars))
}

fn allPrimary(
    mut crefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut primary: &BackendDAE::Variables,
) -> Result<bool> {
    let mut b: bool = true;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    for mut cr in &**crefs {
        if !(ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), &(DAE::crefTime().clone()))?) {
            match '__try0: {
                (vars, _) = unwrap_break_err!(BackendVariable::getVar(cr.clone(), primary), '__try0);
                b = ((vars).len() as i32)
                    == unwrap_break_err!(elementCount(&(unwrap_break_err!(ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr)), '__try0))), '__try0);
                Ok::<_, &'static str>((b.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    b = __try0_o0;
                }
                Err(_) => {
                    b = false;
                }
            }
            if !(b) {
                return Ok(b);
            }
        }
    }
    Ok(b)
}

fn elementCount(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<i32> {
    let mut n: i32;
    n = (match &**ty {
        DAE::Type::T_ARRAY {
            dims: __ty_dims,
            ty: __ty_ty,
        } => {
            elementCount(metamodelica::AsArg::as_arg(&__ty_ty))?
                * ({
                    let mut __acc: i32 = 1;
                    for mut d in (__ty_dims.clone()).into_iter().cloned() {
                        let __x = Expression::dimensionSize(&(d.clone()))?;
                        __acc *= __x;
                    }
                    __acc
                })
        }
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            varLst: __ty_varLst,
            ..
        } => {
            ({
                let mut __acc: i32 = 0;
                for mut v in (__ty_varLst.clone()).into_iter().cloned() {
                    let __x = elementCount(&(v.ty.clone()))?;
                    __acc += __x;
                }
                __acc
            })
        }
        _ => 1,
    });
    Ok(n)
}

fn addExtObjToGlobalKnownVars(
    mut extObj: metamodelica::Ref<BackendDAE::Var>,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)> {
    let mut extObj: metamodelica::Ref<BackendDAE::Var> = extObj;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    globalKnownVars = (::match_deref::match_deref! { match &(&*extObj) {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, bindExp: Some(_), .. } => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            var = BackendVariable::setVarFixed(extObj.clone(), true)?;
            globalKnownVars = BackendVariable::addVar(var, globalKnownVars)?;
            globalKnownVars
        },
        _ => {
            globalKnownVars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((extObj, globalKnownVars))
}

fn createGlobalKnownVarsEquations(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut parameterEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut parameterEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    > = parameterEqns;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut startValue: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut s: ArcStr;
    let mut r#str: ArcStr;
    let mut info: SourceInfo;
    lhs = BackendVariable::varExp(&var)?;
    if BackendVariable::isParam(&var) && !(BackendVariable::varHasBindExp(&var)) && BackendVariable::varFixed(&var) {
        s = ExpressionBasics::printExpStr(lhs.clone())?;
        startValue = BackendVariable::varStartValue(&var)?;
        r#str = ExpressionBasics::printExpStr(startValue.clone())?;
        v = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
        v = BackendVariable::setBindExp(v, Some(startValue));
        v = BackendVariable::setVarFixed(v, true)?;
        info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(&v));
        Error::addSourceMessage(
            &(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone()),
            list![s, r#str],
            &info,
        )?;
    }
    rhs = BackendVariable::varBindExpStartValueNoFail(&var)?;
    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
        exp: lhs,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
    });
    parameterEqns = BackendEquation::add(eqn, parameterEqns)?;
    Ok((var, parameterEqns))
}

fn markIndex(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outArray: metamodelica::Array<i32> = inArray;
    {
        let __cell0 = 1;
        let __idx0 = inIndex;
        *metamodelica::index_mut_checked(&mut outArray.clone().borrow_mut(), __idx0)? = __cell0;
    }
    Ok(outArray)
}

fn selectSecondaryParameters(
    mut inOrdering: &metamodelica::List<i32>,
    mut inParameters: &BackendDAE::Variables,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut inSecondaryParams: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let __ab_inM = inM.borrow();
    let mut outSecondaryParams: metamodelica::Array<i32> = inSecondaryParams;
    let mut param: metamodelica::Ref<BackendDAE::Var>;
    for mut i in &**inOrdering {
        param = BackendVariable::getVarAt(inParameters, i.clone())?;
        outSecondaryParams = if (if (BackendVariable::isVarAlg(&param)) {
            false
        } else {
            !(BackendVariable::varFixed(&param))
        } || 1
            == ({
                let __elt = (*metamodelica::index_checked(&outSecondaryParams.borrow(), i.clone())?).clone();
                __elt
            })) {
            List::fold(
                &(*metamodelica::index_checked(&__ab_inM, i.clone())?),
                &markIndex,
                outSecondaryParams.clone(),
            )?
        } else {
            outSecondaryParams.clone()
        };
    }
    Ok(outSecondaryParams)
}

pub(crate) fn flattenParamComp(
    mut paramIndices: &metamodelica::List<i32>,
    mut inAllParameters: &BackendDAE::Variables,
) -> Result<i32> {
    let mut outFlatComp: i32;
    outFlatComp = (::match_deref::match_deref! { match paramIndices {
        Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil } => {
            i.clone()
        },
        _ => {
            let mut i: i32 = 0;
            let mut paramLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut param: metamodelica::Ref<BackendDAE::Var>;
            paramLst = metamodelica::nil();
            for mut i in &**paramIndices {
                let mut i = i.clone();
                param = BackendVariable::getVarAt(inAllParameters, i)?;
                paramLst = metamodelica::cons(param, paramLst);
            }
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cyclically dependent parameters found:\n")); __mm_s.push_str(&*warnAboutVars2(paramLst)?); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFlatComp)
}

fn selectInitializationVariables(
    mut inEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    outVars = List::fold(
        inEqSystems,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: BackendDAE::Variables| {
            selectInitializationVariables1(&__a0, __a1)
        },
        outVars,
    )?;
    Ok(outVars)
}

fn selectInitializationVariables1(
    mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inVars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = BackendVariable::traverseBackendDAEVars(
        inEqSystem.orderedVars.clone(),
        (std::sync::Arc::new(fnptr!(
            selectInitializationVariables2,
            metamodelica::Ref<BackendDAE::Var>,
            BackendDAE::Variables
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendDAE::Variables,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                    + 'static,
            >),
        inVars,
    )?;
    Ok(outVars)
}

fn selectInitializationVariables2(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: BackendDAE::Variables,
) -> (metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outVars: BackendDAE::Variables;
    (outVar, outVars) = 'mc: {
        let __mc_input = (&*inVar, inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, vars) => {
                    let mut vars = (*vars).clone();
                    let false = (BackendVariable::varFixed(&inVar)) else { return Err("pattern mismatch") };
                    vars = BackendVariable::addVar(inVar.clone(), vars.clone())?;
                    Ok((inVar.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ty, arryDim, .. }, vars) => {
                    let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                    let mut vars = (*vars).clone();
                    let false = (BackendVariable::varFixed(&inVar)) else { return Err("pattern mismatch") };
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = metamodelica::Ref::new(BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
                    Ok((inVar.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outVars)
}

// =============================================================================
// section for simplifying initial functions
//
// =============================================================================
fn simplifyInitialFunctions(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(simplifyInitialFunctionsExp, metamodelica::Ref<DAE::Exp>, bool),
        inUseHomotopy,
    )?;
    Ok((outExp, outUseHomotopy))
}

fn simplifyInitialFunctionsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. } => {
            (metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: expr, tail: _ } }, .. } => {
            (expr.clone(), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            (inExp, true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

// =============================================================================
// section for pre-balancing the initial system
//
// This section removes unused pre variables and auto-fixes non-pre variables,
// which occur in no equation.
// =============================================================================
fn preBalanceInitialSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut initVars: &BackendDAE::Variables,
    mut isLambda0: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem> = inEqSystem.clone();
    let mut outDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut b: bool;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    (_, mt) = BackendDAEUtil::adjacencyMatrix(
        &inEqSystem,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        true,
    )?;
    (orderedVars, orderedEqs, b, outDumpVars) = preBalanceInitialSystem1(
        metamodelica::arrayLength(mt.clone()),
        mt.clone(),
        inEqSystem.orderedVars.clone(),
        inEqSystem.orderedEqs.clone(),
        initVars,
        isLambda0,
        false,
        metamodelica::nil(),
    )?;
    if b {
        assign_field!(
            outEqSystem.orderedEqs = orderedEqs,
            outEqSystem.orderedVars = orderedVars
        );
        outEqSystem = BackendDAEUtil::clearEqSyst(&outEqSystem);
    }
    Ok((outEqSystem, outDumpVars))
}

fn preBalanceInitialSystem1<'__b>(
    mut n: i32,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut inVars: BackendDAE::Variables,
    mut inEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut initVars: &'__b BackendDAE::Variables,
    mut isLambda0: bool,
    mut inB: bool,
    mut inDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    bool,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    '__tco: loop {
        match (n, inB) {
            (0, false) => return Ok((inVars, inEqs, false, inDumpVars)),
            (0, true) => {
                let mut vars: BackendDAE::Variables;
                vars = BackendVariable::listVar1(&(BackendVariable::varList(&inVars)?))?;
                return Ok((vars, inEqs, true, inDumpVars));
            }
            _ => {
                let mut b: bool;
                let mut vars: BackendDAE::Variables;
                let mut eqs: metamodelica::Ref<
                    ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                >;
                let mut dumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let true = (n > 0) else { return Err("pattern mismatch") };
                (vars, eqs, b, dumpVars) =
                    preBalanceInitialSystem2(n, mt.clone(), inVars, inEqs, initVars, isLambda0, inB, inDumpVars)?;
                {
                    (n, mt, inVars, inEqs, initVars, isLambda0, inB, inDumpVars) =
                        (n - 1, mt.clone(), vars, eqs, initVars, isLambda0, b, dumpVars);
                    continue '__tco;
                }
            }
        }
    }
}

fn preBalanceInitialSystem2(
    mut n: i32,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut inVars: BackendDAE::Variables,
    mut inEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut initVars: &BackendDAE::Variables,
    mut isLambda0: bool,
    mut inB: bool,
    mut inDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    bool,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let __ab_mt = mt.borrow();
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        inEqs.clone();
    let mut outB: bool = inB;
    let mut outDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inDumpVars.clone();
    let mut row: metamodelica::List<i32>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut r#str: ArcStr;
    let mut err_str: ArcStr = literal!(" with unknown reason.");
    match '__try0: {
        row = (*unwrap_break_err!(metamodelica::index_checked(&__ab_mt, n), '__try0)).clone();
        if (row).is_empty() {
            outB = true;
            var = unwrap_break_err!(BackendVariable::getVarAt(&inVars, n), '__try0);
            cref = BackendVariable::varCref(&var);
            if ComponentReference::isPreCref(&cref) {
                (outVars, _) = BackendVariable::removeVars(&(list![n]), &inVars, &(metamodelica::nil()));
            } else if BackendVariable::containsVar(&var, initVars) {
                (outEqs, outDumpVars) = unwrap_break_err!(addStartValueEquations(&(list![var.clone()]), inEqs.clone(), inDumpVars.clone()), '__try0);
            } else {
                r#str = if (isLambda0) {
                    literal!("lambda 0 ")
                } else {
                    literal!("")
                };
                err_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" because variable "));
                    __mm_s.push_str(&*unwrap_break_err!(BackendDump::varString(&var), '__try0));
                    __mm_s.push_str(&*literal!(" does not appear in any equation in the "));
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("initial system and is not fixable."));
                    ArcStr::from(__mm_s)
                };
                break '__try0 Err::<_, _>("fail");
            }
        }
        Ok::<_, &'static str>((row.clone(),))
    } {
        Ok((__try0_o0,)) => {
            row = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Initialization.preBalanceInitialSystem2"));
                    __mm_s.push_str(&*literal!(" failed"));
                    __mm_s.push_str(&*err_str);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/Initialization.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outVars, outEqs, outB, outDumpVars))
}

fn analyzeInitialSystem(
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitVars: BackendDAE::Variables,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::EqSystem>,
                metamodelica::Ref<BackendDAE::Shared>,
                i32,
                BackendDAE::Variables,
                DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
                DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
            ) -> Result<(
                metamodelica::Ref<BackendDAE::EqSystem>,
                metamodelica::Ref<BackendDAE::Shared>,
                i32,
            )> + 'static,
    >,
) -> Result<(
    metamodelica::Ref<BackendDAE::BackendDAE>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut outDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outRemovedEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut dumpVars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>;
    let mut removedEqns: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>;
    let mut filtered_initial_eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    eqs = metamodelica::nil();
    dumpVars = DoubleEnded::fromList(&(metamodelica::nil()))?;
    removedEqns = DoubleEnded::fromList(&(metamodelica::nil()))?;
    for mut syst in &*inInitDAE.eqs.clone() {
        let mut syst = syst.clone();
        if BackendDAEUtil::nonEmptySystem(&syst) {
            eqs = metamodelica::cons(syst, eqs);
        } else {
            filtered_initial_eqs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                for mut eqn in (BackendEquation::equationList(syst.orderedEqs.clone())?)
                    .into_iter()
                    .cloned()
                {
                    if !(BackendEquation::hasAnyUnknown(eqn.clone(), inInitVars.clone())?) {
                        continue;
                    }
                    let __x = eqn.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            DoubleEnded::push_list_back(removedEqns.clone(), &filtered_initial_eqs)?;
            DoubleEnded::push_list_back(
                removedEqns.clone(),
                &(BackendEquation::equationList(syst.removedEqs.clone())?),
            )?;
        }
    }
    dae = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqs,
        shared: inInitDAE.shared.clone(),
    });
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        &dae,
        &({
            let __pe_b3 = inInitVars;
            let __pe_b4 = dumpVars.clone();
            let __pe_b5 = removedEqns.clone();
            move |__pe_a0, __pe_a1, __pe_a2| {
                func(
                    __pe_a0,
                    __pe_a1,
                    __pe_a2,
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        }),
        0,
    )?;
    outRemovedEqns = DoubleEnded::toListAndClear(removedEqns, metamodelica::nil())?;
    outDumpVars = DoubleEnded::toListAndClear(dumpVars, metamodelica::nil())?;
    Ok((outDAE, outDumpVars, outRemovedEqns))
}

fn getInitEqIndices(
    mut equations: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32> = metamodelica::nil();
    let mut i: i32 = 1;
    for mut eq in &**equations {
        if BackendEquation::isInitialEquation(metamodelica::AsArg::as_arg(&eq))? {
            indices = metamodelica::cons(i, indices);
        }
        i = i + 1;
    }
    indices = metamodelica::Dangerous::listReverseInPlace(indices);
    Ok(indices)
}

type constraintHandlerFunc = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            i32,
            BackendDAE::Variables,
            DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
            DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
        ) -> Result<(
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            i32,
        )> + 'static,
>;

fn balanceInitialSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut dummy: i32,
    mut initVars: BackendDAE::Variables,
    mut dumpVars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
    mut removedEqns: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
)> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut debug: bool = false;
    let mut init_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut sim_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut scal_to_arr: metamodelica::Array<i32>;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut comps: metamodelica::List<metamodelica::List<i32>>;
    let mut redundantEqns: metamodelica::List<i32>;
    let mut unfixedVars: metamodelica::List<i32>;
    let mut initASSC: bool = Flags::getConfigBool(Flags::INIT_ASSC.clone())?;
    if BackendVariable::varsSize(&inEqSystem.orderedVars) > 0 {
        (init_eqns, sim_eqns) = List::splitOnTrue(
            &(BackendEquation::equationList(inEqSystem.orderedEqs.clone())?),
            &move |__a0: metamodelica::Ref<BackendDAE::Equation>| BackendEquation::isInitialEquation(&__a0),
        )?;
        outEqSystem = BackendDAEUtil::createEqSystem(
            BackendVariable::sortInitialVars(inEqSystem.orderedVars.clone(), &initVars)?,
            BackendEquation::listEquation(&sim_eqns)?,
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        assign_field!(outEqSystem.removedEqs = inEqSystem.removedEqs.clone());
        funcs = BackendDAEUtil::getFunctions(&inShared);
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(
            outEqSystem,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(funcs.clone()),
            true,
        )?;
        nVars = BackendVariable::varsSize(&outEqSystem.orderedVars);
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns, nVars)?;
        assign_field!(outEqSystem.orderedEqs = BackendEquation::addList(&init_eqns, outEqSystem.orderedEqs.clone())?);
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(
            outEqSystem,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(funcs.clone()),
            true,
        )?;
        (eqn_to_var, var_to_eqn, _, _, _) =
            Matching::ContinueMatching(mT.clone(), nEqns, nVars, eqn_to_var.clone(), var_to_eqn.clone(), false)?;
        unfixedVars = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=metamodelica::arrayLength(var_to_eqn.clone())).into_iter() {
                if !(({
                    let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), i.clone())?).clone();
                    __elt
                }) < 0)
                {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        redundantEqns = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=metamodelica::arrayLength(eqn_to_var.clone())).into_iter() {
                if !(({
                    let __elt = (*metamodelica::index_checked(&eqn_to_var.borrow(), i.clone())?).clone();
                    __elt
                }) < 0)
                {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !((redundantEqns).is_empty() && (unfixedVars).is_empty()) {
            if !((redundantEqns).is_empty()) {
                consistencyCheck(
                    &redundantEqns,
                    outEqSystem.orderedEqs.clone(),
                    &(outEqSystem.orderedVars.clone()),
                    &(inShared.clone()),
                    0,
                    m.clone(),
                    &(enhancedRows(&outEqSystem, inShared.clone())),
                    var_to_eqn.clone(),
                    eqn_to_var.clone(),
                    scal_to_arr.clone(),
                )?;
                redundantEqns = List::unique(
                    &({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut i in (redundantEqns).into_iter().cloned() {
                            let __x = ({
                                let __elt = (*metamodelica::index_checked(&scal_to_arr.borrow(), i.clone())?).clone();
                                __elt
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                );
            }
            outEqSystem = resolveOverAndUnderconstraints(
                outEqSystem,
                &initVars,
                unfixedVars,
                redundantEqns,
                dumpVars.clone(),
                removedEqns.clone(),
            )?;
            (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(
                outEqSystem,
                openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
                Some(funcs.clone()),
                true,
            )?;
            nVars = BackendVariable::varsSize(&outEqSystem.orderedVars);
            nEqns = BackendEquation::equationArraySize(outEqSystem.orderedEqs.clone())?;
            (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns, nVars)?;
        } else if !(initASSC) {
            outEqSystem = inEqSystem;
        }
        if debug {
            BackendDump::dumpEqSystem(outEqSystem.clone(), &(literal!("fixInitialSystem")))?;
            BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
            BackendDump::dumpMatchingVars(var_to_eqn.clone())?;
            BackendDump::dumpMatchingEqns(eqn_to_var.clone())?;
        }
        if initASSC {
            comps = Sorting::Tarjan(m.clone(), var_to_eqn.clone(), nEqns)?;
            for mut comp in &*comps {
                (eqn_to_var, var_to_eqn, outEqSystem, changed) = BackendDAEUtil::analyticalToStructuralSingularity(
                    metamodelica::AsArg::as_arg(&comp),
                    eqn_to_var.clone(),
                    var_to_eqn.clone(),
                    outEqSystem,
                    changed,
                    true,
                )?;
            }
            if changed {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outEqSystem.clone()) {
                    Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                m = metamodelica::Own::own(__pa0);
                mT = metamodelica::Own::own(__pa1);
                (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(
                    outEqSystem,
                    openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                    Some(funcs.clone()),
                    true,
                )?;
                (eqn_to_var, var_to_eqn, _, _, _) = Matching::ContinueMatching(
                    mT.clone(),
                    nEqns,
                    nVars,
                    eqn_to_var.clone(),
                    var_to_eqn.clone(),
                    false,
                )?;
                unfixedVars = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (1..=metamodelica::arrayLength(var_to_eqn.clone())).into_iter() {
                        if !(({
                            let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), i.clone())?).clone();
                            __elt
                        }) < 0)
                        {
                            continue;
                        }
                        let __x = i.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                redundantEqns = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (1..=metamodelica::arrayLength(eqn_to_var.clone())).into_iter() {
                        if !(({
                            let __elt = (*metamodelica::index_checked(&eqn_to_var.borrow(), i.clone())?).clone();
                            __elt
                        }) < 0)
                        {
                            continue;
                        }
                        let __x = i.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                redundantEqns = List::unique(
                    &({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut i in (redundantEqns).into_iter().cloned() {
                            let __x = ({
                                let __elt = (*metamodelica::index_checked(&scal_to_arr.borrow(), i.clone())?).clone();
                                __elt
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                );
                if !((redundantEqns).is_empty() && (unfixedVars).is_empty()) {
                    if !((redundantEqns).is_empty()) {
                        consistencyCheck(
                            &redundantEqns,
                            outEqSystem.orderedEqs.clone(),
                            &(outEqSystem.orderedVars.clone()),
                            &(inShared.clone()),
                            0,
                            m.clone(),
                            &(enhancedRows(&outEqSystem, inShared)),
                            var_to_eqn.clone(),
                            eqn_to_var.clone(),
                            scal_to_arr.clone(),
                        )?;
                    }
                    outEqSystem = resolveOverAndUnderconstraints(
                        outEqSystem,
                        &initVars,
                        unfixedVars,
                        redundantEqns,
                        dumpVars,
                        removedEqns,
                    )?;
                    (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(
                        outEqSystem,
                        openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
                        Some(funcs),
                        true,
                    )?;
                }
            }
        }
    } else {
        outEqSystem = inEqSystem;
    }
    Ok((outEqSystem, outShared, dummy))
}

fn resolveOverAndUnderconstraints(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut initVars: &BackendDAE::Variables,
    mut unfixedVars: metamodelica::List<i32>,
    mut redundantEqns: metamodelica::List<i32>,
    mut dumpVars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
    mut removedEqns: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut debug: bool = false;
    let mut redundant_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut failed_var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut new_eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    redundant_lst = BackendEquation::getList(redundantEqns.clone(), syst.orderedEqs.clone())?;
    DoubleEnded::push_list_back(removedEqns, &redundant_lst)?;
    new_eqns = BackendEquation::deleteList(syst.orderedEqs.clone(), &redundantEqns)?;
    if debug {
        BackendDump::dumpEquationList(&redundant_lst, &(literal!("removed eqns")))?;
    }
    var_lst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut i in (unfixedVars).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(&syst.orderedVars, i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (new_eqns, var_lst) = addStartValueEquations(&var_lst, new_eqns, metamodelica::nil())?;
    DoubleEnded::push_list_back(dumpVars, &var_lst)?;
    if debug {
        failed_var_lst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut var in (var_lst.clone()).into_iter().cloned() {
                if !(!(BackendVariable::containsVar(&(var.clone()), initVars))) {
                    continue;
                }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        BackendDump::dumpVarList(&var_lst, &(literal!("fixed vars")))?;
        BackendDump::dumpVarList(&failed_var_lst, &(literal!("failed vars")))?;
    }
    syst = BackendDAEUtil::setEqSystEqs(syst, BackendEquation::sortInitialEqns(new_eqns)?);
    Ok(syst)
}

fn fixInitialSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut dummy: i32,
    mut initVars: BackendDAE::Variables,
    mut dumpVars: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Var>>,
    mut removedEqns: DoubleEnded::MutableList<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
)> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut eqns2: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut dumpVars2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut removedEqns2: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut nAddEqs: i32;
    let mut nAddVars: i32;
    let mut stateIndices: metamodelica::List<i32>;
    let mut range: metamodelica::List<i32>;
    let mut redundantEqns: metamodelica::List<i32>;
    let mut initVarList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut ass1: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut ass2: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut m_: metamodelica::Array<metamodelica::List<i32>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = BackendDAEUtil::createEqSystem(
        inEqSystem.orderedVars.clone(),
        inEqSystem.orderedEqs.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut mapIncRowEqn: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut perfectMatching: bool;
    let mut maxMixedDeterminedIndex: i32 = intMax(0, Flags::getConfigInt(Flags::MAX_MIXED_DETERMINED_INDEX.clone())?);
    let mut eMarks: metamodelica::Array<bool> = arrayCreate(0, false);
    let mut vMarks: metamodelica::Array<bool> = arrayCreate(0, false);
    let mut singular_eqns_idx: metamodelica::List<i32>;
    let mut singular_vars_idx: metamodelica::List<i32>;
    let mut overDetIndex: i32;
    let mut underDetIndex: i32;
    let mut scalarEqnSize: i32;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let debug: bool = false;
    for mut index in 0..=maxMixedDeterminedIndex {
        nVars = BackendVariable::varsSize(&inEqSystem.orderedVars);
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        syst = BackendDAEUtil::createEqSystem(
            inEqSystem.orderedVars.clone(),
            inEqSystem.orderedEqs.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        funcs = BackendDAEUtil::getFunctions(&inShared);
        (m_, _, _, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
            &syst,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(funcs),
            BackendDAEUtil::isInitializationDAE(&inShared),
        )?;
        if debug {
            BackendDump::dumpEqSystem(syst.clone(), &(literal!("fixInitialSystem")))?;
            BackendDump::dumpVariables(&initVars, &(literal!("selected initialization variables")))?;
            BackendDump::dumpVariables(&inEqSystem.orderedVars, &(literal!("vars in the system")))?;
            BackendDump::dumpAdjacencyMatrix(m_.clone())?;
        }
        stateIndices =
            BackendVariable::getVarIndexFromVariablesIndexInFirstSet(inEqSystem.orderedVars.clone(), initVars.clone())?;
        nAddEqs = intMax(nVars - nEqns + index, index);
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("nAddEqs: "));
                __mm_s.push_str(&*intString(nAddEqs));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        m = fixUnderDeterminedSystem(m_.clone(), stateIndices, nEqns, nAddEqs)?;
        nAddVars = intMax(nEqns - nVars + index, index);
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("nAddVars: "));
                __mm_s.push_str(&*intString(nAddVars));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        m = fixOverDeterminedSystem(m.clone(), inEqSystem.orderedEqs.clone(), nVars, nAddVars)?;
        (ass1, ass2, perfectMatching, eMarks, vMarks) =
            Matching::RegularMatching(m.clone(), nVars + nAddVars, nEqns + nAddEqs)?;
        if debug {
            BackendDump::dumpMatchingVars(ass1.clone())?;
            BackendDump::dumpMatchingEqns(ass2.clone())?;
        }
        if perfectMatching {
            if index > 0 {
                Error::addCompilerNotification({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("The given system is mixed-determined.   [index = "));
                    __mm_s.push_str(&*intString(index));
                    __mm_s.push_str(&*literal!("]"));
                    ArcStr::from(__mm_s)
                })?;
            }
            if nAddVars > 0 {
                range = List::intRange2(nVars + 1, nVars + nAddVars);
                redundantEqns = mapIndices(range, ass1.clone())?;
                consistencyCheck(
                    &redundantEqns,
                    inEqSystem.orderedEqs.clone(),
                    &(inEqSystem.orderedVars.clone()),
                    &(inShared.clone()),
                    nAddVars,
                    m_.clone(),
                    &(enhancedRows(&syst, inShared)),
                    ass1.clone(),
                    ass2.clone(),
                    mapIncRowEqn.clone(),
                )?;
                removedEqns2 = BackendEquation::getList(redundantEqns.clone(), inEqSystem.orderedEqs.clone())?;
                eqns2 = BackendEquation::deleteList(inEqSystem.orderedEqs.clone(), &redundantEqns)?;
                DoubleEnded::push_list_back(removedEqns, &removedEqns2)?;
            } else {
                eqns2 = inEqSystem.orderedEqs.clone();
            }
            if nAddEqs > 0 {
                range = List::intRange2(nEqns + 1, nEqns + nAddEqs);
                range = mapIndices(range, ass2.clone())?;
                initVarList = List::map1r(
                    range,
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    inEqSystem.orderedVars.clone(),
                )?;
                (eqns2, dumpVars2) = addStartValueEquations(&initVarList, eqns2, metamodelica::nil())?;
                DoubleEnded::push_list_back(dumpVars, &dumpVars2)?;
            }
            outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem, eqns2);
            return Ok((outEqSystem, outShared, dummy));
        }
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("index-"));
                __mm_s.push_str(&*intString(index));
                __mm_s.push_str(&*literal!(" ende\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        overDetIndex = (({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=metamodelica::arrayLength(ass1.clone())).into_iter() {
                if !(({
                    let __elt = (*metamodelica::index_checked(&ass1.borrow(), i.clone())?).clone();
                    __elt
                }) < 0)
                {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .len() as i32);
        underDetIndex = (({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=metamodelica::arrayLength(ass2.clone())).into_iter() {
                if !(({
                    let __elt = (*metamodelica::index_checked(&ass2.borrow(), i.clone())?).clone();
                    __elt
                }) < 0)
                {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .len() as i32);
        singular_eqns_idx = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=metamodelica::arrayLength(mapIncRowEqn.clone())).into_iter() {
                if !({
                    let __elt = (*metamodelica::index_checked(&eMarks.borrow(), i.clone())?).clone();
                    __elt
                }) {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        singular_vars_idx = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=BackendVariable::varsSize(&syst.orderedVars)).into_iter() {
                if !({
                    let __elt = (*metamodelica::index_checked(&vMarks.borrow(), i.clone())?).clone();
                    __elt
                }) {
                    continue;
                }
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        scalarEqnSize = ((singular_eqns_idx).len() as i32);
        singular_eqns_idx = List::uniqueOnTrue(
            &({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (singular_eqns_idx).into_iter().cloned() {
                    let __x = ({
                        let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), i.clone())?).clone();
                        __elt
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            &fnptr!(intEq, i32, i32),
        )?;
        metamodelica::print(literal!("\n------------ UNBALANCED INITIAL SYSTEM ------------\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "The initial system is over- as well as underdetermined and it could not be resolved after "
            ));
            __mm_s.push_str(&*intString(maxMixedDeterminedIndex));
            __mm_s.push_str(&*literal!(" iterations.\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("==== OVERDETERMINATION BY "));
            __mm_s.push_str(&*intString(overDetIndex));
            __mm_s.push_str(&*literal!(" EQUATION(S)\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("==== UNDERDETERMINATION OF "));
            __mm_s.push_str(&*intString(underDetIndex));
            __mm_s.push_str(&*literal!(" VARIABLE(S)\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n---- involved set eqns ("));
            __mm_s.push_str(&*intString(scalarEqnSize));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*intString(((singular_eqns_idx).len() as i32)));
            __mm_s.push_str(&*literal!("):\n"));
            ArcStr::from(__mm_s)
        });
        for mut eqn in &*singular_eqns_idx {
            eq = BackendEquation::get(
                syst.orderedEqs.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&mapIncRowEqn.borrow(), eqn.clone())?).clone();
                    __elt
                }),
            )?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("  "));
                __mm_s.push_str(&*intString(eqn.clone()));
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(BackendEquation::equationSize(&eq)?));
                __mm_s.push_str(&*literal!("):\t"));
                __mm_s.push_str(&*BackendDump::equationString(&eq)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n---- involved set vars ("));
            __mm_s.push_str(&*intString(((singular_vars_idx).len() as i32)));
            __mm_s.push_str(&*literal!("):\n"));
            ArcStr::from(__mm_s)
        });
        for mut var in &*singular_vars_idx {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("  "));
                __mm_s.push_str(&*intString(var.clone()));
                __mm_s.push_str(&*literal!(":\t"));
                __mm_s.push_str(&*BackendDump::varString(
                    &(BackendVariable::getVarAt(&syst.orderedVars, var.clone())?),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("--------------------------------------------------\n"));
    }
    Error::addMessage(
        Error::MIXED_DETERMINED.clone(),
        list![intString(maxMixedDeterminedIndex)],
    )?;
    return Err("fail");
    Ok((outEqSystem, outShared, dummy))
}

fn updateFixedAttribute(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut vars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut vars: BackendDAE::Variables = vars;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = BackendVariable::varCref(&var);
    if BackendVariable::containsCref(cr, &vars) {
        var = BackendVariable::setVarFixed(var, true)?;
    }
    Ok((var, vars))
}

fn fixUnderDeterminedSystem(
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut inInitVarIndices: metamodelica::List<i32>,
    mut inNEqns: i32,
    mut inNAddEqns: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<metamodelica::List<i32>>;
    let mut newEqIndices: metamodelica::List<i32>;
    if inNAddEqns < 0 {
        Error::addInternalError(
            literal!("function fixUnderDeterminedSystem failed due to invalid input"),
            metamodelica::sourceInfo!("BackEnd/Initialization.mo"),
        )?;
        return Err("fail");
    }
    if inNAddEqns > 0 {
        outM = arrayCreate(inNEqns + inNAddEqns, metamodelica::nil());
        outM = Array::copy(inM.clone(), outM.clone())?;
        newEqIndices = List::intRange2(inNEqns + 1, inNEqns + inNAddEqns);
        outM = List::fold1(&newEqIndices, &squareAdjacencyMatrix1, inInitVarIndices, outM.clone())?;
    } else {
        outM = metamodelica::arrayFromVec(inM.clone().borrow().clone());
    }
    Ok(outM)
}

fn squareAdjacencyMatrix1(
    mut inPos: i32,
    mut inDependency: metamodelica::List<i32>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<metamodelica::List<i32>> = inM;
    {
        let __cell0 = inDependency;
        let __idx0 = inPos;
        *metamodelica::index_mut_checked(&mut outM.clone().borrow_mut(), __idx0)? = __cell0;
    }
    Ok(outM)
}

fn fixOverDeterminedSystem(
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inNVars: i32,
    mut inNAddVars: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<metamodelica::List<i32>>;
    let mut newVarIndices: metamodelica::List<i32>;
    let mut initEqsIndices: metamodelica::List<i32>;
    if inNAddVars < 0 {
        Error::addInternalError(
            literal!("function fixOverDeterminedSystem failed due to invalid input"),
            metamodelica::sourceInfo!("BackEnd/Initialization.mo"),
        )?;
        return Err("fail");
    }
    if inNAddVars > 0 {
        initEqsIndices = getInitEqIndices(&(BackendEquation::equationList(orderedEqs)?))?;
        newVarIndices = List::intRange2(inNVars + 1, inNVars + inNAddVars);
        outM = List::fold1(&initEqsIndices, &squareAdjacencyMatrix2, newVarIndices, inM.clone())?;
    } else {
        outM = inM.clone();
    }
    Ok(outM)
}

fn squareAdjacencyMatrix2(
    mut inPos: i32,
    mut inRange: metamodelica::List<i32>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<metamodelica::List<i32>> = inM.clone();
    {
        let __cell0 = listAppend(
            ({
                let __elt = (*metamodelica::index_checked(&inM.borrow(), inPos)?).clone();
                __elt
            }),
            inRange,
        );
        let __idx0 = inPos;
        *metamodelica::index_mut_checked(&mut outM.clone().borrow_mut(), __idx0)? = __cell0;
    }
    Ok(outM)
}

fn addStartValueEquations(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        inEqns;
    let mut outDumpVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inDumpVars;
    let mut dumpVar: metamodelica::Ref<BackendDAE::Var>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut crefExp: metamodelica::Ref<DAE::Exp>;
    let mut startExp: metamodelica::Ref<DAE::Exp>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut isPreCref: bool;
    for mut var in &**inVarLst {
        cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
        tp = BackendVariable::varType(metamodelica::AsArg::as_arg(&var));
        crefExp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: cref.clone(),
            ty: tp,
        });
        isPreCref = ComponentReference::isPreCref(&cref);
        if isPreCref {
            cref = ComponentReference::popPreCref(cref);
        }
        e = Expression::crefExp(cref.clone())?;
        tp = Expression::r#typeof(e)?;
        startExp = Expression::crefExp(ComponentReference::crefPrefixStart(cref.clone()))?;
        eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: crefExp,
            scalar: startExp,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(),
        });
        outEqns = BackendEquation::add(eqn, outEqns)?;
        if isPreCref {
            dumpVar = BackendVariable::copyVarNewName(cref, var.clone());
            dumpVar = BackendVariable::setVarFixed(dumpVar, true)?;
            outDumpVars = metamodelica::cons(dumpVar, outDumpVars);
        } else {
            dumpVar = BackendVariable::setVarFixed(var.clone(), true)?;
            outDumpVars = metamodelica::cons(dumpVar, outDumpVars);
        }
    }
    Ok((outEqns, outDumpVars))
}

// =============================================================================
// section for symbolic consistency check
//
// =============================================================================
/// The enhanced adjacency rows of a system, computed for the equations the
///  consistency check marks; the whole matrix differentiates every equation.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EnhancedRows {
    pub vars: BackendDAE::Variables,
    pub eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    pub shared: metamodelica::Ref<BackendDAE::Shared>,
    pub rows: metamodelica::Array<
        Option<
            metamodelica::List<(
                i32,
                BackendDAE::Solvability,
                metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
            )>,
        >,
    >,
    pub rowmark: metamodelica::Array<i32>,
}

impl metamodelica::gc::MMTrace for EnhancedRows {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shared, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rows, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rowmark, __mmv)?;
        Ok(())
    }
}
impl Default for EnhancedRows {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            eqns: Default::default(),
            shared: Default::default(),
            rows: Default::default(),
            rowmark: Default::default(),
        }
    }
}

pub type ENHANCED_ROWS = EnhancedRows;

fn enhancedRows(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> EnhancedRows {
    let mut me: EnhancedRows;
    me = EnhancedRows {
        vars: syst.orderedVars.clone(),
        eqns: syst.orderedEqs.clone(),
        shared: shared,
        rows: arrayCreate(BackendEquation::getNumberOfEquations(syst.orderedEqs.clone()), None),
        rowmark: arrayCreate(BackendVariable::varsSize(&syst.orderedVars), 0),
    };
    me
}

fn enhancedRow(
    mut me: &EnhancedRows,
    mut eqn: i32,
) -> Result<
    metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
> {
    let mut row: metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>;
    row = (::match_deref::match_deref! { match &(metamodelica::arrayGet(me.rows.clone(), eqn)?) {
        Some(__esc_row) => {
            row = (*__esc_row).clone();
            row.clone()
        },
        _ => {
            (row, _, _) = BackendDAEUtil::adjacencyRowEnhanced(me.vars.clone(), &(BackendEquation::get(me.eqns.clone(), eqn)?), eqn, me.rowmark.clone(), &me.shared.globalKnownVars, false, &me.shared)?;
            metamodelica::arrayUpdate(me.rows.clone(), eqn, Some(row.clone()))?;
            row
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(row)
}

fn consistencyCheck(
    mut inRedundantEqns: &metamodelica::List<i32>,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inVars: &BackendDAE::Variables,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut nAddVars: i32,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut me: &EnhancedRows,
    mut vecVarToEqs: metamodelica::Array<i32>,
    mut vecEqsToVar: metamodelica::Array<i32>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut outConsistentEquations: metamodelica::List<i32>;
    let mut outInconsistentEquations: metamodelica::List<i32>;
    let mut outUncheckedEquations: metamodelica::List<i32>;
    (outConsistentEquations, outInconsistentEquations, outUncheckedEquations) = 'mc: {
        let __mc_input = &**inRedundantEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut outRange: metamodelica::List<i32>;
                    let mut flatComps: metamodelica::List<i32>;
                    let mut markedComps: metamodelica::List<i32>;
                    let mut outLoopListComps: metamodelica::List<i32>;
                    let mut consistentEquations: metamodelica::List<i32>;
                    let mut consistentEquations2: metamodelica::List<i32>;
                    let mut inconsistentEquations: metamodelica::List<i32>;
                    let mut uncheckedEquations: metamodelica::List<i32>;
                    let mut uncheckedEquations2: metamodelica::List<i32>;
                    let mut nEqns: i32;
                    let mut redundantEqn: i32;
                    let mut comps: metamodelica::List<metamodelica::List<i32>>;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut substEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    nEqns = BackendEquation::equationArraySize(inEqns.clone())?;
                    comps = Sorting::Tarjan(inM.clone(), vecVarToEqs.clone(), nEqns)?;
                    flatComps = List::flatten(comps.clone())?;
                    (_, outLoopListComps) = splitStrongComponents(&comps);
                    redundantEqn = mapIndex(currRedundantEqn.clone(), mapIncRowEqn.clone())?;
                    flatComps = mapIndices(flatComps.clone(), mapIncRowEqn.clone())?;
                    outLoopListComps = mapIndices(outLoopListComps.clone(), mapIncRowEqn.clone())?;
                    markedComps = compsMarker(currRedundantEqn.clone(), vecVarToEqs.clone(), inM.clone(), flatComps.clone(), outLoopListComps.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = setupVarReplacements(&markedComps, inEqns.clone(), inVars, vecEqsToVar.clone(), &repl, mapIncRowEqn.clone(), me, inShared);
                    substEqns = applyVarReplacements(redundantEqn, inEqns.clone(), &repl)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getConsistentEquation(redundantEqn, substEqns.clone(), inEqns.clone(), inM.clone(), vecVarToEqs.clone(), inVars.clone(), inShared, 1)?) {
                        (__pa0, true, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    outRange = metamodelica::Own::own(__pa0);
                    uncheckedEquations = metamodelica::Own::own(__pa1);
                    (consistentEquations, inconsistentEquations, uncheckedEquations2) = consistencyCheck(metamodelica::AsArg::as_arg(&restRedundantEqns), inEqns.clone(), inVars, inShared, nAddVars, inM.clone(), me, vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    consistentEquations2 = listAppend(consistentEquations.clone(), outRange.clone());
                    uncheckedEquations2 = listAppend(uncheckedEquations.clone(), uncheckedEquations2.clone());
                    Ok((consistentEquations2.clone(), inconsistentEquations.clone(), uncheckedEquations2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut consistentEquations: metamodelica::List<i32>;
                    let mut inconsistentEquations: metamodelica::List<i32>;
                    let mut uncheckedEquations: metamodelica::List<i32>;
                    (consistentEquations, inconsistentEquations, uncheckedEquations) = consistencyCheck(metamodelica::AsArg::as_arg(&restRedundantEqns), inEqns.clone(), inVars, inShared, nAddVars, inM.clone(), me, vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    Ok((consistentEquations.clone(), metamodelica::cons(currRedundantEqn.clone(), inconsistentEquations.clone()), uncheckedEquations.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outConsistentEquations, outInconsistentEquations, outUncheckedEquations))
}

fn isVarExplicitSolvable<'__b>(
    mut inElem: &'__b metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
    mut inVarID: i32,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inElem {
            Deref @ metamodelica::ListNode::Nil => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. }, _), tail: _ } if (intEq(id.clone(), inVarID)) => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. }, _), tail: _ } if (intEq(id.clone(), inVarID)) => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: (_, _, _), tail: elem } => {
                let mut b: bool;
                { (inElem, inVarID) = (elem, inVarID); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn splitStrongComponents(
    mut inComps: &metamodelica::List<metamodelica::List<i32>>,
) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut outListComps: metamodelica::List<i32>;
    let mut outLoopListComps: metamodelica::List<i32>;
    (outListComps, outLoopListComps) = (::match_deref::match_deref! { match inComps {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: currIndex, tail: Deref @ metamodelica::ListNode::Nil }, tail: restComps } => {
            let mut listComps: metamodelica::List<i32>;
            let mut loopListComps: metamodelica::List<i32>;
            (listComps, loopListComps) = splitStrongComponents(restComps);
            (metamodelica::cons(currIndex.clone(), listComps), loopListComps)
        },
        Deref @ metamodelica::ListNode::Cons { head: currComp, tail: restComps } => {
            let mut listComps: metamodelica::List<i32>;
            let mut loopListComps: metamodelica::List<i32>;
            (listComps, loopListComps) = splitStrongComponents(restComps);
            loopListComps = listAppend(currComp.clone(), loopListComps);
            (listComps, loopListComps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outListComps, outLoopListComps)
}

fn mapIndex(mut inIndex: i32, mut inMapping: metamodelica::Array<i32>) -> Result<i32> {
    let __ab_inMapping = inMapping.borrow();
    let mut outIndex: i32;
    outIndex = (*metamodelica::index_checked(&__ab_inMapping, inIndex)?).clone();
    Ok(outIndex)
}

fn mapIndices(
    mut inIndices: metamodelica::List<i32>,
    mut inMapping: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outIndices: metamodelica::List<i32>;
    outIndices = List::map1(inIndices, &mapIndex, inMapping.clone())?;
    Ok(outIndices)
}

fn mapListIndices(
    mut inListIndices: metamodelica::List<metamodelica::List<i32>>,
    mut inMapping: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outListIndices: metamodelica::List<metamodelica::List<i32>>;
    outListIndices = List::map1(inListIndices, &mapIndices, inMapping.clone())?;
    Ok(outListIndices)
}

fn compsMarker(
    mut inUnassignedEqn: i32,
    mut inVecVarToEq: metamodelica::Array<i32>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut inFlatComps: metamodelica::List<i32>,
    mut inLoopListComps: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outMarkedEqns: metamodelica::List<i32>;
    let mut varList: metamodelica::List<i32>;
    let mut markedEqns: metamodelica::List<i32>;
    match '__try0: {
        let false = (listMember(inUnassignedEqn, inLoopListComps.clone())) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        varList = ({
            let __elt =
                (*unwrap_break_err!(metamodelica::index_checked(&inM.borrow(), inUnassignedEqn), '__try0)).clone();
            __elt
        });
        markedEqns = unwrap_break_err!(compsMarker2(&varList, inVecVarToEq.clone(), inM.clone(), &inFlatComps, &(metamodelica::nil()), &inLoopListComps), '__try0);
        outMarkedEqns = unwrap_break_err!(downCompsMarker(&(inFlatComps.clone().reverse()), inVecVarToEq.clone(), inM.clone(), &(inFlatComps.clone()), markedEqns.clone(), &inLoopListComps), '__try0);
        Ok::<_, &'static str>((markedEqns.clone(), outMarkedEqns.clone(), varList.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            markedEqns = __try0_o0;
            outMarkedEqns = __try0_o1;
            varList = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addCompilerNotification(literal!(
                "It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet."
            ))?;
            return Err(__try0_err);
        }
    }
    Ok(outMarkedEqns)
}

fn compsMarker2(
    mut inVarList: &metamodelica::List<i32>,
    mut inVecVarToEq: metamodelica::Array<i32>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut inFlatComps: &metamodelica::List<i32>,
    mut inMarkedEqns: &metamodelica::List<i32>,
    mut inLoopListComps: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut outMarkedEqns: metamodelica::List<i32>;
    outMarkedEqns = 'mc: {
        let __mc_input = &**inVarList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inMarkedEqns.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32;
                    let mut markedEqns: metamodelica::List<i32>;
                    indexEq = ({let __elt = (*metamodelica::index_checked(&inVecVarToEq.borrow(), indexVar.clone())?).clone(); __elt});
                    let false = (listMember(indexEq, inLoopListComps.clone())) else { return Err("pattern mismatch") };
                    let false = (listMember(indexEq, inMarkedEqns.clone())) else { return Err("pattern mismatch") };
                    markedEqns = compsMarker2(metamodelica::AsArg::as_arg(&var_list2), inVecVarToEq.clone(), inM.clone(), inFlatComps, inMarkedEqns, inLoopListComps)?;
                    Ok(metamodelica::cons(indexEq, markedEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32;
                    let mut markedEqns: metamodelica::List<i32>;
                    indexEq = ({let __elt = (*metamodelica::index_checked(&inVecVarToEq.borrow(), indexVar.clone())?).clone(); __elt});
                    let false = (listMember(indexEq, inLoopListComps.clone())) else { return Err("pattern mismatch") };
                    let true = (listMember(indexEq, inMarkedEqns.clone())) else { return Err("pattern mismatch") };
                    markedEqns = compsMarker2(metamodelica::AsArg::as_arg(&var_list2), inVecVarToEq.clone(), inM.clone(), inFlatComps, inMarkedEqns, inLoopListComps)?;
                    Ok(markedEqns.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addCompilerNotification(literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet."))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMarkedEqns)
}

fn downCompsMarker(
    mut unassignedEqns: &metamodelica::List<i32>,
    mut vecVarToEq: metamodelica::Array<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut flatComps: &metamodelica::List<i32>,
    mut inMarkedEqns: metamodelica::List<i32>,
    mut inLoopListComps: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut inMarkedEqns: metamodelica::List<i32> = inMarkedEqns;
    for mut indexUnassigned in &**unassignedEqns {
        if listMember(indexUnassigned.clone(), inMarkedEqns.clone()) {
            inMarkedEqns = compsMarker2(
                &({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), indexUnassigned.clone())?).clone();
                    __elt
                }),
                vecVarToEq.clone(),
                m.clone(),
                flatComps,
                &inMarkedEqns,
                inLoopListComps,
            )?;
        }
    }
    Ok(inMarkedEqns)
}

fn setupVarReplacements(
    mut inMarkedEqns: &metamodelica::List<i32>,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inVars: &BackendDAE::Variables,
    mut inVecEqToVar: metamodelica::Array<i32>,
    mut inRepls: &BackendVarTransform::VariableReplacements,
    mut inMapIncRowEqn: metamodelica::Array<i32>,
    mut inME: &EnhancedRows,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> BackendVarTransform::VariableReplacements {
    let mut outRepls: BackendVarTransform::VariableReplacements;
    outRepls = 'mc: {
        let __mc_input = &**inMarkedEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inRepls.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: markedEqn, tail: markedEqns } => {
                    let mut indexVar: i32;
                    let mut indexEq: i32;
                    let mut repls: BackendVarTransform::VariableReplacements;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut varName: metamodelica::Ref<DAE::ComponentRef>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut type_: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Exp>;
                    indexVar = ({let __elt = (*metamodelica::index_checked(&inVecEqToVar.borrow(), markedEqn.clone())?).clone(); __elt});
                    indexEq = ({let __elt = (*metamodelica::index_checked(&inMapIncRowEqn.borrow(), markedEqn.clone())?).clone(); __elt});
                    let true = (isVarExplicitSolvable(&(enhancedRow(inME, indexEq)?), indexVar)) else { return Err("pattern mismatch") };
                    var = BackendVariable::getVarAt(inVars, indexVar)?;
                    eqn = BackendEquation::get(inEqns.clone(), indexEq)?;
                    cref = BackendVariable::varCref(&var);
                    type_ = BackendVariable::varType(&var);
                    x = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: type_.clone() });
                    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eqn.clone(), x.clone(), Some(inShared.functionTree.clone()))?) {
                        __pa1 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    eqn = metamodelica::Own::own(__pa1);
                    varName = BackendVariable::varCref(&var);
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), &fnptr!(BackendDAEUtil::replaceCrefsWithValues, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::Ref<DAE::ComponentRef>)), (inVars.clone(), varName.clone()))?;
                    repls = BackendVarTransform::addReplacement(inRepls.clone(), varName.clone(), exp1.clone(), None)?;
                    repls = setupVarReplacements(metamodelica::AsArg::as_arg(&markedEqns), inEqns.clone(), inVars, inVecEqToVar.clone(), &repls, inMapIncRowEqn.clone(), inME, inShared);
                    Ok(repls.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: markedEqns } => {
                    let mut repls: BackendVarTransform::VariableReplacements;
                    repls = setupVarReplacements(metamodelica::AsArg::as_arg(&markedEqns), inEqns.clone(), inVars, inVecEqToVar.clone(), inRepls, inMapIncRowEqn.clone(), inME, inShared);
                    Ok(repls.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outRepls
}

fn applyVarReplacements(
    mut inEqnIndex: i32,
    mut inEqnList: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inVarRepls: &BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut outEqnList: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    outEqnList = BackendEquation::copyEquationArray(inEqnList);
    eqn = BackendEquation::get(outEqnList.clone(), inEqnIndex)?;
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqn], inVarRepls, None)?) {
        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eqn = metamodelica::Own::own(__pa0);
    outEqnList = BackendEquation::setAtIndex(outEqnList, inEqnIndex, eqn)?;
    Ok(outEqnList)
}

fn getConsistentEquation(
    mut inUnassignedEqn: i32,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inEqnsOrig: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
    mut vecVarToEqs: metamodelica::Array<i32>,
    mut vars: BackendDAE::Variables,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut counter: i32,
) -> Result<(metamodelica::List<i32>, bool, metamodelica::List<i32>)> {
    let mut outUnassignedEqns: metamodelica::List<i32>;
    let mut outConsistent: bool;
    let mut outRemovedEqns: metamodelica::List<i32>;
    (outUnassignedEqns, outConsistent, outRemovedEqns) = 'mc: {
        let __mc_input = inUnassignedEqn;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            nVars = BackendVariable::varsSize(&vars);
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns - nVars)) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            lhs = metamodelica::Own::own(__pa0);
            rhs = metamodelica::Own::own(__pa1);
            exp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: lhs.clone(),
                operator: DAE::Operator::SUB {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: rhs.clone(),
            });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let true = (Expression::isZero(&exp)?) else {
                return Err("pattern mismatch");
            };
            BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Ok((list![inUnassignedEqn], true, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            nVars = BackendVariable::varsSize(&vars);
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intGt(counter, nEqns - nVars)) else {
                return Err("pattern mismatch");
            };
            Error::addCompilerError(literal!(
                "Initialization problem is structural singular. Please, check the initial conditions."
            ))?;
            Ok((metamodelica::nil(), true, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqn2: metamodelica::Ref<BackendDAE::Equation>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut listParameter: metamodelica::List<ArcStr>;
            nVars = BackendVariable::varsSize(&vars);
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns - nVars)) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            lhs = metamodelica::Own::own(__pa0);
            rhs = metamodelica::Own::own(__pa1);
            exp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: lhs.clone(),
                operator: DAE::Operator::SUB {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: rhs.clone(),
            });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(&exp)?) else {
                return Err("pattern mismatch");
            };
            let __pa2 = ::match_deref::match_deref! { match &(parameterCheck(exp.clone())?) {
                (__pa2, false) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            listParameter = metamodelica::Own::own(__pa2);
            let true = ((listParameter).is_empty()) else {
                return Err("pattern mismatch");
            };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "The initialization problem is inconsistent due to the following equation: "
                ));
                __mm_s.push_str(&*BackendDump::equationString(&eqn2)?);
                __mm_s.push_str(&*literal!(" ("));
                __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut listVar: metamodelica::List<i32>;
            let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut listParameter: metamodelica::List<ArcStr>;
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut system: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut list_inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            nVars = BackendVariable::varsSize(&vars);
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns - nVars)) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            lhs = metamodelica::Own::own(__pa0);
            rhs = metamodelica::Own::own(__pa1);
            exp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: lhs.clone(),
                operator: DAE::Operator::SUB {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: rhs.clone(),
            });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(&exp)?) else {
                return Err("pattern mismatch");
            };
            (listParameter, _) = parameterCheck(exp.clone())?;
            let false = ((listParameter).is_empty()) else {
                return Err("pattern mismatch");
            };
            list_inEqns = BackendEquation::equationList(inEqns.clone())?;
            list_inEqns = List::set(list_inEqns.clone(), inUnassignedEqn, eqn.clone())?;
            eqns = BackendEquation::listEquation(&list_inEqns)?;
            funcs = BackendDAEUtil::getFunctions(shared);
            system = BackendDAEUtil::createEqSystem(
                vars.clone(),
                eqns.clone(),
                metamodelica::nil(),
                openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                BackendEquation::emptyEqns(),
            );
            (m, _) = BackendDAEUtil::adjacencyMatrix(
                &system,
                openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                Some(funcs.clone()),
                BackendDAEUtil::isInitializationDAE(shared),
            )?;
            listVar = ({
                let __elt = (*metamodelica::index_checked(&m.borrow(), inUnassignedEqn)?).clone();
                __elt
            });
            let false = ((listVar).is_empty()) else {
                return Err("pattern mismatch");
            };
            BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerNotification(literal!(
                "It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet."
            ))?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqn2: metamodelica::Ref<BackendDAE::Equation>;
            let mut lhs: metamodelica::Ref<DAE::Exp>;
            let mut rhs: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut listParameter: metamodelica::List<ArcStr>;
            let mut anyStartValue: bool;
            nVars = BackendVariable::varsSize(&vars);
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns - nVars)) else {
                return Err("pattern mismatch");
            };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            lhs = metamodelica::Own::own(__pa0);
            rhs = metamodelica::Own::own(__pa1);
            exp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: lhs.clone(),
                operator: DAE::Operator::SUB {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: rhs.clone(),
            });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(&exp)?) else {
                return Err("pattern mismatch");
            };
            (listParameter, anyStartValue) = parameterCheck(exp.clone())?;
            let true = (!((listParameter).is_empty()) || anyStartValue) else {
                return Err("pattern mismatch");
            };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("It was not possible to determine if the initialization problem is consistent, because of not evaluable parameters/start values during compile time: "));
                __mm_s.push_str(&*BackendDump::equationString(&eqn2)?);
                __mm_s.push_str(&*literal!(" ("));
                __mm_s.push_str(&*BackendDump::equationString(&eqn)?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })?;
            Ok((metamodelica::nil(), true, list![inUnassignedEqn]))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outUnassignedEqns, outConsistent, outRemovedEqns))
}

fn parameterCheck(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::List<ArcStr>, bool)> {
    let mut outParameters: metamodelica::List<ArcStr>;
    let mut outAnyStartValue: bool;
    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(inExp, &parameterCheck2, (metamodelica::nil(), false))?;
    outParameters = metamodelica::Own::own(__pa0);
    outAnyStartValue = metamodelica::Own::own(__pa1);
    Ok((outParameters, outAnyStartValue))
}

fn parameterCheck2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inParams: (metamodelica::List<ArcStr>, bool),
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, (metamodelica::List<ArcStr>, bool))> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outParams: (metamodelica::List<ArcStr>, bool);
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut parameters: metamodelica::List<ArcStr>;
    let mut anyStartValue: bool;
    (parameters, anyStartValue) = inParams.clone();
    (outParams, outContinue) = (match &*inExp {
        DAE::Exp::CREF {
            componentRef: __esc_componentRef,
            ..
        } => {
            componentRef = (*__esc_componentRef).clone();
            if ComponentReference::isStartCref(metamodelica::AsArg::as_arg(&componentRef)) {
                anyStartValue = true;
            } else {
                parameters = metamodelica::cons(
                    ComponentReference::crefStr(metamodelica::AsArg::as_arg(&componentRef))?,
                    parameters,
                );
            }
            ((parameters, anyStartValue), !(anyStartValue))
        }
        _ => (inParams, true),
    });
    Ok((outExp, outContinue, outParams))
}

// =============================================================================
// section for introducing pre-variables for alias variables
//
// =============================================================================
fn introducePreVarsForAliasVariables(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ty, arryDim, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool;
                    let mut isFixed: bool;
                    let mut startValue: metamodelica::Ref<DAE::Exp>;
                    let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                    let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                    isFixed = BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var));
                    startValue = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = metamodelica::Ref::new(BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue.clone()))?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed && isFixed) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varType: ty, arryDim, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool;
                    let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                    let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = metamodelica::Ref::new(BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false });
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
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

// =============================================================================
// section for collecting initial vars/eqns
//
// =============================================================================
fn collectInitialVarsEqnsSystem(
    mut eqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut vars: BackendDAE::Variables,
    mut fixVars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut reEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut allPrimaryParams: metamodelica::Ref<AvlSetCR::Tree>,
    mut datareconFlag: bool,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut fixVars: BackendDAE::Variables = fixVars;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = eqns;
    let mut reEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        reEqns;
    let mut stateSetFixCounts: metamodelica::Array<i32>;
    for mut eq in &**eqSystems {
        let () = (match &*eq.clone() {
            BackendDAE::EqSystem {
                partitionKind: BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { .. },
                ..
            } => {
                (vars, eqns) = BackendVariable::traverseBackendDAEVars(
                    eq.orderedVars.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<BackendDAE::Var>,
                              __a1: (
                            BackendDAE::Variables,
                            metamodelica::Ref<
                                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                            >,
                        )| collectInitialClockedVarsEqns(__a0, &__a1),
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<BackendDAE::Var>,
                                    (
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                    ),
                                ) -> Result<(
                                    metamodelica::Ref<BackendDAE::Var>,
                                    (
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                    ),
                                )> + 'static,
                        >),
                    (vars, eqns),
                )?;
                ()
            }
            _ => {
                stateSetFixCounts = arrayCreate(((eq.stateSets).len() as i32), 0);
                (vars, fixVars, eqns, stateSetFixCounts, _, _, _) = BackendVariable::traverseBackendDAEVars(
                    eq.orderedVars.clone(),
                    (std::sync::Arc::new(collectInitialVars)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<BackendDAE::Var>,
                                    (
                                        BackendDAE::Variables,
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                        metamodelica::Array<i32>,
                                        (
                                            metamodelica::Array<
                                                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
                                            >,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                            ),
                                            i32,
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                            ),
                                        ),
                                        metamodelica::Ref<AvlSetCR::Tree>,
                                        bool,
                                    ),
                                ) -> Result<(
                                    metamodelica::Ref<BackendDAE::Var>,
                                    (
                                        BackendDAE::Variables,
                                        BackendDAE::Variables,
                                        metamodelica::Ref<
                                            ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                        >,
                                        metamodelica::Array<i32>,
                                        (
                                            metamodelica::Array<
                                                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
                                            >,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                            ),
                                            i32,
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::ComponentRef>,
                                                        )
                                                            -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                            ),
                                        ),
                                        metamodelica::Ref<AvlSetCR::Tree>,
                                        bool,
                                    ),
                                )> + 'static,
                        >),
                    (
                        vars,
                        fixVars,
                        eqns,
                        stateSetFixCounts.clone(),
                        hs.clone(),
                        allPrimaryParams.clone(),
                        datareconFlag,
                    ),
                )?;
                (eqns, reEqns) =
                    BackendEquation::traverseEquationArray(eq.orderedEqs.clone(), &collectInitialEqns, (eqns, reEqns))?;
                if Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                    (vars, eqns) = collectInitialStateSets(&eq.stateSets, stateSetFixCounts.clone(), vars, eqns)?;
                }
                GCExt::free(stateSetFixCounts.clone());
                ()
            }
        });
    }
    Ok((vars, fixVars, eqns, reEqns))
}

fn collectInitialStateSets(
    mut stateSets: &metamodelica::List<BackendDAE::StateSet>,
    mut stateSetFixCounts: metamodelica::Array<i32>,
    mut iVars: BackendDAE::Variables,
    mut iEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut oEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut stateSet: BackendDAE::StateSet = <BackendDAE::StateSet as ::std::default::Default>::default();
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut initEqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut crLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut statesToFix: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut unfixedStates: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut toFix: i32;
    let mut recordSize: Option<i32>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    (oVars, oEqns) = (iVars, iEqns);
    for mut stateSet in &**stateSets {
        let mut stateSet = stateSet.clone();
        oVars = BackendVariable::addVars(&stateSet.varA, oVars)?;
        lhs = Expression::crefToExp(stateSet.crA.clone())?;
        tp = ComponentReference::crefTypeFull(&stateSet.crA)?;
        tp = DAEUtil::expTypeElementType(&tp);
        if DAEUtil::expTypeComplex(&tp) {
            recordSize = Some(Expression::sizeOf(&tp));
        } else {
            recordSize = None;
        }
        expLst = metamodelica::nil();
        crLst = SymbolicJacobian::getJacobianDependencies(&stateSet.jacobian)?;
        expLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut cr in (crLst).into_iter().cloned() {
                let __x = Expression::crefToExp(cr.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        expLst = metamodelica::cons(
            metamodelica::Ref::new(DAE::Exp::ICONST {
                integer: stateSet.index.clone() - 1,
            }),
            expLst,
        );
        rhs = metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("$stateSelectionSet"),
            }),
            expLst: expLst,
            attr: DAE::callAttrBuiltinOther().clone(),
        });
        eqn = metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION {
            dimSize: list![((stateSet.varA).len() as i32)],
            left: lhs,
            right: rhs,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(),
            recordSize: recordSize,
        });
        (oEqns, _) = ExpandableArray::add(eqn.clone(), oEqns)?;
        if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
            BackendDump::dumpEquationList(
                &(list![eqn]),
                &(literal!("initial state selection equation generated:")),
            )?;
        }
        if metamodelica::arrayLength(stateSetFixCounts.clone()) >= stateSet.index.clone()
            && metamodelica::arrayGet(stateSetFixCounts.clone(), stateSet.index.clone())? > 0
        {
            unfixedStates = metamodelica::nil();
            for mut state in &*stateSet.statescandidates.clone() {
                if !(BackendVariable::varFixed(metamodelica::AsArg::as_arg(&state))) {
                    unfixedStates = metamodelica::cons(state.clone(), unfixedStates);
                }
            }
            toFix = metamodelica::arrayGet(stateSetFixCounts.clone(), stateSet.index.clone())?;
            statesToFix = metamodelica::nil();
            statesToFix = SymbolicJacobian::getFixedStatesForSelfdependentSets(&stateSet, unfixedStates, toFix)?;
            for mut state in &*statesToFix {
                lhs = Expression::crefToExp(state.varName.clone())?;
                rhs = IndexReduction::makeStartExp(state.varName.clone())?;
                initEqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                    exp: lhs,
                    scalar: rhs,
                    source: DAE::emptyElementSource().clone(),
                    attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(),
                });
                (oEqns, _) = ExpandableArray::add(initEqn, oEqns)?;
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("StateSet "));
                    __mm_s.push_str(&*intString(stateSet.index.clone()));
                    __mm_s.push_str(&*literal!(" is underconstraint for the initial system.\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print(literal!("======================================\n"));
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("# States left to fix: "));
                    __mm_s.push_str(&*intString(toFix));
                    __mm_s.push_str(&*literal!(".\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("# Unfixed candidates: "));
                    __mm_s.push_str(&*intString(((stateSet.statescandidates).len() as i32) - toFix));
                    __mm_s.push_str(&*literal!(".\n"));
                    ArcStr::from(__mm_s)
                });
                BackendDump::dumpVarList(&statesToFix, &(literal!("Chosen states to fix:")))?;
            }
        }
    }
    Ok((oVars, oEqns))
}

fn collectInitialVars(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<i32>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<i32>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<i32>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::STATE { .. }, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                            let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut derVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut startVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                            let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut derCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut startCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut isFixed: bool;
                            let mut preUsed: bool;
                            let mut startExp: metamodelica::Ref<DAE::Exp>;
                            let mut crefExp: metamodelica::Ref<DAE::Exp>;
                            let mut stateSetIdxString: ArcStr;
                            let mut stateSetSplit: metamodelica::List<ArcStr>;
                            let mut stateSetIdx: i32;
                            let mut parameters: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut var = (*var).clone();
                            let mut vars = (*vars).clone();
                            let mut eqns = (*eqns).clone();
                            isFixed = BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var));
                            preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                            crefExp = Expression::crefExp(cr.clone())?;
                            startCR = ComponentReference::crefPrefixStart(cr.clone());
                            startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                            startVar = BackendVariable::setBindExp(startVar.clone(), None);
                            startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                            startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                            startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                            startExp = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
                            parameters = Expression::getAllCrefs(startExp.clone())?;
                            if !(({
                let mut __acc: Option<bool> = None;
                for mut p in (parameters.clone()).into_iter().cloned() {
                            let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
                }
                __acc.unwrap_or(true)
            })) {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                                vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                            }
                            if isFixed {
                                if StringUtil::startsWith(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?, literal!("$STATESET")) && Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                                    stateSetSplit = Util::stringSplitAtChar(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?, literal!("."))?;
                                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stateSetSplit.clone()) {
                                                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    stateSetIdxString = metamodelica::Own::own(__pa0);
                                    stateSetSplit = metamodelica::Own::own(__pa1);
                                    stateSetIdxString = substring(stateSetIdxString.clone(), 10, ((stateSetIdxString).len() as i32))?;
                                    stateSetIdx = stringInt(stateSetIdxString.clone())?;
                                    metamodelica::arrayUpdate(stateSetFixCounts.clone(), stateSetIdx, metamodelica::arrayGet(stateSetFixCounts.clone(), stateSetIdx)? + 1)?;
                                } else {
                                    if Expression::isConstValue(&startExp)? {
                                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                                    } else {
                                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                                    }
                                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                                }
                            }
                            var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                            derCR = ComponentReference::crefPrefixDer(cr.clone());
                            derVar = BackendVariable::copyVarNewName(derCR.clone(), var.clone());
                            derVar = BackendVariable::setVarDirection(derVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            derVar = BackendVariable::setBindExp(derVar.clone(), None);
                            preCR = ComponentReference::crefPrefixPre(cr.clone());
                            preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                            preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            preVar = BackendVariable::setBindExp(preVar.clone(), None);
                            preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                            preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                            eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            vars = BackendVariable::addVar(derVar.clone(), vars.clone())?;
                            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                            vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                            eqns = if (preUsed) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                            Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                    let mut startValue_: metamodelica::Ref<DAE::Exp>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BaseHashSet::has(cr.clone(), &(hs.clone()))?) else { return Err("pattern mismatch") };
                    let true = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    startValue_ = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue_.clone()))?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue_.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                    let mut preUsed: bool;
                    let mut startValue: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                    startValue = BackendVariable::varStartValueOption(metamodelica::AsArg::as_arg(&var));
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), startValue.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: None, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: metamodelica::Ref<DAE::Exp>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let true = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    startExp = BackendVariable::varStartValueType(metamodelica::AsArg::as_arg(&var))?;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    r#str = ExpressionBasics::printExpStr(startExp.clone())?;
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), Some(startExp.clone()));
                    var = BackendVariable::setVarFixed(var.clone(), true)?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(metamodelica::AsArg::as_arg(&var)));
                    Error::addSourceMessage(&(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone()), list![s.clone(), r#str.clone()], &info)?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (intGt(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    r#str = ExpressionBasics::printExpStr(bindExp.clone())?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(metamodelica::AsArg::as_arg(&var)));
                    Error::addSourceMessage(&(Error::UNFIXED_PARAMETER_WITH_BINDING.clone()), list![s.clone(), s.clone(), r#str.clone()], &info)?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: bindExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(metamodelica::AsArg::as_arg(&var))) {
                        None => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    var = BackendVariable::setVarStartValue(var.clone(), bindExp.clone())?;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    r#str = ExpressionBasics::printExpStr(bindExp.clone())?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(metamodelica::AsArg::as_arg(&var)));
                    Error::addSourceMessage(&(Error::UNFIXED_PARAMETER_WITH_BINDING_31.clone()), list![s.clone(), s.clone(), r#str.clone()], &info)?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: metamodelica::Ref<DAE::Exp>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sv: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(metamodelica::AsArg::as_arg(&var))) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    startExp = metamodelica::Own::own(__pa0);
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    r#str = ExpressionBasics::printExpStr(bindExp.clone())?;
                    sv = ExpressionBasics::printExpStr(startExp.clone())?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(metamodelica::AsArg::as_arg(&var)));
                    Error::addSourceMessage(&(Error::UNFIXED_PARAMETER_WITH_BINDING_AND_START_VALUE_31.clone()), list![s.clone(), sv.clone(), s.clone(), r#str.clone()], &info)?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut vars = (*vars).clone();
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST { .. }, .. }, _) => {
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (var @ Deref @ BackendDAE::Var { varName: cr, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                            let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut startVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                            let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut startCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut isInput: bool;
                            let mut preUsed: bool;
                            let mut startExp: metamodelica::Ref<DAE::Exp>;
                            let mut parameters: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut var = (*var).clone();
                            let mut vars = (*vars).clone();
                            let mut fixvars = (*fixvars).clone();
                            let mut eqns = (*eqns).clone();
                            let true = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                            if datarecon.clone() {
                                isInput = checkComponentNames(var.varDirection.clone(), metamodelica::AsArg::as_arg(&cr));
                            } else {
                                isInput = BackendVariable::isVarOnTopLevelAndInput(metamodelica::AsArg::as_arg(&var));
                            }
                            preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                            startCR = ComponentReference::crefPrefixStart(cr.clone());
                            startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                            startVar = BackendVariable::setBindExp(startVar.clone(), None);
                            startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                            startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                            startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                            startExp = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
                            parameters = Expression::getAllCrefs(startExp.clone())?;
                            if !(({
                let mut __acc: Option<bool> = None;
                for mut p in (parameters.clone()).into_iter().cloned() {
                            let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
                }
                __acc.unwrap_or(true)
            })) {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                                vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                            }
                            var = BackendVariable::setVarFixed(var.clone(), false)?;
                            preCR = ComponentReference::crefPrefixPre(cr.clone());
                            preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                            preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            preVar = BackendVariable::setBindExp(preVar.clone(), None);
                            preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                            preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                            if Expression::isConstValue(&startExp)? {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            } else {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            }
                            vars = if (!(isInput)) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                            fixvars = if (isInput) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                            vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                            eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                            Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (var @ Deref @ BackendDAE::Var { varName: cr, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                            let mut preVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut startVar: metamodelica::Ref<BackendDAE::Var>;
                            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                            let mut preCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut startCR: metamodelica::Ref<DAE::ComponentRef>;
                            let mut isInput: bool;
                            let mut preUsed: bool;
                            let mut startExp: metamodelica::Ref<DAE::Exp>;
                            let mut parameters: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut vars = (*vars).clone();
                            let mut fixvars = (*fixvars).clone();
                            let mut eqns = (*eqns).clone();
                            let false = (BackendVariable::varFixed(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                            if datarecon.clone() {
                                isInput = checkComponentNames(var.varDirection.clone(), metamodelica::AsArg::as_arg(&cr));
                            } else {
                                isInput = BackendVariable::isVarOnTopLevelAndInput(metamodelica::AsArg::as_arg(&var));
                            }
                            preUsed = BaseHashSet::has(cr.clone(), &(hs.clone()))?;
                            startCR = ComponentReference::crefPrefixStart(cr.clone());
                            startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                            startVar = BackendVariable::setBindExp(startVar.clone(), None);
                            startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                            startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                            startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                            startExp = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
                            parameters = Expression::getAllCrefs(startExp.clone())?;
                            if !(({
                let mut __acc: Option<bool> = None;
                for mut p in (parameters.clone()).into_iter().cloned() {
                            let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
                }
                __acc.unwrap_or(true)
            })) {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                                vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                            }
                            preCR = ComponentReference::crefPrefixPre(cr.clone());
                            preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                            preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                            preVar = BackendVariable::setBindExp(preVar.clone(), None);
                            preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                            preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                            if Expression::isConstValue(&startExp)? {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            } else {
                                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            }
                            vars = if (!(isInput)) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                            fixvars = if (isInput) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                            vars = if (preUsed) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                            eqns = if (preUsed) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                            Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialVars failed for: ")); __mm_s.push_str(&*BackendDump::varString(&inVar)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, outTpl))
}

fn checkComponentNames(
    mut inVarDirection: DAE::VarDirection,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut isTopLevel: bool;
    isTopLevel = (::match_deref::match_deref! { match &((inVarDirection, &**inComponentRef)) {
        (DAE::VarDirection::INPUT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        (DAE::VarDirection::INPUT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => true,
        (_, _) => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTopLevel
}

fn collectInitialClockedVarsEqns(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    (vars, eqns) = inTpl.clone();
    (outVar, outTpl) = (::match_deref::match_deref! { match &(inVar) {
        var @ Deref @ BackendDAE::Var { varName: cr, varType: ty, varKind: kind, .. } => {
            let mut crExp: metamodelica::Ref<DAE::Exp>;
            let mut startExp: metamodelica::Ref<DAE::Exp>;
            crExp = Expression::crefExp(cr.clone())?;
            (vars, eqns) = (match kind.clone() {
        BackendDAE::VarKind::CLOCKED_STATE { previousName: ref previousCR, .. } => {
            let mut previousVar: metamodelica::Ref<BackendDAE::Var>;
            let mut previousExp: metamodelica::Ref<DAE::Exp>;
            previousVar = BackendVariable::copyVarNewName(previousCR.clone(), var.clone());
            previousVar = BackendVariable::setVarKind(previousVar, openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
            previousVar = BackendVariable::setVarDirection(previousVar, openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            previousVar = BackendVariable::setBindExp(previousVar, None);
            previousVar = BackendVariable::setVarFixed(previousVar, true)?;
            previousVar = BackendVariable::setVarStartValueOption(previousVar, Some(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
            previousExp = Expression::crefExp(previousCR.clone())?;
            vars = BackendVariable::addVar(previousVar, vars)?;
            eqns = BackendEquation::add(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: previousExp, scalar: crExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns)?;
            startExp = BackendVariable::varStartValue(metamodelica::AsArg::as_arg(&var))?;
            vars = BackendVariable::addVar(var.clone(), vars)?;
            eqns = BackendEquation::add(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: crExp, scalar: startExp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns)?;
            (vars, eqns)
        },
        _ => {
            (vars, eqns)
        },
    });
            (var.clone(), (vars, eqns))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outTpl))
}

fn collectInitialEqns(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    let mut eqn1: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut reeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut size: i32;
    let mut b: bool;
    (eqns, reeqns) = inTpl;
    (eqn1, _) = BackendEquation::traverseExpsOfEquation(
        inEq,
        (std::sync::Arc::new(Expression::traverseSubexpressionsDummyHelper)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>>
                                + 'static,
                        >,
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>>
                                + 'static,
                        >,
                    )> + 'static,
            >),
        (std::sync::Arc::new(fnptr!(replaceDerPreCref, metamodelica::Ref<DAE::Exp>))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    size = BackendEquation::equationSize(&eqn1)?;
    b = intGt(size, 0);
    eqns = if (b) {
        BackendEquation::add(eqn1.clone(), eqns)?
    } else {
        eqns
    };
    reeqns = if (!(b)) {
        BackendEquation::add(eqn1, reeqns)?
    } else {
        reeqns
    };
    outTpl = (eqns, reeqns);
    Ok((outEq, outTpl))
}

fn replaceDerPreCref(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: metamodelica::Ref<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixDer(cr.clone());
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: metamodelica::Ref<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixPre(cr.clone());
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: metamodelica::Ref<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixPrevious(cr.clone());
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// =============================================================================
// section for bindings
//
// =============================================================================
fn collectInitialBindings(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    (outVar, outTpl) = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (var @ Deref @ BackendDAE::Var { bindExp: None, .. }, _) => {
            (var.clone(), inTpl)
        },
        (var @ Deref @ BackendDAE::Var { varName: cr, bindExp: Some(bindExp), varKind: BackendDAE::VarKind::EXTOBJ { .. }, source, .. }, (eqns, reeqns)) => {
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut eqns = (*eqns).clone();
            eqn = metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            eqns = BackendEquation::add(eqn, eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        (var @ Deref @ BackendDAE::Var { varName: cr, bindExp: Some(bindExp), varType: ty, source, .. }, (eqns, reeqns)) => {
            let mut basic_ty: metamodelica::Ref<DAE::Type>;
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
            let mut record_size: Option<i32>;
            let mut eqns = (*eqns).clone();
            crefExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() });
            if Types::isArray(metamodelica::AsArg::as_arg(&ty)) {
                basic_ty = Types::getBasicType(metamodelica::AsArg::as_arg(&ty));
                record_size = if (Types::isRecord(&basic_ty)) {Some(Types::getDimensionProduct(&basic_ty)?)} else {None};
                eqn = metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: Types::getDimensionSizes(metamodelica::AsArg::as_arg(&ty))?, left: crefExp, right: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), recordSize: record_size });
            } else {
                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: crefExp, scalar: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            }
            eqns = BackendEquation::add(eqn, eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialBindings failed for: ")); __mm_s.push_str(&*BackendDump::varString(&inVar)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outTpl))
}

// =============================================================================
// section for post-optimization module "removeInitializationStuff"
//
// =============================================================================
pub(crate) fn removeInitializationStuff(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut removedEqsList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = inDAE.shared.clone();
    for mut eqs in &*outDAE.eqs.clone() {
        BackendDAEUtil::traverseBackendDAEExpsEqns(
            eqs.orderedEqs.clone(),
            (std::sync::Arc::new(removeInitializationStuff1)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                        + 'static,
                >),
            false,
        )?;
        BackendDAEUtil::traverseBackendDAEExpsEqns(
            eqs.removedEqs.clone(),
            (std::sync::Arc::new(removeInitializationStuff1)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                        + 'static,
                >),
            false,
        )?;
    }
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        shared.removedEqs.clone(),
        (std::sync::Arc::new(removeInitializationStuff1)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone())? {
        removedEqsList = (match BackendEquation::equationKind(metamodelica::AsArg::as_arg(&eq))? {
            BackendDAE::EquationKind::INITIAL_EQUATION { .. } => removedEqsList,
            _ => filterWhenEquation(eq.clone(), removedEqsList)?,
        });
    }
    assign_field!(
        shared.removedEqs = BackendEquation::listEquation(&(removedEqsList.reverse()))?,
        shared.initialEqs = BackendEquation::emptyEqns()
    );
    assign_field!(outDAE.shared = shared);
    Ok(outDAE)
}

fn filterWhenEquation(
    mut inEqn: metamodelica::Ref<BackendDAE::Equation>,
    mut inEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    outEqnLst = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition, elsewhenPart: None, .. }, .. } if ((((BackendDAEUtil::getConditionList(condition.clone())?).0)).is_empty()) => inEqnLst,
        _ => metamodelica::cons(inEqn, inEqnLst),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnLst)
}

fn removeInitializationStuff1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp, &removeInitializationStuff2, inUseHomotopy)?;
    Ok((outExp, outUseHomotopy))
}

fn removeInitializationStuff2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: actual, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. } => {
            (if (Flags::getConfigBool(Flags::DAE_MODE.clone())?) {inExp} else {actual.clone()}, true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outUseHomotopy))
}

// =============================================================================
// section for post-optimization module "replaceHomotopyWithSimplified"
//
// =============================================================================
pub(crate) fn replaceHomotopyWithSimplified(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE;
    assign_field!(
        outDAE.eqs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
            for mut eqs in (outDAE.eqs.clone()).into_iter().cloned() {
                let __x = replaceHomotopyWithSimplifiedEqs(eqs.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(outDAE)
}

pub(crate) fn replaceHomotopyWithSimplifiedEqs(
    mut eqs: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut eqs: metamodelica::Ref<BackendDAE::EqSystem> = eqs;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        eqs.orderedEqs.clone(),
        (std::sync::Arc::new(replaceHomotopyWithSimplified1)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        eqs.removedEqs.clone(),
        (std::sync::Arc::new(replaceHomotopyWithSimplified1)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    eqs = BackendDAEUtil::clearEqSyst(&eqs);
    Ok(eqs)
}

fn replaceHomotopyWithSimplified1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(replaceHomotopyWithSimplified2, metamodelica::Ref<DAE::Exp>, bool),
        inUseHomotopy,
    )?;
    Ok((outExp, outUseHomotopy))
}

fn replaceHomotopyWithSimplified2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUseHomotopy: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: simplified, tail: _ } }, .. } => {
            (simplified.clone(), true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}
