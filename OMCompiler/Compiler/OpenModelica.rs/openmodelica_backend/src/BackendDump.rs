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
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpHTML;
use crate::GraphvizDump;
use crate::HpcOmTaskGraph;
use crate::Initialization;
use crate::Matching;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::MMath;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all print* functions
//
// These are functions, that print directly to the standard-stream.
//   - printBackendDAE
//   - printEqSystem
//   - printEquation
//   - printEquationArray
//   - printEquationList
//   - printEquations
//   - printClassAttributes
//   - printShared
//   - printStateSets
//   - printVar
//   - printVariables
//   - printVarList
// =============================================================================
pub(crate) fn printBackendDAE(mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    List::map_0(&eqs, &printEqSystem)?;
    metamodelica::print(literal!("\n"));
    printShared(&shared)?;
    Ok(())
}

pub(crate) fn printEqSystem(mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*partitionKindString(inSyst.partitionKind.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpVariables(&inSyst.orderedVars, &(literal!("Variables")))?;
    dumpEquationArray(inSyst.orderedEqs.clone(), &(literal!("Equations")))?;
    dumpEquationArray(inSyst.removedEqs.clone(), &(literal!("Simple Equations")))?;
    dumpStateSets(&inSyst.stateSets, &(literal!("State Sets")))?;
    dumpOption(inSyst.m.clone(), &dumpAdjacencyMatrix)?;
    dumpOption(inSyst.mT.clone(), &dumpAdjacencyMatrixT)?;
    metamodelica::print(literal!("\n"));
    dumpFullMatching(&(inSyst.matching.clone()), Some(inSyst))?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn printEquation(mut inEquation: &metamodelica::Ref<BackendDAE::Equation>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*equationString(inEquation)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn printEquationArray(
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<()> {
    List::fold(
        &(BackendEquation::equationList(eqns)?),
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (i32, i32)| printEquationList2(&__a0, __a1),
        (1, 1),
    )?;
    Ok(())
}

pub(crate) fn printEquationList(mut eqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>) -> Result<()> {
    List::fold(
        eqns,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (i32, i32)| printEquationList2(&__a0, __a1),
        (1, 1),
    )?;
    Ok(())
}

fn printEquationList2(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inInteger: (i32, i32),
) -> Result<(i32, i32)> {
    let mut oInteger: (i32, i32);
    let mut iscalar: i32;
    let mut i: i32;
    let mut size: i32;
    let mut attr: BackendDAE::EquationAttributes;
    (i, iscalar) = inInteger;
    size = BackendEquation::equationSize(inEquation)?;
    attr = BackendEquation::getEquationAttributes(inEquation)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(i));
        __mm_s.push_str(&*literal!("/"));
        __mm_s.push_str(&*intString(iscalar));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(size));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*equationString(inEquation)?);
        __mm_s.push_str(&*literal!("   "));
        __mm_s.push_str(&*equationAttrString(attr)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oInteger = (i + 1, iscalar + size);
    Ok(oInteger)
}

pub(crate) fn equationListString(
    mut inEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut heading: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, _, buffer) = List::fold(inEqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (i32, i32, ArcStr)| equationList2String(&__a0, &__a1), (1, 1, literal!("")))?;
            buffer
        },
        _ => {
            let mut buffer: ArcStr;
            (_, _, buffer) = List::fold(inEqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (i32, i32, ArcStr)| equationList2String(&__a0, &__a1), (1, 1, literal!("")))?;
            buffer = { let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer); ArcStr::from(__mm_s) };
            buffer
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn equationList2String(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: &(i32, i32, ArcStr),
) -> Result<(i32, i32, ArcStr)> {
    let mut outTuple: (i32, i32, ArcStr);
    let mut iscalar: i32;
    let mut i: i32;
    let mut size: i32;
    let mut buffer: ArcStr;
    (i, iscalar, buffer) = inTuple.clone();
    size = BackendEquation::equationSize(inEquation)?;
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*intString(i));
        __mm_s.push_str(&*literal!("/"));
        __mm_s.push_str(&*intString(iscalar));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(size));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*equationString(inEquation)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    outTuple = (i + 1, iscalar + size, buffer);
    Ok(outTuple)
}

pub(crate) fn printEquations(
    mut inIntegerLst: &metamodelica::List<i32>,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: n, tail: rest } => {
            printEquations(rest, syst)?;
            printEquationNo(n.clone(), syst)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEquationNo(mut inInteger: i32, mut syst: &metamodelica::Ref<BackendDAE::EqSystem>) -> Result<()> {
    let () = (match &**syst {
        BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
            let mut eqno = inInteger;
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            eq = BackendEquation::get(eqns.clone(), eqno)?;
            printEquation(&eq)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn printClassAttributes(mut optimicaFun: &metamodelica::Ref<DAE::ClassAttributes>) -> Result<()> {
    let mut e1: Option<metamodelica::Ref<DAE::Exp>>;
    let mut e2: Option<metamodelica::Ref<DAE::Exp>>;
    let __arc2 = &(*optimicaFun);
    let DAE::OPTIMIZATION_ATTRS {
        objetiveE: __pa0,
        objectiveIntegrandE: __pa1,
        ..
    } = &**__arc2;
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Mayer"));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(ExpressionDump::printOptExpStr(e1)?);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Lagrange"));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(ExpressionDump::printOptExpStr(e2)?);
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn printShared(mut inShared: &metamodelica::Ref<BackendDAE::Shared>) -> Result<()> {
    metamodelica::print(literal!("\nBackendDAEType: "));
    printBackendDAEType(inShared.backendDAEType.clone())?;
    metamodelica::print(literal!("\n\n"));
    dumpVariables(
        &inShared.globalKnownVars,
        &(literal!("Known variables only depending on parameters and constants - globalKnownVars")),
    )?;
    dumpVariables(
        &inShared.localKnownVars,
        &(literal!("Known variables only depending on states and inputs - localKnownVars")),
    )?;
    dumpVariables(&inShared.externalObjects, &(literal!("External Objects")))?;
    dumpExternalObjectClasses(&inShared.extObjClasses, &(literal!("Classes of External Objects")))?;
    dumpVariables(&inShared.aliasVars, &(literal!("Alias Variables")))?;
    dumpEquationArray(inShared.removedEqs.clone(), &(literal!("Simple Shared Equations")))?;
    dumpEquationArray(inShared.initialEqs.clone(), &(literal!("Initial Equations")))?;
    dumpZeroCrossingList(
        &(ZeroCrossings::toList(&inShared.eventInfo.zeroCrossings)),
        &(literal!("Zero Crossings")),
    )?;
    dumpZeroCrossingList(
        &(ZeroCrossings::toList(&inShared.eventInfo.relations)),
        &(literal!("Relations")),
    )?;
    if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
        dumpZeroCrossingList(
            &(ZeroCrossings::toList(&inShared.eventInfo.samples)),
            &(literal!("Samples")),
        )?;
    } else {
        dumpTimeEvents(&inShared.eventInfo.timeEvents, &(literal!("Time Events")))?;
    }
    dumpConstraintList(&inShared.constraints, &(literal!("Constraints")))?;
    dumpBasePartitions(
        inShared.partitionsInfo.basePartitions.clone(),
        &(literal!("Base partitions")),
    )?;
    dumpSubPartitions(
        inShared.partitionsInfo.subPartitions.clone(),
        &(literal!("Sub partitions")),
    )?;
    if Flags::isSet(Flags::DUMP_FUNCTIONS.clone())? {
        DAEDump::dumpFunctionTree(&inShared.functionTree, &(literal!("Functions")))?;
    }
    Ok(())
}

pub(crate) fn printBasePartitions(mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>) -> Result<()> {
    let mut clkExpStr: ArcStr;
    let mut nSubClocksStr: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(basePartitions.clone()) {
        clkExpStr = Tpl::tplString2(
            (std::sync::Arc::new(
                move |__a0: Tpl::Text, __a1: metamodelica::Ref<DAE::ClockKind>, __a2: ArcStr| {
                    ExpressionDumpTpl::dumpClockKind(__a0, &__a1, &__a2)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<DAE::ClockKind>, ArcStr) -> Result<Tpl::Text>
                        + 'static,
                >),
            ({
                let __elt = (*metamodelica::index_checked(&basePartitions.borrow(), i)?)
                    .clock
                    .clone();
                __elt
            }),
            literal!(""),
        )?;
        nSubClocksStr = intString(
            ({
                let __elt = (*metamodelica::index_checked(&basePartitions.borrow(), i)?)
                    .nSubClocks
                    .clone();
                __elt
            }),
        );
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*clkExpStr);
            __mm_s.push_str(&*literal!("["));
            __mm_s.push_str(&*nSubClocksStr);
            __mm_s.push_str(&*literal!("]"));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn printSubPartitions(mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>) -> Result<()> {
    let mut subClockStr: ArcStr;
    let mut eventStr: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(subPartitions.clone()) {
        subClockStr = subClockString(
            &({
                let __elt = (*metamodelica::index_checked(&subPartitions.borrow(), i)?)
                    .clock
                    .clone();
                __elt
            }),
        );
        eventStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("event("));
            __mm_s.push_str(&*boolString(
                ({
                    let __elt = (*metamodelica::index_checked(&subPartitions.borrow(), i)?)
                        .holdEvents
                        .clone();
                    __elt
                }),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*subClockStr);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*eventStr);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn subClockString(mut subClock: &BackendDAE::SubClock) -> ArcStr {
    let mut subClockString: ArcStr;
    subClockString = (match subClock.clone() {
        BackendDAE::SubClock::INFERED_SUBCLOCK { .. } => {
            literal!("INFERED_SUBCLOCK")
        }
        BackendDAE::SubClock::SUBCLOCK { factor: _, .. } => {
            let mut factorStr: ArcStr;
            let mut shiftStr: ArcStr;
            let mut solverStr: ArcStr;
            factorStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("factor("));
                __mm_s.push_str(&*MMath::rationalString(
                    var_field!(subClock.factor, BackendDAE::SubClock::SUBCLOCK).clone(),
                ));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            shiftStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("shift("));
                __mm_s.push_str(&*MMath::rationalString(
                    var_field!(subClock.shift, BackendDAE::SubClock::SUBCLOCK).clone(),
                ));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            solverStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("solver("));
                __mm_s.push_str(&*optionString(
                    var_field!(subClock.solver, BackendDAE::SubClock::SUBCLOCK).clone(),
                ));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            if ((solverStr).len() as i32) > 8 {
                subClockString = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*factorStr);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*shiftStr);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*solverStr);
                    ArcStr::from(__mm_s)
                };
            } else {
                subClockString = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*factorStr);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*shiftStr);
                    __mm_s.push_str(&*literal!(" "));
                    ArcStr::from(__mm_s)
                };
            }
            subClockString
        }
    });
    subClockString
}

pub(crate) fn optionString(mut option: Option<ArcStr>) -> ArcStr {
    let mut optionString: ArcStr;
    optionString = (match option {
        Some(mut s) => s,
        _ => {
            literal!("")
        }
    });
    optionString
}

pub(crate) fn printBackendDAEType(mut btp: BackendDAE::BackendDAEType) -> Result<()> {
    metamodelica::print(printBackendDAEType2String(btp)?);
    Ok(())
}

pub(crate) fn printBackendDAEType2String(mut btp: BackendDAE::BackendDAEType) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match btp {
        BackendDAE::BackendDAEType::SIMULATION { .. } => literal!("simulation"),
        BackendDAE::BackendDAEType::JACOBIAN { .. } => literal!("jacobian"),
        BackendDAE::BackendDAEType::ALGEQSYSTEM { .. } => literal!("algebraic loop"),
        BackendDAE::BackendDAEType::ARRAYSYSTEM { .. } => literal!("multidim equation arrays"),
        BackendDAE::BackendDAEType::PARAMETERSYSTEM { .. } => literal!("parameter system"),
        BackendDAE::BackendDAEType::INITIALSYSTEM { .. } => literal!("initialization"),
        BackendDAE::BackendDAEType::INLINESYSTEM { .. } => literal!("inline system"),
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub(crate) fn printStateSets(mut stateSets: &metamodelica::List<BackendDAE::StateSet>) -> Result<()> {
    List::map_0(stateSets, &move |__a0: BackendDAE::StateSet| printStateSet(&__a0))?;
    Ok(())
}

fn printStateSet(mut inStateSet: &BackendDAE::StateSet) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("StateSet \""));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(ComponentReferenceBasics::crefFirstCref(inStateSet.crA.clone())?),
        )?);
        __mm_s.push_str(&*literal!("\" (rang "));
        __mm_s.push_str(&*intString(inStateSet.rang.clone()));
        __mm_s.push_str(&*literal!(")\n"));
        ArcStr::from(__mm_s)
    });
    dumpVarList(&inStateSet.statescandidates, &(literal!("state candidates")))?;
    dumpEquationList(&inStateSet.eqns, &(literal!("eqns")))?;
    dumpVarList(&inStateSet.ovars, &(literal!("ovars")))?;
    dumpEquationList(&inStateSet.oeqns, &(literal!("oeqns")))?;
    dumpVarList(&inStateSet.varA, &(literal!("varA")))?;
    dumpVarList(&inStateSet.varJ, &(literal!("varJ")))?;
    Ok(())
}

pub(crate) fn printVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*varString(inVar)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn printVariables(mut vars: &BackendDAE::Variables) -> Result<()> {
    List::fold(
        &(BackendVariable::varList(vars)?),
        &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: i32| printVars1(&__a0, __a1),
        1,
    )?;
    Ok(())
}

pub(crate) fn printVarList(mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<()> {
    List::fold(
        vars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: i32| printVars1(&__a0, __a1),
        1,
    )?;
    Ok(())
}

fn printVars1(mut inVar: &metamodelica::Ref<BackendDAE::Var>, mut inVarNo: i32) -> Result<i32> {
    let mut outVarNo: i32;
    metamodelica::print(intString(inVarNo));
    metamodelica::print(literal!(": "));
    printVar(inVar)?;
    outVarNo = inVarNo + 1;
    Ok(outVarNo)
}

pub(crate) fn varListString(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut heading: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| var1String(&__a0, &__a1), (1, literal!("")))?;
            buffer
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| var1String(&__a0, &__a1), (1, literal!("")))?;
            buffer = { let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer); ArcStr::from(__mm_s) };
            buffer
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn var1String(mut inVar: &metamodelica::Ref<BackendDAE::Var>, mut inTpl: &(i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl.clone();
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*intString(varNo));
        __mm_s.push_str(&*literal!(": "));
        ArcStr::from(__mm_s)
    };
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*varString(inVar)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

pub(crate) fn varListStringShort(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut heading: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| varNameString(&__a0, &__a1), (1, literal!("")))?;
            buffer
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| varNameString(&__a0, &__a1), (1, literal!("")))?;
            buffer = { let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer); ArcStr::from(__mm_s) };
            buffer
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn varNameString(mut inVar: &metamodelica::Ref<BackendDAE::Var>, mut inTpl: &(i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl.clone();
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*intString(varNo));
        __mm_s.push_str(&*literal!(": "));
        ArcStr::from(__mm_s)
    };
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inVar.varName)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

pub(crate) fn varListStringIndented(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut heading: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| var1StringIndented(&__a0, &__a1), (1, literal!("")))?;
            buffer
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: (i32, ArcStr)| var1StringIndented(&__a0, &__a1), (1, literal!("")))?;
            buffer = { let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer); ArcStr::from(__mm_s) };
            buffer
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn var1StringIndented(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(i32, ArcStr),
) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl.clone();
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*literal!("   "));
        __mm_s.push_str(&*intString(varNo));
        __mm_s.push_str(&*literal!(": "));
        ArcStr::from(__mm_s)
    };
    buffer = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*buffer);
        __mm_s.push_str(&*varString(inVar)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

fn printExternalObjectClasses(mut cls: &metamodelica::List<BackendDAE::ExternalObjectClass>) -> Result<()> {
    let () = (::match_deref::match_deref! { match cls {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: BackendDAE::ExternalObjectClass { path, source }, tail: _ } => {
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut paths_lst: metamodelica::List<ArcStr>;
            let mut path_str: ArcStr;
            metamodelica::print(literal!("class "));
            metamodelica::print(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            metamodelica::print(literal!("\n  extends ExternalObject;"));
            metamodelica::print(literal!("\n origin: "));
            paths = ElementSource::getElementSourceTypes(metamodelica::AsArg::as_arg(&source));
            paths_lst = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (paths).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            path_str = stringDelimitList(paths_lst, literal!(", "));
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*path_str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            metamodelica::print(literal!("end "));
            metamodelica::print(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printSparsityPatternCrefs(
    mut inPattern: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
) -> Result<()> {
    for mut e in &**inPattern {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                &(Util::tuple21(e.clone())),
            )?);
            __mm_s.push_str(&*literal!(" affects the following ("));
            __mm_s.push_str(&*intString(((Util::tuple22(e.clone())).len() as i32)));
            __mm_s.push_str(&*literal!(") outputs\n  "));
            ArcStr::from(__mm_s)
        });
        ComponentReference::printComponentRefList(Util::tuple22(e.clone()))?;
    }
    Ok(())
}

// =============================================================================
// section for all graphviz* functions
//
// =============================================================================
pub(crate) fn graphvizBackendDAE(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inFileNameSuffix: ArcStr,
) -> Result<()> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
    dae = setAdjacencyMatrix(inBackendDAE)?;
    Tpl::tplNoret2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<BackendDAE::BackendDAE>, __a2: ArcStr| {
                GraphvizDump::dumpBackendDAE(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text>
                    + 'static,
            >),
        dae,
        inFileNameSuffix,
    )?;
    Ok(())
}

pub(crate) fn graphvizAdjacencyMatrix(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inFileNameSuffix: ArcStr,
) -> Result<()> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
    dae = setAdjacencyMatrix(inBackendDAE)?;
    Tpl::tplNoret2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<BackendDAE::BackendDAE>, __a2: ArcStr| {
                GraphvizDump::dumpAdjacencyMatrix(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text>
                    + 'static,
            >),
        dae,
        inFileNameSuffix,
    )?;
    Ok(())
}

fn setAdjacencyMatrix(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqSystems = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    eqSystems = List::map1(
        eqSystems,
        &setAdjacencyMatrix1,
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqSystems,
        shared: shared,
    });
    Ok(outBackendDAE)
}

fn setAdjacencyMatrix1(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut isInitial: bool,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    (outEqSystem, _, _) = BackendDAEUtil::getAdjacencyMatrix(
        inEqSystem,
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        isInitial,
    )?;
    Ok(outEqSystem)
}

// =============================================================================
// section for all dump* functions
//
// These are functions, that print directly to the standard-stream and separates
// there output (e.g. with some kind of headings).
//   - dumpBackendDAE
//   - dumpBackendDAEEqnList
//   - dumpBackendDAEVarList
//   - dumpComponent
//   - dumpComponents
//   - dumpComponentsAdvanced
//   - dumpEqnsSolved
//   - dumpEqSystem
//   - dumpEqSystems
//   - dumpEquationArray
//   - dumpEquationList
//   - dumpHashSet
//   - dumpSparsityPattern
//   - dumpTearing
//   - dumpVariables
//   - dumpVarList
// =============================================================================
pub(crate) const BORDER: &'static str = "########################################";

pub(crate) const UNDERLINE: &'static str = "========================================";

pub(crate) fn dumpDAE(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    dumpBackendDAE(&inDAE, &(literal!("dumpDAE")))?;
    Ok(outDAE)
}

