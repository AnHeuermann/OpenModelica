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
use crate::Matching;
use crate::Sorting;
use crate::SymbolTable;
use crate::SymbolicJacobian;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type ExtAdjacencyMatrixRow = (i32, metamodelica::List<i32>);

pub type ExtAdjacencyMatrix = metamodelica::List<(i32, metamodelica::List<i32>)>;

pub(crate) const UNDERLINE: &'static str = "==========================================================================";

pub(crate) fn newExtractionAlgorithm(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outResidualEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut setC_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut setS_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut residualEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut complexEquationList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut swappedEquationList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut varCount: i32;
    let mut eqCount: i32;
    let mut ebltEqsLst: metamodelica::List<i32> = metamodelica::nil();
    let mut matchedEqsLst: metamodelica::List<i32>;
    let mut approximatedEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut setC: metamodelica::List<i32>;
    let mut tempSetS: metamodelica::List<i32> = metamodelica::nil();
    let mut setS: metamodelica::List<i32>;
    let mut boundaryConditionEquations: metamodelica::List<i32>;
    let mut bindingEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut residualVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut unMeasuredVariables: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut simCodeJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut r#str: ArcStr;
    let mut modelicaOutput: ArcStr;
    let mut modelicaFileName: ArcStr;
    let mut modelName: ArcStr;
    let mut auxillaryConditionsFilename: ArcStr;
    let mut auxillaryEquations: ArcStr;
    let mut intermediateEquationsFilename: ArcStr;
    let mut intermediateEquations: ArcStr;
    let mut csvfileName: ArcStr;
    let mut mappedEbltSetS: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>;
    let mut allVarsList: metamodelica::List<i32>;
    let mut knowns: metamodelica::List<i32> = metamodelica::nil();
    let mut boundaryConditionVars: metamodelica::List<i32> = metamodelica::nil();
    let mut exactEquationVars: metamodelica::List<i32> = metamodelica::nil();
    let mut extractedVarsfromSetS: metamodelica::List<i32>;
    let mut boundaryConditionTaggedEquationSolvedVars: metamodelica::List<i32>;
    let mut unMeasuredVariablesOfInterest: metamodelica::List<i32> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables;
    let mut outDiffVars: BackendDAE::Variables;
    let mut outOtherVars: BackendDAE::Variables;
    let mut outResidualVars: BackendDAE::Variables;
    let mut procedureCount: i32;
    let mut measurementcsvData: metamodelica::List<(ArcStr, ArcStr)>;
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    currentSystem = metamodelica::Own::own(__pa0);
    shared = inDAE.shared.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nModelInfo: "));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem, inDAE.shared.clone(), debug)?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status) {
        BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("OrderedVariables")))?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), &(literal!("OrderedEquation")))?;
        allVarsList = List::intRange(BackendVariable::varsSize(&currentSystem.orderedVars));
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
            &currentSystem,
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount, eqCount)?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(&currentSystem, mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(
            bindingEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(
            &(BackendEquation::equationList(currentSystem.orderedEqs.clone())?),
            1,
        );
        if debug {
            BackendDump::dumpEquationList(
                &(List::map1r(
                    approximatedEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("ApproximatedEquations")),
            )?;
            BackendDump::dumpEquationList(
                &(List::map1r(
                    boundaryConditionEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("boundaryConditionEquations")),
            )?;
        }
        approximatedEquations = List::flatten(List::map1r(
            approximatedEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionEquations = List::flatten(List::map1r(
            boundaryConditionEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionTaggedEquationSolvedVars =
            getBoundaryConditionVariables(&boundaryConditionEquations, &solvedEqsAndVarsInfo)?;
        if debug {
            metamodelica::print(literal!(
                "\nApproximated and BoundaryCondition Equation Indexes :\n==========================================="
            ));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :"));
                __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\n"));
        }
        (
            knowns,
            boundaryConditionVars,
            exactEquationVars,
            unMeasuredVariablesOfInterest,
        ) = getVariablesBlockCategories(&currentSystem.orderedVars, &allVarsList)?;
        boundaryConditionVars = listAppend(boundaryConditionVars, boundaryConditionTaggedEquationSolvedVars);
        if debug {
            metamodelica::print(literal!("\nVariablesCategories\n============================="));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nknownVars                    :"));
                __mm_s.push_str(&*dumplistInteger(knowns.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nboundaryConditionVars        :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nexactEquationVars            :"));
                __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nadjacencyMatrix              :"));
                __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        dumpSetSVarsSolvedInfo(
            &matchedEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Standard BLT of the original model")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                knowns.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Variables of interest")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                boundaryConditionVars.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Boundary conditions")),
        )?;
        dumpSetSVarsSolvedInfo(
            &bindingEquations,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Binding equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                approximatedEquations.clone(),
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("Approximated equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                boundaryConditionEquations,
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("boundary condition equations")),
        )?;
        ebltEqsLst = getEBLTEquations(
            knowns.clone(),
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            &currentSystem,
        );
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst, &bindingEquations, &fnptr!(intEq, i32, i32))?;
        dumpSetSVarsSolvedInfo(
            &ebltEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("E-BLT: equations that compute the variables of interest")),
        )?;
        (
            currentSystem,
            tempSetS,
            mappedEbltSetS,
            status,
            setBFailedBoundaryConditionEquations,
        ) = traverseEBLTAndExtractSetCAndSetS(
            currentSystem.clone(),
            &ebltEqsLst,
            &sBltAdjacencyMatrix,
            &knowns,
            boundaryConditionVars.clone(),
            &(currentSystem.orderedVars.clone()),
            currentSystem.orderedEqs.clone(),
            mapIncRowEqn.clone(),
            &solvedEqsAndVarsInfo,
            debug,
            setBFailedBoundaryConditionEquations,
            bindingEquations,
        )?;
        if !(status) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: "));
                __mm_s.push_str(&*intString(procedureCount));
                __mm_s.push_str(&*literal!(", re-running with modified model\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        procedureCount = procedureCount + 1;
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nExtraction procedure is successfully completed in iteration count: "
        ));
        __mm_s.push_str(&*intString(procedureCount - 1));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    tempSetS = List::setDifferenceOnTrue(tempSetS, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    (ebltEqsLst, tempSetS, complexEquationList, swappedEquationList) = swapComplexEquationsInSetC(
        ebltEqsLst,
        tempSetS,
        &mappedEbltSetS,
        &currentSystem,
        mapIncRowEqn.clone(),
    )?;
    if debug {
        dumpSetSVarsSolvedInfo(
            &tempSetS,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Set-S Solved-Variables Information")),
        )?;
    }
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), &sBltAdjacencyMatrix);
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS, &knowns, &fnptr!(intEq, i32, i32))?;
    setC = List::unique(&(getAbsoluteIndexHelper(&ebltEqsLst, mapIncRowEqn.clone())?));
    setS = List::unique(&(getAbsoluteIndexHelper(&tempSetS, mapIncRowEqn.clone())?));
    setC_Eq = getEquationsFromSBLTAndEBLT(&setC, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    setS_Eq = getEquationsFromSBLTAndEBLT(&setS, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_C: "));
        __mm_s.push_str(&*dumplistInteger(setC.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_S: "));
        __mm_s.push_str(&*dumplistInteger(setS.clone())?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setC_Eq)?, &(literal!("SET_C")))?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setS_Eq)?, &(literal!("SET_S")))?;
    unMeasuredVariables = List::map1r(
        unMeasuredVariablesOfInterest.clone().reverse(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?;
    outDiffVars = BackendVariable::listVar(List::map1r(
        knowns.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    outDiffVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(&shared)?;
    outDiffVars = setStartValuesToMeasurements(&outDiffVars, &measurementcsvData, &csvfileName)?;
    (_, residualEquations) = BackendEquation::traverseEquationArray(
        BackendEquation::listEquation(&setC_Eq)?,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::Ref<AvlTreePathFunction::Tree>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1),
        (shared.functionTree.clone(), metamodelica::nil()),
    )?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(
        &(residualEquations.reverse()),
        &(literal!("$res_F_")),
        1,
        false,
    )?;
    outResidualVars = BackendVariable::listVar(residualVars.reverse())?;
    outResidualEqns = BackendEquation::listEquation(&residualEquations)?;
    outOtherEqns = BackendEquation::listEquation(&setS_Eq)?;
    paramVars = BackendEquation::equationsVars(
        BackendEquation::merge(outOtherEqns.clone(), outResidualEqns.clone())?,
        shared.globalKnownVars.clone(),
    )?;
    outOtherVars = BackendVariable::listVar(List::map1r(
        extractedVarsfromSetS,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    dumpSetSVars(&outOtherVars, &(literal!("Unknown variables in SET_S")))?;
    BackendDump::dumpVariables(
        &(BackendVariable::listVar(paramVars.clone())?),
        &(literal!("Parameters in SET_S")),
    )?;
    auxillaryConditionsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_AuxiliaryConditions.html"));
        ArcStr::from(__mm_s)
    };
    auxillaryEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setC_Eq)?,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Auxiliary conditions"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                BackendEquation::listEquation(&setC_Eq)?,
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                BackendEquation::listEquation(&setC_Eq)?,
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(auxillaryConditionsFilename, auxillaryEquations)?;
    intermediateEquationsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_IntermediateEquations.html"));
        ArcStr::from(__mm_s)
    };
    intermediateEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setS_Eq)?,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Intermediate equations for measured variables"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                BackendEquation::listEquation(&setS_Eq)?,
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                BackendEquation::listEquation(&setS_Eq)?,
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(intermediateEquationsFilename, intermediateEquations)?;
    dumpRelatedBoundaryConditionsEquations(&setBFailedBoundaryConditionEquations, &shared.info.fileNamePrefix)?;
    VerifyDataReconciliation(
        &ebltEqsLst,
        &tempSetS,
        knowns,
        &boundaryConditionVars,
        &sBltAdjacencyMatrix,
        &solvedEqsAndVarsInfo,
        &exactEquationVars,
        &approximatedEquations,
        currentSystem.orderedVars.clone(),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        &outOtherVars,
        &setS_Eq,
        &shared,
        setC,
        setS,
        ((unMeasuredVariablesOfInterest).len() as i32),
    )?;
    if debug {
        BackendDump::dumpVariables(&outDiffVars, &(literal!("Jacobian_knownVariables")))?;
        BackendDump::dumpVariables(&outResidualVars, &(literal!("Jacobian_outResidualVars")))?;
        BackendDump::dumpVariables(&outOtherVars, &(literal!("Jacobian_outOtherVars")))?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), &(literal!("Jacobian_ResidualEquation")))?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), &(literal!("Jacobian_other_Equation")))?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(
        &outDiffVars,
        outResidualEqns.clone(),
        outResidualVars.clone(),
        outOtherEqns.clone(),
        outOtherVars.clone(),
        shared,
        &(outOtherVars.clone()),
        literal!("F"),
        false,
    )?;
    assign_field!(
        shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData {
            symbolicJacobian: simCodeJacobian,
            setcVars: outResidualVars.clone(),
            datareconinputs: outDiffVars.clone(),
            setBVars: Some(BackendVariable::listVar(unMeasuredVariables)?),
            symbolicJacobianH: None,
            relatedBoundaryConditions: ((setBFailedBoundaryConditionEquations).len() as i32)
        })
    );
    currentSystem = BackendDAEUtil::setEqSystVars(
        currentSystem,
        BackendVariable::mergeVariables(outResidualVars.clone(), outOtherVars.clone(), true)?,
    );
    currentSystem = BackendDAEUtil::setEqSystEqs(
        currentSystem,
        BackendEquation::merge(outResidualEqns.clone(), outOtherEqns.clone())?,
    );
    assign_field!(currentSystem.removedEqs = BackendEquation::emptyEqns());
    inputVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarDirection,
            metamodelica::Ref<BackendDAE::Var>,
            DAE::VarDirection
        ),
        openmodelica_frontend_types::DAE::VarDirection::INPUT,
    )?)?;
    shared = BackendDAEUtil::setSharedGlobalKnownVars(
        shared,
        BackendVariable::mergeVariables(BackendVariable::listVar(paramVars.clone())?, inputVars, true)?,
    );
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Inputs.csv"));
        ArcStr::from(__mm_s)
    })) {
        r#str = literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n");
        r#str = dumpToCsv(&r#str, &(BackendVariable::varList(&outDiffVars)?))?;
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_Inputs.csv"));
                ArcStr::from(__mm_s)
            },
            r#str,
        )?;
    }
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Correlation_Inputs.csv"));
        ArcStr::from(__mm_s)
    })) {
        r#str = dumpCorrelationVarsToCsv(&(BackendVariable::varList(&outDiffVars)?))?;
        r#str = dumpToCsv(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }),
            &(BackendVariable::varList(&outDiffVars)?),
        )?;
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_Correlation_Inputs.csv"));
                ArcStr::from(__mm_s)
            },
            r#str,
        )?;
    }
    modelicaFileName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Reconciled_tmp"));
        ArcStr::from(__mm_s)
    };
    modelName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Reconciled_"));
        __mm_s.push_str(&*System::stringReplace(
            shared.info.fileNamePrefix.clone(),
            literal!("."),
            literal!("_"),
        )?);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = literal!(
        "/* This is a Reconciled Model which is generated by the Data Reconciliation extraction algorithm */\n"
    );
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("model "));
        __mm_s.push_str(&*modelName);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outDiffVars)?),
        &(literal!("Variables of Interest")),
    )?;
    modelicaOutput = dumpExtractedVars(&modelicaOutput, &paramVars, &(literal!("parameters in SET-S")))?;
    modelicaOutput = dumpResidualVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outResidualVars)?),
        &(literal!("residualVars")),
    )?;
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outOtherVars)?),
        &(literal!("remaining variables in setS")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nequation"));
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedEquations(&modelicaOutput, outResidualEqns, &(literal!("set-C Canonical form")))?;
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        outOtherEqns,
        &(literal!("remaining equations in Set-S")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nend "));
        __mm_s.push_str(&*modelName);
        __mm_s.push_str(&*literal!(";"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*modelicaFileName);
            __mm_s.push_str(&*literal!(".mo"));
            ArcStr::from(__mm_s)
        },
        modelicaOutput,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![currentSystem],
        shared: shared,
    });
    Ok(outDAE)
}

// extract the "-sx =.csv" file path from simflags
pub(crate) fn extractSxPath(mut simflags: ArcStr) -> Result<ArcStr> {
    let mut csvFilePath: ArcStr = arcstr::literal!("");
    let mut nummatches: i32;
    let mut filePath: ArcStr = literal!("");
    if System::stringFind(simflags.clone(), literal!("-sx"))? < 0 {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                ": No -sx flag found in simflags, hence no csv file will be read for setting start values of the variables of interest for data reconciliation initialization."
            )],
        )?;
        return Err("fail");
    }
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-sx[ \t]*=[ \t]*(\"[^\"]*\"|[^, \t]+)"), 2, true, false)) {
            (__pa1, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nummatches = metamodelica::Own::own(__pa1);
        filePath = metamodelica::Own::own(__pa2);
        if nummatches == 2 {
            csvFilePath =
                unwrap_break_err!(System::stringReplace(filePath.clone(), literal!(" "), literal!("")), '__try0);
            csvFilePath =
                unwrap_break_err!(System::stringReplace(csvFilePath.clone(), literal!("\""), literal!("")), '__try0);
            return Ok(csvFilePath);
        }
        Ok::<_, &'static str>((filePath.clone(), nummatches.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            filePath = __try0_o0;
            nummatches = __try0_o1;
        }
        Err(_) => {
            return Ok(csvFilePath);
        }
    }
    Ok(csvFilePath)
}

// read the csv file and extract the measurement data for setting start values for data reconciliation initialization.
fn readMeasurementsFromCSV(
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(ArcStr, metamodelica::List<(ArcStr, ArcStr)>)> {
    let mut csvFileName: ArcStr;
    let mut measurementData: metamodelica::List<(ArcStr, ArcStr)> = metamodelica::nil();
    let mut content: ArcStr;
    let mut tokens: metamodelica::List<ArcStr>;
    let mut lines: metamodelica::List<ArcStr>;
    let mut p: Absyn::Program;
    if (shared.info.simflags).is_none() {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                ": simflags is NONE, expected the simulation flags to be present in shared.info.simflags for reading measurements from csv file for data reconciliation initialization."
            )],
        )?;
        return Err("fail");
    }
    csvFileName = extractSxPath(shared.info.simflags.clone().ok_or("pattern mismatch")?)?;
    if stringEmpty(&csvFileName) {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                ": No csv file provided or failed to read file with -sx flag in simflags."
            )],
        )?;
        return Err("fail");
    }
    if StringUtil::startsWith(csvFileName.clone(), literal!("modelica://"))
        || StringUtil::startsWith(csvFileName.clone(), literal!("file://"))
    {
        p = SymbolTable::getAbsyn();
        csvFileName = ProgramUtil::getFullPathFromUri(&p, csvFileName, true)?;
    }
    content = System::readFile(csvFileName.clone())?;
    if stringEmpty(&content) {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(": Failed to read csv file content from "));
                __mm_s.push_str(&*csvFileName);
                __mm_s.push_str(&*literal!(" and hence start values can not be set."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    lines = System::strtok(content, literal!("\n"));
    for mut line in &*lines {
        let mut line = line.clone();
        line = System::stringReplace(line, literal!(";"), literal!(","))?;
        line = System::trim(line, literal!(" \u{c}\n\r\t\u{b}"));
        tokens = Util::stringSplitAtChar(line, literal!(","))?;
        if !((tokens).is_empty()) && ((tokens).len() as i32) >= 2 {
            measurementData = metamodelica::cons(((tokens).get(1)?, (tokens).get(2)?), measurementData);
        }
    }
    Ok((csvFileName, measurementData))
}

fn setStartValuesToMeasurements(
    mut inVariables: &BackendDAE::Variables,
    mut measurementData: &metamodelica::List<(ArcStr, ArcStr)>,
    mut csvFileName: &ArcStr,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varName: ArcStr;
    let mut valueStr: ArcStr;
    let mut value: metamodelica::Real;
    let mut foundMeasurement: bool;
    varList = metamodelica::nil();
    for mut var in &*BackendVariable::varList(inVariables)? {
        let mut var = var.clone();
        (valueStr, foundMeasurement) = checkVarExistenceInMeasurementData(&var, measurementData)?;
        if !(foundMeasurement) {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(": Entry for variable of interest "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
                    __mm_s.push_str(&*literal!(" not found in the measurement csv file "));
                    __mm_s.push_str(&*csvFileName);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        if let Ok(__iflet0) = stringReal(valueStr.clone()) {
            value = __iflet0;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(": Failed to convert the measurement value \""));
                    __mm_s.push_str(&*valueStr);
                    __mm_s.push_str(&*literal!("\" for variable of interest "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
                    __mm_s.push_str(&*literal!(" from csv file "));
                    __mm_s.push_str(&*csvFileName);
                    __mm_s.push_str(&*literal!(
                        " to a valid Real number for setting start value for data reconciliation initialization."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        var = BackendVariable::setVarStartValue(var, metamodelica::Ref::new(DAE::Exp::RCONST { real: value }))?;
        varList = metamodelica::cons(var, varList);
    }
    outVariables = BackendVariable::listVar(varList.reverse())?;
    Ok(outVariables)
}

fn checkVarExistenceInMeasurementData(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut measurementData: &metamodelica::List<(ArcStr, ArcStr)>,
) -> Result<(ArcStr, bool)> {
    let mut valueStr: ArcStr = literal!("");
    let mut exists: bool = false;
    let mut varName: ArcStr;
    for mut measurement in &**measurementData {
        (varName, valueStr) = measurement.clone();
        if metamodelica::stringEq(&varName, &(ComponentReference::crefStr(&var.varName)?)) {
            exists = true;
            break;
        }
    }
    Ok((valueStr, exists))
}

fn dumpRelatedBoundaryConditionsEquations(
    mut setBFailedBoundaryConditionEquations: &metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut fileNamePrefix: &ArcStr,
) -> Result<()> {
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut r#str: ArcStr;
    let mut count: i32;
    count = 1;
    r#str = literal!("");
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("<html>\n<body>\n<h2> Related boundary conditions"));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((setBFailedBoundaryConditionEquations).len() as i32)));
        __mm_s.push_str(&*literal!(") "));
        __mm_s.push_str(&*literal!("</h2>\n<ol>"));
        ArcStr::from(__mm_s)
    };
    if (setBFailedBoundaryConditionEquations).is_empty() {
        r#str = literal!("The set of Related boundary conditions are empty.");
    } else {
        for mut i in &**setBFailedBoundaryConditionEquations {
            (_, eq, _) = i.clone();
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("  <li>"));
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(BackendEquation::equationSize(&eq)?));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*BackendDump::equationString(&eq)?);
                __mm_s.push_str(&*literal!(" </li>"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n</ol>\n</body>\n</html>"));
            ArcStr::from(__mm_s)
        };
    }
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fileNamePrefix);
            __mm_s.push_str(&*literal!("_relatedBoundaryConditionsEquations.html"));
            ArcStr::from(__mm_s)
        },
        r#str,
    )?;
    Ok(())
}

