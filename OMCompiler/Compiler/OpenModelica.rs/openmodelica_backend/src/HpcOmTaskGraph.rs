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
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::HpcOmBenchmark;
use crate::HpcOmScheduler;
use crate::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_codegen_util::HpcOmCodegenUtil;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

//----------------------------
//  Graph Structure
//----------------------------
pub type TaskGraph = metamodelica::Array<metamodelica::List<i32>>;

pub type Communications = metamodelica::List<Communication>;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Communication {
    pub numberOfVars: i32,
    pub integerVars: metamodelica::List<i32>,
    pub floatVars: metamodelica::List<i32>,
    pub booleanVars: metamodelica::List<i32>,
    pub stringVars: metamodelica::List<i32>,
    pub childNode: i32,
    pub requiredTime: metamodelica::Real,
}

impl metamodelica::gc::MMTrace for Communication {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.integerVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.floatVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.booleanVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.childNode, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.requiredTime, __mmv)?;
        Ok(())
    }
}
impl Default for Communication {
    fn default() -> Self {
        Self {
            numberOfVars: Default::default(),
            integerVars: Default::default(),
            floatVars: Default::default(),
            booleanVars: Default::default(),
            stringVars: Default::default(),
            childNode: Default::default(),
            requiredTime: Default::default(),
        }
    }
}

pub type COMMUNICATION = Communication;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ComponentInfo {
    pub isPartOfODESystem: bool,
    pub isPartOfZeroFuncSystem: bool,
    pub isRemovedComponent: bool,
}

impl metamodelica::gc::MMTrace for ComponentInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.isPartOfODESystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isPartOfZeroFuncSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isRemovedComponent, __mmv)?;
        Ok(())
    }
}
impl Default for ComponentInfo {
    fn default() -> Self {
        Self {
            isPartOfODESystem: Default::default(),
            isPartOfZeroFuncSystem: Default::default(),
            isRemovedComponent: Default::default(),
        }
    }
}

pub type COMPONENTINFO = ComponentInfo;

// TODO: Store compParamMapping, compNames and compDescs in ComponentInfo
// TODO: Change nodeMark to compMarks
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TaskGraphMeta {
    pub inComps: metamodelica::Array<metamodelica::List<i32>>,
    pub varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub compParamMapping: metamodelica::Array<metamodelica::List<i32>>,
    pub compNames: metamodelica::Array<ArcStr>,
    pub compDescs: metamodelica::Array<ArcStr>,
    pub exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    pub commCosts: metamodelica::Array<metamodelica::List<Communication>>,
    pub nodeMark: metamodelica::Array<i32>,
    pub compInformations: metamodelica::Array<ComponentInfo>,
}

impl metamodelica::gc::MMTrace for TaskGraphMeta {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.inComps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varCompMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqCompMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compParamMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compNames, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compDescs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exeCosts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.commCosts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nodeMark, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compInformations, __mmv)?;
        Ok(())
    }
}
impl Default for TaskGraphMeta {
    fn default() -> Self {
        Self {
            inComps: Default::default(),
            varCompMapping: Default::default(),
            eqCompMapping: Default::default(),
            compParamMapping: Default::default(),
            compNames: Default::default(),
            compDescs: Default::default(),
            exeCosts: Default::default(),
            commCosts: Default::default(),
            nodeMark: Default::default(),
            compInformations: Default::default(),
        }
    }
}

pub type TASKGRAPHMETA = TaskGraphMeta;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum VariableType {
    INTEGER = 1,
    REAL = 2,
    BOOLEAN = 3,
    STRING = 4,
}
impl PartialOrd for VariableType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for VariableType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for VariableType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub type VariableList = (
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
);

//variables <int, float, bool, string>
//----------------------------------------------------------
//  Functions to build the task graph from the BLT structure
//----------------------------------------------------------
pub(crate) fn createTaskGraph(
    mut iDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iAnalyzeParameters: bool,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oGraph: TaskGraph;
    let mut oGraphData: TaskGraphMeta;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let __arc2 = &(*iDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    (graph, graphData) = getEmptyTaskGraph(0, 0, 0);
    (oGraph, oGraphData, _) = List::fold(
        &systs,
        &({
            let __pe_b1 = shared;
            let __pe_b2 = iAnalyzeParameters;
            move |__pe_a0, __pe_a3| createTaskGraph0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_a3)
        }),
        (graph.clone(), graphData, 1),
    )?;
    Ok((oGraph, oGraphData))
}

pub(crate) fn createTaskGraph0(
    mut iSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
    mut iAnalyzeParameters: bool,
    mut iGraphInfo: &(metamodelica::Array<metamodelica::List<i32>>, TaskGraphMeta, i32),
) -> Result<(metamodelica::Array<metamodelica::List<i32>>, TaskGraphMeta, i32)> {
    let mut oGrapInfo: (metamodelica::Array<metamodelica::List<i32>>, TaskGraphMeta, i32);
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut vars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut sharedFuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut iGraphData: TaskGraphMeta;
    let mut tmpGraphData: TaskGraphMeta;
    let mut iGraph: TaskGraph;
    let mut tmpGraph: TaskGraph;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut numberOfVars: i32;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut eqSysIdx: i32;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let __arc3 = iSyst.clone();
    let BackendDAE::EQSYSTEM {
        matching: __pa0,
        orderedVars: __pa1,
        orderedEqs: __pa2,
        ..
    } = &*__arc3;
    matching = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    orderedEqs = metamodelica::Own::own(__pa2);
    comps = BackendDAEUtil::getCompsOfMatching(&matching);
    let __arc5 = iShared.clone();
    let BackendDAE::SHARED {
        functionTree: __pa4, ..
    } = &*__arc5;
    sharedFuncs = metamodelica::Own::own(__pa4);
    (iGraph, iGraphData, eqSysIdx) = iGraphInfo.clone();
    (_, adjacencyMatrix, _) = BackendDAEUtil::getAdjacencyMatrix(
        iSyst.clone(),
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        Some(sharedFuncs),
        BackendDAEUtil::isInitializationDAE(&iShared),
    )?;
    numberOfVars = BackendVariable::varsSize(&vars);
    (tmpGraph, tmpGraphData) = getEmptyTaskGraph(
        ((comps).len() as i32),
        numberOfVars,
        ExpandableArray::getNumberOfElements(orderedEqs),
    );
    let TaskGraphMeta {
        inComps: __pa6,
        compNames: __pa7,
        exeCosts: __pa8,
        commCosts: __pa9,
        nodeMark: __pa10,
        varCompMapping: __pa11,
        eqCompMapping: __pa12,
        compParamMapping: __pa13,
        compInformations: __pa14,
        ..
    } = tmpGraphData;
    inComps = metamodelica::Own::own(__pa6);
    compNames = metamodelica::Own::own(__pa7);
    exeCosts = metamodelica::Own::own(__pa8);
    commCosts = metamodelica::Own::own(__pa9);
    nodeMark = metamodelica::Own::own(__pa10);
    varCompMapping = metamodelica::Own::own(__pa11);
    eqCompMapping = metamodelica::Own::own(__pa12);
    compParamMapping = metamodelica::Own::own(__pa13);
    compInformations = metamodelica::Own::own(__pa14);
    (varCompMapping, eqCompMapping) =
        getVarEqCompMapping(&comps, eqSysIdx, 0, 0, varCompMapping.clone(), eqCompMapping.clone())?;
    compDescs = getEquationStrings(&comps, iSyst.clone())?;
    (tmpGraph, inComps, compParamMapping, commCosts, compNames, nodeMark, _) = List::fold(
        &(comps.clone()),
        &({
            let __pe_b1 = (adjacencyMatrix.clone(), iSyst, iShared, ((comps).len() as i32));
            let __pe_b2 = (varCompMapping.clone(), eqCompMapping.clone(), metamodelica::nil());
            let __pe_b3 = iAnalyzeParameters;
            move |__pe_a0, __pe_a4| createTaskGraph1(&__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), __pe_a4)
        }),
        (
            tmpGraph.clone(),
            inComps.clone(),
            compParamMapping.clone(),
            commCosts.clone(),
            compNames.clone(),
            nodeMark.clone(),
            1,
        ),
    )?;
    tmpGraph = Array::mapNoCopy(
        tmpGraph.clone(),
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> =
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>);
            move |__pe_a0| List::sort(__pe_a0, __pe_b1.clone())
        }),
    )?;
    tmpGraphData = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    if intGt(eqSysIdx, 1) {
        (tmpGraph, tmpGraphData) = taskGraphAppend(iGraph.clone(), iGraphData, tmpGraph.clone(), tmpGraphData)?;
    }
    oGrapInfo = (tmpGraph.clone(), tmpGraphData, eqSysIdx + 1);
    Ok(oGrapInfo)
}

pub(crate) fn getSystemComponents(
    mut iDae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
)> {
    let mut oComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut oMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut tmpSystems: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut tmpComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    (oComps, oMapping) = (match &**iDae {
        BackendDAE::BackendDAE { eqs: __esc_systs, .. } => {
            systs = (*__esc_systs).clone();
            (tmpComps, tmpSystems, _) = List::fold(
                metamodelica::AsArg::as_arg(&systs),
                &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                       __a1: (
                    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
                    metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
                    i32,
                )| getSystemComponents0(__a0, &__a1),
                (metamodelica::nil(), metamodelica::nil(), 1),
            )?;
            (
                tmpComps,
                metamodelica::arrayFromVec(tmpSystems.into_iter().cloned().collect()),
            )
        }
        _ => return Err("fail"),
    });
    Ok((oComps, oMapping))
}

fn getSystemComponents0(
    mut iSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut iSystMapping: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
        i32,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    i32,
)> {
    let mut oSystMapping: (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
        i32,
    );
    let mut tmpComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut tmpSystMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut currentIdx: i32;
    oSystMapping = (::match_deref::match_deref! { match &((iSyst.clone(), iSystMapping.clone())) {
        (Deref @ BackendDAE::EqSystem { matching: __esc_matching, .. }, (__esc_tmpComps, __esc_tmpSystMapping, __esc_currentIdx)) => {
            matching = (*__esc_matching).clone();
            tmpComps = (*__esc_tmpComps).clone();
            tmpSystMapping = (*__esc_tmpSystMapping).clone();
            currentIdx = (*__esc_currentIdx).clone();
            comps = BackendDAEUtil::getCompsOfMatching(metamodelica::AsArg::as_arg(&matching));
            tmpSystMapping = List::fold2(&comps, &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>, __a1: metamodelica::Ref<BackendDAE::EqSystem>, __a2: i32, __a3: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getSystemComponents1(&__a0, __a1, __a2, __a3)) }, iSyst, currentIdx.clone(), tmpSystMapping.clone())?;
            comps = listAppend(tmpComps.clone(), comps);
            (comps, tmpSystMapping.clone(), currentIdx.clone() + 1)
        },
        _ => {
            metamodelica::print(literal!("getSystemComponents0 failed\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSystMapping)
}

fn getSystemComponents1(
    mut icomp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut isystIdx: i32,
    mut iMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
) -> metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)> {
    let mut oMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    oMapping = listAppend(iMapping, list![(isyst, isystIdx)]);
    oMapping
}

fn getNumberOfSystemComponents(mut iDae: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<i32> {
    let mut oNumOfComps: i32;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*iDae);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    eqs = metamodelica::Own::own(__pa0);
    oNumOfComps = List::fold(
        &eqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: i32| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getNumberOfEqSystemComponents(&__a0, __a1))
        },
        0,
    )?;
    Ok(oNumOfComps)
}

fn getNumberOfEqSystemComponents(mut iEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>, mut iNumOfComps: i32) -> i32 {
    let mut oNumOfComps: i32;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let __arc1 = &(*iEqSystem);
    let BackendDAE::EQSYSTEM { matching: __pa0, .. } = &**__arc1;
    matching = metamodelica::Own::own(__pa0);
    comps = BackendDAEUtil::getCompsOfMatching(&matching);
    oNumOfComps = iNumOfComps + ((comps).len() as i32);
    oNumOfComps
}

pub(crate) fn getEmptyTaskGraph(mut numComps: i32, mut numVars: i32, mut numEqs: i32) -> (TaskGraph, TaskGraphMeta) {
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    graph = arrayCreate(numComps, metamodelica::nil());
    inComps = arrayCreate(numComps, metamodelica::nil());
    compParamMapping = arrayCreate(numComps, metamodelica::nil());
    varCompMapping = arrayCreate(numVars, (0, 0, 0));
    eqCompMapping = arrayCreate(numEqs, (0, 0, 0));
    compNames = arrayCreate(numComps, literal!(""));
    compDescs = arrayCreate(numComps, literal!(""));
    exeCosts = arrayCreate(numComps, (-1, metamodelica::OrderedFloat(-1.0_f64)));
    commCosts = arrayCreate(numComps, metamodelica::nil());
    nodeMark = arrayCreate(numComps, 0);
    compInformations = arrayCreate(
        numComps,
        ComponentInfo {
            isPartOfODESystem: false,
            isPartOfZeroFuncSystem: false,
            isRemovedComponent: false,
        },
    );
    graphData = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    (graph, graphData)
}

pub(crate) fn copyTaskGraphMeta(mut graphDataIn: TaskGraphMeta) -> TaskGraphMeta {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut inComps1: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compParamMapping1: metamodelica::Array<metamodelica::List<i32>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut commCosts1: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = graphDataIn;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    exeCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeMark = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    inComps1 = metamodelica::arrayFromVec(inComps.clone().borrow().clone());
    varCompMapping1 = metamodelica::arrayFromVec(varCompMapping.clone().borrow().clone());
    eqCompMapping1 = metamodelica::arrayFromVec(eqCompMapping.clone().borrow().clone());
    compParamMapping1 = metamodelica::arrayFromVec(compParamMapping.clone().borrow().clone());
    compNames1 = metamodelica::arrayFromVec(compNames.clone().borrow().clone());
    compDescs1 = metamodelica::arrayFromVec(compDescs.clone().borrow().clone());
    exeCosts1 = metamodelica::arrayFromVec(exeCosts.clone().borrow().clone());
    commCosts1 = metamodelica::arrayFromVec(commCosts.clone().borrow().clone());
    nodeMark1 = metamodelica::arrayFromVec(nodeMark.clone().borrow().clone());
    compInformations1 = metamodelica::arrayFromVec(compInformations.clone().borrow().clone());
    graphDataOut = TaskGraphMeta {
        inComps: inComps1.clone(),
        varCompMapping: varCompMapping1.clone(),
        eqCompMapping: eqCompMapping1.clone(),
        compParamMapping: compParamMapping1.clone(),
        compNames: compNames1.clone(),
        compDescs: compDescs1.clone(),
        exeCosts: exeCosts1.clone(),
        commCosts: commCosts1.clone(),
        nodeMark: nodeMark1.clone(),
        compInformations: compInformations1.clone(),
    };
    graphDataOut
}

fn taskGraphAppend(
    mut graph1In: TaskGraph,
    mut graphData1In: TaskGraphMeta,
    mut graph2In: TaskGraph,
    mut graphData2In: TaskGraphMeta,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut eqOffset: i32;
    let mut idxOffset: i32;
    let mut varOffset: i32;
    let mut commCosts1: metamodelica::Array<metamodelica::List<Communication>>;
    let mut commCosts2: metamodelica::Array<metamodelica::List<Communication>>;
    let mut inComps1: metamodelica::Array<metamodelica::List<i32>>;
    let mut inComps2: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut nodeMark2: metamodelica::Array<i32>;
    let mut compParamMapping1: metamodelica::Array<metamodelica::List<i32>>;
    let mut compParamMapping2: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compNames2: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut compDescs2: metamodelica::Array<ArcStr>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let mut compInformations2: metamodelica::Array<ComponentInfo>;
    let mut graph2: TaskGraph;
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = graphData1In;
    inComps1 = metamodelica::Own::own(__pa0);
    varCompMapping1 = metamodelica::Own::own(__pa1);
    eqCompMapping1 = metamodelica::Own::own(__pa2);
    compParamMapping1 = metamodelica::Own::own(__pa3);
    compNames1 = metamodelica::Own::own(__pa4);
    compDescs1 = metamodelica::Own::own(__pa5);
    exeCosts1 = metamodelica::Own::own(__pa6);
    commCosts1 = metamodelica::Own::own(__pa7);
    nodeMark1 = metamodelica::Own::own(__pa8);
    compInformations1 = metamodelica::Own::own(__pa9);
    let TaskGraphMeta {
        inComps: __pa10,
        varCompMapping: __pa11,
        eqCompMapping: __pa12,
        compParamMapping: __pa13,
        compNames: __pa14,
        compDescs: __pa15,
        exeCosts: __pa16,
        commCosts: __pa17,
        nodeMark: __pa18,
        compInformations: __pa19,
    } = graphData2In;
    inComps2 = metamodelica::Own::own(__pa10);
    varCompMapping2 = metamodelica::Own::own(__pa11);
    eqCompMapping2 = metamodelica::Own::own(__pa12);
    compParamMapping2 = metamodelica::Own::own(__pa13);
    compNames2 = metamodelica::Own::own(__pa14);
    compDescs2 = metamodelica::Own::own(__pa15);
    exeCosts2 = metamodelica::Own::own(__pa16);
    commCosts2 = metamodelica::Own::own(__pa17);
    nodeMark2 = metamodelica::Own::own(__pa18);
    compInformations2 = metamodelica::Own::own(__pa19);
    eqOffset = metamodelica::arrayLength(eqCompMapping1.clone());
    idxOffset = metamodelica::arrayLength(graph1In.clone());
    varOffset = metamodelica::arrayLength(varCompMapping1.clone());
    eqOffset = metamodelica::arrayLength(eqCompMapping1.clone());
    graph2 = Array::map1(graph2In.clone(), &updateTaskGraphSystem, idxOffset)?;
    graphOut = metamodelica::arrayAppend(graph1In.clone(), graph2.clone());
    inComps2 = Array::map1(inComps2.clone(), &updateTaskGraphSystem, idxOffset)?;
    inComps2 = metamodelica::arrayAppend(inComps1.clone(), inComps2.clone());
    varCompMapping2 = Array::map1(
        varCompMapping2.clone(),
        &fnptr!(modifyMapping, (i32, i32, i32), i32),
        idxOffset,
    )?;
    varCompMapping2 = metamodelica::arrayAppend(varCompMapping1.clone(), varCompMapping2.clone());
    eqCompMapping2 = Array::map1(
        eqCompMapping2.clone(),
        &fnptr!(modifyMapping, (i32, i32, i32), i32),
        idxOffset,
    )?;
    eqCompMapping2 = metamodelica::arrayAppend(eqCompMapping1.clone(), eqCompMapping2.clone());
    compParamMapping2 = metamodelica::arrayAppend(compParamMapping1.clone(), compParamMapping2.clone());
    compNames2 = Array::map1(
        compNames2.clone(),
        &fnptr!(stringAppend, ArcStr, ArcStr),
        literal!(" subsys"),
    )?;
    compNames2 = metamodelica::arrayAppend(compNames1.clone(), compNames2.clone());
    compDescs2 = metamodelica::arrayAppend(compDescs1.clone(), compDescs2.clone());
    exeCosts2 = metamodelica::arrayAppend(exeCosts1.clone(), exeCosts2.clone());
    commCosts2 = Array::map1(commCosts2.clone(), &updateCommCosts, idxOffset)?;
    commCosts2 = metamodelica::arrayAppend(commCosts1.clone(), commCosts2.clone());
    nodeMark2 = metamodelica::arrayAppend(nodeMark1.clone(), nodeMark2.clone());
    compInformations2 = metamodelica::arrayAppend(compInformations1.clone(), compInformations2.clone());
    graphDataOut = TaskGraphMeta {
        inComps: inComps2.clone(),
        varCompMapping: varCompMapping2.clone(),
        eqCompMapping: eqCompMapping2.clone(),
        compParamMapping: compParamMapping2.clone(),
        compNames: compNames2.clone(),
        compDescs: compDescs2.clone(),
        exeCosts: exeCosts2.clone(),
        commCosts: commCosts2.clone(),
        nodeMark: nodeMark2.clone(),
        compInformations: compInformations2.clone(),
    };
    Ok((graphOut, graphDataOut))
}

fn modifyMapping(mut iMappingTuple: (i32, i32, i32), mut iOffset: i32) -> (i32, i32, i32) {
    let mut oMappingTuple: (i32, i32, i32);
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    (i1, i2, i3) = iMappingTuple;
    oMappingTuple = (i1 + iOffset, i2, iOffset);
    oMappingTuple
}

fn updateCommCosts(mut commCostsIn: Communications, mut idxOffset: i32) -> Result<Communications> {
    let mut commCostsOut: Communications;
    commCostsOut = List::map1(commCostsIn, &fnptr!(updateCommCosts1, Communication, i32), idxOffset)?;
    Ok(commCostsOut)
}

fn updateCommCosts1(mut commCostsIn: Communication, mut idxOffset: i32) -> Communication {
    let mut commCostsOut: Communication;
    let mut numberOfVars: i32;
    let mut childNode: i32;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    let mut requiredTime: metamodelica::Real;
    let Communication {
        numberOfVars: __pa0,
        integerVars: __pa1,
        floatVars: __pa2,
        booleanVars: __pa3,
        stringVars: __pa4,
        childNode: __pa5,
        requiredTime: __pa6,
    } = commCostsIn;
    numberOfVars = metamodelica::Own::own(__pa0);
    integerVars = metamodelica::Own::own(__pa1);
    floatVars = metamodelica::Own::own(__pa2);
    booleanVars = metamodelica::Own::own(__pa3);
    stringVars = metamodelica::Own::own(__pa4);
    childNode = metamodelica::Own::own(__pa5);
    requiredTime = metamodelica::Own::own(__pa6);
    childNode = childNode + idxOffset;
    commCostsOut = Communication {
        numberOfVars: numberOfVars,
        integerVars: integerVars,
        floatVars: floatVars,
        booleanVars: booleanVars,
        stringVars: stringVars,
        childNode: childNode,
        requiredTime: requiredTime,
    };
    commCostsOut
}

fn updateTaskGraphSystem(
    mut graphRowIn: metamodelica::List<i32>,
    mut idxOffset: i32,
) -> Result<metamodelica::List<i32>> {
    let mut graphRowOut: metamodelica::List<i32>;
    graphRowOut = List::map1(graphRowIn, &fnptr!(intAdd, i32, i32), idxOffset)?;
    Ok(graphRowOut)
}

fn createTaskGraph1(
    mut iComponent: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iSystInfo: &(
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<BackendDAE::EqSystem>,
        metamodelica::Ref<BackendDAE::Shared>,
        i32,
    ),
    mut iVarInfo: &(
        metamodelica::Array<(i32, i32, i32)>,
        metamodelica::Array<(i32, i32, i32)>,
        metamodelica::List<i32>,
    ),
    mut iAnalyzeParameters: bool,
    mut graphInfoIn: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<Communication>>,
        metamodelica::Array<ArcStr>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<Communication>>,
    metamodelica::Array<ArcStr>,
    metamodelica::Array<i32>,
    i32,
)> {
    let mut graphInfoOut: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<Communication>>,
        metamodelica::Array<ArcStr>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>;
    let mut isyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut ishared: metamodelica::Ref<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut localKnownVars: BackendDAE::Variables;
    let mut knownVars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut graphIn: TaskGraph;
    let mut graphTmp: TaskGraph;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut commCostsOfNode: Communications;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut unsolvedVars: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut eventVarLst: metamodelica::List<i32>;
    let mut componentIndex: i32;
    let mut numberOfComps: i32;
    let mut requiredSccs_RefCount: metamodelica::List<(
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>;
    let mut compName: ArcStr;
    let mut paramVars: metamodelica::List<i32>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut requiredSccs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            i32,
            (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ),
        >,
    >;
    (adjacencyMatrix, isyst, ishared, numberOfComps) = iSystInfo.clone();
    let __arc2 = ishared;
    let BackendDAE::SHARED {
        globalKnownVars: __pa0,
        localKnownVars: __pa1,
        ..
    } = &*__arc2;
    globalKnownVars = metamodelica::Own::own(__pa0);
    localKnownVars = metamodelica::Own::own(__pa1);
    let __arc5 = isyst;
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa3,
        orderedEqs: __pa4,
        ..
    } = &*__arc5;
    orderedVars = metamodelica::Own::own(__pa3);
    orderedEqs = metamodelica::Own::own(__pa4);
    (varCompMapping, eqCompMapping, eventVarLst) = iVarInfo.clone();
    (
        graphIn,
        inComps,
        compParamMapping,
        commCosts,
        compNames,
        nodeMark,
        componentIndex,
    ) = graphInfoIn;
    inComps = metamodelica::arrayUpdate(inComps.clone(), componentIndex, list![componentIndex])?;
    compName = BackendDump::strongComponentString(iComponent)?;
    compNames = metamodelica::arrayUpdate(compNames.clone(), componentIndex, compName)?;
    HpcOmBenchmark::benchSystem()?;
    if iAnalyzeParameters {
        knownVars = BackendVariable::addVariables(globalKnownVars, localKnownVars)?;
    } else {
        knownVars = globalKnownVars;
    }
    (unsolvedVars, paramVars) = getUnsolvedVarsBySCC(
        iComponent,
        adjacencyMatrix.clone(),
        orderedVars,
        knownVars,
        orderedEqs,
        &eventVarLst,
        iAnalyzeParameters,
    )?;
    compParamMapping = metamodelica::arrayUpdate(compParamMapping.clone(), componentIndex, paramVars)?;
    requiredSccs = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    for mut intVar in &*Util::tuple41(unsolvedVars.clone()) {
        fillRequiredSccs(
            (intVar.clone(), 1),
            VariableType::INTEGER.clone(),
            varCompMapping.clone(),
            requiredSccs.clone(),
        )?;
    }
    for mut floatVar in &*Util::tuple42(unsolvedVars.clone()) {
        fillRequiredSccs(
            floatVar.clone(),
            VariableType::REAL.clone(),
            varCompMapping.clone(),
            requiredSccs.clone(),
        )?;
    }
    for mut boolVar in &*Util::tuple43(unsolvedVars.clone()) {
        fillRequiredSccs(
            (boolVar.clone(), 1),
            VariableType::BOOLEAN.clone(),
            varCompMapping.clone(),
            requiredSccs.clone(),
        )?;
    }
    for mut stringVar in &*Util::tuple44(unsolvedVars) {
        fillRequiredSccs(
            (stringVar.clone(), 1),
            VariableType::STRING.clone(),
            varCompMapping.clone(),
            requiredSccs.clone(),
        )?;
    }
    requiredSccs_RefCount = createRequiredSccsRefCount(requiredSccs);
    (commCosts, commCostsOfNode) = updateCommCostBySccRef(requiredSccs_RefCount, componentIndex, commCosts.clone())?;
    graphTmp = fillAdjacencyList(graphIn.clone(), componentIndex, &commCostsOfNode, 1);
    graphInfoOut = (
        graphTmp.clone(),
        inComps.clone(),
        compParamMapping.clone(),
        commCosts.clone(),
        compNames.clone(),
        nodeMark.clone(),
        componentIndex + 1,
    );
    Ok(graphInfoOut)
}

fn createRequiredSccsRefCount(
    mut requiredSccs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            i32,
            (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ),
        >,
    >,
) -> metamodelica::List<(
    i32,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut requiredSccsRefCount: metamodelica::List<(
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    let mut scc_idx: i32;
    let mut int_vars: metamodelica::List<i32>;
    let mut float_vars: metamodelica::List<i32>;
    let mut bool_vars: metamodelica::List<i32>;
    let mut string_vars: metamodelica::List<i32>;
    for mut e in &*UnorderedMap::toList(requiredSccs) {
        let (__pa0, (__pa1, __pa2, __pa3, __pa4)) = e.clone();
        scc_idx = metamodelica::Own::own(__pa0);
        int_vars = metamodelica::Own::own(__pa1);
        float_vars = metamodelica::Own::own(__pa2);
        bool_vars = metamodelica::Own::own(__pa3);
        string_vars = metamodelica::Own::own(__pa4);
        requiredSccsRefCount = metamodelica::cons(
            (scc_idx, int_vars, float_vars, bool_vars, string_vars),
            requiredSccsRefCount,
        );
    }
    requiredSccsRefCount
}

fn updateCommCostBySccRef(
    mut requiredSccs_RefCount: metamodelica::List<(
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
    mut nodeIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<(metamodelica::Array<metamodelica::List<Communication>>, Communications)> {
    let mut oCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut oNodeComms: Communications;
    let mut tmpComms: Communications;
    tmpComms = List::map1(
        requiredSccs_RefCount,
        &move |__a0: (
            i32,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
               __a1: metamodelica::Real|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(createCommunicationObject(&__a0, __a1)) },
        metamodelica::OrderedFloat(-1.0_f64),
    )?;
    oCommCosts = List::fold1(&tmpComms, &updateCommCostBySccRef1, nodeIdx, iCommCosts.clone())?;
    oNodeComms = tmpComms;
    Ok((oCommCosts, oNodeComms))
}

fn createCommunicationObject(
    mut iTuple: &(
        i32,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    mut requiredTime: metamodelica::Real,
) -> Communication {
    let mut oComm: Communication;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    let mut sccIdx: i32;
    let mut refCountSum: i32;
    (sccIdx, integerVars, floatVars, booleanVars, stringVars) = iTuple.clone();
    refCountSum = ((integerVars).len() as i32)
        + ((floatVars).len() as i32)
        + ((booleanVars).len() as i32)
        + ((stringVars).len() as i32);
    oComm = Communication {
        numberOfVars: refCountSum,
        integerVars: integerVars,
        floatVars: floatVars,
        booleanVars: booleanVars,
        stringVars: stringVars,
        childNode: sccIdx,
        requiredTime: requiredTime,
    };
    oComm
}

fn updateCommCostBySccRef1(
    mut iEdgeSource: Communication,
    mut iEdgeTarget: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Array<metamodelica::List<Communication>>> {
    let mut oCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut oldComms: Communications;
    let mut sourceSccIdx: i32;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    let mut numberOfVars: i32;
    let mut requiredTime: metamodelica::Real;
    let mut tmpComm: Communication;
    let Communication {
        numberOfVars: __pa0,
        integerVars: __pa1,
        floatVars: __pa2,
        booleanVars: __pa3,
        stringVars: __pa4,
        childNode: __pa5,
        requiredTime: __pa6,
    } = iEdgeSource;
    numberOfVars = metamodelica::Own::own(__pa0);
    integerVars = metamodelica::Own::own(__pa1);
    floatVars = metamodelica::Own::own(__pa2);
    booleanVars = metamodelica::Own::own(__pa3);
    stringVars = metamodelica::Own::own(__pa4);
    sourceSccIdx = metamodelica::Own::own(__pa5);
    requiredTime = metamodelica::Own::own(__pa6);
    oldComms = metamodelica::arrayGet(iCommCosts.clone(), sourceSccIdx)?;
    tmpComm = Communication {
        numberOfVars: numberOfVars,
        integerVars: integerVars,
        floatVars: floatVars,
        booleanVars: booleanVars,
        stringVars: stringVars,
        childNode: iEdgeTarget,
        requiredTime: requiredTime,
    };
    oCommCosts = metamodelica::arrayUpdate(iCommCosts.clone(), sourceSccIdx, metamodelica::cons(tmpComm, oldComms))?;
    Ok(oCommCosts)
}

fn fillAdjacencyList(
    mut adjLstIn: metamodelica::Array<metamodelica::List<i32>>,
    mut childNode: i32,
    mut parentLst: &Communications,
    mut Idx: i32,
) -> metamodelica::Array<metamodelica::List<i32>> {
    let mut adjLstOut: metamodelica::Array<metamodelica::List<i32>>;
    adjLstOut = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parentNode: Communication;
            let mut parentRow: metamodelica::List<i32>;
            let mut adjLst: metamodelica::Array<metamodelica::List<i32>>;
            let mut parentNodeIdx: i32;
            let true = (((parentLst).len() as i32) >= Idx) else {
                return Err("pattern mismatch");
            };
            parentNode = (parentLst).get(Idx)?;
            let Communication { childNode: __pa0, .. } = &parentNode;
            parentNodeIdx = metamodelica::Own::own(__pa0);
            parentRow = metamodelica::arrayGet(adjLstIn.clone(), parentNodeIdx)?;
            parentRow = metamodelica::cons(childNode, parentRow.clone());
            parentRow = List::removeOnTrue(parentNodeIdx, &fnptr!(intEq, i32, i32), parentRow.clone())?;
            adjLst = metamodelica::arrayUpdate(adjLstIn.clone(), parentNodeIdx, parentRow.clone())?;
            adjLst = fillAdjacencyList(adjLst.clone(), childNode, parentLst, Idx + 1);
            Ok(adjLst.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(adjLstIn.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    adjLstOut
}

fn getEquationStrings(
    mut iComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Array<ArcStr>> {
    let mut eqDescsOut: metamodelica::Array<ArcStr>;
    let mut eqDescs: metamodelica::List<ArcStr>;
    eqDescs = List::fold1(
        iComps,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: metamodelica::Ref<BackendDAE::EqSystem>,
               __a2: metamodelica::List<ArcStr>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getEquationStrings2(&__a0, &__a1, __a2))
        },
        iEqSystem,
        metamodelica::nil(),
    )?;
    eqDescs = eqDescs.reverse();
    eqDescsOut = metamodelica::arrayFromVec(eqDescs.into_iter().cloned().collect());
    Ok(eqDescsOut)
}

fn getEquationStrings2(
    mut comp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut iEqDesc: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut oEqDesc: metamodelica::List<ArcStr>;
    oEqDesc = 'mc: {
        let __mc_input = (&**comp, &**iEqSystem);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. }) => {
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut varString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varString = getVarString(&(BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&orderedVars), v.clone())?))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR ")); __mm_s.push_str(&*varString); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: _ }, .. }, Deref @ BackendDAE::EqSystem { .. }) => {
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut desc: ArcStr;
                    desc = literal!("Equation System");
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ARRAY:")); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), List::map(varLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| getVarString(&__a0))?)?, literal!(" AND "))); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALGO: ")); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), List::map(varLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| getVarString(&__a0))?)?, literal!(" AND "))); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("COMPLEX: ")); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), List::map(varLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| getVarString(&__a0))?)?, literal!(" AND "))); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WHEN:")); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), List::map(varLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| getVarString(&__a0))?)?, literal!(" AND "))); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = BackendDump::equationString(&(BackendEquation::get(orderedEqs.clone(), i.clone())?))?;
                    varLst = BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?;
                    desc = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IFEQ:")); __mm_s.push_str(&*eqString); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), List::map(varLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Var>| getVarString(&__a0))?)?, literal!(" AND "))); ArcStr::from(__mm_s) };
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut desc: ArcStr;
                    desc = literal!("Torn linear System");
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut desc: ArcStr;
                    desc = literal!("Torn nonlinear System");
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut descLst: metamodelica::List<ArcStr>;
                    let mut desc: ArcStr;
                    desc = literal!("no singleEquation");
                    descLst = metamodelica::cons(desc.clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oEqDesc
}

pub(crate) fn getVarString(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<ArcStr> {
    let mut varString: ArcStr = arcstr::literal!("");
    varString = 'mc: {
        let __mc_input = &**inVar;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varDescLst: metamodelica::List<ArcStr>;
                    let mut varString: ArcStr = varString.clone();
                    let true = (BackendVariable::isNonStateVar(inVar)) else { return Err("pattern mismatch") };
                    varString = BackendDump::varString(inVar)?;
                    varDescLst = stringListStringChar(varString.clone());
                    varDescLst = shortenVarString(varDescLst.clone())?;
                    varString = stringCharListString(varDescLst.clone());
                    Ok((varString.clone(), varString.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            varString = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut varDescLst: metamodelica::List<ArcStr>;
                    let mut varString: ArcStr = varString.clone();
                    let false = (BackendVariable::isNonStateVar(inVar)) else { return Err("pattern mismatch") };
                    varString = BackendDump::varString(inVar)?;
                    varDescLst = stringListStringChar(varString.clone());
                    varDescLst = shortenVarString(varDescLst.clone())?;
                    varString = stringCharListString(varDescLst.clone());
                    varString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" der(")); __mm_s.push_str(&*varString); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Ok((varString.clone(), varString.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            varString = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(varString)
}

fn shortenVarString(mut iString: metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut oString: metamodelica::List<ArcStr>;
    let mut pos: i32;
    pos = List::position(literal!(":"), &iString)? - 1;
    (oString, _) = List::split(iString, pos)?;
    Ok(oString)
}

fn getEventNodes(
    mut systIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut eventNodes: metamodelica::List<i32>;
    let mut eqLst: metamodelica::List<i32>;
    let mut systemsIn: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*systIn);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    systemsIn = metamodelica::Own::own(__pa0);
    (eqLst, _) = List::fold(
        &systemsIn,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: (metamodelica::List<i32>, i32)| {
            getEventNodeEqs(&__a0, &__a1)
        },
        (metamodelica::nil(), 0),
    )?;
    eventNodes = getArrayTuple31(eqLst, eqCompMapping.clone())?;
    Ok(eventNodes)
}

fn getEventNodeEqs(
    mut systIn: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut eventInfoIn: &(metamodelica::List<i32>, i32),
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut eventInfoOut: (metamodelica::List<i32>, i32);
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eventEqs: metamodelica::List<i32>;
    let mut eventEqsIn: metamodelica::List<i32>;
    let mut offset: i32;
    let __arc2 = &(*systIn);
    let BackendDAE::EQSYSTEM {
        orderedEqs: __pa0,
        matching: __pa1,
        ..
    } = &**__arc2;
    orderedEqs = metamodelica::Own::own(__pa0);
    matching = metamodelica::Own::own(__pa1);
    comps = BackendDAEUtil::getCompsOfMatching(&matching);
    (eventEqsIn, offset) = eventInfoIn.clone();
    eventEqs = getEventNodeEqs1(&comps, offset, &(metamodelica::nil()))?;
    offset = offset + ExpandableArray::getNumberOfElements(orderedEqs);
    eventInfoOut = (listAppend(eventEqs, eventEqsIn), offset);
    Ok(eventInfoOut)
}

fn getEventNodeEqs1(
    mut comps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut offset: i32,
    mut eventEqsIn: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut eventEqsOut: metamodelica::List<i32>;
    eventEqsOut = 'mc: {
        let __mc_input = &**comps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut eqn: i32;
                    let mut eventEqs: metamodelica::List<i32>;
                    let true = (isWhenEquation(metamodelica::AsArg::as_arg(&head))) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(head.clone()) {
                        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa0);
                    eqn = eqn + offset;
                    eventEqs = getEventNodeEqs1(metamodelica::AsArg::as_arg(&rest), offset, &(metamodelica::cons(eqn, eventEqsIn.clone())))?;
                    Ok(eventEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut eventEqs: metamodelica::List<i32>;
                    let false = (isWhenEquation(metamodelica::AsArg::as_arg(&head))) else { return Err("pattern mismatch") };
                    eventEqs = getEventNodeEqs1(metamodelica::AsArg::as_arg(&rest), offset, eventEqsIn)?;
                    Ok(eventEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(eventEqsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(eventEqsOut)
}

fn getArrayTuple31(
    mut list1: metamodelica::List<i32>,
    mut assign: metamodelica::Array<(i32, i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut list2Out: metamodelica::List<i32>;
    let mut tplLst: metamodelica::List<(i32, i32, i32)>;
    tplLst = List::map1(list1, &Array::getIndexFirst, assign.clone())?;
    list2Out = List::map(tplLst, &fnptr!(Util::tuple31, _))?;
    Ok(list2Out)
}

fn isWhenEquation(mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>) -> bool {
    let mut isWhenEq: bool;
    isWhenEq = (match &**inComp {
        BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. } => true,
        _ => false,
    });
    isWhenEq
}

fn fillRequiredSccs(
    mut var: (i32, i32),
    mut varType: VariableType,
    mut varMapping: metamodelica::Array<(i32, i32, i32)>,
    mut requiredSccs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            i32,
            (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ),
        >,
    >,
) -> Result<()> {
    let __ab_varMapping = varMapping.borrow();
    let mut var_idx: i32;
    let mut scc_idx: i32;
    let mut not_derived: i32;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    (var_idx, not_derived) = var;
    if not_derived == 1 {
        (scc_idx, _, _) = (*metamodelica::index_checked(&__ab_varMapping, var_idx)?).clone();
        (integerVars, floatVars, booleanVars, stringVars) = UnorderedMap::getOrDefault(
            scc_idx,
            requiredSccs.clone(),
            (
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
            ),
        )?;
        let () = (match varType {
            VariableType::INTEGER { .. } => {
                integerVars = metamodelica::cons(var_idx, integerVars);
                ()
            }
            VariableType::REAL { .. } => {
                floatVars = metamodelica::cons(var_idx, floatVars);
                ()
            }
            VariableType::BOOLEAN => {
                booleanVars = metamodelica::cons(var_idx, booleanVars);
                ()
            }
            VariableType::STRING { .. } => {
                stringVars = metamodelica::cons(var_idx, stringVars);
                ()
            }
        });
        UnorderedMap::add(scc_idx, (integerVars, floatVars, booleanVars, stringVars), requiredSccs)?;
    }
    Ok(())
}

fn getUnsolvedVarsBySCC(
    mut iComponent: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iAdjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut iOrderedVars: BackendDAE::Variables,
    mut iKnownVars: BackendDAE::Variables,
    mut iOrderedEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iEventVarLst: &metamodelica::List<i32>,
    mut iAnalyzeParameters: bool,
) -> Result<(
    (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    metamodelica::List<i32>,
)> {
    let mut oUnsolvedVars: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut oParamVars: metamodelica::List<i32>;
    (oUnsolvedVars, oParamVars) = 'mc: {
        let __mc_input = &**iComponent;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: varIdx, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), list![varIdx.clone()], iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: varIdc, .. }, .. } => {
                    let mut tmpVars: (metamodelica::List<i32>, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, metamodelica::List<i32>);
                    let mut paramVars: metamodelica::List<i32>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst, iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getUnsolvedVarsBySCC failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC0(
    mut iComponent: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iAdjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut iOrderedVars: BackendDAE::Variables,
    mut iKnownVars: BackendDAE::Variables,
    mut iOrderedEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iVarIdc: metamodelica::List<i32>,
    mut iEventVarLst: &metamodelica::List<i32>,
    mut iAnalyzeParameters: bool,
) -> Result<(
    (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    metamodelica::List<i32>,
)> {
    let mut oUnsolvedVars: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut oParamVars: metamodelica::List<i32>;
    let mut tmpVars: metamodelica::List<(i32, i32)>;
    (tmpVars, oParamVars) = getVarsBySCC(
        iComponent,
        iAdjacencyMatrix.clone(),
        &iOrderedVars,
        iKnownVars,
        iOrderedEquations,
        iAnalyzeParameters,
    )?;
    tmpVars = List::filter1OnTrue(
        tmpVars,
        (std::sync::Arc::new(
            move |__a0: (i32, i32), __a1: metamodelica::List<i32>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isTupleMember(__a0, &__a1))
            },
        )
            as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::List<i32>) -> Result<bool> + 'static>),
        iVarIdc,
    )?;
    tmpVars = removeEventVars(iEventVarLst, &tmpVars, 1);
    oUnsolvedVars = List::fold1(
        &tmpVars,
        &move |__a0: (i32, i32),
               __a1: BackendDAE::Variables,
               __a2: (
            metamodelica::List<i32>,
            metamodelica::List<(i32, i32)>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        )| getUnsolvedVarsBySCC1(__a0, &__a1, &__a2),
        iOrderedVars,
        (
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
        ),
    )?;
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC1(
    mut iVarIdx: (i32, i32),
    mut orderedVars: &BackendDAE::Variables,
    mut iUnsolvedVars: &(
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut oUnsolvedVars: (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut varType: metamodelica::Ref<DAE::Type>;
    var = BackendVariable::getVarAt(orderedVars, Util::tuple21(iVarIdx))?;
    varType = BackendVariable::getVarType(&var);
    oUnsolvedVars = getUnsolvedVarsBySCC2(varType, iVarIdx, iUnsolvedVars);
    Ok(oUnsolvedVars)
}

fn getUnsolvedVarsBySCC2<'__b>(
    mut iVarType: metamodelica::Ref<DAE::Type>,
    mut iVarIdx: (i32, i32),
    mut iUnsolvedVars: &'__b (
        metamodelica::List<i32>,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
) -> (
    metamodelica::List<i32>,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
) {
    '__tco: loop {
        let mut intVarIdc: metamodelica::List<i32>;
        let mut boolVarIdc: metamodelica::List<i32>;
        let mut stringVarIdc: metamodelica::List<i32>;
        let mut realVarIdc: metamodelica::List<(i32, i32)>;
        let mut varIdx: i32;
        let mut derived: i32;
        let mut ty: metamodelica::Ref<DAE::Type>;
        ::match_deref::match_deref! { match &((iVarType, iVarIdx, iUnsolvedVars.clone())) {
            (Deref @ DAE::Type::T_INTEGER { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                intVarIdc = metamodelica::cons(varIdx.clone(), intVarIdc.clone());
                return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
            },
            (Deref @ DAE::Type::T_REAL { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                realVarIdc = metamodelica::cons((varIdx.clone(), derived.clone()), realVarIdc.clone());
                return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
            },
            (Deref @ DAE::Type::T_BOOL { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                boolVarIdc = metamodelica::cons(varIdx.clone(), boolVarIdc.clone());
                return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
            },
            (Deref @ DAE::Type::T_ARRAY { ty: __esc_ty, .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                ty = (*__esc_ty).clone();
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                { (iVarType, iVarIdx, iUnsolvedVars) = (ty.clone(), iVarIdx, iUnsolvedVars); continue '__tco; }
            },
            (Deref @ DAE::Type::T_ENUMERATION { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                stringVarIdc = metamodelica::cons(varIdx.clone(), stringVarIdc.clone());
                return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
            },
            (Deref @ DAE::Type::T_STRING { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
                varIdx = (*__esc_varIdx).clone();
                derived = (*__esc_derived).clone();
                intVarIdc = (*__esc_intVarIdc).clone();
                realVarIdc = (*__esc_realVarIdc).clone();
                boolVarIdc = (*__esc_boolVarIdc).clone();
                stringVarIdc = (*__esc_stringVarIdc).clone();
                stringVarIdc = metamodelica::cons(varIdx.clone(), stringVarIdc.clone());
                return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
            },
            _ => {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getUnsolvedVarsBySCC2: Warning, unknown varType for variable ")); __mm_s.push_str(&*intString(Util::tuple21(iVarIdx))); __mm_s.push_str(&*literal!(" !\n")); ArcStr::from(__mm_s) });
                return iUnsolvedVars.clone()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn removeEventVars(
    mut eventVarLst: &metamodelica::List<i32>,
    mut varLstIn: &metamodelica::List<(i32, i32)>,
    mut varIdx: i32,
) -> metamodelica::List<(i32, i32)> {
    let mut varLstOut: metamodelica::List<(i32, i32)>;
    varLstOut = 'mc: {
        let __mc_input = varIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut varTpl: (i32, i32);
            let mut varLst: metamodelica::List<(i32, i32)>;
            let mut var: i32;
            let true = (intLe(varIdx, ((varLstIn).len() as i32))) else {
                return Err("pattern mismatch");
            };
            varTpl = (varLstIn).get(varIdx)?;
            (var, _) = varTpl;
            let true = (List::isMemberOnTrue(var, eventVarLst, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            varLst = listDelete(varLstIn.clone(), varIdx)?;
            varLst = removeEventVars(eventVarLst, &varLst, varIdx);
            Ok(varLst.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut varTpl: (i32, i32);
            let mut varLst: metamodelica::List<(i32, i32)>;
            let mut var: i32;
            let true = (intLe(varIdx, ((varLstIn).len() as i32))) else {
                return Err("pattern mismatch");
            };
            varTpl = (varLstIn).get(varIdx)?;
            (var, _) = varTpl;
            let false = (List::isMemberOnTrue(var, eventVarLst, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            varLst = removeEventVars(eventVarLst, varLstIn, varIdx + 1);
            Ok(varLst.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(varLstIn.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    varLstOut
}

fn isTupleMember(mut inTuple: (i32, i32), mut varIdc: &metamodelica::List<i32>) -> bool {
    let mut isNotMember: bool;
    let mut varIdx: i32;
    let mut varState: i32;
    let mut returnValue: bool = false;
    isNotMember = 'mc: {
        let __mc_input = inTuple;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut varIdx, mut varState) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut returnValue: bool = returnValue.clone();
            let true = (intGt(varIdx, 0)) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(varState, 1)) else {
                return Err("pattern mismatch");
            };
            returnValue = List::isMemberOnTrue(varIdx, varIdc, &fnptr!(intEq, i32, i32))?;
            Ok((!(returnValue), returnValue.clone()))
        })() {
            returnValue = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(true)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isNotMember
}

fn compareTupleByVarIdx(mut varIdx: i32, mut var2Idx: (i32, i32)) -> bool {
    let mut equal: bool;
    equal = intEq(Util::tuple21(var2Idx), varIdx);
    equal
}

pub(crate) fn compareTasksByExecTime(
    mut iTask1: i32,
    mut iTask2: i32,
    mut iTaskComps: metamodelica::Array<metamodelica::List<i32>>,
    mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut iDescending: bool,
) -> Result<bool> {
    let mut oResult: bool;
    let mut exeCosts1: metamodelica::Real;
    let mut exeCosts2: metamodelica::Real;
    let mut taskComps1: metamodelica::List<i32>;
    let mut taskComps2: metamodelica::List<i32>;
    taskComps1 = metamodelica::arrayGet(iTaskComps.clone(), iTask1)?;
    taskComps2 = metamodelica::arrayGet(iTaskComps.clone(), iTask2)?;
    exeCosts1 = addUpExeCostsForNode(taskComps1, iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    exeCosts2 = addUpExeCostsForNode(taskComps2, iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    if iDescending {
        oResult = realLt(exeCosts1, exeCosts2);
    } else {
        oResult = realGt(exeCosts1, exeCosts2);
    }
    Ok(oResult)
}

fn getVarsBySCC(
    mut iComponent: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iAdjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut iOrderedVars: &BackendDAE::Variables,
    mut iKnownVars: BackendDAE::Variables,
    mut iOrderedEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iAnalyzeParameters: bool,
) -> Result<(metamodelica::List<(i32, i32)>, metamodelica::List<i32>)> {
    let mut oVars: metamodelica::List<(i32, i32)>;
    let mut oParamVars: metamodelica::List<i32>;
    (oVars, oParamVars) = (match &**iComponent {
        BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                eqns,
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqnVars, paramVars) = getVarsByEqns(
                &(list![eqnIdx.clone()]),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        BackendDAE::StrongComponent::TORNSYSTEM {
            strictTearingSet:
                BackendDAE::TearingSet {
                    residualequations: resEqns,
                    innerEquations,
                    ..
                },
            ..
        } => {
            let mut eqns: metamodelica::List<i32>;
            let mut eqnVars: metamodelica::List<(i32, i32)>;
            let mut paramVars: metamodelica::List<i32>;
            (eqns, _, _) = List::map_3(
                metamodelica::AsArg::as_arg(&innerEquations),
                &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0))
                },
            )?;
            (eqnVars, paramVars) = getVarsByEqns(
                &(listAppend(resEqns.clone(), eqns)),
                iAdjacencyMatrix.clone(),
                iOrderedVars,
                iKnownVars,
                iOrderedEquations,
                iAnalyzeParameters,
            )?;
            (eqnVars, paramVars)
        }
        _ => {
            metamodelica::print(literal!("Error in getVarsBySCC! Unsupported component-type \n"));
            return Err("fail");
        }
    });
    Ok((oVars, oParamVars))
}

fn tupleToString(mut inTuple: (i32, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = (match inTuple {
        (mut int1, mut int2) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(int1));
            __mm_s.push_str(&*literal!(","));
            __mm_s.push_str(&*intString(int2));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    result
}

fn tuple3ToString(mut inTuple: (i32, i32, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = (match inTuple {
        (mut int1, mut int2, mut int3) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(int1));
            __mm_s.push_str(&*literal!(","));
            __mm_s.push_str(&*intString(int2));
            __mm_s.push_str(&*literal!(","));
            __mm_s.push_str(&*intString(int3));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    result
}

fn getVarsByEqns(
    mut iEqnIdc: &metamodelica::List<i32>,
    mut iAdjacencyMatrix: metamodelica::Array<metamodelica::List<i32>>,
    mut iOrderedVars: &BackendDAE::Variables,
    mut iKnownVars: BackendDAE::Variables,
    mut iOrderedEquations: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iAnalyzeParameters: bool,
) -> Result<(metamodelica::List<(i32, i32)>, metamodelica::List<i32>)> {
    let mut oAdjacencyVars: metamodelica::List<(i32, i32)>;
    let mut oParamVars: metamodelica::List<i32>;
    let mut adjacencyVars: metamodelica::List<i32> = metamodelica::nil();
    let mut paramVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    for mut eqIdx in &**iEqnIdc {
        adjacencyVars = listAppend(
            metamodelica::arrayGet(iAdjacencyMatrix.clone(), eqIdx.clone())?,
            adjacencyVars,
        );
        eqs = metamodelica::cons(BackendEquation::get(iOrderedEquations.clone(), eqIdx.clone())?, eqs);
    }
    oAdjacencyVars = List::map(adjacencyVars, &fnptr!(getVarTuple, i32))?;
    if iAnalyzeParameters {
        (paramVars, oParamVars) = BackendEquation::equationsParams(&eqs, iKnownVars)?;
    } else {
        oParamVars = metamodelica::nil();
    }
    Ok((oAdjacencyVars, oParamVars))
}

fn getVarTuple(mut varIdx: i32) -> (i32, i32) {
    let mut outIdx: (i32, i32);
    outIdx = if (intLe(0, varIdx)) {
        (varIdx, 1)
    } else {
        (-(varIdx), 0)
    };
    outIdx
}

fn compareIntTuple2(mut tuple1: (i32, i32), mut tuple2: (i32, i32)) -> bool {
    let mut equals: bool;
    equals = (match (tuple1, tuple2) {
        ((mut int1, mut int2), (mut int3, mut int4))
            if (int1.clone() == int3.clone() && int2.clone() == int4.clone()) =>
        {
            true
        }
        _ => false,
    });
    equals
}

fn getVarEqCompMapping(
    mut components: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iEqSysIdx: i32,
    mut iVarIdxOffset: i32,
    mut iEqIdxOffset: i32,
    mut ivarCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(
    metamodelica::Array<(i32, i32, i32)>,
    metamodelica::Array<(i32, i32, i32)>,
)> {
    let mut ovarCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut oeqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    List::fold4(
        components,
        &move |__a0: metamodelica::Ref<BackendDAE::StrongComponent>,
               __a1: metamodelica::Array<(i32, i32, i32)>,
               __a2: metamodelica::Array<(i32, i32, i32)>,
               __a3: i32,
               __a4: (i32, i32),
               __a5: i32| getVarEqCompMapping0(&__a0, __a1, __a2, __a3, __a4, __a5),
        ivarCompMapping.clone(),
        ieqCompMapping.clone(),
        iEqSysIdx,
        (iVarIdxOffset, iEqIdxOffset),
        1,
    )?;
    ovarCompMapping = ivarCompMapping.clone();
    oeqCompMapping = ieqCompMapping.clone();
    Ok((ovarCompMapping, oeqCompMapping))
}

fn getVarEqCompMapping0(
    mut component: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut iEqSysIdx: i32,
    mut iVarEqOffset: (i32, i32),
    mut iSccIdx: i32,
) -> Result<i32> {
    let mut oSccIdx: i32;
    oSccIdx = 'mc: {
        let __mc_input = (&**component, iVarEqOffset);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: compVarIdx, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    metamodelica::arrayUpdate(varCompMapping.clone(), compVarIdx.clone() + iVarOffset.clone(), (iSccIdx, iEqSysIdx, iVarOffset.clone()))?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: compVarIdc, eqns, .. }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    List::fold3(metamodelica::AsArg::as_arg(&eqns), &updateMappingTuple, iSccIdx, iEqSysIdx, iEqOffset.clone(), eqCompMapping.clone())?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: compVarIdc, residualequations: residuals, innerEquations, .. }, .. }, (iVarOffset, iEqOffset)) => {
                    let mut eqns: metamodelica::List<i32>;
                    let mut othereqs: metamodelica::List<i32>;
                    let mut othervars: metamodelica::List<i32>;
                    let mut othervarsLst: metamodelica::List<metamodelica::List<i32>>;
                    let mut compVarIdc = (*compVarIdc).clone();
                    (othereqs, othervarsLst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    othervars = List::flatten(othervarsLst.clone())?;
                    compVarIdc = listAppend(othervars.clone(), compVarIdc.clone());
                    eqns = listAppend(othereqs.clone(), residuals.clone());
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    List::fold3(&eqns, &updateMappingTuple, iSccIdx, iEqSysIdx, iEqOffset.clone(), eqCompMapping.clone())?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(metamodelica::AsArg::as_arg(&compVarIdc), &updateMappingTuple, iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut helperStr: ArcStr;
                    helperStr = BackendDump::strongComponentString(component)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getVarEqCompMapping0 - Unsupported component-type:\n")); __mm_s.push_str(&*helperStr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oSccIdx)
}

pub(crate) fn getSccNodeMapping(
    mut iNumberOfSccs: i32,
    mut iTaskGraphMeta: TaskGraphMeta,
) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    tmpMappingArray = arrayCreate(iNumberOfSccs, -1);
    let TaskGraphMeta {
        inComps: __pa0,
        nodeMark: __pa1,
        ..
    } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    nodeMark = metamodelica::Own::own(__pa1);
    (oMapping, _) = Array::fold(
        inComps.clone(),
        &({
            let __pe_b1 = nodeMark.clone();
            move |__pe_a0, __pe_a2| getSccNodeMapping0(&__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        (tmpMappingArray.clone(), 1),
    )?;
    Ok(oMapping)
}

fn getSccNodeMapping0(
    mut iCompsOfNode: &metamodelica::List<i32>,
    mut iNodeMarks: metamodelica::Array<i32>,
    mut iArrayNodeIdx: (metamodelica::Array<i32>, i32),
) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut nodeIdx: i32;
    (tmpMappingArray, nodeIdx) = List::fold1(
        iCompsOfNode,
        &fnptr!(
            getSccNodeMapping1,
            i32,
            metamodelica::Array<i32>,
            (metamodelica::Array<i32>, i32)
        ),
        iNodeMarks.clone(),
        iArrayNodeIdx,
    )?;
    oArrayNodeIdx = (tmpMappingArray.clone(), nodeIdx + 1);
    Ok(oArrayNodeIdx)
}

fn getSccNodeMapping1(
    mut iCompIdx: i32,
    mut iNodeMark: metamodelica::Array<i32>,
    mut iArrayNodeIdx: (metamodelica::Array<i32>, i32),
) -> (metamodelica::Array<i32>, i32) {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut iNodeIdx: i32;
    let mut nodeMark: i32 = 0;
    let mut iMappingArray: metamodelica::Array<i32>;
    oArrayNodeIdx = 'mc: {
        let __mc_input = iArrayNodeIdx;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut iMappingArray, mut iNodeIdx) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut nodeMark: i32 = nodeMark.clone();
            nodeMark = metamodelica::arrayGet(iNodeMark.clone(), iCompIdx)?;
            let true = (intNe(-1, nodeMark)) else {
                return Err("pattern mismatch");
            };
            iMappingArray = metamodelica::arrayUpdate(iMappingArray.clone(), iCompIdx, iNodeIdx)?;
            Ok(((iMappingArray.clone(), iNodeIdx), nodeMark.clone()))
        })() {
            nodeMark = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut iMappingArray, mut iNodeIdx) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((iMappingArray.clone(), iNodeIdx))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oArrayNodeIdx
}

fn othersInTearComp(
    mut otherEqnVarTpl: &(i32, metamodelica::List<i32>),
    mut othersIn: &(metamodelica::List<i32>, metamodelica::List<i32>),
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut othersOut: (metamodelica::List<i32>, metamodelica::List<i32>);
    othersOut = 'mc: {
        let __mc_input = othersIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eq: i32;
                    let mut eqLst: metamodelica::List<i32>;
                    let mut varTplLst: metamodelica::List<i32>;
                    let mut varLst: metamodelica::List<i32>;
                    (eq, varTplLst) = otherEqnVarTpl.clone();
                    (varTplLst).get(1)?;
                    (eqLst, varLst) = othersIn.clone();
                    varLst = listAppend(varTplLst.clone(), varLst.clone());
                    eqLst = metamodelica::cons(eq, eqLst.clone());
                    Ok((eqLst.clone(), varLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("check number of vars in relation to number of eqs in otherEqnVarTpl in the torn system\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(othersOut)
}

fn updateMapping(
    mut varIdx: i32,
    mut sccIdx: i32,
    mut iMapping: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), varIdx, sccIdx)?;
    Ok(oMapping)
}

fn updateMappingTuple(
    mut varIdx: i32,
    mut sccIdx: i32,
    mut iEqSysIdx: i32,
    mut iVarOffset: i32,
    mut iMapping: metamodelica::Array<(i32, i32, i32)>,
) -> Result<metamodelica::Array<(i32, i32, i32)>> {
    let mut oMapping: metamodelica::Array<(i32, i32, i32)>;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), varIdx + iVarOffset, (sccIdx, iEqSysIdx, iVarOffset))?;
    Ok(oMapping)
}

//--------------------------------------------------------
//  Functions to get the ODEsystem graph and adjacencyList
//--------------------------------------------------------
pub(crate) fn getOdeSystem(
    mut graphIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
    mut systIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOdeOut: TaskGraph;
    let mut graphDataOdeOut: TaskGraphMeta;
    let mut stateNodes: metamodelica::List<i32>;
    let mut whenNodes: metamodelica::List<i32>;
    let mut cutNodes: metamodelica::List<i32>;
    let mut cutNodeChildren: metamodelica::List<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut graphTmp: TaskGraph;
    let TaskGraphMeta {
        varCompMapping: __pa0,
        eqCompMapping: __pa1,
        inComps: __pa2,
        ..
    } = &graphDataIn;
    varCompMapping = metamodelica::Own::own(__pa0);
    eqCompMapping = metamodelica::Own::own(__pa1);
    inComps = metamodelica::Own::own(__pa2);
    let __arc4 = &(*systIn);
    let BackendDAE::DAE { eqs: __pa3, shared: _ } = &**__arc4;
    systs = metamodelica::Own::own(__pa3);
    (stateNodes, _) = List::fold2(
        &systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: metamodelica::Array<(i32, i32, i32)>,
               __a2: metamodelica::Array<metamodelica::List<i32>>,
               __a3: (metamodelica::List<i32>, i32)| getAllStateNodes(&__a0, __a1, __a2, &__a3),
        varCompMapping.clone(),
        inComps.clone(),
        (metamodelica::nil(), 0),
    )?;
    whenNodes = getEventNodes(systIn, eqCompMapping.clone())?;
    graphTmp = metamodelica::arrayFromVec(graphIn.clone().borrow().clone());
    (graphOdeOut, cutNodes) = cutTaskGraph(graphTmp.clone(), stateNodes, whenNodes.clone())?;
    cutNodeChildren = List::flatten(List::map1(
        listAppend(cutNodes.clone(), whenNodes),
        &Array::getIndexFirst,
        graphIn.clone(),
    )?)?;
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren, cutNodes.clone(), &fnptr!(intEq, i32, i32))?;
    graphDataOdeOut = cutSystemData(graphDataIn, listAppend(cutNodes, metamodelica::nil()), &cutNodeChildren)?;
    Ok((graphOdeOut, graphDataOdeOut))
}

fn getAllStateNodes(
    mut systIn: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut stateInfoIn: &(metamodelica::List<i32>, i32),
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut stateInfoOut: (metamodelica::List<i32>, i32);
    stateInfoOut = 'mc: {
        let __mc_input = stateInfoIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stateNodesIn, varOffset) => {
                    let mut stateNodes: metamodelica::List<i32>;
                    let mut stateVars: metamodelica::List<i32>;
                    let mut varOffsetNew: i32;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let __arc1 = &(*systIn);
                    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
                    orderedVars = metamodelica::Own::own(__pa0);
                    varLst = BackendVariable::varList(&orderedVars)?;
                    stateVars = getStates(&varLst, &(metamodelica::nil()), 1)?;
                    let false = ((stateVars).is_empty()) else { return Err("pattern mismatch") };
                    stateVars = List::map1(stateVars.clone(), &fnptr!(intAdd, i32, i32), varOffset.clone())?;
                    stateNodes = getArrayTuple31(stateVars.clone(), varCompMapping.clone())?;
                    stateNodes = List::map3(stateNodes.clone(), &getCompInComps, 1, inComps.clone(), arrayCreate(metamodelica::arrayLength(inComps.clone()), 0))?;
                    stateNodes = listAppend(stateNodesIn.clone(), stateNodes.clone());
                    varOffsetNew = ((varLst).len() as i32) + varOffset.clone();
                    Ok((stateNodes.clone(), varOffsetNew))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stateNodesIn, varOffset) => {
                    let mut stateVars: metamodelica::List<i32>;
                    let mut varOffsetNew: i32;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let __arc1 = &(*systIn);
                    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
                    orderedVars = metamodelica::Own::own(__pa0);
                    varLst = BackendVariable::varList(&orderedVars)?;
                    stateVars = getStates(&varLst, &(metamodelica::nil()), 1)?;
                    let true = ((stateVars).is_empty()) else { return Err("pattern mismatch") };
                    varOffsetNew = ((varLst).len() as i32) + varOffset.clone();
                    Ok((stateNodesIn.clone(), varOffsetNew))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut stateVars: metamodelica::List<i32>;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let __arc1 = &(*systIn);
                    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &**__arc1;
                    orderedVars = metamodelica::Own::own(__pa0);
                    varLst = BackendVariable::varList(&orderedVars)?;
                    stateVars = getStates(&varLst, &(metamodelica::nil()), 1)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getAllStateNodes failed! StateVars-Count: ")); __mm_s.push_str(&*intString(((stateVars).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(stateInfoOut)
}

fn getStates(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut stateVarsIn: &metamodelica::List<i32>,
    mut Idx: i32,
) -> Result<metamodelica::List<i32>> {
    let mut stateVarsOut: metamodelica::List<i32>;
    stateVarsOut = 'mc: {
        let __mc_input = &**inVarLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut stateVars: metamodelica::List<i32>;
                    let false = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&head))) else { return Err("pattern mismatch") };
                    stateVars = getStates(metamodelica::AsArg::as_arg(&rest), stateVarsIn, Idx + 1)?;
                    Ok(stateVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut stateVars: metamodelica::List<i32>;
                    let true = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&head))) else { return Err("pattern mismatch") };
                    stateVars = getStates(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(Idx, stateVarsIn.clone())), Idx + 1)?;
                    Ok(stateVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(stateVarsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(stateVarsOut)
}

fn cutTaskGraph(
    mut graphIn: TaskGraph,
    mut exceptNodes: metamodelica::List<i32>,
    mut whenNodes: metamodelica::List<i32>,
) -> Result<(TaskGraph, metamodelica::List<i32>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: metamodelica::List<i32>;
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = &*exceptNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (-1), tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok((graphIn.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sizeDAE: i32;
                    let mut sizeODE: i32;
                    let mut graphT: TaskGraph;
                    let mut graphODE: TaskGraph;
                    let mut cutNodes: metamodelica::List<i32>;
                    let mut odeNodes: metamodelica::List<i32>;
                    let mut odeMap: metamodelica::Array<i32>;
                    sizeDAE = metamodelica::arrayLength(graphIn.clone());
                    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), sizeDAE)?;
                    odeNodes = listAppend(exceptNodes.clone(), getAllSuccessors(exceptNodes.clone(), graphT.clone())?);
                    (_, odeNodes, _) = List::intersection1OnTrue(odeNodes.clone(), whenNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    (odeNodes, _, _) = List::intersection1OnTrue(List::intRange(sizeDAE), odeNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    odeNodes = List::sort(odeNodes.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    sizeODE = ((odeNodes).len() as i32);
                    odeMap = arrayCreate(sizeDAE, -1);
                    List::threadMap1_0(&odeNodes, List::intRange(sizeODE), &Array::updateIndexFirst, odeMap.clone())?;
                    graphODE = arrayCreate(sizeODE, metamodelica::nil());
                    (graphODE, cutNodes) = cutTaskGraph2(&(List::intRange(sizeDAE)), graphODE.clone(), &(metamodelica::nil()), graphIn.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("cutTaskGraph failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((graphOut, cutNodesOut))
}

fn cutTaskGraph2(
    mut daeNodes: &metamodelica::List<i32>,
    mut graphODE: TaskGraph,
    mut cutNodesIn: &metamodelica::List<i32>,
    mut graphDAE: TaskGraph,
    mut odeMap: metamodelica::Array<i32>,
) -> Result<(TaskGraph, metamodelica::List<i32>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: metamodelica::List<i32>;
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = &**daeNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: daeIdx, tail: rest } => {
                    let mut odeIdx: i32;
                    let mut row: metamodelica::List<i32>;
                    let mut cutNodes: metamodelica::List<i32>;
                    odeIdx = metamodelica::arrayGet(odeMap.clone(), daeIdx.clone())?;
                    let true = (intGt(odeIdx, 0)) else { return Err("pattern mismatch") };
                    row = metamodelica::arrayGet(graphDAE.clone(), daeIdx.clone())?;
                    row = List::map1(row.clone(), &Array::getIndexFirst, odeMap.clone())?;
                    row = List::filter1OnTrue(row.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    metamodelica::arrayUpdate(graphODE.clone(), odeIdx, row.clone())?;
                    (_, cutNodes) = cutTaskGraph2(metamodelica::AsArg::as_arg(&rest), graphODE.clone(), cutNodesIn, graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: daeIdx, tail: rest } => {
                    let mut odeIdx: i32;
                    let mut cutNodes: metamodelica::List<i32>;
                    odeIdx = metamodelica::arrayGet(odeMap.clone(), daeIdx.clone())?;
                    let true = (intEq(odeIdx, -1)) else { return Err("pattern mismatch") };
                    (_, cutNodes) = cutTaskGraph2(metamodelica::AsArg::as_arg(&rest), graphODE.clone(), &(metamodelica::cons(daeIdx.clone(), cutNodesIn.clone())), graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((graphODE.clone(), cutNodesIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((graphOut, cutNodesOut))
}

fn cutSystemData(
    mut graphDataIn: TaskGraphMeta,
    mut cutNodes: metamodelica::List<i32>,
    mut cutNodeChildren: &metamodelica::List<i32>,
) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut rangeLst: metamodelica::List<i32>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = graphDataIn;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    exeCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeMark = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    inComps = metamodelica::arrayFromVec(
        List::deletePositions(
            inComps
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
            cutNodes.clone(),
            false,
        )?
        .into_iter()
        .cloned()
        .collect(),
    );
    rangeLst = List::intRange(metamodelica::arrayLength(nodeMark.clone()));
    nodeMark = List::fold1(
        &rangeLst,
        &move |__a0: i32, __a1: metamodelica::List<i32>, __a2: metamodelica::Array<i32>| {
            markRemovedNodes(__a0, &__a1, __a2)
        },
        cutNodes,
        nodeMark.clone(),
    )?;
    graphDataOut = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok(graphDataOut)
}

fn markRemovedNodes(
    mut nodeMarkIdx: i32,
    mut removedNodes: &metamodelica::List<i32>,
    mut nodeMarkIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = nodeMarkIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intEq(-2, metamodelica::arrayGet(nodeMarkIn.clone(), nodeMarkIdx)?)) else {
                return Err("pattern mismatch");
            };
            Ok(nodeMarkIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let false = (List::isMemberOnTrue(nodeMarkIdx, removedNodes, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            Ok(nodeMarkIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nodeMarkTmp: metamodelica::Array<i32>;
            let true = (List::isMemberOnTrue(nodeMarkIdx, removedNodes, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            nodeMarkTmp = Array::replaceAtWithFill(nodeMarkIdx, -1, 999, nodeMarkIn.clone())?;
            Ok(nodeMarkTmp.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(nodeMarkOut)
}

pub(crate) fn getCompInComps(
    mut compIn: i32,
    mut compIdx: i32,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut nodeMark: metamodelica::Array<i32>,
) -> Result<i32> {
    let mut compOut: i32;
    compOut = 'mc: {
        let __mc_input = nodeMark.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut mergedComp: metamodelica::List<i32>;
            let mut compTmp: i32;
            let true = (metamodelica::arrayLength(inComps.clone()) >= compIdx) else {
                return Err("pattern mismatch");
            };
            mergedComp = metamodelica::arrayGet(inComps.clone(), compIdx)?;
            let false = (List::isMemberOnTrue(compIn, &mergedComp, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            compTmp = getCompInComps(compIn, compIdx + 1, inComps.clone(), nodeMark.clone())?;
            Ok(compTmp)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut mergedComp: metamodelica::List<i32>;
            let true = (metamodelica::arrayLength(inComps.clone()) >= compIdx) else {
                return Err("pattern mismatch");
            };
            mergedComp = metamodelica::arrayGet(inComps.clone(), compIdx)?;
            let true = (List::isMemberOnTrue(compIn, &mergedComp, &fnptr!(intEq, i32, i32))?) else {
                return Err("pattern mismatch");
            };
            Ok(compIdx)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nodeMarkEntry: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMark.clone(), compIn)?;
            let true = (intLt(nodeMarkEntry, 0)) else {
                return Err("pattern mismatch");
            };
            Ok(-1)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("getCompInComps failed! CompIn idx: "));
                __mm_s.push_str(&*intString(compIn));
                __mm_s.push_str(&*literal!(" | Component array-size: "));
                __mm_s.push_str(&*intString(metamodelica::arrayLength(inComps.clone())));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(compOut)
}

pub(crate) fn getAllSuccessors(
    mut nodes: metamodelica::List<i32>,
    mut graph: TaskGraph,
) -> Result<metamodelica::List<i32>> {
    let mut successors: metamodelica::List<i32>;
    successors = 'mc: {
        let __mc_input = graph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut alreadyVisited: metamodelica::Array<bool>;
            let mut check: metamodelica::List<bool>;
            let mut successors1: metamodelica::List<i32>;
            alreadyVisited = arrayCreate(metamodelica::arrayLength(graph.clone()), false);
            List::map2_0(&nodes, &Array::updateIndexFirst, true, alreadyVisited.clone())?;
            successors1 = List::flatten(List::map1(nodes.clone(), &Array::getIndexFirst, graph.clone())?)?;
            check = List::map1(successors1.clone(), &Array::getIndexFirst, alreadyVisited.clone())?;
            (_, successors1) = List::filterOnTrueSync(&check, &fnptr!(boolNot, bool), successors1.clone())?;
            successors1 = List::unique(&successors1);
            Ok(getAllSuccessors2(
                successors1.clone(),
                graph.clone(),
                alreadyVisited.clone(),
                successors1.clone(),
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("getAllSuccessors failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(successors)
}

fn getAllSuccessors2(
    mut nodes: metamodelica::List<i32>,
    mut graph: TaskGraph,
    mut alreadyVisited: metamodelica::Array<bool>,
    mut successorsIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(nodes.clone()) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(List::unique(&successorsIn))
            },
            _ => {
                let mut check: metamodelica::List<bool>;
                let mut successors1: metamodelica::List<i32>;
                successors1 = List::flatten(List::map1(nodes, &Array::getIndexFirst, graph.clone())?)?;
                check = List::map1(successors1.clone(), &Array::getIndexFirst, alreadyVisited.clone())?;
                (_, successors1) = List::filterOnTrueSync(&check, &fnptr!(boolNot, bool), successors1)?;
                successors1 = List::unique(&successors1);
                List::map2_0(&successors1, &Array::updateIndexFirst, true, alreadyVisited.clone())?;
                { (nodes, graph, alreadyVisited, successorsIn) = (successors1.clone(), graph.clone(), alreadyVisited.clone(), listAppend(successors1, successorsIn)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getChildNodes(
    mut adjacencyLstIn: metamodelica::Array<metamodelica::List<i32>>,
    mut parents: &metamodelica::List<i32>,
    mut childLstTmp: &metamodelica::List<i32>,
    mut Idx: i32,
) -> metamodelica::List<i32> {
    let mut childLsts: metamodelica::List<i32>;
    childLsts = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parent: i32;
            let mut row: metamodelica::List<i32>;
            let mut childLst: metamodelica::List<i32>;
            let true = (((parents).len() as i32) >= Idx) else {
                return Err("pattern mismatch");
            };
            parent = (parents).get(Idx)?;
            row = metamodelica::arrayGet(adjacencyLstIn.clone(), parent)?;
            childLst = listAppend(childLstTmp.clone(), row.clone());
            childLst = getChildNodes(adjacencyLstIn.clone(), parents, &childLst, Idx + 1);
            Ok(childLst.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(childLstTmp.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    childLsts
}

pub(crate) fn updateContinuousEntriesInList(
    mut lstIn: metamodelica::List<i32>,
    mut deleteEntriesIn: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut lstOut: metamodelica::List<i32>;
    lstOut = (::match_deref::match_deref! { match &((lstIn.clone(), deleteEntriesIn.clone())) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::ListNode::Nil) => {
            lstIn
        },
        (Deref @ metamodelica::ListNode::Cons { head: start, tail: rest }, _) => {
            let mut lstTmp: metamodelica::List<i32>;
            let mut deleteArr: metamodelica::Array<i32>;
            deleteArr = arrayCreate(List::fold(&(listAppend(rest.clone(), deleteEntriesIn.clone())), &fnptr!(intMax, i32, i32), start.clone())?, 0);
            List::map2_0(&deleteEntriesIn, &Array::updateIndexFirst, 1, deleteArr.clone())?;
            (deleteArr, _) = Array::mapFold(deleteArr.clone(), &setDeleteArr, 0)?;
            lstTmp = List::map1(lstIn, &fnptr!(removeContinuousEntries1, i32, metamodelica::Array<i32>), deleteArr.clone())?;
            lstTmp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(lstOut)
}

fn setDeleteArr(mut entryIn: i32, mut offsetIn: i32) -> Result<(i32, i32)> {
    let mut entryOut: i32;
    let mut offsetOut: i32;
    (entryOut, offsetOut) = (match entryIn {
        0 => (offsetIn, offsetIn),
        1 => (offsetIn + 1, offsetIn + 1),
        _ => return Err("match: no arm matched"),
    });
    Ok((entryOut, offsetOut))
}

fn removeContinuousEntries1(mut entryIn: i32, mut deleteEntriesIn: metamodelica::Array<i32>) -> i32 {
    let mut entryOut: i32;
    entryOut = 'mc: {
        let __mc_input = deleteEntriesIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut offset: i32;
            offset = metamodelica::arrayGet(deleteEntriesIn.clone(), entryIn)?;
            Ok(entryIn - offset)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("removeContinuousEntries1 failed!\n"));
            Ok(entryIn)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    entryOut
}

fn deleteRowInAdjLst(
    mut adjacencyLstIn: metamodelica::Array<metamodelica::List<i32>>,
    mut rowsDel: metamodelica::List<i32>,
) -> Result<(metamodelica::Array<metamodelica::List<i32>>, metamodelica::List<i32>)> {
    let mut adjacencyLstOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut odeMapping: metamodelica::List<i32>;
    let mut adjLst: metamodelica::Array<metamodelica::List<i32>>;
    let mut copiedRows: metamodelica::List<i32>;
    let mut size: i32;
    size = metamodelica::arrayLength(adjacencyLstIn.clone()) - ((rowsDel).len() as i32);
    adjLst = arrayCreate(size, metamodelica::nil());
    copiedRows = List::intRange(metamodelica::arrayLength(adjacencyLstIn.clone()));
    copiedRows = List::deletePositions(copiedRows, rowsDel, false)?;
    adjacencyLstOut = arrayCopyRows(adjacencyLstIn.clone(), adjLst.clone(), &copiedRows, 1);
    odeMapping = copiedRows;
    Ok((adjacencyLstOut, odeMapping))
}

fn arrayCopyRows(
    mut inArray: metamodelica::Array<metamodelica::List<i32>>,
    mut newArray: metamodelica::Array<metamodelica::List<i32>>,
    mut copiedRows: &metamodelica::List<i32>,
    mut Idx: i32,
) -> metamodelica::Array<metamodelica::List<i32>> {
    let mut outArray: metamodelica::Array<metamodelica::List<i32>>;
    outArray = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut copyRow: i32;
            let mut row: metamodelica::List<i32>;
            let mut arrayTmp: metamodelica::Array<metamodelica::List<i32>>;
            let true = (((copiedRows).len() as i32) >= Idx) else {
                return Err("pattern mismatch");
            };
            copyRow = (copiedRows).get(Idx)?;
            row = metamodelica::arrayGet(inArray.clone(), copyRow)?;
            arrayTmp = Array::replaceAtWithFill(Idx, row.clone(), list![111, 222], newArray.clone())?;
            arrayTmp = arrayCopyRows(inArray.clone(), arrayTmp.clone(), copiedRows, Idx + 1);
            Ok(arrayTmp.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(newArray.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outArray
}

pub(crate) fn getRootNodes(mut iTaskGraph: TaskGraph) -> Result<metamodelica::List<i32>> {
    let mut rootsOut: metamodelica::List<i32>;
    let mut size: i32;
    let mut taskGraphT: TaskGraph;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    rootsOut = getLeafNodes(taskGraphT.clone())?;
    Ok(rootsOut)
}

pub(crate) fn getLeafNodes(mut iTaskGraph: TaskGraph) -> Result<metamodelica::List<i32>> {
    let mut oLeafNodes: metamodelica::List<i32>;
    let mut tmpLeafNodes: metamodelica::List<i32>;
    let mut nodeSuccessors: metamodelica::List<i32>;
    let mut nodeIdx: i32 = 0;
    tmpLeafNodes = metamodelica::nil();
    for mut nodeIdx in 1..=metamodelica::arrayLength(iTaskGraph.clone()) {
        nodeSuccessors = metamodelica::arrayGet(iTaskGraph.clone(), nodeIdx)?;
        if (nodeSuccessors).is_empty() {
            tmpLeafNodes = metamodelica::cons(nodeIdx, tmpLeafNodes);
        }
    }
    oLeafNodes = tmpLeafNodes;
    Ok(oLeafNodes)
}

pub(crate) fn getLevelNodes(mut iTaskGraph: TaskGraph) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut oLevelNodes: metamodelica::List<metamodelica::List<i32>>;
    let mut refCounter: metamodelica::Array<i32>;
    let mut roots: metamodelica::List<i32>;
    refCounter = createRefCounter(iTaskGraph.clone())?;
    roots = getNodesWithRefCountZero(refCounter.clone())?;
    oLevelNodes = getLevelNodes0(iTaskGraph.clone(), refCounter.clone(), roots, metamodelica::nil())?;
    Ok(oLevelNodes)
}

fn getLevelNodes0(
    mut iTaskGraph: TaskGraph,
    mut iRefCounter: metamodelica::Array<i32>,
    mut iNodesWithRefZero: metamodelica::List<i32>,
    mut iLevelNodes: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    '__tco: loop {
        let mut tmpLevelNodes: metamodelica::List<metamodelica::List<i32>>;
        let mut zeroRefNodes: metamodelica::List<i32>;
        ::match_deref::match_deref! { match &(iNodesWithRefZero) {
            Deref @ metamodelica::ListNode::Nil => return Ok(iLevelNodes.reverse()),
            __esc_zeroRefNodes => {
                zeroRefNodes = (*__esc_zeroRefNodes).clone();
                tmpLevelNodes = metamodelica::cons(zeroRefNodes.clone(), iLevelNodes);
                zeroRefNodes = List::fold2(metamodelica::AsArg::as_arg(&zeroRefNodes), &getLevelNodes1, iTaskGraph.clone(), iRefCounter.clone(), metamodelica::nil())?;
                { (iTaskGraph, iRefCounter, iNodesWithRefZero, iLevelNodes) = (iTaskGraph.clone(), iRefCounter.clone(), zeroRefNodes.clone(), tmpLevelNodes); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getLevelNodes1(
    mut iNodeIdx: i32,
    mut iTaskGraph: TaskGraph,
    mut iRefCounter: metamodelica::Array<i32>,
    mut iNodesWithRefZero: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oNodesWithRefZero: metamodelica::List<i32>;
    let mut childNodes: metamodelica::List<i32>;
    let mut tmpNodesWithRefZero: metamodelica::List<i32>;
    childNodes = metamodelica::arrayGet(iTaskGraph.clone(), iNodeIdx)?;
    tmpNodesWithRefZero = List::fold1(
        &childNodes,
        &fnptr!(getLevelNodes2, i32, metamodelica::Array<i32>, metamodelica::List<i32>),
        iRefCounter.clone(),
        metamodelica::nil(),
    )?;
    oNodesWithRefZero = listAppend(tmpNodesWithRefZero, iNodesWithRefZero);
    Ok(oNodesWithRefZero)
}

fn getLevelNodes2(
    mut iNodeIdx: i32,
    mut iRefCounter: metamodelica::Array<i32>,
    mut iNodesWithRefZero: metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut oNodesWithRefZero: metamodelica::List<i32>;
    let mut tmpNodesWithRefZero: metamodelica::List<i32>;
    let mut refCounter: i32 = 0;
    oNodesWithRefZero = 'mc: {
        let __mc_input = iNodesWithRefZero.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tmpNodesWithRefZero => {
                    let mut tmpNodesWithRefZero = (*tmpNodesWithRefZero).clone();
                    let mut refCounter: i32 = refCounter.clone();
                    refCounter = metamodelica::arrayGet(iRefCounter.clone(), iNodeIdx)? - 1;
                    metamodelica::arrayUpdate(iRefCounter.clone(), iNodeIdx, refCounter)?;
                    let true = (intEq(refCounter, 0)) else { return Err("pattern mismatch") };
                    tmpNodesWithRefZero = metamodelica::cons(iNodeIdx, tmpNodesWithRefZero.clone());
                    Ok((tmpNodesWithRefZero.clone(), refCounter.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            refCounter = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iNodesWithRefZero.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oNodesWithRefZero
}

fn createRefCounter(mut iTaskGraph: TaskGraph) -> Result<metamodelica::Array<i32>> {
    let mut oRefCounter: metamodelica::Array<i32>;
    let mut tmpRefCounter: metamodelica::Array<i32>;
    tmpRefCounter = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    tmpRefCounter = Array::fold(iTaskGraph.clone(), &createRefCounter0, tmpRefCounter.clone())?;
    oRefCounter = tmpRefCounter.clone();
    Ok(oRefCounter)
}

fn createRefCounter0(
    mut iChildNodes: metamodelica::List<i32>,
    mut iRefCounter: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    '__tco: loop {
        let mut tmpRefCounter: metamodelica::Array<i32>;
        let mut counter: i32;
        let mut head: i32;
        let mut tail: metamodelica::List<i32>;
        ::match_deref::match_deref! { match &(iChildNodes) {
            Deref @ metamodelica::ListNode::Nil => return Ok(iRefCounter.clone()),
            Deref @ metamodelica::ListNode::Cons { head: __esc_head, tail: __esc_tail } => {
                head = (*__esc_head).clone();
                tail = (*__esc_tail).clone();
                counter = metamodelica::arrayGet(iRefCounter.clone(), head.clone())? + 1;
                tmpRefCounter = metamodelica::arrayUpdate(iRefCounter.clone(), head.clone(), counter)?;
                { (iChildNodes, iRefCounter) = (tail.clone(), tmpRefCounter.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getNodesWithRefCountZero(mut iRefCounter: metamodelica::Array<i32>) -> Result<metamodelica::List<i32>> {
    let mut oZeroIdc: metamodelica::List<i32>;
    (oZeroIdc, _) = Array::fold(
        iRefCounter.clone(),
        &move |__a0: i32, __a1: (metamodelica::List<i32>, i32)| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getNodesWithRefCountZero0(__a0, &__a1))
        },
        (metamodelica::nil(), 1),
    )?;
    Ok(oZeroIdc)
}

fn getNodesWithRefCountZero0(
    mut iRefCount: i32,
    mut iZeroIdc: &(metamodelica::List<i32>, i32),
) -> (metamodelica::List<i32>, i32) {
    let mut oZeroIdc: (metamodelica::List<i32>, i32);
    let mut resultList: metamodelica::List<i32>;
    let mut currentNodeIdx: i32;
    oZeroIdc = (::match_deref::match_deref! { match &((iRefCount, iZeroIdc)) {
        (0, (__esc_resultList, __esc_currentNodeIdx)) => {
            resultList = (*__esc_resultList).clone();
            currentNodeIdx = (*__esc_currentNodeIdx).clone();
            resultList = metamodelica::cons(currentNodeIdx.clone(), resultList.clone());
            (resultList.clone(), currentNodeIdx.clone() + 1)
        },
        (_, (__esc_resultList, __esc_currentNodeIdx)) => {
            resultList = (*__esc_resultList).clone();
            currentNodeIdx = (*__esc_currentNodeIdx).clone();
            (resultList.clone(), currentNodeIdx.clone() + 1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oZeroIdc
}

//----------------------------------
//  Functions to get the event-graph
//----------------------------------
pub(crate) fn getZeroFuncsSystem(
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
    mut iBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iNumberOfSccs: i32,
    mut iZeroCrossingEquationIdc: &metamodelica::List<i32>,
    mut iSimCodeEqCompMapping: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut nodeList: metamodelica::List<i32>;
    let mut newNodeList: metamodelica::List<i32>;
    let mut predecessors: metamodelica::List<i32>;
    let mut successors: metamodelica::List<i32>;
    let mut successorsTmp: metamodelica::List<i32>;
    let mut predecessorsTmp: metamodelica::List<i32>;
    let mut zeroFuncNodeMarks: metamodelica::Array<i32>;
    let mut sccNodeMapping: metamodelica::Array<i32>;
    let mut handledNodes: metamodelica::Array<bool>;
    let mut whenNodeMarks: metamodelica::Array<bool>;
    let mut iTaskGraphTCopy: TaskGraph;
    let mut iTaskGraphCopy: TaskGraph;
    let mut zeroFuncTaskGraph: TaskGraph;
    let mut zeroFuncTaskGraphMeta: TaskGraphMeta;
    let mut whenNodes: metamodelica::List<i32>;
    let mut zeroFuncInComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32;
    let mut nodeIdx: i32 = 0;
    let mut successor: i32 = 0;
    let mut predecessor: i32 = 0;
    let mut zeroFuncNodeMark: i32;
    let mut successorMark: i32;
    let mut zeroFuncNodeCount: i32;
    let mut zeroFuncNodeIdx: i32;
    let mut nodeToZeroFuncNodeMapping: metamodelica::Array<i32>;
    let mut stop: bool;
    let TaskGraphMeta {
        inComps: __pa0,
        eqCompMapping: __pa1,
        ..
    } = &iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    eqCompMapping = metamodelica::Own::own(__pa1);
    zeroFuncNodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    handledNodes = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), false);
    nodeToZeroFuncNodeMapping = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), -1);
    whenNodes = getEventNodes(iBackendDAE, eqCompMapping.clone())?;
    whenNodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), false);
    sccNodeMapping = getSccNodeMapping(iNumberOfSccs, iTaskGraphMeta.clone())?;
    iTaskGraphCopy = metamodelica::arrayFromVec(iTaskGraph.clone().borrow().clone());
    iTaskGraphTCopy =
        AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    for mut eqIdx in &**iZeroCrossingEquationIdc {
        let mut eqIdx = eqIdx.clone();
        compIdx = metamodelica::arrayGet(iSimCodeEqCompMapping.clone(), eqIdx)?;
        nodeIdx = metamodelica::arrayGet(sccNodeMapping.clone(), compIdx)?;
        zeroFuncNodeMarks = metamodelica::arrayUpdate(zeroFuncNodeMarks.clone(), nodeIdx, 1)?;
    }
    for mut nodeIdx in &*whenNodes {
        let mut nodeIdx = nodeIdx.clone();
        whenNodeMarks = metamodelica::arrayUpdate(whenNodeMarks.clone(), nodeIdx, true)?;
    }
    nodeList = getRootNodes(iTaskGraphTCopy.clone())?;
    zeroFuncNodeCount = 0;
    zeroFuncNodeIdx = 1;
    while boolNot((nodeList).is_empty()) {
        newNodeList = metamodelica::nil();
        for mut nodeIdx in &*nodeList {
            let mut nodeIdx = nodeIdx.clone();
            if boolNot(metamodelica::arrayGet(handledNodes.clone(), nodeIdx)?) {
                handledNodes = metamodelica::arrayUpdate(handledNodes.clone(), nodeIdx, true)?;
                predecessors = metamodelica::arrayGet(iTaskGraphTCopy.clone(), nodeIdx)?;
                successors = metamodelica::arrayGet(iTaskGraphCopy.clone(), nodeIdx)?;
                zeroFuncNodeMark = -1;
                if metamodelica::arrayGet(whenNodeMarks.clone(), nodeIdx)? {
                    for mut predecessor in &*predecessors {
                        let mut predecessor = predecessor.clone();
                        successorsTmp = metamodelica::arrayGet(iTaskGraphCopy.clone(), predecessor)?;
                        metamodelica::arrayUpdate(
                            iTaskGraphCopy.clone(),
                            predecessor,
                            listAppend(successorsTmp, successors.clone()),
                        )?;
                    }
                    for mut successor in &*successors {
                        let mut successor = successor.clone();
                        predecessorsTmp = metamodelica::arrayGet(iTaskGraphTCopy.clone(), successor)?;
                        metamodelica::arrayUpdate(
                            iTaskGraphTCopy.clone(),
                            successor,
                            listAppend(predecessorsTmp, predecessors.clone()),
                        )?;
                    }
                } else {
                    if intGt(metamodelica::arrayGet(zeroFuncNodeMarks.clone(), nodeIdx)?, 0) {
                        zeroFuncNodeMark = zeroFuncNodeIdx;
                    } else {
                        stop = false;
                        while boolAnd(boolNot(stop), boolNot((successors).is_empty())) {
                            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(successors) {
                                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            successor = metamodelica::Own::own(__pa2);
                            successors = metamodelica::Own::own(__pa3);
                            successorMark = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), successor)?;
                            if intGt(successorMark, 0) {
                                zeroFuncNodeMark = zeroFuncNodeIdx;
                                stop = true;
                            }
                        }
                    }
                    if intGt(zeroFuncNodeMark, 0) {
                        zeroFuncNodeCount = zeroFuncNodeCount + 1;
                        nodeToZeroFuncNodeMapping =
                            metamodelica::arrayUpdate(nodeToZeroFuncNodeMapping.clone(), nodeIdx, zeroFuncNodeCount)?;
                        zeroFuncNodeIdx = zeroFuncNodeIdx + 1;
                    }
                }
                zeroFuncNodeMarks = metamodelica::arrayUpdate(zeroFuncNodeMarks.clone(), nodeIdx, zeroFuncNodeMark)?;
                newNodeList = List::append_reverse(&predecessors, newNodeList);
            }
        }
        nodeList = newNodeList.reverse();
    }
    zeroFuncTaskGraph = arrayCreate(zeroFuncNodeCount, metamodelica::nil());
    zeroFuncInComps = arrayCreate(zeroFuncNodeCount, metamodelica::nil());
    nodeIdx = metamodelica::arrayLength(zeroFuncNodeMarks.clone());
    while intGt(nodeIdx, 0) {
        zeroFuncNodeIdx = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), nodeIdx)?;
        if intGt(zeroFuncNodeIdx, 0) {
            successors = metamodelica::arrayGet(iTaskGraphCopy.clone(), nodeIdx)?;
            zeroFuncInComps = metamodelica::arrayUpdate(
                zeroFuncInComps.clone(),
                zeroFuncNodeIdx,
                metamodelica::arrayGet(inComps.clone(), nodeIdx)?,
            )?;
            newNodeList = metamodelica::nil();
            while boolNot((successors).is_empty()) {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(successors) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                successor = metamodelica::Own::own(__pa4);
                successors = metamodelica::Own::own(__pa5);
                successor = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), successor)?;
                if intGt(successor, 0) {
                    newNodeList = metamodelica::cons(successor, newNodeList);
                }
            }
            newNodeList = List::sort(
                newNodeList,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            newNodeList = List::sortedUnique(newNodeList, &fnptr!(intEq, i32, i32))?;
            zeroFuncTaskGraph = metamodelica::arrayUpdate(zeroFuncTaskGraph.clone(), zeroFuncNodeIdx, newNodeList)?;
        }
        nodeIdx = nodeIdx - 1;
    }
    zeroFuncTaskGraphMeta = copyTaskGraphMeta(iTaskGraphMeta);
    zeroFuncTaskGraphMeta = setInCompsInMeta(zeroFuncInComps.clone(), zeroFuncTaskGraphMeta);
    (oTaskGraph, oTaskGraphMeta) = reverseTaskGraphIndices(zeroFuncTaskGraph.clone(), zeroFuncTaskGraphMeta)?;
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn reverseTaskGraphIndices(
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut nTasks: i32;
    let mut idxMap: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    nTasks = metamodelica::arrayLength(iTaskGraph.clone());
    idxMap = arrayCreate(nTasks, -1);
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    exeCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeMark = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    for mut i in 1..=nTasks {
        idxMap = metamodelica::arrayUpdate(idxMap.clone(), i, nTasks - i + 1)?;
    }
    (oTaskGraph, _) = Array::mapNoCopy_1(
        iTaskGraph.clone(),
        &move |__a0: (metamodelica::List<i32>, metamodelica::Array<i32>)| mapIntegers(&__a0),
        idxMap.clone(),
    )?;
    oTaskGraph = Array::reverse(oTaskGraph.clone())?;
    inComps = Array::reverse(inComps.clone())?;
    oTaskGraphMeta = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn mapIntegers(
    mut iTpl: &(metamodelica::List<i32>, metamodelica::Array<i32>),
) -> Result<(metamodelica::List<i32>, metamodelica::Array<i32>)> {
    let mut oTpl: (metamodelica::List<i32>, metamodelica::Array<i32>);
    let mut map: metamodelica::Array<i32>;
    let mut iLst: metamodelica::List<i32>;
    let mut oLst: metamodelica::List<i32> = metamodelica::nil();
    (iLst, map) = iTpl.clone();
    for mut i in &*iLst {
        oLst = metamodelica::cons(metamodelica::arrayGet(map.clone(), i.clone())?, oLst);
    }
    oLst = oLst.reverse();
    oTpl = (oLst, map.clone());
    Ok(oTpl)
}

fn getEventSystem(
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
    mut iSyst: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iZeroCrossings: metamodelica::List<BackendDAE::ZeroCrossing>,
    mut iSimCodeEqCompMapping: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut discreteNodes: metamodelica::List<i32>;
    let mut cutNodes: metamodelica::List<i32>;
    let mut cutNodeChildren: metamodelica::List<i32>;
    let mut zeroCrossingNodes: metamodelica::List<i32>;
    let mut sccsContainingTime: metamodelica::List<i32>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut graphTmp: TaskGraph;
    let TaskGraphMeta {
        varCompMapping: __pa0,
        eqCompMapping: __pa1,
        inComps: __pa2,
        ..
    } = &iTaskGraphMeta;
    varCompMapping = metamodelica::Own::own(__pa0);
    eqCompMapping = metamodelica::Own::own(__pa1);
    inComps = metamodelica::Own::own(__pa2);
    let __arc5 = &(*iSyst);
    let BackendDAE::DAE {
        eqs: __pa3,
        shared: __pa4,
    } = &**__arc5;
    systs = metamodelica::Own::own(__pa3);
    shared = metamodelica::Own::own(__pa4);
    discreteNodes = getDiscreteNodes(iSyst, eqCompMapping.clone())?;
    zeroCrossingNodes = List::flatten(List::map1(
        iZeroCrossings,
        &move |__a0: BackendDAE::ZeroCrossing, __a1: metamodelica::Array<i32>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getComponentsOfZeroCrossing(&__a0, __a1))
        },
        iSimCodeEqCompMapping.clone(),
    )?)?;
    sccsContainingTime = metamodelica::nil();
    discreteNodes = List::flatten(list![discreteNodes, sccsContainingTime, zeroCrossingNodes])?;
    graphTmp = iTaskGraph.clone();
    (graphTmp, cutNodes) = cutTaskGraph(graphTmp.clone(), discreteNodes, metamodelica::nil())?;
    cutNodeChildren = List::flatten(List::map1(cutNodes.clone(), &Array::getIndexFirst, iTaskGraph.clone())?)?;
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren, cutNodes.clone(), &fnptr!(intEq, i32, i32))?;
    oTaskGraphMeta = cutSystemData(iTaskGraphMeta, cutNodes, &cutNodeChildren)?;
    oTaskGraph = graphTmp.clone();
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn getComponentsOfZeroCrossing(
    mut iZeroCrossing: &BackendDAE::ZeroCrossing,
    mut iSimCodeEqCompMapping: metamodelica::Array<i32>,
) -> metamodelica::List<i32> {
    let mut oCompIdc: metamodelica::List<i32>;
    let mut occurEquLst: metamodelica::List<i32>;
    let mut tmpCompIdc: metamodelica::List<i32> = metamodelica::nil();
    oCompIdc = 'mc: {
        let __mc_input = iZeroCrossing.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let BackendDAE::ZeroCrossing {
                occurEquLst: mut occurEquLst,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut occurEquLst = occurEquLst.clone();
            let mut tmpCompIdc: metamodelica::List<i32> = tmpCompIdc.clone();
            occurEquLst = List::filter1OnTrue(
                occurEquLst.clone(),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                0,
            )?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: simEqs: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(occurEquLst.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            tmpCompIdc = List::map1(
                occurEquLst.clone(),
                &Array::getIndexFirst,
                iSimCodeEqCompMapping.clone(),
            )?;
            tmpCompIdc = List::filter1OnTrue(
                tmpCompIdc.clone(),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                0,
            )?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: components: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(tmpCompIdc.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok((tmpCompIdc.clone(), tmpCompIdc.clone()))
        })() {
            tmpCompIdc = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCompIdc
}

fn getComponentsIncludingTime(
    mut iSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut iOffsetResList: &(i32, metamodelica::List<i32>),
) -> Result<(i32, metamodelica::List<i32>)> {
    let mut oOffsetResList: (i32, metamodelica::List<i32>);
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut offset: i32;
    let mut resultList: metamodelica::List<i32>;
    let __arc1 = &(*iSystem);
    let BackendDAE::EQSYSTEM { orderedEqs: __pa0, .. } = &**__arc1;
    orderedEqs = metamodelica::Own::own(__pa0);
    (offset, resultList) = iOffsetResList.clone();
    (offset, resultList, _, _) = BackendEquation::traverseEquationArray(
        orderedEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: (i32, metamodelica::List<i32>, metamodelica::Array<(i32, i32, i32)>, i32)|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getComponentsIncludingTime0(__a0, &__a1))
        },
        (offset, resultList, iEqCompMapping.clone(), 1),
    )?;
    oOffsetResList = (offset, resultList);
    Ok(oOffsetResList)
}

fn getComponentsIncludingTime0(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut iOffsetResList: &(i32, metamodelica::List<i32>, metamodelica::Array<(i32, i32, i32)>, i32),
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    (i32, metamodelica::List<i32>, metamodelica::Array<(i32, i32, i32)>, i32),
) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut oOffsetResList: (i32, metamodelica::List<i32>, metamodelica::Array<(i32, i32, i32)>, i32);
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut offset: i32;
    let mut eqIdx: i32;
    let mut sccIdx: i32 = 0;
    let mut resultList: metamodelica::List<i32>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    (outEq, oOffsetResList) = 'mc: {
        let __mc_input = (inEq, iOffsetResList);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    let mut resultList = (*resultList).clone();
                    let mut sccIdx: i32 = sccIdx.clone();
                    (sccIdx, _, _) = metamodelica::arrayGet(eqCompMapping.clone(), eqIdx.clone() + offset.clone())?;
                    let true = (BackendDAEUtil::traverseBackendDAEExpsOptEqn(Some(eq.clone()), (std::sync::Arc::new(getComponentsIncludingTime1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false)?) else { return Err("pattern mismatch") };
                    resultList = metamodelica::cons(sccIdx, resultList.clone());
                    Ok(((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)), sccIdx.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            sccIdx = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    Ok((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, oOffsetResList)
}

fn getComponentsIncludingTime1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inB: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut res: bool;
    (e, res) = (::match_deref::match_deref! { match &((inExp.clone(), inB)) {
        (__esc_e, false) => {
            e = (*__esc_e).clone();
            res = Expression::traverseCrefsFromExp(e.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(getComponentsIncludingTime2(&__a0, __a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, bool) -> Result<bool> + 'static>), false)?;
            (e.clone(), res)
        },
        _ => (inExp, inB),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, res))
}

fn getComponentsIncludingTime2(mut iRef: &metamodelica::Ref<DAE::ComponentRef>, mut iIncludingTime: bool) -> bool {
    let mut oIncludingTime: bool;
    oIncludingTime = (::match_deref::match_deref! { match iRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. } => true,
        _ => false || iIncludingTime,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oIncludingTime
}

fn getDiscreteNodes(
    mut systIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut eventNodes: metamodelica::List<i32>;
    let mut eqLst: metamodelica::List<i32>;
    let mut systemsIn: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*systIn);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    systemsIn = metamodelica::Own::own(__pa0);
    (eqLst, _) = List::fold(
        &systemsIn,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: (metamodelica::List<i32>, i32)|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(getDiscreteNodesEqs(&__a0, &__a1)) },
        (metamodelica::nil(), 0),
    )?;
    eventNodes = getArrayTuple31(eqLst, eqCompMapping.clone())?;
    Ok(eventNodes)
}

fn getDiscreteNodesEqs(
    mut systIn: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut eventInfoIn: &(metamodelica::List<i32>, i32),
) -> (metamodelica::List<i32>, i32) {
    let mut eventInfoOut: (metamodelica::List<i32>, i32);
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut orderedVars: BackendDAE::Variables;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eventEqs: metamodelica::List<i32>;
    let mut eventEqsIn: metamodelica::List<i32>;
    let mut offset: i32;
    let __arc3 = &(*systIn);
    let BackendDAE::EQSYSTEM {
        orderedEqs: __pa0,
        orderedVars: __pa1,
        matching: __pa2,
        ..
    } = &**__arc3;
    orderedEqs = metamodelica::Own::own(__pa0);
    orderedVars = metamodelica::Own::own(__pa1);
    matching = metamodelica::Own::own(__pa2);
    comps = BackendDAEUtil::getCompsOfMatching(&matching);
    (eventEqsIn, offset) = eventInfoIn.clone();
    eventEqs = getDiscreteNodesEqs1(&comps, offset, &orderedVars, &(metamodelica::nil()));
    offset = offset + ExpandableArray::getNumberOfElements(orderedEqs);
    eventInfoOut = (listAppend(eventEqs, eventEqsIn), offset);
    eventInfoOut
}

fn getDiscreteNodesEqs1(
    mut comps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut offset: i32,
    mut iOrderedVars: &BackendDAE::Variables,
    mut discreteEqsIn: &metamodelica::List<i32>,
) -> metamodelica::List<i32> {
    let mut discreteEqsOut: metamodelica::List<i32>;
    discreteEqsOut = 'mc: {
        let __mc_input = &**comps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut eqn: i32;
                    let mut eventEqs: metamodelica::List<i32>;
                    let (true, __pa0) = (solvesDiscreteValue(metamodelica::AsArg::as_arg(&head), iOrderedVars.clone())) else { return Err("pattern mismatch") };
                    eqn = metamodelica::Own::own(__pa0);
                    eqn = eqn + offset;
                    eventEqs = getDiscreteNodesEqs1(metamodelica::AsArg::as_arg(&rest), offset, iOrderedVars, &(metamodelica::cons(eqn, discreteEqsIn.clone())));
                    Ok(eventEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut eventEqs: metamodelica::List<i32>;
                    eventEqs = getDiscreteNodesEqs1(metamodelica::AsArg::as_arg(&rest), offset, iOrderedVars, discreteEqsIn);
                    Ok(eventEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(discreteEqsIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    discreteEqsOut
}

fn solvesDiscreteValue(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut iOrderedVars: BackendDAE::Variables,
) -> (bool, i32) {
    let mut oSolvesDiscreteValue: bool;
    let mut oFirstEqIdx: i32;
    (oSolvesDiscreteValue, oFirstEqIdx) = 'mc: {
        let __mc_input = &**inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var, eqn } => {
                    let mut backendVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut solvesDiscreteValue: bool;
                    backendVar = BackendVariable::getVarAt(&iOrderedVars, var.clone())?;
                    solvesDiscreteValue = BackendVariable::isVarDiscrete(&backendVar);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars, eqns, .. } => {
                    let mut eqn: i32;
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    eqn = (eqns).head().cloned()?;
                    Ok((solvesDiscreteValue, eqn))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars, eqn } => {
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars, eqn } => {
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars, eqn } => {
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars, eqn } => {
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars, eqn } => {
                    let mut backendVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(&backendVars);
                    Ok((solvesDiscreteValue, eqn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, -1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oSolvesDiscreteValue, oFirstEqIdx)
}

//------------------------------------------
//Methods to write blt-structure as xml-file
//------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct GraphDumpOptions {
    pub visualizeCriticalPath: bool,
    pub visualizeTaskStartAndFinishTime: bool,
    pub visualizeTaskCalcTime: bool,
    pub visualizeCommTime: bool,
}

impl metamodelica::gc::MMTrace for GraphDumpOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeCriticalPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeTaskStartAndFinishTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeTaskCalcTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeCommTime, __mmv)?;
        Ok(())
    }
}
pub type GRAPHDUMPOPTIONS = GraphDumpOptions;

pub fn dumpTaskGraph(mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>, mut fileName: &ArcStr) -> Result<()> {
    let mut name: ArcStr;
    let mut taskGraph: TaskGraph;
    let mut taskGraphData: TaskGraphMeta;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    (taskGraph, taskGraphData) = createTaskGraph(dae, false)?;
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("TaskGraph_"));
        __mm_s.push_str(&*fileName);
        __mm_s.push_str(&*literal!(".graphml"));
        ArcStr::from(__mm_s)
    };
    schedulerInfo = arrayCreate(
        metamodelica::arrayLength(taskGraph.clone()),
        (-1, -1, metamodelica::OrderedFloat(-1.0_f64)),
    );
    sccSimEqMapping = arrayCreate(metamodelica::arrayLength(taskGraph.clone()), list![-1]);
    dumpAsGraphMLSccLevel(
        taskGraph.clone(),
        taskGraphData,
        name,
        literal!(""),
        metamodelica::nil(),
        metamodelica::nil(),
        sccSimEqMapping.clone(),
        schedulerInfo.clone(),
        GraphDumpOptions {
            visualizeCriticalPath: false,
            visualizeTaskStartAndFinishTime: false,
            visualizeTaskCalcTime: true,
            visualizeCommTime: true,
        },
    )?;
    Ok(())
}

pub(crate) fn dumpAsGraphMLSccLevel(
    mut iGraph: TaskGraph,
    mut iGraphData: TaskGraphMeta,
    mut iFileName: ArcStr,
    mut iCriticalPathInfo: ArcStr,
    mut iCriticalPath: metamodelica::List<(i32, i32)>,
    mut iCriticalPathWoC: metamodelica::List<(i32, i32)>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iGraphDumpOptions: GraphDumpOptions,
) -> Result<()> {
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = convertToGraphMLSccLevel(
        iGraph.clone(),
        iGraphData,
        iCriticalPathInfo,
        iCriticalPath,
        iCriticalPathWoC,
        iSccSimEqMapping.clone(),
        iSchedulerInfo.clone(),
        iGraphDumpOptions,
    )?;
    GraphML::dumpGraph(graphInfo, iFileName)?;
    Ok(())
}

pub(crate) fn convertToGraphMLSccLevel(
    mut iGraph: TaskGraph,
    mut iGraphData: TaskGraphMeta,
    mut iCriticalPathInfo: ArcStr,
    mut iCriticalPath: metamodelica::List<(i32, i32)>,
    mut iCriticalPathWoC: metamodelica::List<(i32, i32)>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iGraphDumpOptions: GraphDumpOptions,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph(literal!("TaskGraph"), true, graphInfo)?;
    graphInfo = metamodelica::Own::own(__pa0);
    graphIdx = metamodelica::Own::own(__pa1);
    annotationInfo = arrayCreate(
        metamodelica::arrayLength(iGraph.clone()),
        literal!("uncomment in HpcOmTaskGraph and +showAnnotations"),
    );
    oGraphInfo = convertToGraphMLSccLevelSubgraph(
        iGraph.clone(),
        iGraphData,
        iCriticalPathInfo,
        iCriticalPath,
        iCriticalPathWoC,
        iSccSimEqMapping.clone(),
        iSchedulerInfo.clone(),
        annotationInfo.clone(),
        graphIdx,
        iGraphDumpOptions,
        graphInfo,
    )?;
    Ok(oGraphInfo)
}

pub(crate) fn convertToGraphMLSccLevelSubgraph(
    mut iGraph: TaskGraph,
    mut iGraphData: TaskGraphMeta,
    mut iCriticalPathInfo: ArcStr,
    mut iCriticalPath: metamodelica::List<(i32, i32)>,
    mut iCriticalPathWoC: metamodelica::List<(i32, i32)>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iAnnotationInfo: metamodelica::Array<ArcStr>,
    mut iGraphIdx: i32,
    mut iGraphDumpOptions: GraphDumpOptions,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nameAttIdx: i32;
    let mut calcTimeAttIdx: i32;
    let mut opCountAttIdx: i32;
    let mut yCoordAttIdx: i32;
    let mut taskIdAttIdx: i32;
    let mut commCostAttIdx: i32;
    let mut commVarsAttIdx: i32;
    let mut commVarsIntAttIdx: i32;
    let mut commVarsFloatAttIdx: i32;
    let mut commVarsBoolAttIdx: i32;
    let mut critPathAttIdx: i32;
    let mut simCodeEqAttIdx: i32;
    let mut threadIdAttIdx: i32;
    let mut taskNumberAttIdx: i32;
    let mut annotAttIdx: i32;
    let mut compsIdAttIdx: i32;
    let mut partOfEventAttIdx: i32;
    let mut partOfOdeAttIdx: i32;
    let mut removedCompAttIdx: i32;
    let mut nodeIdc: metamodelica::List<i32>;
    oGraphInfo = (match iGraphInfo.clone() {
        _ => {
            let (__pa0, (_, __pa1)) = GraphML::addAttribute(
                literal!(""),
                literal!("Name"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                iGraphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa0);
            nameAttIdx = metamodelica::Own::own(__pa1);
            let (__pa2, (_, __pa3)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("Operations"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa2);
            opCountAttIdx = metamodelica::Own::own(__pa3);
            let (__pa4, (_, __pa5)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CalcTime"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_DOUBLE,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa4);
            calcTimeAttIdx = metamodelica::Own::own(__pa5);
            let (__pa6, (_, __pa7)) = GraphML::addAttribute(
                literal!(""),
                literal!("TaskID"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa6);
            taskIdAttIdx = metamodelica::Own::own(__pa7);
            let (__pa8, (_, __pa9)) = GraphML::addAttribute(
                literal!(""),
                literal!("Components"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa8);
            compsIdAttIdx = metamodelica::Own::own(__pa9);
            let (__pa10, (_, __pa11)) = GraphML::addAttribute(
                literal!("17"),
                literal!("yCoord"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa10);
            yCoordAttIdx = metamodelica::Own::own(__pa11);
            let (__pa12, (_, __pa13)) = GraphML::addAttribute(
                literal!(""),
                literal!("SimCodeEqs"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa12);
            simCodeEqAttIdx = metamodelica::Own::own(__pa13);
            let (__pa14, (_, __pa15)) = GraphML::addAttribute(
                literal!(""),
                literal!("ThreadId"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa14);
            threadIdAttIdx = metamodelica::Own::own(__pa15);
            let (__pa16, (_, __pa17)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("TaskNumber"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa16);
            taskNumberAttIdx = metamodelica::Own::own(__pa17);
            let (__pa18, (_, __pa19)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CommCost"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_DOUBLE,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa18);
            commCostAttIdx = metamodelica::Own::own(__pa19);
            let (__pa20, (_, __pa21)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CommVars"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa20);
            commVarsAttIdx = metamodelica::Own::own(__pa21);
            let (__pa22, (_, __pa23)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CommVarsInt"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa22);
            commVarsIntAttIdx = metamodelica::Own::own(__pa23);
            let (__pa24, (_, __pa25)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CommVarsFloat"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa24);
            commVarsFloatAttIdx = metamodelica::Own::own(__pa25);
            let (__pa26, (_, __pa27)) = GraphML::addAttribute(
                literal!("-1"),
                literal!("CommVarsBool"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa26);
            commVarsBoolAttIdx = metamodelica::Own::own(__pa27);
            let (__pa28, (_, __pa29)) = GraphML::addAttribute(
                literal!("annotation"),
                literal!("Annotations"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa28);
            annotAttIdx = metamodelica::Own::own(__pa29);
            let (__pa30, (_, __pa31)) = GraphML::addAttribute(
                literal!(""),
                literal!("CriticalPath"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_GRAPH,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa30);
            critPathAttIdx = metamodelica::Own::own(__pa31);
            let (__pa32, (_, __pa33)) = GraphML::addAttribute(
                literal!("false"),
                literal!("isPartOfZeroFuncSystem"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa32);
            partOfEventAttIdx = metamodelica::Own::own(__pa33);
            let (__pa34, (_, __pa35)) = GraphML::addAttribute(
                literal!("false"),
                literal!("IsPartOfOdeSystem"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa34);
            partOfOdeAttIdx = metamodelica::Own::own(__pa35);
            let (__pa36, (_, __pa37)) = GraphML::addAttribute(
                literal!("false"),
                literal!("IsRemovedComponent"),
                openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN,
                openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE,
                graphInfo,
            )?;
            graphInfo = metamodelica::Own::own(__pa36);
            removedCompAttIdx = metamodelica::Own::own(__pa37);
            graphInfo = GraphML::addGraphAttributeValue((critPathAttIdx, iCriticalPathInfo), iGraphIdx, graphInfo)?;
            nodeIdc = List::intRange(metamodelica::arrayLength(iGraph.clone()));
            (graphInfo, _) = List::fold(
                &nodeIdc,
                &({
                    let __pe_b1 = (iGraph.clone(), iGraphData);
                    let __pe_b2 = (
                        nameAttIdx,
                        opCountAttIdx,
                        calcTimeAttIdx,
                        taskIdAttIdx,
                        compsIdAttIdx,
                        yCoordAttIdx,
                        commCostAttIdx,
                        commVarsAttIdx,
                        commVarsIntAttIdx,
                        commVarsFloatAttIdx,
                        commVarsBoolAttIdx,
                        simCodeEqAttIdx,
                        threadIdAttIdx,
                        taskNumberAttIdx,
                        annotAttIdx,
                        partOfEventAttIdx,
                        partOfOdeAttIdx,
                        removedCompAttIdx,
                    );
                    let __pe_b3 = iSccSimEqMapping.clone();
                    let __pe_b4 = (
                        iCriticalPath,
                        iCriticalPathWoC,
                        iSchedulerInfo.clone(),
                        iAnnotationInfo.clone(),
                    );
                    let __pe_b5 = iGraphDumpOptions;
                    move |__pe_a0, __pe_a6| {
                        addNodeToGraphML(
                            __pe_a0,
                            &__pe_b1,
                            __pe_b2.clone(),
                            __pe_b3.clone(),
                            &__pe_b4,
                            __pe_b5.clone(),
                            &__pe_a6,
                        )
                    }
                }),
                (graphInfo, iGraphIdx),
            )?;
            graphInfo
        }
    });
    Ok(oGraphInfo)
}

fn addNodeToGraphML(
    mut nodeIdx: i32,
    mut tGraphDataTuple: &(metamodelica::Array<metamodelica::List<i32>>, TaskGraphMeta),
    mut attIdc: (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ),
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfoCritPath: &(
        metamodelica::List<(i32, i32)>,
        metamodelica::List<(i32, i32)>,
        metamodelica::Array<(i32, i32, metamodelica::Real)>,
        metamodelica::Array<ArcStr>,
    ),
    mut iGraphDumpOptions: GraphDumpOptions,
    mut iGraph: &(GraphML::GraphInfo, i32),
) -> Result<(GraphML::GraphInfo, i32)> {
    let mut oGraph: (GraphML::GraphInfo, i32);
    let mut tGraphIn: TaskGraph;
    let mut tGraphDataIn: TaskGraphMeta;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut graphIdx: i32;
    let mut opCount: i32;
    let mut nameAttIdx: i32;
    let mut calcTimeAttIdx: i32;
    let mut opCountAttIdx: i32;
    let mut taskIdAttIdx: i32;
    let mut compsIdAttIdx: i32;
    let mut yCoordAttIdx: i32;
    let mut commCostAttIdx: i32;
    let mut commVarsAttIdx: i32;
    let mut commVarsAttIntIdx: i32;
    let mut commVarsAttFloatIdx: i32;
    let mut commVarsAttBoolIdx: i32;
    let mut yCoord: i32;
    let mut simCodeEqAttIdx: i32;
    let mut threadIdAttIdx: i32;
    let mut taskNumberAttIdx: i32;
    let mut annotationAttIdx: i32;
    let mut partOfEventAttIdx: i32;
    let mut partOfOdeAttIdx: i32;
    let mut removedCompAttIdx: i32;
    let mut calcTime: metamodelica::Real;
    let mut taskFinishTime: metamodelica::Real;
    let mut taskStartTime: metamodelica::Real;
    let mut primalComp: i32;
    let mut childNodes: metamodelica::List<i32>;
    let mut components: metamodelica::List<i32>;
    let mut simCodeEqs: metamodelica::List<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut calcTimeString: ArcStr;
    let mut opCountString: ArcStr;
    let mut yCoordString: ArcStr;
    let mut taskFinishTimeString: ArcStr;
    let mut taskStartTimeString: ArcStr;
    let mut compText: ArcStr;
    let mut compsText: ArcStr;
    let mut nodeDesc: ArcStr;
    let mut componentsString: ArcStr;
    let mut simCodeEqString: ArcStr;
    let mut threadIdxString: ArcStr;
    let mut taskNumberString: ArcStr;
    let mut annotationString: ArcStr;
    let mut schedulerThreadId: i32;
    let mut schedulerTaskNumber: i32;
    let mut nodeLabels: metamodelica::List<GraphML::NodeLabel>;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut criticalPath: metamodelica::List<(i32, i32)>;
    let mut criticalPathWoC: metamodelica::List<(i32, i32)>;
    let mut visualizeTaskStartAndFinishTime: bool;
    let mut visualizeTaskCalcTime: bool;
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (tmpGraph, graphIdx) = iGraph.clone();
    if intGt(nodeIdx, 0) {
        (tGraphIn, tGraphDataIn) = tGraphDataTuple.clone();
        let TaskGraphMeta {
            inComps: __pa0,
            compNames: __pa1,
            compDescs: __pa2,
            exeCosts: __pa3,
            nodeMark: __pa4,
            compInformations: __pa5,
            ..
        } = &tGraphDataIn;
        inComps = metamodelica::Own::own(__pa0);
        compNames = metamodelica::Own::own(__pa1);
        compDescs = metamodelica::Own::own(__pa2);
        exeCosts = metamodelica::Own::own(__pa3);
        nodeMark = metamodelica::Own::own(__pa4);
        compInformations = metamodelica::Own::own(__pa5);
        (
            nameAttIdx,
            opCountAttIdx,
            calcTimeAttIdx,
            taskIdAttIdx,
            compsIdAttIdx,
            yCoordAttIdx,
            commCostAttIdx,
            commVarsAttIdx,
            commVarsAttIntIdx,
            commVarsAttFloatIdx,
            commVarsAttBoolIdx,
            simCodeEqAttIdx,
            threadIdAttIdx,
            taskNumberAttIdx,
            annotationAttIdx,
            partOfEventAttIdx,
            partOfOdeAttIdx,
            removedCompAttIdx,
        ) = attIdc;
        (criticalPath, criticalPathWoC, schedulerInfo, annotationInfo) = iSchedulerInfoCritPath.clone();
        let GraphDumpOptions {
            visualizeTaskStartAndFinishTime: __pa6,
            visualizeTaskCalcTime: __pa7,
            ..
        } = iGraphDumpOptions;
        visualizeTaskStartAndFinishTime = metamodelica::Own::own(__pa6);
        visualizeTaskCalcTime = metamodelica::Own::own(__pa7);
        components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
        (isPartOfODESystem, isPartOfZeroFuncSystem, isRemovedComponent) =
            getNodeMembershipByComponents(&components, compInformations.clone())?;
        if intNe(((components).len() as i32), 1) {
            primalComp = List::last(&components)?;
            simCodeEqs = List::flatten(List::map1(
                components.clone(),
                &Array::getIndexFirst,
                sccSimEqMapping.clone(),
            )?)?;
            nodeDesc = stringDelimitList(
                List::map1(components.clone(), &Array::getIndexFirst, compDescs.clone())?,
                literal!("\n"),
            );
            (opCount, calcTime) = List::fold1(
                &components,
                &addNodeToGraphML1,
                exeCosts.clone(),
                (0, metamodelica::OrderedFloat(0.0_f64)),
            )?;
        } else {
            primalComp = (components).get(1)?;
            simCodeEqs = metamodelica::arrayGet(sccSimEqMapping.clone(), primalComp)?;
            nodeDesc = metamodelica::arrayGet(compDescs.clone(), primalComp)?;
            (_, calcTime) = metamodelica::arrayGet(exeCosts.clone(), primalComp)?;
            (opCount, calcTime) = metamodelica::arrayGet(exeCosts.clone(), primalComp)?;
        }
        compText = metamodelica::arrayGet(compNames.clone(), primalComp)?;
        compsText = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(components, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        };
        annotationString = metamodelica::arrayGet(annotationInfo.clone(), nodeIdx)?;
        calcTimeString = realString(calcTime);
        yCoord = metamodelica::arrayGet(nodeMark.clone(), nodeIdx)? * 100;
        opCountString = intString(opCount);
        yCoordString = intString(yCoord);
        childNodes = metamodelica::arrayGet(tGraphIn.clone(), nodeIdx)?;
        simCodeEqString = stringDelimitList(List::map(simCodeEqs, &fnptr!(intString, i32))?, literal!(", "));
        componentsString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*intString(nodeIdx));
            __mm_s.push_str(&*literal!(" "));
            ArcStr::from(__mm_s)
        };
        (schedulerThreadId, schedulerTaskNumber, taskFinishTime) =
            metamodelica::arrayGet(schedulerInfo.clone(), nodeIdx)?;
        taskStartTime = (taskFinishTime) - (calcTime);
        threadIdxString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Th "));
            __mm_s.push_str(&*intString(schedulerThreadId));
            ArcStr::from(__mm_s)
        };
        taskNumberString = intString(schedulerTaskNumber);
        calcTimeString = System::snprintff(literal!("%.0f"), 25, calcTime)?;
        taskFinishTimeString = System::snprintff(literal!("%.0f"), 25, taskFinishTime)?;
        taskStartTimeString = System::snprintff(literal!("%.0f"), 25, taskStartTime)?;
        nodeLabels = list![GraphML::NodeLabel::NODELABEL_INTERNAL {
            text: componentsString.clone(),
            backgroundColor: None,
            fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN
        }];
        nodeLabels = if (visualizeTaskCalcTime) {
            metamodelica::cons(
                GraphML::NodeLabel::NODELABEL_CORNER {
                    text: calcTimeString.clone(),
                    backgroundColor: Some(arcstr::literal!(GraphML::COLOR_YELLOW)),
                    fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD,
                    position: literal!("se"),
                },
                nodeLabels,
            )
        } else {
            nodeLabels
        };
        nodeLabels = if (visualizeTaskStartAndFinishTime) {
            listAppend(
                nodeLabels,
                list![
                    GraphML::NodeLabel::NODELABEL_CORNER {
                        text: taskStartTimeString,
                        backgroundColor: Some(arcstr::literal!(GraphML::COLOR_CYAN)),
                        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD,
                        position: literal!("nw")
                    },
                    GraphML::NodeLabel::NODELABEL_CORNER {
                        text: taskFinishTimeString,
                        backgroundColor: Some(arcstr::literal!(GraphML::COLOR_PINK)),
                        fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD,
                        position: literal!("sw")
                    }
                ],
            )
        } else {
            nodeLabels
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
            nodeLabels,
            openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE,
            Some(nodeDesc),
            list![
                (nameAttIdx, compText),
                (calcTimeAttIdx, calcTimeString),
                (opCountAttIdx, opCountString),
                (taskIdAttIdx, componentsString),
                (compsIdAttIdx, compsText),
                (yCoordAttIdx, yCoordString),
                (simCodeEqAttIdx, simCodeEqString),
                (threadIdAttIdx, threadIdxString),
                (taskNumberAttIdx, taskNumberString),
                (annotationAttIdx, annotationString),
                (partOfEventAttIdx, boolString(isPartOfODESystem)),
                (partOfOdeAttIdx, boolString(isPartOfZeroFuncSystem)),
                (removedCompAttIdx, boolString(isRemovedComponent))
            ],
            graphIdx,
            tmpGraph,
        )?;
        tmpGraph = List::fold(
            &childNodes,
            &({
                let __pe_b1 = nodeIdx;
                let __pe_b2 = tGraphDataIn;
                let __pe_b3 = (
                    commCostAttIdx,
                    commVarsAttIdx,
                    commVarsAttIntIdx,
                    commVarsAttFloatIdx,
                    commVarsAttBoolIdx,
                );
                let __pe_b4 = (criticalPath, criticalPathWoC);
                let __pe_b5 = iGraphDumpOptions;
                move |__pe_a0, __pe_a6| {
                    addDepToGraph(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        &__pe_b4,
                        __pe_b5.clone(),
                        __pe_a6,
                    )
                }
            }),
            tmpGraph,
        )?;
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!("function addNodeToGraphML failed.")],
        )?;
    }
    oGraph = (tmpGraph, graphIdx);
    Ok(oGraph)
}

fn addNodeToGraphML1(
    mut compIdx: i32,
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut exeCostsIn: (i32, metamodelica::Real),
) -> Result<(i32, metamodelica::Real)> {
    let mut exeCostsOut: (i32, metamodelica::Real);
    let mut opCount: i32;
    let mut opCountIn: i32;
    let mut exeTimeIn: metamodelica::Real;
    let mut exeTime: metamodelica::Real;
    (opCountIn, exeTimeIn) = exeCostsIn;
    (opCount, exeTime) = metamodelica::arrayGet(exeCosts.clone(), compIdx)?;
    exeCostsOut = (opCountIn + opCount, (exeTimeIn) + (exeTime));
    Ok(exeCostsOut)
}

fn addDepToGraph(
    mut childIdx: i32,
    mut parentIdx: i32,
    mut tGraphDataIn: TaskGraphMeta,
    mut iCommAttIdc: (i32, i32, i32, i32, i32),
    mut iCriticalPathEdges: &(metamodelica::List<(i32, i32)>, metamodelica::List<(i32, i32)>),
    mut iGraphDumpOptions: GraphDumpOptions,
    mut iGraph: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraph: GraphML::GraphInfo;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut commCostAttIdx: i32;
    let mut commVarsAttIdx: i32;
    let mut commVarsAttIntIdx: i32;
    let mut commVarsAttFloatIdx: i32;
    let mut commVarsAttBoolIdx: i32;
    let mut numOfCommVars: i32;
    let mut commCost: metamodelica::Real;
    let mut commCostString: ArcStr;
    let mut numOfCommVarsString: ArcStr;
    let mut numOfCommVarsIntString: ArcStr;
    let mut numOfCommVarsFloatString: ArcStr;
    let mut numOfCommVarsBoolString: ArcStr;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut criticalPathEdges: metamodelica::List<(i32, i32)>;
    let mut criticalPathEdgesWoC: metamodelica::List<(i32, i32)>;
    let mut edgeColor: ArcStr = arcstr::literal!(GraphML::COLOR_BLACK);
    let mut visualizeCriticalPath: bool;
    let mut visualizeCommTime: bool;
    let mut edgeLabels: metamodelica::List<GraphML::EdgeLabel>;
    let mut lineWidth: metamodelica::Real;
    let TaskGraphMeta {
        commCosts: __pa0,
        nodeMark: __pa1,
        inComps: __pa2,
        ..
    } = &tGraphDataIn;
    commCosts = metamodelica::Own::own(__pa0);
    nodeMark = metamodelica::Own::own(__pa1);
    inComps = metamodelica::Own::own(__pa2);
    (
        commCostAttIdx,
        commVarsAttIdx,
        commVarsAttIntIdx,
        commVarsAttFloatIdx,
        commVarsAttBoolIdx,
    ) = iCommAttIdc;
    (criticalPathEdges, criticalPathEdgesWoC) = iCriticalPathEdges.clone();
    let GraphDumpOptions {
        visualizeCriticalPath: __pa3,
        visualizeCommTime: __pa4,
        ..
    } = iGraphDumpOptions;
    visualizeCriticalPath = metamodelica::Own::own(__pa3);
    visualizeCommTime = metamodelica::Own::own(__pa4);
    if List::exist1(
        &criticalPathEdges,
        &fnptr!(compareIntTuple2, (i32, i32), (i32, i32)),
        (parentIdx, childIdx),
    )? {
        lineWidth = GraphML::LINEWIDTH_BOLD.clone();
        edgeColor = if (visualizeCriticalPath) {
            arcstr::literal!(GraphML::COLOR_GRAY)
        } else {
            edgeColor
        };
    } else {
        lineWidth = GraphML::LINEWIDTH_STANDARD.clone();
    }
    let Communication {
        numberOfVars: __pa5,
        integerVars: __pa6,
        floatVars: __pa7,
        booleanVars: __pa8,
        requiredTime: __pa9,
        ..
    } = getCommCostBetweenNodes(parentIdx, childIdx, tGraphDataIn)?;
    numOfCommVars = metamodelica::Own::own(__pa5);
    integerVars = metamodelica::Own::own(__pa6);
    floatVars = metamodelica::Own::own(__pa7);
    booleanVars = metamodelica::Own::own(__pa8);
    commCost = metamodelica::Own::own(__pa9);
    numOfCommVarsString = intString(numOfCommVars);
    numOfCommVarsIntString = intString(((integerVars).len() as i32));
    numOfCommVarsFloatString = intString(((floatVars).len() as i32));
    numOfCommVarsBoolString = intString(((booleanVars).len() as i32));
    commCostString = System::snprintff(literal!("%.0f"), 25, commCost)?;
    edgeLabels = if (visualizeCommTime) {
        list![GraphML::EdgeLabel {
            text: commCostString.clone(),
            backgroundColor: Some(edgeColor.clone()),
            fontSize: GraphML::FONTSIZE_STANDARD.clone()
        }]
    } else {
        metamodelica::nil()
    };
    (tmpGraph, _) = GraphML::addEdge(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Edge"));
            __mm_s.push_str(&*intString(parentIdx));
            __mm_s.push_str(&*intString(childIdx));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Node"));
            __mm_s.push_str(&*intString(childIdx));
            ArcStr::from(__mm_s)
        },
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Node"));
            __mm_s.push_str(&*intString(parentIdx));
            ArcStr::from(__mm_s)
        },
        edgeColor,
        openmodelica_codegen_graphml::GraphML::LineType::LINE,
        lineWidth,
        false,
        edgeLabels,
        (
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
            openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
        ),
        list![
            (commCostAttIdx, commCostString),
            (commVarsAttIdx, numOfCommVarsString),
            (commVarsAttIntIdx, numOfCommVarsIntString),
            (commVarsAttFloatIdx, numOfCommVarsFloatString),
            (commVarsAttBoolIdx, numOfCommVarsBoolString)
        ],
        iGraph,
    )?;
    oGraph = tmpGraph;
    Ok(oGraph)
}

fn getNodeMembershipByComponents(
    mut iNodeComponents: &metamodelica::List<i32>,
    mut iCompInformations: metamodelica::Array<ComponentInfo>,
) -> Result<(bool, bool, bool)> {
    let mut oMembership: (bool, bool, bool);
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut compIdx: i32 = 0;
    let mut tmpComponentInformation: ComponentInfo;
    tmpComponentInformation = ComponentInfo {
        isPartOfODESystem: false,
        isPartOfZeroFuncSystem: false,
        isRemovedComponent: false,
    };
    for mut compIdx in &**iNodeComponents {
        let mut compIdx = compIdx.clone();
        tmpComponentInformation = combineComponentInformations(
            metamodelica::arrayGet(iCompInformations.clone(), compIdx)?,
            tmpComponentInformation,
        );
    }
    let ComponentInfo {
        isPartOfODESystem: __pa0,
        isPartOfZeroFuncSystem: __pa1,
        isRemovedComponent: __pa2,
    } = tmpComponentInformation;
    isPartOfODESystem = metamodelica::Own::own(__pa0);
    isPartOfZeroFuncSystem = metamodelica::Own::own(__pa1);
    isRemovedComponent = metamodelica::Own::own(__pa2);
    oMembership = (isPartOfODESystem, isPartOfZeroFuncSystem, isRemovedComponent);
    Ok(oMembership)
}

//-----------------
//  Print functions
//-----------------
pub(crate) fn printTaskGraph(mut graphIn: TaskGraph) -> () {
    let mut graphLst: metamodelica::List<metamodelica::List<i32>>;
    metamodelica::print(literal!("\n"));
    metamodelica::print(literal!("--------------------------------\n"));
    metamodelica::print(literal!("TASKGRAPH\n"));
    metamodelica::print(literal!("--------------------------------\n"));
    graphLst = graphIn
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    dumpAdjacencyLst(&graphLst, 1);
    metamodelica::print(literal!("\n"));
    ()
}

fn dumpAdjacencyLst(mut inIntegerLstLst: &metamodelica::List<metamodelica::List<i32>>, mut rowIndex: i32) -> () {
    let () = (::match_deref::match_deref! { match inIntegerLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: row, tail: rows } => {
            metamodelica::print(intString(rowIndex));
            metamodelica::print(literal!(":"));
            dumpAdjacencyRow(metamodelica::AsArg::as_arg(&row));
            dumpAdjacencyLst(rows, rowIndex + 1);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn dumpAdjacencyRow(mut inIntegerLst: &metamodelica::List<i32>) -> () {
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
            dumpAdjacencyRow(xs);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub(crate) fn printTaskGraphMeta(mut metaDataIn: TaskGraphMeta) -> Result<()> {
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = metaDataIn;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    exeCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeMark = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    metamodelica::print(literal!("\n"));
    metamodelica::print(literal!("--------------------------------\n"));
    metamodelica::print(literal!("TASKGRAPH METADATA\n"));
    metamodelica::print(literal!("--------------------------------\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(inComps.clone())));
        __mm_s.push_str(&*literal!(" nodes include components:\n"));
        ArcStr::from(__mm_s)
    });
    printInComps(inComps.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(varCompMapping.clone())));
        __mm_s.push_str(&*literal!(" vars are solved in the nodes \n"));
        ArcStr::from(__mm_s)
    });
    printVarCompMapping(varCompMapping.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(eqCompMapping.clone())));
        __mm_s.push_str(&*literal!(" equations are computed in the nodes \n"));
        ArcStr::from(__mm_s)
    });
    printEqCompMapping(eqCompMapping.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(metamodelica::arrayLength(compParamMapping.clone())));
        __mm_s.push_str(&*literal!(" parameters are part of the components \n"));
        ArcStr::from(__mm_s)
    });
    printCompParamMapping(compParamMapping.clone())?;
    metamodelica::print(literal!("the names of the components \n"));
    printComponentNames(compNames.clone())?;
    metamodelica::print(literal!("the description of the node\n"));
    printCompDescs(compDescs.clone())?;
    metamodelica::print(literal!("the execution costs of the nodes\n"));
    printExeCosts(exeCosts.clone())?;
    metamodelica::print(literal!("the communication costs of the nodes\n"));
    printCommCosts(commCosts.clone())?;
    metamodelica::print(literal!("the nodeMark of the nodes\n"));
    printNodeMarks(nodeMark.clone())?;
    metamodelica::print(literal!("the component informations are\n"));
    printComponentInformations(compInformations.clone())?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn printInComps(mut iInComps: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut compRow: metamodelica::List<i32>;
    for mut nodeIdx in 1..=metamodelica::arrayLength(iInComps.clone()) {
        compRow = metamodelica::arrayGet(iInComps.clone(), nodeIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("node "));
            __mm_s.push_str(&*intString(nodeIdx));
            __mm_s.push_str(&*literal!(" solves components: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(compRow, &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printVarCompMapping(mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut comp: i32;
    let mut eqSysIdx: i32;
    let mut varOffset: i32;
    for mut varIdx in 1..=metamodelica::arrayLength(iVarCompMapping.clone()) {
        (comp, eqSysIdx, varOffset) = metamodelica::arrayGet(iVarCompMapping.clone(), varIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("variable "));
            __mm_s.push_str(&*intString(varIdx - varOffset));
            __mm_s.push_str(&*literal!(" (offset: "));
            __mm_s.push_str(&*intString(varOffset));
            __mm_s.push_str(&*literal!(") of equation system "));
            __mm_s.push_str(&*intString(eqSysIdx));
            __mm_s.push_str(&*literal!(" is solved in component: "));
            __mm_s.push_str(&*intString(comp));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printEqCompMapping(mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut eqIdx: i32 = 0;
    let mut comp: i32;
    let mut eqSysIdx: i32;
    let mut eqOffset: i32;
    for mut eqIdx in 1..=metamodelica::arrayLength(iEqCompMapping.clone()) {
        (comp, eqSysIdx, eqOffset) = metamodelica::arrayGet(iEqCompMapping.clone(), eqIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("equation "));
            __mm_s.push_str(&*intString(eqIdx));
            __mm_s.push_str(&*literal!(" (offset: "));
            __mm_s.push_str(&*intString(eqOffset));
            __mm_s.push_str(&*literal!(") of equation system "));
            __mm_s.push_str(&*intString(eqSysIdx));
            __mm_s.push_str(&*literal!(" is computed in component: "));
            __mm_s.push_str(&*intString(comp));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printCompParamMapping(mut iCompParamMapping: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut params: metamodelica::List<i32>;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompParamMapping.clone()) {
        params = metamodelica::arrayGet(iCompParamMapping.clone(), compIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" needs the parameters: "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(params, &fnptr!(intString, i32))?,
                literal!(","),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printComponentNames(mut iCompNames: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compName: ArcStr;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompNames.clone()) {
        compName = metamodelica::arrayGet(iCompNames.clone(), compIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" is named "));
            __mm_s.push_str(&*compName);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printCompDescs(mut iCompDescs: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compDesc: ArcStr;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompDescs.clone()) {
        compDesc = metamodelica::arrayGet(iCompDescs.clone(), compIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" is described with: "));
            __mm_s.push_str(&*compDesc);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printExeCosts(mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut opCount: i32;
    let mut execTime: metamodelica::Real;
    for mut compIdx in 1..=metamodelica::arrayLength(iExeCosts.clone()) {
        (opCount, execTime) = metamodelica::arrayGet(iExeCosts.clone(), compIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" has execution cost of: ("));
            __mm_s.push_str(&*intString(opCount));
            __mm_s.push_str(&*literal!(","));
            __mm_s.push_str(&*realString(execTime));
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printCommCosts(mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut nodeComms: Communications;
    for mut nodeIdx in 1..=metamodelica::arrayLength(iCommCosts.clone()) {
        nodeComms = metamodelica::arrayGet(iCommCosts.clone(), nodeIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("edges from node "));
            __mm_s.push_str(&*intString(nodeIdx));
            __mm_s.push_str(&*literal!(": with the communication costs "));
            __mm_s.push_str(&*stringDelimitList(
                List::map(nodeComms, &fnptr!(printCommCost, Communication))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printCommCost(mut iComm: Communication) -> ArcStr {
    let mut oCommString: ArcStr;
    let mut numberOfVars: i32;
    let mut numberOfIntegers: i32;
    let mut numberOfFloats: i32;
    let mut numberOfBooleans: i32;
    let mut childNode: i32;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut requiredTime: metamodelica::Real;
    let Communication {
        numberOfVars: __pa0,
        integerVars: __pa1,
        floatVars: __pa2,
        booleanVars: __pa3,
        childNode: __pa4,
        requiredTime: __pa5,
        ..
    } = iComm;
    numberOfVars = metamodelica::Own::own(__pa0);
    integerVars = metamodelica::Own::own(__pa1);
    floatVars = metamodelica::Own::own(__pa2);
    booleanVars = metamodelica::Own::own(__pa3);
    childNode = metamodelica::Own::own(__pa4);
    requiredTime = metamodelica::Own::own(__pa5);
    numberOfIntegers = ((integerVars).len() as i32);
    numberOfFloats = ((floatVars).len() as i32);
    numberOfBooleans = ((booleanVars).len() as i32);
    oCommString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("(target node: "));
        __mm_s.push_str(&*intString(childNode));
        __mm_s.push_str(&*literal!(" ints: "));
        __mm_s.push_str(&*intString(numberOfIntegers));
        __mm_s.push_str(&*literal!(" floats: "));
        __mm_s.push_str(&*intString(numberOfFloats));
        __mm_s.push_str(&*literal!(" booleans: "));
        __mm_s.push_str(&*intString(numberOfBooleans));
        __mm_s.push_str(&*literal!(" [requiredTime: "));
        __mm_s.push_str(&*realString(requiredTime));
        __mm_s.push_str(&*literal!(" for "));
        __mm_s.push_str(&*intString(numberOfVars));
        __mm_s.push_str(&*literal!(" variables)"));
        ArcStr::from(__mm_s)
    };
    oCommString
}

fn printNodeMarks(mut iNodeMarks: metamodelica::Array<i32>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut mark: i32;
    for mut compIdx in 1..=metamodelica::arrayLength(iNodeMarks.clone()) {
        mark = metamodelica::arrayGet(iNodeMarks.clone(), compIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" has the nodeMark : "));
            __mm_s.push_str(&*intString(mark));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

fn printComponentInformations(mut iComponentInformations: metamodelica::Array<ComponentInfo>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    for mut compIdx in 1..=metamodelica::arrayLength(iComponentInformations.clone()) {
        let ComponentInfo {
            isPartOfODESystem: __pa0,
            isPartOfZeroFuncSystem: __pa1,
            isRemovedComponent: __pa2,
        } = metamodelica::arrayGet(iComponentInformations.clone(), compIdx)?;
        isPartOfODESystem = metamodelica::Own::own(__pa0);
        isPartOfZeroFuncSystem = metamodelica::Own::own(__pa1);
        isRemovedComponent = metamodelica::Own::own(__pa2);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*intString(compIdx));
            __mm_s.push_str(&*literal!(" has component information:\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("   Is part of ODE-System:   "));
            __mm_s.push_str(&*boolString(isPartOfODESystem));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("   Is part of Event-System: "));
            __mm_s.push_str(&*boolString(isPartOfZeroFuncSystem));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("   Is removed component:    "));
            __mm_s.push_str(&*boolString(isRemovedComponent));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("--------------------------------\n"));
    Ok(())
}

pub(crate) fn intLstString(mut lstIn: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut strOut: ArcStr;
    let mut r#str: ArcStr;
    r#str = stringDelimitList(List::map(lstIn.clone(), &fnptr!(intString, i32))?, literal!(","));
    strOut = if ((lstIn).is_empty()) { literal!("---") } else { r#str };
    Ok(strOut)
}

pub(crate) fn dumpCriticalPathInfo(
    mut iCriticalPaths: &(metamodelica::List<metamodelica::List<i32>>, metamodelica::Real),
    mut iCriticalPathsWoC: &(metamodelica::List<metamodelica::List<i32>>, metamodelica::Real),
) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut tmpString: ArcStr;
    let mut critPath: metamodelica::List<metamodelica::List<i32>>;
    let mut critPathWoC: metamodelica::List<metamodelica::List<i32>>;
    let mut costPath: metamodelica::Real;
    let mut costPathWoC: metamodelica::Real;
    oString = (::match_deref::match_deref! { match &((iCriticalPaths, iCriticalPathsWoC)) {
        ((Deref @ metamodelica::ListNode::Nil, _), _) => literal!(""),
        ((__esc_critPath, __esc_costPath), (__esc_critPathWoC, __esc_costPathWoC)) => {
            critPath = (*__esc_critPath).clone();
            costPath = (*__esc_costPath).clone();
            critPathWoC = (*__esc_critPathWoC).clone();
            costPathWoC = (*__esc_costPathWoC).clone();
            tmpString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("critical path with costs of ")); __mm_s.push_str(&*realString(costPath.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) };
            tmpString = { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*dumpCriticalPathInfo1(metamodelica::AsArg::as_arg(&critPath), 1)?); ArcStr::from(__mm_s) };
            tmpString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ;; ")); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*literal!("critical path' with costs of ")); __mm_s.push_str(&*realString(costPathWoC.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) };
            tmpString = { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*dumpCriticalPathInfo1(metamodelica::AsArg::as_arg(&critPathWoC), 1)?); ArcStr::from(__mm_s) };
            tmpString
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oString)
}

fn dumpCriticalPathInfo1(
    mut criticalPathsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut cpIdx: i32,
) -> Result<ArcStr> {
    let mut oString: ArcStr;
    oString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intLstString((criticalPathsIn).get(cpIdx)?)?);
        __mm_s.push_str(&*literal!(""));
        ArcStr::from(__mm_s)
    };
    Ok(oString)
}

fn printCriticalPathInfo(
    mut criticalPathsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut cpCosts: metamodelica::Real,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match criticalPathsIn {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => {
            metamodelica::print(literal!("--------------------------------\n"));
            metamodelica::print(literal!(" CRITICAL PATH INFO\n"));
            metamodelica::print(literal!("--------------------------------\n"));
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("found ")); __mm_s.push_str(&*intString(((criticalPathsIn).len() as i32))); __mm_s.push_str(&*literal!(" critical paths with costs of ")); __mm_s.push_str(&*realString(cpCosts)); __mm_s.push_str(&*literal!(" sec\n")); ArcStr::from(__mm_s) });
            printCriticalPathInfo1(criticalPathsIn, 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printCriticalPathInfo1(
    mut criticalPathsIn: &metamodelica::List<metamodelica::List<i32>>,
    mut cpIdx: i32,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(cpIdx));
        __mm_s.push_str(&*literal!(". path: "));
        __mm_s.push_str(&*intLstString((criticalPathsIn).get(cpIdx)?)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

//--------------------------
//  Functions to merge nodes
//--------------------------
fn mergeSingleNodes(
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
    mut doNotMergeIn: metamodelica::List<i32>,
) -> (TaskGraph, TaskGraphMeta, bool) {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut changed: bool = false;
    (oTaskGraph, oTaskGraphMeta, changed) = 'mc: {
        let __mc_input = &*doNotMergeIn;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32;
                    let mut singleNodes: metamodelica::List<i32>;
                    let mut singleNodes1: metamodelica::List<i32>;
                    let mut pos: metamodelica::List<i32>;
                    let mut exeCosts: metamodelica::List<metamodelica::Real>;
                    let mut taskGraphT: TaskGraph;
                    let mut changed: bool = changed.clone();
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
                    (_, singleNodes) = List::filterOnTrueSync(&(iTaskGraph.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), &fnptr!(listEmpty, _), List::intRange(metamodelica::arrayLength(iTaskGraph.clone())))?;
                    (_, singleNodes1) = List::filterOnTrueSync(&(taskGraphT.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), &fnptr!(listEmpty, _), List::intRange(metamodelica::arrayLength(taskGraphT.clone())))?;
                    (singleNodes, _, _) = List::intersection1OnTrue(singleNodes.clone(), singleNodes1.clone(), &fnptr!(intEq, i32, i32))?;
                    (_, singleNodes, _) = List::intersection1OnTrue(singleNodes.clone(), doNotMergeIn.clone(), &fnptr!(intEq, i32, i32))?;
                    exeCosts = List::map1(singleNodes.clone(), &getExeCostReqCycles, iTaskGraphMeta.clone())?;
                    (exeCosts, pos) = HpcOmScheduler::quicksortWithOrder(exeCosts.clone())?;
                    singleNodes = List::map1(pos.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), singleNodes.clone())?;
                    singleNodes = singleNodes.clone().reverse();
                    exeCosts = exeCosts.clone().reverse();
                    distributeToClusters(singleNodes.clone(), exeCosts.clone(), numProc)?;
                    changed = intGt(((singleNodes).len() as i32), numProc);
                    Ok(((iTaskGraph.clone(), iTaskGraphMeta.clone(), changed), changed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            changed = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iTaskGraph.clone(), iTaskGraphMeta.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oTaskGraph, oTaskGraphMeta, changed)
}

pub(crate) fn distributeToClusters(
    mut items: metamodelica::List<i32>,
    mut values: metamodelica::List<metamodelica::Real>,
    mut numClusters: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::Real>,
)> {
    let mut clustersOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    let mut b: bool;
    let mut itemArr: metamodelica::Array<i32>;
    let mut itemsCopy: metamodelica::Array<metamodelica::List<i32>>;
    let mut clusters: metamodelica::Array<metamodelica::List<i32>>;
    let mut clusterValues: metamodelica::Array<metamodelica::Real>;
    b = intGt(((items).len() as i32), numClusters);
    clusters = metamodelica::arrayFromVec(
        List::map(List::intRange(((items).len() as i32)), &fnptr!(List::create, _))?
            .into_iter()
            .cloned()
            .collect(),
    );
    clusterValues = metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect());
    itemArr = metamodelica::arrayFromVec(items.clone().into_iter().cloned().collect());
    itemsCopy = Array::map(itemArr.clone(), &fnptr!(List::create, _))?;
    clusters = if (true) {
        Array::copy(itemsCopy.clone(), clusters.clone())?
    } else {
        clusters.clone()
    };
    clusterValues = if (!(b)) {
        Array::copy(
            metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect()),
            clusterValues.clone(),
        )?
    } else {
        clusterValues.clone()
    };
    if b {
        (clustersOut, clusterValuesOut) = distributeToClusters1(
            &((items, values)),
            (clusters.clone(), clusterValues.clone()),
            numClusters,
        )?;
    } else {
        (clustersOut, clusterValuesOut) = (clusters.clone(), clusterValues.clone());
    }
    Ok((clustersOut, clusterValuesOut))
}

fn distributeToClusters1(
    mut tplIn: &(metamodelica::List<i32>, metamodelica::List<metamodelica::Real>),
    mut tplFold: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::Real>,
    ),
    mut numClusters: i32,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::Real>,
)> {
    let mut clustersOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    (clustersOut, clusterValuesOut) = 'mc: {
        let __mc_input = (tplIn, tplFold);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, _), (clusters, clusterValues)) => {
                    let mut idcsLst1: metamodelica::List<i32>;
                    let mut clustersFinal: metamodelica::Array<metamodelica::List<i32>>;
                    let mut clusterValuesFinal: metamodelica::Array<metamodelica::Real>;
                    let true = (((itemsIn).len() as i32) <= numClusters) else { return Err("pattern mismatch") };
                    idcsLst1 = List::intRange(numClusters);
                    clustersFinal = Array::select(clusters.clone(), &idcsLst1)?;
                    clusterValuesFinal = Array::select(clusterValues.clone(), &idcsLst1)?;
                    Ok((clustersFinal.clone(), clusterValuesFinal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues)) => {
                    let mut diff: i32;
                    let mut lst1: metamodelica::List<i32>;
                    let mut idcsLst2: metamodelica::List<i32>;
                    let mut idcsLst1: metamodelica::List<i32>;
                    let mut entries: metamodelica::List<metamodelica::List<i32>>;
                    let mut entries2: metamodelica::List<metamodelica::List<i32>>;
                    let mut values: metamodelica::List<metamodelica::Real>;
                    let mut addValues: metamodelica::List<metamodelica::Real>;
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = (((itemsIn).len() as i32) > numClusters) else { return Err("pattern mismatch") };
                    let true = (metamodelica::real_div_checked(metamodelica::OrderedFloat((((itemsIn).len() as i32)) as f64), metamodelica::OrderedFloat((2) as f64))? < metamodelica::OrderedFloat((numClusters) as f64)) else { return Err("pattern mismatch") };
                    (lst1, _) = List::split(itemsIn.clone(), numClusters)?;
                    diff = ((itemsIn).len() as i32) - numClusters;
                    idcsLst1 = List::intRange2(numClusters - diff + 1, numClusters);
                    idcsLst2 = List::intRange2(numClusters + 1, ((itemsIn).len() as i32));
                    entries = List::map1(idcsLst2.clone(), &Array::getIndexFirst, clusters.clone())?;
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1.clone(), &Array::getIndexFirst, clusters.clone())?;
                    entries = List::threadMap(entries.clone(), entries2.clone(), &fnptr!(listAppend, metamodelica::List<i32>, metamodelica::List<i32>))?;
                    List::threadMap1_0(&idcsLst1, entries.clone(), &Array::updateIndexFirst, clusters.clone())?;
                    values = List::map1(idcsLst1.clone(), &Array::getIndexFirst, clusterValues.clone())?;
                    addValues = List::map1(idcsLst2.clone(), &Array::getIndexFirst, clusterValues.clone())?;
                    values = List::threadMap(values.clone(), addValues.clone(), &fnptr!(realAdd, metamodelica::Real, metamodelica::Real))?;
                    List::threadMap1_0(&idcsLst1, values.clone(), &Array::updateIndexFirst, clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1(&((lst1.clone(), valuesIn.clone())), (clusters.clone(), clusterValues.clone()), numClusters)?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues)) => {
                    let mut numCl: i32;
                    let mut lst1: metamodelica::List<i32>;
                    let mut idcsLst1_2: metamodelica::List<i32>;
                    let mut idcsLst2: metamodelica::List<i32>;
                    let mut entries: metamodelica::List<metamodelica::List<i32>>;
                    let mut entries2: metamodelica::List<metamodelica::List<i32>>;
                    let mut values: metamodelica::List<metamodelica::Real>;
                    let mut addValues: metamodelica::List<metamodelica::Real>;
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = (((itemsIn).len() as i32) > numClusters) else { return Err("pattern mismatch") };
                    let true = (metamodelica::real_div_checked(metamodelica::OrderedFloat((((itemsIn).len() as i32)) as f64), metamodelica::OrderedFloat((2) as f64))? >= metamodelica::OrderedFloat((numClusters) as f64)) else { return Err("pattern mismatch") };
                    numCl = nextGreaterPowerOf2(intReal(((itemsIn).len() as i32)));
                    (lst1, _) = List::split(itemsIn.clone(), intDiv(numCl, 2))?;
                    idcsLst2 = List::intRange2(intDiv(numCl, 2) + 1, ((itemsIn).len() as i32));
                    idcsLst1_2 = List::intRange2(intDiv(numCl, 2) - ((idcsLst2).len() as i32) + 1, intDiv(numCl, 2));
                    entries = List::map1(idcsLst2.clone(), &Array::getIndexFirst, clusters.clone())?;
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1_2.clone(), &Array::getIndexFirst, clusters.clone())?;
                    entries = List::threadMap(entries.clone(), entries2.clone(), &fnptr!(listAppend, metamodelica::List<i32>, metamodelica::List<i32>))?;
                    List::threadMap1_0(&idcsLst1_2, entries.clone(), &Array::updateIndexFirst, clusters.clone())?;
                    values = List::map1(idcsLst1_2.clone(), &Array::getIndexFirst, clusterValues.clone())?;
                    addValues = List::map1(idcsLst2.clone(), &Array::getIndexFirst, clusterValues.clone())?;
                    values = List::threadMap(values.clone(), addValues.clone(), &fnptr!(realAdd, metamodelica::Real, metamodelica::Real))?;
                    List::threadMap1_0(&idcsLst1_2, values.clone(), &Array::updateIndexFirst, clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1(&((lst1.clone(), valuesIn.clone())), (clusters.clone(), clusterValues.clone()), numClusters)?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("distributeToClusters failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((clustersOut, clusterValuesOut))
}

fn nextGreaterPowerOf2(mut n: metamodelica::Real) -> i32 {
    let mut powOf2: i32;
    powOf2 = nextGreaterPowerOf2_impl(n, 1);
    powOf2
}

fn nextGreaterPowerOf2_impl(mut n: metamodelica::Real, mut pow: i32) -> i32 {
    '__tco: loop {
        let mut p: metamodelica::Real;
        p = realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow));
        if (n <= p) {
            return ((p).0.floor() as i32);
        } else {
            {
                (n, pow) = (n, pow + 1);
                continue '__tco;
            }
        }
    }
}

pub(crate) fn mergeSimpleNodes(
    mut graphIn: TaskGraph,
    mut graphTIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
    mut contractedTasksIn: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut allNodes: metamodelica::List<i32>;
    let mut oneChildren: metamodelica::List<metamodelica::List<i32>>;
    allNodes = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    oneChildren = findOneChildParents(
        &allNodes,
        graphIn.clone(),
        &(metamodelica::nil()),
        &(list![metamodelica::nil()]),
        0,
        contractedTasksIn.clone(),
    )?;
    oneChildren = listDelete(oneChildren.clone(), ((oneChildren).len() as i32))?;
    oneChildren = List::removeOnTrue(
        1,
        &move |__a0: i32, __a1: metamodelica::List<i32>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(compareListLengthOnTrue(__a0, &__a1))
        },
        oneChildren,
    )?;
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(
        &oneChildren,
        graphIn.clone(),
        graphTIn.clone(),
        graphDataIn,
        contractedTasksIn.clone(),
    )?;
    changed = !((oneChildren).is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub(crate) fn mergeParentNodes(
    mut graphIn: TaskGraph,
    mut graphTIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
    mut contractedTasksIn: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: metamodelica::List<metamodelica::List<i32>>;
    alreadyMerged = arrayCreate(metamodelica::arrayLength(graphIn.clone()), 0);
    mergedNodes = mergeParentNodes0(
        graphIn.clone(),
        graphTIn.clone(),
        &graphDataIn,
        contractedTasksIn.clone(),
        alreadyMerged.clone(),
        1,
        &(metamodelica::nil()),
    );
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(
        &mergedNodes,
        graphIn.clone(),
        graphTIn.clone(),
        graphDataIn,
        contractedTasksIn.clone(),
    )?;
    changed = !((mergedNodes).is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

fn mergeParentNodes0(
    mut iGraph: TaskGraph,
    mut iGraphT: TaskGraph,
    mut iGraphData: &TaskGraphMeta,
    mut contractedTasksIn: metamodelica::Array<i32>,
    mut alreadyMerged: metamodelica::Array<i32>,
    mut iNodeIdx: i32,
    mut iMergedNodes: &metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut oMergedNodes: metamodelica::List<metamodelica::List<i32>>;
    let mut highestParentExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut sumParentExeCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parentNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut mergeNodeList: metamodelica::List<i32> = metamodelica::nil();
    let mut highestCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut parentExeCosts: metamodelica::List<(i32, metamodelica::Real)> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut parentCommCosts: Communications = metamodelica::nil();
    let mut parentChilds: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpMergedNodes: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    oMergedNodes = 'mc: {
        let __mc_input = iGraphData.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let TaskGraphMeta {
                exeCosts: mut exeCosts,
                commCosts: mut commCosts,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut highestCommCost: metamodelica::Real = highestCommCost.clone();
            let mut highestParentExeCost: metamodelica::Real = highestParentExeCost.clone();
            let mut mergeNodeList: metamodelica::List<i32> = mergeNodeList.clone();
            let mut parentChilds: metamodelica::List<metamodelica::List<i32>> = parentChilds.clone();
            let mut parentCommCosts: metamodelica::List<Communication> = parentCommCosts.clone();
            let mut parentExeCosts: metamodelica::List<(i32, metamodelica::Real)> = parentExeCosts.clone();
            let mut parentNodes: metamodelica::List<i32> = parentNodes.clone();
            let mut sumParentExeCosts: metamodelica::Real = sumParentExeCosts.clone();
            let mut tmpMergedNodes: metamodelica::List<metamodelica::List<i32>> = tmpMergedNodes.clone();
            let true = (intLe(iNodeIdx, metamodelica::arrayLength(iGraphT.clone()))) else {
                return Err("pattern mismatch");
            };
            let true = (intNe(metamodelica::arrayGet(contractedTasksIn.clone(), iNodeIdx)?, -1)) else {
                return Err("pattern mismatch");
            };
            let true = (intNe(metamodelica::arrayGet(alreadyMerged.clone(), iNodeIdx)?, -1)) else {
                return Err("pattern mismatch");
            };
            parentNodes = metamodelica::arrayGet(iGraphT.clone(), iNodeIdx)?;
            parentNodes = filterContractedNodes(parentNodes.clone(), contractedTasksIn.clone())?;
            let false = (List::exist1(&parentNodes, &isNodeContracted, alreadyMerged.clone())?) else {
                return Err("pattern mismatch");
            };
            parentCommCosts = List::map2(
                parentNodes.clone(),
                &getCommCostBetweenNodes,
                iNodeIdx,
                iGraphData.clone(),
            )?;
            let Communication {
                requiredTime: __pa0, ..
            } = getHighestCommCost(
                parentCommCosts.clone(),
                Communication {
                    numberOfVars: 0,
                    integerVars: metamodelica::nil(),
                    floatVars: metamodelica::nil(),
                    booleanVars: metamodelica::nil(),
                    stringVars: metamodelica::nil(),
                    childNode: -1,
                    requiredTime: metamodelica::OrderedFloat(-1.0_f64),
                },
            );
            highestCommCost = metamodelica::Own::own(__pa0);
            parentExeCosts = List::map1(parentNodes.clone(), &getExeCost, iGraphData.clone())?;
            (_, sumParentExeCosts) = List::fold(
                &parentExeCosts,
                &fnptr!(addUpExeCosts, (i32, metamodelica::Real), (i32, metamodelica::Real)),
                (0, metamodelica::OrderedFloat(0.0_f64)),
            )?;
            (_, highestParentExeCost) =
                getHighestExecCost(parentExeCosts.clone(), (0, metamodelica::OrderedFloat(0.0_f64)));
            let true = (realGt((highestCommCost) + (highestParentExeCost), sumParentExeCosts)) else {
                return Err("pattern mismatch");
            };
            parentChilds = List::map1(parentNodes.clone(), &Array::getIndexFirst, iGraph.clone())?;
            let true = ((List::removeOnTrue(
                1,
                &fnptr!(intEq, i32, i32),
                List::map(parentChilds.clone(), &fnptr!(listLength, _))?,
            )?)
            .is_empty()) else {
                return Err("pattern mismatch");
            };
            mergeNodeList = metamodelica::cons(iNodeIdx, parentNodes.clone());
            tmpMergedNodes = metamodelica::cons(mergeNodeList.clone(), iMergedNodes.clone());
            List::map_0(
                &mergeNodeList,
                &({
                    let __pe_b1 = -1;
                    let __pe_b2 = alreadyMerged.clone();
                    move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?;
            tmpMergedNodes = mergeParentNodes0(
                iGraph.clone(),
                iGraphT.clone(),
                iGraphData,
                contractedTasksIn.clone(),
                alreadyMerged.clone(),
                iNodeIdx + 1,
                &tmpMergedNodes,
            );
            Ok((
                tmpMergedNodes.clone(),
                highestCommCost.clone(),
                highestParentExeCost.clone(),
                mergeNodeList.clone(),
                parentChilds.clone(),
                parentCommCosts.clone(),
                parentExeCosts.clone(),
                parentNodes.clone(),
                sumParentExeCosts.clone(),
                tmpMergedNodes.clone(),
            ))
        })() {
            highestCommCost = __wb0;
            highestParentExeCost = __wb1;
            mergeNodeList = __wb2;
            parentChilds = __wb3;
            parentCommCosts = __wb4;
            parentExeCosts = __wb5;
            parentNodes = __wb6;
            sumParentExeCosts = __wb7;
            tmpMergedNodes = __wb8;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut tmpMergedNodes: metamodelica::List<metamodelica::List<i32>> = tmpMergedNodes.clone();
            let true = (intLe(iNodeIdx, metamodelica::arrayLength(iGraphT.clone()))) else {
                return Err("pattern mismatch");
            };
            tmpMergedNodes = mergeParentNodes0(
                iGraph.clone(),
                iGraphT.clone(),
                iGraphData,
                contractedTasksIn.clone(),
                alreadyMerged.clone(),
                iNodeIdx + 1,
                iMergedNodes,
            );
            Ok((tmpMergedNodes.clone(), tmpMergedNodes.clone()))
        })() {
            tmpMergedNodes = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iMergedNodes.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oMergedNodes
}

fn mergeSinkNodes(
    mut graphIn: TaskGraph,
    mut graphTIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
    mut contractedTasksIn: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: metamodelica::List<metamodelica::List<i32>>;
    alreadyMerged = arrayCreate(metamodelica::arrayLength(graphIn.clone()), 0);
    mergedNodes = mergeParentNodes0(
        graphIn.clone(),
        graphTIn.clone(),
        &graphDataIn,
        contractedTasksIn.clone(),
        alreadyMerged.clone(),
        1,
        &(metamodelica::nil()),
    );
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(
        &mergedNodes,
        graphIn.clone(),
        graphTIn.clone(),
        graphDataIn,
        contractedTasksIn.clone(),
    )?;
    changed = !((mergedNodes).is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub(crate) fn markSystemComponents(
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
    mut iComponentMarks: (bool, bool, bool),
    mut iTargetTaskGraphMeta: TaskGraphMeta,
) -> Result<TaskGraphMeta> {
    let mut oTargetTaskGraphMeta: TaskGraphMeta;
    let mut odeInComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeComps: metamodelica::List<i32>;
    let mut nodeIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut componentInformation: ComponentInfo;
    let mut iComponentInformation: ComponentInfo;
    iComponentInformation = ComponentInfo {
        isPartOfODESystem: Util::tuple31(iComponentMarks),
        isPartOfZeroFuncSystem: Util::tuple32(iComponentMarks),
        isRemovedComponent: Util::tuple33(iComponentMarks),
    };
    let TaskGraphMeta { inComps: __pa0, .. } = iTaskGraphMeta;
    odeInComps = metamodelica::Own::own(__pa0);
    let TaskGraphMeta {
        inComps: __pa1,
        varCompMapping: __pa2,
        eqCompMapping: __pa3,
        compParamMapping: __pa4,
        compNames: __pa5,
        compDescs: __pa6,
        exeCosts: __pa7,
        commCosts: __pa8,
        nodeMark: __pa9,
        compInformations: __pa10,
    } = iTargetTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa1);
    varCompMapping = metamodelica::Own::own(__pa2);
    eqCompMapping = metamodelica::Own::own(__pa3);
    compParamMapping = metamodelica::Own::own(__pa4);
    compNames = metamodelica::Own::own(__pa5);
    compDescs = metamodelica::Own::own(__pa6);
    exeCosts = metamodelica::Own::own(__pa7);
    commCosts = metamodelica::Own::own(__pa8);
    nodeMark = metamodelica::Own::own(__pa9);
    compInformations = metamodelica::Own::own(__pa10);
    for mut nodeIdx in 1..=metamodelica::arrayLength(iTaskGraph.clone()) {
        nodeComps = metamodelica::arrayGet(odeInComps.clone(), nodeIdx)?;
        for mut compIdx in &*nodeComps {
            let mut compIdx = compIdx.clone();
            componentInformation = combineComponentInformations(
                metamodelica::arrayGet(compInformations.clone(), compIdx)?,
                iComponentInformation,
            );
            compInformations = metamodelica::arrayUpdate(compInformations.clone(), compIdx, componentInformation)?;
        }
    }
    oTargetTaskGraphMeta = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok(oTargetTaskGraphMeta)
}

fn combineComponentInformations(
    mut iComponentInfo: ComponentInfo,
    mut iComponentInfo2: ComponentInfo,
) -> ComponentInfo {
    let mut oComponentInfo: ComponentInfo;
    let mut isPartOfODESystem: bool;
    let mut iIsPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut iisPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut iIsRemovedComponent: bool;
    let ComponentInfo {
        isPartOfODESystem: __pa0,
        isPartOfZeroFuncSystem: __pa1,
        isRemovedComponent: __pa2,
    } = iComponentInfo;
    isPartOfODESystem = metamodelica::Own::own(__pa0);
    isPartOfZeroFuncSystem = metamodelica::Own::own(__pa1);
    isRemovedComponent = metamodelica::Own::own(__pa2);
    let ComponentInfo {
        isPartOfODESystem: __pa3,
        isPartOfZeroFuncSystem: __pa4,
        isRemovedComponent: __pa5,
    } = iComponentInfo2;
    iIsPartOfODESystem = metamodelica::Own::own(__pa3);
    iisPartOfZeroFuncSystem = metamodelica::Own::own(__pa4);
    iIsRemovedComponent = metamodelica::Own::own(__pa5);
    oComponentInfo = ComponentInfo {
        isPartOfODESystem: boolOr(isPartOfODESystem, iIsPartOfODESystem),
        isPartOfZeroFuncSystem: boolOr(isPartOfZeroFuncSystem, iisPartOfZeroFuncSystem),
        isRemovedComponent: boolOr(isRemovedComponent, iIsRemovedComponent),
    };
    oComponentInfo
}

fn addUpExeCosts(
    mut iExeCost1: (i32, metamodelica::Real),
    mut iExeCost2: (i32, metamodelica::Real),
) -> (i32, metamodelica::Real) {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut ex1: metamodelica::Real;
    let mut ex2: metamodelica::Real;
    let mut op1: i32;
    let mut op2: i32;
    (op1, ex1) = iExeCost1;
    (op2, ex2) = iExeCost2;
    oExeCost = (op1 + op2, (ex1) + (ex2));
    oExeCost
}

pub(crate) fn getExeCostReqCycles(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut oExeCost: metamodelica::Real;
    oExeCost = Util::tuple22(getExeCost(iNodeIdx, iGraphData)?);
    Ok(oExeCost)
}

pub(crate) fn getExeCost(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<(i32, metamodelica::Real)> {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut comp: i32 = 0;
    let mut opCount: i32;
    let mut opCount1: i32;
    let mut exeCost: metamodelica::Real;
    let mut exeCost1: metamodelica::Real;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut comps: metamodelica::List<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta {
        inComps: __pa0,
        exeCosts: __pa1,
        ..
    } = iGraphData;
    inComps = metamodelica::Own::own(__pa0);
    exeCosts = metamodelica::Own::own(__pa1);
    exeCost = metamodelica::OrderedFloat(0.0_f64);
    opCount = 0;
    comps = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
    for mut comp in &*comps {
        let mut comp = comp.clone();
        (opCount1, exeCost1) = metamodelica::arrayGet(exeCosts.clone(), comp)?;
        opCount = intAdd(opCount, opCount1);
        exeCost = (exeCost) + (exeCost1);
    }
    oExeCost = (opCount, exeCost);
    Ok(oExeCost)
}

fn getHighestExecCost(
    mut iExecCosts: metamodelica::List<(i32, metamodelica::Real)>,
    mut iHighestTuple: (i32, metamodelica::Real),
) -> (i32, metamodelica::Real) {
    '__tco: loop {
        let mut highestCost: metamodelica::Real;
        let mut currentCost: metamodelica::Real;
        let mut head: (i32, metamodelica::Real);
        let mut rest: metamodelica::List<(i32, metamodelica::Real)>;
        ::match_deref::match_deref! { match &((iExecCosts, iHighestTuple)) {
            (Deref @ metamodelica::ListNode::Cons { head: __esc_head @ (_, currentCost), tail: __esc_rest }, (_, highestCost)) if (realGt(currentCost.clone(), highestCost.clone())) => {
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                { (iExecCosts, iHighestTuple) = (rest.clone(), head.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: __esc_rest }, _) => {
                rest = (*__esc_rest).clone();
                { (iExecCosts, iHighestTuple) = (rest.clone(), iHighestTuple); continue '__tco; }
            },
            _ => return iHighestTuple,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn contractNodesInGraph(
    mut iContractNodes: &metamodelica::List<metamodelica::List<i32>>,
    mut iTaskGraph: TaskGraph,
    mut iTaskGraphT: TaskGraph,
    mut iTaskGraphMeta: TaskGraphMeta,
    mut iContractedTasks: metamodelica::Array<i32>,
) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphT: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut oContractedTasks: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpTaskGraph: TaskGraph = iTaskGraph.clone();
    let mut tmpTaskGraphT: TaskGraph = iTaskGraphT.clone();
    let mut tmpContractedTasks: metamodelica::Array<i32> = iContractedTasks.clone();
    let mut nodeListHeadIdx: i32;
    let mut negNodeListHeadIdx: i32;
    let mut nodeIdx: i32 = 0;
    let mut parentChild: i32 = 0;
    let mut parentChildContractionValue: i32;
    let mut nodeListRestIdc: metamodelica::List<i32>;
    let mut nodeCompIdc: metamodelica::List<i32>;
    let mut headCompIdc: metamodelica::List<i32>;
    let mut parentNodeChildList: metamodelica::List<i32>;
    let mut parentNodeChildListNew: metamodelica::List<i32>;
    let mut outgoingEdges: metamodelica::List<i32>;
    let mut incomingEdges: metamodelica::List<i32>;
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut nodeMarksT: metamodelica::Array<i32>;
    let mut iNodeList: metamodelica::List<i32> = metamodelica::nil();
    let mut nodeList: metamodelica::List<i32>;
    let mut childNodes: metamodelica::List<i32>;
    let mut parentNodes: metamodelica::List<i32>;
    let TaskGraphMeta { inComps: __pa0, .. } = &iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    nodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    nodeMarksT = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    for mut iNodeList in &**iContractNodes {
        let mut iNodeList = iNodeList.clone();
        nodeList = metamodelica::nil();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(iNodeList.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        nodeListHeadIdx = metamodelica::Own::own(__pa1);
        nodeListRestIdc = metamodelica::Own::own(__pa2);
        for mut nodeIdx in &*iNodeList {
            let mut nodeIdx = nodeIdx.clone();
            nodeIdx = getRealTaskIdxOfTask(nodeIdx, tmpContractedTasks.clone())?;
            if intNe(metamodelica::arrayGet(nodeMarks.clone(), nodeIdx)?, nodeListHeadIdx) {
                nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeIdx, nodeListHeadIdx)?;
                nodeList = metamodelica::cons(nodeIdx, nodeList);
            }
        }
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(nodeList.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        nodeListHeadIdx = metamodelica::Own::own(__pa3);
        nodeListRestIdc = metamodelica::Own::own(__pa4);
        nodeListHeadIdx = getRealTaskIdxOfTask(nodeListHeadIdx, tmpContractedTasks.clone())?;
        negNodeListHeadIdx = intMul(-1, nodeListHeadIdx);
        for mut nodeIdx in &*nodeListRestIdc {
            let mut nodeIdx = nodeIdx.clone();
            nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeIdx, nodeListHeadIdx)?;
            nodeMarksT = metamodelica::arrayUpdate(nodeMarksT.clone(), nodeIdx, nodeListHeadIdx)?;
            tmpContractedTasks = metamodelica::arrayUpdate(tmpContractedTasks.clone(), nodeIdx, negNodeListHeadIdx)?;
        }
        nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeListHeadIdx, nodeListHeadIdx)?;
        nodeMarksT = metamodelica::arrayUpdate(nodeMarksT.clone(), nodeListHeadIdx, nodeListHeadIdx)?;
        outgoingEdges = metamodelica::arrayGet(tmpTaskGraph.clone(), nodeListHeadIdx)?;
        (outgoingEdges, _) = List::deleteMemberOnTrue(
            negNodeListHeadIdx,
            outgoingEdges,
            &({
                let __pe_b2 = tmpContractedTasks.clone();
                move |__pe_a0, __pe_a1| checkIfNodeBelongsToCluster(__pe_a0, __pe_a1, __pe_b2.clone())
            }),
        )?;
        incomingEdges = metamodelica::arrayGet(tmpTaskGraphT.clone(), nodeListHeadIdx)?;
        List::map_0(
            &outgoingEdges,
            &({
                let __pe_b1 = nodeListHeadIdx;
                let __pe_b2 = nodeMarks.clone();
                move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        List::map_0(
            &incomingEdges,
            &({
                let __pe_b1 = nodeListHeadIdx;
                let __pe_b2 = nodeMarksT.clone();
                move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        childNodes = List::flatten(List::map(
            nodeListRestIdc.clone(),
            &({
                let __pe_b1 = nodeListHeadIdx;
                let __pe_b2 = tmpTaskGraph.clone();
                let __pe_b3 = tmpContractedTasks.clone();
                let __pe_b4 = nodeMarks.clone();
                move |__pe_a0| {
                    getContractedNodeChildren(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                    )
                }
            }),
        )?)?;
        parentNodes = List::flatten(List::map(
            nodeList,
            &({
                let __pe_b1 = nodeListHeadIdx;
                let __pe_b2 = iTaskGraphT.clone();
                let __pe_b3 = tmpContractedTasks.clone();
                let __pe_b4 = nodeMarks.clone();
                move |__pe_a0| {
                    getContractedNodeChildren(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                    )
                }
            }),
        )?)?;
        headCompIdc = metamodelica::arrayGet(inComps.clone(), nodeListHeadIdx)?;
        for mut nodeIdx in &*nodeListRestIdc {
            let mut nodeIdx = nodeIdx.clone();
            tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeIdx, metamodelica::nil())?;
            tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeIdx, metamodelica::nil())?;
            nodeCompIdc = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            inComps = metamodelica::arrayUpdate(inComps.clone(), nodeIdx, metamodelica::nil())?;
            headCompIdc = List::insertListSorted(&headCompIdc, &nodeCompIdc, &fnptr!(intLt, i32, i32))?;
        }
        metamodelica::arrayUpdate(inComps.clone(), nodeListHeadIdx, headCompIdc)?;
        for mut nodeIdx in &*parentNodes {
            let mut nodeIdx = nodeIdx.clone();
            if intNe(metamodelica::arrayGet(nodeMarksT.clone(), nodeIdx)?, nodeListHeadIdx) {
                incomingEdges = metamodelica::cons(nodeIdx, incomingEdges);
            }
        }
        tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeListHeadIdx, incomingEdges)?;
        for mut nodeIdx in &*childNodes {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = metamodelica::arrayGet(tmpTaskGraphT.clone(), nodeIdx)?;
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = metamodelica::arrayGet(tmpContractedTasks.clone(), parentChild)?;
                parentChild = getRealTaskIdxOfTask(parentChild, tmpContractedTasks.clone())?;
                if intEq(parentChild, nodeListHeadIdx) || intEq(parentChildContractionValue, negNodeListHeadIdx) {
                    if intNe(metamodelica::arrayGet(nodeMarksT.clone(), parentChild)?, nodeIdx) {
                        parentNodeChildListNew = metamodelica::cons(nodeListHeadIdx, parentNodeChildListNew);
                        metamodelica::arrayUpdate(nodeMarksT.clone(), parentChild, nodeIdx)?;
                    }
                } else {
                    parentNodeChildListNew = metamodelica::cons(parentChild, parentNodeChildListNew);
                }
            }
            tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeIdx, parentNodeChildListNew)?;
        }
        outgoingEdges = listAppend(outgoingEdges, childNodes);
        nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeListHeadIdx, 0)?;
        for mut nodeIdx in &*parentNodes {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = metamodelica::arrayGet(tmpTaskGraph.clone(), nodeIdx)?;
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = metamodelica::arrayGet(tmpContractedTasks.clone(), parentChild)?;
                parentChild = getRealTaskIdxOfTask(parentChild, tmpContractedTasks.clone())?;
                if intEq(parentChild, nodeListHeadIdx) || intEq(parentChildContractionValue, negNodeListHeadIdx) {
                    if intNe(metamodelica::arrayGet(nodeMarks.clone(), parentChild)?, nodeIdx) {
                        parentNodeChildListNew = metamodelica::cons(nodeListHeadIdx, parentNodeChildListNew);
                        metamodelica::arrayUpdate(nodeMarks.clone(), parentChild, nodeIdx)?;
                    }
                } else {
                    parentNodeChildListNew = metamodelica::cons(parentChild, parentNodeChildListNew);
                }
            }
            tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeIdx, parentNodeChildListNew)?;
        }
        tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeListHeadIdx, outgoingEdges)?;
    }
    oTaskGraph = tmpTaskGraph.clone();
    oTaskGraphT = tmpTaskGraphT.clone();
    oTaskGraphMeta = iTaskGraphMeta;
    oContractedTasks = iContractedTasks.clone();
    Ok((oTaskGraph, oTaskGraphT, oTaskGraphMeta, oContractedTasks))
}

fn checkIfNodeBelongsToCluster(
    mut iNegativeRefValue: i32,
    mut iNodeIdx: i32,
    mut iContractedTasks: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut oIsNodePartOfCluster: bool;
    oIsNodePartOfCluster = intEq(
        iNegativeRefValue,
        metamodelica::arrayGet(iContractedTasks.clone(), iNodeIdx)?,
    );
    Ok(oIsNodePartOfCluster)
}

fn getContractedNodeChildren(
    mut iParentTask: i32,
    mut iRefValue: i32,
    mut iTaskGraph: TaskGraph,
    mut iContractedTasks: metamodelica::Array<i32>,
    mut iNodeMarks: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut oChildTasks: metamodelica::List<i32>;
    let mut task: i32 = 0;
    let mut taskMark: i32;
    let mut childTasks: metamodelica::List<i32>;
    let mut resultTasks: metamodelica::List<i32> = metamodelica::nil();
    childTasks = metamodelica::arrayGet(iTaskGraph.clone(), iParentTask)?;
    for mut task in &*childTasks {
        let mut task = task.clone();
        task = getRealTaskIdxOfTask(task, iContractedTasks.clone())?;
        taskMark = metamodelica::arrayGet(iNodeMarks.clone(), task)?;
        if boolAnd(intNe(taskMark, iRefValue), intNe(task, iRefValue)) {
            resultTasks = metamodelica::cons(task, resultTasks);
            metamodelica::arrayUpdate(iNodeMarks.clone(), task, iRefValue)?;
        }
    }
    oChildTasks = resultTasks;
    Ok(oChildTasks)
}

fn getRealTaskIdxOfTask(mut iTaskIdx: i32, mut iContractedTasks: metamodelica::Array<i32>) -> Result<i32> {
    '__tco: loop {
        let mut contractionMark: i32;
        contractionMark = metamodelica::arrayGet(iContractedTasks.clone(), iTaskIdx)?;
        if intLt(contractionMark, 0) {
            {
                (iTaskIdx, iContractedTasks) = (intMul(contractionMark, -1), iContractedTasks.clone());
                continue '__tco;
            }
        } else {
            return Ok(iTaskIdx);
        }
    }
}

pub(crate) fn setInCompsInMeta(
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut metaIn: TaskGraphMeta,
) -> TaskGraphMeta {
    let mut metaOut: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta {
        varCompMapping: __pa0,
        eqCompMapping: __pa1,
        compParamMapping: __pa2,
        compNames: __pa3,
        compDescs: __pa4,
        exeCosts: __pa5,
        commCosts: __pa6,
        nodeMark: __pa7,
        compInformations: __pa8,
        ..
    } = metaIn;
    varCompMapping = metamodelica::Own::own(__pa0);
    eqCompMapping = metamodelica::Own::own(__pa1);
    compParamMapping = metamodelica::Own::own(__pa2);
    compNames = metamodelica::Own::own(__pa3);
    compDescs = metamodelica::Own::own(__pa4);
    exeCosts = metamodelica::Own::own(__pa5);
    commCosts = metamodelica::Own::own(__pa6);
    nodeMark = metamodelica::Own::own(__pa7);
    compInformations = metamodelica::Own::own(__pa8);
    metaOut = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    metaOut
}

fn updateInCompsInfo(
    mut contrNode: i32,
    mut removedNodes: metamodelica::List<i32>,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut comps: metamodelica::List<i32>;
    let mut contrComps: metamodelica::List<i32>;
    comps = metamodelica::arrayGet(inComps.clone(), contrNode)?;
    contrComps = List::flatten(List::map(
        removedNodes,
        &({
            let __pe_b1 = inComps.clone();
            move |__pe_a0| Array::getIndexFirst(__pe_a0, __pe_b1.clone())
        }),
    )?)?;
    comps = List::unique(&(listAppend(contrComps, comps)));
    metamodelica::arrayUpdate(inComps.clone(), contrNode, comps)?;
    Ok(())
}

pub(crate) fn filterContractedNodes(
    mut nodesIn: metamodelica::List<i32>,
    mut contrNodes: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut nodesOut: metamodelica::List<i32>;
    nodesOut = List::filterOnFalse(
        nodesIn,
        &({
            let __pe_b1 = contrNodes.clone();
            move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(nodesOut)
}

pub(crate) fn filterNonContractedNodes(
    mut nodesIn: metamodelica::List<i32>,
    mut contrNodes: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut nodesOut: metamodelica::List<i32>;
    nodesOut = List::filterOnTrue(
        nodesIn,
        (std::sync::Arc::new({
            let __pe_b1 = contrNodes.clone();
            move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone())
        }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
    )?;
    Ok(nodesOut)
}

pub(crate) fn isNodeContracted(mut iNode: i32, mut iContrNodes: metamodelica::Array<i32>) -> Result<bool> {
    let mut oIsContracted: bool;
    if intLe(iNode, metamodelica::arrayLength(iContrNodes.clone())) {
        oIsContracted = intLt(metamodelica::arrayGet(iContrNodes.clone(), iNode)?, 0);
    } else {
        oIsContracted = false;
    }
    Ok(oIsContracted)
}

fn contractNodesInGraph1(mut contractNodes: metamodelica::List<i32>, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut graphInT: TaskGraph;
    let mut endNode: i32;
    let mut startNode: i32;
    let mut deleteEntries: metamodelica::List<i32>;
    let mut startNodeChildren: metamodelica::List<i32>;
    let mut endChildren: metamodelica::List<i32>;
    let mut deleteNodesParents: metamodelica::List<i32>;
    let mut graphTmp: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), metamodelica::arrayLength(graphIn.clone()))?;
    startNode = List::last(&contractNodes)?;
    (deleteEntries, _) = List::deleteMemberOnTrue(startNode, contractNodes.clone(), &fnptr!(intEq, i32, i32))?;
    deleteNodesParents = List::flatten(List::map1(
        deleteEntries.clone(),
        &Array::getIndexFirst,
        graphInT.clone(),
    )?)?;
    deleteNodesParents = List::sortedUnique(
        List::sort(
            deleteNodesParents,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        &fnptr!(intEq, i32, i32),
    )?;
    deleteNodesParents = List::setDifferenceOnTrue(deleteNodesParents, &contractNodes, &fnptr!(intEq, i32, i32))?;
    endNode = (contractNodes).head().cloned()?;
    endChildren = metamodelica::arrayGet(graphIn.clone(), endNode)?;
    startNodeChildren = metamodelica::arrayGet(graphIn.clone(), startNode)?;
    startNodeChildren = List::setDifferenceOnTrue(startNodeChildren, &deleteEntries, &fnptr!(intEq, i32, i32))?;
    graphTmp = metamodelica::arrayUpdate(graphIn.clone(), startNode, startNodeChildren)?;
    graphTmp = List::fold2(
        &deleteNodesParents,
        &move |__a0: i32,
               __a1: metamodelica::List<i32>,
               __a2: i32,
               __a3: metamodelica::Array<metamodelica::List<i32>>| {
            contractNodesInGraph2(__a0, &__a1, __a2, __a3)
        },
        deleteEntries,
        startNode,
        graphTmp.clone(),
    )?;
    graphTmp = metamodelica::arrayUpdate(graphIn.clone(), startNode, endChildren)?;
    graphOut = graphTmp.clone();
    Ok(graphOut)
}

fn contractNodesInGraph2(
    mut iParentNode: i32,
    mut iDeletedNodes: &metamodelica::List<i32>,
    mut iNewNodeIdx: i32,
    mut iGraph: TaskGraph,
) -> Result<TaskGraph> {
    let mut oGraph: TaskGraph;
    let mut adjLstEntry: metamodelica::List<i32>;
    adjLstEntry = metamodelica::arrayGet(iGraph.clone(), iParentNode)?;
    adjLstEntry = List::setDifferenceOnTrue(adjLstEntry, iDeletedNodes, &fnptr!(intEq, i32, i32))?;
    adjLstEntry = metamodelica::cons(iNewNodeIdx, adjLstEntry);
    adjLstEntry = List::sortedUnique(
        List::sort(
            adjLstEntry,
            (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?,
        &fnptr!(intEq, i32, i32),
    )?;
    oGraph = metamodelica::arrayUpdate(iGraph.clone(), iParentNode, adjLstEntry)?;
    Ok(oGraph)
}

fn compareListLengthOnTrue(mut inValue: i32, mut inLst: &metamodelica::List<i32>) -> bool {
    let mut equalLength: bool;
    equalLength = intEq(inValue, ((inLst).len() as i32));
    equalLength
}

fn getMergedSystemData(
    mut graphDataIn: TaskGraphMeta,
    mut contractNodes: metamodelica::List<metamodelica::List<i32>>,
) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = graphDataIn;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    exeCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeMark = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    inComps = updateInCompsForMerging(inComps.clone(), contractNodes)?;
    compNames = List::fold2(
        &(List::intRange(metamodelica::arrayLength(compNames.clone()))),
        &updateCompNamesForMerging,
        inComps.clone(),
        nodeMark.clone(),
        compNames.clone(),
    )?;
    graphDataOut = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok(graphDataOut)
}

fn updateCompNamesForMerging(
    mut compIdx: i32,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut nodeMark: metamodelica::Array<i32>,
    mut compNamesIn: metamodelica::Array<ArcStr>,
) -> Result<metamodelica::Array<ArcStr>> {
    let mut compNamesOut: metamodelica::Array<ArcStr>;
    compNamesOut = 'mc: {
        let __mc_input = compNamesIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut unionNode: i32;
            let mut mergedComps: metamodelica::List<i32>;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else {
                return Err("pattern mismatch");
            };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode != -1) else {
                return Err("pattern mismatch");
            };
            mergedComps = metamodelica::arrayGet(inComps.clone(), unionNode)?;
            let true = (((mergedComps).len() as i32) == 1) else {
                return Err("pattern mismatch");
            };
            Ok(compNamesIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut unionNode: i32;
            let mut mergedComps: metamodelica::List<i32>;
            let mut compNamesTmp: metamodelica::Array<ArcStr>;
            let mut compName: ArcStr;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else {
                return Err("pattern mismatch");
            };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode != -1) else {
                return Err("pattern mismatch");
            };
            mergedComps = metamodelica::arrayGet(inComps.clone(), unionNode)?;
            let false = (((mergedComps).len() as i32) == 1) else {
                return Err("pattern mismatch");
            };
            compName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("contracted comps "));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(mergedComps.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                ArcStr::from(__mm_s)
            };
            compNamesTmp = metamodelica::arrayUpdate(compNamesIn.clone(), compIdx, compName.clone())?;
            Ok(compNamesTmp.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut unionNode: i32;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else {
                return Err("pattern mismatch");
            };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode == -1) else {
                return Err("pattern mismatch");
            };
            Ok(compNamesIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("updateCompNamesForMerging failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(compNamesOut)
}

fn updateInCompsForMerging(
    mut inCompsIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mergedPaths: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut inCompsOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut inCompsLst: metamodelica::List<metamodelica::List<i32>>;
    let mut deleteNodes: metamodelica::List<i32>;
    let mut startNodes: metamodelica::List<i32>;
    startNodes = List::map(mergedPaths.clone(), &move |__a0: _| List::last(&__a0))?;
    (_, deleteNodes, _) = List::intersection1OnTrue(
        List::flatten(mergedPaths.clone())?,
        startNodes.clone(),
        &fnptr!(intEq, i32, i32),
    )?;
    inCompsLst = inCompsIn
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    inCompsLst = List::fold2(
        &(List::intRange(metamodelica::arrayLength(inCompsIn.clone()))),
        &move |__a0: i32,
               __a1: (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<metamodelica::List<i32>>,
        ),
               __a2: metamodelica::Array<metamodelica::List<i32>>,
               __a3: metamodelica::List<metamodelica::List<i32>>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(updateInComps1(__a0, &__a1, __a2, __a3))
        },
        (startNodes, deleteNodes, mergedPaths),
        inCompsIn.clone(),
        inCompsLst,
    )?;
    inCompsLst = List::removeOnTrue(
        metamodelica::nil(),
        &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<i32>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(equalLists(&__a0, &__a1))
        },
        inCompsLst,
    )?;
    inCompsOut = metamodelica::arrayFromVec(inCompsLst.into_iter().cloned().collect());
    Ok(inCompsOut)
}

fn updateInComps1(
    mut nodeIdx: i32,
    mut mergeInfo: &(
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<metamodelica::List<i32>>,
    ),
    mut primInComps: metamodelica::Array<metamodelica::List<i32>>,
    mut inCompLstIn: metamodelica::List<metamodelica::List<i32>>,
) -> metamodelica::List<metamodelica::List<i32>> {
    let mut inCompLstOut: metamodelica::List<metamodelica::List<i32>>;
    inCompLstOut = 'mc: {
        let __mc_input = &*inCompLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut mergeGroupIdx: i32;
                    let mut inComps: metamodelica::List<i32>;
                    let mut mergedSet: metamodelica::List<i32>;
                    let mut mergedNodes: metamodelica::List<i32>;
                    let mut startNodes: metamodelica::List<i32>;
                    let mut mergedPaths: metamodelica::List<metamodelica::List<i32>>;
                    let mut inCompLstTmp: metamodelica::List<metamodelica::List<i32>>;
                    (startNodes, _, mergedPaths) = mergeInfo.clone();
                    inComps = (inCompLstIn).get(nodeIdx)?;
                    (inComps).get(1)?;
                    let true = (List::isMemberOnTrue(nodeIdx, &startNodes, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    mergeGroupIdx = List::position(nodeIdx, &startNodes)?;
                    mergedNodes = (mergedPaths).get(mergeGroupIdx)?;
                    mergedSet = List::flatten(List::map1(mergedNodes.clone(), &Array::getIndexFirst, primInComps.clone())?)?;
                    inCompLstTmp = List::fold(&mergedNodes, &updateInComps2, inCompLstIn.clone())?;
                    inCompLstTmp = List::replaceAt(mergedSet.clone(), nodeIdx, inCompLstTmp.clone())?;
                    Ok(inCompLstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCompLstIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    inCompLstOut
}

fn updateInComps2(
    mut iNodeIdx: i32,
    mut inCompLstIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut inCompLstOut: metamodelica::List<metamodelica::List<i32>>;
    inCompLstOut = List::replaceAt(metamodelica::nil(), iNodeIdx, inCompLstIn)?;
    Ok(inCompLstOut)
}

pub(crate) fn equalLists<'__b>(
    mut inList1: &'__b metamodelica::List<i32>,
    mut inList2: &'__b metamodelica::List<i32>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (inList1, inList2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return true
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return false
            },
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return false
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: rest2 }) if (intEq(e1.clone(), e2.clone())) => {
                { (inList1, inList2) = (rest1, rest2); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn findOneChildParents(
    mut allNodes: &metamodelica::List<i32>,
    mut graphIn: TaskGraph,
    mut doNotMerge: &metamodelica::List<i32>,
    mut lstIn: &metamodelica::List<metamodelica::List<i32>>,
    mut inPath: i32,
    mut contrNodes: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut lstOut: metamodelica::List<metamodelica::List<i32>>;
    lstOut = 'mc: {
        let __mc_input = &**allNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(lstIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut nodeChildren: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let true = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let false = (((nodeChildren).len() as i32) == 1) else { return Err("pattern mismatch") };
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, lstIn, 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let true = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    let true = (listMember(head.clone(), doNotMerge.clone())) else { return Err("pattern mismatch") };
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, lstIn, 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let true = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let true = (((nodeChildren).len() as i32) == 1) else { return Err("pattern mismatch") };
                    child = (nodeChildren).get(1)?;
                    let true = (listMember(child, doNotMerge.clone())) else { return Err("pattern mismatch") };
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, lstIn, child, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let true = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let true = (((nodeChildren).len() as i32) == 1) else { return Err("pattern mismatch") };
                    child = (nodeChildren).get(1)?;
                    lstTmp = metamodelica::cons(list![head.clone()], lstIn.clone());
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, &lstTmp, child, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let false = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    let true = (listMember(inPath, doNotMerge.clone())) else { return Err("pattern mismatch") };
                    lstTmp = findOneChildParents(allNodes, graphIn.clone(), doNotMerge, lstIn, 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: metamodelica::List<i32>;
                    let mut parents: metamodelica::List<i32>;
                    let mut pathLst: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), inPath)?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    parents = getParentNodes(inPath, graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone())?;
                    let true = (((nodeChildren).len() as i32) == 1 && !((nodeChildren).is_empty()) && ((parents).len() as i32) == 1) else { return Err("pattern mismatch") };
                    child = (nodeChildren).get(1)?;
                    pathLst = (lstIn).head().cloned()?;
                    pathLst = metamodelica::cons(inPath, pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath, allNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, &lstTmp, child, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut nodeChildren: metamodelica::List<i32>;
                    let mut parents: metamodelica::List<i32>;
                    let mut pathLst: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath, 0)) else { return Err("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), inPath)?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    parents = getParentNodes(inPath, graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone())?;
                    pathLst = (lstIn).head().cloned()?;
                    pathLst = metamodelica::cons(inPath, pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath, allNodes.clone(), &fnptr!(intEq, i32, i32))?;
                    lstTmp = findOneChildParents(metamodelica::AsArg::as_arg(&rest), graphIn.clone(), doNotMerge, &lstTmp, 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("findOneChildParents failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(lstOut)
}

fn getParentNodes(mut nodeIdx: i32, mut graphIn: TaskGraph) -> Result<metamodelica::List<i32>> {
    let mut parentNodes: metamodelica::List<i32>;
    let mut graphInT: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), metamodelica::arrayLength(graphIn.clone()))?;
    parentNodes = metamodelica::arrayGet(graphInT.clone(), nodeIdx)?;
    Ok(parentNodes)
}

fn checkParentNode(
    mut lstIdx: i32,
    mut graphIn: TaskGraph,
    mut lstIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut lstOut: metamodelica::List<metamodelica::List<i32>>;
    lstOut = 'mc: {
        let __mc_input = &*lstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut childLst: metamodelica::List<i32>;
                    let mut child: i32;
                    let mut parent: i32;
                    let mut parents: metamodelica::List<i32>;
                    let mut lstTmp: metamodelica::List<metamodelica::List<i32>>;
                    childLst = (lstIn).get(lstIdx)?;
                    child = List::last(&childLst)?;
                    parents = getParentNodes(child, graphIn.clone())?;
                    let true = (intEq(((parents).len() as i32), 1)) else { return Err("pattern mismatch") };
                    parent = (parents).get(1)?;
                    childLst = childLst.clone().reverse();
                    childLst = metamodelica::cons(parent, childLst.clone());
                    childLst = childLst.clone().reverse();
                    lstTmp = List::replaceAt(childLst.clone(), lstIdx, lstIn.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut childLst: metamodelica::List<i32>;
                    let mut child: i32;
                    let mut parents: metamodelica::List<i32>;
                    childLst = (lstIn).get(lstIdx)?;
                    child = List::last(&childLst)?;
                    parents = getParentNodes(child, graphIn.clone())?;
                    let false = (intEq(((parents).len() as i32), 1)) else { return Err("pattern mismatch") };
                    Ok(lstIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(lstOut)
}

//-----------------------------
//  Functions to generate costs
//-----------------------------
pub(crate) fn createCosts(
    mut iDae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iBenchFilePrefix: &ArcStr,
    mut iSimEqCompMapping: metamodelica::Array<i32>,
    mut iTaskGraphMeta: TaskGraphMeta,
) -> Result<TaskGraphMeta> {
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut compMapping: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>> = Default::default();
    let mut compMapping_withIdx: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)> =
        Default::default();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
    let mut reqTimeCom: (i32, i32) = (0, 0);
    let mut reqTimeOpLstSimCode: metamodelica::List<(i32, i32, metamodelica::Real)> = metamodelica::nil();
    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut tmpTaskGraphMeta: TaskGraphMeta = <TaskGraphMeta as ::std::default::Default>::default();
    let mut reqTimeOp: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    oTaskGraphMeta = 'mc: {
        let __mc_input = (&**iDae, &iTaskGraphMeta);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { shared, .. }, TaskGraphMeta { inComps, commCosts, .. }) => {
                    let mut commCosts = (*commCosts).clone();
                    let mut compMapping: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>> = compMapping.clone();
                    let mut compMapping_withIdx: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)> = compMapping_withIdx.clone();
                    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = comps.clone();
                    let mut reqTimeCom: (i32, i32) = reqTimeCom.clone();
                    let mut reqTimeOp: metamodelica::Array<metamodelica::Real> = reqTimeOp.clone();
                    let mut reqTimeOpLstSimCode: metamodelica::List<(i32, i32, metamodelica::Real)> = reqTimeOpLstSimCode.clone();
                    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)> = reqTimeOpSimCode.clone();
                    let mut tmpTaskGraphMeta: TaskGraphMeta = tmpTaskGraphMeta.clone();
                    (comps, compMapping_withIdx) = getSystemComponents(iDae)?;
                    compMapping = Array::map(compMapping_withIdx.clone(), &fnptr!(Util::tuple21, _))?;
                    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
                    reqTimeOpLstSimCode = HpcOmBenchmark::readCalcTimesFromFile(iBenchFilePrefix)?;
                    reqTimeOpSimCode = arrayCreate(((reqTimeOpLstSimCode).len() as i32), (-1, metamodelica::OrderedFloat(-1.0_f64)));
                    reqTimeOpSimCode = List::fold(&reqTimeOpLstSimCode, &createCosts1, reqTimeOpSimCode.clone())?;
                    reqTimeOp = arrayCreate(((comps).len() as i32), metamodelica::OrderedFloat(-1.0_f64));
                    reqTimeOp = convertSimEqToSccCosts(reqTimeOpSimCode.clone(), iSimEqCompMapping.clone(), reqTimeOp.clone())?;
                    commCosts = createCommCosts(commCosts.clone(), 1, reqTimeCom);
                    (_, tmpTaskGraphMeta) = Array::fold(inComps.clone(), &({ let __pe_b1 = (comps.clone(), shared.clone()); let __pe_b2 = compMapping.clone(); let __pe_b3 = reqTimeOp.clone(); let __pe_b4 = reqTimeCom; move |__pe_a0, __pe_a5| Ok(createCosts0(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), &__pe_a5)) }), (1, iTaskGraphMeta.clone()))?;
                    Ok((tmpTaskGraphMeta.clone(), compMapping.clone(), compMapping_withIdx.clone(), comps.clone(), reqTimeCom.clone(), reqTimeOp.clone(), reqTimeOpLstSimCode.clone(), reqTimeOpSimCode.clone(), tmpTaskGraphMeta.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            compMapping = __wb0;
            compMapping_withIdx = __wb1;
            comps = __wb2;
            reqTimeCom = __wb3;
            reqTimeOp = __wb4;
            reqTimeOpLstSimCode = __wb5;
            reqTimeOpSimCode = __wb6;
            tmpTaskGraphMeta = __wb7;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpTaskGraphMeta: TaskGraphMeta = tmpTaskGraphMeta.clone();
                    tmpTaskGraphMeta = estimateCosts(iDae, iTaskGraphMeta.clone())?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: The costs have been estimated. Maybe ")); __mm_s.push_str(&*iBenchFilePrefix); __mm_s.push_str(&*literal!("-file is missing.\n")); ArcStr::from(__mm_s) });
                    Ok((tmpTaskGraphMeta.clone(), tmpTaskGraphMeta.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpTaskGraphMeta = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oTaskGraphMeta)
}

fn estimateCosts(
    mut daeIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut taskGraphMetaIn: TaskGraphMeta,
) -> Result<TaskGraphMeta> {
    let mut taskGraphMetaOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut comNumLst: metamodelica::List<i32>;
    let mut exeCostsLst: metamodelica::List<(i32, metamodelica::Real)>;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut compsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compIdx: i32;
    let __arc2 = &(*daeIn);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    eqSystems = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    compsLst = List::map(eqSystems.clone(), &move |__a0: metamodelica::Ref<
        BackendDAE::EqSystem,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(BackendDAEUtil::getStrongComponents(&__a0))
    })?;
    comNumLst = List::map(compsLst.clone(), &fnptr!(listLength, _))?;
    let TaskGraphMeta {
        inComps: __pa3,
        varCompMapping: __pa4,
        eqCompMapping: __pa5,
        compParamMapping: __pa6,
        compNames: __pa7,
        compDescs: __pa8,
        exeCosts: __pa9,
        commCosts: __pa10,
        nodeMark: __pa11,
        compInformations: __pa12,
    } = taskGraphMetaIn;
    inComps = metamodelica::Own::own(__pa3);
    varCompMapping = metamodelica::Own::own(__pa4);
    eqCompMapping = metamodelica::Own::own(__pa5);
    compParamMapping = metamodelica::Own::own(__pa6);
    compNames = metamodelica::Own::own(__pa7);
    compDescs = metamodelica::Own::own(__pa8);
    exeCosts = metamodelica::Own::own(__pa9);
    commCosts = metamodelica::Own::own(__pa10);
    nodeMark = metamodelica::Own::own(__pa11);
    compInformations = metamodelica::Own::own(__pa12);
    commCosts = getCommCostsOnly(commCosts.clone())?;
    exeCostsLst = List::flatten(List::map3(
        List::intRange(((compsLst).len() as i32)),
        &move |__a0: i32,
               __a1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>>,
               __a2: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
               __a3: metamodelica::Ref<BackendDAE::Shared>| estimateCosts0(__a0, &__a1, &__a2, &__a3),
        compsLst,
        eqSystems,
        shared,
    )?)?;
    compIdx = 1;
    for mut exeCost in &*exeCostsLst {
        metamodelica::arrayUpdate(exeCosts.clone(), compIdx, exeCost.clone())?;
        compIdx = compIdx + 1;
    }
    taskGraphMetaOut = TaskGraphMeta {
        inComps: inComps.clone(),
        varCompMapping: varCompMapping.clone(),
        eqCompMapping: eqCompMapping.clone(),
        compParamMapping: compParamMapping.clone(),
        compNames: compNames.clone(),
        compDescs: compDescs.clone(),
        exeCosts: exeCosts.clone(),
        commCosts: commCosts.clone(),
        nodeMark: nodeMark.clone(),
        compInformations: compInformations.clone(),
    };
    Ok(taskGraphMetaOut)
}

fn estimateCosts0(
    mut systIdx: i32,
    mut compsLstIn: &metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>>,
    mut eqSystemsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut sharedIn: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::List<(i32, metamodelica::Real)>> {
    let mut exeCostsOut: metamodelica::List<(i32, metamodelica::Real)>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut compsInfos: metamodelica::List<metamodelica::Ref<BackendDAE::CompInfo>>;
    comps = (compsLstIn).get(systIdx)?;
    eqSys = (eqSystemsIn).get(systIdx)?;
    compsInfos =
        BackendDAEOptimize::countOperationstraverseComps(&comps, &eqSys, sharedIn, &(metamodelica::nil()))?.reverse();
    exeCostsOut = List::map(
        compsInfos,
        &move |__a0: metamodelica::Ref<BackendDAE::CompInfo>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(calculateCosts(&__a0))
        },
    )?;
    Ok(exeCostsOut)
}

pub(crate) fn calculateCosts(mut compInfo: &metamodelica::Ref<BackendDAE::CompInfo>) -> (i32, metamodelica::Real) {
    let mut exeCost: (i32, metamodelica::Real);
    exeCost = (match &**compInfo {
        BackendDAE::CompInfo::COUNTER {
            comp,
            numAdds,
            numMul,
            numDiv,
            numTrig,
            numRelations: numRel,
            numLog,
            numOth,
            funcCalls: numFuncs,
        } => {
            let mut costs: i32;
            let mut ops: i32;
            let mut offset: i32;
            ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
            if BackendDAEUtil::isSingleEquationComp(comp) {
                offset = 35;
            } else if BackendDAEUtil::isWhenComp(comp) {
                offset = 113;
            } else if BackendDAEUtil::isArrayComp(comp) {
                offset = 100;
            } else {
                offset = 0;
            }
            costs = offset
                + 12 * numAdds.clone()
                + 32 * numMul.clone()
                + 37 * numDiv.clone()
                + 236 * numTrig.clone()
                + 2 * numRel.clone()
                + 4 * numLog.clone()
                + 110 * numOth.clone()
                + 375 * numFuncs.clone();
            (ops, intReal(costs))
        }
        BackendDAE::CompInfo::SYSTEM {
            size, density: dens, ..
        } => {
            let mut allOpCosts: metamodelica::Real;
            allOpCosts = (metamodelica::OrderedFloat(0.049_f64))
                * (realPow(
                    (intReal(size.clone()))
                        * ((metamodelica::OrderedFloat(1.0_f64))
                            + ((dens.clone()) * (metamodelica::OrderedFloat(19.0_f64)))),
                    metamodelica::OrderedFloat(3.0_f64),
                ));
            (1, allOpCosts)
        }
        BackendDAE::CompInfo::TORN_ANALYSE {
            tornEqs: torn,
            otherEqs: other,
            tornSize: size,
            ..
        } => {
            let mut ops: i32;
            let mut ops1: i32;
            let mut allOpCosts: metamodelica::Real;
            let mut tornCosts: metamodelica::Real;
            let mut otherCosts: metamodelica::Real;
            (ops, tornCosts) = calculateCosts(torn);
            (ops1, otherCosts) = calculateCosts(other);
            allOpCosts = ((metamodelica::OrderedFloat(3000.0_f64))
                + ((metamodelica::OrderedFloat(7.62_f64))
                    * (realPow(intReal(size.clone()), metamodelica::OrderedFloat(3.0_f64)))))
                + (((metamodelica::OrderedFloat(2.0_f64)) * (tornCosts))
                    + ((metamodelica::OrderedFloat(1.4_f64)) * (otherCosts)));
            (ops + ops1, allOpCosts)
        }
        BackendDAE::CompInfo::NO_COMP {
            numAdds,
            numMul,
            numDiv,
            numTrig,
            numRelations: numRel,
            numLog,
            numOth,
            funcCalls: numFuncs,
        } => {
            let mut costs: i32;
            let mut ops: i32;
            let mut offset: i32;
            ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
            offset = 50;
            costs = offset
                + 12 * numAdds.clone()
                + 32 * numMul.clone()
                + 37 * numDiv.clone()
                + 236 * numTrig.clone()
                + 2 * numRel.clone()
                + 4 * numLog.clone()
                + 110 * numOth.clone()
                + 375 * numFuncs.clone();
            (ops, intReal(costs))
        }
        _ => {
            metamodelica::print(literal!("calculate costs failed!\n"));
            (-1, metamodelica::OrderedFloat(-1.0_f64))
        }
    });
    exeCost
}

pub(crate) fn copyCosts(
    mut iSourceTaskGraphData: TaskGraphMeta,
    mut iTargetTaskGraphData: TaskGraphMeta,
) -> Result<TaskGraphMeta> {
    let mut oTaskGraphData: TaskGraphMeta;
    let mut inCompsSource: metamodelica::Array<metamodelica::List<i32>>;
    let mut inCompsTarget: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCostsSource: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCostsTarget: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compIdx: i32;
    let mut commCostsTarget: metamodelica::Array<metamodelica::List<Communication>>;
    let mut reqTimeCom: (i32, i32);
    let TaskGraphMeta {
        inComps: __pa0,
        exeCosts: __pa1,
        ..
    } = iSourceTaskGraphData;
    inCompsSource = metamodelica::Own::own(__pa0);
    exeCostsSource = metamodelica::Own::own(__pa1);
    let TaskGraphMeta {
        inComps: __pa2,
        exeCosts: __pa3,
        commCosts: __pa4,
        ..
    } = &iTargetTaskGraphData;
    inCompsTarget = metamodelica::Own::own(__pa2);
    exeCostsTarget = metamodelica::Own::own(__pa3);
    commCostsTarget = metamodelica::Own::own(__pa4);
    compIdx = intMin(
        metamodelica::arrayLength(exeCostsSource.clone()),
        metamodelica::arrayLength(exeCostsTarget.clone()),
    );
    while intGt(compIdx, 0) {
        exeCostsTarget = metamodelica::arrayUpdate(
            exeCostsTarget.clone(),
            compIdx,
            metamodelica::arrayGet(exeCostsSource.clone(), compIdx)?,
        )?;
        compIdx = compIdx - 1;
    }
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsTarget = createCommCosts(commCostsTarget.clone(), 1, reqTimeCom);
    oTaskGraphData = iTargetTaskGraphData;
    Ok(oTaskGraphData)
}

fn getCommCostsOnly(
    mut commCostsIn: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Array<metamodelica::List<Communication>>> {
    let mut commCostsOut: metamodelica::Array<metamodelica::List<Communication>>;
    let mut reqTimeCom: (i32, i32);
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsOut = createCommCosts(commCostsIn.clone(), 1, reqTimeCom);
    Ok(commCostsOut)
}

fn checkForExecutionCosts(mut dataIn: TaskGraphMeta) -> bool {
    let mut isFine: bool;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta {
        inComps: __pa0,
        exeCosts: __pa1,
        ..
    } = dataIn;
    inComps = metamodelica::Own::own(__pa0);
    exeCosts = metamodelica::Own::own(__pa1);
    isFine = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), 1);
    if !(isFine) {
        metamodelica::print(literal!("There are execution costs with value 0.0!\n"));
    }
    isFine
}

fn checkForExecutionCosts1(
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut nodeIdx: i32,
) -> bool {
    let mut bOut: bool;
    bOut = 'mc: {
        let __mc_input = nodeIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut b: bool;
            let mut isZero: bool;
            let mut comps: metamodelica::List<i32>;
            let true = (metamodelica::arrayLength(inComps.clone()) >= nodeIdx) else {
                return Err("pattern mismatch");
            };
            comps = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            isZero = List::fold1(&comps, &checkTpl2ForZero, exeCosts.clone(), false)?;
            let false = (isZero) else {
                return Err("pattern mismatch");
            };
            b = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), nodeIdx + 1);
            Ok(b)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (metamodelica::arrayLength(inComps.clone()) < nodeIdx) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    bOut
}

fn checkTpl2ForZero(
    mut comp: i32,
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut bIn: bool,
) -> Result<bool> {
    let mut bOut: bool;
    let mut b: bool;
    let mut value: metamodelica::Real;
    let mut tpl: (i32, metamodelica::Real);
    tpl = metamodelica::arrayGet(exeCosts.clone(), comp)?;
    (_, value) = tpl;
    b = realEq(value, metamodelica::OrderedFloat(0.0_f64));
    bOut = b || bIn;
    Ok(bOut)
}

pub(crate) fn convertNodeListToEdgeTuples(mut iNodeList: &metamodelica::List<i32>) -> metamodelica::List<(i32, i32)> {
    let mut oEdgeList: metamodelica::List<(i32, i32)>;
    oEdgeList = convertNodeListToEdgeTuples0(iNodeList, ((iNodeList).len() as i32), metamodelica::nil());
    oEdgeList
}

fn convertNodeListToEdgeTuples0(
    mut iNodeList: &metamodelica::List<i32>,
    mut iNodeIdx: i32,
    mut iEdgeList: metamodelica::List<(i32, i32)>,
) -> metamodelica::List<(i32, i32)> {
    let mut oEdgeList: metamodelica::List<(i32, i32)>;
    let mut tmpEdgeList: metamodelica::List<(i32, i32)>;
    let mut elem: i32 = 0;
    let mut preElem: i32 = 0;
    oEdgeList = 'mc: {
        let __mc_input = iEdgeList.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tmpEdgeList => {
                    let mut tmpEdgeList = (*tmpEdgeList).clone();
                    let mut elem: i32 = elem.clone();
                    let mut preElem: i32 = preElem.clone();
                    let true = (intGt(iNodeIdx, 1)) else { return Err("pattern mismatch") };
                    elem = (iNodeList).get(iNodeIdx)?;
                    preElem = (iNodeList).get(iNodeIdx - 1)?;
                    tmpEdgeList = metamodelica::cons((preElem, elem), tmpEdgeList.clone());
                    tmpEdgeList = convertNodeListToEdgeTuples0(iNodeList, iNodeIdx - 1, tmpEdgeList.clone());
                    Ok((tmpEdgeList.clone(), elem.clone(), preElem.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            elem = __wb0;
            preElem = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iEdgeList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oEdgeList
}

fn convertSimEqToSccCosts(
    mut iReqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)>,
    mut iSimeqCompMapping: metamodelica::Array<i32>,
    mut iReqTimeOp: metamodelica::Array<metamodelica::Real>,
) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oReqTimeOp: metamodelica::Array<metamodelica::Real>;
    (_, oReqTimeOp) = Array::fold(
        iReqTimeOpSimCode.clone(),
        &({
            let __pe_b1 = iSimeqCompMapping.clone();
            move |__pe_a0, __pe_a2| Ok(convertSimEqToSccCosts1(__pe_a0, __pe_b1.clone(), __pe_a2))
        }),
        (1, iReqTimeOp.clone()),
    )?;
    Ok(oReqTimeOp)
}

fn convertSimEqToSccCosts1(
    mut iReqTimeOpSimCode: (i32, metamodelica::Real),
    mut iSimeqCompMapping: metamodelica::Array<i32>,
    mut iReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>),
) -> (i32, metamodelica::Array<metamodelica::Real>) {
    let mut oReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>);
    let mut simEqCalcCount: i32;
    let mut simEqIdx: i32;
    let mut simEqCalcTime: metamodelica::Real;
    let mut realSimEqCalcCount: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut reqTime: metamodelica::Array<metamodelica::Real>;
    oReqTimeOp = 'mc: {
        let __mc_input = (iReqTimeOpSimCode, iReqTimeOp);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), (mut simEqIdx, mut reqTime)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount);
            let true = (realNe(realSimEqCalcCount, metamodelica::OrderedFloat(0.0_f64))) else {
                return Err("pattern mismatch");
            };
            reqTime = convertSimEqToSccCosts2(
                reqTime.clone(),
                realDiv(simEqCalcTime, realSimEqCalcCount),
                simEqIdx,
                iSimeqCompMapping.clone(),
            );
            Ok(((simEqIdx + 1, reqTime.clone()), realSimEqCalcCount.clone()))
        })() {
            realSimEqCalcCount = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), (mut simEqIdx, mut reqTime)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount);
            reqTime = convertSimEqToSccCosts2(
                reqTime.clone(),
                metamodelica::OrderedFloat(0.0_f64),
                simEqIdx,
                iSimeqCompMapping.clone(),
            );
            Ok(((simEqIdx + 1, reqTime.clone()), realSimEqCalcCount.clone()))
        })() {
            realSimEqCalcCount = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("convertSimEqToSccCosts1 failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oReqTimeOp
}

fn convertSimEqToSccCosts2(
    mut iReqTime: metamodelica::Array<metamodelica::Real>,
    mut iSimEqCalcTime: metamodelica::Real,
    mut iSimEqIdx: i32,
    mut iSimeqCompMapping: metamodelica::Array<i32>,
) -> metamodelica::Array<metamodelica::Real> {
    let mut oReqTime: metamodelica::Array<metamodelica::Real>;
    let mut reqTime: metamodelica::Array<metamodelica::Real>;
    let mut sccIdx: i32 = 0;
    oReqTime = 'mc: {
        let __mc_input = iReqTime.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut reqTime = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut sccIdx: i32 = sccIdx.clone();
            let true = (intGe(metamodelica::arrayLength(iSimeqCompMapping.clone()), iSimEqIdx)) else {
                return Err("pattern mismatch");
            };
            sccIdx = metamodelica::arrayGet(iSimeqCompMapping.clone(), iSimEqIdx)?;
            let true = (intGt(sccIdx, 0)) else {
                return Err("pattern mismatch");
            };
            reqTime = metamodelica::arrayUpdate(reqTime.clone(), sccIdx, iSimEqCalcTime)?;
            Ok((reqTime.clone(), sccIdx.clone()))
        })() {
            sccIdx = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iReqTime.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oReqTime
}

fn createCosts0(
    mut iNode: &metamodelica::List<i32>,
    mut iComps_shared: (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
    mut iCompMapping: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut reqTimeOp: metamodelica::Array<metamodelica::Real>,
    mut reqTimeCom: (i32, i32),
    mut iTaskGraphMeta: &(i32, TaskGraphMeta),
) -> (i32, TaskGraphMeta) {
    let mut oTaskGraphMeta: (i32, TaskGraphMeta);
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeRefCount: metamodelica::Array<i32>;
    let mut execCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut nodeNumber: i32;
    let mut taskGraphMeta: TaskGraphMeta;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (nodeNumber, taskGraphMeta) = iTaskGraphMeta.clone();
    let TaskGraphMeta {
        inComps: __pa0,
        varCompMapping: __pa1,
        eqCompMapping: __pa2,
        compParamMapping: __pa3,
        compNames: __pa4,
        compDescs: __pa5,
        exeCosts: __pa6,
        commCosts: __pa7,
        nodeMark: __pa8,
        compInformations: __pa9,
    } = taskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    varCompMapping = metamodelica::Own::own(__pa1);
    eqCompMapping = metamodelica::Own::own(__pa2);
    compParamMapping = metamodelica::Own::own(__pa3);
    compNames = metamodelica::Own::own(__pa4);
    compDescs = metamodelica::Own::own(__pa5);
    execCosts = metamodelica::Own::own(__pa6);
    commCosts = metamodelica::Own::own(__pa7);
    nodeRefCount = metamodelica::Own::own(__pa8);
    compInformations = metamodelica::Own::own(__pa9);
    createExecCost(
        iNode,
        iComps_shared,
        reqTimeOp.clone(),
        execCosts.clone(),
        iCompMapping.clone(),
        nodeNumber,
    );
    oTaskGraphMeta = (
        nodeNumber + 1,
        TaskGraphMeta {
            inComps: inComps.clone(),
            varCompMapping: varCompMapping.clone(),
            eqCompMapping: eqCompMapping.clone(),
            compParamMapping: compParamMapping.clone(),
            compNames: compNames.clone(),
            compDescs: compDescs.clone(),
            exeCosts: execCosts.clone(),
            commCosts: commCosts.clone(),
            nodeMark: nodeRefCount.clone(),
            compInformations: compInformations.clone(),
        },
    );
    oTaskGraphMeta
}

fn createCosts1(
    mut iTuple: (i32, i32, metamodelica::Real),
    mut iReqTime: metamodelica::Array<(i32, metamodelica::Real)>,
) -> Result<metamodelica::Array<(i32, metamodelica::Real)>> {
    let mut oReqTime: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut tmpArray: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut simEqIdx: i32;
    let mut calcTimeCount: i32;
    let mut calcTime: metamodelica::Real;
    oReqTime = (match (iTuple, iReqTime.clone()) {
        ((0, mut __esc_calcTimeCount, mut __esc_calcTime), _) => {
            calcTimeCount = __esc_calcTimeCount.clone();
            calcTime = __esc_calcTime.clone();
            iReqTime.clone()
        }
        ((mut __esc_simEqIdx, mut __esc_calcTimeCount, mut __esc_calcTime), mut __esc_tmpArray) => {
            simEqIdx = __esc_simEqIdx.clone();
            calcTimeCount = __esc_calcTimeCount.clone();
            calcTime = __esc_calcTime.clone();
            tmpArray = __esc_tmpArray.clone();
            tmpArray = metamodelica::arrayUpdate(iReqTime.clone(), simEqIdx, (calcTimeCount, calcTime))?;
            tmpArray.clone()
        }
    });
    Ok(oReqTime)
}

fn createExecCost(
    mut iNodeSccs: &metamodelica::List<i32>,
    mut icomps_shared: (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
    mut iRequiredTime: metamodelica::Array<metamodelica::Real>,
    mut iExecCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut compMapping: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iNodeIdx: i32,
) -> () {
    let () = 'mc: {
        let __mc_input = iNodeIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut execCost: (i32, metamodelica::Real);
            execCost = List::fold3(
                iNodeSccs,
                &move |__a0: i32,
                       __a1: (
                    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
                    metamodelica::Ref<BackendDAE::Shared>,
                ),
                       __a2: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>>,
                       __a3: metamodelica::Array<metamodelica::Real>,
                       __a4: (i32, metamodelica::Real)| createExecCost0(__a0, &__a1, __a2, __a3, __a4),
                icomps_shared.clone(),
                compMapping.clone(),
                iRequiredTime.clone(),
                (0, metamodelica::OrderedFloat(0.0_f64)),
            )?;
            metamodelica::arrayUpdate(iExecCosts.clone(), iNodeIdx, execCost)?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn createExecCost0(
    mut sccIndex: i32,
    mut icomps_shared: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
    mut compMapping: metamodelica::Array<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iRequiredTime: metamodelica::Array<metamodelica::Real>,
    mut iCosts: (i32, metamodelica::Real),
) -> Result<(i32, metamodelica::Real)> {
    let mut oCosts: (i32, metamodelica::Real);
    let mut iCosts_op: i32;
    let mut iCosts_cyc: metamodelica::Real;
    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut reqTime: metamodelica::Real;
    (comps, shared) = icomps_shared.clone();
    (iCosts_op, iCosts_cyc) = iCosts;
    comp = (comps).get(sccIndex)?;
    syst = metamodelica::arrayGet(compMapping.clone(), sccIndex)?;
    reqTime = metamodelica::arrayGet(iRequiredTime.clone(), sccIndex)?;
    oCosts = (-100 + iCosts_op, (iCosts_cyc) + (reqTime));
    Ok(oCosts)
}

fn createCommCosts(
    mut iCosts: metamodelica::Array<metamodelica::List<Communication>>,
    mut iCurrentIndex: i32,
    mut iReqTimeCom: (i32, i32),
) -> metamodelica::Array<metamodelica::List<Communication>> {
    let mut oCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut tmpCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut currentCom: Communications = metamodelica::nil();
    oCosts = 'mc: {
        let __mc_input = iCosts.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut tmpCosts = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut currentCom: metamodelica::List<Communication> = currentCom.clone();
            let true = (intLe(iCurrentIndex, metamodelica::arrayLength(iCosts.clone()))) else {
                return Err("pattern mismatch");
            };
            currentCom = metamodelica::arrayGet(tmpCosts.clone(), iCurrentIndex)?;
            currentCom = List::map1(
                currentCom.clone(),
                &fnptr!(createCommCosts0, Communication, (i32, i32)),
                iReqTimeCom,
            )?;
            tmpCosts = metamodelica::arrayUpdate(tmpCosts.clone(), iCurrentIndex, currentCom.clone())?;
            tmpCosts = createCommCosts(tmpCosts.clone(), iCurrentIndex + 1, iReqTimeCom);
            Ok((tmpCosts.clone(), currentCom.clone()))
        })() {
            currentCom = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iCosts.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCosts
}

fn createCommCosts0(mut iComm: Communication, mut iReqTimeCom: (i32, i32)) -> Communication {
    let mut oComm: Communication;
    let mut childNode: i32;
    let mut reqTimeM: i32;
    let mut reqTimeN: i32;
    let mut numberOfVars: i32;
    let mut requiredTime: metamodelica::Real;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    let Communication {
        numberOfVars: __pa0,
        integerVars: __pa1,
        floatVars: __pa2,
        booleanVars: __pa3,
        stringVars: __pa4,
        childNode: __pa5,
        requiredTime: __pa6,
    } = iComm;
    numberOfVars = metamodelica::Own::own(__pa0);
    integerVars = metamodelica::Own::own(__pa1);
    floatVars = metamodelica::Own::own(__pa2);
    booleanVars = metamodelica::Own::own(__pa3);
    stringVars = metamodelica::Own::own(__pa4);
    childNode = metamodelica::Own::own(__pa5);
    requiredTime = metamodelica::Own::own(__pa6);
    (reqTimeM, reqTimeN) = iReqTimeCom;
    requiredTime = intReal(reqTimeN + numberOfVars * reqTimeM);
    oComm = Communication {
        numberOfVars: numberOfVars,
        integerVars: integerVars,
        floatVars: floatVars,
        booleanVars: booleanVars,
        stringVars: stringVars,
        childNode: childNode,
        requiredTime: requiredTime,
    };
    oComm
}

//---------------------------------
//  Functions to validate the graph
//---------------------------------
pub(crate) fn validateTaskGraphMeta(
    mut iMeta: TaskGraphMeta,
    mut iDae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> bool {
    let mut valid: bool;
    valid = 'mc: {
        let __mc_input = &**iDae;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut systComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut graphComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut systCompsArray: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>;
                    let mut systCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
                    let mut graphCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
                    let mut systCompEqSysMappingIdx: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>;
                    let mut graphCompEqSysMappingIdx: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>;
                    (systComps, systCompEqSysMapping) = getSystemComponents(iDae)?;
                    systCompsArray = metamodelica::arrayFromVec(systComps.clone().into_iter().cloned().collect());
                    (graphComps, graphCompEqSysMapping) = getGraphComponents(iMeta.clone(), systCompsArray.clone(), systCompEqSysMapping.clone())?;
                    (_, _, systCompEqSysMappingIdx) = validateTaskGraphMeta0(systCompEqSysMapping.clone(), (1, systComps.clone(), metamodelica::nil()))?;
                    (_, _, graphCompEqSysMappingIdx) = validateTaskGraphMeta0(graphCompEqSysMapping.clone(), (1, graphComps.clone(), metamodelica::nil()))?;
                    let true = (validateComponents(graphCompEqSysMappingIdx.clone(), systCompEqSysMappingIdx.clone())) else { return Err("pattern mismatch") };
                    let true = (checkForDuplicates(graphCompEqSysMappingIdx.clone())?) else { return Err("pattern mismatch") };
                    let true = (checkForExecutionCosts(iMeta.clone())) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    valid
}

fn validateTaskGraphMeta0(
    mut iEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    mut iCompsTpl: (
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
    ),
) -> Result<(
    i32,
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
)> {
    '__tco: loop {
        let mut currentIdx: i32;
        let mut eqSysIdx: i32;
        let mut rest: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
        let mut head: metamodelica::Ref<BackendDAE::StrongComponent>;
        let mut iCompEqSysMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>;
        let mut oCompEqSysMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>;
        let mut tmpCompsTpl: (
            i32,
            metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
            metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
        );
        ::match_deref::match_deref! { match &(iCompsTpl.clone()) {
            (__esc_currentIdx, Deref @ metamodelica::ListNode::Cons { head: __esc_head, tail: __esc_rest }, __esc_iCompEqSysMapping) => {
                currentIdx = (*__esc_currentIdx).clone();
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                iCompEqSysMapping = (*__esc_iCompEqSysMapping).clone();
                (_, eqSysIdx) = metamodelica::arrayGet(iEqSysMapping.clone(), currentIdx.clone())?;
                oCompEqSysMapping = metamodelica::cons((head.clone(), eqSysIdx), iCompEqSysMapping.clone());
                { (iEqSysMapping, iCompsTpl) = (iEqSysMapping.clone(), (currentIdx.clone() + 1, rest.clone(), oCompEqSysMapping)); continue '__tco; }
            },
            _ => return Ok(iCompsTpl),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn validateComponents(
    mut graphComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
    mut systComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
) -> bool {
    let mut res: bool;
    let mut isEqual: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut comp1: metamodelica::Ref<BackendDAE::StrongComponent> =
        <metamodelica::Ref<BackendDAE::StrongComponent> as ::std::default::Default>::default();
    let mut comp2: metamodelica::Ref<BackendDAE::StrongComponent> =
        <metamodelica::Ref<BackendDAE::StrongComponent> as ::std::default::Default>::default();
    let mut tpl1: (metamodelica::Ref<BackendDAE::StrongComponent>, i32) = (
        <metamodelica::Ref<BackendDAE::StrongComponent> as ::std::default::Default>::default(),
        0,
    );
    let mut tpl2: (metamodelica::Ref<BackendDAE::StrongComponent>, i32) = (
        <metamodelica::Ref<BackendDAE::StrongComponent> as ::std::default::Default>::default(),
        0,
    );
    let mut sortedGraphComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)> =
        metamodelica::nil();
    let mut sortedSystComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)> =
        metamodelica::nil();
    res = 'mc: {
        let __mc_input = &*systComps;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut comp1: metamodelica::Ref<BackendDAE::StrongComponent> = comp1.clone();
                    let mut comp2: metamodelica::Ref<BackendDAE::StrongComponent> = comp2.clone();
                    let mut i1: i32 = i1.clone();
                    let mut i2: i32 = i2.clone();
                    let mut isEqual: bool = isEqual.clone();
                    let mut sortedGraphComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)> = sortedGraphComps.clone();
                    let mut sortedSystComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)> = sortedSystComps.clone();
                    let mut tpl1: (metamodelica::Ref<BackendDAE::StrongComponent>, i32) = tpl1.clone();
                    let mut tpl2: (metamodelica::Ref<BackendDAE::StrongComponent>, i32) = tpl2.clone();
                    sortedGraphComps = List::sort(graphComps.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<BackendDAE::StrongComponent>, i32), __a1: (metamodelica::Ref<BackendDAE::StrongComponent>, i32)| compareComponents(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<BackendDAE::StrongComponent>, i32), (metamodelica::Ref<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    sortedSystComps = List::sort(systComps.clone(), (std::sync::Arc::new(move |__a0: (metamodelica::Ref<BackendDAE::StrongComponent>, i32), __a1: (metamodelica::Ref<BackendDAE::StrongComponent>, i32)| compareComponents(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn((metamodelica::Ref<BackendDAE::StrongComponent>, i32), (metamodelica::Ref<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    if intNe(((sortedSystComps).len() as i32), ((sortedGraphComps).len() as i32)) {
                        metamodelica::print(literal!("the graph and the system have a difference number of components.\n"));
                    }
                    isEqual = true;
                    while isEqual && !((sortedGraphComps).is_empty()) {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sortedGraphComps.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        tpl1 = metamodelica::Own::own(__pa0);
                        sortedGraphComps = metamodelica::Own::own(__pa1);
                        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(sortedSystComps.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        tpl2 = metamodelica::Own::own(__pa2);
                        sortedSystComps = metamodelica::Own::own(__pa3);
                        (comp1, i1) = tpl1.clone();
                        (comp2, i2) = tpl2.clone();
                        if componentsEqual(&tpl1, &tpl2)? {
                            isEqual = true;
                        } else {
                            isEqual = false;
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("comp ")); __mm_s.push_str(&*intString(i1)); __mm_s.push_str(&*BackendDump::printComponent(&comp1, None)?); __mm_s.push_str(&*literal!(" is not equal to ")); __mm_s.push_str(&*literal!("comp")); __mm_s.push_str(&*intString(i2)); __mm_s.push_str(&*BackendDump::printComponent(&comp2, None)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        }
                    }
                    Ok((true, comp1.clone(), comp2.clone(), i1.clone(), i2.clone(), isEqual.clone(), sortedGraphComps.clone(), sortedSystComps.clone(), tpl1.clone(), tpl2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            comp1 = __wb0;
            comp2 = __wb1;
            i1 = __wb2;
            i2 = __wb3;
            isEqual = __wb4;
            sortedGraphComps = __wb5;
            sortedSystComps = __wb6;
            tpl1 = __wb7;
            tpl2 = __wb8;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("Different components in graph and system\n"));
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn checkForDuplicates(
    mut iComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>,
) -> Result<bool> {
    let mut res: bool;
    let mut sortedComps: metamodelica::List<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>;
    sortedComps = List::sort(
        iComps,
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<BackendDAE::StrongComponent>, i32),
                  __a1: (metamodelica::Ref<BackendDAE::StrongComponent>, i32)| {
                compareComponents(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<BackendDAE::StrongComponent>, i32),
                        (metamodelica::Ref<BackendDAE::StrongComponent>, i32),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    (res, _) = List::fold(
        &sortedComps,
        &move |__a0: (metamodelica::Ref<BackendDAE::StrongComponent>, i32),
               __a1: (bool, Option<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>)|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(checkForDuplicates0(__a0, &__a1)) },
        (true, None),
    )?;
    Ok(res)
}

fn checkForDuplicates0(
    mut currentComp_idx: (metamodelica::Ref<BackendDAE::StrongComponent>, i32),
    mut iLastComp: &(bool, Option<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>),
) -> (bool, Option<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>) {
    let mut oLastComp: (bool, Option<(metamodelica::Ref<BackendDAE::StrongComponent>, i32)>);
    let mut lastComp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut currentComp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut lastComp_idx: (metamodelica::Ref<BackendDAE::StrongComponent>, i32);
    let mut idxLast: i32;
    let mut idxCurrent: i32;
    oLastComp = 'mc: {
        let __mc_input = (&currentComp_idx, iLastComp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (false, _)) => {
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, None)) => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((currentComp, idxCurrent), (_, Some(lastComp_idx @ (lastComp, idxLast)))) => {
                    let true = (componentsEqual(&currentComp_idx, &(lastComp_idx.clone()))?) else { return Err("pattern mismatch") };
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Component duplicate detected: current: ")); __mm_s.push_str(&*BackendDump::printComponent(metamodelica::AsArg::as_arg(&currentComp), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxCurrent.clone())); __mm_s.push_str(&*literal!(") last ")); __mm_s.push_str(&*BackendDump::printComponent(metamodelica::AsArg::as_arg(&lastComp), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxLast.clone())); __mm_s.push_str(&*literal!(").\n")); ArcStr::from(__mm_s) });
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oLastComp
}

fn getGraphComponents(
    mut iTaskGraphMeta: TaskGraphMeta,
    mut iSystComps: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
)> {
    let mut oComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut oCompEqGraphMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut tmpComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut tmpMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut nodeMarks: metamodelica::Array<i32>;
    tmpComps = metamodelica::nil();
    tmpMapping = metamodelica::nil();
    let TaskGraphMeta {
        inComps: __pa0,
        nodeMark: __pa1,
        ..
    } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    nodeMarks = metamodelica::Own::own(__pa1);
    (tmpComps, tmpMapping) = Array::fold(
        inComps.clone(),
        &({
            let __pe_b1 = iSystComps.clone();
            let __pe_b2 = iCompEqSysMapping.clone();
            move |__pe_a0, __pe_a3| getGraphComponents0(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_a3)
        }),
        (tmpComps, tmpMapping),
    )?;
    let (_, (__pa2, __pa3)) = Array::fold(
        nodeMarks.clone(),
        &({
            let __pe_b1 = iSystComps.clone();
            let __pe_b2 = iCompEqSysMapping.clone();
            move |__pe_a0, __pe_a3| getGraphComponents2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_a3)
        }),
        (1, (tmpComps, tmpMapping)),
    )?;
    tmpComps = metamodelica::Own::own(__pa2);
    tmpMapping = metamodelica::Own::own(__pa3);
    oComps = tmpComps;
    oCompEqGraphMapping = metamodelica::arrayFromVec(tmpMapping.into_iter().cloned().collect());
    Ok((oComps, oCompEqGraphMapping))
}

fn getGraphComponents0(
    mut inComp: &metamodelica::List<i32>,
    mut systComps: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    mut iNodeComps_Mapping: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
)> {
    let mut oNodeComps_Mapping: (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    );
    let mut iNodeComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut tmpNodeComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut iCompsMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let mut tmpCompsMapping: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    (iNodeComps, iCompsMapping) = iNodeComps_Mapping.clone();
    (tmpNodeComps, tmpCompsMapping) = List::fold2(
        inComp,
        &move |__a0: i32,
               __a1: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>,
               __a2: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
               __a3: (
            metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
            metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
        )| getGraphComponents1(__a0, __a1, __a2, &__a3),
        systComps.clone(),
        iCompEqSysMapping.clone(),
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    tmpNodeComps = listAppend(iNodeComps, tmpNodeComps);
    tmpCompsMapping = listAppend(iCompsMapping, tmpCompsMapping);
    oNodeComps_Mapping = (tmpNodeComps, tmpCompsMapping);
    Ok(oNodeComps_Mapping)
}

fn getGraphComponents1(
    mut compIdx: i32,
    mut systComps: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    mut iNodeComps_Mapping: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
)> {
    let mut oNodeComps_Mapping: (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    );
    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut eqSyst: (metamodelica::Ref<BackendDAE::EqSystem>, i32);
    let mut tmpComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut tmpSysts: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    (tmpComps, tmpSysts) = iNodeComps_Mapping.clone();
    comp = metamodelica::arrayGet(systComps.clone(), compIdx)?;
    eqSyst = metamodelica::arrayGet(iCompEqSysMapping.clone(), compIdx)?;
    tmpComps = metamodelica::cons(comp, tmpComps);
    tmpSysts = metamodelica::cons(eqSyst, tmpSysts);
    oNodeComps_Mapping = (tmpComps, tmpSysts);
    Ok(oNodeComps_Mapping)
}

fn getGraphComponents2(
    mut nodeMark: i32,
    mut systComps: metamodelica::Array<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iCompEqSysMapping: metamodelica::Array<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    mut iNodeComps_Mapping: &(
        i32,
        (
            metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
            metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
        ),
    ),
) -> Result<(
    i32,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
        metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
    ),
)> {
    let mut oNodeComps_Mapping: (
        i32,
        (
            metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
            metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>,
        ),
    );
    let mut nodeIdx: i32;
    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut eqSyst: (metamodelica::Ref<BackendDAE::EqSystem>, i32);
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eqSysts: metamodelica::List<(metamodelica::Ref<BackendDAE::EqSystem>, i32)>;
    let (__pa0, (__pa1, __pa2)) = iNodeComps_Mapping;
    nodeIdx = metamodelica::Own::own(__pa0);
    comps = metamodelica::Own::own(__pa1);
    eqSysts = metamodelica::Own::own(__pa2);
    if !(intGe(nodeMark, 0) || intEq(nodeMark, -2)) {
        comp = metamodelica::arrayGet(systComps.clone(), nodeIdx)?;
        eqSyst = metamodelica::arrayGet(iCompEqSysMapping.clone(), nodeIdx)?;
        comps = metamodelica::cons(comp, comps);
        eqSysts = metamodelica::cons(eqSyst, eqSysts);
    }
    oNodeComps_Mapping = (nodeIdx + 1, (comps, eqSysts));
    Ok(oNodeComps_Mapping)
}

fn componentsEqual(
    mut iComp1: &(metamodelica::Ref<BackendDAE::StrongComponent>, i32),
    mut iComp2: &(metamodelica::Ref<BackendDAE::StrongComponent>, i32),
) -> Result<bool> {
    let mut res: bool;
    let mut comp1Str: ArcStr;
    let mut comp2Str: ArcStr;
    let mut comp1Idx: i32;
    let mut comp2Idx: i32;
    let mut comp1: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut comp2: metamodelica::Ref<BackendDAE::StrongComponent>;
    (comp1, comp1Idx) = iComp1.clone();
    (comp2, comp2Idx) = iComp2.clone();
    comp1Str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*BackendDump::printComponent(&comp1, None)?);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(comp1Idx));
        ArcStr::from(__mm_s)
    };
    comp2Str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*BackendDump::printComponent(&comp2, None)?);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(comp2Idx));
        ArcStr::from(__mm_s)
    };
    if intNe(((comp1Str).len() as i32), ((comp2Str).len() as i32)) {
        res = false;
    } else {
        res = intEq(
            System::strncmp(comp1Str.clone(), comp2Str, ((comp1Str).len() as i32)),
            0,
        );
    }
    Ok(res)
}

fn compareComponents(
    mut iComp1: &(metamodelica::Ref<BackendDAE::StrongComponent>, i32),
    mut iComp2: &(metamodelica::Ref<BackendDAE::StrongComponent>, i32),
) -> Result<bool> {
    let mut res: bool;
    let mut comp1Str: ArcStr;
    let mut comp2Str: ArcStr;
    let mut minLength: i32;
    let mut compRes: i32;
    let mut comp1Idx: i32;
    let mut comp2Idx: i32;
    let mut comp1: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut comp2: metamodelica::Ref<BackendDAE::StrongComponent>;
    if componentsEqual(iComp1, iComp2)? {
        res = false;
    } else {
        (comp1, comp1Idx) = iComp1.clone();
        (comp2, comp2Idx) = iComp2.clone();
        comp1Str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BackendDump::printComponent(&comp1, None)?);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(comp1Idx));
            ArcStr::from(__mm_s)
        };
        comp2Str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BackendDump::printComponent(&comp2, None)?);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(comp2Idx));
            ArcStr::from(__mm_s)
        };
        minLength = intMin(((comp1Str).len() as i32), ((comp2Str).len() as i32));
        compRes = System::strncmp(comp1Str.clone(), comp2Str.clone(), minLength);
        if intEq(compRes, 0) {
            res = intLt(((comp1Str).len() as i32), ((comp2Str).len() as i32));
        } else {
            res = intLt(compRes, 0);
        }
    }
    Ok(res)
}

//------------------------------------
//  Evaluation and analysing functions
//------------------------------------
pub(crate) fn getCriticalPaths(
    mut graphIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
) -> (
    (metamodelica::List<metamodelica::List<i32>>, metamodelica::Real),
    (metamodelica::List<metamodelica::List<i32>>, metamodelica::Real),
) {
    let mut criticalPathOut: (metamodelica::List<metamodelica::List<i32>>, metamodelica::Real);
    let mut criticalPathOutWoC: (metamodelica::List<metamodelica::List<i32>>, metamodelica::Real);
    (criticalPathOut, criticalPathOutWoC) = 'mc: {
        let __mc_input = graphDataIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let TaskGraphMeta { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rootNodes: metamodelica::List<i32>;
            let mut cpWCpaths: metamodelica::List<metamodelica::List<i32>>;
            let mut CpWoCpaths: metamodelica::List<metamodelica::List<i32>>;
            let mut cpWCcosts: metamodelica::Real;
            let mut cpWoCcosts: metamodelica::Real;
            let true = (metamodelica::arrayLength(graphIn.clone()) != 0) else {
                return Err("pattern mismatch");
            };
            rootNodes = getRootNodes(graphIn.clone())?;
            (cpWCpaths, cpWCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), true)?;
            (CpWoCpaths, cpWoCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), false)?;
            cpWCcosts = roundReal(cpWCcosts, 2)?;
            cpWoCcosts = roundReal(cpWoCcosts, 2)?;
            Ok(((cpWCpaths.clone(), cpWCcosts), (CpWoCpaths.clone(), cpWoCcosts)))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (metamodelica::arrayLength(graphIn.clone()) == 0) else {
                return Err("pattern mismatch");
            };
            Ok((
                (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)),
                (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("getCriticalPaths failed!\n"));
            Ok((
                (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)),
                (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)),
            ))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (criticalPathOut, criticalPathOutWoC)
}

fn getCriticalPath(
    mut iGraph: TaskGraph,
    mut iGraphData: TaskGraphMeta,
    mut iRootNodes: metamodelica::List<i32>,
    mut iHandleCommCosts: bool,
) -> Result<(metamodelica::List<metamodelica::List<i32>>, metamodelica::Real)> {
    let mut oCriticalPathsOut: metamodelica::List<metamodelica::List<i32>>;
    let mut oCpCosts: metamodelica::Real;
    let mut nodeCriticalPaths: metamodelica::Array<(metamodelica::Real, metamodelica::List<i32>)>;
    let mut criticalPaths: metamodelica::List<(metamodelica::Real, metamodelica::List<i32>)>;
    let mut criticalPathIdx: i32;
    let mut criticalPath: metamodelica::List<i32>;
    nodeCriticalPaths = arrayCreate(
        metamodelica::arrayLength(iGraph.clone()),
        (metamodelica::OrderedFloat(-1.0_f64), metamodelica::nil()),
    );
    criticalPaths = List::map4(
        iRootNodes,
        &getCriticalPath1,
        iGraph.clone(),
        iGraphData,
        iHandleCommCosts,
        nodeCriticalPaths.clone(),
    )?;
    criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1);
    (oCpCosts, criticalPath) = (criticalPaths).get(criticalPathIdx)?;
    oCriticalPathsOut = list![criticalPath];
    Ok((oCriticalPathsOut, oCpCosts))
}

fn getCriticalPath1(
    mut iNode: i32,
    mut iGraph: TaskGraph,
    mut iGraphData: TaskGraphMeta,
    mut iHandleCommCosts: bool,
    mut iNodeCriticalPaths: metamodelica::Array<(metamodelica::Real, metamodelica::List<i32>)>,
) -> Result<(metamodelica::Real, metamodelica::List<i32>)> {
    let mut criticalPathOut: (metamodelica::Real, metamodelica::List<i32>);
    let mut cpCalcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut commTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut criticalPathIdx: i32 = 0;
    let mut commCost: Communication = <Communication as ::std::default::Default>::default();
    let mut childNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut criticalPathChild: metamodelica::List<i32> = metamodelica::nil();
    let mut criticalPath: metamodelica::List<i32> = metamodelica::nil();
    let mut nodeComps: metamodelica::List<i32> = metamodelica::nil();
    let mut criticalPaths: metamodelica::List<(metamodelica::Real, metamodelica::List<i32>)> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    criticalPathOut = 'mc: {
        let __mc_input = iGraphData.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let TaskGraphMeta {
                inComps: mut inComps,
                exeCosts: mut exeCosts,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPath: metamodelica::List<i32> = criticalPath.clone();
            (cpCalcTime, criticalPath) = metamodelica::arrayGet(iNodeCriticalPaths.clone(), iNode)?;
            let true = (realGe(cpCalcTime, metamodelica::OrderedFloat(0.0_f64))) else {
                return Err("pattern mismatch");
            };
            Ok((
                (cpCalcTime, criticalPath.clone()),
                cpCalcTime.clone(),
                criticalPath.clone(),
            ))
        })() {
            cpCalcTime = __wb0;
            criticalPath = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            let TaskGraphMeta {
                inComps: mut inComps,
                exeCosts: mut exeCosts,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut childNodes: metamodelica::List<i32> = childNodes.clone();
            let mut commCost: Communication = commCost.clone();
            let mut commTime: metamodelica::Real = commTime.clone();
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPath: metamodelica::List<i32> = criticalPath.clone();
            let mut criticalPathChild: metamodelica::List<i32> = criticalPathChild.clone();
            let mut criticalPathIdx: i32 = criticalPathIdx.clone();
            let mut criticalPaths: metamodelica::List<(metamodelica::Real, metamodelica::List<i32>)> =
                criticalPaths.clone();
            let mut nodeComps: metamodelica::List<i32> = nodeComps.clone();
            childNodes = metamodelica::arrayGet(iGraph.clone(), iNode)?;
            let false = ((childNodes).is_empty()) else {
                return Err("pattern mismatch");
            };
            criticalPaths = List::map4(
                childNodes.clone(),
                &getCriticalPath1,
                iGraph.clone(),
                iGraphData.clone(),
                iHandleCommCosts,
                iNodeCriticalPaths.clone(),
            )?;
            criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1);
            (cpCalcTime, criticalPathChild) = (criticalPaths).get(criticalPathIdx)?;
            criticalPath = metamodelica::cons(iNode, criticalPathChild.clone());
            commCost = if (iHandleCommCosts) {
                getCommCostBetweenNodes(iNode, (criticalPathChild).head().cloned()?, iGraphData.clone())?
            } else {
                Communication {
                    numberOfVars: 0,
                    integerVars: metamodelica::nil(),
                    floatVars: metamodelica::nil(),
                    booleanVars: metamodelica::nil(),
                    stringVars: metamodelica::nil(),
                    childNode: -1,
                    requiredTime: metamodelica::OrderedFloat(0.0_f64),
                }
            };
            nodeComps = metamodelica::arrayGet(inComps.clone(), iNode)?;
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            calcTime = (cpCalcTime) + (calcTime);
            let Communication {
                requiredTime: __pa0, ..
            } = &commCost;
            commTime = metamodelica::Own::own(__pa0);
            calcTime = (calcTime) + (commTime);
            metamodelica::arrayUpdate(iNodeCriticalPaths.clone(), iNode, (calcTime, criticalPath.clone()))?;
            Ok((
                (calcTime, criticalPath.clone()),
                calcTime.clone(),
                childNodes.clone(),
                commCost.clone(),
                commTime.clone(),
                cpCalcTime.clone(),
                criticalPath.clone(),
                criticalPathChild.clone(),
                criticalPathIdx.clone(),
                criticalPaths.clone(),
                nodeComps.clone(),
            ))
        })() {
            calcTime = __wb0;
            childNodes = __wb1;
            commCost = __wb2;
            commTime = __wb3;
            cpCalcTime = __wb4;
            criticalPath = __wb5;
            criticalPathChild = __wb6;
            criticalPathIdx = __wb7;
            criticalPaths = __wb8;
            nodeComps = __wb9;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let TaskGraphMeta {
                inComps: mut inComps,
                exeCosts: mut exeCosts,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut childNodes: metamodelica::List<i32> = childNodes.clone();
            let mut criticalPath: metamodelica::List<i32> = criticalPath.clone();
            let mut nodeComps: metamodelica::List<i32> = nodeComps.clone();
            childNodes = metamodelica::arrayGet(iGraph.clone(), iNode)?;
            let true = ((childNodes).is_empty()) else {
                return Err("pattern mismatch");
            };
            criticalPath = metamodelica::cons(iNode, metamodelica::nil());
            nodeComps = metamodelica::arrayGet(inComps.clone(), iNode)?;
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            metamodelica::arrayUpdate(iNodeCriticalPaths.clone(), iNode, (calcTime, criticalPath.clone()))?;
            Ok((
                (calcTime, criticalPath.clone()),
                calcTime.clone(),
                childNodes.clone(),
                criticalPath.clone(),
                nodeComps.clone(),
            ))
        })() {
            calcTime = __wb0;
            childNodes = __wb1;
            criticalPath = __wb2;
            nodeComps = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("HpcOmTaskGraph.getCriticalPath_1 failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(criticalPathOut)
}

fn getCriticalPath2(
    mut iCriticalPaths: metamodelica::List<(metamodelica::Real, metamodelica::List<i32>)>,
    mut iListIdx: i32,
    mut iLongestPath: metamodelica::Real,
    mut iLongestPathIndex: i32,
) -> i32 {
    '__tco: loop {
        let mut cpCost: metamodelica::Real;
        let mut criticalPath: metamodelica::List<i32>;
        let mut rest: metamodelica::List<(metamodelica::Real, metamodelica::List<i32>)>;
        ::match_deref::match_deref! { match &(iCriticalPaths) {
            Deref @ metamodelica::ListNode::Cons { head: (cpCost, __esc_criticalPath), tail: __esc_rest } if (realGt(cpCost.clone(), iLongestPath)) => {
                criticalPath = (*__esc_criticalPath).clone();
                rest = (*__esc_rest).clone();
                { (iCriticalPaths, iListIdx, iLongestPath, iLongestPathIndex) = (rest.clone(), iListIdx + 1, cpCost.clone(), iListIdx); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: (__esc_cpCost, __esc_criticalPath), tail: __esc_rest } => {
                cpCost = (*__esc_cpCost).clone();
                criticalPath = (*__esc_criticalPath).clone();
                rest = (*__esc_rest).clone();
                { (iCriticalPaths, iListIdx, iLongestPath, iLongestPathIndex) = (rest.clone(), iListIdx + 1, iLongestPath, iLongestPathIndex); continue '__tco; }
            },
            _ => return iLongestPathIndex,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn addUpExeCostsForNode(
    mut iNodeComps: metamodelica::List<i32>,
    mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut iExeCost: metamodelica::Real,
) -> Result<metamodelica::Real> {
    '__tco: loop {
        let mut head: i32;
        let mut rest: metamodelica::List<i32>;
        let mut cost: metamodelica::Real;
        ::match_deref::match_deref! { match &(iNodeComps) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_head, tail: __esc_rest } => {
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                (_, cost) = metamodelica::arrayGet(iExeCosts.clone(), head.clone())?;
                cost = (cost) + (iExeCost);
                { (iNodeComps, iExeCosts, iExeCost) = (rest.clone(), iExeCosts.clone(), cost); continue '__tco; }
            },
            _ => return Ok(iExeCost),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn gatherParallelSets(
    mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut parallelSetsOut: metamodelica::List<metamodelica::List<i32>>;
    let mut numLevels: i32;
    numLevels = Array::fold(
        nodeInfo.clone(),
        &fnptr!(numberOfLevels, (i32, metamodelica::Real, i32), i32),
        0,
    )?;
    parallelSetsOut = List::fold1(
        &(List::intRange(metamodelica::arrayLength(nodeInfo.clone()))),
        &gatherParallelSets1,
        nodeInfo.clone(),
        List::fill(metamodelica::nil(), numLevels),
    )?;
    Ok(parallelSetsOut)
}

fn numberOfLevels(mut nodeInfoEntry: (i32, metamodelica::Real, i32), mut numLevelsIn: i32) -> i32 {
    let mut numLevelsOut: i32;
    let mut levelIn: i32;
    (levelIn, _, _) = nodeInfoEntry;
    numLevelsOut = intMax(levelIn, numLevelsIn);
    numLevelsOut
}

fn gatherParallelSets1(
    mut idx: i32,
    mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>,
    mut parallelSetIn: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut parallelSetOut: metamodelica::List<metamodelica::List<i32>>;
    let mut level: i32;
    let mut pSet: metamodelica::List<i32>;
    (level, _, _) = metamodelica::arrayGet(nodeInfo.clone(), idx)?;
    pSet = (parallelSetIn).get(level)?;
    pSet = metamodelica::cons(idx, pSet);
    parallelSetOut = List::replaceAt(pSet, level, parallelSetIn)?;
    Ok(parallelSetOut)
}

fn getCostsForNode(
    mut parentNode: i32,
    mut childNode: i32,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut commCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = 'mc: {
        let __mc_input = parentNode;
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { return Err("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChild: i32;
            let mut primalChildLst: metamodelica::List<i32>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            let true = (((primalChildLst).len() as i32) == 1) else {
                return Err("pattern mismatch");
            };
            primalChild = (primalChildLst).get(1)?;
            (_, costs) = metamodelica::arrayGet(exeCosts.clone(), primalChild)?;
            Ok(costs)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { return Err("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChildLst: metamodelica::List<i32>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            let true = (((primalChildLst).len() as i32) > 1) else {
                return Err("pattern mismatch");
            };
            (primalChildLst).get(1)?;
            costs = getCostsForContractedNodes(&primalChildLst, exeCosts.clone())?;
            Ok(costs)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut costs: metamodelica::Real;
            let mut commCost: metamodelica::Real;
            let mut primalChild: i32;
            let mut primalParent: i32;
            let mut primalChildLst: metamodelica::List<i32>;
            let mut primalParentLst: metamodelica::List<i32>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            primalParentLst = metamodelica::arrayGet(inComps.clone(), parentNode)?;
            let true = (((primalChildLst).len() as i32) == 1) else {
                return Err("pattern mismatch");
            };
            primalChild = (primalChildLst).get(1)?;
            primalParent = (primalParentLst).get(1)?;
            (_, costs) = metamodelica::arrayGet(exeCosts.clone(), primalChild)?;
            let Communication {
                requiredTime: __pa0, ..
            } = getCommunicationCost(primalChild, primalParent, commCosts.clone())?;
            commCost = metamodelica::Own::own(__pa0);
            costs = costs + commCost;
            Ok(costs)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChildLst: metamodelica::List<i32>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            metamodelica::arrayGet(inComps.clone(), parentNode)?;
            let true = (((primalChildLst).len() as i32) > 1) else {
                return Err("pattern mismatch");
            };
            costs = getCostsForContractedNodes(&primalChildLst, exeCosts.clone())?;
            Ok(costs)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("getCostsForNode failed! \n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(costsOut)
}

pub(crate) fn getCostsForContractedNodes(
    mut nodeList: &metamodelica::List<i32>,
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = List::fold1(
        nodeList,
        &getCostsForContractedNodes1,
        exeCosts.clone(),
        metamodelica::OrderedFloat(0.0_f64),
    )?;
    Ok(costsOut)
}

fn getCostsForContractedNodes1(
    mut node: i32,
    mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    mut costsIn: metamodelica::Real,
) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    let mut exeCost: metamodelica::Real;
    (_, exeCost) = metamodelica::arrayGet(exeCosts.clone(), node)?;
    costsOut = (costsIn) + (exeCost);
    Ok(costsOut)
}

fn getNodeCoords(
    mut parallelSets: metamodelica::List<metamodelica::List<i32>>,
    mut graphIn: TaskGraph,
) -> Result<metamodelica::Array<(i32, i32)>> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut nodeCoords: metamodelica::Array<(i32, i32)>;
    let mut size: i32;
    size = metamodelica::arrayLength(graphIn.clone());
    nodeCoords = arrayCreate(size, (0, 0));
    nodeCoords = List::fold1(
        &(List::intRange(size)),
        &move |__a0: i32, __a1: metamodelica::List<metamodelica::List<i32>>, __a2: metamodelica::Array<(i32, i32)>| {
            getYCoordForNode(__a0, &__a1, __a2)
        },
        parallelSets,
        nodeCoords.clone(),
    )?;
    nodeCoordsOut = nodeCoords.clone();
    Ok(nodeCoordsOut)
}

fn getYCoordForNode(
    mut compIdx: i32,
    mut parallelSets: &metamodelica::List<metamodelica::List<i32>>,
    mut nodeCoordsIn: metamodelica::Array<(i32, i32)>,
) -> Result<metamodelica::Array<(i32, i32)>> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut parallelSetIdx: i32;
    let mut xCoord: i32;
    let mut yCoord: i32;
    let mut coords: (i32, i32);
    parallelSetIdx = getParallelSetForComp(compIdx, 1, parallelSets)?;
    (xCoord, yCoord) = metamodelica::arrayGet(nodeCoordsIn.clone(), compIdx)?;
    coords = (xCoord, parallelSetIdx);
    nodeCoordsOut = metamodelica::arrayUpdate(nodeCoordsIn.clone(), compIdx, coords)?;
    Ok(nodeCoordsOut)
}

fn getParallelSetForComp(
    mut compIn: i32,
    mut setIdx: i32,
    mut parallelSets: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<i32> {
    let mut parallelSetOut: i32;
    parallelSetOut = 'mc: {
        let __mc_input = &**parallelSets;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut parallelSet: metamodelica::List<i32>;
                    let true = (setIdx <= ((parallelSets).len() as i32)) else { return Err("pattern mismatch") };
                    parallelSet = (parallelSets).get(setIdx)?;
                    let true = (List::isMemberOnTrue(compIn, &parallelSet, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    Ok(setIdx)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut parallelSet: metamodelica::List<i32>;
                    let mut parallelSetTmp: i32;
                    let true = (setIdx <= ((parallelSets).len() as i32)) else { return Err("pattern mismatch") };
                    parallelSet = (parallelSets).get(setIdx)?;
                    let false = (List::isMemberOnTrue(compIn, &parallelSet, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    parallelSetTmp = getParallelSetForComp(compIn, setIdx + 1, parallelSets)?;
                    Ok(parallelSetTmp)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getParallelSetForComp failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(parallelSetOut)
}

fn setLevelInNodeMark(
    mut nodeIdx: i32,
    mut inComps: metamodelica::Array<metamodelica::List<i32>>,
    mut nodeCoords: metamodelica::Array<(i32, i32)>,
    mut nodeMarkIn: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = nodeMarkIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut components: metamodelica::List<i32>;
            let mut primalComp: i32;
            let mut nodeMarkEntry: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), nodeIdx)?;
            components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            primalComp = List::last(&components)?;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), primalComp)?;
            let true = (intEq(-1, nodeMarkEntry)) else {
                return Err("pattern mismatch");
            };
            Ok(nodeMarkIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut nodeMarkTmp: metamodelica::Array<i32>;
            let mut components: metamodelica::List<i32>;
            let mut primalComp: i32;
            let mut nodeMarkEntry: i32;
            let mut yCoord: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), nodeIdx)?;
            components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            primalComp = List::last(&components)?;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), primalComp)?;
            let false = (intEq(-1, nodeMarkEntry)) else {
                return Err("pattern mismatch");
            };
            (_, yCoord) = metamodelica::arrayGet(nodeCoords.clone(), nodeIdx)?;
            nodeMarkTmp = metamodelica::arrayUpdate(nodeMarkIn.clone(), primalComp, yCoord)?;
            Ok(nodeMarkTmp.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(nodeMarkOut)
}

fn tupleToStringIntRealInt(mut inTuple: (i32, metamodelica::Real, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = (match inTuple {
        (mut int1, mut real1, mut int2) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(int1));
            __mm_s.push_str(&*literal!(","));
            __mm_s.push_str(&*realString(real1));
            __mm_s.push_str(&*literal!(" , "));
            __mm_s.push_str(&*intString(int2));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    result
}

pub(crate) fn transposeCommCosts(
    mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Array<metamodelica::List<Communication>>> {
    let mut oCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut tmpCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    tmpCommCosts = arrayCreate(metamodelica::arrayLength(iCommCosts.clone()), metamodelica::nil());
    (_, tmpCommCosts) = Array::fold(
        iCommCosts.clone(),
        &move |__a0: metamodelica::List<Communication>,
               __a1: (i32, metamodelica::Array<metamodelica::List<Communication>>)| {
            transposeCommCosts0(&__a0, __a1)
        },
        (1, tmpCommCosts.clone()),
    )?;
    oCommCosts = tmpCommCosts.clone();
    Ok(oCommCosts)
}

fn transposeCommCosts0(
    mut iCosts: &Communications,
    mut iCommCosts: (i32, metamodelica::Array<metamodelica::List<Communication>>),
) -> Result<(i32, metamodelica::Array<metamodelica::List<Communication>>)> {
    let mut oCommCosts: (i32, metamodelica::Array<metamodelica::List<Communication>>);
    let mut iParentCompIdx: i32;
    let mut tmpCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    (iParentCompIdx, tmpCommCosts) = iCommCosts;
    tmpCommCosts = List::fold1(
        iCosts,
        &move |__a0: Communication,
               __a1: i32,
               __a2: metamodelica::Array<metamodelica::List<Communication>>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(transposeCommCosts1(&__a0, __a1, __a2)) },
        iParentCompIdx,
        tmpCommCosts.clone(),
    )?;
    oCommCosts = (iParentCompIdx + 1, tmpCommCosts.clone());
    Ok(oCommCosts)
}

fn transposeCommCosts1(
    mut iCost: &Communication,
    mut iParentCompIdx: i32,
    mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> metamodelica::Array<metamodelica::List<Communication>> {
    let mut oCommCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut tmpCommCosts: metamodelica::Array<metamodelica::List<Communication>> = Default::default();
    let mut costs: Communications = metamodelica::nil();
    let mut numberOfVars: i32;
    let mut nodeIdx: i32;
    let mut integerVars: metamodelica::List<i32>;
    let mut floatVars: metamodelica::List<i32>;
    let mut booleanVars: metamodelica::List<i32>;
    let mut stringVars: metamodelica::List<i32>;
    let mut requiredTime: metamodelica::Real;
    oCommCosts = 'mc: {
        let __mc_input = iCost.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let Communication {
                numberOfVars: mut numberOfVars,
                integerVars: mut integerVars,
                floatVars: mut floatVars,
                booleanVars: mut booleanVars,
                stringVars: mut stringVars,
                childNode: mut nodeIdx,
                requiredTime: mut requiredTime,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut costs: metamodelica::List<Communication> = costs.clone();
            let mut tmpCommCosts: metamodelica::Array<metamodelica::List<Communication>> = tmpCommCosts.clone();
            let true = (intLe(nodeIdx, metamodelica::arrayLength(iCommCosts.clone()))) else {
                return Err("pattern mismatch");
            };
            costs = metamodelica::arrayGet(iCommCosts.clone(), nodeIdx)?;
            costs = metamodelica::cons(
                Communication {
                    numberOfVars: numberOfVars,
                    integerVars: integerVars.clone(),
                    floatVars: floatVars.clone(),
                    booleanVars: booleanVars.clone(),
                    stringVars: stringVars.clone(),
                    childNode: iParentCompIdx,
                    requiredTime: requiredTime,
                },
                costs.clone(),
            );
            tmpCommCosts = metamodelica::arrayUpdate(iCommCosts.clone(), nodeIdx, costs.clone())?;
            Ok((tmpCommCosts.clone(), costs.clone(), tmpCommCosts.clone()))
        })() {
            costs = __wb0;
            tmpCommCosts = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(iCommCosts.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCommCosts
}

//TODO: Can this be merged with getCommCostBetweenNodes?
fn getCommunicationCost(
    mut childIdx: i32,
    mut parentIdx: i32,
    mut commCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<Communication> {
    let mut oComm: Communication;
    let mut commRow: Communications;
    let mut commEntry: Communication;
    commRow = metamodelica::arrayGet(commCosts.clone(), parentIdx)?;
    commEntry = getCommunicationByChildIdx(&commRow, childIdx)?;
    oComm = commEntry;
    Ok(oComm)
}

fn getCommunicationByChildIdx(mut iComms: &Communications, mut iChildIdx: i32) -> Result<Communication> {
    let mut oComm: Communication;
    oComm = 'mc: {
        let __mc_input = &**iComms;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Communication { childNode: currentCommChild, .. }, tail: rest } => {
                    let mut tmpComm: Communication;
                    let false = (intEq(currentCommChild.clone(), iChildIdx)) else { return Err("pattern mismatch") };
                    tmpComm = getCommunicationByChildIdx(metamodelica::AsArg::as_arg(&rest), iChildIdx)?;
                    Ok(tmpComm.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head @ Communication { childNode: currentCommChild, .. }, tail: _ } => {
                    let true = (intEq(currentCommChild.clone(), iChildIdx)) else { return Err("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCommunicationByChildIdx failed! - the child idx ")); __mm_s.push_str(&*intString(iChildIdx)); __mm_s.push_str(&*literal!(" can not be found in the list of edges\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oComm)
}

pub(crate) fn getCommCostTimeBetweenNodes(
    mut iParentNodeIdx: i32,
    mut iChildNodeIdx: i32,
    mut iTaskGraphMeta: TaskGraphMeta,
) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real;
    let mut requiredTime: metamodelica::Real;
    let Communication {
        requiredTime: __pa0, ..
    } = getCommCostBetweenNodes(iParentNodeIdx, iChildNodeIdx, iTaskGraphMeta)?;
    requiredTime = metamodelica::Own::own(__pa0);
    oCommCost = requiredTime;
    Ok(oCommCost)
}

fn getCommCostBetweenNodes(
    mut iParentNodeIdx: i32,
    mut iChildNodeIdx: i32,
    mut iTaskGraphMeta: TaskGraphMeta,
) -> Result<Communication> {
    let mut oCommCost: Communication;
    let mut childComps: metamodelica::List<i32>;
    let mut parentComps: metamodelica::List<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut commCosts: metamodelica::Array<metamodelica::List<Communication>>;
    let mut concreteCommCostsOpt: metamodelica::List<Option<Communication>>;
    let mut concreteCommCosts: Communications;
    let TaskGraphMeta {
        inComps: __pa0,
        commCosts: __pa1,
        ..
    } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    commCosts = metamodelica::Own::own(__pa1);
    parentComps = metamodelica::arrayGet(inComps.clone(), iParentNodeIdx)?;
    childComps = metamodelica::arrayGet(inComps.clone(), iChildNodeIdx)?;
    concreteCommCostsOpt = List::map2(
        parentComps,
        &fnptr!(
            getCommCostBetweenNodes0,
            i32,
            metamodelica::List<i32>,
            metamodelica::Array<metamodelica::List<Communication>>
        ),
        childComps,
        commCosts.clone(),
    )?;
    concreteCommCosts = ({
        let mut __acc: metamodelica::List<Communication> = metamodelica::nil();
        for mut c in (concreteCommCostsOpt).into_iter().cloned() {
            if !((c).is_some()) {
                continue;
            }
            let __x = Util::getOption(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    oCommCost = getHighestCommCost(
        concreteCommCosts,
        Communication {
            numberOfVars: 0,
            integerVars: metamodelica::nil(),
            floatVars: metamodelica::nil(),
            booleanVars: metamodelica::nil(),
            stringVars: metamodelica::nil(),
            childNode: -1,
            requiredTime: metamodelica::OrderedFloat(-1.0_f64),
        },
    );
    Ok(oCommCost)
}

fn getCommCostBetweenNodes0(
    mut iParentComp: i32,
    mut iChildComps: metamodelica::List<i32>,
    mut iCommCosts: metamodelica::Array<metamodelica::List<Communication>>,
) -> Option<Communication> {
    let mut oHighestComm: Option<Communication>;
    let mut commCosts: Communications = metamodelica::nil();
    let mut filteredCommCosts: Communications = metamodelica::nil();
    let mut highestCommCost: Communication = <Communication as ::std::default::Default>::default();
    oHighestComm = 'mc: {
        let __mc_input = iCommCosts.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut commCosts: metamodelica::List<Communication> = commCosts.clone();
            let mut filteredCommCosts: metamodelica::List<Communication> = filteredCommCosts.clone();
            let mut highestCommCost: Communication = highestCommCost.clone();
            commCosts = metamodelica::arrayGet(iCommCosts.clone(), iParentComp)?;
            filteredCommCosts = List::filter1OnTrue(
                commCosts.clone(),
                (std::sync::Arc::new(move |__a0: Communication, __a1: metamodelica::List<i32>| {
                    getCommCostBetweenNodes1(__a0, &__a1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Communication, metamodelica::List<i32>) -> Result<bool> + 'static,
                    >),
                iChildComps.clone(),
            )?;
            let false = ((filteredCommCosts).is_empty()) else {
                return Err("pattern mismatch");
            };
            highestCommCost = getHighestCommCost(
                filteredCommCosts.clone(),
                Communication {
                    numberOfVars: 0,
                    integerVars: metamodelica::nil(),
                    floatVars: metamodelica::nil(),
                    booleanVars: metamodelica::nil(),
                    stringVars: metamodelica::nil(),
                    childNode: -1,
                    requiredTime: metamodelica::OrderedFloat(-1.0_f64),
                },
            );
            Ok((
                Some(highestCommCost.clone()),
                commCosts.clone(),
                filteredCommCosts.clone(),
                highestCommCost.clone(),
            ))
        })() {
            commCosts = __wb0;
            filteredCommCosts = __wb1;
            highestCommCost = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(None)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oHighestComm
}

fn getCommCostBetweenNodes1(mut iCommCost: Communication, mut iChildComps: &metamodelica::List<i32>) -> Result<bool> {
    let mut oResult: bool;
    let mut compIdx: i32;
    let Communication { childNode: __pa0, .. } = iCommCost;
    compIdx = metamodelica::Own::own(__pa0);
    oResult = List::exist1(iChildComps, &fnptr!(intEq, i32, i32), compIdx)?;
    Ok(oResult)
}

fn getHighestCommCost(mut iCommCosts: Communications, mut iHighestTuple: Communication) -> Communication {
    '__tco: loop {
        let mut highestCost: metamodelica::Real;
        let mut currentCost: metamodelica::Real;
        let mut head: Communication;
        let mut rest: Communications;
        ::match_deref::match_deref! { match &((iCommCosts, iHighestTuple.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: __esc_head @ Communication { requiredTime: currentCost, .. }, tail: __esc_rest }, Communication { requiredTime: highestCost, .. }) if (realGt(currentCost.clone(), highestCost.clone())) => {
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                { (iCommCosts, iHighestTuple) = (rest.clone(), head.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: __esc_head, tail: __esc_rest }, _) => {
                head = (*__esc_head).clone();
                rest = (*__esc_rest).clone();
                { (iCommCosts, iHighestTuple) = (rest.clone(), iHighestTuple); continue '__tco; }
            },
            _ => return iHighestTuple,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn sumUpExeCosts(mut iGraph: TaskGraph, mut iMeta: &TaskGraphMeta) -> Result<(i32, metamodelica::Real)> {
    let mut execCosts: (i32, metamodelica::Real);
    let mut cost1: i32;
    let mut cost2: metamodelica::Real;
    let mut comps: metamodelica::List<i32>;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCostLst: metamodelica::List<(i32, metamodelica::Real)>;
    execCosts = (match iMeta.clone() {
        TaskGraphMeta {
            inComps: mut __esc_inComps,
            exeCosts: mut __esc_exeCosts,
            ..
        } => {
            inComps = __esc_inComps.clone();
            exeCosts = __esc_exeCosts.clone();
            comps = List::flatten(List::map1(
                List::intRange(metamodelica::arrayLength(iGraph.clone())),
                &Array::getIndexFirst,
                inComps.clone(),
            )?)?;
            exeCostLst = List::map1(comps, &Array::getIndexFirst, exeCosts.clone())?;
            cost1 = List::fold(
                &(List::map(exeCostLst.clone(), &fnptr!(Util::tuple21, _))?),
                &fnptr!(intAdd, i32, i32),
                0,
            )?;
            cost2 = List::fold(
                &(List::map(exeCostLst, &fnptr!(Util::tuple22, _))?),
                &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
                metamodelica::OrderedFloat(0.0_f64),
            )?;
            (cost1, cost2)
        }
        _ => (0, metamodelica::OrderedFloat(0.0_f64)),
    });
    Ok(execCosts)
}

pub(crate) fn getAllSCCsOfGraph(mut iTaskGraphMeta: TaskGraphMeta) -> Result<metamodelica::List<i32>> {
    let mut oSccs: metamodelica::List<i32>;
    let mut taskIdx: i32 = 0;
    let mut inComps: metamodelica::Array<metamodelica::List<i32>>;
    let mut comps: metamodelica::List<i32>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut tmpSccs: metamodelica::List<i32>;
    tmpSccs = metamodelica::nil();
    let TaskGraphMeta {
        inComps: __pa0,
        nodeMark: __pa1,
        ..
    } = iTaskGraphMeta;
    inComps = metamodelica::Own::own(__pa0);
    nodeMark = metamodelica::Own::own(__pa1);
    for mut taskIdx in 1..=metamodelica::arrayLength(inComps.clone()) {
        comps = metamodelica::arrayGet(inComps.clone(), taskIdx)?;
        tmpSccs = List::append_reverse(&comps, tmpSccs);
    }
    oSccs = tmpSccs.reverse();
    Ok(oSccs)
}

//TODO: Remove
pub(crate) fn roundReal(mut inReal: metamodelica::Real, mut nIn: i32) -> Result<metamodelica::Real> {
    let mut outReal: metamodelica::Real;
    let mut real: metamodelica::Real;
    real = inReal * (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn) as f64));
    real = (real).floor();
    outReal = metamodelica::real_div_checked(
        real,
        (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn) as f64)),
    )?;
    Ok(outReal)
}

//--------------------------------------------------------
//  Get annotations from backendDAE and display in graphML
//--------------------------------------------------------
fn setAnnotationsForTasks(
    mut taskGraphInfo: TaskGraphMeta,
    mut backendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut annotInfoIn: metamodelica::Array<ArcStr>,
) -> Result<metamodelica::Array<ArcStr>> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*backendDAE);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    systs = metamodelica::Own::own(__pa0);
    (_, annotInfoOut) = List::fold1(
        &systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: TaskGraphMeta,
               __a2: (i32, metamodelica::Array<ArcStr>)| setAnnotationsForTasks1(&__a0, __a1, __a2),
        taskGraphInfo,
        (0, annotInfoIn.clone()),
    )?;
    Ok(annotInfoOut)
}

fn setAnnotationsForTasks1(
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut taskGraphInfo: TaskGraphMeta,
    mut infoIn: (i32, metamodelica::Array<ArcStr>),
) -> Result<(i32, metamodelica::Array<ArcStr>)> {
    let mut infoOut: (i32, metamodelica::Array<ArcStr>);
    let mut idx: i32;
    let mut annots: metamodelica::Array<ArcStr>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    (idx, annots) = infoIn;
    let __arc2 = &(*syst);
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &**__arc2;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    annots = List::fold3(
        &(List::intRange(BackendVariable::varsSize(&vars))),
        &move |__a0: i32,
               __a1: BackendDAE::Variables,
               __a2: TaskGraphMeta,
               __a3: i32,
               __a4: metamodelica::Array<ArcStr>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(setAnnotationsForVar(__a0, &__a1, &__a2, __a3, __a4))
        },
        vars.clone(),
        taskGraphInfo,
        idx,
        annots.clone(),
    )?;
    infoOut = (BackendVariable::varsSize(&vars) + idx, annots.clone());
    Ok(infoOut)
}

fn setAnnotationsForVar(
    mut backendVarIdx: i32,
    mut vars: &BackendDAE::Variables,
    mut taskGraphInfo: &TaskGraphMeta,
    mut eqSysOffset: i32,
    mut annotInfoIn: metamodelica::Array<ArcStr>,
) -> metamodelica::Array<ArcStr> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    annotInfoOut = 'mc: {
        let __mc_input = taskGraphInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let TaskGraphMeta {
                inComps: mut inComps,
                varCompMapping: mut varCompMapping,
                nodeMark: mut nodeMark,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut compIdx: i32;
            let mut taskIdx: i32;
            let mut annotString: ArcStr;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut annot: Option<metamodelica::Ref<SCode::Comment>>;
            var = BackendVariable::getVarAt(vars, backendVarIdx)?;
            BackendDump::printVar(&var)?;
            let true = (BackendVariable::hasAnnotation(&var)) else {
                return Err("pattern mismatch");
            };
            (compIdx, _, _) = metamodelica::arrayGet(varCompMapping.clone(), backendVarIdx + eqSysOffset)?;
            taskIdx = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            annot = BackendVariable::getAnnotationComment(&var)?;
            annotString = metamodelica::arrayGet(annotInfoIn.clone(), taskIdx)?;
            cr = BackendVariable::varCref(&var);
            annotString = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*annotString);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?);
                __mm_s.push_str(&*literal!(": "));
                __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(annot.clone()));
                __mm_s.push_str(&*literal!(") "));
                ArcStr::from(__mm_s)
            };
            metamodelica::arrayUpdate(annotInfoIn.clone(), taskIdx, annotString.clone())?;
            Ok(annotInfoIn.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(annotInfoIn.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    annotInfoOut
}

//--------------------------------------------------------
//  Append removed equations like asserts to the DAE graph
//--------------------------------------------------------
pub(crate) fn appendRemovedEquations(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut graphIn: TaskGraph,
    mut graphDataIn: TaskGraphMeta,
) -> (TaskGraph, TaskGraphMeta) {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    (graphOut, graphDataOut) = 'mc: {
        let __mc_input = graphDataIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut numNewComps: i32;
            let mut newComps: metamodelica::List<i32>;
            let mut nodeVarLst: metamodelica::List<metamodelica::List<(i32, i32)>>;
            let mut varCompMap: metamodelica::Array<(i32, i32, i32)>;
            let mut graph: TaskGraph;
            let mut graphData: TaskGraphMeta;
            let mut remEqs: metamodelica::Ref<
                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
            >;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut crefsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
            let mut inComps1: metamodelica::Array<metamodelica::List<i32>>;
            let mut inComps2: metamodelica::Array<metamodelica::List<i32>>;
            let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
            let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
            let mut compParamMapping1: metamodelica::Array<metamodelica::List<i32>>;
            let mut compNames1: metamodelica::Array<ArcStr>;
            let mut compNames2: metamodelica::Array<ArcStr>;
            let mut compDescs1: metamodelica::Array<ArcStr>;
            let mut compDescs2: metamodelica::Array<ArcStr>;
            let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut commCosts1: metamodelica::Array<metamodelica::List<Communication>>;
            let mut nodeMark1: metamodelica::Array<i32>;
            let mut nodeMark2: metamodelica::Array<i32>;
            let mut compInformations1: metamodelica::Array<ComponentInfo>;
            let mut compInformations2: metamodelica::Array<ComponentInfo>;
            let __arc1 = dae.clone();
            let BackendDAE::DAE { shared: __pa0, .. } = &*__arc1;
            shared = metamodelica::Own::own(__pa0);
            remEqs = BackendDAEUtil::collapseRemovedEqs(&dae)?;
            let TaskGraphMeta {
                varCompMapping: __pa2, ..
            } = &graphDataIn;
            varCompMap = metamodelica::Own::own(__pa2);
            eqLst = BackendEquation::equationList(remEqs.clone())?;
            numNewComps = ((eqLst).len() as i32);
            let true = (intNe(numNewComps, 0)) else {
                return Err("pattern mismatch");
            };
            crefsLst = List::map(eqLst.clone(), &BackendEquation::equationCrefs)?;
            nodeVarLst = List::map2(crefsLst.clone(), &getNodeForCrefLst, dae.clone(), varCompMap.clone())?;
            let TaskGraphMeta {
                inComps: __pa3,
                varCompMapping: __pa4,
                eqCompMapping: __pa5,
                compParamMapping: __pa6,
                compNames: __pa7,
                compDescs: __pa8,
                exeCosts: __pa9,
                commCosts: __pa10,
                nodeMark: __pa11,
                compInformations: __pa12,
            } = &graphDataIn;
            inComps1 = metamodelica::Own::own(__pa3);
            varCompMapping1 = metamodelica::Own::own(__pa4);
            eqCompMapping1 = metamodelica::Own::own(__pa5);
            compParamMapping1 = metamodelica::Own::own(__pa6);
            compNames1 = metamodelica::Own::own(__pa7);
            compDescs1 = metamodelica::Own::own(__pa8);
            exeCosts1 = metamodelica::Own::own(__pa9);
            commCosts1 = metamodelica::Own::own(__pa10);
            nodeMark1 = metamodelica::Own::own(__pa11);
            compInformations1 = metamodelica::Own::own(__pa12);
            graph = metamodelica::arrayAppend(graphIn.clone(), arrayCreate(numNewComps, metamodelica::nil()));
            newComps = List::intRange2(
                metamodelica::arrayLength(graphIn.clone()) + 1,
                metamodelica::arrayLength(graphIn.clone()) + numNewComps,
            );
            graph = List::threadFold(&nodeVarLst, newComps.clone(), &addEdgesToGraph, graph.clone())?;
            inComps2 = metamodelica::arrayFromVec(
                List::map(newComps.clone(), &fnptr!(List::create, _))?
                    .into_iter()
                    .cloned()
                    .collect(),
            );
            compNames2 = arrayCreate(numNewComps, literal!("assert"));
            compDescs2 = metamodelica::arrayFromVec(
                List::map(eqLst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| {
                    BackendDump::equationString(&__a0)
                })?
                .into_iter()
                .cloned()
                .collect(),
            );
            nodeMark2 = arrayCreate(numNewComps, -2);
            exeCosts2 = metamodelica::arrayFromVec(
                List::map1(
                    eqLst.clone(),
                    &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                           __a1: metamodelica::Ref<BackendDAE::Shared>| {
                        estimateEquationCosts(__a0, &__a1)
                    },
                    shared.clone(),
                )?
                .into_iter()
                .cloned()
                .collect(),
            );
            compInformations2 = arrayCreate(
                numNewComps,
                ComponentInfo {
                    isPartOfODESystem: false,
                    isPartOfZeroFuncSystem: false,
                    isRemovedComponent: true,
                },
            );
            inComps1 = metamodelica::arrayAppend(inComps1.clone(), inComps2.clone());
            compNames1 = metamodelica::arrayAppend(compNames1.clone(), compNames2.clone());
            compDescs1 = metamodelica::arrayAppend(compDescs1.clone(), compDescs2.clone());
            nodeMark1 = metamodelica::arrayAppend(nodeMark1.clone(), nodeMark2.clone());
            exeCosts1 = metamodelica::arrayAppend(exeCosts1.clone(), exeCosts2.clone());
            compInformations1 = metamodelica::arrayAppend(compInformations1.clone(), compInformations2.clone());
            commCosts1 = List::threadFold1(
                &nodeVarLst,
                newComps.clone(),
                &move |__a0: metamodelica::List<(i32, i32)>,
                       __a1: i32,
                       __a2: metamodelica::Real,
                       __a3: metamodelica::Array<metamodelica::List<Communication>>| {
                    setCommCostsToParent(&__a0, __a1, __a2, __a3)
                },
                metamodelica::OrderedFloat(74.0_f64),
                commCosts1.clone(),
            )?;
            graphData = TaskGraphMeta {
                inComps: inComps1.clone(),
                varCompMapping: varCompMapping1.clone(),
                eqCompMapping: eqCompMapping1.clone(),
                compParamMapping: compParamMapping1.clone(),
                compNames: compNames1.clone(),
                compDescs: compDescs1.clone(),
                exeCosts: exeCosts1.clone(),
                commCosts: commCosts1.clone(),
                nodeMark: nodeMark1.clone(),
                compInformations: compInformations1.clone(),
            };
            Ok((graph.clone(), graphData.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((graphIn.clone(), graphDataIn.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (graphOut, graphDataOut)
}

fn estimateEquationCosts(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut sharedIn: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(i32, metamodelica::Real)> {
    let mut tplOut: (i32, metamodelica::Real);
    let mut numAdd: i32;
    let mut numMul: i32;
    let mut numDiv: i32;
    let mut numTrig: i32;
    let mut numRel: i32;
    let mut numOth: i32;
    let mut numFuncs: i32;
    let mut numLog: i32;
    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(
        eqIn,
        (std::sync::Arc::new({
            let __pe_b1 = sharedIn.clone();
            move |__pe_a0, __pe_a2| BackendDAEOptimize::countOperationsExp(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (i32, i32, i32, i32, i32, i32, i32, i32),
                    )
                        -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))>
                    + 'static,
            >),
        (0, 0, 0, 0, 0, 0, 0, 0),
    )?;
    numAdd = metamodelica::Own::own(__pa0);
    numMul = metamodelica::Own::own(__pa1);
    numDiv = metamodelica::Own::own(__pa2);
    numTrig = metamodelica::Own::own(__pa3);
    numRel = metamodelica::Own::own(__pa4);
    numLog = metamodelica::Own::own(__pa5);
    numOth = metamodelica::Own::own(__pa6);
    numFuncs = metamodelica::Own::own(__pa7);
    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::NO_COMP {
        numAdds: numAdd,
        numMul: numMul,
        numDiv: numDiv,
        numTrig: numTrig,
        numRelations: numRel,
        numLog: numLog,
        numOth: numOth,
        funcCalls: numFuncs,
    });
    tplOut = calculateCosts(&compInfo);
    Ok(tplOut)
}

fn printNodeVars(mut nodes: metamodelica::List<(i32, i32)>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(nodes, &fnptr!(printNodeVars1, (i32, i32)))?,
            literal!(" | "),
        ));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn printNodeVars1(mut node: (i32, i32)) -> ArcStr {
    let mut s: ArcStr;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*intString(Util::tuple21(node)));
        __mm_s.push_str(&*literal!(","));
        __mm_s.push_str(&*intString(Util::tuple22(node)));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    s
}

fn setCommCostsToParent(
    mut parents: &metamodelica::List<(i32, i32)>,
    mut child: i32,
    mut reqCycles: metamodelica::Real,
    mut commCostsIn: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Array<metamodelica::List<Communication>>> {
    let mut commCostsOut: metamodelica::Array<metamodelica::List<Communication>>;
    commCostsOut = List::fold2(parents, &setCommCosts, child, reqCycles, commCostsIn.clone())?;
    Ok(commCostsOut)
}

fn setCommCosts(
    mut parent: (i32, i32),
    mut child: i32,
    mut reqCycles: metamodelica::Real,
    mut commCostsIn: metamodelica::Array<metamodelica::List<Communication>>,
) -> Result<metamodelica::Array<metamodelica::List<Communication>>> {
    let mut commCostsOut: metamodelica::Array<metamodelica::List<Communication>>;
    let mut row: Communications;
    let mut parentNodeIdx: i32;
    let mut varIdx: i32;
    (parentNodeIdx, varIdx) = parent;
    row = metamodelica::arrayGet(commCostsIn.clone(), parentNodeIdx)?;
    row = List::filter1OnTrue(
        row,
        (std::sync::Arc::new(fnptr!(isCommunicationChildEqualToIdx, Communication, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32) -> Result<bool> + 'static>),
        child,
    )?;
    row = metamodelica::cons(
        Communication {
            numberOfVars: 1,
            integerVars: metamodelica::nil(),
            floatVars: list![varIdx],
            booleanVars: metamodelica::nil(),
            stringVars: metamodelica::nil(),
            childNode: child,
            requiredTime: reqCycles,
        },
        row,
    );
    commCostsOut = metamodelica::arrayUpdate(commCostsIn.clone(), parentNodeIdx, row)?;
    Ok(commCostsOut)
}

fn isCommunicationChildEqualToIdx(mut iComm: Communication, mut iIdx: i32) -> bool {
    let mut isEq: bool;
    let mut childNode: i32;
    let Communication { childNode: __pa0, .. } = iComm;
    childNode = metamodelica::Own::own(__pa0);
    isEq = intNe(childNode, iIdx);
    isEq
}

fn addEdgesToGraph(
    mut parents: metamodelica::List<(i32, i32)>,
    mut child: i32,
    mut graphIn: TaskGraph,
) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    graphOut = List::fold1(
        &(List::map(parents, &fnptr!(Util::tuple21, _))?),
        &addEdgeToGraph,
        child,
        graphIn.clone(),
    )?;
    Ok(graphOut)
}

fn addEdgeToGraph(mut parent: i32, mut child: i32, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut row: metamodelica::List<i32>;
    row = metamodelica::arrayGet(graphIn.clone(), parent)?;
    row = List::unique(&(metamodelica::cons(child, row)));
    graphOut = metamodelica::arrayUpdate(graphIn.clone(), parent, row)?;
    Ok(graphOut)
}

fn getNodeForCrefLst(
    mut iCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut iDae: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iVarCompMap: metamodelica::Array<(i32, i32, i32)>,
) -> Result<metamodelica::List<(i32, i32)>> {
    let mut oNodeVarLst: metamodelica::List<(i32, i32)>;
    let mut tmpNodeVarLst: metamodelica::List<(i32, i32)>;
    tmpNodeVarLst = List::map2(
        iCrefs,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
               __a1: metamodelica::Ref<BackendDAE::BackendDAE>,
               __a2: metamodelica::Array<(i32, i32, i32)>| getNodeForCref(&__a0, &__a1, __a2),
        iDae,
        iVarCompMap.clone(),
    )?;
    oNodeVarLst = List::filterOnTrue(
        tmpNodeVarLst,
        (std::sync::Arc::new(fnptr!(nodeIsDependent, (i32, i32)))
            as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<bool> + 'static>),
    )?;
    Ok(oNodeVarLst)
}

fn nodeIsDependent(mut node: (i32, i32)) -> bool {
    let mut dep: bool;
    let mut tpl1: i32;
    (tpl1, _) = node;
    dep = intNe(tpl1, -1);
    dep
}

fn getNodeForCref(
    mut iCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut iDae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(i32, i32)> {
    let mut oNodeVarIdx: (i32, i32);
    let mut eqSysIdx: i32;
    let mut varIdx: i32;
    let mut nodeIdx: i32;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*iDae);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    eqSystems = metamodelica::Own::own(__pa0);
    (eqSysIdx, varIdx, _) = getNodeForCref1(&eqSystems, iCref, 1)?;
    nodeIdx = getNodeForVarIdx(varIdx, eqSysIdx, iVarCompMapping.clone(), varIdx)?;
    oNodeVarIdx = (nodeIdx, varIdx);
    Ok(oNodeVarIdx)
}

fn getNodeForCref1(
    mut eqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut eqSysIdxIn: i32,
) -> Result<(i32, i32, bool)> {
    let mut eqSysIdx: i32;
    let mut varIdx: i32;
    let mut found: bool;
    (eqSysIdx, varIdx, found) = 'mc: {
        let __mc_input = &**eqSystems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }, tail: _ } => {
                    let mut b: bool;
                    let mut esIdx: i32;
                    let mut vIdx: i32;
                    let mut lst: metamodelica::List<i32>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (varLst, lst) = BackendVariable::getVar(cref.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    if intNe(((lst).len() as i32), 1) {
                        metamodelica::print(literal!("Check if there is a assert or something that is dependent of arrayEquations"));
                    }
                    if BackendVariable::isStateVar(&((varLst).head().cloned()?)) {
                        (esIdx, vIdx, b) = (-1, -1, false);
                    } else {
                        (esIdx, vIdx, b) = (eqSysIdxIn, (lst).head().cloned()?, true);
                    }
                    Ok((esIdx, vIdx, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { .. }, tail: rest } => {
                    let mut b: bool;
                    let mut esIdx: i32;
                    let mut vIdx: i32;
                    (esIdx, vIdx, b) = getNodeForCref1(metamodelica::AsArg::as_arg(&rest), cref, eqSysIdxIn + 1)?;
                    Ok((esIdx, vIdx, b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((-1, -1, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((eqSysIdx, varIdx, found))
}

fn getNodeForVarIdx(
    mut varIdx: i32,
    mut eqSysIdx: i32,
    mut varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut inTryThisIndex: i32,
) -> Result<i32> {
    let mut node: i32 = 0;
    let mut offset: i32;
    let mut eqSys: i32;
    let mut tryThisIndex: i32 = inTryThisIndex;
    let mut n: i32 = 0;
    let mut arrayLengthVarCompMapping: i32;
    arrayLengthVarCompMapping = metamodelica::arrayLength(varCompMapping.clone());
    loop {
        if tryThisIndex >= 1 && tryThisIndex <= arrayLengthVarCompMapping {
            (node, eqSys, offset) = metamodelica::arrayGet(varCompMapping.clone(), tryThisIndex)?;
            if eqSys == eqSysIdx {
                node = node + varIdx - 1;
                return Ok(node);
            } else {
                tryThisIndex = offset + 2;
            }
        } else if varIdx == -1 && eqSysIdx == -1 {
            node = -1;
            return Ok(node);
        } else {
            metamodelica::print(literal!("HpcOmTaskGraph.getNodeForVarIdx failed\n"));
        }
        n = n + 1;
        if n > arrayLengthVarCompMapping {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("HpcOmTaskGraph.getNodeForVarIdx"));
                    __mm_s.push_str(&*literal!(" failed (there is a loop somewhere)"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/HpcOmTaskGraph.mo"),
            )?;
            return Err("fail");
        }
    }
    Ok(node)
}

//----------------------------
//  MULTIRATE PARTITIONING
//----------------------------
pub(crate) fn multirate_partitioning(
    mut odeGraph: TaskGraph,
    mut odeGraphData: &TaskGraphMeta,
    mut backendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<SimCode::PartitionData> {
    let mut partitionDataOut: SimCode::PartitionData;
    let mut stateTaskAssign: metamodelica::Array<metamodelica::List<i32>>;
    let mut stateTasks: metamodelica::List<i32>;
    let mut tasksPerLevel: metamodelica::List<metamodelica::List<i32>>;
    let mut partitions: metamodelica::List<metamodelica::List<i32>>;
    let mut odeGraphT: TaskGraph;
    let mut numPartitions: i32;
    let mut activatorsForPartitions: metamodelica::List<metamodelica::List<i32>>;
    let mut stateToActivators: metamodelica::List<i32>;
    tasksPerLevel = getLevelNodes(odeGraph.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("tasksPerLevel "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(tasksPerLevel.clone(), &intLstString)?,
            literal!("\n"),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    stateTasks = getLeafNodes(odeGraph.clone())?;
    stateTasks = multirate_orderStateTasksInSimVarStateOrder(stateTasks, odeGraphData, backendDAE, simCode)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("stateTasks "));
        __mm_s.push_str(&*intLstString(stateTasks.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    odeGraphT =
        AdjacencyMatrix::transposeAdjacencyMatrix(odeGraph.clone(), metamodelica::arrayLength(odeGraph.clone()))?;
    stateTaskAssign = multirate_assignTasksToStates(tasksPerLevel, &stateTasks, odeGraphT.clone())?;
    dumpStateAssign(stateTaskAssign.clone())?;
    partitions = multirate_getPartitions(stateTaskAssign.clone(), stateTasks.clone(), odeGraphT.clone())?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("PARTITIONS :\n"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(partitions.clone(), &intLstString)?,
            literal!("\n"),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    activatorsForPartitions = List::mapMap(
        partitions.clone(),
        &listHead,
        &({
            let __pe_b1 = stateTaskAssign.clone();
            move |__pe_a0| Array::getIndexFirst(__pe_a0, __pe_b1.clone())
        }),
    )?;
    partitions = List::map1(partitions, &getSimEqsIdxLstForSCCIdxLst, sccSimEqMapping.clone())?;
    numPartitions = ((partitions).len() as i32);
    stateToActivators = List::intRange(((stateTasks).len() as i32));
    partitionDataOut = SimCode::PartitionData {
        numPartitions: numPartitions,
        partitions: partitions,
        activatorsForPartitions: activatorsForPartitions,
        stateToActivators: stateToActivators,
    };
    dumpPartitionData(partitionDataOut.clone())?;
    Ok(partitionDataOut)
}

fn multirate_orderStateTasksInSimVarStateOrder(
    mut stateTasks: metamodelica::List<i32>,
    mut taskGraphData: &TaskGraphMeta,
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::List<i32>> {
    let mut orderedTasks: metamodelica::List<i32>;
    let mut state: i32 = 0;
    let mut compIdx: i32;
    let mut eqSysIdx: i32;
    let mut offset: i32;
    let mut varIdx: i32;
    let mut simVarIdx: i32;
    let mut simVarIdxs: metamodelica::List<i32>;
    let mut order: metamodelica::List<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut simVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let __arc1 = &(*dae);
    let BackendDAE::DAE { eqs: __pa0, .. } = &**__arc1;
    eqSystems = metamodelica::Own::own(__pa0);
    simVarIdxs = metamodelica::nil();
    for mut state in &*stateTasks {
        let mut state = state.clone();
        compIdx = (metamodelica::arrayGet(taskGraphData.inComps.clone(), state)?)
            .head()
            .cloned()?;
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(Array::findFirstOnTrueWithIdx(taskGraphData.varCompMapping.clone(), &({ let __pe_b1 = compIdx; move |__pe_a0| Ok(varMappingTupleCompEqual(__pe_a0, __pe_b1.clone())) }))?) {
            (Some((__pa2, __pa3, __pa4)), __pa5) => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        compIdx = metamodelica::Own::own(__pa2);
        eqSysIdx = metamodelica::Own::own(__pa3);
        offset = metamodelica::Own::own(__pa4);
        varIdx = metamodelica::Own::own(__pa5);
        eqSys = (eqSystems).get(eqSysIdx)?;
        varIdx = varIdx - offset;
        var = BackendVariable::getVarAt(&eqSys.orderedVars, varIdx)?;
        cref = var.varName.clone();
        let __pa6 = ::match_deref::match_deref! { match &(SimCodeUtil::getSimVars2Crefs(&(list![cref]), &(simCode.crefToSimVarHT.clone()))) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil } => __pa6.clone(),
            _ => return Err("pattern mismatch"),
        } };
        simVar = metamodelica::Own::own(__pa6);
        simVarIdx = simVar.index.clone();
        simVarIdxs = metamodelica::cons(simVarIdx, simVarIdxs);
    }
    (_, order) = HpcOmScheduler::quicksortWithOrder(List::map(simVarIdxs.reverse(), &fnptr!(intReal, i32))?)?;
    orderedTasks = List::map1(
        order,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        stateTasks,
    )?;
    Ok(orderedTasks)
}

fn varMappingTupleCompEqual(mut tpl: (i32, i32, i32), mut compIdx: i32) -> bool {
    let mut compEqual: bool;
    compEqual = intEq(compIdx, Util::tuple31(tpl));
    compEqual
}

fn getSimEqIdxForSCCIdx(
    mut sccIdx: i32,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<i32> {
    let mut simEqIdx: i32;
    simEqIdx = (metamodelica::arrayGet(sccSimEqMapping.clone(), sccIdx)?)
        .head()
        .cloned()?;
    Ok(simEqIdx)
}

fn getSimEqsIdxLstForSCCIdxLst(
    mut sccIdxs: metamodelica::List<i32>,
    mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut simEqIdxs: metamodelica::List<i32>;
    simEqIdxs = List::map1(sccIdxs, &getSimEqIdxForSCCIdx, sccSimEqMapping.clone())?;
    Ok(simEqIdxs)
}

fn multirate_getPartitions(
    mut stateTaskAssign: metamodelica::Array<metamodelica::List<i32>>,
    mut stateTasks: metamodelica::List<i32>,
    mut odeGraphT: TaskGraph,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut partitions: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut numStates: i32;
    let mut numAssigns: i32 = 0;
    let mut leaveNodes: metamodelica::List<i32>;
    let mut samePartTasks: metamodelica::List<i32>;
    let mut partition: metamodelica::List<i32>;
    let mut otherPartTasks: metamodelica::List<i32>;
    let mut stateAss: metamodelica::List<i32>;
    let mut visitedTasks: metamodelica::Array<i32>;
    let mut leaveNodesWithNassigns: metamodelica::Array<metamodelica::List<i32>>;
    visitedTasks = arrayCreate(metamodelica::arrayLength(odeGraphT.clone()), -1);
    numStates = ((stateTasks).len() as i32);
    leaveNodesWithNassigns = arrayCreate(numStates, metamodelica::nil());
    metamodelica::arrayUpdate(leaveNodesWithNassigns.clone(), 1, stateTasks)?;
    for mut numAssigns in &*List::intRange(numStates) {
        let mut numAssigns = numAssigns.clone();
        leaveNodes = metamodelica::arrayGet(leaveNodesWithNassigns.clone(), numAssigns)?;
        leaveNodes = List::unique(&leaveNodes);
        while !((leaveNodes).is_empty()) {
            stateAss = metamodelica::arrayGet(stateTaskAssign.clone(), (leaveNodes).head().cloned()?)?;
            (samePartTasks, leaveNodes) = List::separateOnTrue(
                &leaveNodes,
                &({
                    let __pe_b1 = stateTaskAssign.clone();
                    let __pe_b2 = stateAss.clone();
                    move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?;
            (partition, otherPartTasks) = multirate_getPartitionPredecessors(
                samePartTasks,
                odeGraphT.clone(),
                stateTaskAssign.clone(),
                &stateAss,
                visitedTasks.clone(),
            )?;
            partition = List::sort(
                partition,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            multirate_dispatchLeaveNodes(&otherPartTasks, stateTaskAssign.clone(), leaveNodesWithNassigns.clone())?;
            partitions = metamodelica::cons(partition, partitions);
        }
    }
    Ok(partitions)
}

fn multirate_dispatchLeaveNodes(
    mut tasksIn: &metamodelica::List<i32>,
    mut stateTaskAssign: metamodelica::Array<metamodelica::List<i32>>,
    mut leaveNodesWithNassigns: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut numAss: i32;
    let mut stateAss: metamodelica::List<i32>;
    let mut leaveNodes: metamodelica::List<i32>;
    for mut task in &**tasksIn {
        stateAss = metamodelica::arrayGet(stateTaskAssign.clone(), task.clone())?;
        numAss = ((stateAss).len() as i32);
        leaveNodes = metamodelica::arrayGet(leaveNodesWithNassigns.clone(), numAss)?;
        leaveNodes = metamodelica::cons(task.clone(), leaveNodes);
        metamodelica::arrayUpdate(leaveNodesWithNassigns.clone(), numAss, leaveNodes)?;
    }
    Ok(())
}

fn multirate_getPartitionPredecessors(
    mut leavesIn: metamodelica::List<i32>,
    mut odeGraphT: TaskGraph,
    mut stateTaskAssign: metamodelica::Array<metamodelica::List<i32>>,
    mut refStateAssign: &metamodelica::List<i32>,
    mut visitedTasks: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut partitionTasks: metamodelica::List<i32> = metamodelica::nil();
    let mut otherLeaveNodes: metamodelica::List<i32> = metamodelica::nil();
    let mut cont: bool;
    let mut task: i32;
    let mut tasks: metamodelica::List<i32>;
    let mut predecessors: metamodelica::List<i32>;
    let mut samePartTasks: metamodelica::List<i32>;
    let mut otherLeaves: metamodelica::List<i32>;
    cont = true;
    tasks = leavesIn;
    while cont {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tasks) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        task = metamodelica::Own::own(__pa0);
        tasks = metamodelica::Own::own(__pa1);
        predecessors = metamodelica::arrayGet(odeGraphT.clone(), task)?;
        predecessors = List::filter1OnTrue(
            predecessors,
            (std::sync::Arc::new(taskIsNotVisited)
                as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>),
            visitedTasks.clone(),
        )?;
        (samePartTasks, otherLeaves) = List::separateOnTrue(
            &predecessors,
            &({
                let __pe_b1 = stateTaskAssign.clone();
                let __pe_b2 = refStateAssign.clone();
                move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        partitionTasks = metamodelica::cons(task, partitionTasks);
        partitionTasks = listAppend(samePartTasks.clone(), partitionTasks);
        tasks = listAppend(samePartTasks.clone(), tasks);
        otherLeaveNodes = listAppend(otherLeaves.clone(), otherLeaveNodes);
        metamodelica::arrayUpdate(visitedTasks.clone(), task, 0)?;
        List::map2_0(&samePartTasks, &Array::updateIndexFirst, 0, visitedTasks.clone())?;
        List::map2_0(&otherLeaves, &Array::updateIndexFirst, 0, visitedTasks.clone())?;
        if (tasks).is_empty() {
            cont = false;
        }
    }
    partitionTasks = List::unique(&partitionTasks);
    otherLeaveNodes = List::unique(&otherLeaveNodes);
    Ok((partitionTasks, otherLeaveNodes))
}

fn taskIsNotVisited(mut task: i32, mut visitedTasks: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNotVisited: bool;
    isNotVisited = intEq(-1, metamodelica::arrayGet(visitedTasks.clone(), task)?);
    Ok(isNotVisited)
}

fn hasSameStateAssign(
    mut task: i32,
    mut stateTaskAssign: metamodelica::Array<metamodelica::List<i32>>,
    mut refStateAssign: metamodelica::List<i32>,
) -> Result<bool> {
    let mut sameStateAssign: bool;
    sameStateAssign = List::isEqual(
        metamodelica::arrayGet(stateTaskAssign.clone(), task)?,
        refStateAssign,
        true,
    )?;
    Ok(sameStateAssign)
}

fn multirate_assignTasksToStates(
    mut tasksPerLevel: metamodelica::List<metamodelica::List<i32>>,
    mut stateTasks: &metamodelica::List<i32>,
    mut odeGraphT: TaskGraph,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut stateTaskAssignOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskIdx: i32;
    let mut assignments: metamodelica::List<i32>;
    let mut predecessors: metamodelica::List<i32>;
    stateTaskAssignOut = arrayCreate(metamodelica::arrayLength(odeGraphT.clone()), metamodelica::nil());
    taskIdx = 1;
    for mut task in &**stateTasks {
        stateTaskAssignOut = metamodelica::arrayUpdate(stateTaskAssignOut.clone(), task.clone(), list![taskIdx])?;
        taskIdx = taskIdx + 1;
    }
    for mut levelTasks in &*tasksPerLevel.reverse() {
        for mut task in &*levelTasks.clone() {
            assignments = metamodelica::arrayGet(stateTaskAssignOut.clone(), task.clone())?;
            predecessors = metamodelica::arrayGet(odeGraphT.clone(), task.clone())?;
            stateTaskAssignOut = List::fold1(
                &predecessors,
                &appendToElementUnique,
                assignments,
                stateTaskAssignOut.clone(),
            )?;
        }
    }
    stateTaskAssignOut = Array::map1(
        stateTaskAssignOut.clone(),
        &List::sort,
        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    Ok(stateTaskAssignOut)
}

fn appendToElementUnique<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(
    mut inIndex: i32,
    mut inElements: metamodelica::List<T>,
    mut inArray: metamodelica::Array<metamodelica::List<T>>,
) -> Result<metamodelica::Array<metamodelica::List<T>>> {
    let mut outArray: metamodelica::Array<metamodelica::List<T>>;
    outArray = metamodelica::arrayUpdate(
        inArray.clone(),
        inIndex,
        List::unique(
            &(listAppend(
                ({
                    let __elt = (*metamodelica::index_checked(&inArray.borrow(), inIndex)?).clone();
                    __elt
                }),
                inElements,
            )),
        ),
    )?;
    Ok(outArray)
}

fn dumpStateAssign(mut stateAssign: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("stateAssign "));
        __mm_s.push_str(&*stringDelimitList(
            List::mapArray(stateAssign.clone(), &intLstString)?,
            literal!("\n"),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn dumpPartitionData(mut partData: SimCode::PartitionData) -> Result<()> {
    let mut numPartitions: i32;
    let mut act: i32;
    let mut part: i32 = 0;
    let mut state: i32 = 0;
    let mut activatorsForPartitions: metamodelica::List<metamodelica::List<i32>>;
    let mut partitions: metamodelica::List<metamodelica::List<i32>>;
    let mut stateToActivators: metamodelica::List<i32>;
    let SimCode::PARTITIONDATA {
        numPartitions: __pa0,
        partitions: __pa1,
        activatorsForPartitions: __pa2,
        stateToActivators: __pa3,
    } = partData;
    numPartitions = metamodelica::Own::own(__pa0);
    partitions = metamodelica::Own::own(__pa1);
    activatorsForPartitions = metamodelica::Own::own(__pa2);
    stateToActivators = metamodelica::Own::own(__pa3);
    metamodelica::print(literal!("Multirate Partition Data\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(numPartitions));
        __mm_s.push_str(&*literal!(" partitions:\n"));
        ArcStr::from(__mm_s)
    });
    act = 1;
    for mut state in &*stateToActivators {
        let mut state = state.clone();
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("activator "));
            __mm_s.push_str(&*intString(act));
            __mm_s.push_str(&*literal!(" is state "));
            __mm_s.push_str(&*intString(state));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        act = act + 1;
    }
    metamodelica::print(literal!("\n"));
    for mut part in 1..=numPartitions {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("activators: "));
            __mm_s.push_str(&*intLstString((activatorsForPartitions).get(part)?)?);
            __mm_s.push_str(&*literal!("\t\t\t\tderStateTasks: "));
            __mm_s.push_str(&*intLstString(List::map1(
                (activatorsForPartitions).get(part)?,
                &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
                stateToActivators.clone(),
            )?)?);
            __mm_s.push_str(&*literal!("\t\t\t\tnodes: \t"));
            __mm_s.push_str(&*intLstString((partitions).get(part)?)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

//----------------------------
//  MAPPING FUNCTIONS
//----------------------------
pub(crate) fn setUpHpcOmMapping(
    mut daeIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut simCodeIn: &metamodelica::Ref<SimCode::SimCode>,
    mut lastEqMappingIdx: i32,
    mut equationSccMappingIn: metamodelica::List<(i32, i32)>,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut simeqCompMapping: metamodelica::Array<i32>;
    let mut sccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut daeSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut highestSccIdx: i32;
    let mut compCountPlusDummy: i32;
    let mut equationSccMapping: metamodelica::List<(i32, i32)>;
    let mut equationSccMapping1: metamodelica::List<(i32, i32)>;
    let mut allComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    (allComps, _) = getSystemComponents(daeIn)?;
    highestSccIdx = findHighestSccIdxInMapping(equationSccMappingIn.clone(), -1);
    compCountPlusDummy = ((allComps).len() as i32) + 1;
    equationSccMapping1 = removeDummyStateFromMapping(&equationSccMappingIn)?;
    equationSccMapping = if (intEq(highestSccIdx, compCountPlusDummy)) {
        equationSccMapping1
    } else {
        equationSccMappingIn
    };
    sccSimEqMapping = convertToSccSimEqMapping(&equationSccMapping, ((allComps).len() as i32))?;
    simeqCompMapping = convertToSimeqCompMapping(&equationSccMapping, lastEqMappingIdx)?;
    daeSccSimEqMapping = metamodelica::arrayFromVec(
        List::map(
            SimCodeUtil::getRemovedEquationSimEqSysIdxes(simCodeIn)?,
            &fnptr!(List::create, _),
        )?
        .into_iter()
        .cloned()
        .collect(),
    );
    daeSccSimEqMapping = metamodelica::arrayAppend(sccSimEqMapping.clone(), daeSccSimEqMapping.clone());
    Ok((simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping))
}

fn findHighestSccIdxInMapping(mut iEquationSccMapping: metamodelica::List<(i32, i32)>, mut iHighestIndex: i32) -> i32 {
    '__tco: loop {
        let mut eqIdx: i32;
        let mut sccIdx: i32;
        let mut rest: metamodelica::List<(i32, i32)>;
        ::match_deref::match_deref! { match &(iEquationSccMapping) {
            Deref @ metamodelica::ListNode::Cons { head: (__esc_eqIdx, sccIdx), tail: __esc_rest } if (intGt(sccIdx.clone(), iHighestIndex)) => {
                eqIdx = (*__esc_eqIdx).clone();
                rest = (*__esc_rest).clone();
                { (iEquationSccMapping, iHighestIndex) = (rest.clone(), sccIdx.clone()); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: (__esc_eqIdx, __esc_sccIdx), tail: __esc_rest } => {
                eqIdx = (*__esc_eqIdx).clone();
                sccIdx = (*__esc_sccIdx).clone();
                rest = (*__esc_rest).clone();
                { (iEquationSccMapping, iHighestIndex) = (rest.clone(), iHighestIndex); continue '__tco; }
            },
            _ => return iHighestIndex,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn removeDummyStateFromMapping(
    mut iEquationSccMapping: &metamodelica::List<(i32, i32)>,
) -> Result<metamodelica::List<(i32, i32)>> {
    let mut oEquationSccMapping: metamodelica::List<(i32, i32)>;
    oEquationSccMapping = List::fold(
        iEquationSccMapping,
        &fnptr!(removeDummyStateFromMapping1, (i32, i32), metamodelica::List<(i32, i32)>),
        metamodelica::nil(),
    )?;
    Ok(oEquationSccMapping)
}

fn removeDummyStateFromMapping1(
    mut iTuple: (i32, i32),
    mut iNewList: metamodelica::List<(i32, i32)>,
) -> metamodelica::List<(i32, i32)> {
    let mut oNewList: metamodelica::List<(i32, i32)>;
    let mut eqIdx: i32;
    let mut sccIdx: i32;
    let mut newElem: (i32, i32);
    oNewList = (match iTuple {
        (mut __esc_eqIdx, mut sccIdx) if (intEq(sccIdx, 1)) => {
            eqIdx = __esc_eqIdx.clone();
            iNewList
        }
        (mut __esc_eqIdx, mut __esc_sccIdx) => {
            eqIdx = __esc_eqIdx.clone();
            sccIdx = __esc_sccIdx.clone();
            newElem = (eqIdx, sccIdx - 1);
            metamodelica::cons(newElem, iNewList)
        }
        _ => {
            metamodelica::print(literal!("removeDummyStateFromMapping1 failed\n"));
            iNewList
        }
    });
    oNewList
}

fn convertToSccSimEqMapping(
    mut iMapping: &metamodelica::List<(i32, i32)>,
    mut numOfSccs: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpMapping: metamodelica::Array<metamodelica::List<i32>>;
    tmpMapping = arrayCreate(numOfSccs, metamodelica::nil());
    List::fold(iMapping, &convertToSccSimEqMapping1, tmpMapping.clone())?;
    oMapping = tmpMapping.clone();
    Ok(oMapping)
}

fn convertToSccSimEqMapping1(
    mut iMapping: (i32, i32),
    mut iSccMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oSccMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut i1: i32;
    let mut i2: i32;
    let mut tmpList: metamodelica::List<i32>;
    (i1, i2) = iMapping;
    tmpList = metamodelica::arrayGet(iSccMapping.clone(), i2)?;
    tmpList = metamodelica::cons(i1, tmpList);
    oSccMapping = metamodelica::arrayUpdate(iSccMapping.clone(), i2, tmpList)?;
    Ok(oSccMapping)
}

fn convertToSimeqCompMapping(
    mut iMapping: &metamodelica::List<(i32, i32)>,
    mut numOfSimEqs: i32,
) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMapping: metamodelica::Array<i32>;
    tmpMapping = arrayCreate(numOfSimEqs, -1);
    oMapping = List::fold(iMapping, &convertToSimeqCompMapping1, tmpMapping.clone())?;
    Ok(oMapping)
}

fn convertToSimeqCompMapping1(
    mut iSimEqTuple: (i32, i32),
    mut iMapping: metamodelica::Array<i32>,
) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut simEqIdx: i32;
    let mut sccIdx: i32;
    (simEqIdx, sccIdx) = iSimEqTuple;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), simEqIdx, sccIdx)?;
    Ok(oMapping)
}

fn getSimEqIdxSimEqMapping(
    mut iAllEquations: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut iSimEqSystemHighestIdx: i32,
) -> Result<metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut oMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut tmpMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    tmpMapping = arrayCreate(iSimEqSystemHighestIdx, None);
    oMapping = List::fold(iAllEquations, &getSimEqIdxSimEqMapping1, tmpMapping.clone())?;
    Ok(oMapping)
}

fn getSimEqIdxSimEqMapping1(
    mut iEquation: metamodelica::Ref<SimCode::SimEqSystem>,
    mut iMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut oMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut simEqIdx: i32 = 0;
    let mut tmpMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>> = Default::default();
    oMapping = 'mc: {
        let __mc_input = iMapping.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut simEqIdx: i32 = simEqIdx.clone();
            let mut tmpMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>> =
                tmpMapping.clone();
            (simEqIdx, _) = HpcOmCodegenUtil::getIndexBySimCodeEq(&iEquation)?;
            tmpMapping = metamodelica::arrayUpdate(iMapping.clone(), simEqIdx, Some(iEquation.clone()))?;
            Ok((tmpMapping.clone(), simEqIdx.clone(), tmpMapping.clone()))
        })() {
            simEqIdx = __wb0;
            tmpMapping = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut simEqIdx: i32 = simEqIdx.clone();
            (simEqIdx, _) = HpcOmCodegenUtil::getIndexBySimCodeEq(&iEquation)?;
            Ok((iMapping.clone(), simEqIdx.clone()))
        })() {
            simEqIdx = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oMapping)
}

fn getSimCodeEqByIndexAndMapping(
    mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
    mut iIdx: i32,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut tmpSimEqSystem: Option<metamodelica::Ref<SimCode::SimEqSystem>>;
    tmpSimEqSystem = metamodelica::arrayGet(iSimEqIdxSimEqMapping.clone(), iIdx)?;
    oSimEqSystem = getSimCodeEqByIndexAndMapping1(tmpSimEqSystem, iIdx)?;
    Ok(oSimEqSystem)
}

fn getSimCodeEqByIndexAndMapping1(
    mut iSimEqSystem: Option<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut iIdx: i32,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: metamodelica::Ref<SimCode::SimEqSystem>;
    let mut tmpSys: metamodelica::Ref<SimCode::SimEqSystem>;
    oSimEqSystem = (::match_deref::match_deref! { match &(iSimEqSystem) {
        Some(__esc_tmpSys) => {
            tmpSys = (*__esc_tmpSys).clone();
            tmpSys.clone()
        },
        _ => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndexAndMapping1 failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSimEqSystem)
}

fn getSimCodeEqsByTaskList(
    mut iTaskList: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>,
    mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut oSimEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut tmpSimEqs: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    tmpSimEqs = List::map1(
        iTaskList,
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>| {
            getSimCodeEqsByTaskList0(&__a0, __a1)
        },
        iSimEqIdxSimEqMapping.clone(),
    )?;
    oSimEqs = List::flatten(tmpSimEqs)?;
    Ok(oSimEqs)
}

fn getSimCodeEqsByTaskList0(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut oSimEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut eqIdc: metamodelica::List<i32>;
    let mut tmpSimEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    oSimEqs = (match &**iTask {
        HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdc, .. } => {
            eqIdc = (*__esc_eqIdc).clone();
            tmpSimEqs = List::map1r(
                eqIdc.clone(),
                &getSimCodeEqByIndexAndMapping,
                iSimEqIdxSimEqMapping.clone(),
            )?;
            tmpSimEqs
        }
        HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, .. } => {
            eqIdc = (*__esc_eqIdc).clone();
            tmpSimEqs = List::map1r(
                eqIdc.clone(),
                &getSimCodeEqByIndexAndMapping,
                iSimEqIdxSimEqMapping.clone(),
            )?;
            tmpSimEqs
        }
        _ => metamodelica::nil(),
    });
    Ok(oSimEqs)
}

fn dumpSimEqSCCMapping(mut iSccMapping: metamodelica::Array<i32>) -> Result<()> {
    let mut text: ArcStr;
    text = literal!("SimEqToSCCMapping");
    (_, text) = Array::fold(
        iSccMapping.clone(),
        &move |__a0: i32, __a1: (i32, ArcStr)| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpSimEqSCCMapping1(__a0, &__a1))
        },
        (1, text),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn dumpSimEqSCCMapping1(mut iMapping: i32, mut iIndexText: &(i32, ArcStr)) -> (i32, ArcStr) {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32;
    let mut text: ArcStr;
    let mut iText: ArcStr;
    (iIndex, iText) = iIndexText.clone();
    text = intString(iMapping);
    text = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iText);
        __mm_s.push_str(&*literal!("\nSimEq "));
        __mm_s.push_str(&*intString(iIndex));
        __mm_s.push_str(&*literal!(": {"));
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    oIndexText = (iIndex + 1, text);
    oIndexText
}

fn dumpSccSimEqMapping(mut iSccMapping: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    let mut text: ArcStr;
    text = literal!("SccToSimEqMapping");
    (_, text) = Array::fold(
        iSccMapping.clone(),
        &move |__a0: metamodelica::List<i32>, __a1: (i32, ArcStr)| dumpSccSimEqMapping1(&__a0, &__a1),
        (1, text),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn dumpSccSimEqMapping1(
    mut iMapping: &metamodelica::List<i32>,
    mut iIndexText: &(i32, ArcStr),
) -> Result<(i32, ArcStr)> {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32;
    let mut text: ArcStr;
    let mut iText: ArcStr;
    (iIndex, iText) = iIndexText.clone();
    text = List::fold(
        iMapping,
        &move |__a0: i32, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpSccSimEqMapping2(__a0, &__a1))
        },
        literal!(" "),
    )?;
    text = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iText);
        __mm_s.push_str(&*literal!("\nSCC "));
        __mm_s.push_str(&*intString(iIndex));
        __mm_s.push_str(&*literal!(": {"));
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    oIndexText = (iIndex + 1, text);
    Ok(oIndexText)
}

fn dumpSccSimEqMapping2(mut iIndex: i32, mut iText: &ArcStr) -> ArcStr {
    let mut oText: ArcStr;
    oText = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iText);
        __mm_s.push_str(&*intString(iIndex));
        __mm_s.push_str(&*literal!(" "));
        ArcStr::from(__mm_s)
    };
    oText
}