pub(crate) fn dumpBackendDAE(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    printBackendDAE(inBackendDAE)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpEqSystem(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    printEqSystem(inEqSystem)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpEqSystemShort(
    mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*partitionKindString(inEqSystem.partitionKind.clone())?);
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpVariables(&inEqSystem.orderedVars, &(literal!("Variables")))?;
    dumpEquationArray(inEqSystem.orderedEqs.clone(), &(literal!("Equations")))?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpEqSystems(
    mut inEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((inEqSystems).len() as i32)));
        __mm_s.push_str(&*literal!(" partitions)\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    List::map_0(inEqSystems, &printEqSystem)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpBasePartitions(
    mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>,
    mut heading: &ArcStr,
) -> Result<()> {
    if metamodelica::arrayLength(basePartitions.clone()) > 0 {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(metamodelica::arrayLength(basePartitions.clone())));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printBasePartitions(basePartitions.clone())?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn dumpSubPartitions(
    mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>,
    mut heading: &ArcStr,
) -> Result<()> {
    if metamodelica::arrayLength(subPartitions.clone()) > 0 {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(metamodelica::arrayLength(subPartitions.clone())));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printSubPartitions(subPartitions.clone())?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub fn dumpVariables(mut inVars: &BackendDAE::Variables, mut heading: &ArcStr) -> Result<()> {
    if BackendVariable::varsSize(inVars) > 0 {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendVariable::varsSize(inVars)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printVariables(inVars)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub fn dumpVarList(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((inVars).len() as i32)));
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    printVarList(inVars)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub fn dumpEquationArray(
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut heading: &ArcStr,
) -> Result<()> {
    if BackendEquation::getNumberOfEquations(inEqns.clone()) + BackendEquation::equationArraySize(inEqns.clone())? > 0 {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(inEqns.clone())));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(BackendEquation::equationArraySize(inEqns.clone())?));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printEquationArray(inEqns)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub fn dumpEquationList(
    mut inEqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut heading: &ArcStr,
) -> Result<()> {
    if !((inEqns).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(((inEqns).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printEquationList(inEqns)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn dumpExternalObjectClasses(
    mut inEOC: &metamodelica::List<BackendDAE::ExternalObjectClass>,
    mut heading: &ArcStr,
) -> Result<()> {
    if !((inEOC).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(((inEOC).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printExternalObjectClasses(inEOC)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn dumpStateSets(
    mut stateSets: &metamodelica::List<BackendDAE::StateSet>,
    mut heading: &ArcStr,
) -> Result<()> {
    if !((stateSets).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        printStateSets(stateSets)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn dumpZeroCrossingList(
    mut inZeroCrossingList: &metamodelica::List<BackendDAE::ZeroCrossing>,
    mut heading: &ArcStr,
) -> Result<()> {
    let mut zeroCrossing: BackendDAE::ZeroCrossing = <BackendDAE::ZeroCrossing as ::std::default::Default>::default();
    if !((inZeroCrossingList).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(((inZeroCrossingList).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        for mut zeroCrossing in &**inZeroCrossingList {
            let mut zeroCrossing = zeroCrossing.clone();
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*zeroCrossingString(&zeroCrossing)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn dumpTimeEvents(
    mut inTimeEvents: &metamodelica::List<BackendDAE::TimeEvent>,
    mut heading: &ArcStr,
) -> Result<()> {
    let mut timeEvent: BackendDAE::TimeEvent = BackendDAE::TimeEvent::SIMPLE_TIME_EVENT;
    if !((inTimeEvents).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(((inTimeEvents).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        for mut timeEvent in &**inTimeEvents {
            let mut timeEvent = timeEvent.clone();
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*timeEventString(&timeEvent)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn dumpConstraintList(
    mut inConstraintArray: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut heading: &ArcStr,
) -> Result<()> {
    if !((inConstraintArray).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*heading);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(((inConstraintArray).len() as i32)));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        dumpConstraints(inConstraintArray, 0)?;
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

pub(crate) fn dumpHashSet(
    mut hashSet: &(
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
    mut heading: &ArcStr,
) -> Result<()> {
    let mut size: i32;
    size = BaseHashSet::currentSize(hashSet);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(size));
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    BaseHashSet::printHashSet(hashSet)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpSparsityPattern(
    mut inPattern: &(
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        ),
        i32,
    ),
    mut heading: &ArcStr,
) -> Result<()> {
    let mut pattern: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut patternT: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut diffVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut diffedVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut nnz: i32;
    let (__pa0, __pa1, (__pa2, __pa3), __pa4) = inPattern;
    pattern = metamodelica::Own::own(__pa0);
    patternT = metamodelica::Own::own(__pa1);
    diffVars = metamodelica::Own::own(__pa2);
    diffedVars = metamodelica::Own::own(__pa3);
    nnz = metamodelica::Own::own(__pa4);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Number of non zero elements: "));
        __mm_s.push_str(&*intString(nnz));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Independents [or inputs] ("));
        __mm_s.push_str(&*intString(((diffVars).len() as i32)));
        __mm_s.push_str(&*literal!(")\n"));
        ArcStr::from(__mm_s)
    });
    ComponentReference::printComponentRefList(diffVars)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Dependents [or outputs] ("));
        __mm_s.push_str(&*intString(((diffedVars).len() as i32)));
        __mm_s.push_str(&*literal!(")\n"));
        ArcStr::from(__mm_s)
    });
    ComponentReference::printComponentRefList(diffedVars)?;
    printSparsityPatternCrefs(&pattern)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("Transposed pattern"));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    printSparsityPatternCrefs(&patternT)?;
    Ok(())
}

pub(crate) fn dumpSparseColoring(
    mut inColoring: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    mut heading: &ArcStr,
) -> Result<()> {
    let mut i: i32 = 0;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Number of colors: "));
        __mm_s.push_str(&*intString(((inColoring).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    for mut crList in &**inColoring {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("The following ("));
            __mm_s.push_str(&*intString(((crList).len() as i32)));
            __mm_s.push_str(&*literal!(") independents belong to one color\n"));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!(": "));
            ArcStr::from(__mm_s)
        });
        ComponentReference::printComponentRefList(crList.clone())?;
        i = i + 1;
    }
    Ok(())
}

pub(crate) fn dumpTearing(
    mut inResEqn: &metamodelica::List<metamodelica::List<i32>>,
    mut inTearVar: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (inResEqn, inTearVar) {
        (Deref @ metamodelica::ListNode::Cons { head: residualeqns, tail: r }, Deref @ metamodelica::ListNode::Cons { head: tearingvars, tail: t }) => {
            let mut str_r: metamodelica::List<ArcStr>;
            let mut str_t: metamodelica::List<ArcStr>;
            let mut str_r_f: ArcStr;
            let mut str_r_1: ArcStr;
            let mut str_t_f: ArcStr;
            let mut str_t_1: ArcStr;
            let mut r#str: ArcStr;
            let mut sr: ArcStr;
            let mut st: ArcStr;
            str_r = List::map(residualeqns.clone(), &fnptr!(intString, i32))?;
            str_r_f = stringDelimitList(str_r, literal!(", "));
            str_r_1 = stringAppend(str_r_f, literal!("\n"));
            sr = stringAppend(literal!("ResidualEqns: "), str_r_1);
            str_t = List::map(tearingvars.clone(), &fnptr!(intString, i32))?;
            str_t_f = stringDelimitList(str_t, literal!(", "));
            str_t_1 = stringAppend(str_t_f, literal!("\n"));
            st = stringAppend(literal!("TearingVars: "), str_t_1);
            r#str = stringAppend(sr, st);
            metamodelica::print(r#str);
            metamodelica::print(literal!("\n"));
            dumpTearing(r, t)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn dumpBackendDAEEqnList(
    mut inBackendDAEEqnList: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut header: &ArcStr,
    mut printExpTree: bool,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*header);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpBackendDAEEqnList2(inBackendDAEEqnList, printExpTree)?;
    metamodelica::print(literal!("===================\n"));
    Ok(())
}

fn dumpBackendDAEEqnList2(
    mut inBackendDAEEqnList: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut printExpTree: bool,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inBackendDAEEqnList;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = literal!("EQUATION: ");
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    metamodelica::print(r#str.clone());
                    r#str = literal!("LHS:\n");
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = literal!("COMPLEX_EQUATION: ");
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    metamodelica::print(r#str.clone());
                    r#str = literal!("LHS:\n");
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    metamodelica::print(literal!("SOLVED_EQUATION: "));
                    r#str = ExpressionBasics::printExpStr(e.clone())?;
                    metamodelica::print(r#str.clone());
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    r#str = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    metamodelica::print(literal!("\n"));
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = literal!("RESIDUAL_EQUATION: ");
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    metamodelica::print(r#str.clone());
                    r#str = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    metamodelica::print(literal!("ARRAY_EQUATION: "));
                    r#str = ExpressionBasics::printExpStr(e1.clone())?;
                    metamodelica::print(r#str.clone());
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    r#str = ExpressionDump::dumpExpStr(e1.clone(), 0)?;
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    metamodelica::print(literal!("\n"));
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { alg, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    metamodelica::print(literal!("ALGORITHM: "));
                    dumpAlgorithms(&(list![alg.clone()]), 0)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: _ } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut r#str: ArcStr;
                    metamodelica::print(literal!("WHEN_EQUATION: "));
                    r#str = whenEquationString(metamodelica::AsArg::as_arg(&weqn), true)?;
                    metamodelica::print(r#str.clone());
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) };
                    e = weqn.condition.clone();
                    r#str = ExpressionDump::dumpExpStr(e.clone(), 0)?;
                    r#str = if (printExpTree) {r#str.clone()} else {literal!("")};
                    metamodelica::print(r#str.clone());
                    metamodelica::print(literal!("\n"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: res } => {
                    metamodelica::print(literal!("SKIPED EQUATION\n"));
                    dumpBackendDAEEqnList2(metamodelica::AsArg::as_arg(&res), printExpTree)?;
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

pub(crate) fn dumpBackendDAEVarList(
    mut inBackendDAEVarList: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut header: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*header);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    printVarList(inBackendDAEVarList)?;
    metamodelica::print(literal!("===================\n"));
    Ok(())
}

pub(crate) fn dumpEqnsSolved(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut heading: &ArcStr,
) -> Result<()> {
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    let __arc1 = &(*inBackendDAE);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    eqs = metamodelica::Own::own(__pa0);
    List::map_0(&eqs, &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| {
        dumpEqnsSolved1(&__a0)
    })?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn dumpEqnsSolved1(mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inEqSystem {
        Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. } => {
            dumpEqnsSolved2(metamodelica::AsArg::as_arg(&comps), eqns.clone(), vars);
            ()
        },
        _ => {
            metamodelica::print(literal!("No Matching\n"));
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqnsSolved2(
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &BackendDAE::Variables,
) -> () {
    let () = 'mc: {
        let __mc_input = &**inComps;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, var: v }, tail: rest } => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SingleEquation: ")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    var = BackendVariable::getVarAt(vars, v.clone())?;
                    printVarList(&(list![var.clone()]))?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, vars: vlst, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac }, jacType, .. }, tail: rest } => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equationsystem ")); __mm_s.push_str(&*jacobianTypeStr(jacType.clone())); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) });
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Jac:\n")); __mm_s.push_str(&*dumpJacobianStr(jac.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    metamodelica::print(literal!("ArrayEquation:\n"));
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    metamodelica::print(literal!("IfEquation:\n"));
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    metamodelica::print(literal!("Algorithm:\n"));
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    metamodelica::print(literal!("ComplexEquation:\n"));
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    metamodelica::print(literal!("WhenEquation:\n"));
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(&(list![eqn.clone()]))?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, casualTearingSet: None, linear: b, .. }, tail: rest } => {
                    let mut vlst1: metamodelica::List<i32>;
                    let mut elst1: metamodelica::List<i32>;
                    let mut vlst1Lst: metamodelica::List<metamodelica::List<i32>>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut s: ArcStr;
                    s = if (b.clone()) {literal!("linear")} else {literal!("nonlinear")};
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("torn ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) });
                    (elst1, vlst1Lst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal vars (")); __mm_s.push_str(&*intString(((varlst).len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    printVarList(&varlst)?;
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual vars (")); __mm_s.push_str(&*intString(((varlst).len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    printVarList(&varlst)?;
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal equations (")); __mm_s.push_str(&*intString(((eqnlst).len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    printEquationList(&eqnlst)?;
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual equations (")); __mm_s.push_str(&*intString(((eqnlst).len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: vlst2, residualequations: elst2, innerEquations: innerEquations2, .. }), linear: b, .. }, tail: rest } => {
                    let mut vlst1: metamodelica::List<i32>;
                    let mut elst1: metamodelica::List<i32>;
                    let mut vlst1Lst: metamodelica::List<metamodelica::List<i32>>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut s: ArcStr;
                    s = if (b.clone()) {literal!("linear")} else {literal!("nonlinear")};
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Strict torn ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) });
                    (elst1, vlst1Lst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    varlst = List::map1r(vlst.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    metamodelica::print(literal!("\n"));
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Casual torn ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) });
                    (elst1, vlst1Lst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations2), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    varlst = List::map1r(vlst2.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), vars.clone())?;
                    printVarList(&varlst)?;
                    metamodelica::print(literal!("\n"));
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    eqnlst = BackendEquation::getList(elst2.clone(), eqns.clone())?;
                    printEquationList(&eqnlst)?;
                    metamodelica::print(literal!("\n"));
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("BackendDump.dumpEqnsSolved2 failed!"))?;
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
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
                    dumpEqnsSolved2(metamodelica::AsArg::as_arg(&rest), eqns.clone(), vars);
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

pub(crate) fn dumpLoops(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut isyst: i32 = 1;
    let mut firstComp: bool = true;
    for mut syst in &*inDAE.eqs.clone() {
        firstComp = true;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        vars = metamodelica::Own::own(__pa0);
        eqns = metamodelica::Own::own(__pa1);
        comps = metamodelica::Own::own(__pa2);
        for mut comp in &*comps {
            if BackendEquation::isEquationsSystem(metamodelica::AsArg::as_arg(&comp))
                || BackendEquation::isTornSystem(metamodelica::AsArg::as_arg(&comp))
            {
                if firstComp {
                    firstComp = false;
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("\nsystem "));
                        __mm_s.push_str(&*intString(isyst));
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n dumpLoops: SORTED COMPONENT \n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
                dumpEqnsSolved2(&(list![comp.clone()]), eqns.clone(), &vars);
                if Flags::isSet(Flags::DUMP_LOOPS_VERBOSE.clone())? {
                    printComponentAdjacencyMatrixEnhanced(
                        metamodelica::AsArg::as_arg(&comp),
                        eqns.clone(),
                        vars.clone(),
                        &outDAE.shared,
                    )?;
                }
            }
        }
        isyst = isyst + 1;
    }
    Ok(outDAE)
}

pub(crate) fn printComponentAdjacencyMatrixEnhanced(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: BackendDAE::Variables,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<()> {
    let mut compEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut compVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut compEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut compVars: BackendDAE::Variables;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    let mut mT: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    (compVarLst, _, compEqnLst, _) = BackendDAEUtil::getStrongComponentVarsAndEquations(comp, vars, eqns)?;
    compEqns = BackendEquation::listEquation(&compEqnLst)?;
    compVars = BackendVariable::listVar(compVarLst)?;
    syst = BackendDAEUtil::createEqSystem(
        compVars.clone(),
        compEqns.clone(),
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (m, mT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(&syst, shared, false)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!(
            "\n dumpLoopsVerbose: UNSORTED COMPONENT WITH ENHANCED ADJACENCY MATRIX \n"
        ));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    dumpVariables(&compVars, &(literal!("component variables")))?;
    dumpEquationArray(compEqns, &(literal!("component equations")))?;
    dumpAdjacencyMatrixEnhanced(m.clone())?;
    metamodelica::print(literal!("\n\n"));
    dumpAdjacencyMatrixTEnhanced(mT.clone())?;
    Ok(())
}

pub(crate) fn dumpComponentsAdvanced(
    mut l: &metamodelica::List<metamodelica::List<i32>>,
    mut v2: metamodelica::Array<i32>,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    metamodelica::print(literal!("Blocks\n"));
    metamodelica::print(literal!("=======\n"));
    vars = BackendVariable::daeVars(syst);
    dumpComponentsAdvanced2(l, 1, v2.clone(), &vars)?;
    Ok(())
}

fn dumpComponentsAdvanced2(
    mut inIntegerLstLst: &metamodelica::List<metamodelica::List<i32>>,
    mut inInteger: i32,
    mut v2: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: lst } => {
            let mut i = inInteger;
            let mut i_1: i32;
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            metamodelica::print(literal!("{"));
            ls = List::map(l.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            metamodelica::print(s);
            metamodelica::print(literal!("} "));
            dumpComponentsAdvanced3(metamodelica::AsArg::as_arg(&l), v2.clone(), vars)?;
            metamodelica::print(literal!("\n"));
            i_1 = i + 1;
            dumpComponentsAdvanced2(lst, i_1, v2.clone(), vars)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpComponentsAdvanced3(
    mut inIntegerLst: &metamodelica::List<i32>,
    mut v2: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut v: i32;
            let mut s: ArcStr;
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut b: bool;
            v = ({let __elt = (*metamodelica::index_checked(&v2.borrow(), i.clone())?).clone(); __elt});
            var = BackendVariable::getVarAt(vars, v)?;
            c = BackendVariable::varCref(&var);
            b = BackendVariable::isStateVar(&var);
            s = if (b) {literal!("der(")} else {literal!("")};
            metamodelica::print(s);
            s = ComponentReferenceBasics::printComponentRefStr(&c)?;
            metamodelica::print(s);
            s = if (b) {literal!(") ")} else {literal!(" ")};
            metamodelica::print(s);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: i, tail: l } => {
            let mut v: i32;
            let mut s: ArcStr;
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut b: bool;
            v = ({let __elt = (*metamodelica::index_checked(&v2.borrow(), i.clone())?).clone(); __elt});
            var = BackendVariable::getVarAt(vars, v)?;
            c = BackendVariable::varCref(&var);
            b = BackendVariable::isStateVar(&var);
            s = if (b) {literal!("der(")} else {literal!("")};
            metamodelica::print(s);
            s = ComponentReferenceBasics::printComponentRefStr(&c)?;
            metamodelica::print(s);
            s = if (b) {literal!(") ")} else {literal!(" ")};
            metamodelica::print(s);
            dumpComponentsAdvanced3(l, v2.clone(), vars)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpComponents(
    mut inComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inSyst: Option<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<()> {
    metamodelica::print(literal!("StrongComponents\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    List::map1(
        inComps,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: Option<metamodelica::Ref<BackendDAE::EqSystem>>| dumpComponent(&__a0, __a1),
        inSyst,
    )?;
    Ok(())
}

pub(crate) fn dumpComponent(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inSyst: Option<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<()> {
    metamodelica::print(printComponent(inComp, inSyst)?);
    Ok(())
}

pub(crate) fn printComponent(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inSyst: Option<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut tmpStr: ArcStr;
    let mut tmpStr2: ArcStr;
    oString = (match &**inComp {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v } => {
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*intString(v.clone()));
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::EQUATIONSYSTEM {
            eqns: ilst,
            vars: vlst,
            jacType,
            ..
        } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            ls = List::map(ilst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s2 = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("} Size: "));
                __mm_s.push_str(&*intString(((vlst).len() as i32)));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*jacobianTypeStr(jacType.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Array "));
                __mm_s.push_str(&*literal!(" {{"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("IfEquation "));
                __mm_s.push_str(&*literal!(" {{"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Algorithm "));
                __mm_s.push_str(&*literal!(" {{"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("ComplexEquation "));
                __mm_s.push_str(&*literal!(" {"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("WhenEquation "));
                __mm_s.push_str(&*literal!(" {"));
                __mm_s.push_str(&*intString(i.clone()));
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            tmpStr
        }
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    residualequations: ilst,
                    tearingvars: vlst,
                    innerEquations,
                    ..
                },
            casualTearingSet: None,
            linear: b,
            ..
        } => {
            let mut innerEqLst: metamodelica::List<i32>;
            let mut innerVarLst: metamodelica::List<metamodelica::List<i32>>;
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut eSys: metamodelica::Ref<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), &move |__a0: BackendDAE::InnerEquation| {
                innerEquationString(&__a0)
            })?;
            s = stringDelimitList(ls, literal!(", "));
            ls = List::map(ilst.clone(), &fnptr!(intString, i32))?;
            s2 = stringDelimitList(ls, literal!(", "));
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s3 = stringDelimitList(ls, literal!(", "));
            s4 = if (b.clone()) {
                literal!("linear")
            } else {
                literal!("nonlinear")
            };
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{{"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}\n,{"));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s3);
                __mm_s.push_str(&*literal!("}} Size: "));
                __mm_s.push_str(&*intString(((vlst).len() as i32)));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*s4);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            if (inSyst).is_some() {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eSys = metamodelica::Own::own(__pa0);
                    (innerEqLst, innerVarLst, _) =
                        BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(metamodelica::AsArg::as_arg(&innerEquations));
                    tmpStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr);
                        __mm_s.push_str(&*literal!(
                            "\nTearing Variables:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedVars(&eSys, vlst.clone())?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!(
                            "Residual Equations:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, ilst.clone())?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedVarsLsts(&eSys, &innerVarLst)?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, innerEqLst)?);
                        ArcStr::from(__mm_s)
                    };
                } else {
                    tmpStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr);
                        __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n"));
                        ArcStr::from(__mm_s)
                    };
                }
            }
            tmpStr
        }
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    residualequations: ilst,
                    tearingvars: vlst,
                    innerEquations,
                    ..
                },
            casualTearingSet:
                Some(BackendDAE::TearingSet {
                    residualequations: ilst2,
                    tearingvars: vlst2,
                    innerEquations: innerEquations2,
                    ..
                }),
            linear: b,
            ..
        } => {
            let mut innerEqLst: metamodelica::List<i32>;
            let mut innerVarLst: metamodelica::List<metamodelica::List<i32>>;
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut eSys: metamodelica::Ref<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), &move |__a0: BackendDAE::InnerEquation| {
                innerEquationString(&__a0)
            })?;
            s = stringDelimitList(ls, literal!(", "));
            ls = List::map(ilst.clone(), &fnptr!(intString, i32))?;
            s2 = stringDelimitList(ls, literal!(", "));
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s3 = stringDelimitList(ls, literal!(", "));
            s4 = if (b.clone()) {
                literal!("linear")
            } else {
                literal!("nonlinear")
            };
            tmpStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{{"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}\n,{"));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s3);
                __mm_s.push_str(&*literal!("}} Size: "));
                __mm_s.push_str(&*intString(((vlst).len() as i32)));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*s4);
                __mm_s.push_str(&*literal!(" (strict tearing set)\n"));
                ArcStr::from(__mm_s)
            };
            if (inSyst).is_some() {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eSys = metamodelica::Own::own(__pa0);
                    (innerEqLst, innerVarLst, _) =
                        BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(metamodelica::AsArg::as_arg(&innerEquations));
                    tmpStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr);
                        __mm_s.push_str(&*literal!(
                            "\nTearing Variables:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedVars(&eSys, vlst.clone())?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!(
                            "Residual Equations:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, ilst.clone())?);
                        __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedVarsLsts(&eSys, &innerVarLst)?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, innerEqLst)?);
                        ArcStr::from(__mm_s)
                    };
                } else {
                    tmpStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr);
                        __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n"));
                        ArcStr::from(__mm_s)
                    };
                }
            }
            ls = List::map(innerEquations2.clone(), &move |__a0: BackendDAE::InnerEquation| {
                innerEquationString(&__a0)
            })?;
            s = stringDelimitList(ls, literal!(", "));
            ls = List::map(ilst2.clone(), &fnptr!(intString, i32))?;
            s2 = stringDelimitList(ls, literal!(", "));
            ls = List::map(vlst2.clone(), &fnptr!(intString, i32))?;
            s3 = stringDelimitList(ls, literal!(", "));
            s4 = if (b.clone()) {
                literal!("linear")
            } else {
                literal!("nonlinear")
            };
            tmpStr2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{{"));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!("}\n,{"));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*s3);
                __mm_s.push_str(&*literal!("}} Size: "));
                __mm_s.push_str(&*intString(((vlst2).len() as i32)));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*s4);
                __mm_s.push_str(&*literal!(" (casual tearing set)\n"));
                ArcStr::from(__mm_s)
            };
            if (inSyst).is_some() {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa1 = ::match_deref::match_deref! { match &(inSyst) {
                        Some(__pa1) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eSys = metamodelica::Own::own(__pa1);
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(
                        metamodelica::AsArg::as_arg(&innerEquations2),
                    );
                    tmpStr2 = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr2);
                        __mm_s.push_str(&*literal!(
                            "\nTearing Variables:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedVars(&eSys, vlst2.clone())?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!(
                            "Residual Equations:\n-------------------------------------\n"
                        ));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, ilst2.clone())?);
                        __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedVarsLsts(&eSys, &innerVarLst)?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n"));
                        __mm_s.push_str(&*dumpMarkedEqns(&eSys, innerEqLst)?);
                        ArcStr::from(__mm_s)
                    };
                } else {
                    tmpStr2 = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpStr2);
                        __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n"));
                        ArcStr::from(__mm_s)
                    };
                }
            }
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*tmpStr);
                __mm_s.push_str(&*tmpStr2);
                ArcStr::from(__mm_s)
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(oString)
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
        __mm_s.push_str(&*stringDelimitList(List::map(lstLst, &intListStr)?, literal!("\n")));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

// =============================================================================
// section for all *String functions
//
// These are functions, that return their output with a String.
//   - equationString
//   - strongComponentString
// =============================================================================
pub(crate) fn strongComponentString(mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>) -> Result<ArcStr> {
    let mut outS: ArcStr;
    outS = (match &**inComp {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = intString(i.clone());
            s1 = intString(v.clone());
            s = stringAppendList(list![literal!("{"), s, literal!(":"), s1, literal!("}")]);
            s
        }
        BackendDAE::StrongComponent::EQUATIONSYSTEM {
            eqns: ilst,
            vars: vlst,
            jacType,
            ..
        } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut ls1: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            let mut sj: ArcStr;
            ls = List::map(ilst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            ls1 = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s1 = stringDelimitList(ls1, literal!(", "));
            sl = intString(((ilst).len() as i32));
            sj = jacobianTypeStr(jacType.clone());
            s2 = stringAppendList(list![
                literal!("{"),
                s,
                literal!(":"),
                s1,
                literal!("} Size: "),
                sl,
                literal!(" "),
                sj
            ]);
            s2
        }
        BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            sl = intString(i.clone());
            s2 = stringAppendList(list![literal!("Array "), sl, literal!(" {"), s, literal!("}")]);
            s2
        }
        BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            sl = intString(i.clone());
            s2 = stringAppendList(list![literal!("Array "), sl, literal!(" {"), s, literal!("}")]);
            s2
        }
        BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            sl = intString(i.clone());
            s2 = stringAppendList(list![literal!("Algorithm "), sl, literal!(" {"), s, literal!("}")]);
            s2
        }
        BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            sl = intString(i.clone());
            s2 = stringAppendList(list![
                literal!("ComplexEquation "),
                sl,
                literal!(" {"),
                s,
                literal!("}")
            ]);
            s2
        }
        BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vlst } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            sl = intString(i.clone());
            s2 = stringAppendList(list![literal!("WhenEquation "), sl, literal!(" {"), s, literal!("}")]);
            s2
        }
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    residualequations: ilst,
                    tearingvars: vlst,
                    innerEquations,
                    ..
                },
            linear: b,
            ..
        } => {
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            let mut sj: ArcStr;
            ls = List::map(innerEquations.clone(), &move |__a0: BackendDAE::InnerEquation| {
                innerEquationString(&__a0)
            })?;
            s = stringDelimitList(ls, literal!(", "));
            ls = List::map(ilst.clone(), &fnptr!(intString, i32))?;
            s1 = stringDelimitList(ls, literal!(", "));
            ls = List::map(vlst.clone(), &fnptr!(intString, i32))?;
            s2 = stringDelimitList(ls, literal!(", "));
            sj = intString(((vlst).len() as i32));
            sl = if (b.clone()) {
                literal!("linear")
            } else {
                literal!("nonlinear")
            };
            s2 = stringAppendList(list![
                literal!("torn "),
                sl,
                literal!(" Equationsystem"),
                literal!("{{"),
                s,
                literal!("},\n{"),
                s1,
                literal!(":"),
                s2,
                literal!("} Size: "),
                sj
            ]);
            s2
        }
    });
    Ok(outS)
}

pub fn whenEquationString(
    mut inWhenEqn: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut inStart: bool,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut conditionStr: ArcStr;
    let mut whenStmtStr: ArcStr;
    let mut elseWhenStr: ArcStr;
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut weqn: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut oweqn: Option<metamodelica::Ref<BackendDAE::WhenEquation>>;
    let mut whenStmtLst: metamodelica::List<BackendDAE::WhenOperator>;
    let __arc3 = &(*inWhenEqn);
    let BackendDAE::WHEN_STMTS {
        condition: __pa0,
        whenStmtLst: __pa1,
        elsewhenPart: __pa2,
    } = &**__arc3;
    cond = metamodelica::Own::own(__pa0);
    whenStmtLst = metamodelica::Own::own(__pa1);
    oweqn = metamodelica::Own::own(__pa2);
    conditionStr = ExpressionBasics::printExpStr(cond)?;
    whenStmtStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*stringDelimitList(
            List::map(whenStmtLst, &move |__a0: BackendDAE::WhenOperator| {
                dumpWhenOperatorStr(&__a0)
            })?,
            literal!(";\n  "),
        ));
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    };
    if (oweqn).is_some() {
        let __pa4 = ::match_deref::match_deref! { match &(oweqn) {
            Some(__pa4) => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        weqn = metamodelica::Own::own(__pa4);
        elseWhenStr = whenEquationString(&weqn, false)?;
    } else {
        elseWhenStr = literal!("");
    }
    if inStart {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("when "));
            __mm_s.push_str(&*conditionStr);
            __mm_s.push_str(&*literal!(" then\n  "));
            __mm_s.push_str(&*whenStmtStr);
            __mm_s.push_str(&*elseWhenStr);
            __mm_s.push_str(&*literal!("end when;"));
            ArcStr::from(__mm_s)
        };
    } else {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("elsewhen "));
            __mm_s.push_str(&*conditionStr);
            __mm_s.push_str(&*literal!(" then\n  "));
            __mm_s.push_str(&*whenStmtStr);
            __mm_s.push_str(&*elseWhenStr);
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

pub(crate) fn equationString(mut inEquation: &metamodelica::Ref<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inEquation {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" = "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![s1, literal!(" := "), s2]);
            res
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. } => {
            let mut res: ArcStr;
            res = whenEquationString(weqn, true)?;
            res
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
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
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = stringDelimitList(List::map(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| equationString(&__a0))?, literal!("\n  "));
            s3 = stringAppendList(list![literal!("if "), s1, literal!(" then\n  "), s2]);
            res = ifequationString(metamodelica::AsArg::as_arg(&expl), metamodelica::AsArg::as_arg(&eqnstrue), eqnsfalse, s3)?;
            res
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { iter, start, stop, body: eqn, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(iter.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(start.clone())?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(stop.clone())?); ArcStr::from(__mm_s) };
            s2 = equationString(eqn)?;
            res = stringAppendList(list![literal!("for "), s1, literal!(" loop\n    "), s2, literal!("; end for; ")]);
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn zeroCrossingString(mut inZeroCrossing: &BackendDAE::ZeroCrossing) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inZeroCrossing) {
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::RELATION { index: index_, .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: metamodelica::List<ArcStr>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut str_index: ArcStr;
            eq_s_list = List::map(eq.clone(), &fnptr!(intString, i32))?;
            eq_s = stringDelimitList(eq_s_list, literal!(","));
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            str_index = intString(index_.clone());
            str2 = stringAppendList(list![r#str, literal!(" with index = "), str_index, literal!(" in equations ["), eq_s, literal!("]")]);
            str2
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::LBINARY { .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: metamodelica::List<ArcStr>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), &fnptr!(intString, i32))?;
            eq_s = stringDelimitList(eq_s_list, literal!(","));
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            str2 = stringAppendList(list![r#str, literal!(" in equations ["), eq_s, literal!("]")]);
            str2
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::LUNARY { .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: metamodelica::List<ArcStr>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), &fnptr!(intString, i32))?;
            eq_s = stringDelimitList(eq_s_list, literal!(","));
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            str2 = stringAppendList(list![r#str, literal!(" in equations ["), eq_s, literal!("]")]);
            str2
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { .. }, .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: metamodelica::List<ArcStr>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), &fnptr!(intString, i32))?;
            eq_s = stringDelimitList(eq_s_list, literal!(","));
            r#str = ExpressionBasics::printExpStr(e.clone())?;
            str2 = stringAppendList(list![r#str, literal!(" in equations ["), eq_s, literal!("]")]);
            str2
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn timeEventString(mut inTimeEvent: &BackendDAE::TimeEvent) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inTimeEvent.clone() {
        BackendDAE::TimeEvent::SIMPLE_TIME_EVENT { .. } => literal!("SIMPLE_TIME_EVENT"),
        BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(
                var_field!(inTimeEvent.index, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone(),
            ));
            __mm_s.push_str(&*literal!(": sample("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(
                var_field!(inTimeEvent.startExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone(),
            )?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(
                var_field!(inTimeEvent.intervalExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone(),
            )?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        _ => literal!("unknown time event"),
    });
    Ok(outString)
}

// =============================================================================
// section for all debug* functions
//
// description: ???
// =============================================================================
pub(crate) fn debugStrCrefLstStr(
    mut a: &ArcStr,
    mut b: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut c: &ArcStr,
    mut d: &ArcStr,
) -> Result<()> {
    metamodelica::print(a.clone());
    debuglst(
        b,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0),
        c,
        d,
    )?;
    Ok(())
}

pub(crate) fn debugCrefStr(mut a: &metamodelica::Ref<DAE::ComponentRef>, mut b: &ArcStr) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a)?);
        __mm_s.push_str(&*b);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrIntStr(mut a: &ArcStr, mut b: i32, mut c: &ArcStr) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*intString(b));
        __mm_s.push_str(&*c);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrIntStrIntStr(
    mut a: &ArcStr,
    mut b: i32,
    mut c: &ArcStr,
    mut d: i32,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*intString(b));
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*intString(d));
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugCrefStrIntStr(
    mut a: &metamodelica::Ref<DAE::ComponentRef>,
    mut b: &ArcStr,
    mut c: i32,
    mut d: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a)?);
        __mm_s.push_str(&*b);
        __mm_s.push_str(&*intString(c));
        __mm_s.push_str(&*d);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrCrefStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<DAE::ComponentRef>,
    mut c: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?);
        __mm_s.push_str(&*c);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrCrefStrIntStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<DAE::ComponentRef>,
    mut c: &ArcStr,
    mut d: i32,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*intString(d));
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrCrefStrRealStrRealStrRealStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<DAE::ComponentRef>,
    mut c: &ArcStr,
    mut d: metamodelica::Real,
    mut e: &ArcStr,
    mut f: metamodelica::Real,
    mut g: &ArcStr,
    mut h: metamodelica::Real,
    mut i: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*realString(d));
        __mm_s.push_str(&*e);
        __mm_s.push_str(&*realString(f));
        __mm_s.push_str(&*g);
        __mm_s.push_str(&*realString(h));
        __mm_s.push_str(&*i);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrRealStrRealStrRealStrRealStr(
    mut a: &ArcStr,
    mut b: metamodelica::Real,
    mut c: &ArcStr,
    mut d: metamodelica::Real,
    mut e: &ArcStr,
    mut f: metamodelica::Real,
    mut g: &ArcStr,
    mut h: metamodelica::Real,
    mut i: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*realString(b));
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*realString(d));
        __mm_s.push_str(&*e);
        __mm_s.push_str(&*realString(f));
        __mm_s.push_str(&*g);
        __mm_s.push_str(&*realString(h));
        __mm_s.push_str(&*i);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrCrefStrExpStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<DAE::ComponentRef>,
    mut c: &ArcStr,
    mut d: metamodelica::Ref<DAE::Exp>,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?);
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrCrefStrCrefStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<DAE::ComponentRef>,
    mut c: &ArcStr,
    mut d: &metamodelica::Ref<DAE::ComponentRef>,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d)?);
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugExpStr(mut a: metamodelica::Ref<DAE::Exp>, mut b: &ArcStr) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ExpressionBasics::printExpStr(a)?);
        __mm_s.push_str(&*b);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrExpStr(mut a: &ArcStr, mut b: metamodelica::Ref<DAE::Exp>, mut c: &ArcStr) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?);
        __mm_s.push_str(&*c);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrExpLstStr(
    mut a: &ArcStr,
    mut b: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut c: &ArcStr,
    mut d: &ArcStr,
) -> Result<()> {
    metamodelica::print(a.clone());
    debuglst(b, &ExpressionBasics::printExpStr, c, d)?;
    Ok(())
}