pub(crate) fn extractBoundaryCondition(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outBoundaryConditionEquations: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut setS_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut failedboundaryConditionEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut varCount: i32;
    let mut eqCount: i32;
    let mut ebltEqsLst: metamodelica::List<i32> = metamodelica::nil();
    let mut matchedEqsLst: metamodelica::List<i32>;
    let mut approximatedEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut tempSetS: metamodelica::List<i32>;
    let mut setS: metamodelica::List<i32>;
    let mut boundaryConditionEquations: metamodelica::List<i32>;
    let mut bindingEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut setSPrime: metamodelica::List<i32>;
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut setSVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut failedboundaryConditionVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut extraVarsinSetSPrime: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut unMeasuredVariables: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut simCodeJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut r#str: ArcStr;
    let mut modelicaOutput: ArcStr;
    let mut modelicaFileName: ArcStr;
    let mut modelName: ArcStr;
    let mut auxillaryConditionsFilename: ArcStr;
    let mut auxillaryEquations: ArcStr;
    let mut intermediateEquationsFilename: ArcStr;
    let mut intermediateEquations: ArcStr;
    let mut csvfileName: ArcStr;
    let mut mappedEbltSetS: metamodelica::List<(i32, metamodelica::List<i32>)>;
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>;
    let mut allVarsList: metamodelica::List<i32>;
    let mut knowns: metamodelica::List<i32> = metamodelica::nil();
    let mut boundaryConditionVars: metamodelica::List<i32> = metamodelica::nil();
    let mut exactEquationVars: metamodelica::List<i32>;
    let mut boundaryConditionTaggedEquationSolvedVars: metamodelica::List<i32>;
    let mut unMeasuredVariablesOfInterest: metamodelica::List<i32> = metamodelica::nil();
    let mut inputVars: BackendDAE::Variables;
    let mut outDiffVars: BackendDAE::Variables;
    let mut outOtherVars: BackendDAE::Variables;
    let mut outBoundaryConditionVars: BackendDAE::Variables;
    let mut procedureCount: i32;
    let mut measurementcsvData: metamodelica::List<(ArcStr, ArcStr)>;
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    currentSystem = metamodelica::Own::own(__pa0);
    shared = inDAE.shared.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nModelInfo: "));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem, inDAE.shared.clone(), debug)?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status) {
        BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("OrderedVariables")))?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), &(literal!("OrderedEquation")))?;
        allVarsList = List::intRange(BackendVariable::varsSize(&currentSystem.orderedVars));
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
            &currentSystem,
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount, eqCount)?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(&currentSystem, mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(
            bindingEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(
            &(BackendEquation::equationList(currentSystem.orderedEqs.clone())?),
            1,
        );
        if debug {
            BackendDump::dumpEquationList(
                &(List::map1r(
                    approximatedEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("ApproximatedEquations")),
            )?;
            BackendDump::dumpEquationList(
                &(List::map1r(
                    boundaryConditionEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("boundaryConditionEquations")),
            )?;
        }
        approximatedEquations = List::flatten(List::map1r(
            approximatedEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionEquations = List::flatten(List::map1r(
            boundaryConditionEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionTaggedEquationSolvedVars =
            getBoundaryConditionVariables(&boundaryConditionEquations, &solvedEqsAndVarsInfo)?;
        if debug {
            metamodelica::print(literal!(
                "\nApproximated and BoundaryCondition Equation Indexes :\n==========================================="
            ));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :"));
                __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\n"));
        }
        (
            knowns,
            boundaryConditionVars,
            exactEquationVars,
            unMeasuredVariablesOfInterest,
        ) = getVariablesBlockCategories(&currentSystem.orderedVars, &allVarsList)?;
        boundaryConditionVars = listAppend(boundaryConditionVars, boundaryConditionTaggedEquationSolvedVars);
        if debug {
            metamodelica::print(literal!("\nVariablesCategories\n============================="));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nknownVars                    :"));
                __mm_s.push_str(&*dumplistInteger(knowns.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nboundaryConditionVars        :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nexactEquationVars            :"));
                __mm_s.push_str(&*dumplistInteger(exactEquationVars)?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nadjacencyMatrix              :"));
                __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        dumpSetSVarsSolvedInfo(
            &matchedEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Standard BLT of the original model")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                knowns.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Variables of interest")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                boundaryConditionVars.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Boundary conditions")),
        )?;
        dumpSetSVarsSolvedInfo(
            &bindingEquations,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Binding equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                approximatedEquations.clone(),
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("Approximated equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                boundaryConditionEquations,
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("boundary condition equations")),
        )?;
        ebltEqsLst = getEBLTEquations(
            knowns.clone(),
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            &currentSystem,
        );
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst, &bindingEquations, &fnptr!(intEq, i32, i32))?;
        dumpSetSVarsSolvedInfo(
            &ebltEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("E-BLT: equations that compute the variables of interest")),
        )?;
        (
            currentSystem,
            tempSetS,
            mappedEbltSetS,
            status,
            setBFailedBoundaryConditionEquations,
        ) = traverseEBLTAndExtractSetCAndSetS(
            currentSystem.clone(),
            &ebltEqsLst,
            &sBltAdjacencyMatrix,
            &knowns,
            boundaryConditionVars.clone(),
            &(currentSystem.orderedVars.clone()),
            currentSystem.orderedEqs.clone(),
            mapIncRowEqn.clone(),
            &solvedEqsAndVarsInfo,
            debug,
            setBFailedBoundaryConditionEquations,
            bindingEquations.clone(),
        )?;
        if !(status) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: "));
                __mm_s.push_str(&*intString(procedureCount));
                __mm_s.push_str(&*literal!(", re-running with modified model\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        procedureCount = procedureCount + 1;
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nExtraction procedure is successfully completed in iteration count: "
        ));
        __mm_s.push_str(&*intString(procedureCount - 1));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpFailedBoundaryConditionEquationAndVars(
        setBFailedBoundaryConditionEquations.clone(),
        &currentSystem.orderedVars,
        &(metamodelica::nil()),
        false,
    )?;
    (
        _,
        setSPrime,
        failedboundaryConditionEquations,
        failedboundaryConditionVars,
        status,
    ) = ExtractSetSPrime(
        currentSystem.clone(),
        setBFailedBoundaryConditionEquations.clone(),
        &sBltAdjacencyMatrix,
        &knowns,
        boundaryConditionVars.clone(),
        &(currentSystem.orderedVars.clone()),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        &solvedEqsAndVarsInfo,
        bindingEquations,
        debug,
    )?;
    setSPrime = List::setDifferenceOnTrue(setSPrime, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    if debug {
        dumpSetSVarsSolvedInfo(
            &setSPrime,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Set-S Solved-Variables Information")),
        )?;
    }
    setS = List::unique(&(getAbsoluteIndexHelper(&setSPrime, mapIncRowEqn.clone())?));
    setS_Eq = getEquationsFromSBLTAndEBLT(&setS, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    BackendDump::dumpEquationArray(
        BackendEquation::listEquation(&failedboundaryConditionEquations)?,
        &(literal!("SET_B")),
    )?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setS_Eq)?, &(literal!("SET_S'")))?;
    paramVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&(listAppend(failedboundaryConditionEquations.clone(), setS_Eq.clone())))?,
        shared.globalKnownVars.clone(),
    )?;
    setSVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&(listAppend(failedboundaryConditionEquations.clone(), setS_Eq.clone())))?,
        currentSystem.orderedVars.clone(),
    )?;
    (knownVars, setSVars) = List::extractOnTrue(
        &setSVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varHasUncertainValueRefine(&__a0))
        },
    )?;
    (_, setSVars) = List::extract1OnTrue(
        &setSVars,
        &fnptr!(
            isBoundaryConditionVars,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ),
        failedboundaryConditionVars.clone(),
    )?;
    (extraVarsinSetSPrime, _) = List::extract1OnTrue(
        &setSVars,
        &fnptr!(
            isBoundaryConditionVars,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ),
        List::map1r(
            boundaryConditionVars.reverse(),
            &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
            currentSystem.orderedVars.clone(),
        )?,
    )?;
    BackendDump::dumpVarList(&failedboundaryConditionVars, &(literal!("Boundary condition Vars'")))?;
    BackendDump::dumpVarList(&setSVars, &(literal!("Intermediate vars in set-S'")))?;
    BackendDump::dumpVarList(&knownVars, &(literal!("Known vars in set-S'")))?;
    BackendDump::dumpVarList(&paramVars, &(literal!("Param vars in set-S'")))?;
    unMeasuredVariables = List::map1r(
        unMeasuredVariablesOfInterest.reverse(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?;
    outDiffVars = BackendVariable::listVar(List::map1r(
        knowns,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    outDiffVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(&shared)?;
    outDiffVars = setStartValuesToMeasurements(&outDiffVars, &measurementcsvData, &csvfileName)?;
    outBoundaryConditionVars = BackendVariable::listVar(List::map1(
        failedboundaryConditionVars.clone().reverse(),
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    outBoundaryConditionEquations = BackendEquation::listEquation(&failedboundaryConditionEquations)?;
    outOtherEqns = BackendEquation::listEquation(&setS_Eq)?;
    outOtherVars = BackendVariable::listVar(setSVars)?;
    auxillaryConditionsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html"));
        ArcStr::from(__mm_s)
    };
    auxillaryEquations = dumpExtractedEquationsToHTML(
        outBoundaryConditionEquations.clone(),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Boundary conditions"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                outBoundaryConditionEquations.clone(),
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                outBoundaryConditionEquations.clone(),
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(auxillaryConditionsFilename, auxillaryEquations)?;
    intermediateEquationsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html"));
        ArcStr::from(__mm_s)
    };
    intermediateEquations = dumpExtractedEquationsToHTML(
        outOtherEqns.clone(),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Intermediate equations"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(outOtherEqns.clone())));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(outOtherEqns.clone())?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(intermediateEquationsFilename, intermediateEquations)?;
    VerifySetSPrime(
        &outBoundaryConditionVars,
        &outOtherVars,
        &outDiffVars,
        &extraVarsinSetSPrime,
        outBoundaryConditionEquations.clone(),
        outOtherEqns.clone(),
        &shared,
        ((ebltEqsLst).len() as i32),
        ((setBFailedBoundaryConditionEquations).len() as i32),
        false,
    )?;
    if debug {
        BackendDump::dumpVariables(&outDiffVars, &(literal!("Jacobian_knownVariables")))?;
        BackendDump::dumpEquationArray(
            outBoundaryConditionEquations.clone(),
            &(literal!("Jacobian_ResidualEquation")),
        )?;
        BackendDump::dumpVariables(&outBoundaryConditionVars, &(literal!("Jacobian_outResidualVars")))?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), &(literal!("Jacobian_outOtherEquations")))?;
        BackendDump::dumpVariables(&outOtherVars, &(literal!("Jacobian_outOtherVars")))?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(
        &outDiffVars,
        outBoundaryConditionEquations.clone(),
        outBoundaryConditionVars.clone(),
        outOtherEqns.clone(),
        outOtherVars.clone(),
        shared,
        &(outOtherVars.clone()),
        literal!("F"),
        false,
    )?;
    assign_field!(
        shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData {
            symbolicJacobian: simCodeJacobian,
            setcVars: outBoundaryConditionVars.clone(),
            datareconinputs: outDiffVars.clone(),
            setBVars: Some(BackendVariable::listVar(unMeasuredVariables)?),
            symbolicJacobianH: None,
            relatedBoundaryConditions: ((setBFailedBoundaryConditionEquations).len() as i32)
        })
    );
    currentSystem = BackendDAEUtil::setEqSystEqs(
        currentSystem,
        BackendEquation::merge(outBoundaryConditionEquations, outOtherEqns.clone())?,
    );
    currentSystem = BackendDAEUtil::setEqSystVars(
        currentSystem,
        BackendVariable::mergeVariables(outBoundaryConditionVars.clone(), outOtherVars.clone(), true)?,
    );
    assign_field!(currentSystem.removedEqs = BackendEquation::emptyEqns());
    inputVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarDirection,
            metamodelica::Ref<BackendDAE::Var>,
            DAE::VarDirection
        ),
        openmodelica_frontend_types::DAE::VarDirection::INPUT,
    )?)?;
    shared = BackendDAEUtil::setSharedGlobalKnownVars(
        shared,
        BackendVariable::mergeVariables(BackendVariable::listVar(paramVars.clone())?, inputVars, true)?,
    );
    r#str = dumpToCsv(&(literal!("")), &(BackendVariable::varList(&outBoundaryConditionVars)?))?;
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*shared.info.fileNamePrefix);
            __mm_s.push_str(&*literal!("_BoundaryConditionVars.txt"));
            ArcStr::from(__mm_s)
        },
        r#str,
    )?;
    modelicaFileName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Reconciled_tmp"));
        ArcStr::from(__mm_s)
    };
    modelName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Reconciled_"));
        __mm_s.push_str(&*System::stringReplace(
            shared.info.fileNamePrefix.clone(),
            literal!("."),
            literal!("_"),
        )?);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = literal!(
        "/* This is a Reconciled Model which is generated by the Boundary condition extraction algorithm */\n"
    );
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("model "));
        __mm_s.push_str(&*modelName);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outDiffVars)?),
        &(literal!("Variables of Interest")),
    )?;
    modelicaOutput = dumpExtractedVars(&modelicaOutput, &paramVars, &(literal!("parameters in SET-S")))?;
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &failedboundaryConditionVars,
        &(literal!("boundary condition Vars")),
    )?;
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outOtherVars)?),
        &(literal!("remaining variables in setS")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nequation"));
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        BackendEquation::listEquation(&failedboundaryConditionEquations)?,
        &(literal!("boundary condition equations")),
    )?;
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        outOtherEqns,
        &(literal!("remaining equations in Set-S'")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nend "));
        __mm_s.push_str(&*modelName);
        __mm_s.push_str(&*literal!(";"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*modelicaFileName);
            __mm_s.push_str(&*literal!(".mo"));
            ArcStr::from(__mm_s)
        },
        modelicaOutput,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![currentSystem],
        shared: shared,
    });
    Ok(outDAE)
}

pub(crate) fn stateEstimation(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outOtherEqnsSetSPrime: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut outResidualEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut outBoundaryConditionEquations: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut setC_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut setS_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut setSPrime_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut residualEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut failedboundaryConditionEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut allDaeEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut varCount: i32;
    let mut eqCount: i32;
    let mut ebltEqsLst: metamodelica::List<i32> = metamodelica::nil();
    let mut matchedEqsLst: metamodelica::List<i32>;
    let mut approximatedEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut setC: metamodelica::List<i32>;
    let mut tempSetS: metamodelica::List<i32> = metamodelica::nil();
    let mut setS: metamodelica::List<i32>;
    let mut boundaryConditionEquations: metamodelica::List<i32>;
    let mut bindingEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut setSPrime: metamodelica::List<i32>;
    let mut unMeasuredEqsLst: metamodelica::List<i32>;
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut paramVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut setSVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut residualVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut knownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut failedboundaryConditionVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut extraVarsinSetSPrime: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut unMeasuredVariables: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut simCodeJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut simCodeJacobianH: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut r#str: ArcStr;
    let mut modelicaOutput: ArcStr;
    let mut modelicaFileName: ArcStr;
    let mut modelName: ArcStr;
    let mut auxillaryConditionsFilename: ArcStr;
    let mut auxillaryEquations: ArcStr;
    let mut intermediateEquationsFilename: ArcStr;
    let mut intermediateEquations: ArcStr;
    let mut csvfileName: ArcStr;
    let mut mappedEbltSetS: metamodelica::List<(i32, metamodelica::List<i32>)>;
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>;
    let mut allVarsList: metamodelica::List<i32>;
    let mut knowns: metamodelica::List<i32> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: metamodelica::List<i32> = metamodelica::nil();
    let mut failedboundaryConditionEquationIndex: metamodelica::List<i32>;
    let mut boundaryConditionVars: metamodelica::List<i32> = metamodelica::nil();
    let mut exactEquationVars: metamodelica::List<i32> = metamodelica::nil();
    let mut extractedVarsfromSetS: metamodelica::List<i32>;
    let mut boundaryConditionTaggedEquationSolvedVars: metamodelica::List<i32>;
    let mut inputVars: BackendDAE::Variables;
    let mut outDiffVars: BackendDAE::Variables;
    let mut outOtherVars: BackendDAE::Variables;
    let mut outResidualVars: BackendDAE::Variables;
    let mut outBoundaryConditionVars: BackendDAE::Variables;
    let mut outOtherVarsSetSPrime: BackendDAE::Variables;
    let mut procedureCount: i32;
    let mut numRelatedBoundaryConditions: i32;
    let mut measurementcsvData: metamodelica::List<(ArcStr, ArcStr)>;
    let mut debug: bool = false;
    let mut status: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    currentSystem = metamodelica::Own::own(__pa0);
    shared = inDAE.shared.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nModelInfo: "));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem, inDAE.shared.clone(), debug)?;
    procedureCount = 1;
    setBFailedBoundaryConditionEquations = metamodelica::nil();
    while !(status) {
        BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("OrderedVariables")))?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), &(literal!("OrderedEquation")))?;
        allVarsList = List::intRange(BackendVariable::varsSize(&currentSystem.orderedVars));
        varCount = currentSystem.orderedVars.numberOfVars.clone();
        eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
        (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
            &currentSystem,
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
        (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount, eqCount)?;
        BackendDump::dumpMatching(match1.clone())?;
        (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
        bindingEquations = getBindingEquation(&currentSystem, mapIncRowEqn.clone())?;
        bindingEquations = List::flatten(List::map1r(
            bindingEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(
            &(BackendEquation::equationList(currentSystem.orderedEqs.clone())?),
            1,
        );
        if debug {
            BackendDump::dumpEquationList(
                &(List::map1r(
                    approximatedEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("ApproximatedEquations")),
            )?;
            BackendDump::dumpEquationList(
                &(List::map1r(
                    boundaryConditionEquations.clone(),
                    &BackendEquation::get,
                    currentSystem.orderedEqs.clone(),
                )?),
                &(literal!("boundaryConditionEquations")),
            )?;
        }
        approximatedEquations = List::flatten(List::map1r(
            approximatedEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionEquations = List::flatten(List::map1r(
            boundaryConditionEquations,
            &listGet,
            mapEqnIncRow
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?)?;
        boundaryConditionTaggedEquationSolvedVars =
            getBoundaryConditionVariables(&boundaryConditionEquations, &solvedEqsAndVarsInfo)?;
        if debug {
            metamodelica::print(literal!(
                "\nApproximated and BoundaryCondition Equation Indexes :\n==========================================="
            ));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :"));
                __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\n"));
        }
        (
            knowns,
            boundaryConditionVars,
            exactEquationVars,
            unMeasuredVariablesOfInterest,
        ) = getVariablesBlockCategories(&currentSystem.orderedVars, &allVarsList)?;
        boundaryConditionVars = listAppend(boundaryConditionVars, boundaryConditionTaggedEquationSolvedVars);
        if debug {
            metamodelica::print(literal!("\nVariablesCategories\n============================="));
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nknownVars                    :"));
                __mm_s.push_str(&*dumplistInteger(knowns.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nunMeasuredVars               :"));
                __mm_s.push_str(&*dumplistInteger(unMeasuredVariablesOfInterest.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nboundaryConditionVars        :"));
                __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nexactEquationVars            :"));
                __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())?);
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nadjacencyMatrix              :"));
                __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        dumpSetSVarsSolvedInfo(
            &matchedEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Standard BLT of the original model")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                knowns.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Variables of interest")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                unMeasuredVariablesOfInterest.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("unMeasured Variables of interest")),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                boundaryConditionVars.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("Boundary conditions")),
        )?;
        dumpSetSVarsSolvedInfo(
            &bindingEquations,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Binding equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                approximatedEquations.clone(),
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("Approximated equations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                boundaryConditionEquations,
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("boundary condition equations")),
        )?;
        ebltEqsLst = getEBLTEquations(
            knowns.clone(),
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            &currentSystem,
        );
        ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst, &bindingEquations, &fnptr!(intEq, i32, i32))?;
        dumpSetSVarsSolvedInfo(
            &ebltEqsLst,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("E-BLT: equations that compute the variables of interest")),
        )?;
        (
            currentSystem,
            tempSetS,
            mappedEbltSetS,
            status,
            setBFailedBoundaryConditionEquations,
        ) = traverseEBLTAndExtractSetCAndSetS(
            currentSystem.clone(),
            &ebltEqsLst,
            &sBltAdjacencyMatrix,
            &knowns,
            boundaryConditionVars.clone(),
            &(currentSystem.orderedVars.clone()),
            currentSystem.orderedEqs.clone(),
            mapIncRowEqn.clone(),
            &solvedEqsAndVarsInfo,
            debug,
            setBFailedBoundaryConditionEquations,
            bindingEquations.clone(),
        )?;
        if !(status) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nExtraction procedure failed for iteration count: "));
                __mm_s.push_str(&*intString(procedureCount));
                __mm_s.push_str(&*literal!(", re-running with modified model\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        procedureCount = procedureCount + 1;
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nExtraction procedure is successfully completed in iteration count: "
        ));
        __mm_s.push_str(&*intString(procedureCount - 1));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    ebltEqsLst = List::setDifferenceOnTrue(ebltEqsLst, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    tempSetS = List::setDifferenceOnTrue(tempSetS, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), &sBltAdjacencyMatrix);
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS, &knowns, &fnptr!(intEq, i32, i32))?;
    setC = List::unique(&(getAbsoluteIndexHelper(&ebltEqsLst, mapIncRowEqn.clone())?));
    setS = List::unique(&(getAbsoluteIndexHelper(&tempSetS, mapIncRowEqn.clone())?));
    setC_Eq = getEquationsFromSBLTAndEBLT(&setC, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    setS_Eq = getEquationsFromSBLTAndEBLT(&setS, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nFinal set of equations after extraction algorithm\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_C: "));
        __mm_s.push_str(&*dumplistInteger(setC.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_S: "));
        __mm_s.push_str(&*dumplistInteger(setS.clone())?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setC_Eq)?, &(literal!("SET_C")))?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setS_Eq)?, &(literal!("SET_S")))?;
    outDiffVars = BackendVariable::listVar(List::map1r(
        knowns.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    outDiffVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    (csvfileName, measurementcsvData) = readMeasurementsFromCSV(&shared)?;
    outDiffVars = setStartValuesToMeasurements(&outDiffVars, &measurementcsvData, &csvfileName)?;
    (_, residualEquations) = BackendEquation::traverseEquationArray(
        BackendEquation::listEquation(&setC_Eq)?,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::Ref<AvlTreePathFunction::Tree>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1),
        (shared.functionTree.clone(), metamodelica::nil()),
    )?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(
        &(residualEquations.reverse()),
        &(literal!("$res_F_")),
        1,
        false,
    )?;
    outResidualVars = BackendVariable::listVar(residualVars.clone().reverse())?;
    outResidualEqns = BackendEquation::listEquation(&residualEquations)?;
    outOtherEqns = BackendEquation::listEquation(&setS_Eq)?;
    paramVars = BackendEquation::equationsVars(
        BackendEquation::merge(outOtherEqns.clone(), outResidualEqns.clone())?,
        shared.globalKnownVars.clone(),
    )?;
    outOtherVars = BackendVariable::listVar(List::map1r(
        extractedVarsfromSetS,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    dumpSetSVars(&outOtherVars, &(literal!("Unknown variables in SET_S")))?;
    BackendDump::dumpVariables(
        &(BackendVariable::listVar(paramVars)?),
        &(literal!("Parameters in SET_S")),
    )?;
    auxillaryConditionsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_AuxiliaryConditions.html"));
        ArcStr::from(__mm_s)
    };
    auxillaryEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setC_Eq)?,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Auxiliary conditions"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                BackendEquation::listEquation(&setC_Eq)?,
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                BackendEquation::listEquation(&setC_Eq)?,
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(auxillaryConditionsFilename, auxillaryEquations)?;
    intermediateEquationsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_IntermediateEquations.html"));
        ArcStr::from(__mm_s)
    };
    intermediateEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setS_Eq)?,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Intermediate equations for measured variables"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                BackendEquation::listEquation(&setS_Eq)?,
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                BackendEquation::listEquation(&setS_Eq)?,
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(intermediateEquationsFilename, intermediateEquations)?;
    dumpRelatedBoundaryConditionsEquations(&setBFailedBoundaryConditionEquations, &shared.info.fileNamePrefix)?;
    numRelatedBoundaryConditions = ((setBFailedBoundaryConditionEquations).len() as i32);
    VerifyDataReconciliation(
        &ebltEqsLst,
        &tempSetS,
        knowns.clone(),
        &boundaryConditionVars,
        &sBltAdjacencyMatrix,
        &solvedEqsAndVarsInfo,
        &exactEquationVars,
        &approximatedEquations,
        currentSystem.orderedVars.clone(),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        &outOtherVars,
        &setS_Eq,
        &shared,
        setC.clone(),
        setS,
        ((unMeasuredVariablesOfInterest).len() as i32),
    )?;
    unMeasuredEqsLst = getEBLTEquations(
        unMeasuredVariablesOfInterest.clone(),
        &solvedEqsAndVarsInfo,
        mapIncRowEqn.clone(),
        &currentSystem,
    );
    unMeasuredEqsLst = List::setDifferenceOnTrue(unMeasuredEqsLst, &bindingEquations, &fnptr!(intEq, i32, i32))?;
    unMeasuredVariables = List::map1r(
        unMeasuredVariablesOfInterest.reverse(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?;
    dumpFailedBoundaryConditionEquationAndVars(
        setBFailedBoundaryConditionEquations.clone(),
        &currentSystem.orderedVars,
        &unMeasuredVariables,
        true,
    )?;
    (
        setBFailedBoundaryConditionEquations,
        failedboundaryConditionEquationIndex,
    ) = prepareUnmeasuredVariablesEquations(
        &unMeasuredEqsLst,
        &sBltAdjacencyMatrix,
        &knowns,
        &solvedEqsAndVarsInfo,
        currentSystem.orderedEqs.clone(),
        &(currentSystem.orderedVars.clone()),
        mapIncRowEqn.clone(),
        setBFailedBoundaryConditionEquations,
    )?;
    dumpSetSVarsSolvedInfo(
        &unMeasuredEqsLst,
        &solvedEqsAndVarsInfo,
        mapIncRowEqn.clone(),
        currentSystem.orderedEqs.clone(),
        &(currentSystem.orderedVars.clone()),
        &(literal!("E-BLT: equations in the BLT that compute the unmeasured variables of interest")),
    )?;
    (
        _,
        setSPrime,
        failedboundaryConditionEquations,
        failedboundaryConditionVars,
        status,
    ) = ExtractSetSPrime(
        currentSystem.clone(),
        setBFailedBoundaryConditionEquations,
        &sBltAdjacencyMatrix,
        &knowns,
        boundaryConditionVars.clone(),
        &(currentSystem.orderedVars.clone()),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        &solvedEqsAndVarsInfo,
        bindingEquations,
        debug,
    )?;
    setSPrime = List::unique(&(listAppend(failedboundaryConditionEquationIndex, setSPrime)));
    setSPrime = List::setDifferenceOnTrue(setSPrime, &approximatedEquations, &fnptr!(intEq, i32, i32))?;
    setSPrime = List::setDifferenceOnTrue(setSPrime, &unMeasuredEqsLst, &fnptr!(intEq, i32, i32))?;
    if debug {
        dumpSetSVarsSolvedInfo(
            &setSPrime,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Set-SPrime Solved-Variables Information")),
        )?;
    }
    setSPrime = List::unique(&(getAbsoluteIndexHelper(&setSPrime, mapIncRowEqn.clone())?));
    setSPrime_Eq = getEquationsFromSBLTAndEBLT(&setSPrime, currentSystem.orderedEqs.clone(), &(metamodelica::nil()))?;
    BackendDump::dumpEquationArray(
        BackendEquation::listEquation(&failedboundaryConditionEquations)?,
        &(literal!("SET_B")),
    )?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setSPrime_Eq)?, &(literal!("SET_SPrime")))?;
    paramVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&setSPrime_Eq)?,
        shared.globalKnownVars.clone(),
    )?;
    setSVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&setSPrime_Eq)?,
        currentSystem.orderedVars.clone(),
    )?;
    (knownVars, setSVars) = List::extractOnTrue(
        &setSVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varHasUncertainValueRefine(&__a0))
        },
    )?;
    (_, setSVars) = List::extract1OnTrue(
        &setSVars,
        &fnptr!(
            isBoundaryConditionVars,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ),
        failedboundaryConditionVars,
    )?;
    (extraVarsinSetSPrime, _) = List::extract1OnTrue(
        &setSVars,
        &fnptr!(
            isBoundaryConditionVars,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ),
        List::map1r(
            boundaryConditionVars.reverse(),
            &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
            currentSystem.orderedVars.clone(),
        )?,
    )?;
    if debug {
        BackendDump::dumpVarList(&unMeasuredVariables, &(literal!("unmeasured variables")))?;
        BackendDump::dumpVarList(&setSVars, &(literal!("Intermediate vars in set-S'")))?;
        BackendDump::dumpVarList(&knownVars, &(literal!("Known vars in set-S'")))?;
        BackendDump::dumpVarList(&paramVars, &(literal!("Param vars in set-S'")))?;
        BackendDump::dumpVarList(&extraVarsinSetSPrime, &(literal!("extra vars in set-S'")))?;
    }
    outBoundaryConditionVars = BackendVariable::listVar(List::map1(
        unMeasuredVariables.reverse(),
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    outBoundaryConditionEquations = BackendEquation::listEquation(&failedboundaryConditionEquations)?;
    outOtherEqnsSetSPrime = BackendEquation::listEquation(&setSPrime_Eq)?;
    outOtherVarsSetSPrime = BackendVariable::listVar(setSVars)?;
    dumpSetSVars(&outOtherVarsSetSPrime, &(literal!("Unknown variables in SET_SPrime")))?;
    auxillaryConditionsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_BoundaryConditionsEquations.html"));
        ArcStr::from(__mm_s)
    };
    auxillaryEquations = dumpExtractedEquationsToHTML(
        outBoundaryConditionEquations.clone(),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Boundary conditions"));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                outBoundaryConditionEquations.clone(),
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                outBoundaryConditionEquations.clone(),
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(auxillaryConditionsFilename, auxillaryEquations)?;
    intermediateEquationsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html"));
        ArcStr::from(__mm_s)
    };
    intermediateEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone())))?,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Intermediate equations for unmeasured variables "));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                BackendEquation::listEquation(
                    &(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone())),
                )?,
            )));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                BackendEquation::listEquation(
                    &(listAppend(failedboundaryConditionEquations.clone(), setSPrime_Eq.clone())),
                )?,
            )?));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    System::writeFile(intermediateEquationsFilename, intermediateEquations)?;
    VerifySetSPrime(
        &outBoundaryConditionVars,
        &outOtherVarsSetSPrime,
        &outDiffVars,
        &extraVarsinSetSPrime,
        outBoundaryConditionEquations.clone(),
        outOtherEqnsSetSPrime.clone(),
        &shared,
        ((setC).len() as i32),
        numRelatedBoundaryConditions,
        true,
    )?;
    if debug {
        BackendDump::dumpVariables(&outDiffVars, &(literal!("Jacobian_knownVariables")))?;
        BackendDump::dumpVariables(&outResidualVars, &(literal!("Jacobian_outResidualVars")))?;
        BackendDump::dumpVariables(&outOtherVars, &(literal!("Jacobian_outOtherVars")))?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), &(literal!("Jacobian_ResidualEquation")))?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), &(literal!("Jacobian_other_Equation")))?;
    }
    if debug {
        BackendDump::dumpVariables(&outDiffVars, &(literal!("Jacobian_knownVariables")))?;
        BackendDump::dumpEquationArray(
            outBoundaryConditionEquations.clone(),
            &(literal!("Jacobian_ResidualEquation")),
        )?;
        BackendDump::dumpVariables(&outBoundaryConditionVars, &(literal!("Jacobian_outResidualVars")))?;
        BackendDump::dumpEquationArray(outOtherEqnsSetSPrime.clone(), &(literal!("Jacobian_outOtherEquations")))?;
        BackendDump::dumpVariables(&outOtherVarsSetSPrime, &(literal!("Jacobian_outOtherVars")))?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(
        &outDiffVars,
        outResidualEqns,
        outResidualVars.clone(),
        outOtherEqns,
        outOtherVars.clone(),
        shared,
        &(outOtherVars),
        literal!("F"),
        false,
    )?;
    (simCodeJacobianH, shared) = SymbolicJacobian::getSymbolicJacobian(
        &outDiffVars,
        outBoundaryConditionEquations,
        outBoundaryConditionVars.clone(),
        outOtherEqnsSetSPrime,
        outOtherVarsSetSPrime.clone(),
        shared,
        &(outOtherVarsSetSPrime),
        literal!("H"),
        false,
    )?;
    assign_field!(
        shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData {
            symbolicJacobian: simCodeJacobian,
            setcVars: outResidualVars.clone(),
            datareconinputs: outDiffVars.clone(),
            setBVars: Some(outBoundaryConditionVars.clone()),
            symbolicJacobianH: Some(simCodeJacobianH),
            relatedBoundaryConditions: numRelatedBoundaryConditions
        })
    );
    setSPrime_Eq = List::unique(&(listAppend(setSPrime_Eq, failedboundaryConditionEquations)));
    setSPrime_Eq = List::unique(&(listAppend(setSPrime_Eq, setS_Eq)));
    allDaeEqs = List::unique(&(listAppend(setSPrime_Eq.clone(), residualEquations.clone())));
    BackendDump::dumpEquationArray(
        BackendEquation::listEquation(&allDaeEqs)?,
        &(literal!("Final DAE with set-c, set-S and set-SPrime combined")),
    )?;
    paramVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&allDaeEqs)?,
        shared.globalKnownVars.clone(),
    )?;
    setSVars = BackendEquation::equationsVars(
        BackendEquation::listEquation(&allDaeEqs)?,
        currentSystem.orderedVars.clone(),
    )?;
    (knownVars, setSVars) = List::extractOnTrue(
        &setSVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varHasUncertainValueRefine(&__a0))
        },
    )?;
    (_, setSVars) = List::extractOnTrue(
        &setSVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varHasUncertainValuePropagate(&__a0))
        },
    )?;
    setSVars = listAppend(BackendVariable::varList(&outBoundaryConditionVars)?, setSVars);
    BackendDump::dumpVarList(
        &(listAppend(setSVars.clone(), residualVars.clone())),
        &(literal!("Intermediate vars in final DAE updated'")),
    )?;
    BackendDump::dumpVarList(&paramVars, &(literal!("parameters in final DAE updated")))?;
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem, BackendEquation::listEquation(&allDaeEqs)?);
    currentSystem = BackendDAEUtil::setEqSystVars(
        currentSystem,
        BackendVariable::listVar(listAppend(setSVars.clone(), residualVars))?,
    );
    assign_field!(currentSystem.removedEqs = BackendEquation::emptyEqns());
    inputVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarDirection,
            metamodelica::Ref<BackendDAE::Var>,
            DAE::VarDirection
        ),
        openmodelica_frontend_types::DAE::VarDirection::INPUT,
    )?)?;
    shared = BackendDAEUtil::setSharedGlobalKnownVars(
        shared,
        BackendVariable::mergeVariables(BackendVariable::listVar(paramVars.clone())?, inputVars, true)?,
    );
    if debug {
        BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("FinalOrderedVariables")))?;
        BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), &(literal!("FinalOrderedEquation")))?;
        BackendDump::dumpVariables(&shared.globalKnownVars, &(literal!("FinalGlobalKnownVars")))?;
    }
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Inputs.csv"));
        ArcStr::from(__mm_s)
    })) {
        r#str = literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n");
        r#str = dumpToCsv(&r#str, &(BackendVariable::varList(&outDiffVars)?))?;
        r#str = dumpToCsv(&r#str, &(BackendVariable::varList(&outBoundaryConditionVars)?))?;
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_Inputs.csv"));
                ArcStr::from(__mm_s)
            },
            r#str,
        )?;
    }
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Correlation_Inputs.csv"));
        ArcStr::from(__mm_s)
    })) {
        r#str = dumpCorrelationVarsToCsv(&(BackendVariable::varList(&outDiffVars)?))?;
        r#str = dumpToCsv(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }),
            &(BackendVariable::varList(&outDiffVars)?),
        )?;
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_Correlation_Inputs.csv"));
                ArcStr::from(__mm_s)
            },
            r#str,
        )?;
    }
    r#str = dumpToCsv(&(literal!("")), &(BackendVariable::varList(&outBoundaryConditionVars)?))?;
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*shared.info.fileNamePrefix);
            __mm_s.push_str(&*literal!("_BoundaryConditionVars.txt"));
            ArcStr::from(__mm_s)
        },
        r#str,
    )?;
    modelicaFileName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Reconciled_tmp"));
        ArcStr::from(__mm_s)
    };
    modelName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Reconciled_"));
        __mm_s.push_str(&*System::stringReplace(
            shared.info.fileNamePrefix.clone(),
            literal!("."),
            literal!("_"),
        )?);
        ArcStr::from(__mm_s)
    };
    modelicaOutput =
        literal!("/* This is a Reconciled Model which is generated by the State Estimation extraction algorithm */\n");
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("model "));
        __mm_s.push_str(&*modelName);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outDiffVars)?),
        &(literal!("Variables of Interest")),
    )?;
    modelicaOutput = dumpExtractedVars(&modelicaOutput, &paramVars, &(literal!("parameters")))?;
    modelicaOutput = dumpResidualVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outResidualVars)?),
        &(literal!("residualVars")),
    )?;
    modelicaOutput = dumpExtractedVars(&modelicaOutput, &setSVars, &(literal!("intermediate variables")))?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nequation"));
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        BackendEquation::listEquation(&residualEquations)?,
        &(literal!("residual equations")),
    )?;
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        BackendEquation::listEquation(&setSPrime_Eq)?,
        &(literal!("remaining equations in Set-S'")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nend "));
        __mm_s.push_str(&*modelName);
        __mm_s.push_str(&*literal!(";"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*modelicaFileName);
            __mm_s.push_str(&*literal!(".mo"));
            ArcStr::from(__mm_s)
        },
        modelicaOutput,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![currentSystem],
        shared: shared,
    });
    Ok(outDAE)
}