pub(crate) fn debugStrExpStrCrefStr(
    mut a: &ArcStr,
    mut b: metamodelica::Ref<DAE::Exp>,
    mut c: &ArcStr,
    mut d: &metamodelica::Ref<DAE::ComponentRef>,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d)?);
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrExpStrExpStr(
    mut a: &ArcStr,
    mut b: metamodelica::Ref<DAE::Exp>,
    mut c: &ArcStr,
    mut d: metamodelica::Ref<DAE::Exp>,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?);
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugExpStrExpStrExpStr(
    mut a: metamodelica::Ref<DAE::Exp>,
    mut b: &ArcStr,
    mut c: metamodelica::Ref<DAE::Exp>,
    mut d: &ArcStr,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut f: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ExpressionBasics::printExpStr(a)?);
        __mm_s.push_str(&*b);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(c)?);
        __mm_s.push_str(&*d);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(e)?);
        __mm_s.push_str(&*f);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrExpStrExpStrExpStr(
    mut a: &ArcStr,
    mut b: metamodelica::Ref<DAE::Exp>,
    mut c: &ArcStr,
    mut d: metamodelica::Ref<DAE::Exp>,
    mut e: &ArcStr,
    mut f: metamodelica::Ref<DAE::Exp>,
    mut g: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?);
        __mm_s.push_str(&*e);
        __mm_s.push_str(&*ExpressionBasics::printExpStr(f)?);
        __mm_s.push_str(&*g);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrEqnStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<BackendDAE::Equation>,
    mut c: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*equationString(b)?);
        __mm_s.push_str(&*c);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debugStrEqnStrEqnStr(
    mut a: &ArcStr,
    mut b: &metamodelica::Ref<BackendDAE::Equation>,
    mut c: &ArcStr,
    mut d: &metamodelica::Ref<BackendDAE::Equation>,
    mut e: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*a);
        __mm_s.push_str(&*equationString(b)?);
        __mm_s.push_str(&*c);
        __mm_s.push_str(&*equationString(d)?);
        __mm_s.push_str(&*e);
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn debuglst<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: &metamodelica::List<Type_a>,
    mut f: &dyn ::std::ops::Fn(Type_a) -> Result<ArcStr>,
    mut c: &ArcStr,
    mut se: &ArcStr,
) -> Result<()> {
    pub type FuncTypeType_aToStr<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::print(se.clone());
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::print(f(a.clone())?);
            metamodelica::print(se.clone());
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: rest } => {
            metamodelica::print(f(a.clone())?);
            metamodelica::print(c.clone());
            debuglst(rest, f, c, se)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// =============================================================================
// unsorted section
//
// These section should be empty. Feel free to sort these functions into one of
// the upper sections.
// =============================================================================
pub(crate) fn printCallFunction2StrDIVISION<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut stringDelimiter: ArcStr,
    mut opcreffunc: Option<(
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>,
        Type_a,
    )>,
) -> Result<ArcStr> {
    pub type strongComponentStringRefStrFunc<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = ExpressionDump::printExp2Str(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: e2.clone() }), &stringDelimiter, opcreffunc, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _| printCallFunction2StrDIVISION(&__a0, __a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)));
            s
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = ExpressionDump::printExp2Str(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: ty.clone() }, exp2: e2.clone() }), &stringDelimiter, opcreffunc, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _| printCallFunction2StrDIVISION(&__a0, __a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)));
            s
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_SCALAR_ARRAY" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = ExpressionDump::printExp2Str(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: ty.clone() }, exp2: e2.clone() }), &stringDelimiter, opcreffunc, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _| printCallFunction2StrDIVISION(&__a0, __a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)));
            s
        },
        Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
            let mut s: ArcStr;
            let mut s_1: ArcStr;
            let mut s_2: ArcStr;
            let mut fs: ArcStr;
            let mut argstr: ArcStr;
            fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
            argstr = stringDelimitList(List::map3(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(ExpressionDump::printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter, opcreffunc, Some((std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _| printCallFunction2StrDIVISION(&__a0, __a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))?, literal!(","));
            s = stringAppend(fs, literal!("("));
            s_1 = stringAppend(s, argstr);
            s_2 = stringAppend(s_1, literal!(")"));
            s_2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

// protected function printVarsStatistics "author: PA
//
//   Prints statistics on variables, etc.
// "
//   input BackendDAE.Variables inVariables1;
//   input BackendDAE.Variables inVariables2;
// algorithm
//   _:=
//   matchcontinue (inVariables1,inVariables2)
//     local
//       String lenstr,bstr;
//       BackendDAE.VariableArray v1,v2;
//       Integer bsize1,n1,bsize2,n2;
//     case (BackendDAE.VARIABLES(varArr = v1,bucketSize = bsize1,numberOfVars = n1),BackendDAE.VARIABLES(varArr = v2,bucketSize = bsize2,numberOfVars = n2))
//       equation
//         print("Variable Statistics\n");
//         print("===================\n");
//         print("Number of variables: ");
//         lenstr = intString(n1);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//         print("Number of known variables: ");
//         lenstr = intString(n2);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for known variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//       then
//         ();
//   end matchcontinue;
// end printVarsStatistics;
pub(crate) fn dumpWhenOperatorStr(mut inWhenOperator: &BackendDAE::WhenOperator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inWhenOperator.clone() {
        BackendDAE::WhenOperator::ASSIGN {
            left: ref e1,
            right: ref e,
            ..
        } => {
            let mut scr: ArcStr;
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            scr = ExpressionBasics::printExpStr(e1.clone())?;
            se = ExpressionBasics::printExpStr(e.clone())?;
            r#str = stringAppendList(list![scr, literal!(" := "), se]);
            r#str
        }
        BackendDAE::WhenOperator::REINIT {
            stateVar: ref cr,
            value: ref e,
            ..
        } => {
            let mut scr: ArcStr;
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            scr = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            se = ExpressionBasics::printExpStr(e.clone())?;
            r#str = stringAppendList(list![literal!("reinit("), scr, literal!(","), se, literal!(")")]);
            r#str
        }
        BackendDAE::WhenOperator::ASSERT {
            condition: ref e,
            message: ref e1,
            ..
        } => {
            let mut se: ArcStr;
            let mut se1: ArcStr;
            let mut r#str: ArcStr;
            se = ExpressionBasics::printExpStr(e.clone())?;
            se1 = ExpressionBasics::printExpStr(e1.clone())?;
            r#str = stringAppendList(list![literal!("assert("), se, literal!(","), se1, literal!(")")]);
            r#str
        }
        BackendDAE::WhenOperator::TERMINATE { message: ref e, .. } => {
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = ExpressionBasics::printExpStr(e.clone())?;
            r#str = stringAppendList(list![literal!("terminate("), se, literal!(")")]);
            r#str
        }
        BackendDAE::WhenOperator::NORETCALL { exp: ref e, .. } => ExpressionBasics::printExpStr(e.clone())?,
    });
    Ok(outString)
}

pub(crate) fn dumpOption<Type_A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inType: Option<Type_A>,
    mut infunc: &dyn ::std::ops::Fn(Type_A) -> Result<()>,
) -> Result<()> {
    pub type printType_A<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_A) -> Result<()> + 'static>;

    let () = (match inType {
        Some(mut a) => {
            infunc(a)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn dumpAlgorithms(
    mut ialgs: &metamodelica::List<metamodelica::Ref<DAE::Algorithm>>,
    mut indx: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match ialgs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Algorithm { statementLst: stmts }, tail: algs } => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr;
            is = intString(indx);
            myStream = IOStream::create(literal!(""), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream, stringAppend(is, literal!(". ")))?;
            myStream = DAEDump::dumpAlgorithmStream(&(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts.clone() }), source: DAE::emptyElementSource().clone() })), myStream);
            IOStream::print(&myStream, IOStream::stdOutput.clone())?;
            dumpAlgorithms(algs, indx + 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpConstraints(
    mut ionstrs: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut indx: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match ionstrs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, tail: constrs } => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr;
            is = intString(indx);
            myStream = IOStream::create(literal!(""), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream, stringAppend(is, literal!(". ")))?;
            myStream = DAEDump::dumpConstraintStream(&(list![metamodelica::Ref::new(DAE::Element::CONSTRAINT { constraints: metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps.clone() }), source: DAE::emptyElementSource().clone() })]), myStream)?;
            IOStream::print(&myStream, IOStream::stdOutput.clone())?;
            dumpConstraints(constrs, indx + 1)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn dumpSparsePatternArray(mut inSparsePatter: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Print sparse pattern: "));
        __mm_s.push_str(&*intString(metamodelica::arrayLength(inSparsePatter.clone())));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpSparsePattern2(
        &(inSparsePatter
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>()),
        1,
    )?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpSparsePattern(mut inSparsePatter: &metamodelica::List<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Print sparse pattern: "));
        __mm_s.push_str(&*intString(((inSparsePatter).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    dumpSparsePattern2(inSparsePatter, 1)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpSparsePattern2(
    mut inSparsePatter: &metamodelica::List<metamodelica::List<i32>>,
    mut inInteger: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inSparsePatter {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
            let mut sparsepatternStr: ArcStr;
            sparsepatternStr = List::toStringCustom(elem.clone(), &fnptr!(intString, i32), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Row[")); __mm_s.push_str(&*intString(inInteger)); __mm_s.push_str(&*literal!("] = ")); ArcStr::from(__mm_s) }, literal!("{"), literal!(";"), literal!("}"), true, 0)?;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*sparsepatternStr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            dumpSparsePattern2(rest, inInteger + 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpJacobianStr(
    mut inTplIntegerIntegerEquationLstOption: Option<
        metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
    >,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inTplIntegerIntegerEquationLstOption) {
        Some(eqns) => {
            let mut res: metamodelica::List<ArcStr>;
            let mut res_1: ArcStr;
            res = dumpJacobianStr2(metamodelica::AsArg::as_arg(&eqns))?;
            res_1 = stringDelimitList(res, literal!(",\n"));
            res_1
        },
        None => {
            literal!("No analytic jacobian available\n")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn dumpJacobianStr2(
    mut inTplIntegerIntegerEquationLst: &metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = (::match_deref::match_deref! { match inTplIntegerIntegerEquationLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (row, col, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: eqns } => {
            let mut estr: ArcStr;
            let mut rowstr: ArcStr;
            let mut colstr: ArcStr;
            let mut r#str: ArcStr;
            let mut strs: metamodelica::List<ArcStr>;
            estr = ExpressionBasics::printExpStr(e.clone())?;
            rowstr = intString(row.clone());
            colstr = intString(col.clone());
            r#str = stringAppendList(list![literal!("{"), rowstr, literal!(","), colstr, literal!("}:"), estr]);
            strs = dumpJacobianStr2(eqns)?;
            metamodelica::cons(r#str, strs)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStringLst)
}

pub(crate) fn jacobianTypeStr(mut inJacobianType: BackendDAE::JacobianType) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inJacobianType {
        BackendDAE::JacobianType::JAC_CONSTANT { .. } => literal!("Jacobian Constant"),
        BackendDAE::JacobianType::JAC_LINEAR { .. } => literal!("Jacobian Linear"),
        BackendDAE::JacobianType::JAC_NONLINEAR { .. } => literal!("Jacobian Nonlinear"),
        BackendDAE::JacobianType::JAC_GENERIC { .. } => literal!("Generic Jacobian via directional derivatives"),
        BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. } => literal!("No analytic jacobian"),
    });
    outString
}

pub(crate) fn dumpJacobianString(mut jacIn: &metamodelica::Ref<BackendDAE::Jacobian>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match jacIn {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fJac } => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("###############\n")); __mm_s.push_str(&*literal!(" FULL_JACOBIAN \n")); __mm_s.push_str(&*literal!("###############\n\n")); __mm_s.push_str(&*dumpJacobianStr(fJac.clone())?); ArcStr::from(__mm_s) };
            metamodelica::print(s);
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(sJac), sparsePattern, .. } => {
            let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
            (dae, _, _, _, _, _) = sJac.clone();
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) });
            dumpBackendDAE(&dae, &(literal!("Directional Derivatives System")))?;
            dumpSparsityPattern(sparsePattern, &(literal!("Sparse Pattern")))?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, sparsePattern, .. } => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) });
            dumpSparsityPattern(sparsePattern, &(literal!("Sparse Pattern")))?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. } => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("################\n")); __mm_s.push_str(&*literal!(" EMPTY_JACOBIAN \n")); __mm_s.push_str(&*literal!("################\n\n")); ArcStr::from(__mm_s) });
            literal!("")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn symJacString(
    mut jacIn: &(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
) -> Result<ArcStr> {
    let mut sOut: ArcStr;
    sOut = (::match_deref::match_deref! { match &(jacIn) {
        (Some(sJac), sparsePattern, _) => {
            let mut dae: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut s: ArcStr;
            (dae, _, _, _, _, _) = sJac.clone();
            s = literal!("GENERIC JACOBIAN:\n");
            dumpBackendDAE(&dae, &(literal!("Directional Derivatives System")))?;
            dumpSparsityPattern(&(sparsePattern.clone()), &(literal!("Sparse Pattern")))?;
            s
        },
        (None, sparsePattern, _) => {
            let mut s: ArcStr;
            s = literal!("GENERIC JACOBIAN:\n");
            dumpSparsityPattern(&(sparsePattern.clone()), &(literal!("Sparse Pattern")))?;
            s
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sOut)
}

pub(crate) fn dumpEqnsStr(mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(dumpEqnsStr2(eqns, 1, metamodelica::nil())?, literal!("\n"));
    Ok(r#str)
}

fn dumpEqnsStr2(
    mut inEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inInteger: i32,
    mut inAcc: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEquationLst, inInteger, inAcc)) {
            (Deref @ metamodelica::ListNode::Nil, _, acc) => {
                return Ok(acc.clone().reverse())
            },
            (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, index, acc) => {
                let mut es: ArcStr;
                let mut is: ArcStr;
                let mut r#str: ArcStr;
                let mut index_1: i32;
                let mut acc = (*acc).clone();
                es = equationString(metamodelica::AsArg::as_arg(&eqn))?;
                is = intString(index.clone());
                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*is); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*es); ArcStr::from(__mm_s) };
                index_1 = index.clone() + 1;
                acc = metamodelica::cons(r#str, acc.clone());
                { (inEquationLst, inInteger, inAcc) = (eqns.clone(), index_1, acc.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn ifequationString<'__b>(
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut eqnstrue: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eqnsfalse: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iString: ArcStr,
) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match (conditions, eqnstrue, eqnsfalse) {
            (Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Nil) => {
                let mut s: ArcStr;
                return Ok(stringAppendList(list![iString, literal!("\nend if")]))
            },
            (Deref @ metamodelica::ListNode::Nil, _, _) => {
                let mut seqns: ArcStr;
                let mut s: ArcStr;
                seqns = stringDelimitList(List::map(eqnsfalse.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| equationString(&__a0))?, literal!("\n  "));
                return Ok(stringAppendList(list![iString, literal!("\nelse\n  "), seqns, literal!("\nend if")]))
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: elst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }, _) => {
                let mut seqns: ArcStr;
                let mut s: ArcStr;
                let mut se: ArcStr;
                se = ExpressionBasics::printExpStr(e.clone())?;
                seqns = stringDelimitList(List::map(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| equationString(&__a0))?, literal!("\n  "));
                s = stringAppendList(list![iString, literal!("\nelseif "), se, literal!(" then\n  "), seqns]);
                { (conditions, eqnstrue, eqnsfalse, iString) = (elst, eqnslst, eqnsfalse, s); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn varString(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut paths_lst: metamodelica::List<ArcStr>;
    let mut unreplaceableStr: ArcStr;
    let mut dimensions: ArcStr;
    paths = ElementSource::getElementSourceTypes(&inVar.source);
    paths_lst = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (paths).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unreplaceableStr = if (inVar.unreplaceable.clone()) {
        literal!(" unreplaceable")
    } else {
        literal!("")
    };
    dimensions = ExpressionBasics::dimensionsString(inVar.arryDim.clone())?;
    dimensions = if (!metamodelica::stringEq(&dimensions, &(literal!("")))) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" ["));
            __mm_s.push_str(&*dimensions);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!("")
    };
    outStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*DAEDump::dumpDirectionStr(inVar.varDirection.clone()));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inVar.varName)?);
        __mm_s.push_str(&*if ((inVar.tplExp).is_some()) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" in "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(
                    inVar.tplExp.clone().ok_or("pattern mismatch")?,
                )?);
                ArcStr::from(__mm_s)
            }
        } else {
            literal!("")
        });
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*kindString(&inVar.varKind)?);
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*connectorTypeString(&inVar.connectorType));
        __mm_s.push_str(&*attributesString(inVar.values.clone())?);
        __mm_s.push_str(&*literal!(") "));
        __mm_s.push_str(&*optExpressionString(inVar.bindExp.clone(), &(literal!("")))?);
        __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(inVar.comment.clone()));
        __mm_s.push_str(&*stringDelimitList(paths_lst, literal!(", ")));
        __mm_s.push_str(&*literal!(" type: "));
        __mm_s.push_str(&*DAEDump::daeTypeStr(&inVar.varType)?);
        __mm_s.push_str(&*dimensions);
        __mm_s.push_str(&*unreplaceableStr);
        ArcStr::from(__mm_s)
    };
    Ok(outStr)
}

pub(crate) fn varStringShort(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ComponentReferenceBasics::printComponentRefStr(&inVar.varName)?;
    Ok(outStr)
}

pub(crate) fn dumpKind(mut inVarKind: &BackendDAE::VarKind) -> Result<()> {
    metamodelica::print(kindString(inVarKind)?);
    Ok(())
}

pub(crate) fn kindString(mut inVarKind: &BackendDAE::VarKind) -> Result<ArcStr> {
    let mut kindStr: ArcStr;
    kindStr = (::match_deref::match_deref! { match &(inVarKind) {
        BackendDAE::VarKind::VARIABLE { .. } => {
            literal!("VARIABLE")
        },
        BackendDAE::VarKind::STATE { index: i, derName: None, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE { index: i, derName: Some(dcr), .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&dcr))?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE_DER { .. } => {
            literal!("STATE_DER")
        },
        BackendDAE::VarKind::DUMMY_DER { .. } => {
            literal!("DUMMY_DER")
        },
        BackendDAE::VarKind::DUMMY_STATE { .. } => {
            literal!("DUMMY_STATE")
        },
        BackendDAE::VarKind::CLOCKED_STATE { .. } => {
            literal!("CLOCKED_STATE")
        },
        BackendDAE::VarKind::DISCRETE { .. } => {
            literal!("DISCRETE")
        },
        BackendDAE::VarKind::PARAM { .. } => {
            literal!("PARAM")
        },
        BackendDAE::VarKind::CONST { .. } => {
            literal!("CONST")
        },
        BackendDAE::VarKind::EXTOBJ { fullClassName: path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EXTOBJ: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::JAC_VAR { .. } => {
            literal!("JACOBIAN_VAR")
        },
        BackendDAE::VarKind::JAC_TMP_VAR { .. } => {
            literal!("JACOBIAN_TMP_VAR")
        },
        BackendDAE::VarKind::OPT_CONSTR { .. } => {
            literal!("OPT_CONSTR")
        },
        BackendDAE::VarKind::OPT_FCONSTR { .. } => {
            literal!("OPT_FCONSTR")
        },
        BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. } => {
            literal!("OPT_INPUT_WITH_DER")
        },
        BackendDAE::VarKind::OPT_INPUT_DER { .. } => {
            literal!("OPT_INPUT_DER")
        },
        BackendDAE::VarKind::OPT_TGRID { .. } => {
            literal!("OPT_TGRID")
        },
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => {
            literal!("OPT_LOOP_INPUT")
        },
        BackendDAE::VarKind::ALG_STATE { .. } => {
            literal!("ALG_STATE")
        },
        BackendDAE::VarKind::ALG_STATE_OLD { .. } => {
            literal!("ALG_STATE_OLD")
        },
        BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. } => {
            literal!("DAE_RESIDUAL_VAR")
        },
        BackendDAE::VarKind::DAE_AUX_VAR { .. } => {
            literal!("DAE_AUX_VAR")
        },
        BackendDAE::VarKind::LOOP_ITERATION { .. } => {
            literal!("LOOP_ITERATION")
        },
        BackendDAE::VarKind::LOOP_SOLVED { .. } => {
            literal!("LOOP_SOLVED")
        },
        _ => {
            literal!("ERROR: BackendDump.kindString varKind not implemented")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(kindStr)
}

pub(crate) fn dumpConnectorType(mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> Result<()> {
    metamodelica::print(connectorTypeString(inConnectorType));
    Ok(())
}

pub(crate) fn connectorTypeString(mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> ArcStr {
    let mut connectorTypeStr: ArcStr;
    connectorTypeStr = (match &**inConnectorType {
        DAE::ConnectorType::FLOW { .. } => literal!("flow=true "),
        DAE::ConnectorType::POTENTIAL { .. } => literal!("flow=false "),
        _ => literal!(""),
    });
    connectorTypeStr
}

pub(crate) fn dumpAttributes(mut inAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inAttr) {
        None => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, isProtected: None, finalPrefix: None, distributionOption: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min, max, start, fixed, nominal, stateSelectOption, isProtected, finalPrefix, distributionOption: dist, .. }) => {
            dumpOptExpression(min.clone(), literal!("min"))?;
            dumpOptExpression(max.clone(), literal!("max"))?;
            dumpOptExpression(start.clone(), literal!("start"))?;
            dumpOptExpression(fixed.clone(), literal!("fixed"))?;
            dumpOptExpression(nominal.clone(), literal!("nominal"))?;
            dumpOptStateSelection(stateSelectOption.clone())?;
            dumpOptBoolean(isProtected.clone(), literal!("protected"))?;
            dumpOptBoolean(finalPrefix.clone(), literal!("final"))?;
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, distributionOption: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min, max, start, fixed, isProtected, finalPrefix, distributionOption: dist, .. }) => {
            dumpOptExpression(min.clone(), literal!("min"))?;
            dumpOptExpression(max.clone(), literal!("max"))?;
            dumpOptExpression(start.clone(), literal!("start"))?;
            dumpOptExpression(fixed.clone(), literal!("fixed"))?;
            dumpOptBoolean(isProtected.clone(), literal!("protected"))?;
            dumpOptBoolean(finalPrefix.clone(), literal!("final"))?;
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start, fixed, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(start.clone(), literal!("start"))?;
            dumpOptExpression(fixed.clone(), literal!("fixed"))?;
            dumpOptBoolean(isProtected.clone(), literal!("protected"))?;
            dumpOptBoolean(finalPrefix.clone(), literal!("final"))?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(start.clone(), literal!("start"))?;
            dumpOptBoolean(isProtected.clone(), literal!("protected"))?;
            dumpOptBoolean(finalPrefix.clone(), literal!("final"))?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min, max, start, fixed, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(min.clone(), literal!("min"))?;
            dumpOptExpression(max.clone(), literal!("max"))?;
            dumpOptExpression(start.clone(), literal!("start"))?;
            dumpOptExpression(fixed.clone(), literal!("fixed"))?;
            dumpOptBoolean(isProtected.clone(), literal!("protected"))?;
            dumpOptBoolean(finalPrefix.clone(), literal!("final"))?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptDistribution(mut dist: Option<metamodelica::Ref<DAE::Distribution>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dist) {
        None => {
            ()
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) });
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptStateSelection(mut ss: Option<DAE::StateSelect>) -> Result<()> {
    let () = (match ss {
        Some(DAE::StateSelect::NEVER { .. }) => {
            metamodelica::print(literal!("stateSelect=StateSelect.never "));
            ()
        }
        Some(DAE::StateSelect::AVOID { .. }) => {
            metamodelica::print(literal!("stateSelect=StateSelect.avoid "));
            ()
        }
        Some(DAE::StateSelect::DEFAULT { .. }) => (),
        Some(DAE::StateSelect::PREFER { .. }) => {
            metamodelica::print(literal!("stateSelect=StateSelect.prefer "));
            ()
        }
        Some(DAE::StateSelect::ALWAYS { .. }) => {
            metamodelica::print(literal!("stateSelect=StateSelect.alwas "));
            ()
        }
        _ => (),
    });
    Ok(())
}

fn dumpOptExpression(mut inExp: Option<metamodelica::Ref<DAE::Exp>>, mut inString: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inExp) {
        Some(e) => {
            let mut s = inString;
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = ExpressionBasics::printExpStr(e.clone())?;
            r#str = stringAppendList(list![s, literal!(" = "), se, literal!(" ")]);
            metamodelica::print(r#str);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptBoolean(mut inExp: Option<bool>, mut inString: ArcStr) -> Result<()> {
    let () = (match (inExp, inString) {
        (Some(true), mut s) => {
            let mut r#str: ArcStr;
            r#str = stringAppendList(list![s, literal!(" = true ")]);
            metamodelica::print(r#str);
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn attributesString(mut inAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inAttr) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: None, max: None, start: None, unit: None, fixed: None, nominal: None, stateSelectOption: None, isProtected: None, finalPrefix: None, distributionOption: None, uncertainOption: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min, max, start, unit, fixed, nominal, stateSelectOption, isProtected, finalPrefix, distributionOption: dist, uncertainOption: uncertainopt, .. }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), &(literal!("min")))?); __mm_s.push_str(&*optExpressionString(max.clone(), &(literal!("max")))?); __mm_s.push_str(&*optExpressionString(start.clone(), &(literal!("start")))?); __mm_s.push_str(&*optExpressionString(unit.clone(), &(literal!("unit")))?); __mm_s.push_str(&*optExpressionString(fixed.clone(), &(literal!("fixed")))?); __mm_s.push_str(&*optExpressionString(nominal.clone(), &(literal!("nominal")))?); __mm_s.push_str(&*optStateSelectionString(stateSelectOption.clone())); __mm_s.push_str(&*optBooleanString(isProtected.clone(), &(literal!("protected")))); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), &(literal!("final")))); __mm_s.push_str(&*optDistributionString(dist.clone())?); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) };
            r#str
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, distributionOption: None, uncertainOption: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min, max, start, fixed, isProtected, finalPrefix, uncertainOption: uncertainopt, .. }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), &(literal!("min")))?); __mm_s.push_str(&*optExpressionString(max.clone(), &(literal!("max")))?); __mm_s.push_str(&*optExpressionString(start.clone(), &(literal!("start")))?); __mm_s.push_str(&*optExpressionString(fixed.clone(), &(literal!("fixed")))?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), &(literal!("protected")))); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), &(literal!("final")))); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) };
            r#str
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start, fixed, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), &(literal!("start")))?); __mm_s.push_str(&*optExpressionString(fixed.clone(), &(literal!("fixed")))?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), &(literal!("protected")))); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), &(literal!("final")))); ArcStr::from(__mm_s) };
            r#str
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), &(literal!("start")))?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), &(literal!("protected")))); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), &(literal!("final")))); ArcStr::from(__mm_s) };
            r#str
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min, max, start, fixed, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), &(literal!("min")))?); __mm_s.push_str(&*optExpressionString(max.clone(), &(literal!("max")))?); __mm_s.push_str(&*optExpressionString(start.clone(), &(literal!("start")))?); __mm_s.push_str(&*optExpressionString(fixed.clone(), &(literal!("fixed")))?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), &(literal!("protected")))); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), &(literal!("final")))); ArcStr::from(__mm_s) };
            r#str
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn optDistributionString(mut dist: Option<metamodelica::Ref<DAE::Distribution>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(dist) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn optUncertainty(mut uncertainty: Option<DAE::Uncertainty>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match uncertainty {
        None => literal!(""),
        Some(DAE::Uncertainty::GIVEN { .. }) => literal!("uncertain=Uncertainty.given"),
        Some(DAE::Uncertainty::SOUGHT { .. }) => literal!("uncertain=Uncertainty.sought"),
        Some(DAE::Uncertainty::REFINE { .. }) => literal!("uncertain=Uncertainty.refine"),
        Some(DAE::Uncertainty::PROPAGATE { .. }) => literal!("uncertain=Uncertainty.propagate"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn optStateSelectionString(mut ss: Option<DAE::StateSelect>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match ss {
        Some(DAE::StateSelect::NEVER { .. }) => literal!("stateSelect=StateSelect.never "),
        Some(DAE::StateSelect::AVOID { .. }) => literal!("stateSelect=StateSelect.avoid "),
        Some(DAE::StateSelect::DEFAULT { .. }) => literal!(""),
        Some(DAE::StateSelect::PREFER { .. }) => literal!("stateSelect=StateSelect.prefer "),
        Some(DAE::StateSelect::ALWAYS { .. }) => literal!("stateSelect=StateSelect.always "),
        _ => literal!(""),
    });
    outString
}

pub(crate) fn partitionKindString(mut inPartitionKind: BackendDAE::BaseClockPartitionKind) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inPartitionKind {
        BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: mut idx } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("clocked partition("));
            __mm_s.push_str(&*intString(idx.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION { .. } => {
            literal!("continuous time partition")
        }
        BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION { .. } => {
            literal!("unspecified partition")
        }
        BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION { .. } => {
            literal!("unknown partition")
        }
        _ => {
            Error::addInternalError(
                literal!("function partitionKindString failed"),
                metamodelica::sourceInfo!("BackEnd/BackendDump.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn equationAttrString(mut inEqAttr: BackendDAE::EquationAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut kind: BackendDAE::EquationKind;
    let mut evalStages: BackendDAE::EvaluationStages;
    let BackendDAE::EQUATION_ATTRIBUTES {
        kind: __pa0,
        evalStages: __pa1,
        ..
    } = inEqAttr;
    kind = metamodelica::Own::own(__pa0);
    evalStages = metamodelica::Own::own(__pa1);
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("["));
        __mm_s.push_str(&*equationKindString(kind)?);
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*equationEvaluationStageString(evalStages));
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    Ok(outString)
}

fn equationKindString(mut inEqKind: BackendDAE::EquationKind) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inEqKind {
        BackendDAE::EquationKind::BINDING_EQUATION { .. } => {
            literal!("binding")
        }
        BackendDAE::EquationKind::DYNAMIC_EQUATION { .. } => {
            literal!("dynamic")
        }
        BackendDAE::EquationKind::INITIAL_EQUATION { .. } => {
            literal!("initial")
        }
        BackendDAE::EquationKind::AUX_EQUATION { .. } => {
            literal!("auxiliary")
        }
        BackendDAE::EquationKind::DISCRETE_EQUATION { .. } => {
            literal!("discrete")
        }
        BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND { .. } => {
            literal!("unknown")
        }
        BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut i } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX));
                    __mm_s.push_str(&*intString(i.clone()));
                    ArcStr::from(__mm_s)
                },
                identType: DAE::T_CLOCK_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("clocked("));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        _ => {
            Error::addInternalError(
                literal!("function equationKindString failed"),
                metamodelica::sourceInfo!("BackEnd/BackendDump.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn equationEvaluationStageString(mut inEqEvalStage: BackendDAE::EvaluationStages) -> ArcStr {
    let mut outString: ArcStr = literal!("|");
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*outString);
        __mm_s.push_str(&*if (inEqEvalStage.dynamicEval.clone()) {
            literal!("1|")
        } else {
            literal!("0|")
        });
        ArcStr::from(__mm_s)
    };
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*outString);
        __mm_s.push_str(&*if (inEqEvalStage.algebraicEval.clone()) {
            literal!("1|")
        } else {
            literal!("0|")
        });
        ArcStr::from(__mm_s)
    };
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*outString);
        __mm_s.push_str(&*if (inEqEvalStage.zerocrossEval.clone()) {
            literal!("1|")
        } else {
            literal!("0|")
        });
        ArcStr::from(__mm_s)
    };
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*outString);
        __mm_s.push_str(&*if (inEqEvalStage.discreteEval.clone()) {
            literal!("1|")
        } else {
            literal!("0|")
        });
        ArcStr::from(__mm_s)
    };
    outString
}