fn isBoundaryConditionVars(
    mut setSVars: metamodelica::Ref<BackendDAE::Var>,
    mut boundaryConditionsVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> bool {
    let mut result: bool = false;
    if listMember(setSVars, boundaryConditionsVars) {
        result = true;
    }
    result
}

fn dumpFailedBoundaryConditionEquationAndVars(
    mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut orderedVars: &BackendDAE::Variables,
    mut unmeasuredVariables: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut stateEstimation: bool,
) -> Result<()> {
    let mut failedboundaryConditionEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut count: i32;
    let mut varIndex: i32;
    let mut varlist: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if stateEstimation {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nStart of extraction procedure for unmeasured variables of interest\nSet of equations that failed the extraction of set S and that contain an unmeasured variable of interest: ("));
            __mm_s.push_str(&*intString(((setBFailedBoundaryConditionEquations).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            ArcStr::from(__mm_s)
        });
    } else {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nStart of extraction procedure for boundary conditions\nSet of boundary conditions equations that failed the extraction of set S: ("));
            __mm_s.push_str(&*intString(((setBFailedBoundaryConditionEquations).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            ArcStr::from(__mm_s)
        });
    }
    count = 1;
    varlist = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations.reverse() {
        (varIndex, failedboundaryConditionEquation, _) = item.clone();
        varlist = metamodelica::cons(BackendVariable::getVarAt(orderedVars, varIndex)?, varlist);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*intString(count));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*BackendDump::equationString(&failedboundaryConditionEquation)?);
            ArcStr::from(__mm_s)
        });
        count = count + 1;
    }
    metamodelica::print(literal!("\n"));
    if stateEstimation {
        BackendDump::dumpVarList(unmeasuredVariables, &(literal!("umeasured variables to be computed")))?;
    } else {
        BackendDump::dumpVarList(&(varlist.reverse()), &(literal!("Boundary conditions to be computed")))?;
    }
    Ok(())
}

fn prepareUnmeasuredVariablesEquations(
    mut unMeasuredEqsLst: &metamodelica::List<i32>,
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut knownVars: &metamodelica::List<i32>,
    mut solvedEqsAndVarsInfo: &metamodelica::List<(i32, i32)>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut orderedVars: &BackendDAE::Variables,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
) -> Result<(
    metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<i32>)>,
    metamodelica::List<i32>,
)> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )> = setBFailedBoundaryConditionEquations;
    let mut failedboundaryConditionEquationIndex: metamodelica::List<i32> = metamodelica::nil();
    let mut varIndex: i32;
    let mut intermediateVars: metamodelica::List<i32>;
    let mut unmeasuredEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut unMeasuredVariablesAndEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>;
    for mut eq in &**unMeasuredEqsLst {
        varIndex = getSolvedVariableNumber(eq.clone(), solvedEqsAndVarsInfo)?;
        intermediateVars = getVariablesAfterExtraction(list![eq.clone()], metamodelica::nil(), sBltAdjacencyMatrix);
        intermediateVars = List::setDifferenceOnTrue(intermediateVars, knownVars, &fnptr!(intEq, i32, i32))?.reverse();
        unmeasuredEq = BackendEquation::get(
            orderedEqs.clone(),
            (*metamodelica::index_checked(&__ab_mapIncRowEqn, eq.clone())?).clone(),
        )?;
        setBFailedBoundaryConditionEquations = metamodelica::cons(
            (varIndex, unmeasuredEq, intermediateVars),
            setBFailedBoundaryConditionEquations,
        );
    }
    unMeasuredVariablesAndEquations = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations {
        (varIndex, _, _) = item.clone();
        if BackendVariable::varHasUncertainValuePropagate(&(BackendVariable::getVarAt(orderedVars, varIndex)?)) {
            unMeasuredVariablesAndEquations = metamodelica::cons(item.clone(), unMeasuredVariablesAndEquations);
        }
    }
    setBFailedBoundaryConditionEquations = List::unique(&unMeasuredVariablesAndEquations);
    Ok((
        setBFailedBoundaryConditionEquations,
        failedboundaryConditionEquationIndex,
    ))
}

fn addUnmeasuredEquationtoBoundaryConditionEquationAndVars(
    mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut orderedVars: &BackendDAE::Variables,
    mut unMeasuredEqsLst: &metamodelica::List<i32>,
) -> Result<metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<i32>)>> {
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )> = setBFailedBoundaryConditionEquations;
    let mut failedboundaryConditionEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut count: i32;
    let mut varIndex: i32;
    let mut varlist: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nStart of extraction procedure for boundary conditions\nSet of boundary conditions equations that failed the extraction of set S: ("));
        __mm_s.push_str(&*intString(((setBFailedBoundaryConditionEquations).len() as i32)));
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        ArcStr::from(__mm_s)
    });
    count = 1;
    varlist = metamodelica::nil();
    for mut item in &*setBFailedBoundaryConditionEquations.clone().reverse() {
        (varIndex, failedboundaryConditionEquation, _) = item.clone();
        varlist = metamodelica::cons(BackendVariable::getVarAt(orderedVars, varIndex)?, varlist);
    }
    metamodelica::print(literal!("\n"));
    Ok(setBFailedBoundaryConditionEquations)
}

fn getEBLTEquations(
    mut knowns: metamodelica::List<i32>,
    mut solvedEqsAndVarsInfo: &metamodelica::List<(i32, i32)>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut currentSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> metamodelica::List<i32> {
    let mut ebltequations: metamodelica::List<i32> = metamodelica::nil();
    let mut eq: i32;
    let mut var: i32;
    for mut v in &**solvedEqsAndVarsInfo {
        (eq, var) = v.clone();
        if listMember(var, knowns.clone()) {
            ebltequations = metamodelica::cons(eq, ebltequations);
        }
    }
    ebltequations
}

fn getBindingEquation(
    mut currentSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut bindingEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut eq in &*BackendEquation::equationList(currentSystem.orderedEqs.clone())? {
        if BackendEquation::isBindingEquation(metamodelica::AsArg::as_arg(&eq))? {
            bindingEquations = metamodelica::cons(index, bindingEquations);
        }
        index = index + 1;
    }
    Ok(bindingEquations)
}

fn swapComplexEquationsInSetC(
    mut ebltEqsLst: metamodelica::List<i32>,
    mut tempSetS: metamodelica::List<i32>,
    mut mappedEbltSetS: &metamodelica::List<(i32, metamodelica::List<i32>)>,
    mut currentSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut ebltEqsLst: metamodelica::List<i32> = ebltEqsLst;
    let mut tempSetS: metamodelica::List<i32> = tempSetS;
    let mut complexEquationList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut swappedEquationList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqIndex: i32;
    let mut matchedEqsLst: metamodelica::List<i32>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut swapEq: metamodelica::Ref<BackendDAE::Equation>;
    complexEquationList = metamodelica::nil();
    swappedEquationList = metamodelica::nil();
    for mut item in &**mappedEbltSetS {
        (eqIndex, matchedEqsLst) = item.clone();
        eq = BackendEquation::get(
            currentSystem.orderedEqs.clone(),
            (*metamodelica::index_checked(&__ab_mapIncRowEqn, eqIndex)?).clone(),
        )?;
        if BackendEquation::isComplexEquation(&eq) {
            complexEquationList = metamodelica::cons(eq, complexEquationList);
            for mut index in &*matchedEqsLst {
                swapEq = BackendEquation::get(
                    currentSystem.orderedEqs.clone(),
                    (*metamodelica::index_checked(&__ab_mapIncRowEqn, index.clone())?).clone(),
                )?;
                if !(BackendEquation::isComplexEquation(&swapEq)) {
                    ebltEqsLst = List::removeOnTrue(eqIndex, &fnptr!(intEq, i32, i32), ebltEqsLst)?;
                    tempSetS = List::removeOnTrue(index.clone(), &fnptr!(intEq, i32, i32), tempSetS)?;
                    tempSetS = metamodelica::cons(eqIndex, tempSetS);
                    ebltEqsLst = metamodelica::cons(index.clone(), ebltEqsLst);
                    swappedEquationList = metamodelica::cons(swapEq, swappedEquationList);
                    break;
                }
            }
        }
    }
    if !((complexEquationList).is_empty()) {
        BackendDump::dumpEquationArray(
            BackendEquation::listEquation(&(complexEquationList.clone().reverse()))?,
            &(literal!("Warning complex equation detected in Set-C")),
        )?;
        BackendDump::dumpEquationArray(
            BackendEquation::listEquation(&(swappedEquationList.clone().reverse()))?,
            &(literal!("Swapping Equations from Set-S")),
        )?;
    }
    Ok((ebltEqsLst, tempSetS, complexEquationList, swappedEquationList))
}

fn traverseEBLTAndExtractSetCAndSetS(
    mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ebltEquations: &metamodelica::List<i32>,
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut knownVars: &metamodelica::List<i32>,
    mut boundaryConditionVars: metamodelica::List<i32>,
    mut orderedVars: &BackendDAE::Variables,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut solvedEqsAndVarsInfo: &metamodelica::List<(i32, i32)>,
    mut debug: bool,
    mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut bindingEquations: metamodelica::List<i32>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::List<i32>,
    metamodelica::List<(i32, metamodelica::List<i32>)>,
    bool,
    metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<i32>)>,
)> {
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem> = currentSystem;
    let mut finalSetS: metamodelica::List<i32>;
    let mut mappedEbltSetS: metamodelica::List<(i32, metamodelica::List<i32>)>;
    let mut outStatus: bool = false;
    let mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )> = setBFailedBoundaryConditionEquations;
    let mut intermediateVars: metamodelica::List<i32>;
    let mut minimalSetS: metamodelica::List<i32>;
    let mut visitedVars: metamodelica::List<i32>;
    let mut eqlistToRemove: metamodelica::List<i32>;
    let mut intermediateVarsInBoundaryConditionEquation: metamodelica::List<i32>;
    let mut status: bool;
    let mut setB: metamodelica::List<(i32, i32)>;
    let mut varnumber: i32;
    let mut eqnumber: i32;
    let mut boundaryConditionVarIndex: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut failedboundaryConditionEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut newEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nExtracting SET-C and SET-S from E-BLT\nProcedure is applied on each equation in the E-BLT\n"
        ));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        ArcStr::from(__mm_s)
    });
    setB = metamodelica::nil();
    eqlistToRemove = metamodelica::nil();
    finalSetS = metamodelica::nil();
    mappedEbltSetS = metamodelica::nil();
    for mut eq in &**ebltEquations {
        intermediateVars = getVariablesAfterExtraction(list![eq.clone()], metamodelica::nil(), sBltAdjacencyMatrix);
        intermediateVars = List::setDifferenceOnTrue(intermediateVars, knownVars, &fnptr!(intEq, i32, i32))?.reverse();
        dumpSetSTargetEquations(
            eq.clone(),
            solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            orderedEqs.clone(),
            orderedVars,
            &(literal!(">>>")),
        )?;
        minimalSetS = metamodelica::nil();
        visitedVars = metamodelica::nil();
        status = true;
        (_, minimalSetS, visitedVars, status, boundaryConditionVarIndex) = extractNewMinimalSetS(
            intermediateVars,
            sBltAdjacencyMatrix,
            knownVars,
            boundaryConditionVars.clone(),
            orderedVars,
            orderedEqs.clone(),
            mapIncRowEqn.clone(),
            minimalSetS,
            visitedVars,
            solvedEqsAndVarsInfo,
            status,
            bindingEquations.clone(),
            true,
            debug,
        )?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nProcedure "));
            __mm_s.push_str(&*boolSuccessOrFailed(status));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        mappedEbltSetS = metamodelica::cons((eq.clone(), minimalSetS.clone().reverse()), mappedEbltSetS);
        for mut index in &*minimalSetS {
            if !(listMember(index.clone(), finalSetS.clone())) {
                finalSetS = metamodelica::cons(index.clone(), finalSetS);
            }
        }
        if !(status) {
            varnumber = getSolvedVariableNumber(eq.clone(), solvedEqsAndVarsInfo)?;
            if (minimalSetS).is_empty() {
                minimalSetS = list![eq.clone()];
            }
            if !(listMember((minimalSetS).head().cloned()?, eqlistToRemove.clone())) {
                eqlistToRemove = metamodelica::cons((minimalSetS).head().cloned()?, eqlistToRemove);
                setB = metamodelica::cons((varnumber, (minimalSetS).head().cloned()?), setB);
                if !(boundaryConditionVarExist(&setBFailedBoundaryConditionEquations, boundaryConditionVarIndex)) {
                    intermediateVarsInBoundaryConditionEquation = getVariablesAfterExtraction(
                        list![(minimalSetS).head().cloned()?],
                        metamodelica::nil(),
                        sBltAdjacencyMatrix,
                    );
                    intermediateVarsInBoundaryConditionEquation = List::setDifferenceOnTrue(
                        intermediateVarsInBoundaryConditionEquation,
                        knownVars,
                        &fnptr!(intEq, i32, i32),
                    )?
                    .reverse();
                    failedboundaryConditionEquation = BackendEquation::get(
                        orderedEqs.clone(),
                        ({
                            let __elt =
                                (*metamodelica::index_checked(&mapIncRowEqn.borrow(), (minimalSetS).head().cloned()?)?)
                                    .clone();
                            __elt
                        }),
                    )?;
                    setBFailedBoundaryConditionEquations = metamodelica::cons(
                        (
                            boundaryConditionVarIndex,
                            failedboundaryConditionEquation,
                            intermediateVarsInBoundaryConditionEquation,
                        ),
                        setBFailedBoundaryConditionEquations,
                    );
                }
            }
        }
    }
    if !((setB).is_empty()) {
        newEqnLst = metamodelica::nil();
        for mut item in &*setB.reverse() {
            (varnumber, eqnumber) = item.clone();
            var = BackendVariable::getVarAt(orderedVars, varnumber)?;
            lhs = BackendVariable::varExp(&var)?;
            rhs = metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat((0) as f64),
            });
            eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: lhs,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
            });
            newEqnLst = metamodelica::cons(eqn, newEqnLst);
        }
        if debug {
            metamodelica::print(literal!(
                "\nGenerate Modified Model, For each failed procedure, the equation involving the boundary condition that failed the procedure is replaced by x = 0 where x is the variable of interest of the procedure.\n"
            ));
            dumpSetSVarsSolvedInfo(
                &eqlistToRemove,
                solvedEqsAndVarsInfo,
                mapIncRowEqn.clone(),
                currentSystem.orderedEqs.clone(),
                &(currentSystem.orderedVars.clone()),
                &(literal!("Equations to remove")),
            )?;
            BackendDump::dumpEquationList(&newEqnLst, &(literal!("Equations to add")))?;
        }
        eqlistToRemove = List::unique(
            &(List::map1r(
                eqlistToRemove,
                &listGet,
                mapIncRowEqn
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
            )?),
        );
        currentSystem = deleteEquationsFromEqSyst(currentSystem, &eqlistToRemove)?;
        assign_field!(
            currentSystem.orderedEqs = BackendEquation::merge(
                currentSystem.orderedEqs.clone(),
                BackendEquation::listEquation(&(newEqnLst.reverse()))?
            )?
        );
    } else {
        outStatus = true;
        finalSetS = finalSetS.reverse();
        mappedEbltSetS = mappedEbltSetS.reverse();
    }
    Ok((
        currentSystem,
        finalSetS,
        mappedEbltSetS,
        outStatus,
        setBFailedBoundaryConditionEquations,
    ))
}

fn ExtractSetSPrime(
    mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut setBFailedBoundaryConditionEquations: metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut knownVars: &metamodelica::List<i32>,
    mut boundaryConditionVars: metamodelica::List<i32>,
    mut orderedVars: &BackendDAE::Variables,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut solvedEqsAndVarsInfo: &metamodelica::List<(i32, i32)>,
    mut bindingEquations: metamodelica::List<i32>,
    mut debug: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    bool,
)> {
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem> = currentSystem;
    let mut finalSetS: metamodelica::List<i32>;
    let mut failedboundaryConditionEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut failedboundaryConditionVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outStatus: bool = false;
    let mut intermediateVars: metamodelica::List<i32>;
    let mut minimalSetS: metamodelica::List<i32>;
    let mut visitedVars: metamodelica::List<i32>;
    let mut status: bool;
    let mut boundaryConditionVarIndex: i32;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nExtract set-S' to compute the boundary conditions\nProcedure is applied on each equation in the failed boundary conditions\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        ArcStr::from(__mm_s)
    });
    finalSetS = metamodelica::nil();
    failedboundaryConditionEquations = metamodelica::nil();
    failedboundaryConditionVars = metamodelica::nil();
    for mut items in &*setBFailedBoundaryConditionEquations.reverse() {
        (boundaryConditionVarIndex, eq, intermediateVars) = items.clone();
        failedboundaryConditionEquations = metamodelica::cons(eq.clone(), failedboundaryConditionEquations);
        failedboundaryConditionVars = metamodelica::cons(
            BackendVariable::getVarAt(orderedVars, boundaryConditionVarIndex)?,
            failedboundaryConditionVars,
        );
        intermediateVars = List::setDifferenceOnTrue(intermediateVars, knownVars, &fnptr!(intEq, i32, i32))?.reverse();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!(">>>"));
            __mm_s.push_str(&*BackendDump::equationString(&eq)?);
            ArcStr::from(__mm_s)
        });
        minimalSetS = metamodelica::nil();
        visitedVars = metamodelica::nil();
        status = true;
        (_, minimalSetS, visitedVars, status, _) = extractNewMinimalSetS(
            intermediateVars,
            sBltAdjacencyMatrix,
            knownVars,
            boundaryConditionVars.clone(),
            orderedVars,
            orderedEqs.clone(),
            mapIncRowEqn.clone(),
            minimalSetS,
            visitedVars,
            solvedEqsAndVarsInfo,
            status,
            bindingEquations.clone(),
            false,
            debug,
        )?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nProcedure "));
            __mm_s.push_str(&*boolSuccessOrFailed(status));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        for mut index in &*minimalSetS {
            if !(listMember(index.clone(), finalSetS.clone())) {
                finalSetS = metamodelica::cons(index.clone(), finalSetS);
            }
        }
    }
    failedboundaryConditionEquations = failedboundaryConditionEquations.reverse();
    failedboundaryConditionVars = failedboundaryConditionVars.reverse();
    Ok((
        currentSystem,
        finalSetS,
        failedboundaryConditionEquations,
        failedboundaryConditionVars,
        outStatus,
    ))
}

fn boundaryConditionVarExist(
    mut setBFailedBoundaryConditionEquations: &metamodelica::List<(
        i32,
        metamodelica::Ref<BackendDAE::Equation>,
        metamodelica::List<i32>,
    )>,
    mut boundaryConditionVarIndex: i32,
) -> bool {
    let mut status: bool = false;
    let mut varIndex: i32;
    for mut item in &**setBFailedBoundaryConditionEquations {
        (varIndex, _, _) = item.clone();
        if intEq(varIndex, boundaryConditionVarIndex) {
            status = true;
            break;
        }
    }
    status
}

fn boolSuccessOrFailed(mut status: bool) -> ArcStr {
    let mut outString: ArcStr;
    outString = if (status) {
        literal!("success")
    } else {
        literal!("failed")
    };
    outString
}

fn extractNewMinimalSetS(
    mut unknownsInSetC: metamodelica::List<i32>,
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut knownVars: &metamodelica::List<i32>,
    mut boundaryConditionVars: metamodelica::List<i32>,
    mut orderedVars: &BackendDAE::Variables,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut minimalSetS: metamodelica::List<i32>,
    mut visitedVars: metamodelica::List<i32>,
    mut solvedEqsAndVarsInfo: &metamodelica::List<(i32, i32)>,
    mut status: bool,
    mut bindingEquations: metamodelica::List<i32>,
    mut extractSetCAndSetS: bool,
    mut debug: bool,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    bool,
    i32,
)> {
    let mut unknownsInSetC: metamodelica::List<i32> = unknownsInSetC;
    let mut minimalSetS: metamodelica::List<i32> = minimalSetS;
    let mut visitedVars: metamodelica::List<i32> = visitedVars;
    let mut status: bool = status;
    let mut boundaryConditionVarIndex: i32 = -1;
    let mut mappedEq: i32;
    let mut varIndex: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut rest: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut intermediateVars: metamodelica::List<i32>;
    let mut intermediateVarsInMatchedEquation: metamodelica::List<i32>;
    while !((unknownsInSetC).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unknownsInSetC.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        varIndex = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        visitedVars = metamodelica::cons(varIndex, visitedVars);
        var = BackendVariable::getVarAt(orderedVars, varIndex)?;
        if listMember(varIndex, boundaryConditionVars.clone()) && extractSetCAndSetS {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
                __mm_s.push_str(&*literal!(" is a boundary condition ---> exit procedure"));
                ArcStr::from(__mm_s)
            });
            status = false;
            boundaryConditionVarIndex = varIndex;
            break;
        }
        mappedEq = getSolvedEquationNumber(varIndex, solvedEqsAndVarsInfo)?;
        if !(listMember(mappedEq, bindingEquations.clone())) {
            minimalSetS = metamodelica::cons(mappedEq, minimalSetS);
            dumpSetSTargetEquations(
                mappedEq,
                solvedEqsAndVarsInfo,
                mapIncRowEqn.clone(),
                orderedEqs.clone(),
                orderedVars,
                &(literal!("")),
            )?;
        }
        vars = getVariablesAfterExtraction(list![mappedEq], metamodelica::nil(), sBltAdjacencyMatrix);
        intermediateVarsInMatchedEquation = List::setDifferenceOnTrue(vars, knownVars, &fnptr!(intEq, i32, i32))?;
        intermediateVars = List::setDifferenceOnTrue(
            intermediateVarsInMatchedEquation.clone(),
            &(list![varIndex]),
            &fnptr!(intEq, i32, i32),
        )?;
        intermediateVars = List::setDifferenceOnTrue(intermediateVars, &visitedVars, &fnptr!(intEq, i32, i32))?;
        rest = List::setDifferenceOnTrue(rest, &visitedVars, &fnptr!(intEq, i32, i32))?;
        unknownsInSetC = List::unique(&(listAppend(intermediateVars, rest.clone())));
        if debug {
            dumpMininimalExtraction(
                varIndex,
                &var,
                mappedEq,
                mapIncRowEqn.clone(),
                orderedEqs.clone(),
                minimalSetS.clone(),
                intermediateVarsInMatchedEquation,
                rest,
                unknownsInSetC.clone(),
                false,
                visitedVars.clone(),
            )?;
        }
    }
    Ok((
        unknownsInSetC,
        minimalSetS,
        visitedVars,
        status,
        boundaryConditionVarIndex,
    ))
}