fn optExpressionString(mut inExp: Option<metamodelica::Ref<DAE::Exp>>, mut inString: &ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inExp) {
        Some(e) => {
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = ExpressionBasics::printExpStr(e.clone())?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*inString); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*se); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn optBooleanString(mut inExp: Option<bool>, mut inString: &ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inExp {
        Some(true) => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inString);
                __mm_s.push_str(&*literal!(" = true "));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        _ => {
            literal!("")
        }
    });
    outString
}

pub(crate) fn dumpAdjacencyMatrix(mut m: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    let mut rowIndex: i32 = 0;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nAdjacency Matrix (row: equation)\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("number of rows: "));
        __mm_s.push_str(&*intString(metamodelica::arrayLength(m.clone())));
        ArcStr::from(__mm_s)
    });
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex + 1;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*intString(rowIndex));
            __mm_s.push_str(&*literal!(":"));
            ArcStr::from(__mm_s)
        });
        for mut i in &*row {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*intString(i.clone()));
                ArcStr::from(__mm_s)
            });
        }
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixT(mut mT: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    let mut rowIndex: i32 = 0;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nTransposed Adjacency Matrix (row: variable)\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("number of rows: "));
        __mm_s.push_str(&*intString(metamodelica::arrayLength(mT.clone())));
        ArcStr::from(__mm_s)
    });
    let __range0 = mT.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex + 1;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*intString(rowIndex));
            __mm_s.push_str(&*literal!(":"));
            ArcStr::from(__mm_s)
        });
        for mut i in &*row {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*intString(i.clone()));
                ArcStr::from(__mm_s)
            });
        }
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpAdjacencyRow(mut inIntegerLst: &metamodelica::List<i32>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::print(literal!("\n"));
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut s: ArcStr;
            s = intString(x.clone());
            metamodelica::print(s);
            metamodelica::print(literal!(" "));
            dumpAdjacencyRow(xs)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixEnhanced(
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<()> {
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: metamodelica::List<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    metamodelica::print(literal!("Adjacency Matrix Enhanced (row == equation)\n"));
    metamodelica::print(literal!("====================================\n"));
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = intString(mlen);
    metamodelica::print(literal!("number of rows: "));
    metamodelica::print(mlen_str);
    metamodelica::print(literal!("\n"));
    m_1 = m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
    dumpAdjacencyMatrixEnhanced2(&m_1, 1)?;
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixTEnhanced(
    mut m: metamodelica::Array<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
) -> Result<()> {
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: metamodelica::List<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >;
    metamodelica::print(literal!("Transpose Adjacency Matrix Enhanced (row == var)\n"));
    metamodelica::print(literal!("=====================================\n"));
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = intString(mlen);
    metamodelica::print(literal!("number of rows: "));
    metamodelica::print(mlen_str);
    metamodelica::print(literal!("\n"));
    m_1 = m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
    dumpAdjacencyMatrixEnhanced2(&m_1, 1)?;
    Ok(())
}

fn dumpAdjacencyMatrixEnhanced2(
    mut inRows: &metamodelica::List<
        metamodelica::List<(
            i32,
            BackendDAE::Solvability,
            metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        )>,
    >,
    mut rowIndex: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inRows {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: row, tail: rows } => {
            metamodelica::print(intString(rowIndex));
            metamodelica::print(literal!(":"));
            dumpAdjacencyRowEnhanced(metamodelica::AsArg::as_arg(&row))?;
            dumpAdjacencyMatrixEnhanced2(rows, rowIndex + 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpAdjacencyRowEnhanced(
    mut inRow: &metamodelica::List<(
        i32,
        BackendDAE::Solvability,
        metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    )>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inRow {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::print(literal!("\n"));
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (x, solva, Deref @ metamodelica::ListNode::Nil), tail: xs } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = intString(x.clone());
            s1 = dumpSolvability(solva.clone());
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) });
            metamodelica::print(literal!(" "));
            dumpAdjacencyRowEnhanced(xs)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (x, solva, cons), tail: xs } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s = intString(x.clone());
            s1 = dumpSolvability(solva.clone());
            s2 = ExpressionDump::constraintDTlistToString(metamodelica::AsArg::as_arg(&cons), &(literal!(",")))?;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) });
            metamodelica::print(literal!(" "));
            dumpAdjacencyRowEnhanced(xs)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpSolvability(mut solva: BackendDAE::Solvability) -> ArcStr {
    let mut s: ArcStr;
    s = (match solva {
        BackendDAE::Solvability::SOLVABILITY_SOLVED { .. } => {
            literal!("solved")
        }
        BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. } => {
            literal!("constone")
        }
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut b } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("const("));
            __mm_s.push_str(&*boolString(b.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut b } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("param("));
            __mm_s.push_str(&*boolString(b.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: mut b } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("variable("));
            __mm_s.push_str(&*boolString(b.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. } => {
            literal!("nonlinear")
        }
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. } => {
            literal!("unsolvable")
        }
        BackendDAE::Solvability::SOLVABILITY_SOLVABLE { .. } => {
            literal!("solvable")
        }
    });
    s
}

pub(crate) fn dumpFullMatching(
    mut inMatch: &metamodelica::Ref<BackendDAE::Matching>,
    mut inSyst: Option<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<()> {
    let () = (match &**inMatch {
        BackendDAE::Matching::NO_MATCHING { .. } => {
            metamodelica::print(literal!("no matching\n"));
            ()
        }
        BackendDAE::Matching::MATCHING { ass1, ass2: _, comps } => {
            dumpMatching(ass1.clone())?;
            metamodelica::print(literal!("\n\n"));
            dumpComponents(comps.clone(), inSyst)?;
            ()
        }
    });
    Ok(())
}

pub fn dumpMatching(mut v: metamodelica::Array<i32>) -> Result<()> {
    let mut len: i32;
    let mut len_str: ArcStr;
    metamodelica::print(literal!("Matching\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    len = metamodelica::arrayLength(v.clone());
    len_str = intString(len);
    metamodelica::print(len_str);
    metamodelica::print(literal!(" variables and equations\n"));
    dumpMatching2(v.clone(), 1, len)?;
    Ok(())
}

fn dumpMatching2(mut v: metamodelica::Array<i32>, mut i: i32, mut len: i32) -> Result<()> {
    let __ab_v = v.borrow();
    for mut j in i..=len {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("var "));
            __mm_s.push_str(&*intString(j));
            __mm_s.push_str(&*literal!(" is solved in eqn "));
            __mm_s.push_str(&*intString((*metamodelica::index_checked(&__ab_v, j)?).clone()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn dumpMatchingVars(mut ass1: metamodelica::Array<i32>) -> Result<()> {
    let mut varIndex: i32 = 0;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nMatching\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(ass1.clone())));
        __mm_s.push_str(&*literal!(" variables\n"));
        ArcStr::from(__mm_s)
    });
    let __range0 = ass1.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        varIndex = varIndex + 1;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("var "));
            __mm_s.push_str(&*intString(varIndex));
            __mm_s.push_str(&*literal!(" is solved in eqn "));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn dumpMatchingEqns(mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut eqnIndex: i32 = 0;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nMatching\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(ass2.clone())));
        __mm_s.push_str(&*literal!(" equations\n"));
        ArcStr::from(__mm_s)
    });
    let __range0 = ass2.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqnIndex = eqnIndex + 1;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("eqn "));
            __mm_s.push_str(&*intString(eqnIndex));
            __mm_s.push_str(&*literal!(" is solved for var "));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn dumpMarkedEqns(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inIntegerLst: metamodelica::List<i32>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut slst: metamodelica::List<ArcStr>;
    let __arc1 = &(*syst);
    let BackendDAE::EQSYSTEM { orderedEqs: __pa0, .. } = &**__arc1;
    eqns = metamodelica::Own::own(__pa0);
    slst = List::map1(inIntegerLst, &dumpMarkedEqns1, eqns)?;
    outString = stringDelimitList(slst, literal!("\n"));
    Ok(outString)
}

fn dumpMarkedEqns1(
    mut index: i32,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    eqn = BackendEquation::get(eqns, index)?;
    outS = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  "));
        __mm_s.push_str(&*intString(index));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*equationString(&eqn)?);
        ArcStr::from(__mm_s)
    };
    Ok(outS)
}

pub(crate) fn dumpMarkedVarsLsts(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inIntegerLstLst: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    for mut inIntegerLst in &**inIntegerLstLst {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outString);
            __mm_s.push_str(&*dumpMarkedVars(syst, inIntegerLst.clone())?);
            __mm_s.push_str(&*literal!(","));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

pub(crate) fn dumpMarkedVars(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inIntegerLst: metamodelica::List<i32>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut vars: BackendDAE::Variables;
    let mut slst: metamodelica::List<ArcStr>;
    let __arc1 = &(*syst);
    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
    vars = metamodelica::Own::own(__pa0);
    slst = List::map1(
        inIntegerLst,
        &move |__a0: i32, __a1: BackendDAE::Variables| dumpMarkedVars1(__a0, &__a1),
        vars,
    )?;
    outString = stringDelimitList(slst, literal!("\n"));
    Ok(outString)
}

fn dumpMarkedVars1(mut index: i32, mut vars: &BackendDAE::Variables) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = BackendVariable::getVarAt(vars, index)?;
    outS = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  "));
        __mm_s.push_str(&*intString(index));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*varString(&var)?);
        ArcStr::from(__mm_s)
    };
    Ok(outS)
}