pub(crate) fn extractionAlgorithm(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outResidualEqns: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut newEqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut setC_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut setS_Eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut residualEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut match1: metamodelica::Array<i32>;
    let mut match2: metamodelica::Array<i32>;
    let mut solvedEqsAndVarsInfo: metamodelica::List<(i32, i32)>;
    let mut varCount: i32;
    let mut eqCount: i32;
    let mut matchedEqsLst: metamodelica::List<i32>;
    let mut unMatchedEqsLst: metamodelica::List<i32>;
    let mut unMatchedEqsLstCorrectIndex: metamodelica::List<i32>;
    let mut approximatedEquations: metamodelica::List<i32>;
    let mut tempSetC: metamodelica::List<i32>;
    let mut setC: metamodelica::List<i32>;
    let mut tempSetS: metamodelica::List<i32>;
    let mut setS: metamodelica::List<i32>;
    let mut boundaryConditionEquations: metamodelica::List<i32>;
    let mut s_BLTBlocks: metamodelica::List<metamodelica::List<i32>>;
    let mut e_BLTBlocks: metamodelica::List<metamodelica::List<i32>>;
    let mut allBlocks: metamodelica::List<metamodelica::List<i32>>;
    let mut allBlocksStatusVarInfo: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut e_BLT_EquationsWithIndex: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>)>;
    let mut eBltAdjacencyMatrix: ExtAdjacencyMatrix;
    let mut sBltAdjacencyMatrix: ExtAdjacencyMatrix;
    let mut setS_BLTAdjacencyMatrix: ExtAdjacencyMatrix;
    let mut e_BLTSolvedEqsAndVars: metamodelica::List<(i32, i32)>;
    let mut e_BLTBlockRanks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut s_BLTBlockRanks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut s_BLTBlockTargetInfo: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>;
    let mut predecessorBlockTargetInfo: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>;
    let mut paramVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut residualVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut simCodeJacobian: metamodelica::Ref<BackendDAE::Jacobian>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut r#str: ArcStr;
    let mut modelicaOutput: ArcStr;
    let mut modelicaFileName: ArcStr;
    let mut modelName: ArcStr;
    let mut auxillaryConditionsFilename: ArcStr;
    let mut auxillaryEquations: ArcStr;
    let mut intermediateEquationsFilename: ArcStr;
    let mut intermediateEquations: ArcStr;
    let mut allVarsList: metamodelica::List<i32>;
    let mut knowns: metamodelica::List<i32>;
    let mut boundaryConditionVars: metamodelica::List<i32>;
    let mut exactEquationVars: metamodelica::List<i32>;
    let mut extractedVarsfromSetS: metamodelica::List<i32>;
    let mut knownVariablesWithEquationBinding: metamodelica::List<i32>;
    let mut boundaryConditionTaggedEquationSolvedVars: metamodelica::List<i32>;
    let mut unknownVarsInSetC: metamodelica::List<i32>;
    let mut inputVars: BackendDAE::Variables;
    let mut outDiffVars: BackendDAE::Variables;
    let mut outOtherVars: BackendDAE::Variables;
    let mut outResidualVars: BackendDAE::Variables;
    let mut debug: bool = false;
    if Flags::isSet(Flags::DUMP_DATARECONCILIATION.clone())? {
        debug = true;
    }
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    currentSystem = metamodelica::Own::own(__pa0);
    shared = inDAE.shared.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nModelInfo: "));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("OrderedVariables")))?;
    BackendDump::dumpEquationArray(currentSystem.orderedEqs.clone(), &(literal!("OrderedEquation")))?;
    (currentSystem, shared) = setBoundaryConditionEquationsAndVars(currentSystem, inDAE.shared.clone(), debug)?;
    if debug {
        BackendDump::dumpVariables(
            &currentSystem.orderedVars,
            &(literal!("Updated-OrderedVariables-withBoundaryConditionVars")),
        )?;
        BackendDump::dumpEquationArray(
            currentSystem.orderedEqs.clone(),
            &(literal!("Updated-OrderedVariables-withBoundaryConditionEqs")),
        )?;
        BackendDump::dumpVariables(
            &shared.globalKnownVars,
            &(literal!("Updated-GlobalKnownVars-withBoundaryConditionVarsRemoved")),
        )?;
    }
    allVarsList = List::intRange(BackendVariable::varsSize(&currentSystem.orderedVars));
    (adjacencyMatrix, _, _, _) = BackendDAEUtil::adjacencyMatrixScalar(
        &currentSystem,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    (knowns, boundaryConditionVars, exactEquationVars, _) =
        getVariablesBlockCategories(&currentSystem.orderedVars, &allVarsList)?;
    if debug {
        metamodelica::print(literal!("\nVariablesCategories\n============================="));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nknownVars                    :"));
            __mm_s.push_str(&*dumplistInteger(knowns.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nboundaryConditionVars        :"));
            __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nexactEquationVars            :"));
            __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nadjacencyMatrix              :"));
            __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    allVarsList = List::intRange(BackendVariable::varsSize(&currentSystem.orderedVars));
    knownVariablesWithEquationBinding = getUncertainRefineVariablesBindedEquations(adjacencyMatrix.clone(), &knowns);
    if debug {
        metamodelica::print(literal!(
            "\nEquations with KnownBindings:\n==================================="
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nAdjacency Matrix                     :"));
            __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nLength of Adjacency Matrix           :"));
            __mm_s.push_str(&*intString(metamodelica::arrayLength(adjacencyMatrix.clone())));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nList of known equation with bindings :"));
            __mm_s.push_str(&*anyString(knownVariablesWithEquationBinding.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    newEqnsLst = inverseModelicaModel(&currentSystem.orderedVars, &knownVariablesWithEquationBinding)?;
    assign_field!(
        currentSystem.orderedEqs = BackendEquation::merge(
            currentSystem.orderedEqs.clone(),
            BackendEquation::listEquation(&newEqnsLst)?
        )?
    );
    BackendDump::dumpEquationArray(
        currentSystem.orderedEqs.clone(),
        &(literal!("OverDetermined-System-Equations")),
    )?;
    (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
        &currentSystem,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    varCount = currentSystem.orderedVars.numberOfVars.clone();
    eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
    if debug {
        metamodelica::print(literal!(
            "\nOverDetermined-Systems-Information :\n====================================\n"
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nAdjacency Matrix     :"));
            __mm_s.push_str(&*anyString(adjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nNumber of Vars       :"));
            __mm_s.push_str(&*intString(varCount));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nNumber of Equations  :"));
            __mm_s.push_str(&*intString(eqCount));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n\n"));
    }
    (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount, eqCount)?;
    BackendDump::dumpMatching(match1.clone())?;
    (solvedEqsAndVarsInfo, matchedEqsLst) = getSolvedEquationAndVarsInfo(match1.clone());
    unMatchedEqsLst = List::setDifference(List::intRange(eqCount), &matchedEqsLst)?;
    unMatchedEqsLstCorrectIndex = List::unique(
        &(List::map1r(
            unMatchedEqsLst.clone(),
            &listGet,
            mapIncRowEqn
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?),
    );
    if debug {
        metamodelica::print(literal!(
            "\nFinding unmatched subset of equations :\n=========================================\n"
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSolvedEqsAndVarsInfo                   :"));
            __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nList of Equations                      :"));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(
                currentSystem.orderedEqs.clone(),
            )));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nMatchedEquationsLst                    :"));
            __mm_s.push_str(&*anyString(List::sort(
                matchedEqsLst.clone(),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSizeofMatchedEquationLST               :"));
            __mm_s.push_str(&*intString(((matchedEqsLst).len() as i32)));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnMatchedSubSetOfEquations             :"));
            __mm_s.push_str(&*anyString(unMatchedEqsLst.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnMatchedSubSetOfEquationsMappedIndex  :"));
            __mm_s.push_str(&*anyString(unMatchedEqsLstCorrectIndex.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    (
        e_BLT_EquationsWithIndex,
        eBltAdjacencyMatrix,
        e_BLTSolvedEqsAndVars,
        e_BLTBlocks,
        e_BLTBlockRanks,
    ) = setEBLTEquationsWithIndexAndRank(
        &unMatchedEqsLst,
        &unMatchedEqsLstCorrectIndex,
        currentSystem.orderedEqs.clone(),
        adjacencyMatrix.clone(),
    )?;
    BackendDump::dumpEquationList(
        &(List::map1r(
            unMatchedEqsLstCorrectIndex.clone(),
            &BackendEquation::get,
            currentSystem.orderedEqs.clone(),
        )?),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("E-BLT-Equations "));
            __mm_s.push_str(&*dumplistInteger(unMatchedEqsLst)?);
            ArcStr::from(__mm_s)
        }),
    )?;
    if debug {
        metamodelica::print(literal!("\nE-BLT Information\n================"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nE-BLT-Blocks   :"));
            __mm_s.push_str(&*anyString(e_BLTBlocks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nE-BLT-Blocks-with ranks   :"));
            __mm_s.push_str(&*anyString(e_BLTBlockRanks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nE-BLT-Adjacency-Matrix    :"));
            __mm_s.push_str(&*anyString(eBltAdjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nE_BLTSolvedEqsAndVars     :"));
            __mm_s.push_str(&*anyString(e_BLTSolvedEqsAndVars.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    currentSystem = deleteEquationsFromEqSyst(currentSystem, &unMatchedEqsLstCorrectIndex)?;
    varCount = currentSystem.orderedVars.numberOfVars.clone();
    eqCount = BackendEquation::equationArraySize(currentSystem.orderedEqs.clone())?;
    BackendDump::dumpEquationArray(
        currentSystem.orderedEqs.clone(),
        &(literal!("reOrdered-Equations-after-removal")),
    )?;
    BackendDump::dumpVariables(&currentSystem.orderedVars, &(literal!("reOrderedVariables")))?;
    (adjacencyMatrix, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
        &currentSystem,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    (match1, match2, _, _, _) = Matching::RegularMatching(adjacencyMatrix.clone(), varCount, eqCount)?;
    BackendDump::dumpMatching(match1.clone())?;
    s_BLTBlocks = Sorting::Tarjan(
        adjacencyMatrix.clone(),
        match1.clone(),
        metamodelica::arrayLength(match1.clone()),
    )?;
    sBltAdjacencyMatrix = getSBLTAdjacencyMatrix(adjacencyMatrix.clone());
    (solvedEqsAndVarsInfo, _) = getSolvedEquationAndVarsInfo(match1.clone());
    s_BLTBlockRanks = List::toListWithPositions(&s_BLTBlocks);
    if debug {
        metamodelica::print(literal!("\nS-BLT-Information\n================"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS-BLT Number of Vars       :"));
            __mm_s.push_str(&*intString(varCount));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS-BLT Number of Equations  :"));
            __mm_s.push_str(&*intString(eqCount));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS-BLT-Blocks               :"));
            __mm_s.push_str(&*anyString(s_BLTBlocks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS-BLT-Blocks-with ranks    :"));
            __mm_s.push_str(&*anyString(s_BLTBlockRanks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS-BLT Adjacency Matrix     :"));
            __mm_s.push_str(&*anyString(sBltAdjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS_BLTSolvedEqsAndVars      :"));
            __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n\n"));
    }
    s_BLTBlocks = listAppend(s_BLTBlocks, e_BLTBlocks);
    s_BLTBlockRanks = listAppend(s_BLTBlockRanks, e_BLTBlockRanks.clone());
    sBltAdjacencyMatrix = listAppend(sBltAdjacencyMatrix, eBltAdjacencyMatrix);
    solvedEqsAndVarsInfo = listAppend(solvedEqsAndVarsInfo, e_BLTSolvedEqsAndVars);
    if debug {
        metamodelica::print(literal!(
            "\nCombined S-BLT and E-BLT Information\n================================"
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nCombined S-BLT-Blocks and E-BLT-Blocks                :"));
            __mm_s.push_str(&*anyString(s_BLTBlocks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nCombined S-BLT-Blocks and E-BLT-Blocks with Ranks     :"));
            __mm_s.push_str(&*anyString(s_BLTBlockRanks.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nCombined Adjacency Matrix with S-BLT and E-BLT        :"));
            __mm_s.push_str(&*anyString(sBltAdjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nCombined SolvedEquationsVarsInfo with S-BLT and E-BLT :"));
            __mm_s.push_str(&*anyString(solvedEqsAndVarsInfo.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    dumpListList(s_BLTBlocks.clone(), &(literal!("BLT_BLOCKS")))?;
    (approximatedEquations, boundaryConditionEquations) = getEquationsTaggedApproximatedOrBoundaryCondition(
        &(BackendEquation::equationList(currentSystem.orderedEqs.clone())?),
        1,
    );
    if debug {
        BackendDump::dumpEquationList(
            &(List::map1r(
                approximatedEquations.clone(),
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("ApproximatedEquations")),
        )?;
        BackendDump::dumpEquationList(
            &(List::map1r(
                boundaryConditionEquations.clone(),
                &BackendEquation::get,
                currentSystem.orderedEqs.clone(),
            )?),
            &(literal!("boundaryConditionEquations")),
        )?;
    }
    approximatedEquations = List::flatten(List::map1r(
        approximatedEquations,
        &listGet,
        mapEqnIncRow
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?)?;
    boundaryConditionEquations = List::flatten(List::map1r(
        boundaryConditionEquations,
        &listGet,
        mapEqnIncRow
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?)?;
    if debug {
        metamodelica::print(literal!(
            "\nApproximated and BoundaryCondition Equation Indexes :\n==========================================="
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nApproximatedEquationIndexes      :"));
            __mm_s.push_str(&*dumplistInteger(approximatedEquations.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBoundayConditionEquationIndexes  :"));
            __mm_s.push_str(&*dumplistInteger(boundaryConditionEquations.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    boundaryConditionTaggedEquationSolvedVars =
        getBoundaryConditionVariables(&boundaryConditionEquations, &solvedEqsAndVarsInfo)?;
    if debug {
        BackendDump::dumpVarList(
            &(List::map1r(
                boundaryConditionTaggedEquationSolvedVars.clone().reverse(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                currentSystem.orderedVars.clone(),
            )?),
            &(literal!("boundaryConditionTaggedEquationSolvedVars")),
        )?;
    }
    exactEquationVars = List::setDifferenceOnTrue(
        exactEquationVars,
        &boundaryConditionTaggedEquationSolvedVars,
        &fnptr!(intEq, i32, i32),
    )?;
    boundaryConditionVars = listAppend(boundaryConditionVars, boundaryConditionTaggedEquationSolvedVars);
    if debug {
        metamodelica::print(literal!("\nUpdatedVariablesCategories\n============================="));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nknownVars                    :"));
            __mm_s.push_str(&*dumplistInteger(knowns.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nboundaryConditionVars        :"));
            __mm_s.push_str(&*dumplistInteger(boundaryConditionVars.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nexactEquationVars            :"));
            __mm_s.push_str(&*dumplistInteger(exactEquationVars.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    (allBlocks, allBlocksStatusVarInfo) = traverseBLTAndUpdateBlockStatus(
        &s_BLTBlocks,
        knowns.clone(),
        &boundaryConditionVars,
        exactEquationVars.clone(),
        &solvedEqsAndVarsInfo,
    )?;
    if debug {
        dumpBlockStatus(&allBlocks, &allBlocksStatusVarInfo)?;
    }
    s_BLTBlockTargetInfo = findBlockTargets(
        allBlocks,
        &allBlocksStatusVarInfo,
        solvedEqsAndVarsInfo.clone(),
        sBltAdjacencyMatrix.clone(),
        &s_BLTBlockRanks,
        debug,
    )?;
    if debug {
        dumpBlockTargets(&s_BLTBlockTargetInfo)?;
    }
    predecessorBlockTargetInfo = findPredecessorBlocks(&s_BLTBlockTargetInfo)?;
    dumpPredecessorBlocks(&predecessorBlockTargetInfo)?;
    (tempSetC, tempSetS) = ExtractEquationsUsingSetOperations(
        &predecessorBlockTargetInfo,
        &e_BLTBlockRanks,
        &approximatedEquations,
        debug,
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nFINAL SET OF EQUATIONS After Reconciliation\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_C: "));
        __mm_s.push_str(&*dumplistInteger(tempSetC.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_S: "));
        __mm_s.push_str(&*dumplistInteger(tempSetS.clone())?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    if debug {
        dumpSetSVarsSolvedInfo(
            &tempSetS,
            &solvedEqsAndVarsInfo,
            mapIncRowEqn.clone(),
            currentSystem.orderedEqs.clone(),
            &(currentSystem.orderedVars.clone()),
            &(literal!("Set-S Solved-Variables Information")),
        )?;
    }
    setC = List::unique(&(getAbsoluteIndexHelper(&tempSetC, mapIncRowEqn.clone())?));
    setS = List::unique(&(getAbsoluteIndexHelper(&tempSetS, mapIncRowEqn.clone())?));
    setC_Eq = getEquationsFromSBLTAndEBLT(&setC, currentSystem.orderedEqs.clone(), &e_BLT_EquationsWithIndex)?;
    setS_Eq = getEquationsFromSBLTAndEBLT(&setS, currentSystem.orderedEqs.clone(), &e_BLT_EquationsWithIndex)?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setC_Eq)?, &(literal!("SET_C")))?;
    BackendDump::dumpEquationArray(BackendEquation::listEquation(&setS_Eq)?, &(literal!("SET_S")))?;
    unknownVarsInSetC = getVariablesAfterExtraction(tempSetC.clone(), metamodelica::nil(), &sBltAdjacencyMatrix);
    unknownVarsInSetC = List::setDifferenceOnTrue(unknownVarsInSetC, &knowns, &fnptr!(intEq, i32, i32))?.reverse();
    setS_BLTAdjacencyMatrix = getSetSAdjacencyMatrix(&sBltAdjacencyMatrix, tempSetS);
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nStart of Extract Minimal Set-S Algorithm\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSet-S Adjacency MAtrix : "));
            __mm_s.push_str(&*intString(((setS_BLTAdjacencyMatrix).len() as i32)));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*anyString(setS_BLTAdjacencyMatrix.clone()));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\nS'        : {}"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nV_C       :"));
            __mm_s.push_str(&*dumplistInteger(unknownVarsInSetC.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (_, tempSetS) = extractMinimalSetS(
        unknownVarsInSetC,
        &setS_BLTAdjacencyMatrix,
        &knowns,
        &(currentSystem.orderedVars.clone()),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        metamodelica::nil(),
        debug,
    )?;
    if debug {
        metamodelica::print(literal!("\n****End of Minimal extraction Algorithm****\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSet-S after running minimal extraction algorithm\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("SET_S: "));
            __mm_s.push_str(&*dumplistInteger(tempSetS.clone())?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    extractedVarsfromSetS = getVariablesAfterExtraction(metamodelica::nil(), tempSetS.clone(), &sBltAdjacencyMatrix);
    extractedVarsfromSetS = List::setDifferenceOnTrue(extractedVarsfromSetS, &knowns, &fnptr!(intEq, i32, i32))?;
    setC = List::unique(&(getAbsoluteIndexHelper(&tempSetC, mapIncRowEqn.clone())?));
    setS = List::unique(&(getAbsoluteIndexHelper(&tempSetS, mapIncRowEqn.clone())?));
    setC_Eq = getEquationsFromSBLTAndEBLT(&setC, currentSystem.orderedEqs.clone(), &e_BLT_EquationsWithIndex)?;
    setS_Eq = getEquationsFromSBLTAndEBLT(&setS, currentSystem.orderedEqs.clone(), &e_BLT_EquationsWithIndex)?;
    if !((tempSetS).is_empty()) {
        BackendDump::dumpEquationArray(
            BackendEquation::listEquation(&setS_Eq)?,
            &(literal!("SET_S_After_Minimal_Extraction")),
        )?;
    } else {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSET_S_After_Minimal_Extraction (0, 0)\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    outDiffVars = BackendVariable::listVar(List::map1r(
        knowns.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    outDiffVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarUnreplaceable,
            metamodelica::Ref<BackendDAE::Var>,
            bool
        ),
        true,
    )?)?;
    (_, residualEquations) = BackendEquation::traverseEquationArray(
        BackendEquation::listEquation(&setC_Eq)?,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (
            metamodelica::Ref<AvlTreePathFunction::Tree>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1),
        (shared.functionTree.clone(), metamodelica::nil()),
    )?;
    (residualEquations, residualVars, _) = BackendEquation::convertResidualsIntoSolvedEquations(
        &(residualEquations.reverse()),
        &(literal!("$res_F_")),
        1,
        false,
    )?;
    outResidualVars = BackendVariable::listVar(residualVars.reverse())?;
    outResidualEqns = BackendEquation::listEquation(&residualEquations)?;
    outOtherEqns = BackendEquation::listEquation(&setS_Eq)?;
    paramVars = BackendEquation::equationsVars(outOtherEqns.clone(), shared.globalKnownVars.clone())?;
    outOtherVars = BackendVariable::listVar(List::map1r(
        extractedVarsfromSetS,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        currentSystem.orderedVars.clone(),
    )?)?;
    dumpSetSVars(&outOtherVars, &(literal!("Unknown variables in SET_S ")))?;
    BackendDump::dumpVariables(
        &(BackendVariable::listVar(paramVars.clone())?),
        &(literal!("Parameters in SET_S")),
    )?;
    auxillaryConditionsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_AuxiliaryConditions.html"));
        ArcStr::from(__mm_s)
    };
    auxillaryEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setC_Eq)?,
        &(literal!("Auxiliary conditions")),
    )?;
    System::writeFile(auxillaryConditionsFilename, auxillaryEquations)?;
    intermediateEquationsFilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_IntermediateEquations.html"));
        ArcStr::from(__mm_s)
    };
    intermediateEquations = dumpExtractedEquationsToHTML(
        BackendEquation::listEquation(&setS_Eq)?,
        &(literal!("Intermediate equations")),
    )?;
    System::writeFile(intermediateEquationsFilename, intermediateEquations)?;
    VerifyDataReconciliation(
        &tempSetC,
        &tempSetS,
        knowns,
        &boundaryConditionVars,
        &sBltAdjacencyMatrix,
        &solvedEqsAndVarsInfo,
        &exactEquationVars,
        &approximatedEquations,
        currentSystem.orderedVars.clone(),
        currentSystem.orderedEqs.clone(),
        mapIncRowEqn.clone(),
        &outOtherVars,
        &setS_Eq,
        &shared,
        setC,
        setS,
        0,
    )?;
    if debug {
        BackendDump::dumpVariables(&outDiffVars, &(literal!("Jacobian_knownVariables")))?;
        BackendDump::dumpVariables(&outResidualVars, &(literal!("Jacobian_outResidualVars")))?;
        BackendDump::dumpVariables(&outOtherVars, &(literal!("Jacobian_outOtherVars")))?;
        BackendDump::dumpEquationArray(outResidualEqns.clone(), &(literal!("Jacobian_ResidualEquation")))?;
        BackendDump::dumpEquationArray(outOtherEqns.clone(), &(literal!("Jacobian_other_Equation")))?;
    }
    (simCodeJacobian, shared) = SymbolicJacobian::getSymbolicJacobian(
        &outDiffVars,
        outResidualEqns.clone(),
        outResidualVars.clone(),
        outOtherEqns.clone(),
        outOtherVars.clone(),
        shared,
        &(outOtherVars.clone()),
        literal!("F"),
        false,
    )?;
    assign_field!(
        shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData {
            symbolicJacobian: simCodeJacobian,
            setcVars: outResidualVars.clone(),
            datareconinputs: outDiffVars.clone(),
            setBVars: None,
            symbolicJacobianH: None,
            relatedBoundaryConditions: 0
        })
    );
    currentSystem = BackendDAEUtil::setEqSystVars(
        currentSystem,
        BackendVariable::mergeVariables(outResidualVars.clone(), outOtherVars.clone(), true)?,
    );
    currentSystem = BackendDAEUtil::setEqSystEqs(
        currentSystem,
        BackendEquation::merge(outResidualEqns.clone(), outOtherEqns.clone())?,
    );
    inputVars = BackendVariable::listVar(List::map1(
        BackendVariable::varList(&outDiffVars)?,
        &fnptr!(
            BackendVariable::setVarDirection,
            metamodelica::Ref<BackendDAE::Var>,
            DAE::VarDirection
        ),
        openmodelica_frontend_types::DAE::VarDirection::INPUT,
    )?)?;
    shared = BackendDAEUtil::setSharedGlobalKnownVars(
        shared.clone(),
        BackendVariable::mergeVariables(shared.globalKnownVars.clone(), inputVars, true)?,
    );
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inDAE.shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Inputs.csv"));
        ArcStr::from(__mm_s)
    })) {
        r#str = literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval\n");
        r#str = dumpToCsv(&r#str, &(BackendVariable::varList(&outDiffVars)?))?;
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_Inputs.csv"));
                ArcStr::from(__mm_s)
            },
            r#str,
        )?;
    }
    modelicaFileName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("_Reconciled_tmp"));
        ArcStr::from(__mm_s)
    };
    modelName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Reconciled_"));
        __mm_s.push_str(&*System::stringReplace(
            shared.info.fileNamePrefix.clone(),
            literal!("."),
            literal!("_"),
        )?);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = literal!(
        "/* This is a Reconciled Model which is generated by the Data Reconciliation extraction algorithm */\n"
    );
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("model "));
        __mm_s.push_str(&*modelName);
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outDiffVars)?),
        &(literal!("Variables of Interest")),
    )?;
    modelicaOutput = dumpExtractedVars(&modelicaOutput, &paramVars, &(literal!("parameters in SET-S")))?;
    modelicaOutput = dumpResidualVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outResidualVars)?),
        &(literal!("residualVars")),
    )?;
    modelicaOutput = dumpExtractedVars(
        &modelicaOutput,
        &(BackendVariable::varList(&outOtherVars)?),
        &(literal!("remaining variables in setS")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nequation"));
        ArcStr::from(__mm_s)
    };
    modelicaOutput = dumpExtractedEquations(&modelicaOutput, outResidualEqns, &(literal!("set-C Canonical form")))?;
    modelicaOutput = dumpExtractedEquations(
        &modelicaOutput,
        outOtherEqns,
        &(literal!("remaining equations in Set-S")),
    )?;
    modelicaOutput = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*modelicaOutput);
        __mm_s.push_str(&*literal!("\nend "));
        __mm_s.push_str(&*modelName);
        __mm_s.push_str(&*literal!(";"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*modelicaFileName);
            __mm_s.push_str(&*literal!(".mo"));
            ArcStr::from(__mm_s)
        },
        modelicaOutput,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![currentSystem],
        shared: shared,
    });
    Ok(outDAE)
}

fn getSetSAdjacencyMatrix(
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut setS: metamodelica::List<i32>,
) -> ExtAdjacencyMatrix {
    let mut setS_BltAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut eq: i32;
    for mut i in &**sBltAdjacencyMatrix {
        (eq, _) = i.clone();
        if listMember(eq, setS.clone()) {
            setS_BltAdjacencyMatrix = metamodelica::cons(i.clone(), setS_BltAdjacencyMatrix);
        }
    }
    setS_BltAdjacencyMatrix
}

fn extractMinimalSetS(
    mut unknownsInSetC: metamodelica::List<i32>,
    mut sBltAdjacencyMatrix: &ExtAdjacencyMatrix,
    mut knownVars: &metamodelica::List<i32>,
    mut orderedVars: &BackendDAE::Variables,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut minimalSetS: metamodelica::List<i32>,
    mut debug: bool,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut unknownsInSetC: metamodelica::List<i32> = unknownsInSetC;
    let mut minimalSetS: metamodelica::List<i32> = minimalSetS;
    let mut firstMatchedEquation: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut rest: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut intermediateVars: metamodelica::List<i32> = metamodelica::nil();
    let mut V_EQ: metamodelica::List<i32>;
    for mut varIndex in &*unknownsInSetC.clone() {
        if (unknownsInSetC).is_empty() {
            break;
        }
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nIntermediate varList : "));
                __mm_s.push_str(&*dumplistInteger(unknownsInSetC.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        let __pa0 = ::match_deref::match_deref! { match &(unknownsInSetC) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rest = metamodelica::Own::own(__pa0);
        (firstMatchedEquation, vars) =
            getVariableFirstOccurrenceInEquation(sBltAdjacencyMatrix, varIndex.clone(), minimalSetS.clone());
        var = BackendVariable::getVarAt(orderedVars, varIndex.clone())?;
        if !(intEq(firstMatchedEquation, 0)) {
            minimalSetS = metamodelica::cons(firstMatchedEquation, minimalSetS);
            minimalSetS = List::unique(&minimalSetS);
            intermediateVars = List::setDifferenceOnTrue(vars, knownVars, &fnptr!(intEq, i32, i32))?;
            V_EQ = List::unique(&(listAppend(intermediateVars.clone(), rest.clone())));
            if debug {
                dumpMininimalExtraction(
                    varIndex.clone(),
                    &var,
                    firstMatchedEquation,
                    mapIncRowEqn.clone(),
                    orderedEqs.clone(),
                    minimalSetS.clone(),
                    intermediateVars,
                    rest,
                    V_EQ.clone(),
                    false,
                    metamodelica::nil(),
                )?;
            }
            (unknownsInSetC, minimalSetS) = extractMinimalSetS(
                V_EQ,
                sBltAdjacencyMatrix,
                knownVars,
                orderedVars,
                orderedEqs.clone(),
                mapIncRowEqn.clone(),
                minimalSetS,
                debug,
            )?;
        } else {
            if debug {
                dumpMininimalExtraction(
                    varIndex.clone(),
                    &var,
                    0,
                    mapIncRowEqn.clone(),
                    orderedEqs.clone(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                    rest.clone(),
                    metamodelica::nil(),
                    true,
                    metamodelica::nil(),
                )?;
            }
            unknownsInSetC = rest;
        }
    }
    Ok((unknownsInSetC, minimalSetS))
}

fn dumpMininimalExtraction(
    mut varIndex: i32,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut firstMatchedEquation: i32,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut minimalSetS: metamodelica::List<i32>,
    mut intermediateVars: metamodelica::List<i32>,
    mut rest: metamodelica::List<i32>,
    mut V_EQ: metamodelica::List<i32>,
    mut falseBlock: bool,
    mut visitedVars: metamodelica::List<i32>,
) -> Result<()> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut mappedEq: i32;
    let mut tmpEq: metamodelica::Ref<BackendDAE::Equation>;
    if falseBlock {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nVarIndex           : "));
            __mm_s.push_str(&*intString(varIndex));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nVariable Name      : "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEquation Not Exist : "));
            __mm_s.push_str(&*literal!("NIL"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nRemainingVars      : "));
            __mm_s.push_str(&*dumplistInteger(rest)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        mappedEq = (*metamodelica::index_checked(&__ab_mapIncRowEqn, firstMatchedEquation)?).clone();
        tmpEq = BackendEquation::get(orderedEqs, mappedEq)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nVarIndex                     : "));
            __mm_s.push_str(&*intString(varIndex));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nVariable Name                : "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEquation Exist               : "));
            __mm_s.push_str(&*intString(firstMatchedEquation));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nmappedEquation               : "));
            __mm_s.push_str(&*intString(mappedEq));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nMatched Equation             : "));
            __mm_s.push_str(&*BackendDump::equationString(&tmpEq)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nS'                           : "));
            __mm_s.push_str(&*dumplistInteger(minimalSetS)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnknowns in matchedEquation  : "));
            __mm_s.push_str(&*dumplistInteger(intermediateVars)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nVisited vars                 : "));
            __mm_s.push_str(&*dumplistInteger(visitedVars)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nRemaining Vars               : "));
            __mm_s.push_str(&*dumplistInteger(rest)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nV_EQ                         : "));
            __mm_s.push_str(&*dumplistInteger(V_EQ)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn getVariableFirstOccurrenceInEquation(
    mut m: &ExtAdjacencyMatrix,
    mut varIndex: i32,
    mut minimalSetS: metamodelica::List<i32>,
) -> (i32, metamodelica::List<i32>) {
    let mut matchedEquation: (i32, metamodelica::List<i32>) = (0, metamodelica::nil());
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    for mut i in &**m {
        (eq, vars) = i.clone();
        if eq > 0 {
            if !(listMember(eq, minimalSetS.clone())) {
                if listMember(varIndex, vars) {
                    matchedEquation = i.clone();
                    break;
                }
            }
        }
    }
    matchedEquation
}

fn dumpResidualVars(
    mut instring: &ArcStr,
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut comment: &ArcStr,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n  //"));
        __mm_s.push_str(&*comment);
        ArcStr::from(__mm_s)
    };
    for mut var in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*literal!("\n  "));
            __mm_s.push_str(&*DAEDump::daeTypeStr(&var.varType)?);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*System::stringReplace(
                ComponentReference::crefStr(&cr)?,
                literal!("."),
                literal!("_"),
            )?);
            __mm_s.push_str(&*literal!(";"));
            ArcStr::from(__mm_s)
        };
        outstring = System::stringReplace(outstring, literal!("$"), literal!(""))?;
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instring);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

fn dumpExtractedVars(
    mut instring: &ArcStr,
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut comment: &ArcStr,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut creflast: metamodelica::Ref<DAE::ComponentRef>;
    let mut isRec: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut recordvarlist: metamodelica::List<ArcStr>;
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n  //"));
        __mm_s.push_str(&*comment);
        ArcStr::from(__mm_s)
    };
    recordvarlist = metamodelica::nil();
    for mut var in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
        (cr1, isRec) = ComponentReference::crefGetFirstRec(&cr)?;
        if BackendVariable::varHasUncertainValueRefine(metamodelica::AsArg::as_arg(&var)) {
            outstring = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*outstring);
                __mm_s.push_str(&*literal!("\n  parameter "));
                __mm_s.push_str(&*DAEDump::daeTypeStr(&var.varType)?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*System::stringReplace(
                    ComponentReference::crefStr(&cr)?,
                    literal!("."),
                    literal!("_"),
                )?);
                __mm_s.push_str(&*literal!(";"));
                ArcStr::from(__mm_s)
            };
        } else if BackendVariable::isParam(metamodelica::AsArg::as_arg(&var)) {
            outstring = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*outstring);
                __mm_s.push_str(&*literal!("\n  parameter "));
                __mm_s.push_str(&*DAEDump::daeTypeStr(&var.varType)?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*System::stringReplace(
                    ComponentReference::crefStr(&cr)?,
                    literal!("."),
                    literal!("_"),
                )?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionDump::printOptExpStr(var.bindExp.clone())?);
                __mm_s.push_str(&*literal!(";"));
                ArcStr::from(__mm_s)
            };
        } else if isRec && !(listMember(ComponentReference::crefStr(&cr1)?, recordvarlist.clone())) {
            creflast = ComponentReferenceBasics::crefLastCref(&cr1)?;
            path = Types::getRecordPath(&(ComponentReference::crefType(&creflast)?))?;
            recordvarlist = metamodelica::cons(ComponentReference::crefStr(&cr1)?, recordvarlist);
            outstring = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*outstring);
                __mm_s.push_str(&*literal!("\n  "));
                __mm_s.push_str(&*AbsynUtil::pathString(path, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*System::stringReplace(
                    ComponentReference::crefStr(&cr1)?,
                    literal!("."),
                    literal!("_"),
                )?);
                __mm_s.push_str(&*literal!(";"));
                ArcStr::from(__mm_s)
            };
        } else if !(isRec) {
            outstring = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*outstring);
                __mm_s.push_str(&*literal!("\n  "));
                __mm_s.push_str(&*DAEDump::daeTypeStr(&var.varType)?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*System::stringReplace(
                    ComponentReference::crefStr(&cr)?,
                    literal!("."),
                    literal!("_"),
                )?);
                __mm_s.push_str(&*literal!(";"));
                ArcStr::from(__mm_s)
            };
        }
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instring);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

fn dumpExtractedEquations(
    mut instring: &ArcStr,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut comment: &ArcStr,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n  //"));
        __mm_s.push_str(&*comment);
        ArcStr::from(__mm_s)
    };
    for mut eq in &*BackendEquation::equationList(eqs)? {
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*literal!("\n  "));
            __mm_s.push_str(&*dumpEquationString(metamodelica::AsArg::as_arg(&eq))?);
            __mm_s.push_str(&*literal!(";"));
            ArcStr::from(__mm_s)
        };
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instring);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

fn dumpExtractedEquationsToHTML(
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut comment: &ArcStr,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    if (BackendEquation::equationList(eqs.clone())?).is_empty() {
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("The set of "));
            __mm_s.push_str(&*comment);
            __mm_s.push_str(&*literal!(" is empty."));
            ArcStr::from(__mm_s)
        };
    } else {
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<html>\n<body>\n<h2>"));
            __mm_s.push_str(&*comment);
            __mm_s.push_str(&*literal!("</h2>\n<ol>"));
            ArcStr::from(__mm_s)
        };
        for mut eq in &*BackendEquation::equationList(eqs)? {
            outstring = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*outstring);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("  <li>"));
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(BackendEquation::equationSize(
                    metamodelica::AsArg::as_arg(&eq),
                )?));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?);
                __mm_s.push_str(&*literal!(" </li>"));
                ArcStr::from(__mm_s)
            };
        }
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*literal!("\n</ol>\n</body>\n</html>"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outstring)
}

pub(crate) fn setBoundaryConditionEquationsAndVars(
    mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
    mut debug: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem> = currentSystem;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut daeVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut updatedGlobalKnownVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    for mut var in &*BackendVariable::varList(&shared.globalKnownVars)? {
        let mut var = var.clone();
        if BackendVariable::isRealParam(&var)
            && (BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(&var)
                || BackendVariable::varHasUncertainValueRefine(&var)
                || BackendVariable::varHasUncertainValuePropagate(&var))
        {
            lhs = BackendVariable::varExp(&var)?;
            rhs = BackendVariable::varBindExpStartValueNoFail(&var)?;
            eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: lhs,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone(),
            });
            eqnLst = metamodelica::cons(eqn, eqnLst);
            var = BackendVariable::setVarKind(var, openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
            var = BackendVariable::setBindExp(var, None);
            daeVarsLst = metamodelica::cons(var, daeVarsLst);
        } else if (BackendVariable::isIntParam(&var) || BackendVariable::isBoolParam(&var))
            && BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(&var)
        {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        ": Boundary Condition cannot be set on Integer or Boolean parameters: "
                    ));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
                    __mm_s.push_str(&*literal!(" must be Real, The extraction algorithm will fail"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else {
            updatedGlobalKnownVarsLst = metamodelica::cons(var, updatedGlobalKnownVarsLst);
        }
    }
    if debug {
        BackendDump::dumpVarList(&daeVarsLst, &(literal!("boundaryConditionVarsTaggedAsParmeters")))?;
    }
    currentSystem = BackendVariable::addVarsDAE(&daeVarsLst, currentSystem)?;
    assign_field!(
        currentSystem.orderedEqs = BackendEquation::merge(
            currentSystem.orderedEqs.clone(),
            BackendEquation::listEquation(&eqnLst)?
        )?
    );
    shared = BackendDAEUtil::setSharedGlobalKnownVars(shared, BackendVariable::listVar(updatedGlobalKnownVarsLst)?);
    Ok((currentSystem, shared))
}

fn deleteEquationsFromEqSyst(
    mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut eqIndex: &metamodelica::List<i32>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem> = currentSystem;
    let mut newOrderedEquationArray: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    assign_field!(currentSystem.orderedEqs = BackendEquation::deleteList(currentSystem.orderedEqs.clone(), eqIndex)?);
    newOrderedEquationArray = BackendEquation::emptyEqns();
    BackendEquation::addList(
        &(BackendEquation::equationList(currentSystem.orderedEqs.clone())?),
        newOrderedEquationArray.clone(),
    )?;
    currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem, newOrderedEquationArray);
    Ok(currentSystem)
}

fn getBoundaryConditionsEquationIndex(
    mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut boundaryConditions: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut boundaryConditionsEquationIndexes: metamodelica::List<i32> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        for mut j in &**boundaryConditions {
            if i.clone() == list![j.clone()] {
                boundaryConditionsEquationIndexes = metamodelica::cons(count, boundaryConditionsEquationIndexes);
                break;
            }
        }
        count = count + 1;
    }
    boundaryConditionsEquationIndexes
}

fn getUncertainRefineVariablesBindedEquations(
    mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut knowns: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut knownsWithBindedEquations: metamodelica::List<i32> = metamodelica::nil();
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        for mut j in &**knowns {
            if i.clone() == list![j.clone()] {
                knownsWithBindedEquations = metamodelica::cons(j.clone(), knownsWithBindedEquations);
            }
        }
    }
    knownsWithBindedEquations
}

fn getExactConstantVariables(
    mut constantEquations: &metamodelica::List<i32>,
    mut solvedEqsVarInfo: &metamodelica::List<(i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut constantVariables: metamodelica::List<i32> = metamodelica::nil();
    let mut varNumber: i32;
    for mut eq in &**constantEquations {
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo)?;
        constantVariables = metamodelica::cons(varNumber, constantVariables);
    }
    Ok(constantVariables)
}

fn getBoundaryConditionVariables(
    mut boundaryConditionEquations: &metamodelica::List<i32>,
    mut solvedEqsVarInfo: &metamodelica::List<(i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut boundaryConditionVariables: metamodelica::List<i32> = metamodelica::nil();
    let mut varNumber: i32;
    for mut eq in &**boundaryConditionEquations {
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo)?;
        boundaryConditionVariables = metamodelica::cons(varNumber, boundaryConditionVariables);
    }
    Ok(boundaryConditionVariables)
}

fn getEquationsFromSBLTAndEBLT(
    mut inList: &metamodelica::List<i32>,
    mut sBLT_Equations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eBLT_Equations: &metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEquationsList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    for mut eqIndex in &**inList {
        if eqIndex.clone() > 0 {
            outEquationsList = metamodelica::cons(
                BackendEquation::get(sBLT_Equations.clone(), eqIndex.clone())?,
                outEquationsList,
            );
        } else {
            outEquationsList =
                metamodelica::cons(getEquationsFromEBLT(eqIndex.clone(), eBLT_Equations)?, outEquationsList);
        }
    }
    outEquationsList = outEquationsList.reverse();
    Ok(outEquationsList)
}

fn getEquationsFromEBLT(
    mut eBLTIndex: i32,
    mut eBLT_Equations: &metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut outEquations: metamodelica::Ref<BackendDAE::Equation>;
    let mut index: i32;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    for mut eqs in &**eBLT_Equations {
        (index, eq) = eqs.clone();
        if intEq(eBLTIndex, index) {
            outEquations = eq;
            return Ok(outEquations);
        }
    }
    return Err("fail");
    Ok(outEquations)
}

fn getAbsoluteIndexHelper(
    mut inList: &metamodelica::List<i32>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut outList: metamodelica::List<i32> = metamodelica::nil();
    for mut i in &**inList {
        if i.clone() > 0 {
            outList = metamodelica::cons(
                (*metamodelica::index_checked(&__ab_mapIncRowEqn, i.clone())?).clone(),
                outList,
            );
        } else {
            outList = metamodelica::cons(i.clone(), outList);
        }
    }
    outList = outList.reverse();
    Ok(outList)
}

fn dumpSetSTargetEquations(
    mut eq: i32,
    mut solvedEqsVarInfo: &metamodelica::List<(i32, i32)>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut orderedVars: &BackendDAE::Variables,
    mut heading: &ArcStr,
) -> Result<()> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut count: i32 = 1;
    let mut varNumber: i32;
    let mut mappedEq: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut tmpEq: metamodelica::Ref<BackendDAE::Equation>;
    varNumber = getSolvedVariableNumber(eq, solvedEqsVarInfo)?;
    var = BackendVariable::getVarAt(orderedVars, varNumber)?;
    mappedEq = (*metamodelica::index_checked(&__ab_mapIncRowEqn, eq)?).clone();
    tmpEq = BackendEquation::get(orderedEqs, mappedEq)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*intString(varNumber));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(mappedEq));
        __mm_s.push_str(&*literal!("/"));
        __mm_s.push_str(&*intString(eq));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(BackendEquation::equationSize(&tmpEq)?));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*BackendDump::equationString(&tmpEq)?);
        ArcStr::from(__mm_s)
    });
    count = count + 1;
    Ok(())
}

fn dumpSetSVarsSolvedInfo(
    mut tempSetS: &metamodelica::List<i32>,
    mut solvedEqsVarInfo: &metamodelica::List<(i32, i32)>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut orderedVars: &BackendDAE::Variables,
    mut heading: &ArcStr,
) -> Result<()> {
    let __ab_mapIncRowEqn = mapIncRowEqn.borrow();
    let mut count: i32 = 1;
    let mut varNumber: i32;
    let mut mappedEq: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut tmpEq: metamodelica::Ref<BackendDAE::Equation>;
    if !(stringEmpty(&heading)) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(((tempSetS).len() as i32)));
            __mm_s.push_str(&*literal!(")"));
            __mm_s.push_str(&*literal!(
                "\n============================================================\n"
            ));
            ArcStr::from(__mm_s)
        });
    }
    for mut eq in &**tempSetS {
        varNumber = getSolvedVariableNumber(eq.clone(), solvedEqsVarInfo)?;
        var = BackendVariable::getVarAt(orderedVars, varNumber)?;
        mappedEq = (*metamodelica::index_checked(&__ab_mapIncRowEqn, eq.clone())?).clone();
        tmpEq = BackendEquation::get(orderedEqs.clone(), mappedEq)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*intString(varNumber));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(mappedEq));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*intString(eq.clone()));
            __mm_s.push_str(&*literal!("): "));
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(BackendEquation::equationSize(&tmpEq)?));
            __mm_s.push_str(&*literal!("): "));
            __mm_s.push_str(&*BackendDump::equationString(&tmpEq)?);
            ArcStr::from(__mm_s)
        });
        count = count + 1;
    }
    metamodelica::print(literal!("\n\n"));
    Ok(())
}

fn dumpSetSVars(mut setSVars: &BackendDAE::Variables, mut heading: &ArcStr) -> Result<()> {
    let mut count: i32 = 1;
    let mut var: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(BackendVariable::varsSize(setSVars)));
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*literal!("========================================"));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    for mut var in &*BackendVariable::varList(setSVars)? {
        let mut var = var.clone();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*intString(count));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&var.varName)?);
            __mm_s.push_str(&*literal!(" type: "));
            __mm_s.push_str(&*DAEDump::daeTypeStr(&var.varType)?);
            ArcStr::from(__mm_s)
        });
        count = count + 1;
    }
    metamodelica::print(literal!("\n\n"));
    Ok(())
}

fn dumpBlockStatus(
    mut allBlocks: &metamodelica::List<metamodelica::List<i32>>,
    mut allBlocksStatusVarInfo: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<()> {
    let mut count: i32 = 1;
    metamodelica::print(literal!("\nBLT-BLOCK_STATUS\n=================\n"));
    for mut blocks in &**allBlocks {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBlock :"));
            __mm_s.push_str(&*dumplistInteger(blocks.clone())?);
            __mm_s.push_str(&*literal!(" || blockStatusVarInfo :"));
            __mm_s.push_str(&*anyString((allBlocksStatusVarInfo).get(count)?));
            ArcStr::from(__mm_s)
        });
        count = count + 1;
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn dumpBlockTargets(
    mut s_BLTBlockTargetInfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
) -> Result<()> {
    let mut mainBlock: metamodelica::List<i32>;
    let mut targetBlocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetBlocksStatusVarInfo: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    metamodelica::print(literal!("\nS-BLTBlocks-TargetInfo\n=======================\n"));
    for mut blocks in &**s_BLTBlockTargetInfo {
        (mainBlock, targetBlocks, targetBlocksStatusVarInfo) = blocks.clone();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBlock :"));
            __mm_s.push_str(&*dumplistInteger(mainBlock)?);
            __mm_s.push_str(&*literal!(" || blockTargetsInfo :"));
            __mm_s.push_str(&*anyString(targetBlocks));
            __mm_s.push_str(&*literal!(" || blockStatusVarInfo :"));
            __mm_s.push_str(&*anyString(targetBlocksStatusVarInfo));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn dumpPredecessorBlocks(
    mut predecessorBlockInfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
) -> Result<()> {
    let mut knownBlocks: metamodelica::List<i32>;
    let mut constantBlocks: metamodelica::List<i32>;
    let mut blueBlocksTargets: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    let mut redBlocksTargets: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    let mut constantBlocksTargets: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    metamodelica::print(literal!(
        "\nTargets of blocks without predecessors:\n========================================"
    ));
    for mut blocks in &**predecessorBlockInfo {
        (_, _, _, knownBlocks, constantBlocks, _) = blocks.clone();
        if !((knownBlocks).is_empty()) {
            blueBlocksTargets = metamodelica::cons(blocks.clone(), blueBlocksTargets);
        } else if !((constantBlocks).is_empty()) {
            constantBlocksTargets = metamodelica::cons(blocks.clone(), constantBlocksTargets);
        } else {
            redBlocksTargets = metamodelica::cons(blocks.clone(), redBlocksTargets);
        }
    }
    metamodelica::print(literal!("\n"));
    dumpPredecessorBlocksHelper(
        blueBlocksTargets,
        &(literal!("knowns")),
        &(literal!("Targets of Blue blocks")),
    )?;
    dumpPredecessorBlocksHelper(
        redBlocksTargets,
        &(literal!("unknowns")),
        &(literal!("Targets of Red blocks")),
    )?;
    dumpPredecessorBlocksHelper(
        constantBlocksTargets,
        &(literal!("constant")),
        &(literal!("Targets of Brown blocks")),
    )?;
    Ok(())
}

fn dumpPredecessorBlocksHelper(
    mut predecessorBlockInfo: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
    mut blockInfo: &ArcStr,
    mut header: &ArcStr,
) -> Result<()> {
    let mut mainBlock: metamodelica::List<i32>;
    let mut targetBlocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut knownBlocks: metamodelica::List<i32>;
    let mut constantBlocks: metamodelica::List<i32>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*header);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((predecessorBlockInfo).len() as i32)));
        __mm_s.push_str(&*literal!(")"));
        __mm_s.push_str(&*literal!("\n==============================\n"));
        ArcStr::from(__mm_s)
    });
    for mut blocks in &*predecessorBlockInfo.reverse() {
        (mainBlock, targetBlocks, _, knownBlocks, constantBlocks, _) = blocks.clone();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBlock :"));
            __mm_s.push_str(&*dumplistInteger(mainBlock)?);
            __mm_s.push_str(&*literal!(" || blockTargetsInfo :"));
            __mm_s.push_str(&*anyString(targetBlocks));
            __mm_s.push_str(&*literal!(" || KnownBlocks :"));
            __mm_s.push_str(&*dumplistInteger(knownBlocks)?);
            __mm_s.push_str(&*literal!(" || constantBlocks :"));
            __mm_s.push_str(&*dumplistInteger(constantBlocks)?);
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("\n\n"));
    Ok(())
}

pub(crate) fn ExtractEquationsUsingSetOperations(
    mut predecessorBlockInfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
    mut e_BLTBlockRanks: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut approximatedEquations: &metamodelica::List<i32>,
    mut debug: bool,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut setC: metamodelica::List<i32>;
    let mut setS: metamodelica::List<i32>;
    let mut mainBlock: metamodelica::List<i32>;
    let mut tmpSetC_1: metamodelica::List<i32>;
    let mut tmpSetC_2: metamodelica::List<i32>;
    let mut tmpSetS_1: metamodelica::List<i32>;
    let mut tmpSetS_2: metamodelica::List<i32>;
    let mut z1: metamodelica::List<i32>;
    let mut z2: metamodelica::List<i32>;
    let mut targetBlocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut knownBlocks: metamodelica::List<i32>;
    let mut constantBlocks: metamodelica::List<i32>;
    let mut e_BLTBlockRanksWithoutRanks: metamodelica::List<i32> = metamodelica::nil();
    let mut targetBlocksWithKnowns: metamodelica::List<i32> = metamodelica::nil();
    let mut targetBlocksWithUnknowns: metamodelica::List<i32> = metamodelica::nil();
    let mut targetBlocksWithConstants: metamodelica::List<i32> = metamodelica::nil();
    for mut blocks in &**predecessorBlockInfo {
        (mainBlock, targetBlocks, _, knownBlocks, constantBlocks, _) = blocks.clone();
        if !((knownBlocks).is_empty()) {
            targetBlocksWithKnowns = filterTargetBlocksWithoutRanks(&((targetBlocks).rest()?), targetBlocksWithKnowns);
        } else if !((constantBlocks).is_empty()) {
            targetBlocksWithConstants = filterTargetBlocksWithoutRanks(&targetBlocks, targetBlocksWithConstants);
        } else {
            targetBlocksWithUnknowns = filterTargetBlocksWithoutRanks(&targetBlocks, targetBlocksWithUnknowns);
        }
    }
    targetBlocksWithKnowns = List::unique(&targetBlocksWithKnowns);
    targetBlocksWithUnknowns = List::unique(&targetBlocksWithUnknowns);
    targetBlocksWithConstants = List::unique(&targetBlocksWithConstants);
    e_BLTBlockRanksWithoutRanks = filterTargetBlocksWithoutRanks(e_BLTBlockRanks, e_BLTBlockRanksWithoutRanks);
    if debug {
        metamodelica::print(literal!(
            "\nUnion of Blue, Red and Yellow and E-BLT-Blocks\n====================================================="
        ));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnion-E-BLT-blocks                                     :"));
            __mm_s.push_str(&*dumplistInteger(e_BLTBlockRanksWithoutRanks.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnion-Blue-TargetBlockInfo (blocks with Knowns)        :"));
            __mm_s.push_str(&*dumplistInteger(targetBlocksWithKnowns.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnion-Red-TargetBlockInfo  (blocks with UnKnowns)      :"));
            __mm_s.push_str(&*dumplistInteger(targetBlocksWithUnknowns.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nUnion-Brown-TargetBlockInfo  (blocks with Exact eqns)  :"));
            __mm_s.push_str(&*dumplistInteger(targetBlocksWithConstants.clone())?);
            ArcStr::from(__mm_s)
        });
    }
    tmpSetC_1 = List::intersectionOnTrue(
        &targetBlocksWithKnowns,
        &e_BLTBlockRanksWithoutRanks,
        &fnptr!(intEq, i32, i32),
    )?;
    tmpSetC_2 = List::intersectionOnTrue(
        &targetBlocksWithUnknowns,
        &e_BLTBlockRanksWithoutRanks,
        &fnptr!(intEq, i32, i32),
    )?;
    setC = List::setDifferenceOnTrue(tmpSetC_1.clone(), &tmpSetC_2, &fnptr!(intEq, i32, i32))?;
    setC = List::setDifferenceOnTrue(setC, approximatedEquations, &fnptr!(intEq, i32, i32))?;
    if debug {
        metamodelica::print(literal!("\n\nSetC-Operations\n===================="));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n(BlocksWithKnowns) intersection (e_BLTBlocks)   :"));
            __mm_s.push_str(&*dumplistInteger(tmpSetC_1)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n(BlocksWithUnknowns) intersection (e_BLTBlocks) :"));
            __mm_s.push_str(&*dumplistInteger(tmpSetC_2)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nSetC                                            :"));
            __mm_s.push_str(&*dumplistInteger(setC.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    tmpSetS_1 = List::setDifferenceOnTrue(
        targetBlocksWithKnowns,
        &targetBlocksWithUnknowns,
        &fnptr!(intEq, i32, i32),
    )?;
    tmpSetS_2 = List::setDifferenceOnTrue(
        tmpSetS_1.clone(),
        &e_BLTBlockRanksWithoutRanks,
        &fnptr!(intEq, i32, i32),
    )?;
    z1 = List::setDifferenceOnTrue(
        targetBlocksWithConstants,
        &targetBlocksWithUnknowns,
        &fnptr!(intEq, i32, i32),
    )?;
    z2 = List::setDifferenceOnTrue(z1.clone(), &e_BLTBlockRanksWithoutRanks, &fnptr!(intEq, i32, i32))?;
    setS = List::unique(&(List::union(&tmpSetS_2, &z2)));
    setS = List::setDifferenceOnTrue(setS, approximatedEquations, &fnptr!(intEq, i32, i32))?;
    if debug {
        metamodelica::print(literal!("\nSetS-Operations\n=================="));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\n(BlocksWithKnowns - BlocksWithUnknowns)                  :"
            ));
            __mm_s.push_str(&*dumplistInteger(tmpSetS_1)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\n((BlocksWithKnowns - BlocksWithUnknowns) - e_BLTBlocks)) :"
            ));
            __mm_s.push_str(&*dumplistInteger(tmpSetS_2)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\nz1(B) => (ConstantBlocks - UnknownsBlocks)               :"
            ));
            __mm_s.push_str(&*dumplistInteger(z1)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\nz2(B) => (z1(B) - e_BLTBlocks)                           :"
            ));
            __mm_s.push_str(&*dumplistInteger(z2)?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "\nSetS                                                     :"
            ));
            __mm_s.push_str(&*dumplistInteger(setS.clone())?);
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\n"));
    }
    Ok((setC, setS))
}

pub(crate) fn filterTargetBlocksWithoutRanks(
    mut targetBlocks: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inBlocks: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut outBlocks: metamodelica::List<i32>;
    let mut mainBlocks: metamodelica::List<i32> = metamodelica::nil();
    for mut blocks in &**targetBlocks {
        mainBlocks = List::append_reverse(&(Util::tuple21(blocks.clone())), mainBlocks);
    }
    outBlocks = listAppend(inBlocks, mainBlocks.reverse());
    outBlocks
}

pub(crate) fn setEBLTEquationsWithIndexAndRank(
    mut unMatchedEqList: &metamodelica::List<i32>,
    mut unMatchedEqsLstCorrectIndex: &metamodelica::List<i32>,
    mut inEqArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>)>,
    ExtAdjacencyMatrix,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<(metamodelica::List<i32>, i32)>,
)> {
    let mut eBLT_Equation_WithIndex: metamodelica::List<(i32, metamodelica::Ref<BackendDAE::Equation>)> =
        metamodelica::nil();
    let mut e_BLTAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut e_BLTSolvedEqsAndVars: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut e_BLTBlocks: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_BLTBlockRanks: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut actualIndex: i32;
    let mut index: i32 = -1;
    let mut varsInfoList: metamodelica::List<i32>;
    for mut i in &**unMatchedEqList {
        actualIndex = (unMatchedEqsLstCorrectIndex).get(count)?;
        eBLT_Equation_WithIndex = metamodelica::cons(
            (index, BackendEquation::get(inEqArray.clone(), actualIndex)?),
            eBLT_Equation_WithIndex,
        );
        varsInfoList = metamodelica::arrayGet(adjacencyMatrix.clone(), i.clone())?;
        e_BLTAdjacencyMatrix = metamodelica::cons((index, varsInfoList.clone()), e_BLTAdjacencyMatrix);
        e_BLTSolvedEqsAndVars = metamodelica::cons(
            (
                index,
                (List::sort(
                    varsInfoList,
                    (std::sync::Arc::new(fnptr!(intLt, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?)
                .get(1)?,
            ),
            e_BLTSolvedEqsAndVars,
        );
        e_BLTBlocks = metamodelica::cons(list![index], e_BLTBlocks);
        e_BLTBlockRanks = metamodelica::cons((list![index], index), e_BLTBlockRanks);
        index = index - 1;
        count = count + 1;
    }
    eBLT_Equation_WithIndex = eBLT_Equation_WithIndex.reverse();
    e_BLTAdjacencyMatrix = e_BLTAdjacencyMatrix.reverse();
    e_BLTSolvedEqsAndVars = e_BLTSolvedEqsAndVars.reverse();
    e_BLTBlocks = e_BLTBlocks.reverse();
    e_BLTBlockRanks = e_BLTBlockRanks.reverse();
    Ok((
        eBLT_Equation_WithIndex,
        e_BLTAdjacencyMatrix,
        e_BLTSolvedEqsAndVars,
        e_BLTBlocks,
        e_BLTBlockRanks,
    ))
}

pub(crate) fn inverseModelicaModel(
    mut inVar: &BackendDAE::Variables,
    mut knownVariablesWithEquationBinding: &metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut variablesOfInterest: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut variablesOfInterestIndexes: metamodelica::List<i32>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    variablesOfInterest = List::filterOnTrue(
        BackendVariable::varList(inVar)?,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::varHasUncertainValueRefine(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    for mut var in &*variablesOfInterest {
        variablesOfInterestIndexes = BackendVariable::getVarIndexFromVars(&(list![var.clone()]), inVar);
        if (List::intersectionOnTrue(
            &variablesOfInterestIndexes,
            knownVariablesWithEquationBinding,
            &fnptr!(intEq, i32, i32),
        )?)
        .is_empty()
        {
            eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: Expression::crefExp(var.varName.clone())?,
                scalar: metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(),
            });
            eqnlst = metamodelica::cons(eq, eqnlst);
        }
    }
    Ok(eqnlst)
}

pub(crate) fn dumplistInteger(mut inlist: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut outstring: ArcStr;
    let mut s: metamodelica::List<ArcStr>;
    s = List::map(inlist, &fnptr!(intString, i32))?;
    outstring = stringDelimitList(s, literal!(", "));
    outstring = stringAppendList(list![literal!("{"), outstring, literal!("}")]);
    Ok(outstring)
}

pub(crate) fn traverseBLTAndUpdateBlockStatus(
    mut inlist: &metamodelica::List<metamodelica::List<i32>>,
    mut knowns: metamodelica::List<i32>,
    mut boundaryConditionVars: &metamodelica::List<i32>,
    mut exactEquationVars: metamodelica::List<i32>,
    mut solvedVariables: &metamodelica::List<(i32, i32)>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<ArcStr>>,
)> {
    let mut outlist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut outstringlist: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blocks: metamodelica::List<i32>;
    let mut blockinfo: metamodelica::List<ArcStr>;
    for mut i in &**inlist {
        (blocks, blockinfo) = checkBlueOrRedOrBrownBlocks(
            metamodelica::AsArg::as_arg(&i),
            knowns.clone(),
            boundaryConditionVars,
            exactEquationVars.clone(),
            solvedVariables,
        )?;
        outlist = metamodelica::cons(blocks, outlist);
        outstringlist = metamodelica::cons(blockinfo, outstringlist);
    }
    outlist = outlist.reverse();
    outstringlist = outstringlist.reverse();
    Ok((outlist, outstringlist))
}

pub(crate) fn checkBlueOrRedOrBrownBlocks(
    mut inlist: &metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut boundaryConditionVars: &metamodelica::List<i32>,
    mut exactEquationVars: metamodelica::List<i32>,
    mut solvedVar: &metamodelica::List<(i32, i32)>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<ArcStr>)> {
    let mut outIntegerList: metamodelica::List<i32> = metamodelica::nil();
    let mut outStringList: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut varNumber: i32;
    for mut i in &**inlist {
        varNumber = getSolvedVariableNumber(i.clone(), solvedVar)?;
        if listMember(varNumber, knowns.clone()) {
            outStringList = metamodelica::cons(literal!("knowns"), outStringList);
            outIntegerList = metamodelica::cons(i.clone(), outIntegerList);
        } else if listMember(varNumber, exactEquationVars.clone()) {
            outStringList = metamodelica::cons(literal!("constants"), outStringList);
            outIntegerList = metamodelica::cons(i.clone(), outIntegerList);
        } else {
            outStringList = metamodelica::cons(literal!("unknowns"), outStringList);
            outIntegerList = metamodelica::cons(i.clone(), outIntegerList);
        }
    }
    outIntegerList = outIntegerList.reverse();
    outStringList = outStringList.reverse();
    Ok((outIntegerList, outStringList))
}

pub(crate) fn getSolvedVariableNumber(mut eqnumber: i32, mut inlist: &metamodelica::List<(i32, i32)>) -> Result<i32> {
    let mut solvedvar: i32;
    let mut solvedeq: i32;
    for mut var in &**inlist {
        (solvedeq, solvedvar) = var.clone();
        if intEq(eqnumber, solvedeq) {
            return Ok(solvedvar);
        }
    }
    return Err("fail");
    Ok(solvedvar)
}

pub(crate) fn getSolvedEquationNumber(mut varnumber: i32, mut inlist: &metamodelica::List<(i32, i32)>) -> Result<i32> {
    let mut solvedeq: i32;
    let mut solvedvar: i32;
    for mut var in &**inlist {
        (solvedeq, solvedvar) = var.clone();
        if intEq(varnumber, solvedvar) {
            return Ok(solvedeq);
        }
    }
    return Err("fail");
    Ok(solvedeq)
}

pub(crate) fn getSolvedEquationAndVarsInfo(
    mut v: metamodelica::Array<i32>,
) -> (metamodelica::List<(i32, i32)>, metamodelica::List<i32>) {
    let mut eqvarlist: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut solvedEqLst: metamodelica::List<i32> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = v.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqvarlist = metamodelica::cons((i, count), eqvarlist);
        solvedEqLst = metamodelica::cons(i, solvedEqLst);
        count = count + 1;
    }
    (eqvarlist, solvedEqLst)
}

fn getVariablesBlockCategories(
    mut allVariables: &BackendDAE::Variables,
    mut variableIndexList: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut knowns: metamodelica::List<i32> = metamodelica::nil();
    let mut boundaryConditionVars: metamodelica::List<i32> = metamodelica::nil();
    let mut exactEquationVars: metamodelica::List<i32> = metamodelica::nil();
    let mut unMeasuredVariablesOfInterest: metamodelica::List<i32> = metamodelica::nil();
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    for mut index in &**variableIndexList {
        var = BackendVariable::getVarAt(allVariables, index.clone())?;
        if BackendVariable::varHasUncertainValueRefine(&(BackendVariable::getVarAt(allVariables, index.clone())?)) {
            knowns = metamodelica::cons(index.clone(), knowns);
        } else if BackendVariable::hasOpenModelicaBoundaryConditionAnnotation(&var) {
            boundaryConditionVars = metamodelica::cons(index.clone(), boundaryConditionVars);
        } else {
            exactEquationVars = metamodelica::cons(index.clone(), exactEquationVars);
        }
        if BackendVariable::varHasUncertainValuePropagate(&(BackendVariable::getVarAt(allVariables, index.clone())?)) {
            unMeasuredVariablesOfInterest = metamodelica::cons(index.clone(), unMeasuredVariablesOfInterest);
        }
    }
    Ok((
        knowns,
        boundaryConditionVars,
        exactEquationVars,
        unMeasuredVariablesOfInterest,
    ))
}

fn getUncertainRefineAndUnknownVariableIndexes(
    mut allVariables: &BackendDAE::Variables,
    mut variableIndexList: &metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut knowns: metamodelica::List<i32> = metamodelica::nil();
    let mut unknowns: metamodelica::List<i32> = metamodelica::nil();
    for mut index in &**variableIndexList {
        if BackendVariable::varHasUncertainValueRefine(&(BackendVariable::getVarAt(allVariables, index.clone())?)) {
            knowns = metamodelica::cons(index.clone(), knowns);
        } else {
            unknowns = metamodelica::cons(index.clone(), unknowns);
        }
    }
    Ok((knowns, unknowns))
}

pub(crate) fn dumpListList(
    mut lstLst: metamodelica::List<metamodelica::List<i32>>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(":\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(List::map(lstLst, &dumplistInteger)?, literal!(",")));
        __mm_s.push_str(&*literal!("}"));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn getEquationsTaggedApproximatedOrBoundaryCondition(
    mut eqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut index: i32,
) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut approximatedEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut boundaryConditionEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut isApproximateEquations: bool;
    let mut isConstantEquations: bool;
    let mut i: i32;
    i = index;
    for mut eq in &**eqs {
        (isApproximateEquations, isConstantEquations) =
            isEquationTaggedApproximatedOrBoundaryCondition(metamodelica::AsArg::as_arg(&eq));
        if isApproximateEquations {
            approximatedEquations = metamodelica::cons(i, approximatedEquations);
        } else if isConstantEquations {
            boundaryConditionEquations = metamodelica::cons(i, boundaryConditionEquations);
        }
        i = i + 1;
    }
    (approximatedEquations, boundaryConditionEquations)
}

fn isEquationTaggedApproximatedOrBoundaryCondition(mut eqn: &metamodelica::Ref<BackendDAE::Equation>) -> (bool, bool) {
    let mut approximatedEquations: bool;
    let mut boundaryConditionEquations: bool;
    (approximatedEquations, boundaryConditionEquations) = (::match_deref::match_deref! { match eqn {
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { comment, .. }, .. } => {
            let mut isApproximatedEquation: bool;
            let mut isboundaryConditionEquations: bool;
            (isApproximatedEquation, isboundaryConditionEquations) = isEquationTaggedApproximatedOrBoundaryConditionHelper(metamodelica::AsArg::as_arg(&comment));
            (isApproximatedEquation, isboundaryConditionEquations)
        },
        _ => {
            (false, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (approximatedEquations, boundaryConditionEquations)
}

fn isEquationTaggedApproximatedOrBoundaryConditionHelper(
    mut commentIn: &metamodelica::List<metamodelica::Ref<SCode::Comment>>,
) -> (bool, bool) {
    let mut approximatedEquations: bool;
    let mut boundaryConditionEquations: bool;
    (approximatedEquations, boundaryConditionEquations) = 'mc: {
        let __mc_input = &**commentIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((false, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst, .. } }), .. }, tail: t } => {
                    let mut isApproximatedEquation: bool;
                    let mut isboundaryConditionEquation: bool;
                    isApproximatedEquation = List::any(metamodelica::AsArg::as_arg(&subModLst), &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isEquationTaggedApproximated(&__a0)) })? || (isEquationTaggedApproximatedOrBoundaryConditionHelper(metamodelica::AsArg::as_arg(&t))).0;
                    isboundaryConditionEquation = List::any(metamodelica::AsArg::as_arg(&subModLst), &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isEquationTaggedBoundaryCondition(&__a0)) })? || (isEquationTaggedApproximatedOrBoundaryConditionHelper(metamodelica::AsArg::as_arg(&t))).0;
                    Ok((isApproximatedEquation, isboundaryConditionEquation))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: t } => {
                    let mut isApproximatedEquation: bool;
                    let mut isboundaryConditionEquation: bool;
                    (isApproximatedEquation, isboundaryConditionEquation) = isEquationTaggedApproximatedOrBoundaryConditionHelper(metamodelica::AsArg::as_arg(&t));
                    Ok((isApproximatedEquation, isboundaryConditionEquation))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (approximatedEquations, boundaryConditionEquations)
}

fn isEquationTaggedApproximated(mut m: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut approximatedEquations: bool;
    approximatedEquations = (::match_deref::match_deref! { match m {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ApproximatedEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    approximatedEquations
}

fn isEquationTaggedBoundaryCondition(mut m: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut boundaryCondition: bool;
    boundaryCondition = (::match_deref::match_deref! { match m {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_BoundaryCondition", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    boundaryCondition
}

fn isEquationTaggedConstant(mut m: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut constantEquations: bool;
    constantEquations = (::match_deref::match_deref! { match m {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ExactConstantEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    constantEquations
}

pub(crate) fn getSBLTAdjacencyMatrix(
    mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
) -> ExtAdjacencyMatrix {
    let mut extAdjacencyMatrix: ExtAdjacencyMatrix = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = adjacencyMatrix.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut vars in __range0 {
        extAdjacencyMatrix = metamodelica::cons((count, vars), extAdjacencyMatrix);
        count = count + 1;
    }
    extAdjacencyMatrix = extAdjacencyMatrix.reverse();
    extAdjacencyMatrix
}

/*
 Block Target Alogrithm
*/
pub(crate) fn findBlockTargets(
    mut inlist1: metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut solvedvariables: metamodelica::List<(i32, i32)>,
    mut mxt: ExtAdjacencyMatrix,
    mut blockranks: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut debug: bool,
) -> Result<
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
> {
    let mut outlist: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )> = metamodelica::nil();
    let mut targetblocks: metamodelica::List<metamodelica::List<i32>>;
    let mut eBLTBlocks: metamodelica::List<metamodelica::List<i32>>;
    let mut targetvarlist: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockvarlst: metamodelica::List<ArcStr>;
    let mut ranklist: metamodelica::List<i32>;
    let mut blocks1: metamodelica::List<i32>;
    let mut rank: i32;
    let mut updatedblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    if debug {
        metamodelica::print(literal!(
            "\nDetailed BlockTarget Dependency tree:\n========================================\n"
        ));
    }
    for mut i in &*inlist1 {
        if (i).get(1)? > 0 {
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\nFIND Blocks target of :"));
                    __mm_s.push_str(&*anyString(i.clone()));
                    __mm_s.push_str(&*literal!("\n========================"));
                    ArcStr::from(__mm_s)
                });
            }
            (targetblocks, eBLTBlocks) = findBlockTargetsHelper(
                &(list![i.clone()]),
                inlist2,
                solvedvariables.clone(),
                mxt.clone(),
                inlist1.clone(),
                debug,
            )?;
            targetblocks = listAppend(metamodelica::cons(i.clone(), targetblocks), eBLTBlocks);
            (updatedblocks, ranklist) = findBlocksRanks(blockranks, &targetblocks)?;
            updatedblocks = sortBlocks(&ranklist, &updatedblocks);
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\nFinal-Target-Blocks : "));
                    __mm_s.push_str(&*anyString(updatedblocks.clone()));
                    __mm_s.push_str(&*literal!(" || rankList"));
                    __mm_s.push_str(&*anyString(ranklist));
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
            targetvarlist = metamodelica::nil();
            for mut blocks in &*updatedblocks {
                (blocks1, rank) = blocks.clone();
                blockvarlst = getBlockVarList(&blocks1, &inlist1, inlist2)?;
                targetvarlist = metamodelica::cons((blockvarlst, rank), targetvarlist);
            }
            outlist = metamodelica::cons((i.clone(), updatedblocks, targetvarlist.reverse()), outlist);
        }
    }
    outlist = outlist.reverse();
    Ok(outlist)
}

pub(crate) fn findBlockTargetsHelper(
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut solvedvariables: metamodelica::List<(i32, i32)>,
    mut mxt: ExtAdjacencyMatrix,
    mut actualblocks: metamodelica::List<metamodelica::List<i32>>,
    mut debug: bool,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut outSBLT: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut outEBLT: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    (outSBLT, outEBLT) = (::match_deref::match_deref! { match (inlist1, inlist2) {
        (Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: firstitem, tail: restitem }) => {
            let mut solvar = solvedvariables;
            let mut mxt1 = mxt;
            let mut originalblocks = actualblocks;
            let mut b = debug;
            let mut dependencyequation: metamodelica::List<i32>;
            let mut targetblocks: metamodelica::List<metamodelica::List<i32>>;
            let mut targetblocks1: metamodelica::List<metamodelica::List<i32>>;
            let mut eBLTList1: metamodelica::List<metamodelica::List<i32>>;
            let mut eBLTList2: metamodelica::List<metamodelica::List<i32>>;
            (dependencyequation, eBLTList1) = findBlockTargetsHelper1(&(metamodelica::cons(first.clone(), rest.clone())), &solvar, &mxt1)?;
            if debug {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTargetBlocks :")); __mm_s.push_str(&*anyString(dependencyequation.clone())); __mm_s.push_str(&*literal!(" || EBLT_Block")); __mm_s.push_str(&*anyString(eBLTList1.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            targetblocks = getActualBlocks(&dependencyequation, &originalblocks, metamodelica::AsArg::as_arg(&first))?;
            (targetblocks1, eBLTList2) = findBlockTargetsHelper(&targetblocks, &(metamodelica::cons(firstitem.clone(), restitem.clone())), solvar, mxt1, originalblocks, b)?;
            (List::unique(&(listAppend(targetblocks, targetblocks1))), List::unique(&(listAppend(eBLTList1, eBLTList2))))
        },
        (_, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSBLT, outEBLT))
}

pub(crate) fn findBlockTargetsHelper1(
    mut inlist: &metamodelica::List<metamodelica::List<i32>>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
    mut mxt: &ExtAdjacencyMatrix,
) -> Result<(metamodelica::List<i32>, metamodelica::List<metamodelica::List<i32>>)> {
    let mut outSBLT: metamodelica::List<i32> = metamodelica::nil();
    let mut outEBLT: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSBLT: metamodelica::List<i32>;
    let mut tmpEBLT: metamodelica::List<i32>;
    for mut i in &**inlist {
        (tmpSBLT, tmpEBLT) = getDependencyequation(i.clone(), metamodelica::nil(), solvedvariables, mxt)?;
        outSBLT = listAppend(outSBLT, tmpSBLT);
        outEBLT = List::appendElt(tmpEBLT, outEBLT);
    }
    Ok((outSBLT, outEBLT))
}

pub(crate) fn getDependencyequation(
    mut inlist: metamodelica::List<i32>,
    mut inlist1: metamodelica::List<i32>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
    mut m: &ExtAdjacencyMatrix,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut outSBLT: metamodelica::List<i32>;
    let mut outEBLT: metamodelica::List<i32> = metamodelica::nil();
    let mut t: metamodelica::List<i32> = metamodelica::nil();
    let mut nonsq: metamodelica::List<i32>;
    let mut eqnumber: i32 = 0;
    let mut varnumber: i32;
    for mut eqnumber in &*inlist {
        let mut eqnumber = eqnumber.clone();
        varnumber = getSolvedVariableNumber(eqnumber, solvedvariables)?;
        (nonsq, outEBLT) = getdirectOccurrencesinEquation(m, eqnumber, varnumber);
        for mut lst in &*nonsq {
            if !(listMember(lst.clone(), inlist.clone())) {
                t = metamodelica::cons(lst.clone(), t);
            }
        }
    }
    outSBLT = listAppend(t, inlist1);
    Ok((outSBLT, outEBLT))
}

pub(crate) fn getdirectOccurrencesinEquation(
    mut m: &ExtAdjacencyMatrix,
    mut eqnumber: i32,
    mut varnumber: i32,
) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut outSBLT: metamodelica::List<i32> = metamodelica::nil();
    let mut outEBLT: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    for mut i in &**m {
        (eq, vars) = i.clone();
        if !(intEq(eq, eqnumber)) {
            if listMember(varnumber, vars) {
                if eq > 0 {
                    outSBLT = metamodelica::cons(eq, outSBLT);
                } else {
                    outEBLT = metamodelica::cons(eq, outEBLT);
                    break;
                }
            }
        }
    }
    outSBLT = outSBLT.reverse();
    outEBLT = outEBLT.reverse();
    (outSBLT, outEBLT)
}

pub(crate) fn findBlocksRanks(
    mut inlist1: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<(metamodelica::List<i32>, i32)>,
    metamodelica::List<i32>,
)> {
    let mut outlist: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut ranklist: metamodelica::List<i32> = metamodelica::nil();
    let mut blocks: metamodelica::List<i32>;
    let mut s_BLTRanks: metamodelica::List<i32> = metamodelica::nil();
    let mut e_BLTRanks: metamodelica::List<i32> = metamodelica::nil();
    let mut rank: i32;
    for mut i in &**inlist2 {
        for mut j in &**inlist1 {
            (blocks, rank) = j.clone();
            if i.clone() == blocks {
                outlist = metamodelica::cons((i.clone(), rank), outlist);
                if rank > 0 {
                    s_BLTRanks = metamodelica::cons(rank, s_BLTRanks);
                } else {
                    e_BLTRanks = metamodelica::cons(rank, e_BLTRanks);
                }
            }
        }
    }
    outlist = outlist.reverse();
    ranklist = listAppend(
        List::sort(
            s_BLTRanks,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        e_BLTRanks.reverse(),
    );
    Ok((outlist, ranklist))
}

pub(crate) fn sortBlocks(
    mut sortedranklist: &metamodelica::List<i32>,
    mut inlist2: &metamodelica::List<(metamodelica::List<i32>, i32)>,
) -> metamodelica::List<(metamodelica::List<i32>, i32)> {
    let mut outlist: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut e1: i32;
    let mut blocks: metamodelica::List<i32>;
    for mut i in &**sortedranklist {
        for mut j in &**inlist2 {
            (blocks, e1) = j.clone();
            if i.clone() == e1 {
                outlist = metamodelica::cons((blocks, e1), outlist);
            }
        }
    }
    outlist = outlist.reverse();
    outlist
}

pub(crate) fn getBlockVarList(
    mut blocktofind: &metamodelica::List<i32>,
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outstringlist: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut blockFound: bool;
    for mut i in &**inlist1 {
        blockFound = List::setEqualOnTrue(metamodelica::AsArg::as_arg(&i), blocktofind, &fnptr!(intEq, i32, i32))?;
        if blockFound {
            outstringlist = (inlist2).get(count)?;
        }
        count = count + 1;
    }
    Ok(outstringlist)
}

pub(crate) fn getActualBlocks(
    mut searchblock: &metamodelica::List<i32>,
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outlist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &**inlist1 {
        if !((List::intersectionOnTrue(searchblock, metamodelica::AsArg::as_arg(&i), &fnptr!(intEq, i32, i32))?)
            .is_empty())
        {
            outlist = metamodelica::cons(i.clone(), outlist);
        }
    }
    outlist = outlist.reverse();
    Ok(outlist)
}

/* ### End of Block-target Algorithm functions ### */
/*
  finding PredecessorBlocks Algorithm
*/
pub(crate) fn findPredecessorBlocks(
    mut blockinfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
> {
    let mut outblockinfo: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    let mut dependencyequation: metamodelica::List<i32>;
    let mut constantEquations: metamodelica::List<i32>;
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut tmptargetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockitems1: metamodelica::List<i32>;
    let mut foundblockranks: metamodelica::List<i32>;
    let mut count: i32 = 1;
    let mut tmpcount: i32;
    let mut exist: bool;
    let mut targetexist: bool;
    for mut blocks in &**blockinfo {
        (blockitems1, targetblocks, targetblocksvar) = blocks.clone();
        tmpcount = 1;
        targetexist = false;
        for mut tmpblocks in &**blockinfo {
            (_, tmptargetblocks, _) = tmpblocks.clone();
            if !(intEq(count, tmpcount)) {
                if listMember((targetblocks).head().cloned()?, tmptargetblocks) {
                    targetexist = true;
                }
            }
            tmpcount = tmpcount + 1;
        }
        if !(targetexist) {
            (exist, dependencyequation, constantEquations, foundblockranks) =
                findSquareAndNonSquareBlocksHelper1(&targetblocks, &targetblocksvar)?;
            outblockinfo = metamodelica::cons(
                (
                    blockitems1,
                    targetblocks,
                    targetblocksvar,
                    dependencyequation,
                    constantEquations,
                    foundblockranks,
                ),
                outblockinfo,
            );
        }
        count = count + 1;
    }
    outblockinfo = outblockinfo.reverse();
    Ok(outblockinfo)
}

pub(crate) fn findSquareAndNonSquareBlocksHelper1(
    mut inlist1: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: &metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
) -> Result<(
    bool,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut exists: bool = false;
    let mut foundknownblocks: metamodelica::List<i32> = metamodelica::nil();
    let mut constantBlocks: metamodelica::List<i32> = metamodelica::nil();
    let mut blockranks: metamodelica::List<i32> = metamodelica::nil();
    let mut blocksvarlist: metamodelica::List<ArcStr>;
    let mut count: i32 = 1;
    let mut rank: i32;
    let mut targetblocks: metamodelica::List<i32>;
    for mut i in &**inlist2 {
        (blocksvarlist, rank) = i.clone();
        if rank > 0 && count == 1 {
            (targetblocks, _) = (inlist1).get(count)?;
            if listMember(literal!("knowns"), blocksvarlist.clone()) {
                exists = true;
                blockranks = metamodelica::cons(rank, blockranks);
                foundknownblocks =
                    getKnownOrExactEquationBlocksHelper(&blocksvarlist, &targetblocks, literal!("knowns"))?;
            } else if listMember(literal!("constants"), blocksvarlist.clone()) {
                exists = true;
                blockranks = metamodelica::cons(rank, blockranks);
                constantBlocks =
                    getKnownOrExactEquationBlocksHelper(&blocksvarlist, &targetblocks, literal!("constants"))?;
            }
        }
        count = count + 1;
    }
    foundknownblocks = foundknownblocks.reverse();
    blockranks = blockranks.reverse();
    Ok((exists, foundknownblocks, constantBlocks, blockranks))
}

fn getKnownOrExactEquationBlocksHelper(
    mut blocksVarList: &metamodelica::List<ArcStr>,
    mut targetBlocks: &metamodelica::List<i32>,
    mut knownOrConstant: ArcStr,
) -> Result<metamodelica::List<i32>> {
    let mut outBlocks: metamodelica::List<i32> = metamodelica::nil();
    let mut count: i32 = 1;
    for mut j in &**blocksVarList {
        if j.clone() == knownOrConstant.clone() {
            outBlocks = metamodelica::cons((targetBlocks).get(count)?, outBlocks);
            return Ok(outBlocks);
        }
        count = count + 1;
    }
    Ok(outBlocks)
}

/* end of finding PredecessorBlocks Algorithm */
pub(crate) fn getVariablesAfterExtraction(
    mut setc: metamodelica::List<i32>,
    mut sets: metamodelica::List<i32>,
    mut mext: &ExtAdjacencyMatrix,
) -> metamodelica::List<i32> {
    let mut finalvars: metamodelica::List<i32> = metamodelica::nil();
    let mut fulleqs: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    fulleqs = listAppend(setc, sets);
    for mut i in &*fulleqs {
        for mut j in &**mext {
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq) {
                for mut k in &*vars {
                    finalvars = metamodelica::cons(k.clone(), finalvars);
                }
            }
        }
    }
    finalvars = List::unique(&finalvars);
    finalvars
}

fn VerifySetSPrime(
    mut boundaryConditionsVars: &BackendDAE::Variables,
    mut intermediateVars: &BackendDAE::Variables,
    mut knownVars: &BackendDAE::Variables,
    mut extraVarsinSetSPrime: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut boundaryConditionsEquations: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >,
    mut intermediateEquations: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut auxillaryEquations: i32,
    mut numRelatedBoundaryConditions: i32,
    mut stateEstimation: bool,
) -> Result<()> {
    let mut eqSize: i32;
    let mut varSize: i32;
    let mut count: i32;
    let mut extraVarLength: i32;
    let mut condition5: ArcStr;
    let mut msg: ArcStr;
    eqSize = intAdd(
        BackendEquation::equationArraySize(boundaryConditionsEquations.clone())?,
        BackendEquation::equationArraySize(intermediateEquations)?,
    );
    varSize = intAdd(
        ((BackendVariable::varList(boundaryConditionsVars)?).len() as i32),
        ((BackendVariable::varList(intermediateVars)?).len() as i32),
    );
    if !(intEq(eqSize, varSize)) {
        condition5 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Set-S' has "));
            __mm_s.push_str(&*intString(eqSize));
            __mm_s.push_str(&*literal!(" equations and "));
            __mm_s.push_str(&*intString(varSize));
            __mm_s.push_str(&*literal!(" variables"));
            ArcStr::from(__mm_s)
        };
        msg = literal!("Boundary condition(s) ");
        for mut var in &*BackendVariable::varList(boundaryConditionsVars)? {
            msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*msg);
                __mm_s.push_str(&*BackendDump::varStringShort(metamodelica::AsArg::as_arg(&var))?);
                __mm_s.push_str(&*literal!(","));
                ArcStr::from(__mm_s)
            };
        }
        msg = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*msg);
            __mm_s.push_str(&*literal!(" cannot be computed from the variables of interest only. They must be computed also from boundary conditions(s) "));
            ArcStr::from(__mm_s)
        };
        extraVarLength = ((extraVarsinSetSPrime).len() as i32);
        count = 1;
        for mut var in &**extraVarsinSetSPrime {
            if intEq(count, extraVarLength) {
                msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*msg);
                    __mm_s.push_str(&*BackendDump::varStringShort(metamodelica::AsArg::as_arg(&var))?);
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                };
            } else {
                msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*msg);
                    __mm_s.push_str(&*BackendDump::varStringShort(metamodelica::AsArg::as_arg(&var))?);
                    __mm_s.push_str(&*literal!(","));
                    ArcStr::from(__mm_s)
                };
            }
            count = count + 1;
        }
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*msg);
                __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of boundary conditions from the variables of interest only."));
                ArcStr::from(__mm_s)
            }],
        )?;
        if stateEstimation {
            generateCompileTimeHtmlReport(
                shared,
                &(literal!("")),
                &(intString(BackendEquation::equationArraySize(boundaryConditionsEquations)?)),
                &(intString(((BackendVariable::varList(knownVars)?).len() as i32))),
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                &((literal!(""), metamodelica::nil())),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*msg);
                    __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of unmeasured variables of interest from the variables of interest only."));
                    ArcStr::from(__mm_s)
                }),
                false,
                true,
                auxillaryEquations,
                numRelatedBoundaryConditions,
                0,
            )?;
        } else {
            generateCompileTimeHtmlReport(
                shared,
                &(literal!("")),
                &(intString(BackendEquation::equationArraySize(boundaryConditionsEquations)?)),
                &(intString(((BackendVariable::varList(knownVars)?).len() as i32))),
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                &((literal!(""), metamodelica::nil())),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*msg);
                    __mm_s.push_str(&*literal!(" Therefore, the problem is ill-posed regarding the computation of boundary conditions from the variables of interest only."));
                    ArcStr::from(__mm_s)
                }),
                true,
                false,
                auxillaryEquations,
                numRelatedBoundaryConditions,
                0,
            )?;
        }
        return Err("fail");
    }
    Ok(())
}

fn VerifyDataReconciliation(
    mut setc: &metamodelica::List<i32>,
    mut sets: &metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: &metamodelica::List<i32>,
    mut mExt: &ExtAdjacencyMatrix,
    mut solvedvar: &metamodelica::List<(i32, i32)>,
    mut constantvars: &metamodelica::List<i32>,
    mut approximatedEquations: &metamodelica::List<i32>,
    mut allVars: BackendDAE::Variables,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut outsetS_vars: &BackendDAE::Variables,
    mut outsetS_eq: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut mappedSetC: metamodelica::List<i32>,
    mut mappedSetS: metamodelica::List<i32>,
    mut unMeasuredVariablesOfInterest: i32,
) -> Result<()> {
    let mut matchedeq: metamodelica::List<i32>;
    let mut matchedknownssetc: metamodelica::List<i32>;
    let mut matchedunknownssetc: metamodelica::List<i32>;
    let mut matchedknownssets: metamodelica::List<i32>;
    let mut matchedunknownssets: metamodelica::List<i32>;
    let mut tmplist1: metamodelica::List<i32>;
    let mut tmplist2: metamodelica::List<i32>;
    let mut tmplist3: metamodelica::List<i32>;
    let mut tmplist1sets: metamodelica::List<i32>;
    let mut tmplistvar1: metamodelica::List<i32>;
    let mut tmplistvar2: metamodelica::List<i32>;
    let mut tmplistvar3: metamodelica::List<i32>;
    let mut r#str: ArcStr;
    let mut resstr: ArcStr;
    let mut condition1: ArcStr;
    let mut condition2: ArcStr;
    let mut condition3: ArcStr;
    let mut condition4: ArcStr;
    let mut condition5: ArcStr;
    let mut auxilliaryConditions: ArcStr;
    let mut varsToReconcile: ArcStr;
    let mut var: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut rule2: bool = true;
    let mut condition1_eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\n\nAutomatic Verification Steps of DataReconciliation Algorithm"
        ));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    var = List::map1r(
        knowns.clone().reverse(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        allVars.clone(),
    )?;
    BackendDump::dumpVarList(
        &var,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("knownVariables:"));
            __mm_s.push_str(&*dumplistInteger(knowns.clone().reverse())?);
            ArcStr::from(__mm_s)
        }),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("-SET_C:"));
        __mm_s.push_str(&*dumplistInteger(mappedSetC.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("-SET_S:"));
        __mm_s.push_str(&*dumplistInteger(mappedSetS.clone())?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    auxilliaryConditions = intString(((mappedSetC).len() as i32));
    varsToReconcile = intString(((knowns).len() as i32));
    condition1 = literal!("Condition-1 \"SET_C and SET_S must not have no equations in common\"");
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*condition1);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    matchedeq = List::intersectionOnTrue(&mappedSetC, &mappedSetS, &fnptr!(intEq, i32, i32))?;
    if (matchedeq).is_empty() {
        metamodelica::print(literal!("-Passed\n\n"));
    } else {
        metamodelica::print(literal!("-Failed\n"));
        condition1_eqs = List::map1r(matchedeq.clone(), &BackendEquation::get, allEqs)?;
        BackendDump::dumpEquationList(
            &condition1_eqs,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Sets C and S have equations in common"));
                __mm_s.push_str(&*dumplistInteger(matchedeq)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                ": Condition 1-Failed: SET_C and SET_S must not have no equations in common: The data reconciliation problem is ill-posed"
            )],
        )?;
        generateCompileTimeHtmlReport(
            shared,
            &(literal!(
                "Internal Error: Condition 1-Failed: \"SET_C and SET_S must not have no equations in common\": The data reconciliation problem is ill-posed"
            )),
            &auxilliaryConditions,
            &varsToReconcile,
            &((literal!("Sets C and S have equations in common"), condition1_eqs)),
            &((literal!(""), metamodelica::nil())),
            &(literal!("")),
            &((literal!(""), metamodelica::nil())),
            &(literal!("")),
            false,
            false,
            0,
            0,
            unMeasuredVariablesOfInterest,
        )?;
        return Err("fail");
    }
    (matchedknownssetc, matchedunknownssetc) = getVariableOccurence(setc, mExt, knowns.clone());
    (matchedknownssets, matchedunknownssets) = getVariableOccurence(sets, mExt, knowns.clone());
    condition2 = literal!("Condition-2 \"All variables of interest must be involved in SET_C or SET_S\"");
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*condition2);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    (tmplist1, tmplist2, tmplist3) =
        List::intersection1OnTrue(matchedknownssetc, knowns.clone(), &fnptr!(intEq, i32, i32))?;
    if (tmplist3).is_empty() {
        metamodelica::print(literal!("-Passed\n"));
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has all known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1)?);
                ArcStr::from(__mm_s)
            }),
        )?;
    } else if !((tmplist3).is_empty()) {
        (tmplist1sets, tmplist2, _) = List::intersection1OnTrue(tmplist3, matchedknownssets, &fnptr!(intEq, i32, i32))?;
        if !((tmplist2).is_empty()) {
            r#str = dumplistInteger(tmplist2.clone())?;
            metamodelica::print(literal!("-Failed\n"));
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplist2.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("knownVariables not Found:"));
                    __mm_s.push_str(&*dumplistInteger(tmplist2.clone())?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    ": Condition 2-Failed: All variables of interest must be involved in Set-C or Set-S: The data reconciliation problem is ill-posed"
                )],
            )?;
            rule2 = false;
            r#str = dumpToCsv(
                &(literal!("")),
                &(List::map1r(
                    tmplist2,
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?),
            )?;
            System::writeFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*shared.info.fileNamePrefix);
                    __mm_s.push_str(&*literal!("_NonReconcilcedVars.txt"));
                    ArcStr::from(__mm_s)
                },
                r#str,
            )?;
        }
        if rule2 {
            metamodelica::print(literal!("-Passed\n"));
        }
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1sets.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_S has known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1sets)?);
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    condition3 = literal!("Condition-3 \"SET_C equations must be strictly less than Variable of Interest\"");
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*condition3);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if ((setc).len() as i32) < ((knowns).len() as i32) && !((setc).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-SET_C contains:"));
            __mm_s.push_str(&*intString(((setc).len() as i32)));
            __mm_s.push_str(&*literal!(" equations < "));
            __mm_s.push_str(&*intString(((knowns).len() as i32)));
            __mm_s.push_str(&*literal!(" known variables\n\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        condition3 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Set-C has "));
            __mm_s.push_str(&*intString(((setc).len() as i32)));
            __mm_s.push_str(&*literal!(" equations and "));
            __mm_s.push_str(&*intString(((knowns).len() as i32)));
            __mm_s.push_str(&*literal!(" variables to be reconciled"));
            ArcStr::from(__mm_s)
        };
        resstr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Failed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-"));
            __mm_s.push_str(&*condition3);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        };
        metamodelica::print(resstr);
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                ": Condition 3-Failed: The number of auxiliary conditions must be strictly less than the number of variables to be reconciled. The data reconciliation problem is ill-posed"
            )],
        )?;
        if (setc).is_empty() {
            condition3 = literal!(
                "<b>User Error:</b> Condition 7 failed: \"The set of auxiliary conditions is empty.\" The data reconciliation problem is ill-posed"
            );
            generateCompileTimeHtmlReport(
                shared,
                &(literal!("")),
                &auxilliaryConditions,
                &varsToReconcile,
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &condition3,
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                false,
                false,
                0,
                0,
                unMeasuredVariablesOfInterest,
            )?;
        } else {
            generateCompileTimeHtmlReport(
                shared,
                &(literal!(
                    "<b>User Error:</b> Condition 3-Failed: \"The number of auxiliary conditions must be strictly less than the number of variables to be reconciled.\": The data reconciliation problem is ill-posed"
                )),
                &auxilliaryConditions,
                &varsToReconcile,
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &condition3,
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                false,
                false,
                0,
                0,
                unMeasuredVariablesOfInterest,
            )?;
        }
        return Err("fail");
    }
    condition4 = literal!("Condition-4 \"SET_S should contain all intermediate variables involved in SET_C\"");
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*condition4);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    (tmplistvar1, tmplistvar2, tmplistvar3) = List::intersection1OnTrue(
        matchedunknownssetc.clone(),
        matchedunknownssets,
        &fnptr!(intEq, i32, i32),
    )?;
    if (matchedunknownssetc).is_empty() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-SET_C contains No Intermediate Variables\n\n"));
            ArcStr::from(__mm_s)
        });
        return Ok(());
    } else {
        BackendDump::dumpVarList(
            &(List::map1r(
                matchedunknownssetc.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has intermediate variables:"));
                __mm_s.push_str(&*dumplistInteger(matchedunknownssetc)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        if (tmplistvar2).is_empty() {
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplistvar1.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars,
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-SET_S has intermediate variables involved in SET_C:"));
                    __mm_s.push_str(&*dumplistInteger(tmplistvar1)?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            metamodelica::print(literal!("-Passed\n\n"));
        } else {
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplistvar2.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "-SET_S does not have intermediate variables involved in SET_C:"
                    ));
                    __mm_s.push_str(&*dumplistInteger(tmplistvar2.clone())?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    ": Condition 4-Failed: SET_S should contain all intermediate variables involved in SET_C: The data reconciliation problem is ill-posed"
                )],
            )?;
            generateCompileTimeHtmlReport(
                shared,
                &(literal!(
                    "<b>Internal Error:</b> Condition 4-Failed: \"SET_S should contain all intermediate variables involved in SET_C\": The data reconciliation problem is ill-posed"
                )),
                &auxilliaryConditions,
                &varsToReconcile,
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                &((
                    literal!("Set-S does not have intermediate variables involved in Set-C"),
                    List::map1r(
                        tmplistvar2,
                        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                        allVars,
                    )?,
                )),
                &(literal!("")),
                false,
                false,
                0,
                0,
                unMeasuredVariablesOfInterest,
            )?;
            return Err("fail");
        }
    }
    condition5 = literal!("Condition-5 \"SET_S should be square\"");
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*condition5);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if (outsetS_eq).is_empty() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!(
                "-SET_S contains 0 intermediate variables and 0 equations\n\n"
            ));
            ArcStr::from(__mm_s)
        });
        return Ok(());
    } else {
        if BackendEquation::equationArraySize(BackendEquation::listEquation(outsetS_eq)?)?
            == ((BackendVariable::varList(outsetS_vars)?).len() as i32)
        {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Passed"));
                __mm_s.push_str(&*literal!("\n "));
                __mm_s.push_str(&*literal!("Set_S has "));
                __mm_s.push_str(&*intString(((sets).len() as i32)));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(((BackendVariable::varList(outsetS_vars)?).len() as i32)));
                __mm_s.push_str(&*literal!(" variables\n\n"));
                ArcStr::from(__mm_s)
            });
        } else {
            condition5 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Set-S has "));
                __mm_s.push_str(&*intString(BackendEquation::equationArraySize(
                    BackendEquation::listEquation(outsetS_eq)?,
                )?));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(((BackendVariable::varList(outsetS_vars)?).len() as i32)));
                __mm_s.push_str(&*literal!(" variables"));
                ArcStr::from(__mm_s)
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Failed"));
                __mm_s.push_str(&*literal!("\n "));
                __mm_s.push_str(&*condition5);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    ": Condition 5-Failed: Set_S should be square: The data reconciliation problem is ill-posed"
                )],
            )?;
            generateCompileTimeHtmlReport(
                shared,
                &(literal!(
                    "<b>Internal Error:</b> Condition 5-Failed: \"Set_S should be square\": The data reconciliation problem is ill-posed"
                )),
                &auxilliaryConditions,
                &varsToReconcile,
                &((literal!(""), metamodelica::nil())),
                &((literal!(""), metamodelica::nil())),
                &(literal!("")),
                &((literal!(""), metamodelica::nil())),
                &condition5,
                false,
                false,
                0,
                0,
                unMeasuredVariablesOfInterest,
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

fn generateCompileTimeHtmlReport(
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut conditions: &ArcStr,
    mut auxilliaryConditions: &ArcStr,
    mut varsToReconcile: &ArcStr,
    mut condition1: &(ArcStr, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>),
    mut condition2: &(ArcStr, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>),
    mut condition3: &ArcStr,
    mut condition4: &(ArcStr, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>),
    mut condition5: &ArcStr,
    mut boundaryCondition: bool,
    mut stateEstimation: bool,
    mut setC: i32,
    mut numRelatedBoundaryConditions: i32,
    mut unMeasuredVariables: i32,
) -> Result<()> {
    let mut data: ArcStr;
    let mut condition1_msg: ArcStr;
    let mut condition4_msg: ArcStr;
    let mut condition1_eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut condition4_vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if boundaryCondition {
        data = literal!(
            "<html> \n <head> <h1> Boundary Condition Report</h1></head> \n <body> \n <h2> Overview: </h2> \n"
        );
    } else {
        data = literal!(
            "<html> \n <head> <h1> Data Reconciliation Report</h1></head> \n <body> \n <h2> Overview: </h2> \n"
        );
    }
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<table> \n <tr> \n <th align=right> Model file: </th> \n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<td>"));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!(".mo"));
        __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!(" <tr> \n <th align=right> Model name: </th>\n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<td>"));
        __mm_s.push_str(&*shared.info.fileNamePrefix);
        __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<tr> \n <th align=right> Generated: </th>\n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<td>"));
        __mm_s.push_str(&*System::getCurrentTimeStr()?);
        __mm_s.push_str(&*literal!("<b> by OpenModelica "));
        __mm_s.push_str(&*Settings::getVersionNr());
        __mm_s.push_str(&*literal!("</b>"));
        __mm_s.push_str(&*literal!("</td>\n</tr>\n <table>\n"));
        ArcStr::from(__mm_s)
    };
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<h2> Analysis: </h2>\n<table>"));
        ArcStr::from(__mm_s)
    };
    if boundaryCondition {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of boundary conditions: </th> \n <td>"
            ));
            __mm_s.push_str(&*auxilliaryConditions);
            __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
            ArcStr::from(__mm_s)
        };
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of measured variables: </th> \n <td>"
            ));
            __mm_s.push_str(&*varsToReconcile);
            __mm_s.push_str(&*literal!("</td>\n</tr>\n</table>"));
            ArcStr::from(__mm_s)
        };
    } else {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of auxiliary conditions: </th> \n <td>"
            ));
            __mm_s.push_str(&*intString(setC));
            __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
            ArcStr::from(__mm_s)
        };
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of measured variables: </th> \n <td>"
            ));
            __mm_s.push_str(&*varsToReconcile);
            __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
            ArcStr::from(__mm_s)
        };
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of unmeasured variables: </th> \n <td>"
            ));
            __mm_s.push_str(&*intString(unMeasuredVariables));
            __mm_s.push_str(&*literal!("</td>\n</tr>\n"));
            ArcStr::from(__mm_s)
        };
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!(
                "<tr>\n <th align=right> Number of related boundary conditions: </th> \n <td>"
            ));
            __mm_s.push_str(&*intString(numRelatedBoundaryConditions));
            __mm_s.push_str(&*literal!("</td>\n</tr>\n</table>"));
            ArcStr::from(__mm_s)
        };
    }
    if boundaryCondition {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<h3> <a href="));
            __mm_s.push_str(&*shared.info.fileNamePrefix);
            __mm_s.push_str(&*literal!(
                "_BoundaryConditionsEquations.html target=_blank> Boundary conditions </a> </h3>"
            ));
            ArcStr::from(__mm_s)
        };
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<h3> <a href="));
            __mm_s.push_str(&*shared.info.fileNamePrefix);
            __mm_s.push_str(&*literal!(
                "_BoundaryConditionIntermediateEquations.html target=_blank> Intermediate equations </a> </h3>"
            ));
            ArcStr::from(__mm_s)
        };
    } else {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<h3> <a href="));
            __mm_s.push_str(&*shared.info.fileNamePrefix);
            __mm_s.push_str(&*literal!(
                "_IntermediateEquations.html target=_blank> Intermediate equations for measured variables </a> </h3>"
            ));
            ArcStr::from(__mm_s)
        };
        if numRelatedBoundaryConditions > 0 {
            data = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*data);
                __mm_s.push_str(&*literal!("<h3> <a href="));
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!(
                    "_BoundaryConditionsEquations.html target=_blank> Boundary conditions </a> </h3>"
                ));
                ArcStr::from(__mm_s)
            };
            data = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*data);
                __mm_s.push_str(&*literal!("<h3> <a href="));
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_BoundaryConditionIntermediateEquations.html target=_blank> Intermediate equations for unmeasured variables </a> </h3>"));
                ArcStr::from(__mm_s)
            };
        }
    }
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("<h3> Errors: </h3> "));
        __mm_s.push_str(&*literal!("\n <p>"));
        __mm_s.push_str(&*conditions);
        __mm_s.push_str(&*literal!("</p>"));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    (condition1_msg, condition1_eqs) = condition1.clone();
    if !((condition1_eqs).is_empty()) {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<p>"));
            __mm_s.push_str(&*condition1_msg);
            __mm_s.push_str(&*literal!("\n <ol>"));
            ArcStr::from(__mm_s)
        };
        for mut eq in &*condition1_eqs {
            data = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*data);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*literal!("  <li>"));
                __mm_s.push_str(&*BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?);
                __mm_s.push_str(&*literal!(" </li>"));
                ArcStr::from(__mm_s)
            };
        }
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("\n</ol> \n</p>"));
            ArcStr::from(__mm_s)
        };
    }
    if !(stringEmpty(&condition3)) {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<p>"));
            __mm_s.push_str(&*condition3);
            __mm_s.push_str(&*literal!("</p>"));
            ArcStr::from(__mm_s)
        };
    }
    (condition4_msg, condition4_vars) = condition4.clone();
    if !((condition4_vars).is_empty()) {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<p>"));
            __mm_s.push_str(&*condition4_msg);
            __mm_s.push_str(&*literal!("\n <ol>"));
            ArcStr::from(__mm_s)
        };
        for mut var in &*condition4_vars {
            data = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*data);
                __mm_s.push_str(&*literal!("\n <li>"));
                __mm_s.push_str(&*BackendDump::varStringShort(metamodelica::AsArg::as_arg(&var))?);
                __mm_s.push_str(&*literal!("</li>"));
                ArcStr::from(__mm_s)
            };
        }
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("\n</ol>"));
            ArcStr::from(__mm_s)
        };
    }
    if !(stringEmpty(&condition5)) {
        data = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*data);
            __mm_s.push_str(&*literal!("<p>"));
            __mm_s.push_str(&*condition5);
            __mm_s.push_str(&*literal!("</p>"));
            ArcStr::from(__mm_s)
        };
    }
    data = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*data);
        __mm_s.push_str(&*literal!("\n</html>"));
        ArcStr::from(__mm_s)
    };
    if boundaryCondition {
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!("_BoundaryConditions.html"));
                ArcStr::from(__mm_s)
            },
            data,
        )?;
    } else {
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*shared.info.fileNamePrefix);
                __mm_s.push_str(&*literal!(".html"));
                ArcStr::from(__mm_s)
            },
            data,
        )?;
    }
    Ok(())
}

pub(crate) fn getVariableOccurence(
    mut setCOrSetS: &metamodelica::List<i32>,
    mut mext: &ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut knownvariables: metamodelica::List<i32> = metamodelica::nil();
    let mut unknownvariables: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    for mut i in &**setCOrSetS {
        for mut j in &**mext {
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq) {
                for mut var in &*vars {
                    if listMember(var.clone(), knowns.clone()) {
                        knownvariables = metamodelica::cons(var.clone(), knownvariables);
                    } else {
                        unknownvariables = metamodelica::cons(var.clone(), unknownvariables);
                    }
                }
            }
        }
    }
    knownvariables = List::unique(&knownvariables);
    unknownvariables = List::unique(&unknownvariables);
    (knownvariables, unknownvariables)
}

/* function which dumps the variable names to csv file */
pub(crate) fn dumpToCsv(
    mut instring: &ArcStr,
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    for mut i in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&i));
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*ComponentReference::crefStr(&cr)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instring);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

/* function which dumps the variable names to csv file */
pub(crate) fn dumpCorrelationVarsToCsv(
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut r#str: ArcStr = literal!("Sxij,");
    for mut i in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&i));
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*ComponentReference::crefStr(&cr)?);
            __mm_s.push_str(&*literal!(","));
            ArcStr::from(__mm_s)
        };
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