pub(crate) fn dumpMarkedVarList(
    mut varList: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut selList: &metamodelica::List<i32>,
) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    for mut sel in &**selList {
        if let Ok(__iflet0) = (varList).get(sel.clone()) {
            var = __iflet0;
        } else {
            Error::addInternalError(
                literal!("function dumpMarkedVarList failed"),
                metamodelica::sourceInfo!("BackEnd/BackendDump.mo"),
            )?;
            Error::addCompilerNotification({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Could not get variable "));
                __mm_s.push_str(&*intString(sel.clone()));
                __mm_s.push_str(&*literal!(" from varList \n"));
                __mm_s.push_str(&*varListString(varList, &(literal!("")))?);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outString);
            __mm_s.push_str(&*literal!("  "));
            __mm_s.push_str(&*varString(&var)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

pub(crate) fn dumpComponentsGraphStr(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut n: i32;
    let mut lst: metamodelica::List<ArcStr>;
    let mut s: ArcStr;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa4 @ Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa2, ass2: __pa3, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    mT = metamodelica::Own::own(__pa1);
    ass1 = metamodelica::Own::own(__pa2);
    ass2 = metamodelica::Own::own(__pa3);
    syst = metamodelica::Own::own(__pa4);
    n = BackendDAEUtil::systemSize(&syst)?;
    lst = dumpComponentsGraphStr2(1, n, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
    s = stringDelimitList(lst, literal!(","));
    s = stringAppendList(list![literal!("{"), s, literal!("}")]);
    metamodelica::print(s);
    outDAE = inDAE;
    Ok(outDAE)
}

fn dumpComponentsGraphStr2(
    mut i: i32,
    mut n: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut lst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut llst: metamodelica::List<metamodelica::List<i32>>;
    let mut eqns: metamodelica::List<i32>;
    let mut strLst: metamodelica::List<ArcStr>;
    let mut slst: metamodelica::List<ArcStr>;
    let mut r#str: ArcStr;
    if i <= n {
        eqns = Matching::reachableEquations(i, mT.clone(), ass2.clone())?;
        llst = List::map(eqns, &fnptr!(List::create, _))?;
        llst = List::map1(llst, &fnptr!(List::consr, _, _), i)?;
        slst = List::map(llst, &intListStr)?;
        r#str = stringDelimitList(slst, literal!(","));
        r#str = stringAppendList(list![literal!("{"), r#str, literal!("}")]);
        strLst = dumpComponentsGraphStr2(i + 1, n, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
        lst = metamodelica::cons(r#str, strLst);
    }
    Ok(lst)
}

pub(crate) fn dumpList(mut l: metamodelica::List<i32>, mut r#str: &ArcStr) -> Result<()> {
    let mut s: metamodelica::List<ArcStr>;
    let mut sl: ArcStr;
    s = List::map(l, &fnptr!(intString, i32))?;
    sl = stringDelimitList(s, literal!(", "));
    metamodelica::print(r#str.clone());
    metamodelica::print(sl);
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn dumpComponentsOLD(mut l: &metamodelica::List<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print(literal!("Blocks\n"));
    metamodelica::print(literal!("=======\n"));
    dumpComponents2(l, 1)?;
    Ok(())
}

fn dumpComponents2(
    mut inIntegerLstLst: &metamodelica::List<metamodelica::List<i32>>,
    mut inInteger: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inIntegerLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: lst } => {
            let mut i = inInteger;
            let mut i_1: i32;
            let mut ls: metamodelica::List<ArcStr>;
            let mut s: ArcStr;
            metamodelica::print(literal!("{"));
            ls = List::map(List::sort(l.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, &fnptr!(intString, i32))?;
            s = stringDelimitList(ls, literal!(", "));
            metamodelica::print(s);
            metamodelica::print(literal!("}\n"));
            i_1 = i + 1;
            dumpComponents2(lst, i_1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn intListStr(mut lst: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringDelimitList(List::map(lst, &fnptr!(intString, i32))?, literal!(","));
    res = stringAppendList(list![literal!("{"), res, literal!("}")]);
    Ok(res)
}

// protected function dumpAliasVariable
// "author: Frenkel TUD 2010-11"
//  input tuple<BackendDAE.Var,list<Integer>> inTpl;
//  output tuple<BackendDAE.Var,list<Integer>> outTpl;
// algorithm
//   outTpl:=
//   matchcontinue (inTpl)
//     local
//       BackendDAE.Var v;
//       DAE.ComponentRef cr;
//       DAE.Exp e;
//       String s,scr,se;
//     case ((v,_))
//       equation
//         cr = BackendVariable.varCref(v);
//         e = BackendVariable.varBindExp(v);
//         //print("### dump var : " +  ComponentReferenceBasics.printComponentRefStr(cr) + "\n");
//         scr = ComponentReferenceBasics.printComponentRefStr(cr);
//         se = ExpressionBasics.printExpStr(e);
//         s = stringAppendList({scr," = ",se,"\n"});
//         print(s);
//       then ((v,{}));
//     else inTpl;
//   end matchcontinue;
// end dumpAliasVariable;
pub(crate) fn dumpStateVariables(mut inVars: BackendDAE::Variables) -> Result<()> {
    metamodelica::print(literal!("States Variables\n"));
    metamodelica::print(literal!("=================\n"));
    BackendVariable::traverseBackendDAEVars(
        inVars,
        (std::sync::Arc::new(fnptr!(dumpStateVariable, metamodelica::Ref<BackendDAE::Var>, i32))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        i32,
                    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)>
                    + 'static,
            >),
        1,
    )?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn dumpStateVariable(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inPos: i32,
) -> (metamodelica::Ref<BackendDAE::Var>, i32) {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut pos: i32;
    (v, pos) = 'mc: {
        let __mc_input = (inVar.clone(), inPos);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, pos) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut scr: ArcStr;
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    scr = ComponentReferenceBasics::printComponentRefStr(&cr)?;
                    metamodelica::print(intString(pos.clone()));
                    metamodelica::print(literal!(": "));
                    metamodelica::print(scr.clone());
                    metamodelica::print(literal!("\n"));
                    Ok((v.clone(), pos.clone() + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inPos))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (v, pos)
}

pub(crate) fn bltdump(mut headerline: ArcStr, mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inDAE;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut strlow: ArcStr;
                    let Flags::STRING_FLAG { data: __pa0 } = (Flags::getConfigValue(Flags::DUMP_TARGET.clone())?) else { return Err("pattern mismatch") };
                    r#str = metamodelica::Own::own(__pa0);
                    strlow = System::tolower(r#str.clone());
                    let true = (intGt(System::stringFind(strlow.clone(), literal!(".html"))?, 0)) else { return Err("pattern mismatch") };
                    DumpHTML::dumpDAE(inDAE, headerline.clone(), &r#str)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs, shared } => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*headerline); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) });
                    List::map_0(metamodelica::AsArg::as_arg(&eqs), &printEqSystem)?;
                    metamodelica::print(literal!("\n"));
                    printShared(metamodelica::AsArg::as_arg(&shared))?;
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

pub(crate) fn innerEquationString(mut innerEquation: &BackendDAE::InnerEquation) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut e: i32;
    let mut v: metamodelica::List<i32>;
    (e, v, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation);
    s = stringDelimitList(List::map(v, &fnptr!(intString, i32))?, literal!(","));
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*intString(e));
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

pub type DumpCompShortSystemsTpl = (
    metamodelica::List<i32>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
);

pub type DumpCompShortMixedTpl = (
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<(i32, i32)>,
);

pub type DumpCompShortTornTpl = (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>);

pub(crate) fn dumpCompShort(mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    let mut sys: i32;
    let mut inp: i32;
    let mut st: i32;
    let mut dvar: i32;
    let mut dst: i32;
    let mut seq: i32;
    let mut salg: i32;
    let mut sarr: i32;
    let mut sce: i32;
    let mut swe: i32;
    let mut sie: i32;
    let mut eqsys: i32;
    let mut meqsys: i32;
    let mut teqsys: i32;
    let mut teqsys2: i32;
    let mut strcomps: i32;
    let mut e_jc: metamodelica::List<i32>;
    let mut e_jn: metamodelica::List<i32>;
    let mut e_nj: metamodelica::List<i32>;
    let mut te_l: metamodelica::List<(i32, i32, i32)>;
    let mut te_l2: metamodelica::List<(i32, i32, i32)>;
    let mut te_nl: metamodelica::List<(i32, i32)>;
    let mut te_nl2: metamodelica::List<(i32, i32)>;
    let mut m_se: metamodelica::List<i32>;
    let mut m_salg: metamodelica::List<i32>;
    let mut m_sarr: metamodelica::List<i32>;
    let mut m_sec: metamodelica::List<i32>;
    let mut me_jc: metamodelica::List<(i32, i32)>;
    let mut e_jt: metamodelica::List<(i32, i32)>;
    let mut me_jt: metamodelica::List<(i32, i32)>;
    let mut me_jn: metamodelica::List<(i32, i32)>;
    let mut me_nj: metamodelica::List<(i32, i32)>;
    let mut me_lt: metamodelica::List<(i32, i32)>;
    let mut me_nt: metamodelica::List<(i32, i32)>;
    let mut states: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut discvars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut discstates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut clockedstates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut HS: (
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
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut removedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut sysStr: ArcStr;
    let mut stStr: ArcStr;
    let mut dvarStr: ArcStr;
    let mut dstStr: ArcStr;
    let mut clckStr: ArcStr;
    let mut statesStr: ArcStr;
    let mut discvarsStr: ArcStr;
    let mut discstatesStr: ArcStr;
    let mut clockedstatesStr: ArcStr;
    let mut inpStr: ArcStr;
    let mut strcompsStr: ArcStr;
    let mut seqStr: ArcStr;
    let mut sarrStr: ArcStr;
    let mut salgStr: ArcStr;
    let mut sceStr: ArcStr;
    let mut sweStr: ArcStr;
    let mut sieStr: ArcStr;
    let mut eqsysStr: ArcStr;
    let mut teqsysStr: ArcStr;
    let mut meqsysStr: ArcStr;
    let mut daeType: ArcStr;
    let mut msgs: metamodelica::List<ArcStr>;
    let mut systemsTpl: DumpCompShortSystemsTpl;
    let mut mixedTpl: DumpCompShortMixedTpl;
    let mut tornTpl: DumpCompShortTornTpl;
    let mut tornTpl2: DumpCompShortTornTpl;
    let mut backendDAEType: BackendDAE::BackendDAEType;
    let __arc3 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __t2,
    } = &**__arc3;
    let __arc4 = __t2.clone();
    let BackendDAE::SHARED {
        backendDAEType: __pa1, ..
    } = &*__arc4;
    systs = metamodelica::Own::own(__pa0);
    backendDAEType = metamodelica::Own::own(__pa1);
    removedEqs = BackendDAEUtil::collapseRemovedEqs(inDAE)?;
    daeType = printBackendDAEType2String(backendDAEType)?;
    HS = HashSet::emptyHashSet();
    HS = List::fold(
        &systs,
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
        )| Initialization::collectPreVariablesEqSystem(&__a0, __a1),
        HS,
    )?;
    (_, HS) = BackendDAEUtil::traverseBackendDAEExpsEqns(
        removedEqs,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(Initialization::collectPreVariablesTraverseExp)
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
            HS,
        ),
    )?;
    discstates = BaseHashSet::hashSetList(&HS)?;
    dst = ((discstates).len() as i32);
    for mut syst in &*systs {
        clockedstates = BackendVariable::filterCrefs(
            syst.orderedVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isVarClockedState(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
            clockedstates,
        )?;
    }
    (
        sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, systemsTpl, mixedTpl, tornTpl, tornTpl2,
    ) = BackendDAEUtil::foldEqSystem(
        inDAE,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Ref<BackendDAE::Shared>,
               __a2: (
            i32,
            i32,
            i32,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            i32,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            (
                metamodelica::List<i32>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ),
            (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
            ),
            (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
            (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        )| dumpCompShort1(&__a0, &__a1, &__a2),
        (
            0,
            0,
            0,
            metamodelica::nil(),
            0,
            metamodelica::nil(),
            0,
            0,
            0,
            0,
            0,
            0,
            (
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
            ),
            (
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
            ),
            (metamodelica::nil(), metamodelica::nil()),
            (metamodelica::nil(), metamodelica::nil()),
        ),
    )?;
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl.clone();
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl.clone();
    (te_l, te_nl) = tornTpl.clone();
    (te_l2, te_nl2) = tornTpl2.clone();
    eqsys = ((e_jc).len() as i32) + ((e_jt).len() as i32) + ((e_jn).len() as i32) + ((e_nj).len() as i32);
    meqsys = ((m_se).len() as i32)
        + ((m_sarr).len() as i32)
        + ((m_salg).len() as i32)
        + ((m_sec).len() as i32)
        + ((me_jc).len() as i32)
        + ((me_jt).len() as i32)
        + ((me_jn).len() as i32)
        + ((me_nj).len() as i32)
        + ((me_lt).len() as i32)
        + ((me_nt).len() as i32);
    teqsys = ((te_l).len() as i32) + ((te_nl).len() as i32);
    teqsys2 = ((te_l2).len() as i32) + ((te_nl2).len() as i32);
    strcomps = seq + eqsys + meqsys + sarr + salg + sce + swe + sie + teqsys;
    sysStr = intString(sys);
    stStr = intString(st);
    dvarStr = intString(dvar);
    dstStr = intString(dst);
    clckStr = intString(((clockedstates).len() as i32));
    statesStr = if (Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())?) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*stringDelimitList(
                List::map(states, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!(" ('-d=stateselection' for list of states)")
    };
    discvarsStr = if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*stringDelimitList(
                List::map(discvars, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!(" ('-d=discreteinfo' for list of discrete vars)")
    };
    discstatesStr = if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*stringDelimitList(
                List::map(discstates, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!(" ('-d=discreteinfo' for list of discrete states)")
    };
    clockedstatesStr = if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*stringDelimitList(
                List::map(clockedstates, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                })?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!(" ('-d=discreteinfo' for list of clocked states)")
    };
    stStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*stStr);
        __mm_s.push_str(&*statesStr);
        ArcStr::from(__mm_s)
    };
    dvarStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dvarStr);
        __mm_s.push_str(&*discvarsStr);
        ArcStr::from(__mm_s)
    };
    dstStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dstStr);
        __mm_s.push_str(&*discstatesStr);
        ArcStr::from(__mm_s)
    };
    clckStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*clckStr);
        __mm_s.push_str(&*clockedstatesStr);
        ArcStr::from(__mm_s)
    };
    inpStr = intString(inp);
    msgs = list![daeType.clone(), sysStr, stStr, dvarStr, dstStr, clckStr, inpStr];
    Error::addMessage(Error::BACKENDDAEINFO_STATISTICS.clone(), msgs)?;
    strcompsStr = intString(strcomps);
    seqStr = intString(seq);
    sarrStr = intString(sarr);
    salgStr = intString(salg);
    sceStr = intString(sce);
    sweStr = intString(swe);
    sieStr = intString(sie);
    eqsysStr = intString(eqsys);
    teqsysStr = intString(teqsys);
    meqsysStr = intString(meqsys);
    msgs = list![
        daeType,
        strcompsStr,
        seqStr,
        sarrStr,
        salgStr,
        sceStr,
        sweStr,
        sieStr,
        eqsysStr,
        teqsysStr,
        meqsysStr
    ];
    Error::addMessage(Error::BACKENDDAEINFO_STRONGCOMPONENT_STATISTICS.clone(), msgs)?;
    if intGt(eqsys, 0) {
        dumpCompSystems(&systemsTpl)?;
    }
    if intGt(meqsys, 0) {
        dumpCompMixed(&mixedTpl)?;
    }
    if intGt(teqsys, 0) {
        dumpCompTorn(&tornTpl, literal!("strict"))?;
    }
    if intGt(teqsys2, 0) && !(stringEqual(&(Config::dynamicTearing()?), &(literal!("false")))) {
        dumpCompTorn(&tornTpl2, literal!("casual"))?;
    }
    Ok(())
}

fn dumpCompSystems(mut systemsTpl: &DumpCompShortSystemsTpl) -> Result<()> {
    let mut e_jc: metamodelica::List<i32>;
    let mut e_jn: metamodelica::List<i32>;
    let mut e_nj: metamodelica::List<i32>;
    let mut e_jt: metamodelica::List<(i32, i32)>;
    let mut s_jc: ArcStr;
    let mut s_jn: ArcStr;
    let mut s_nj: ArcStr;
    let mut s_jt: ArcStr;
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl.clone();
    s_jc = equationSizesStr(e_jc, &fnptr!(intString, i32))?;
    s_jt = equationSizesStr(e_jt, &sizeNumNonZeroTplString)?;
    s_jn = equationSizesStr(e_jn, &fnptr!(intString, i32))?;
    s_nj = equationSizesStr(e_nj, &fnptr!(intString, i32))?;
    Error::addMessage(Error::BACKENDDAEINFO_SYSTEMS.clone(), list![s_jc, s_jt, s_jn, s_nj])?;
    Ok(())
}

fn dumpCompTorn(mut systemsTpl: &DumpCompShortTornTpl, mut whichset: ArcStr) -> Result<()> {
    let mut te_l: metamodelica::List<(i32, i32, i32)>;
    let mut te_nl: metamodelica::List<(i32, i32)>;
    let mut s_l: ArcStr;
    let mut s_nl: ArcStr;
    (te_l, te_nl) = systemsTpl.clone();
    s_l = equationSizesStr(te_l, &sizeNumNonZeroTornTplString)?;
    s_nl = equationSizesStr(te_nl, &fnptr!(intTplString, (i32, i32)))?;
    Error::addMessage(Error::BACKENDDAEINFO_TORN.clone(), list![whichset, s_l, s_nl])?;
    Ok(())
}

fn dumpCompMixed(mut mixedTpl: &DumpCompShortMixedTpl) -> Result<()> {
    let mut m_se: metamodelica::List<i32>;
    let mut m_salg: metamodelica::List<i32>;
    let mut m_sarr: metamodelica::List<i32>;
    let mut m_sec: metamodelica::List<i32>;
    let mut me_jc: metamodelica::List<(i32, i32)>;
    let mut me_jt: metamodelica::List<(i32, i32)>;
    let mut me_jn: metamodelica::List<(i32, i32)>;
    let mut me_nj: metamodelica::List<(i32, i32)>;
    let mut me_lt: metamodelica::List<(i32, i32)>;
    let mut me_nt: metamodelica::List<(i32, i32)>;
    let mut s_se: ArcStr;
    let mut s_salg: ArcStr;
    let mut s_sarr: ArcStr;
    let mut s_sec: ArcStr;
    let mut s_jc: ArcStr;
    let mut s_jt: ArcStr;
    let mut s_jn: ArcStr;
    let mut s_nj: ArcStr;
    let mut s_lt: ArcStr;
    let mut s_nt: ArcStr;
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl.clone();
    s_se = equationSizesStr(m_se, &fnptr!(intString, i32))?;
    s_salg = equationSizesStr(m_salg, &fnptr!(intString, i32))?;
    s_sarr = equationSizesStr(m_sarr, &fnptr!(intString, i32))?;
    s_sec = equationSizesStr(m_sec, &fnptr!(intString, i32))?;
    s_jc = equationSizesStr(me_jc, &fnptr!(intTplString, (i32, i32)))?;
    s_jt = equationSizesStr(me_jt, &fnptr!(intTplString, (i32, i32)))?;
    s_jn = equationSizesStr(me_jn, &fnptr!(intTplString, (i32, i32)))?;
    s_nj = equationSizesStr(me_nj, &fnptr!(intTplString, (i32, i32)))?;
    s_lt = equationSizesStr(me_lt, &fnptr!(intTplString, (i32, i32)))?;
    s_nt = equationSizesStr(me_nt, &fnptr!(intTplString, (i32, i32)))?;
    Error::addMessage(
        Error::BACKENDDAEINFO_MIXED.clone(),
        list![s_se, s_salg, s_sarr, s_sec, s_jc, s_jt, s_jn, s_nj, s_lt, s_nt],
    )?;
    Ok(())
}

fn equationSizesStr<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eqs: metamodelica::List<A>,
    mut r#fn: &dyn ::std::ops::Fn(A) -> Result<ArcStr>,
) -> Result<ArcStr> {
    pub type AToStr<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(A) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    let mut len: i32;
    len = ((eqs).len() as i32);
    r#str = if (len == 1) {
        literal!("1 system")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(len));
            __mm_s.push_str(&*literal!(" systems"));
            ArcStr::from(__mm_s)
        }
    };
    r#str = if (len == 0) {
        r#str
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n   {"));
            __mm_s.push_str(&*stringDelimitList(List::map(eqs, r#fn)?, literal!(", ")));
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        }
    };
    Ok(r#str)
}

fn sizeNumNonZeroTplString(mut inTpl: (i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut sz: i32;
    let mut nnz: i32;
    let mut density: metamodelica::Real;
    (sz, nnz) = inTpl;
    density = realDiv(
        (metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz)),
        (intReal(sz)) * (intReal(sz)),
    );
    r#str = System::snprintff(literal!("%.1f"), 20, density)?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(sz));
        __mm_s.push_str(&*literal!(","));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("%)"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn sizeNumNonZeroTornTplString(mut inTpl: (i32, i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut sz: i32;
    let mut nnz: i32;
    let mut others: i32;
    let mut density: metamodelica::Real;
    (sz, others, nnz) = inTpl;
    density = if (nnz == 0) {
        metamodelica::OrderedFloat(0.0_f64)
    } else {
        realDiv(
            (metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz)),
            (intReal(sz)) * (intReal(sz)),
        )
    };
    r#str = System::snprintff(literal!("%.1f"), 20, density)?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(sz));
        __mm_s.push_str(&*literal!(","));
        __mm_s.push_str(&*intString(others));
        __mm_s.push_str(&*literal!(","));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("%)"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn intTplString(mut inTpl: (i32, i32)) -> ArcStr {
    let mut outStr: ArcStr;
    let mut e: i32;
    let mut d: i32;
    (d, e) = inTpl;
    outStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(d));
        __mm_s.push_str(&*literal!(","));
        __mm_s.push_str(&*intString(e));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    outStr
}

fn dumpCompShort1(
    mut inSyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: &(
        i32,
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        (
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
        ),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    ),
) -> Result<(
    i32,
    i32,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
    ),
    (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
)> {
    let mut outTpl: (
        i32,
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        (
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
        ),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    );
    let mut vars: BackendDAE::Variables;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut sys: i32;
    let mut inp: i32;
    let mut st: i32;
    let mut dvar: i32;
    let mut seq: i32;
    let mut salg: i32;
    let mut sarr: i32;
    let mut sce: i32;
    let mut swe: i32;
    let mut sie: i32;
    let mut inp1: i32;
    let mut st1: i32;
    let mut dvar1: i32;
    let mut seq1: i32;
    let mut salg1: i32;
    let mut sarr1: i32;
    let mut sce1: i32;
    let mut swe1: i32;
    let mut sie1: i32;
    let mut eqsys: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut eqsys1: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut meqsys: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
    );
    let mut meqsys1: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
    );
    let mut teqsys: (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>);
    let mut teqsys1: (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>);
    let mut teqsys_2: (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>);
    let mut teqsys1_2: (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>);
    let mut states: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut states1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut discvars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut discvars1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let __arc1 = &(*inSyst);
    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
    vars = metamodelica::Own::own(__pa0);
    (
        sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys_2,
    ) = inTpl.clone();
    (inp1, st1, states1, dvar1, discvars1) = BackendVariable::traverseBackendDAEVars(
        vars,
        (std::sync::Arc::new(fnptr!(
            traversingisStateTopInputVarFinder,
            metamodelica::Ref<BackendDAE::Var>,
            (
                i32,
                i32,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                i32,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            i32,
                            i32,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                            i32,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            i32,
                            i32,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                            i32,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        ),
                    )> + 'static,
            >),
        (inp, st, states, dvar, discvars),
    )?;
    comps = BackendDAEUtil::getStrongComponents(inSyst);
    (
        seq1, salg1, sarr1, sce1, swe1, sie1, eqsys1, meqsys1, teqsys1, teqsys1_2,
    ) = List::fold(
        &comps,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: (
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            (
                metamodelica::List<i32>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ),
            (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
                metamodelica::List<(i32, i32)>,
            ),
            (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
            (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        )| dumpCompShort2(&__a0, &__a1),
        (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys_2),
    )?;
    outTpl = (
        sys + 1,
        inp1,
        st1,
        states1,
        dvar1,
        discvars1,
        seq1,
        salg1,
        sarr1,
        sce1,
        swe1,
        sie1,
        eqsys1,
        meqsys1,
        teqsys1,
        teqsys1_2,
    );
    Ok(outTpl)
}

fn traversingisStateTopInputVarFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        i32,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    (outVar, outTpl) = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (v, (inp, st, states, dvar, discvars)) if (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&v))) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
            (v.clone(), (inp.clone(), st.clone() + 1, metamodelica::cons(cr, states.clone()), dvar.clone(), discvars.clone()))
        },
        (v, (inp, st, states, dvar, discvars)) if (BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&v))) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
            (v.clone(), (inp.clone(), st.clone(), states.clone(), dvar.clone() + 1, metamodelica::cons(cr, discvars.clone())))
        },
        (v, (inp, st, states, dvar, discvars)) if (BackendVariable::isVarOnTopLevelAndInput(metamodelica::AsArg::as_arg(&v))) => {
            (v.clone(), (inp.clone() + 1, st.clone(), states.clone(), dvar.clone(), discvars.clone()))
        },
        _ => {
            (inVar, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVar, outTpl)
}

fn dumpCompShort2(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inTpl: &(
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        (
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
        ),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    ),
) -> Result<(
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
    ),
    (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
)> {
    let mut outTpl: (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        (
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<(i32, i32)>,
        ),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
        (metamodelica::List<(i32, i32, i32)>, metamodelica::List<(i32, i32)>),
    );
    outTpl = (::match_deref::match_deref! { match &((&**inComp, inTpl)) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone() + 1, salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone() + 1, sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone() + 1, eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone() + 1, sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone() + 1, swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone() + 1, sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_CONSTANT { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = ((ilst).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (metamodelica::cons(e, e_jc.clone()), e_jt.clone(), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            let mut nnz: i32;
            e = ((ilst).len() as i32);
            nnz = ((jac).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), metamodelica::cons((e, nnz), e_jt.clone()), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = ((ilst).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), metamodelica::cons(e, e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_GENERIC { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = ((ilst).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), metamodelica::cons(e, e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = ((ilst).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), e_jn.clone(), metamodelica::cons(e, e_nj.clone())), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, .. }, casualTearingSet: None, linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = ((ilst).len() as i32);
            e = ((innerEquations).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d, e, nnz.clone()), te_l.clone()), te_nl.clone()), (metamodelica::cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, .. }, casualTearingSet: None, linear: false, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = ((ilst).len() as i32);
            e = ((innerEquations).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), metamodelica::cons((d, e), te_nl.clone())), (te_l2.clone(), metamodelica::cons((0, 0), te_nl2.clone())))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. }, .. }, casualTearingSet: None, linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = ((ilst).len() as i32);
            e = ((innerEquations).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d, e, 0), te_l.clone()), te_nl.clone()), (metamodelica::cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: ilst2, innerEquations: innerEquations2, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz2), coloring: _, .. }, .. }), linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            let mut e2: i32;
            let mut d2: i32;
            d = ((ilst).len() as i32);
            e = ((innerEquations).len() as i32);
            d2 = ((ilst2).len() as i32);
            e2 = ((innerEquations2).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d, e, nnz.clone()), te_l.clone()), te_nl.clone()), (metamodelica::cons((d2, e2, nnz2.clone()), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: ilst2, innerEquations: innerEquations2, .. }), linear: false, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            let mut e2: i32;
            let mut d2: i32;
            d = ((ilst).len() as i32);
            e = ((innerEquations).len() as i32);
            d2 = ((ilst2).len() as i32);
            e2 = ((innerEquations2).len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), metamodelica::cons((d, e), te_nl.clone())), (te_l2.clone(), metamodelica::cons((d2, e2), te_nl2.clone())))
        },
        _ => {
            metamodelica::print(literal!("dumpCompShort2 failed with:\n"));
            dumpComponent(inComp, None)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

pub(crate) fn dumpNrOfEquations(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut preStr: &ArcStr,
) -> Result<()> {
    let mut nlst: metamodelica::List<i32>;
    let mut n: i32;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*inDAE);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    systs = metamodelica::Own::own(__pa0);
    nlst = List::map(systs, &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| {
        BackendDAEUtil::systemSize(&__a0)
    })?;
    n = List::fold(&nlst, &fnptr!(intAdd, i32, i32), 0)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*preStr);
        __mm_s.push_str(&*literal!(" NrOfEquations: "));
        __mm_s.push_str(&*intString(n));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn dumpCompInfo(mut compInfo: &metamodelica::Ref<BackendDAE::CompInfo>) -> Result<()> {
    metamodelica::print(printCompInfo(compInfo));
    Ok(())
}