/* function which dumps non reconciledVars failing for condition -2 to a log file*/
pub(crate) fn dumpNonReconciledVars(
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    for mut i in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&i));
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*ComponentReference::crefStr(&cr)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outstring)
}

fn dumpEquationString(mut inEquation: &metamodelica::Ref<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inEquation {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionDump::printExp2Str::<()>(e1.clone(), &(literal!("")), None, None);
            s2 = ExpressionDump::printExp2Str::<()>(e2.clone(), &(literal!("")), None, None);
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionDump::printExp2Str::<()>(e1.clone(), &(literal!("")), None, None);
            s2 = ExpressionDump::printExp2Str::<()>(e2.clone(), &(literal!("")), None, None);
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionDump::printExp2Str::<()>(e1.clone(), &(literal!("")), None, None);
            s2 = ExpressionDump::printExp2Str::<()>(e2.clone(), &(literal!("")), None, None);
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
            s1 = System::stringReplace(s1, literal!("."), literal!("_"))?;
            s1 = System::stringReplace(s1, literal!("$"), literal!(""))?;
            s2 = ExpressionDump::printExp2Str::<()>(e2.clone(), &(literal!("")), None, None);
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. } => {
            let mut res: ArcStr;
            res = BackendDump::whenEquationString(weqn, true)?;
            res
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionDump::printExp2Str::<()>(e.clone(), &(literal!("")), None, None);
            res = stringAppendList(list![s1, literal!("= 0")]);
            res
        },
        Deref @ BackendDAE::Equation::ALGORITHM { alg, source, .. } => {
            let mut res: ArcStr;
            res = DAEDump::dumpAlgorithmsStr(&(list![metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })]))?;
            res
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: Deref @ metamodelica::ListNode::Cons { head: e1, tail: expl }, eqnstrue: Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnstrue }, eqnsfalse, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionDump::printExp2Str::<()>(e1.clone(), &(literal!("")), None, None);
            s2 = stringDelimitList(List::map(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| dumpEquationString(&__a0))?, literal!("\n  "));
            s3 = stringAppendList(list![literal!("if "), s1, literal!(" then\n  "), s2]);
            res = BackendDump::ifequationString(metamodelica::AsArg::as_arg(&expl), metamodelica::AsArg::as_arg(&eqnstrue), eqnsfalse, s3)?;
            res
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { iter, start, stop, body: eqn, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionDump::printExp2Str::<()>(iter.clone(), &(literal!("")), None, None)); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionDump::printExp2Str::<()>(start.clone(), &(literal!("")), None, None)); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionDump::printExp2Str::<()>(stop.clone(), &(literal!("")), None, None)); ArcStr::from(__mm_s) };
            s2 = dumpEquationString(eqn)?;
            res = stringAppendList(list![literal!("for "), s1, literal!(" loop\n    "), s2, literal!("; end for; ")]);
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}