fn printCompInfo(mut compInfo: &metamodelica::Ref<BackendDAE::CompInfo>) -> ArcStr {
    let mut sOut: ArcStr;
    sOut = 'mc: {
        let __mc_input = &**compInfo;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::COUNTER { comp, numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
                    let mut s: ArcStr;
                    s = literal!("");
                    if BackendDAEUtil::isSingleEquationComp(metamodelica::AsArg::as_arg(&comp)) {
                        s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SE ")); __mm_s.push_str(&*printComponent(metamodelica::AsArg::as_arg(&comp), None)?); ArcStr::from(__mm_s) };
                    } else if BackendDAEUtil::isWhenComp(metamodelica::AsArg::as_arg(&comp)) {
                        s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WE ")); __mm_s.push_str(&*printComponent(metamodelica::AsArg::as_arg(&comp), None)?); ArcStr::from(__mm_s) };
                    } else if BackendDAEUtil::isArrayComp(metamodelica::AsArg::as_arg(&comp)) {
                        s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AE ")); __mm_s.push_str(&*printComponent(metamodelica::AsArg::as_arg(&comp), None)?); ArcStr::from(__mm_s) };
                    }
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::SYSTEM { allOperations: allOps, comp, size, density: dens } => {
                    let mut s: ArcStr;
                    s = literal!("");
                    if BackendDAEUtil::isLinearEqSystemComp(metamodelica::AsArg::as_arg(&comp)) {
                        s = literal!("LSYS");
                    } else {
                        s = literal!("NLSYS");
                    }
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*printComponent(metamodelica::AsArg::as_arg(&comp), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\tdens|")); __mm_s.push_str(&*intString(((dens.clone() * metamodelica::OrderedFloat(100.0_f64)).0.floor() as i32))); __mm_s.push_str(&*printCompInfo(metamodelica::AsArg::as_arg(&allOps))); ArcStr::from(__mm_s) };
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::TORN_ANALYSE { tornEqs, otherEqs, comp, tornSize: size } => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TS ")); __mm_s.push_str(&*printComponent(metamodelica::AsArg::as_arg(&comp), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\tthe torn eqs:\t")); __mm_s.push_str(&*printCompInfo(metamodelica::AsArg::as_arg(&tornEqs))); ArcStr::from(__mm_s) };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\tthe other eqs:\t")); __mm_s.push_str(&*printCompInfo(metamodelica::AsArg::as_arg(&otherEqs))); ArcStr::from(__mm_s) };
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::NO_COMP { numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
                    let mut s: ArcStr;
                    s = literal!("NC");
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Dont know this compInfo\n"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    sOut
}

// =============================================================================
// section for all html-dumping functions
//
// =============================================================================
pub(crate) fn dumpEqSystemMatrixHTML(mut sys: metamodelica::Ref<BackendDAE::EqSystem>) -> Result<()> {
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    if (sys.m).is_some() {
        m = sys.m.clone().ok_or("pattern mismatch")?;
    } else {
        (_, m, _) = BackendDAEUtil::getAdjacencyMatrix(
            sys.clone(),
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            None,
            false,
        )?;
    }
    dumpEqSystem(sys.clone(), &(literal!("SYS")))?;
    dumpMatrixHTML(
        m.clone(),
        &(List::map(
            List::intRange(BackendDAEUtil::systemSize(&sys)?),
            &fnptr!(intString, i32),
        )?),
        &(List::map(
            BackendVariable::varList(&sys.orderedVars)?,
            &move |__a0: metamodelica::Ref<BackendDAE::Var>| varStringShort(&__a0),
        )?),
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("MATRIX_"));
            __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(&sys)?));
            ArcStr::from(__mm_s)
        }),
    )?;
    Ok(())
}

pub(crate) fn dumpEqSystemBLTmatrixHTML(mut sys: &metamodelica::Ref<BackendDAE::EqSystem>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**sys;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, m: _, mT: _, mapping: _, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, stateSets: _, partitionKind: _, removedEqs: _ } => {
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut vIdxs: metamodelica::List<i32>;
                    let mut eIdxs: metamodelica::List<i32>;
                    let mut vars = (*vars).clone();
                    let mut eqs = (*eqs).clone();
                    (varLst, vIdxs, eqLst, eIdxs) = BackendDAEUtil::getStrongComponentsVarsAndEquations(metamodelica::AsArg::as_arg(&comps), vars.clone(), eqs.clone())?;
                    eqs = BackendEquation::listEquation(&eqLst)?;
                    vars = BackendVariable::listVar1(&varLst)?;
                    (m, _) = BackendDAEUtil::adjacencyMatrixDispatch(metamodelica::AsArg::as_arg(&vars), eqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, false)?;
                    dumpMatrixHTML(m.clone(), &(List::map(eIdxs.clone(), &fnptr!(intString, i32))?), &(List::map(vIdxs.clone(), &fnptr!(intString, i32))?), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLT_MATRIX_")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(sys)?)); ArcStr::from(__mm_s) }))?;
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
                    metamodelica::print(literal!("dumpEqSystemBLTmatrixHTML does not output anything since there is no BLT sorting."));
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

pub(crate) fn dumpMatrixHTML(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut rowNames: &metamodelica::List<ArcStr>,
    mut columNames: &metamodelica::List<ArcStr>,
    mut fileName: &ArcStr,
) -> Result<()> {
    let mut size: i32;
    size = metamodelica::arrayLength(m.clone());
    if ((rowNames).len() as i32) == size && ((columNames).len() as i32) == size {
        DumpHTML::dumpMatrixHTML(m.clone(), rowNames, columNames, fileName)?;
    } else {
        DumpHTML::dumpMatrixHTML(
            m.clone(),
            &(List::fill(literal!("?"), size)),
            &(List::fill(literal!("?"), size)),
            fileName,
        )?;
    }
    Ok(())
}

// =============================================================================
// section for all graphML dumping functions
//
// =============================================================================
pub(crate) fn dumpBipartiteGraphDAE(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut fileName: &ArcStr,
) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
    let __arc2 = &(*dae);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqSysts = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    eqLst = List::flatten(List::mapMap(
        eqSysts.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendEquation::getEqnsFromEqSystem(&__a0))
        },
        &BackendEquation::equationList,
    )?)?;
    varLst = List::flatten(List::mapMap(
        eqSysts,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::daeVars(&__a0))
        },
        &move |__a0: BackendDAE::Variables| BackendVariable::varList(&__a0),
    )?)?;
    vars = BackendVariable::listVar1(&varLst)?;
    eqs = BackendEquation::listEquation(&eqLst)?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        metamodelica::Ref::new(BackendDAE::EqSystem {
            orderedVars: vars.clone(),
            orderedEqs: eqs.clone(),
            m: None,
            mT: None,
            mapping: None,
            matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(),
            stateSets: metamodelica::nil(),
            partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            removedEqs: BackendEquation::emptyEqns(),
        }),
        openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
        Some(BackendDAEUtil::getFunctions(&shared)),
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    varAtts = List::threadMap(
        List::fill(false, ((varLst).len() as i32)),
        List::fill(literal!(""), ((varLst).len() as i32)),
        &fnptr!(Util::makeTuple, _, _),
    )?;
    eqAtts = List::threadMap(
        List::fill(false, ((eqLst).len() as i32)),
        List::fill(literal!(""), ((eqLst).len() as i32)),
        &fnptr!(Util::makeTuple, _, _),
    )?;
    dumpBipartiteGraphStrongComponent2(
        vars,
        eqs,
        m.clone(),
        varAtts,
        &eqAtts,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("BipartiteGraph_"));
            __mm_s.push_str(&*fileName);
            ArcStr::from(__mm_s)
        }),
    )?;
    Ok(())
}

pub(crate) fn dumpBipartiteGraphEqSystem(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut fileName: &ArcStr,
) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mO: Option<metamodelica::Array<metamodelica::List<i32>>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
    let __arc3 = syst.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        m: __pa2,
        ..
    } = &*__arc3;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    mO = metamodelica::Own::own(__pa2);
    varLst = BackendVariable::varList(&vars)?;
    varAtts = List::threadMap(
        List::fill(false, ((varLst).len() as i32)),
        List::fill(literal!(""), ((varLst).len() as i32)),
        &fnptr!(Util::makeTuple, _, _),
    )?;
    eqAtts = List::threadMap(
        List::fill(false, BackendEquation::equationArraySize(eqs.clone())?),
        List::fill(literal!(""), BackendEquation::equationArraySize(eqs.clone())?),
        &fnptr!(Util::makeTuple, _, _),
    )?;
    if (mO).is_some() {
        dumpBipartiteGraphStrongComponent2(
            vars,
            eqs,
            mO.ok_or("pattern mismatch")?,
            varAtts,
            &eqAtts,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("BipartiteGraph_"));
                __mm_s.push_str(&*fileName);
                ArcStr::from(__mm_s)
            }),
        )?;
    } else {
        (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
            syst,
            openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE,
            Some(BackendDAEUtil::getFunctions(shared)),
            BackendDAEUtil::isInitializationDAE(shared),
        )?;
        dumpBipartiteGraphStrongComponent2(
            vars,
            eqs,
            m.clone(),
            varAtts,
            &eqAtts,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("BipartiteGraph2_"));
                __mm_s.push_str(&*fileName);
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok(())
}

pub(crate) fn dumpBipartiteGraphStrongComponent(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut eqSys: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut funcs: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut name: &ArcStr,
) -> Result<()> {
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let __arc2 = &(*eqSys);
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &**__arc2;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    varLst = BackendVariable::varList(&vars)?;
    eqLst = BackendEquation::equationList(eqs)?;
    dumpBipartiteGraphStrongComponent1(inComp, eqLst, varLst, funcs, name)?;
    Ok(())
}

pub(crate) fn dumpBipartiteGraphStrongComponent1(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut eqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut varsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut funcs: Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    mut graphName: &ArcStr,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcs, vars: varIdcs, .. } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut compEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut compVars: BackendDAE::Variables;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut compEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut compVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    compEqLst = List::map1(eqIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), eqsIn.clone())?;
                    compVarLst = List::map1(varIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), varsIn.clone())?;
                    compVars = BackendVariable::listVar1(&compVarLst)?;
                    compEqs = BackendEquation::listEquation(&compEqLst)?;
                    numEqs = ((compEqLst).len() as i32);
                    numVars = ((compVarLst).len() as i32);
                    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(metamodelica::Ref::new(BackendDAE::EqSystem { orderedVars: compVars.clone(), orderedEqs: compEqs.clone(), m: None, mT: None, mapping: None, matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(), stateSets: metamodelica::nil(), partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, funcs.clone(), false)?;
                    varAtts = List::threadMap(List::fill(false, numVars), List::fill(literal!(""), numVars), &fnptr!(Util::makeTuple, _, _))?;
                    eqAtts = List::threadMap(List::fill(false, numEqs), List::fill(literal!(""), numEqs), &fnptr!(Util::makeTuple, _, _))?;
                    dumpBipartiteGraphStrongComponent2(compVars.clone(), compEqs.clone(), m.clone(), varAtts.clone(), &eqAtts, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_eqSys_")); __mm_s.push_str(&*graphName); ArcStr::from(__mm_s) }))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: rEqIdcs, tearingvars: tVarIdcs, innerEquations, .. }, .. } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut tornInfo: metamodelica::List<bool>;
                    let mut addInfo: metamodelica::List<ArcStr>;
                    let mut eqIdcs: metamodelica::List<i32>;
                    let mut varIdcs: metamodelica::List<i32>;
                    let mut tVarIdcsNew: metamodelica::List<i32>;
                    let mut rEqIdcsNew: metamodelica::List<i32>;
                    let mut varIdcsLst: metamodelica::List<metamodelica::List<i32>>;
                    let mut varAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut eqAtts: metamodelica::List<(bool, ArcStr)>;
                    let mut compEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut compVars: BackendDAE::Variables;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut compEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut compVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (eqIdcs, varIdcsLst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    varIdcs = List::flatten(varIdcsLst.clone())?;
                    eqIdcs = listAppend(eqIdcs.clone(), rEqIdcs.clone());
                    varIdcs = listAppend(varIdcs.clone(), tVarIdcs.clone());
                    compEqLst = List::map1(eqIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), eqsIn.clone())?;
                    compVarLst = List::map1(varIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), varsIn.clone())?;
                    compVars = BackendVariable::listVar1(&compVarLst)?;
                    compEqs = BackendEquation::listEquation(&compEqLst)?;
                    numEqs = ((compEqLst).len() as i32);
                    numVars = ((compVarLst).len() as i32);
                    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(metamodelica::Ref::new(BackendDAE::EqSystem { orderedVars: compVars.clone(), orderedEqs: compEqs.clone(), m: None, mT: None, mapping: None, matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(), stateSets: metamodelica::nil(), partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, funcs.clone(), false)?;
                    addInfo = List::map(varIdcs.clone(), &fnptr!(intString, i32))?;
                    tornInfo = List::fill(true, numVars);
                    tVarIdcsNew = List::intRange(numVars - ((tVarIdcs).len() as i32));
                    tornInfo = List::fold1(&tVarIdcsNew, &List::replaceAtIndexFirst, false, tornInfo.clone())?;
                    varAtts = List::threadMap(tornInfo.clone(), addInfo.clone(), &fnptr!(Util::makeTuple, _, _))?;
                    addInfo = List::map(eqIdcs.clone(), &fnptr!(intString, i32))?;
                    tornInfo = List::fill(true, numEqs);
                    rEqIdcsNew = List::intRange(numEqs - ((rEqIdcs).len() as i32));
                    tornInfo = List::fold1(&rEqIdcsNew, &List::replaceAtIndexFirst, false, tornInfo.clone())?;
                    eqAtts = List::threadMap(tornInfo.clone(), addInfo.clone(), &fnptr!(Util::makeTuple, _, _))?;
                    dumpBipartiteGraphStrongComponent2(compVars.clone(), compEqs.clone(), m.clone(), varAtts.clone(), &eqAtts, graphName)?;
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
                    metamodelica::print(literal!("dumpTornSystemBipartiteGraphML1 failed\n"));
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

pub(crate) fn dumpBipartiteGraphStrongComponent2(
    mut varsIn: BackendDAE::Variables,
    mut eqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut varAtts: metamodelica::List<(bool, ArcStr)>,
    mut eqAtts: &metamodelica::List<(bool, ArcStr)>,
    mut name: &ArcStr,
) -> Result<()> {
    let mut nameAttIdx: i32;
    let mut typeAttIdx: i32;
    let mut idxAttIdx: i32;
    let mut numVars: i32;
    let mut numEqs: i32;
    let mut varRange: metamodelica::List<i32>;
    let mut eqRange: metamodelica::List<i32>;
    let mut graphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32;
    numEqs = BackendEquation::equationArraySize(eqsIn.clone())?;
    numVars = BackendVariable::varsSize(&varsIn);
    varRange = List::intRange(numVars);
    eqRange = List::intRange(numEqs);
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("EqSystemGraph"), true, graphInfo)?;
    graphInfo = metamodelica::Own::own(__pa0);
    graphIdx = metamodelica::Own::own(__pa1);
    let (__pa2, (_, __pa3)) = GraphML::addAttribute(
        literal!(""),
        literal!("type"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa2);
    typeAttIdx = metamodelica::Own::own(__pa3);
    let (__pa4, (_, __pa5)) = GraphML::addAttribute(
        literal!(""),
        literal!("name"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa4);
    nameAttIdx = metamodelica::Own::own(__pa5);
    let (__pa6, (_, __pa7)) = GraphML::addAttribute(
        literal!(""),
        literal!("systIdx"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa6);
    idxAttIdx = metamodelica::Own::own(__pa7);
    (graphInfo, graphIdx) = addEqNodesToGraph(
        eqsIn,
        eqAtts,
        &(list![nameAttIdx, typeAttIdx, idxAttIdx]),
        &((graphInfo, graphIdx)),
    )?;
    (graphInfo, graphIdx) = List::fold3(
        &varRange,
        &move |__a0: i32,
               __a1: BackendDAE::Variables,
               __a2: metamodelica::List<(bool, ArcStr)>,
               __a3: metamodelica::List<i32>,
               __a4: (GraphML::GraphInfo, i32)| addVarNodeToGraph(__a0, &__a1, &__a2, &__a3, &__a4),
        varsIn,
        varAtts,
        list![nameAttIdx, typeAttIdx, idxAttIdx],
        (graphInfo, graphIdx),
    )?;
    graphInfo = List::fold1(&eqRange, &addEdgeToGraph, mIn.clone(), graphInfo)?;
    GraphML::dumpGraph(graphInfo, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!(".graphml"));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn addEqNodesToGraph(
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut attsIn: &metamodelica::List<(bool, ArcStr)>,
    mut attributeIdcs: &metamodelica::List<i32>,
    mut graphInfoIn: &(GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut isResEq: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttrIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut size: i32;
    let mut numEqs: i32;
    let mut e: i32;
    let mut eAbs: i32;
    let mut nextE: i32;
    let mut eqString: ArcStr;
    let mut eqNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    nameAttrIdx = (attributeIdcs).get(1)?;
    typeAttrIdx = (attributeIdcs).get(2)?;
    idxAttrIdx = (attributeIdcs).get(3)?;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    numEqs = BackendEquation::getNumberOfEquations(eqs.clone());
    e = 1;
    eAbs = 1;
    size = 1;
    while e <= numEqs {
        eq = BackendEquation::get(eqs.clone(), e)?;
        size = BackendEquation::equationSize(&eq)?;
        nextE = eAbs + size;
        while nextE > eAbs {
            nameAttrIdx = (attributeIdcs).get(1)?;
            typeAttrIdx = (attributeIdcs).get(2)?;
            idxAttrIdx = (attributeIdcs).get(3)?;
            isResEq = Util::tuple21((attsIn).get(e)?);
            daeIdxStr = Util::tuple22((attsIn).get(e)?);
            typeStr = if (isResEq) {
                literal!("residualEq")
            } else {
                literal!("otherEq")
            };
            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![e], eqs.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            eq = metamodelica::Own::own(__pa0);
            eqString = equationString(&eq)?;
            eqNodeId = getEqNodeIdx(eAbs);
            idxString = intString(eAbs);
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: idxString,
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (graphInfo, _) = GraphML::addNode(
                eqNodeId,
                arcstr::literal!(GraphML::COLOR_GREEN2),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![nodeLabel],
                openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
                Some(eqString.clone()),
                list![(nameAttrIdx, eqString), (typeAttrIdx, typeStr), (idxAttrIdx, daeIdxStr)],
                graphIdx,
                graphInfo,
            )?;
            eAbs = eAbs + 1;
            size = size - 1;
        }
        e = e + 1;
    }
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

pub(crate) fn dumpDAGStrongComponent(
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut fileName: &ArcStr,
) -> Result<()> {
    let mut graphIdx: i32;
    let mut nameAttIdx: i32;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("TornSystemGraph"), true, graphInfo)?;
    graphInfo = metamodelica::Own::own(__pa0);
    graphIdx = metamodelica::Own::own(__pa1);
    let (__pa2, (_, __pa3)) = GraphML::addAttribute(
        literal!(""),
        literal!("Name"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa2);
    nameAttIdx = metamodelica::Own::own(__pa3);
    graphInfo = buildGraphInfoDAG(graphIn.clone(), metaIn, graphInfo, graphIdx, &(list![nameAttIdx]))?;
    GraphML::dumpGraph(graphInfo, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fileName);
        __mm_s.push_str(&*literal!(".graphml"));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn buildGraphInfoDAG(
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut graphInfoIn: GraphML::GraphInfo,
    mut graphIdx: i32,
    mut attIdcs: &metamodelica::List<i32>,
) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut nodeIdcs: metamodelica::List<i32>;
    let mut nodes: metamodelica::List<GraphML::Node>;
    let mut nameAttIdx: i32;
    nameAttIdx = (attIdcs).head().cloned()?;
    nodeIdcs = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    graphInfoOut = List::fold4(
        &nodeIdcs,
        &move |__a0: i32,
               __a1: metamodelica::Array<metamodelica::List<i32>>,
               __a2: HpcOmTaskGraph::TaskGraphMeta,
               __a3: i32,
               __a4: metamodelica::List<i32>,
               __a5: GraphML::GraphInfo| addNodeToDAG(__a0, __a1, __a2, __a3, &__a4, __a5),
        graphIn.clone(),
        metaIn,
        graphIdx,
        list![nameAttIdx],
        graphInfoIn,
    )?;
    let GraphML::GRAPHINFO { nodes: __pa0, .. } = (graphInfoOut.clone()) else {
        return Err("pattern mismatch");
    };
    nodes = metamodelica::Own::own(__pa0);
    Ok(graphInfoOut)
}

fn addNodeToDAG(
    mut nodeIdx: i32,
    mut graphIn: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: HpcOmTaskGraph::TaskGraphMeta,
    mut graphIdx: i32,
    mut atts: &metamodelica::List<i32>,
    mut graphInfoIn: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut nameAttIdx: i32;
    let mut childNodes: metamodelica::List<i32>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeLabel: GraphML::NodeLabel;
    let mut nodeString: ArcStr;
    let mut nodeDesc: ArcStr;
    let mut compName: ArcStr;
    let HpcOmTaskGraph::TASKGRAPHMETA {
        inComps: __pa0,
        compDescs: __pa1,
        ..
    } = metaIn;
    inComps = metamodelica::Own::own(__pa0);
    compDescs = metamodelica::Own::own(__pa1);
    nodeDesc = metamodelica::arrayGet(compDescs.clone(), nodeIdx)?;
    nodeString = intString(nodeIdx);
    compName = stringDelimitList(
        List::map(
            metamodelica::arrayGet(inComps.clone(), nodeIdx)?,
            &fnptr!(intString, i32),
        )?,
        literal!(","),
    );
    nameAttIdx = (atts).get(1)?;
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
        text: nodeString,
        backgroundColor: None,
        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
    };
    (tmpGraph, _) = GraphML::addNode(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Node"));
            __mm_s.push_str(&*intString(nodeIdx));
            ArcStr::from(__mm_s)
        },
        arcstr::literal!(GraphML::COLOR_ORANGE),
        GraphML::BORDERWIDTH_STANDARD.clone(),
        list![nodeLabel],
        openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
        Some(nodeDesc),
        list![(nameAttIdx, compName)],
        graphIdx,
        graphInfoIn,
    )?;
    childNodes = metamodelica::arrayGet(graphIn.clone(), nodeIdx)?;
    graphInfoOut = List::fold1(&childNodes, &addDirectedEdge, nodeIdx, tmpGraph)?;
    Ok(graphInfoOut)
}

fn addDirectedEdge(mut child: i32, mut parent: i32, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    (graphInfoOut, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Edge"));
            __mm_s.push_str(&*intString(parent));
            __mm_s.push_str(&*intString(child));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Node"));
            __mm_s.push_str(&*intString(child));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Node"));
            __mm_s.push_str(&*intString(parent));
            ArcStr::from(__mm_s)
        },
        arcstr::literal!(GraphML::COLOR_BLACK),
        openmodelica_codegen_graphml::GraphML::LineType::LINE,
        GraphML::LINEWIDTH_STANDARD.clone(),
        false,
        metamodelica::nil(),
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
        ),
        metamodelica::nil(),
        graphInfoIn,
    )?;
    Ok(graphInfoOut)
}

fn addVarNodeToGraph(
    mut indx: i32,
    mut vars: &BackendDAE::Variables,
    mut attsIn: &metamodelica::List<(bool, ArcStr)>,
    mut attributeIdcs: &metamodelica::List<i32>,
    mut graphInfoIn: &(GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut isTearVar: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut varString: ArcStr;
    let mut varNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    nameAttrIdx = (attributeIdcs).get(1)?;
    typeAttIdx = (attributeIdcs).get(2)?;
    idxAttrIdx = (attributeIdcs).get(3)?;
    isTearVar = Util::tuple21((attsIn).get(indx)?);
    daeIdxStr = Util::tuple22((attsIn).get(indx)?);
    typeStr = if (isTearVar) {
        literal!("tearingVar")
    } else {
        literal!("otherVar")
    };
    var = BackendVariable::getVarAt(vars, indx)?;
    varString = self::varString(&var)?;
    varNodeId = getVarNodeIdx(indx);
    idxString = intString(indx);
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
        text: idxString,
        backgroundColor: None,
        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
    };
    (graphInfo, _) = GraphML::addNode(
        varNodeId,
        arcstr::literal!(GraphML::COLOR_ORANGE2),
        GraphML::BORDERWIDTH_STANDARD.clone(),
        list![nodeLabel],
        openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE,
        Some(varString.clone()),
        list![(nameAttrIdx, varString), (typeAttIdx, typeStr), (idxAttrIdx, daeIdxStr)],
        graphIdx,
        graphInfo,
    )?;
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

fn addEqNodeToGraph(
    mut indx: i32,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut attsIn: &metamodelica::List<(bool, ArcStr)>,
    mut attributeIdcs: &metamodelica::List<i32>,
    mut graphInfoIn: &(GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut isResEq: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttrIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut eqString: ArcStr;
    let mut eqNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    nameAttrIdx = (attributeIdcs).get(1)?;
    typeAttrIdx = (attributeIdcs).get(2)?;
    idxAttrIdx = (attributeIdcs).get(3)?;
    isResEq = Util::tuple21((attsIn).get(indx)?);
    daeIdxStr = Util::tuple22((attsIn).get(indx)?);
    typeStr = if (isResEq) {
        literal!("residualEq")
    } else {
        literal!("otherEq")
    };
    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![indx], eqs)?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eq = metamodelica::Own::own(__pa0);
    eqString = equationString(&eq)?;
    eqNodeId = getEqNodeIdx(indx);
    idxString = intString(indx);
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
        text: idxString,
        backgroundColor: None,
        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
    };
    (graphInfo, _) = GraphML::addNode(
        eqNodeId,
        arcstr::literal!(GraphML::COLOR_GREEN2),
        GraphML::BORDERWIDTH_STANDARD.clone(),
        list![nodeLabel],
        openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
        Some(eqString.clone()),
        list![(nameAttrIdx, eqString), (typeAttrIdx, typeStr), (idxAttrIdx, daeIdxStr)],
        graphIdx,
        graphInfo,
    )?;
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

fn addEdgeToGraph(
    mut eqIdx: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut graphInfoIn: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varLst: metamodelica::List<i32>;
    varLst = metamodelica::arrayGet(m.clone(), eqIdx)?;
    graphInfoOut = List::fold1(&varLst, &addEdgeToGraph2, eqIdx, graphInfoIn)?;
    Ok(graphInfoOut)
}

fn addEdgeToGraph2(
    mut varIdxIn: i32,
    mut eqIdx: i32,
    mut graphInfoIn: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varIdx: i32;
    let mut eqNodeId: ArcStr;
    let mut varNodeId: ArcStr;
    let mut lt: GraphML::LineType;
    if varIdxIn <= 0 {
        lt = openmodelica_codegen_graphml::GraphML::LineType::DASHED;
    } else {
        lt = openmodelica_codegen_graphml::GraphML::LineType::LINE;
    }
    varIdx = intAbs(varIdxIn);
    eqNodeId = getEqNodeIdx(eqIdx);
    varNodeId = getVarNodeIdx(varIdx);
    (graphInfoOut, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Edge_"));
            __mm_s.push_str(&*intString(varIdx));
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(eqIdx));
            ArcStr::from(__mm_s)
        },
        varNodeId,
        eqNodeId,
        arcstr::literal!(GraphML::COLOR_BLACK),
        lt,
        GraphML::LINEWIDTH_STANDARD.clone(),
        false,
        metamodelica::nil(),
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
        ),
        metamodelica::nil(),
        graphInfoIn,
    )?;
    Ok(graphInfoOut)
}

fn getVarNodeIdx(mut idx: i32) -> ArcStr {
    let mut varString: ArcStr;
    varString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("varNode"));
        __mm_s.push_str(&*intString(intAbs(idx)));
        ArcStr::from(__mm_s)
    };
    varString
}

fn getEqNodeIdx(mut idx: i32) -> ArcStr {
    let mut eqString: ArcStr;
    eqString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("eqNode"));
        __mm_s.push_str(&*intString(intAbs(idx)));
        ArcStr::from(__mm_s)
    };
    eqString
}

pub fn dumpBackendDAEBipartiteGraph(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut filename: &ArcStr,
) -> Result<()> {
    let mut graphIdx: i32;
    let mut sysIdx: i32;
    let mut varIdx: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut order: i32;
    let mut nameAttIdx: i32;
    let mut varAttIdx: i32;
    let mut eqAttIdx: i32;
    let mut sysAttIdx: i32;
    let mut tearAttIdx: i32;
    let mut compAttIdx: i32;
    let mut orderAttIdx: i32;
    let mut tearInfo: ArcStr;
    let mut nodeColor: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut shapeType: GraphML::ShapeType;
    let mut lineType: GraphML::LineType;
    let mut lineWidth: metamodelica::Real;
    let mut borderWidth: metamodelica::Real;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqIdxs: metamodelica::List<i32>;
    let mut varIdxs: metamodelica::List<i32>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut ass2: metamodelica::Array<i32>;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("TaskGraph"), true, graphInfo)?;
    graphInfo = metamodelica::Own::own(__pa0);
    graphIdx = metamodelica::Own::own(__pa1);
    let (__pa2, (_, __pa3)) = GraphML::addAttribute(
        literal!(""),
        literal!("Name"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa2);
    nameAttIdx = metamodelica::Own::own(__pa3);
    let (__pa4, (_, __pa5)) = GraphML::addAttribute(
        literal!(""),
        literal!("VarIdx"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa4);
    varAttIdx = metamodelica::Own::own(__pa5);
    let (__pa6, (_, __pa7)) = GraphML::addAttribute(
        literal!(""),
        literal!("EqIdx"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa6);
    eqAttIdx = metamodelica::Own::own(__pa7);
    let (__pa8, (_, __pa9)) = GraphML::addAttribute(
        literal!(""),
        literal!("SysIdx"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa8);
    sysAttIdx = metamodelica::Own::own(__pa9);
    let (__pa10, (_, __pa11)) = GraphML::addAttribute(
        literal!(""),
        literal!("Tearing"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa10);
    tearAttIdx = metamodelica::Own::own(__pa11);
    let (__pa12, (_, __pa13)) = GraphML::addAttribute(
        literal!(""),
        literal!("SCC"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa12);
    compAttIdx = metamodelica::Own::own(__pa13);
    let (__pa14, (_, __pa15)) = GraphML::addAttribute(
        literal!(""),
        literal!("executionOrder"),
        openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
        openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
        graphInfo,
    )?;
    graphInfo = metamodelica::Own::own(__pa14);
    orderAttIdx = metamodelica::Own::own(__pa15);
    let __arc18 = &(*dae);
    let BackendDAE::DAE {
        eqs: __pa16,
        shared: __pa17,
    } = &**__arc18;
    systs = metamodelica::Own::own(__pa16);
    shared = metamodelica::Own::own(__pa17);
    sysIdx = 1;
    for mut sys in &*systs {
        let (__pa19, __pa20, __pa21, __pa22) = ::match_deref::match_deref! { match &(sys.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa19, orderedEqs: __pa20, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa21, ass2: __pa22, .. }, .. } => (__pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone()),
            _ => return Err("pattern mismatch"),
        } };
        vars = metamodelica::Own::own(__pa19);
        eqs = metamodelica::Own::own(__pa20);
        comps = metamodelica::Own::own(__pa21);
        ass2 = metamodelica::Own::own(__pa22);
        (m, mT) = BackendDAEUtil::adjacencyMatrix(
            metamodelica::AsArg::as_arg(&sys),
            openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
            Some(BackendDAEUtil::getFunctions(&shared)),
            BackendDAEUtil::isInitializationDAE(&shared),
        )?;
        order = 1;
        for mut comp in &*comps {
            (varLst, varIdxs, eqLst, eqIdxs) =
                BackendDAEUtil::getStrongComponentsVarsAndEquations(&(list![comp.clone()]), vars.clone(), eqs.clone())?;
            for mut varIdx in &*varIdxs {
                let mut varIdx = varIdx.clone();
                nodeColor = if (isAlgLoop(metamodelica::AsArg::as_arg(&comp))) {
                    arcstr::literal!(GraphML::COLOR_RED2)
                } else {
                    arcstr::literal!(GraphML::COLOR_GREEN2)
                };
                borderWidth = if (BackendVariable::isStateVar(&(BackendVariable::getVarAt(&vars, varIdx)?))) {
                    GraphML::BORDERWIDTH_BOLD.clone()
                } else {
                    GraphML::BORDERWIDTH_STANDARD.clone()
                };
                if isTearingVar(varIdx, metamodelica::AsArg::as_arg(&comp))? {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE;
                    tearInfo = literal!("TearingVar");
                    nodeColor = arcstr::literal!(GraphML::COLOR_RED);
                } else {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE;
                    tearInfo = literal!("AlgebraicVar");
                }
                (graphInfo, _) = GraphML::addNode(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("V_"));
                        __mm_s.push_str(&*intString(sysIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(varIdx));
                        ArcStr::from(__mm_s)
                    },
                    nodeColor,
                    borderWidth,
                    list![GraphML::NodeLabel::NODELABEL_INTERNAL {
                        text: intString(varIdx),
                        backgroundColor: None,
                        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN
                    }],
                    shapeType,
                    Some(varString(&(BackendVariable::getVarAt(&vars, varIdx)?))?),
                    list![
                        (nameAttIdx, {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("V_"));
                            __mm_s.push_str(&*intString(sysIdx));
                            __mm_s.push_str(&*literal!("_"));
                            __mm_s.push_str(&*intString(varIdx));
                            ArcStr::from(__mm_s)
                        }),
                        (varAttIdx, intString(varIdx)),
                        (eqAttIdx, literal!("-")),
                        (compAttIdx, printComponent(metamodelica::AsArg::as_arg(&comp), None)?),
                        (sysAttIdx, intString(sysIdx)),
                        (tearAttIdx, tearInfo),
                        (orderAttIdx, intString(order))
                    ],
                    graphIdx,
                    graphInfo,
                )?;
            }
            for mut eqIdx in &*eqIdxs {
                let mut eqIdx = eqIdx.clone();
                nodeColor = if (isAlgLoop(metamodelica::AsArg::as_arg(&comp))) {
                    arcstr::literal!(GraphML::COLOR_RED2)
                } else {
                    arcstr::literal!(GraphML::COLOR_GREEN2)
                };
                if isResidualEq(eqIdx, metamodelica::AsArg::as_arg(&comp))? {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE;
                    tearInfo = literal!("ResidualEq");
                    nodeColor = arcstr::literal!(GraphML::COLOR_RED);
                } else {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE;
                    tearInfo = literal!("AlgebraicEq");
                }
                (graphInfo, _) = GraphML::addNode(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("E_"));
                        __mm_s.push_str(&*intString(sysIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(eqIdx));
                        ArcStr::from(__mm_s)
                    },
                    nodeColor,
                    GraphML::BORDERWIDTH_STANDARD.clone(),
                    list![GraphML::NodeLabel::NODELABEL_INTERNAL {
                        text: intString(eqIdx),
                        backgroundColor: None,
                        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN
                    }],
                    shapeType,
                    Some(equationString(&(BackendEquation::get(eqs.clone(), eqIdx)?))?),
                    list![
                        (nameAttIdx, {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("E_"));
                            __mm_s.push_str(&*intString(sysIdx));
                            __mm_s.push_str(&*literal!("_"));
                            __mm_s.push_str(&*intString(eqIdx));
                            ArcStr::from(__mm_s)
                        }),
                        (varAttIdx, literal!("-")),
                        (compAttIdx, printComponent(metamodelica::AsArg::as_arg(&comp), None)?),
                        (eqAttIdx, intString(eqIdx)),
                        (sysAttIdx, intString(sysIdx)),
                        (tearAttIdx, tearInfo),
                        (orderAttIdx, intString(order))
                    ],
                    graphIdx,
                    graphInfo,
                )?;
            }
            order = order + 1;
        }
        for mut eqIdx in 1..=metamodelica::arrayLength(m.clone()) {
            for mut varIdx in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
                let mut varIdx = varIdx.clone();
                if intLe(varIdx, 0) {
                    lineType = openmodelica_codegen_graphml::GraphML::LineType::DASHED;
                } else {
                    lineType = openmodelica_codegen_graphml::GraphML::LineType::LINE;
                }
                varIdx = intAbs(varIdx);
                lineWidth = if (intEq(
                    varIdx,
                    ({
                        let __elt = (*metamodelica::index_checked(&ass2.borrow(), eqIdx)?).clone();
                        __elt
                    }),
                )) {
                    GraphML::LINEWIDTH_BOLD.clone()
                } else {
                    GraphML::LINEWIDTH_STANDARD.clone()
                };
                (graphInfo, _) = GraphML::addEdge(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Edge_"));
                        __mm_s.push_str(&*intString(sysIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(eqIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(varIdx));
                        ArcStr::from(__mm_s)
                    },
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("V_"));
                        __mm_s.push_str(&*intString(sysIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(varIdx));
                        ArcStr::from(__mm_s)
                    },
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("E_"));
                        __mm_s.push_str(&*intString(sysIdx));
                        __mm_s.push_str(&*literal!("_"));
                        __mm_s.push_str(&*intString(eqIdx));
                        ArcStr::from(__mm_s)
                    },
                    arcstr::literal!(GraphML::COLOR_BLACK),
                    lineType,
                    lineWidth,
                    false,
                    metamodelica::nil(),
                    (
                        openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
                        openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
                    ),
                    metamodelica::nil(),
                    graphInfo,
                )?;
            }
        }
        sysIdx = sysIdx + 1;
    }
    GraphML::dumpGraph(graphInfo, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*filename);
        __mm_s.push_str(&*literal!(".graphml"));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn isTearingVar(mut varIdx: i32, mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>) -> Result<bool> {
    let mut isTear: bool;
    isTear = (match &**comp {
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet: BackendDAE::TearingSet { tearingvars: tVars, .. },
            ..
        } => List::exist1(metamodelica::AsArg::as_arg(&tVars), &fnptr!(intEq, i32, i32), varIdx)?,
        _ => false,
    });
    Ok(isTear)
}

fn isAlgLoop(mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>) -> bool {
    let mut isLoop: bool;
    isLoop = (match &**comp {
        BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: _, .. } => true,
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet: _, ..
        } => true,
        _ => false,
    });
    isLoop
}

fn isResidualEq(mut eqIdx: i32, mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>) -> Result<bool> {
    let mut isRes: bool;
    isRes = (match &**comp {
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    residualequations: resEqs,
                    ..
                },
            ..
        } => List::exist1(metamodelica::AsArg::as_arg(&resEqs), &fnptr!(intEq, i32, i32), eqIdx)?,
        _ => false,
    });
    Ok(isRes)
}

pub(crate) fn SSSHandlerArgString(
    mut arg: Option<(
        BackendDAE::StateOrder,
        metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    )>,
) -> Result<()> {
    let mut stateorder: BackendDAE::StateOrder;
    let mut constraints: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqs2EqIdxs: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqIdx2Eq: metamodelica::Array<i32>;
    let mut numEqs: i32;
    if (arg).is_some() {
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(arg) {
            Some((__pa0, __pa1, __pa2, __pa3, __pa4)) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        stateorder = metamodelica::Own::own(__pa0);
        constraints = metamodelica::Own::own(__pa1);
        eqs2EqIdxs = metamodelica::Own::own(__pa2);
        eqIdx2Eq = metamodelica::Own::own(__pa3);
        numEqs = metamodelica::Own::own(__pa4);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(numEqs));
            __mm_s.push_str(&*literal!("eqs before IR\n"));
            ArcStr::from(__mm_s)
        });
        dumpStateOrder(&stateorder)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Constraints:\n"));
            __mm_s.push_str(&*constraintEquationString(constraints.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        metamodelica::print(literal!("Empty StructurallySingularSystemHandlerArg\n"));
    }
    Ok(())
}

pub(crate) fn constraintEquationString(
    mut constraints: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut i: i32 = 0;
    let mut s1: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(constraints.clone()) {
        s1 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                List::map(
                    metamodelica::arrayGet(constraints.clone(), i)?,
                    &move |__a0: metamodelica::Ref<BackendDAE::Equation>| equationString(&__a0),
                )?,
                literal!("\n"),
            ));
            __mm_s.push_str(&*literal!("\n------------------\n"));
            ArcStr::from(__mm_s)
        };
        if (metamodelica::arrayGet(constraints.clone(), i)?).is_empty() {
            s1 = literal!("empty Constraints\n");
        }
        s = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("eq "));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*s1);
            __mm_s.push_str(&*s);
            ArcStr::from(__mm_s)
        };
    }
    Ok(s)
}

pub(crate) fn dumpStateOrder(mut inStateOrder: &BackendDAE::StateOrder) -> Result<()> {
    let () = (match inStateOrder.clone() {
        BackendDAE::StateOrder::STATEORDER {
            hashTable: mut ht,
            invHashTable: _,
        } => {
            let mut r#str: ArcStr;
            let mut len_str: ArcStr;
            let mut len: i32;
            let mut tplLst: metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<DAE::ComponentRef>,
            )>;
            tplLst = BaseHashTable::hashTableList(&(ht.clone()))?;
            if !((tplLst).is_empty()) {
                metamodelica::print(literal!("State Order: ("));
                r#str = stringDelimitList(List::map(tplLst.clone(), &printStateOrderStr)?, literal!("\n"));
                len = ((tplLst).len() as i32);
                len_str = intString(len);
                metamodelica::print(len_str);
                metamodelica::print(literal!(")\n"));
                metamodelica::print(literal!("=============\n"));
                metamodelica::print(r#str);
                metamodelica::print(literal!("\n\n"));
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn printStateOrderStr(
    mut tpl: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(Util::tuple21(tpl.clone())),
        )?);
        __mm_s.push_str(&*literal!(" ---d/dt---> "));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&(Util::tuple22(tpl)))?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn dumpBackendDAEModeData(mut inDAEmodeData: &BackendDAE::BackendDAEModeData) -> Result<()> {
    let mut modelVars: BackendDAE::Variables;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(BORDER));
        __mm_s.push_str(&*literal!("\nDAEMode\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if (inDAEmodeData.modelVars).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(inDAEmodeData.modelVars.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        modelVars = metamodelica::Own::own(__pa0);
        dumpVariables(&modelVars, &(literal!("ModelVariables")))?;
    } else {
        metamodelica::print(literal!("No ModelVariables\n"));
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("DAEmode System:\n "));
        __mm_s.push_str(&*intString(inDAEmodeData.numResVars.clone()));
        __mm_s.push_str(&*literal!(" residual variables\n "));
        __mm_s.push_str(&*intString(((inDAEmodeData.stateVars).len() as i32)));
        __mm_s.push_str(&*literal!(" state variables\n "));
        __mm_s.push_str(&*intString(((inDAEmodeData.algStateVars).len() as i32)));
        __mm_s.push_str(&*literal!(" algebraic state variables\n"));
        ArcStr::from(__mm_s)
    });
    dumpVarList(&inDAEmodeData.stateVars, &(literal!("State Variables")))?;
    dumpVarList(&inDAEmodeData.algStateVars, &(literal!("Algebraic State Variables")))?;
    Ok(())
}
