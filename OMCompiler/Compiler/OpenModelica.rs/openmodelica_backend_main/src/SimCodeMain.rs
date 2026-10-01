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

use crate::CevalScriptBackend;
use crate::Interactive;
use crate::NFApi;
use openmodelica_ast::Absyn;
use openmodelica_backend::BackendDAECreate;
use openmodelica_backend::BackendDAEUtil;
use openmodelica_backend::BackendDump;
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVariable;
use openmodelica_backend::DAEMode;
use openmodelica_backend::FindZeroCrossings;
use openmodelica_backend::HpcOmSimCodeMain;
use openmodelica_backend::HpcOmTaskGraph;
use openmodelica_backend::RuntimeSources;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend::SymbolTable;
use openmodelica_backend::SymbolicJacobian;
use openmodelica_backend_tools::SerializeInitXML;
use openmodelica_backend_tools::SerializeModelInfo;
use openmodelica_backend_tools::SerializeSparsityPattern;
use openmodelica_backend_tools::SerializeTaskSystemInfo;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_codegen::CodegenESP32;
use openmodelica_codegen::CodegenEmbeddedC;
use openmodelica_codegen::CodegenJS;
#[cfg(feature = "codegen_c")]
use openmodelica_codegen_c::CodegenC;
#[cfg(feature = "cpp")]
use openmodelica_codegen_cpp::CodegenCpp;
#[cfg(feature = "cpp")]
use openmodelica_codegen_cpp_ext::CodegenCppHpcom;
#[cfg(feature = "cpp")]
use openmodelica_codegen_cpp_ext::CodegenFMUCpp;
#[cfg(feature = "cpp")]
use openmodelica_codegen_cpp_ext::CodegenFMUCppHpcom;
#[cfg(feature = "cpp")]
use openmodelica_codegen_cpp_omsi_ext::CodegenOMSICpp;
#[cfg(feature = "codegen_fmu")]
use openmodelica_codegen_fmu::CodegenFMU2;
#[cfg(feature = "codegen_fmu")]
use openmodelica_codegen_fmu::CodegenFMU3;
#[cfg(feature = "codegen_fmu")]
use openmodelica_codegen_fmu::CodegenFMUCommon;
#[cfg(feature = "codegen_fmu_c")]
use openmodelica_codegen_fmu_c::CodegenFMU;
#[cfg(feature = "codegen_fmu_c")]
use openmodelica_codegen_fmu_omsi::CodegenOMSI_common;
#[cfg(feature = "codegen_fmu_c")]
use openmodelica_codegen_fmu_omsi::CodegenOMSIC;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_codegen_wasm_jit::CodegenWasmJit;
use openmodelica_codegen_xml::CodegenXML;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::StateMachineFlatten;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_nbackend::NBackendDAE;
use openmodelica_nbackend::NSimCode;
use openmodelica_nf_frontend::NFConvertDAE;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFlatten::FunctionTreeImpl;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_program_util::ProgramUtil;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_tpl::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::FMI;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

/* used for new backend */
/// The FMU-grade translation translateModelFMU left behind, which the
///   buildModelFMU that follows exports instead of translating the model again.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmuTranslation {
    pub simCode: metamodelica::Ref<SimCode::SimCode>,
    pub FMUVersion: ArcStr,
    /// cs, or other: the preOptModules differ only for a pure Co-Simulation export
    pub kind: ArcStr,
    pub className: metamodelica::Ref<Absyn::Path>,
    /// what it was translated from, by reference: any edit replaces the record
    pub ast: Absyn::Program,
    pub debugFlags: metamodelica::Array<bool>,
    pub configFlags: metamodelica::Array<Flags::FlagData>,
}

impl metamodelica::gc::MMTrace for FmuTranslation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.simCode, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.FMUVersion, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.className, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ast, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.debugFlags, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.configFlags, __mmv)?;
        Ok(())
    }
}
pub type FMU_TRANSLATION = FmuTranslation;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum TranslateModelKind {
    NORMAL,
    XML,
    FMU {
        kind: ArcStr,
        targetName: ArcStr,
        /// keep the translation in memory instead of writing the FMU (translateModelFMU)
        translateOnly: bool,
    },
}
impl metamodelica::gc::MMTrace for TranslateModelKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TranslateModelKind::NORMAL => Ok(()),
            TranslateModelKind::XML => Ok(()),
            TranslateModelKind::FMU {
                kind,
                targetName,
                translateOnly,
            } => {
                metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(targetName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(translateOnly, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::TranslateModelKind::{FMU, NORMAL, XML};

pub(crate) fn createSimulationSettings(
    mut startTime: metamodelica::Real,
    mut stopTime: metamodelica::Real,
    mut inumberOfIntervals: i32,
    mut tolerance: metamodelica::Real,
    mut method: ArcStr,
    mut options: ArcStr,
    mut outputFormat: ArcStr,
    mut variableFilter: ArcStr,
    mut cflags: ArcStr,
    mut simflags: ArcStr,
) -> Result<SimCode::SimulationSettings> {
    let mut simSettings: SimCode::SimulationSettings;
    let mut stepSize: metamodelica::Real;
    let mut numberOfIntervals: i32;
    numberOfIntervals = if (inumberOfIntervals <= 0) {
        1
    } else {
        inumberOfIntervals
    };
    stepSize = metamodelica::real_div_checked((stopTime - startTime), intReal(numberOfIntervals))?;
    simSettings = SimCode::SimulationSettings {
        startTime: startTime,
        stopTime: stopTime,
        numberOfIntervals: numberOfIntervals,
        stepSize: stepSize,
        tolerance: tolerance,
        method: method,
        options: options,
        outputFormat: outputFormat,
        variableFilter: variableFilter,
        cflags: cflags,
        simflags: simflags,
    };
    Ok(simSettings)
}

fn generateModelCodeFMU(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inFMIDer: &metamodelica::List<(
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
    )>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut p: Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut FMUVersion: ArcStr,
    mut FMUType: ArcStr,
    mut filenamePrefix: ArcStr,
    mut fmuTargetName: ArcStr,
    mut simSettings: Option<SimCode::SimulationSettings>,
    mut translateOnly: bool,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Real,
    metamodelica::Real,
)> {
    let mut libs: metamodelica::List<ArcStr>;
    let mut fileDir: ArcStr;
    let mut timeSimCode: metamodelica::Real;
    let mut timeTemplates: metamodelica::Real;
    let mut includes: metamodelica::List<ArcStr>;
    let mut includeDirs: metamodelica::List<ArcStr>;
    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
    let mut a_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut libPaths: metamodelica::List<ArcStr>;
    let mut literals: (
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
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(&className);
    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsic"))) {
        fileDir = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*(AbsynUtil::pathToStringList(&className)).head().cloned()?);
            __mm_s.push_str(&*literal!(".tmp"));
            ArcStr::from(__mm_s)
        };
    } else {
        fileDir = ProgramUtil::getFileDir(a_cref, p.clone());
    }
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) =
        SimCodeUtilShared::createFunctions(&p, &inBackendDAE.shared.functionTree)?;
    simCode = createSimCode(
        inBackendDAE,
        inInitDAE,
        inInitDAE_lambda0,
        None,
        inRemovedInitialEquationLst,
        className,
        filenamePrefix,
        fileDir.clone(),
        functions,
        includes,
        includeDirs,
        libs.clone(),
        libPaths,
        p.clone(),
        simSettings,
        recordDecls,
        literals,
        &(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
            args: metamodelica::nil(),
            argNames: metamodelica::nil(),
        })),
        true,
        &FMUVersion,
        fmuTargetName,
        inFMIDer,
    )?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat(&(literal!("SimCode")))?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsicpp"))) {
        callTargetTemplatesFMU(simCode, literal!("C"), FMUVersion, FMUType, p, translateOnly)?;
    } else {
        callTargetTemplatesFMU(simCode, Config::simCodeTarget()?, FMUVersion, FMUType, p, translateOnly)?;
    }
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn generateModelCodeXML(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut p: Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut filenamePrefix: ArcStr,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Real,
    metamodelica::Real,
)> {
    let mut libs: metamodelica::List<ArcStr>;
    let mut fileDir: ArcStr;
    let mut timeSimCode: metamodelica::Real;
    let mut timeTemplates: metamodelica::Real;
    let mut includes: metamodelica::List<ArcStr>;
    let mut includeDirs: metamodelica::List<ArcStr>;
    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
    let mut libPaths: metamodelica::List<ArcStr>;
    let mut a_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut literals: (
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
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(&className);
    fileDir = ProgramUtil::getFileDir(a_cref, p.clone());
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) =
        SimCodeUtilShared::createFunctions(&p, &inBackendDAE.shared.functionTree)?;
    (simCode, _) = SimCodeUtil::createSimCode(
        inBackendDAE,
        inInitDAE,
        inInitDAE_lambda0,
        None,
        inRemovedInitialEquationLst,
        className,
        filenamePrefix,
        fileDir.clone(),
        functions,
        includes,
        includeDirs,
        libs.clone(),
        libPaths,
        p,
        simSettingsOpt,
        recordDecls,
        literals,
        &(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
            args: metamodelica::nil(),
            argNames: metamodelica::nil(),
        })),
        false,
        &(literal!("")),
        literal!(""),
        &(metamodelica::nil()),
    )?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat(&(literal!("SimCode")))?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplatesXML(simCode, &(Config::simCodeTarget()?))?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

pub(crate) fn generateModelCode(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inInlineData: Option<BackendDAE::InlineData>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut p: Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut filenamePrefix: ArcStr,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut inFMIDer: &metamodelica::List<(
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
    )>,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Real,
    metamodelica::Real,
)> {
    let mut libs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut includeDirs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut libPaths: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>> = metamodelica::nil();
    let mut simCode: metamodelica::Ref<SimCode::SimCode> =
        <metamodelica::Ref<SimCode::SimCode> as ::std::default::Default>::default();
    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration> = metamodelica::nil();
    let mut a_cref: metamodelica::Ref<Absyn::ComponentRef> = metamodelica::Ref::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (
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
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ) = (
        0,
        (
            Default::default(),
            (0, 0, Default::default()),
            0,
            (
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            ),
        ),
        metamodelica::nil(),
    );
    let mut numCheckpoints: i32;
    numCheckpoints = ErrorExt::getNumCheckpoints();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        StackOverflow::clearStacktraceMessages();
        if Flags::isSet(Flags::GRAPHML.clone())? {
            HpcOmTaskGraph::dumpTaskGraph(&inBackendDAE, &filenamePrefix)?;
            BackendDump::dumpBackendDAEBipartiteGraph(
                &inBackendDAE,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BipartiteGraph_CompleteDAE_"));
                    __mm_s.push_str(&*filenamePrefix);
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        a_cref = AbsynUtil::pathToCref(&className);
        fileDir = ProgramUtil::getFileDir(a_cref.clone(), p.clone());
        (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) =
            SimCodeUtilShared::createFunctions(&p, &inBackendDAE.shared.functionTree)?;
        simCode = createSimCode(
            inBackendDAE.clone(),
            inInitDAE,
            inInitDAE_lambda0.clone(),
            inInlineData.clone(),
            inRemovedInitialEquationLst.clone(),
            className.clone(),
            filenamePrefix.clone(),
            fileDir.clone(),
            functions.clone(),
            includes.clone(),
            includeDirs.clone(),
            libs.clone(),
            libPaths.clone(),
            p.clone(),
            simSettingsOpt.clone(),
            recordDecls.clone(),
            literals.clone(),
            args,
            false,
            &(literal!("")),
            literal!(""),
            inFMIDer,
        )?;
        timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        ExecStat::execStat(&(literal!("SimCode")))?;
        if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
            serializeNotify(simCode.clone(), literal!("SimCode"))?;
            ExecStat::execStat(&(literal!("Serialize simCode")))?;
        }
        System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        callTargetTemplates(simCode.clone(), &(Config::simCodeTarget()?))?;
        timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        ExecStat::execStat(&(literal!("Templates")))?;
        return Ok(true);
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok((
                    libs.clone(),
                    fileDir.clone(),
                    timeSimCode.clone(),
                    timeTemplates.clone(),
                ));
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            {
                let __v = None;
                openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Stack overflow in "));
                    __mm_s.push_str(&*literal!("SimCodeMain.generateModelCode"));
                    __mm_s.push_str(&*literal!("...\n"));
                    __mm_s.push_str(&*stringDelimitList(
                        StackOverflow::readableStacktraceMessages()?,
                        literal!("\n"),
                    ));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
            )?;
            StackOverflow::clearStacktraceMessages();
        }
    }
    return Err("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn createSimCode(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inInlineData: Option<BackendDAE::InlineData>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut filenamePrefix: ArcStr,
    mut inString11: ArcStr,
    mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    mut externalFunctionIncludes: metamodelica::List<ArcStr>,
    mut includeDirs: metamodelica::List<ArcStr>,
    mut libs: metamodelica::List<ArcStr>,
    mut libPaths: metamodelica::List<ArcStr>,
    mut program: Absyn::Program,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
    mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>,
    mut literals: (
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
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut isFMU: bool,
    mut FMUVersion: &ArcStr,
    mut fmuTargetName: ArcStr,
    mut inFMIDer: &metamodelica::List<(
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
    )>,
) -> Result<metamodelica::Ref<SimCode::SimCode>> {
    let mut simCode: metamodelica::Ref<SimCode::SimCode>;
    simCode = 'mc: {
        let __mc_input = &**args;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::MULTIRATE_PARTITION.clone())?) else { return Err("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { return Err("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc == 0) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("hpcom computes the ideal number of processors. If you want to set the number manually, use the flag +n=_\n"));
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { return Err("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc > 0) else { return Err("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpSimCode: metamodelica::Ref<SimCode::SimCode>;
                    (tmpSimCode, _) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE, inInitDAE_lambda0.clone(), inInlineData.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args, isFMU, FMUVersion, fmuTargetName.clone(), inFMIDer)?;
                    Ok(tmpSimCode.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(simCode)
}

fn generateModelCodeNewBackend(
    mut bdae: &metamodelica::Ref<NBackendDAE::NBackendDAE>,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileNamePrefix: ArcStr,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
    mut kind: &TranslateModelKind,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Real,
    metamodelica::Real,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut libs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut oldFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    let mut numCheckpoints: i32;
    let mut simCode: metamodelica::Ref<NSimCode::SimCode::SimCode> =
        <metamodelica::Ref<NSimCode::SimCode::SimCode> as ::std::default::Default>::default();
    let mut oldSimCode: metamodelica::Ref<SimCode::SimCode> =
        <metamodelica::Ref<SimCode::SimCode> as ::std::default::Default>::default();
    numCheckpoints = ErrorExt::getNumCheckpoints();
    StackOverflow::clearStacktraceMessages();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        (simCode, oldFunctionTree) = NSimCode::SimCode::create(
            bdae,
            className.clone(),
            fileNamePrefix.clone(),
            simSettingsOpt.clone(),
            SymbolTable::getAbsyn(),
        )?;
        if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
            metamodelica::print(NSimCode::SimCode::toString(&simCode, literal!(""))?);
        }
        (fileDir, libs) = NSimCode::SimCode::getDirectoryAndLibs(&simCode)?;
        oldSimCode = NSimCode::SimCode::convert(&simCode)?;
        if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
            SimCodeUtil::dumpSimCodeDebug(&oldSimCode)?;
        }
        timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        ExecStat::execStat(&(literal!("SimCode")))?;
        if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
            serializeNotify(oldSimCode.clone(), literal!("SimCode"))?;
            ExecStat::execStat(&(literal!("Serialize simCode")))?;
        }
        System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        let () = (match kind.clone() {
            TranslateModelKind::FMU {
                kind: mut fmuType,
                targetName: mut fmuTarget,
                ..
            } => {
                assign_field!(
                    oldSimCode.fmuTargetName = fmuTarget.clone(),
                    oldSimCode.fullPathPrefix = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*Util::hashFileNamePrefix(&fileNamePrefix)?);
                        __mm_s.push_str(&*literal!(".fmutmp/sources/"));
                        ArcStr::from(__mm_s)
                    },
                    oldSimCode.valueReferences = SimCodeUtil::getValueReferenceMapping(&oldSimCode.modelInfo)?,
                    oldSimCode.modelStructure = SimCodeUtil::createMinimalFMIModelStructure(&oldSimCode.modelInfo)?
                );
                callTargetTemplatesFMU(
                    oldSimCode.clone(),
                    Config::simCodeTarget()?,
                    FMI::getFMIVersionString()?,
                    fmuType.clone(),
                    SymbolTable::getAbsyn(),
                    var_field!(kind.translateOnly, TranslateModelKind::FMU).clone(),
                )?;
                ()
            }
            _ => {
                callTargetTemplates(oldSimCode.clone(), &(Config::simCodeTarget()?))?;
                ()
            }
        });
        timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        ExecStat::execStat(&(literal!("Templates")))?;
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok((
                    libs.clone(),
                    fileDir.clone(),
                    timeSimCode.clone(),
                    timeTemplates.clone(),
                    oldFunctionTree.clone(),
                ));
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            {
                let __v = None;
                openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Stack overflow in "));
                    __mm_s.push_str(&*literal!("SimCodeMain.generateModelCodeNewBackend"));
                    __mm_s.push_str(&*literal!("...\n"));
                    __mm_s.push_str(&*stringDelimitList(
                        StackOverflow::readableStacktraceMessages()?,
                        literal!("\n"),
                    ));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
            )?;
            StackOverflow::clearStacktraceMessages();
            return Err("fail");
        }
    }
    Ok((libs, fileDir, timeSimCode, timeTemplates, oldFunctionTree))
}

type PartialRunTpl = std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>;

type FuncText = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>;

fn runTplWriteFile(
    mut func: Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>,
    mut file: ArcStr,
) -> (bool, metamodelica::List<ArcStr>) {
    let mut res: (bool, metamodelica::List<ArcStr>);
    let mut nErr: i32;
    res = (false, metamodelica::nil());
    if '__try0: {
        unwrap_break_err!(SimCodeCodegenUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        if unwrap_break_err!(Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone()), '__try0) {
            unwrap_break_err!(Tpl::textFileConvertLines(unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), Tpl::emptyTxt.clone()), '__try0), file.clone()), '__try0);
        } else {
            nErr = Error::getNumErrorMessages();
            unwrap_break_err!(Tpl::closeFile(unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), unwrap_break_err!(Tpl::redirectToFile(Tpl::emptyTxt.clone(), file.clone()), '__try0)), '__try0)), '__try0);
            unwrap_break_err!(Tpl::failIfTrue(Error::getNumErrorMessages() > nErr), '__try0);
        }
        res = (true, unwrap_break_err!(SimCodeUtil::getFunctionIndex(), '__try0));
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    res
}

fn runTpl(
    mut func: Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>,
) -> (bool, metamodelica::List<ArcStr>) {
    let mut res: (bool, metamodelica::List<ArcStr>);
    res = (false, metamodelica::nil());
    if '__try0: {
        unwrap_break_err!(SimCodeCodegenUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), Tpl::emptyTxt.clone()), '__try0);
        res = (true, unwrap_break_err!(SimCodeUtil::getFunctionIndex(), '__try0));
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    res
}

// TODO: use another switch ... later make it first class option like -target or so
fn callTargetTemplates(mut simCode: metamodelica::Ref<SimCode::SimCode>, mut target: &ArcStr) -> Result<()> {
    type Func = std::sync::Arc<
        dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static,
    >;

    type FuncText = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>;

    type BoolFunc = std::sync::Arc<
        dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static,
    >;

    fn runToStr(mut func: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>) -> (bool, metamodelica::List<ArcStr>) {
        pub type Func = std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>;

        let mut res: (bool, metamodelica::List<ArcStr>);
        res = (false, metamodelica::nil());
        if '__try0: {
            unwrap_break_err!(SimCodeCodegenUtil::resetFunctionIndex(), '__try0);
            SimCodeFunctionUtil::codegenResetTryThrowIndex();
            unwrap_break_err!(func(), '__try0);
            res = (true, unwrap_break_err!(SimCodeUtil::getFunctionIndex(), '__try0));
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
        res
    }

    fn runCodegenFunc(
        mut func: &dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)>,
    ) -> Result<(bool, metamodelica::List<ArcStr>)> {
        let mut res: (bool, metamodelica::List<ArcStr>);
        let mut b: bool;
        let __pa1 @ (__pa0, _) = &(func()?);
        b = metamodelica::Own::own(__pa0);
        res = metamodelica::Own::own(__pa1);
        if !(b) {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*(System::dladdr(func)).0);
                    __mm_s.push_str(&*literal!(" failed\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
            )?;
        }
        if ErrorExt::getNumMessages() > 0 {
            ErrorExt::moveMessagesToParentThread();
        }
        Ok(res)
    }

    fn runToBoolean(mut func: &dyn ::std::ops::Fn() -> Result<bool>) -> Result<(bool, metamodelica::List<ArcStr>)> {
        type Func = std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>;

        let mut res: (bool, metamodelica::List<ArcStr>);
        res = (func()?, metamodelica::nil());
        Ok(res)
    }

    let mut func: Arc<
        dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static,
    >;
    let mut txt: Tpl::Text;
    let mut generatedObjects: metamodelica::Ref<AvlSetString::Tree> =
        openmodelica_util::AvlSetString::Tree::interned_EMPTY();
    {
        let __v = Some(simCode.clone());
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    SimCodeFunctionUtil::setTrivialRecords(&simCode.recordDecls)?;
    let () = ({
        let mut res: metamodelica::List<(bool, metamodelica::List<ArcStr>)> = metamodelica::nil();
        (::match_deref::match_deref! { match &(target.clone()) {
            Deref @ "Cpp" => {
                let mut r#str: ArcStr = arcstr::literal!("");
                callTargetTemplatesCPP(simCode.clone())?;
                for mut r#str in &*list![literal!("CalcHelperMain.o\n"), literal!(".so\n")] {
                    let mut r#str = r#str.clone();
                    generatedObjects = AvlSetString::add(generatedObjects, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OMCpp")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }))?;
                }
                ()
            },
            Deref @ "C" => {
                let mut r#str: ArcStr = arcstr::literal!("");
                let mut guid: ArcStr;
                let mut codegenFuncs: metamodelica::List<PartialRunTpl>;
                let mut numThreads: i32;
                let mut n: i32;
                let mut strs: metamodelica::List<ArcStr>;
                let mut tmp: metamodelica::List<ArcStr>;
                let mut matches: metamodelica::List<ArcStr>;
                guid = System::getUUIDStr();
                System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
                codegenFuncs = metamodelica::nil();
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = guid.clone(); move || SerializeInitXML::simulationInitFileReturnBool(&__pe_b0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>); move || runToBoolean(&*__pe_b0) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = { #[cfg(feature = "codegen_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenC::translateModel(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.translateModel needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.translateModel needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }; move || Ok(runTpl(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                for mut f in &*list![({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_exo(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_exo needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_exo needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_01exo.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_nls(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_nls needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_nls needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_02nls.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_lsy(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_lsy needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_lsy needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_03lsy.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_set(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_set needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_set needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_04set.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_evt(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_evt needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_evt needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_05evt.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_inz(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_inz needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_inz needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_06inz.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_dly(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_dly needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_dly needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_07dly.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_bnd(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_bnd needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_bnd needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_08bnd.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_alg(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_alg needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_alg needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_09alg.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_asr(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_asr needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_asr needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_10asr.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_jac(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_jac needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_jac needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_12jac.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_jac_header(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_jac_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_jac_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_12jac.h")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_opt(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_opt needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_opt needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_13opt.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_opt_header(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_opt_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_opt_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_13opt.h")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_lnz(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_lnz needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_lnz needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_14lnz.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_syn(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_syn needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_syn needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_15syn.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_dae(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_dae needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_dae needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_16dae.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_dae_header(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_dae_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_dae_header needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_16dae.h")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_inl(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_inl needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_inl needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_17inl.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationFile_spd(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_spd needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_spd needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_18spd.c")), ({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::simulationHeaderFile(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationHeaderFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationHeaderFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, literal!("_model.h"))] {
                    (func, r#str) = f.clone();
                    codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }; move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                    (n, matches) = System::regex(r#str, literal!("\\(.*\\)[.]c$"), 2, false, false);
                    if n == 2 {
                        let __pa0 = ::match_deref::match_deref! { match &(matches) {
                            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        r#str = metamodelica::Own::own(__pa0);
                        generatedObjects = AvlSetString::add(generatedObjects, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }))?;
                    }
                }
                for mut r#str in &*list![literal!("_11mix.o\n"), literal!("_functions.o\n"), literal!("_info.json\n"), literal!("_init.xml\n")] {
                    let mut r#str = r#str.clone();
                    generatedObjects = AvlSetString::add(generatedObjects, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }))?;
                }
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = { #[cfg(feature = "codegen_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = simCode.fileNamePrefix.clone(); move |__pe_a0| CodegenC::simulationFile_mixAndHeader(__pe_a0, &__pe_b1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile_mixAndHeader needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile_mixAndHeader needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }; move || Ok(runTpl(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = { #[cfg(feature = "codegen_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = guid; let __pe_b3 = literal!(""); move |__pe_a0| CodegenC::simulationFile(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }; let __pe_b1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }; move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = { #[cfg(feature = "codegen_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.fileNamePrefix.clone(); let __pe_b2 = simCode.modelInfo.functions.clone(); let __pe_b3 = simCode.generic_loop_calls.clone(); move |__pe_a0| CodegenC::simulationFunctionsFile(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationFunctionsFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationFunctionsFile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }; let __pe_b1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_functions.c")); ArcStr::from(__mm_s) }; move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); move || SerializeSparsityPattern::serialize(__pe_b0.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeModelInfo::serialize(&__pe_b0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                if Flags::getConfigBool(Flags::PARMODAUTO.clone())? {
                    codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeTaskSystemInfo::serializeParMod(&__pe_b0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                    generatedObjects = AvlSetString::add(generatedObjects, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_ode.json\n")); ArcStr::from(__mm_s) }))?;
                }
                if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
                    codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); move || SimCodeUtil::generateRunnerBatScript(&__pe_b0) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                }
                numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
                if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads == 1 {
                    res = ({
            let mut __acc: metamodelica::List<(bool, metamodelica::List<ArcStr>)> = metamodelica::nil();
            for mut codegen_func in (codegenFuncs).into_iter().cloned() {
                let __x = codegen_func()?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                } else {
                    res = System::launchParallelTasks(numThreads, codegenFuncs, (std::sync::Arc::new(move |__a0: Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>| runCodegenFunc(metamodelica::arc_ref(&__a0))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>) -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>))?;
                }
                strs = metamodelica::nil();
                for mut tpl in &*res {
                    let __pa2 = ::match_deref::match_deref! { match &(tpl.clone()) {
                        (true, __pa2) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    tmp = metamodelica::Own::own(__pa2);
                    strs = List::append_reverse(&tmp, strs);
                }
                strs = strs.reverse();
                for mut r#str in &*strs {
                    let mut r#str = r#str.clone();
                    (n, matches) = System::regex(r#str, literal!("\\(.*\\)[.]c$"), 2, false, false);
                    if n == 2 {
                        let __pa3 = ::match_deref::match_deref! { match &(matches) {
                            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: _ } } => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        r#str = metamodelica::Own::own(__pa3);
                        generatedObjects = AvlSetString::add(generatedObjects, &({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }))?;
                    }
                }
                Tpl::closeFile(Tpl::tplCallWithFailError3({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: ArcStr, __a2: metamodelica::Ref<SimCode::SimCode>, __a3: metamodelica::List<ArcStr>| CodegenC::simulationMakefile(__a0, &__a1, &__a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, ArcStr, metamodelica::Ref<SimCode::SimCode>, metamodelica::List<ArcStr>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1, _a2, _a3| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.simulationMakefile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.simulationMakefile needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, ArcStr, metamodelica::Ref<SimCode::SimCode>, metamodelica::List<ArcStr>) -> Result<Tpl::Text> + 'static>) } }, Config::simulationCodeTarget()?, simCode.clone(), strs, Tpl::redirectToFile(Tpl::emptyTxt.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!(".makefile")); ArcStr::from(__mm_s) })?)?)?;
                ()
            },
            Deref @ "ExperimentalEmbeddedC" => {
                let mut r#str: ArcStr;
                let mut codegenFuncs: metamodelica::List<PartialRunTpl>;
                let mut numThreads: i32;
                let mut strs: metamodelica::List<ArcStr>;
                let mut tmp: metamodelica::List<ArcStr>;
                System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
                codegenFuncs = metamodelica::nil();
                for mut f in &*list![((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenEmbeddedC::mainFile(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>), literal!("_main.c"))] {
                    (func, r#str) = f.clone();
                    codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }; move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>), codegenFuncs);
                }
                numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
                if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads == 1 {
                    res = ({
            let mut __acc: metamodelica::List<(bool, metamodelica::List<ArcStr>)> = metamodelica::nil();
            for mut func in (codegenFuncs).into_iter().cloned() {
                let __x = func()?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                } else {
                    res = System::launchParallelTasks(numThreads, codegenFuncs, (std::sync::Arc::new(move |__a0: Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>| runCodegenFunc(metamodelica::arc_ref(&__a0))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<dyn ::std::ops::Fn() -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>) -> Result<(bool, metamodelica::List<ArcStr>)> + 'static>))?;
                }
                strs = metamodelica::nil();
                for mut tpl in &*res {
                    let __pa0 = ::match_deref::match_deref! { match &(tpl.clone()) {
                        (true, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    tmp = metamodelica::Own::own(__pa0);
                    strs = List::append_reverse(&tmp, strs);
                }
                strs = strs.reverse();
                ()
            },
            Deref @ "ESP32" => {
                let mut esp32Dir: ArcStr;
                System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
                esp32Dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_esp32")); ArcStr::from(__mm_s) };
                if !(Util::createDirectoryTree({ let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main")); ArcStr::from(__mm_s) })) {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create directory ")); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                    return Err("fail");
                }
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::projectCMakeFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/CMakeLists.txt")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::componentCMakeFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main/CMakeLists.txt")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::sdkconfigFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/sdkconfig.defaults")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::modelHeaderFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main/")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_model.h")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::modelSourceFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main/")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_model.c")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenESP32::appMainFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/main/main.c")); ArcStr::from(__mm_s) });
                runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode; move |__pe_a0| CodegenESP32::readmeFile(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), { let mut __mm_s = String::new(); __mm_s.push_str(&*esp32Dir); __mm_s.push_str(&*literal!("/README.md")); ArcStr::from(__mm_s) });
                ()
            },
            Deref @ "JavaScript" => {
                let mut guid: ArcStr;
                guid = System::getUUIDStr();
                Tpl::tplNoret({ #[cfg(feature = "codegen_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenC::translateModel(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenC.translateModel needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenC.translateModel needs the 'codegen_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, simCode.clone())?;
                SerializeInitXML::simulationInitFile(&simCode, guid)?;
                System::covertTextFileToCLiteral({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_init.c")); ArcStr::from(__mm_s) }, Config::simulationCodeTarget()?);
                SerializeSparsityPattern::serialize(simCode.clone())?;
                SerializeModelInfo::serialize(&simCode, Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
                Tpl::tplNoret((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenJS::markdownFile(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>), simCode)?;
                ()
            },
            Deref @ "XML" => {
                Tpl::tplNoret((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenXML::translateModel(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>), simCode)?;
                ()
            },
            Deref @ "wasm-jit" => {
                let mut guid: ArcStr;
                CodegenWasmJit::translateModel(simCode.clone())?;
                if Flags::isSet(Flags::OMEDIT.clone())? {
                    guid = System::getUUIDStr();
                    SerializeInitXML::simulationInitFile(&simCode, guid)?;
                }
                if Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? {
                    SerializeModelInfo::serialize(&simCode, true)?;
                }
                ()
            },
            Deref @ "wasm" => {
                CodegenWasmJit::emitStandalone(simCode)?;
                ()
            },
            Deref @ "None" => {
                ()
            },
            _ => {
                let mut r#str: ArcStr;
                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown template target: ")); __mm_s.push_str(&*target); ArcStr::from(__mm_s) };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    if Testsuite::isRunning()? {
        System::appendFile(
            Testsuite::getTempFilesFile()?,
            stringAppendList(AvlSetString::listKeys(&generatedObjects, metamodelica::nil())),
        )?;
    }
    {
        let __v = None;
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

fn callTargetTemplatesCPP(mut iSimCode: metamodelica::Ref<SimCode::SimCode>) -> Result<()> {
    if Flags::isSet(Flags::HPCOM.clone())? {
        Tpl::tplNoret(
            {
                #[cfg(feature = "cpp")]
                {
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| {
                        CodegenCppHpcom::translateModel(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text>
                                + 'static,
                        >)
                }
                #[cfg(not(feature = "cpp"))]
                {
                    (std::sync::Arc::new(|_a0, _a1| {
                        let _ = ::openmodelica_util::Error::addInternalError(
                            ::arcstr::literal!(
                                "CodegenCppHpcom.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"
                            ),
                            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                        );
                        return Err(
                            "CodegenCppHpcom.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without",
                        );
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text>
                                + 'static,
                        >)
                }
            },
            iSimCode,
        )?;
    } else {
        Tpl::tplNoret(
            {
                #[cfg(feature = "cpp")]
                {
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| {
                        CodegenCpp::translateModel(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text>
                                + 'static,
                        >)
                }
                #[cfg(not(feature = "cpp"))]
                {
                    (std::sync::Arc::new(|_a0, _a1| {
                        let _ = ::openmodelica_util::Error::addInternalError(
                            ::arcstr::literal!(
                                "CodegenCpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"
                            ),
                            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                        );
                        return Err(
                            "CodegenCpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without",
                        );
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text>
                                + 'static,
                        >)
                }
            },
            iSimCode,
        )?;
    }
    Ok(())
}

fn callTargetTemplatesOMSICpp(
    mut iSimCode: metamodelica::Ref<SimCode::SimCode>,
    mut program: Absyn::Program,
) -> Result<()> {
    let mut fmuVersion: ArcStr;
    let mut fmuType: ArcStr;
    fmuVersion = literal!("2.0");
    fmuType = literal!("me");
    Tpl::tplNoret3(
        {
            #[cfg(feature = "cpp")]
            {
                (std::sync::Arc::new(
                    move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>, __a2: ArcStr, __a3: ArcStr| {
                        CodegenOMSICpp::translateModel(__a0, &__a1, &__a2, &__a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Tpl::Text,
                                metamodelica::Ref<SimCode::SimCode>,
                                ArcStr,
                                ArcStr,
                            ) -> Result<Tpl::Text>
                            + 'static,
                    >)
            }
            #[cfg(not(feature = "cpp"))]
            {
                (std::sync::Arc::new(|_a0, _a1, _a2, _a3| {
                    let _ = ::openmodelica_util::Error::addInternalError(
                        ::arcstr::literal!(
                            "CodegenOMSICpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"
                        ),
                        metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                    );
                    return Err(
                        "CodegenOMSICpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without",
                    );
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Tpl::Text,
                                metamodelica::Ref<SimCode::SimCode>,
                                ArcStr,
                                ArcStr,
                            ) -> Result<Tpl::Text>
                            + 'static,
                    >)
            }
        },
        iSimCode.clone(),
        fmuVersion.clone(),
        fmuType.clone(),
    )?;
    callTargetTemplatesFMU(iSimCode, literal!("C"), fmuVersion, fmuType, program, false)?;
    Ok(())
}

fn visualizationCadFiles(mut visualXmlFile: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut paths: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut program: Absyn::Program;
    let mut prevType: bool = false;
    let mut low: ArcStr;
    let mut path: ArcStr;
    if !(System::regularFileExists(visualXmlFile.clone())) {
        return Ok(paths);
    }
    program = SymbolTable::getAbsyn();
    for mut tok in &*System::strtok(System::readFile(visualXmlFile)?, literal!("<>")) {
        if prevType {
            low = System::tolower(tok.clone());
            if StringUtil::endsWith(low.clone(), literal!(".dxf"))
                || StringUtil::endsWith(low.clone(), literal!(".stl"))
                || StringUtil::endsWith(low.clone(), literal!(".obj"))
                || StringUtil::endsWith(low, literal!(".3ds"))
            {
                if System::stringFind(tok.clone(), literal!("modelica://"))? == 0 {
                    path = ProgramUtil::getFullPathFromUri(&program, tok.clone(), true)?;
                } else if System::stringFind(tok.clone(), literal!("file://"))? == 0 {
                    path = substring(tok.clone(), 8, ((tok).len() as i32))?;
                } else {
                    path = tok.clone();
                }
                if !(listMember(path.clone(), paths.clone())) {
                    paths = metamodelica::cons(path, paths);
                }
            }
        }
        prevType = stringEqual(&tok, &(literal!("type")));
    }
    paths = paths.reverse();
    Ok(paths)
}

fn fmuTranslationKind(mut FMUType: &ArcStr) -> ArcStr {
    let mut kind: ArcStr = if (FMI::isFMICSType(FMUType) && !(FMI::isFMIMEType(FMUType))) {
        literal!("cs")
    } else {
        literal!("other")
    };
    kind
}

fn fmuTranslationFlags() -> Result<(metamodelica::Array<bool>, metamodelica::Array<Flags::FlagData>)> {
    let mut debugFlags: metamodelica::Array<bool>;
    let mut configFlags: metamodelica::Array<Flags::FlagData>;
    let mut buildingModel: i32;
    let Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: __pa1,
    } = (Flags::getFlags(true))
    else {
        return Err("pattern mismatch");
    };
    debugFlags = metamodelica::Own::own(__pa0);
    configFlags = metamodelica::Own::own(__pa1);
    debugFlags = metamodelica::arrayFromVec(debugFlags.clone().borrow().clone());
    configFlags = metamodelica::arrayFromVec(configFlags.clone().borrow().clone());
    let Flags::CONFIG_FLAG { index: __pa2, .. } = Flags::BUILDING_MODEL.clone();
    buildingModel = metamodelica::Own::own(__pa2);
    metamodelica::arrayUpdate(
        configFlags.clone(),
        buildingModel,
        Flags::FlagData::BOOL_FLAG { data: false },
    )?;
    Ok((debugFlags, configFlags))
}

pub(crate) fn keepFmuTranslation(
    mut simCode: metamodelica::Ref<SimCode::SimCode>,
    mut FMUVersion: ArcStr,
    mut FMUType: &ArcStr,
    mut className: metamodelica::Ref<Absyn::Path>,
) -> Result<()> {
    let mut debugFlags: metamodelica::Array<bool>;
    let mut configFlags: metamodelica::Array<Flags::FlagData>;
    (debugFlags, configFlags) = fmuTranslationFlags()?;
    {
        let __v = Some(FmuTranslation {
            simCode: simCode,
            FMUVersion: FMUVersion,
            kind: fmuTranslationKind(FMUType),
            className: className,
            ast: SymbolTable::getAbsyn(),
            debugFlags: debugFlags.clone(),
            configFlags: configFlags.clone(),
        });
        crate::Globals::fmuTranslation.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

fn sameFmuSettings(mut a: Option<SimCode::SimulationSettings>, mut b: Option<SimCode::SimulationSettings>) -> bool {
    let mut same: bool;
    let mut x: SimCode::SimulationSettings;
    let mut y: SimCode::SimulationSettings;
    same = (match (a, b) {
        (Some(mut __esc_x), Some(mut __esc_y)) => {
            x = __esc_x.clone();
            y = __esc_y.clone();
            x.method = literal!("");
            y.method = literal!("");
            x == y
        }
        (None, None) => true,
        _ => false,
    });
    same
}

pub(crate) fn fmuTranslationFor(
    mut FMUVersion: &ArcStr,
    mut FMUType: &ArcStr,
    mut className: &metamodelica::Ref<Absyn::Path>,
    mut settings: Option<SimCode::SimulationSettings>,
) -> Result<Option<metamodelica::Ref<SimCode::SimCode>>> {
    let mut simCode: Option<metamodelica::Ref<SimCode::SimCode>> = None;
    let mut debugFlags: metamodelica::Array<bool>;
    let mut configFlags: metamodelica::Array<Flags::FlagData>;
    let mut kept: Option<FmuTranslation> = crate::Globals::fmuTranslation.with(|__root| __root.borrow().clone());
    let () = (match kept {
        Some(mut t @ FmuTranslation { .. })
            if (AbsynUtil::pathEqual(&t.className, className)
                && metamodelica::stringEq(&t.FMUVersion, &FMUVersion)
                && metamodelica::stringEq(&t.kind, &(fmuTranslationKind(FMUType)))
                && {
                    let __refeq_sl = &(t.ast.clone());
                    let __refeq_sr = &(SymbolTable::getAbsyn());
                    metamodelica::ReferenceEq::reference_eq(&(__refeq_sl.classes), &(__refeq_sr.classes))
                        && (match (&(__refeq_sl.within_), &(__refeq_sr.within_)) {
                            (Absyn::Within::TOP, Absyn::Within::TOP) => true,
                            (
                                Absyn::Within::WITHIN { path: __refeq_v0l },
                                Absyn::Within::WITHIN { path: __refeq_v0r },
                            ) => referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r)),
                            _ => false,
                        })
                }
                && sameFmuSettings(t.simCode.simulationSettingsOpt.clone(), settings.clone())) =>
        {
            (debugFlags, configFlags) = fmuTranslationFlags()?;
            if Array::isEqual(t.debugFlags.clone(), debugFlags.clone())?
                && Array::isEqual(t.configFlags.clone(), configFlags.clone())?
            {
                simCode = Some(t.simCode.clone());
            }
            ()
        }
        _ => (),
    });
    Ok(simCode)
}

pub(crate) fn wasmFMUSimulationFlagsJson(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut json: ArcStr;
    json = (match simCode.fmiSimulationFlags.clone() {
        Some(SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS_FILE { path: mut path }) => {
            System::readFile(path.clone())?
        }
        Some(mut flags @ SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS { .. }) => Tpl::textString({
            #[cfg(feature = "codegen_fmu")]
            {
                CodegenFMUCommon::fmuSimulationFlagsFile(Tpl::emptyTxt.clone(), metamodelica::AsArg::as_arg(&flags))?
            }
            #[cfg(not(feature = "codegen_fmu"))]
            {
                {
                    let _ = ::openmodelica_util::Error::addInternalError(
                        ::arcstr::literal!(
                            "CodegenFMUCommon.fmuSimulationFlagsFile needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without"
                        ),
                        metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                    );
                    return Err(
                        "CodegenFMUCommon.fmuSimulationFlagsFile needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without",
                    );
                }
            }
        })?,
        _ => {
            literal!("")
        }
    });
    Ok(json)
}

pub(crate) fn emitWasmFMU(
    mut simCode: metamodelica::Ref<SimCode::SimCode>,
    mut FMUVersion: &ArcStr,
    mut FMUType: &ArcStr,
    mut program: Absyn::Program,
) -> Result<()> {
    let mut guid: ArcStr;
    let mut modelDescriptionStr: ArcStr;
    let mut simulationFlagsJson: ArcStr;
    let mut lsDaeManifestStr: ArcStr = literal!("");
    let mut fmutmp: ArcStr = literal!("");
    let mut terminalsDir: ArcStr = literal!("");
    let mut documentationDir: ArcStr = literal!("");
    let mut terminals: metamodelica::List<SimCode::FmiTerminal>;
    let mut bareExport: bool = Flags::getConfigBool(Flags::FMU_DIRECTORY.clone())?;
    {
        let __v = Some(simCode.clone());
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    SimCodeFunctionUtil::setTrivialRecords(&simCode.recordDecls)?;
    guid = System::getUUIDStr();
    if !(bareExport) {
        fmutmp = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Util::hashFileNamePrefix(&simCode.fileNamePrefix)?);
            __mm_s.push_str(&*literal!(".fmutmp"));
            ArcStr::from(__mm_s)
        };
        if System::directoryExists(fmutmp.clone()) && !(System::removeDirectory(fmutmp.clone())) {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to remove directory: "));
                    __mm_s.push_str(&*fmutmp);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
            )?;
            return Err("fail");
        }
    }
    System::realtimeTick(ClockIndexes::RT_CLOCK_FMU_TEMPLATES.clone())?;
    if FMI::isFMIVersion20(FMUVersion)? {
        modelDescriptionStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
            __mm_s.push_str(&*Tpl::textString({ #[cfg(feature = "codegen_fmu")] { CodegenFMU2::fmiModelDescription(Tpl::emptyTxt.clone(), &simCode, &guid, FMUType, &(metamodelica::nil()))? } #[cfg(not(feature = "codegen_fmu"))] { { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU2.fmiModelDescription needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMU2.fmiModelDescription needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without") } } })?);
            ArcStr::from(__mm_s)
        };
    } else {
        modelDescriptionStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
            __mm_s.push_str(&*Tpl::textString({ #[cfg(feature = "codegen_fmu")] { CodegenFMU3::fmiModelDescription(Tpl::emptyTxt.clone(), &simCode, &guid, FMUType, &(metamodelica::nil()))? } #[cfg(not(feature = "codegen_fmu"))] { { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU3.fmiModelDescription needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMU3.fmiModelDescription needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without") } } })?);
            ArcStr::from(__mm_s)
        };
        ExecStat::execStat(&(literal!("FMU modelDescription.xml")))?;
        if (simCode.daeModeData).is_some() && FMI::isFMIMEType(FMUType) {
            lsDaeManifestStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
                __mm_s.push_str(&*Tpl::textString({ #[cfg(feature = "codegen_fmu")] { CodegenFMU3::fmiLsDaeManifest(Tpl::emptyTxt.clone(), &simCode)? } #[cfg(not(feature = "codegen_fmu"))] { { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU3.fmiLsDaeManifest needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMU3.fmiLsDaeManifest needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without") } } })?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(
                Error::FMU_EXPORT_FMI_LS_DAE_DRAFT.clone(),
                list![
                    arcstr::literal!(SimCodeCodegenUtil::FMI_LS_DAE_VERSION),
                    arcstr::literal!(SimCodeUtil::FMI_LS_DAE_DRAFT_DATE),
                    arcstr::literal!(SimCodeUtil::FMI_LS_DAE_DRAFT_COMMIT)
                ],
            )?;
            ExecStat::execStat(&(literal!("FMU fmi-ls-manifest.xml")))?;
        }
        if !(bareExport) {
            terminalsDir = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*fmutmp);
                __mm_s.push_str(&*literal!("/terminalsAndIcons/"));
                ArcStr::from(__mm_s)
            };
            terminals = SimCodeCodegenUtil::getFMI3Terminals(&simCode)?;
            if !((terminals).is_empty()) {
                Util::createDirectoryTree(terminalsDir.clone());
                System::writeFile(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*terminalsDir);
                        __mm_s.push_str(&*literal!("terminalsAndIcons.xml"));
                        ArcStr::from(__mm_s)
                    },
                    Tpl::textString({
                        #[cfg(feature = "codegen_fmu")]
                        {
                            CodegenFMU3::fmiTerminalsAndIcons(Tpl::emptyTxt.clone(), &terminals)?
                        }
                        #[cfg(not(feature = "codegen_fmu"))]
                        {
                            {
                                let _ = ::openmodelica_util::Error::addInternalError(
                                    ::arcstr::literal!(
                                        "CodegenFMU3.fmiTerminalsAndIcons needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without"
                                    ),
                                    metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                                );
                                return Err(
                                    "CodegenFMU3.fmiTerminalsAndIcons needs the 'codegen_fmu' code generation target, which this OpenModelica build was compiled without",
                                );
                            }
                        }
                    })?,
                )?;
            }
            CevalScriptBackend::generateFMI3GraphicalRepresentation(
                simCode.modelInfo.name.clone(),
                &fmutmp,
                &(simCode.fileNamePrefix.clone()),
            );
            if !(System::directoryExists(terminalsDir.clone())) {
                terminalsDir = literal!("");
            }
            ExecStat::execStat(&(literal!("FMU terminalsAndIcons.xml")))?;
        }
    }
    if !(bareExport) && writeFMUDocumentation(program, &simCode, FMUVersion, &fmutmp)? {
        documentationDir = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("/documentation/"));
            ArcStr::from(__mm_s)
        };
        ExecStat::execStat(&(literal!("FMU documentation")))?;
    }
    System::realtimeAccumulate(ClockIndexes::RT_CLOCK_FMU_TEMPLATES.clone())?;
    simulationFlagsJson = wasmFMUSimulationFlagsJson(&simCode)?;
    if FMI::isFMIMEType(FMUType) && FMI::isFMICSType(FMUType) {
        CodegenWasmJit::emitMeCsFmu(
            simCode.clone(),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*simCode.fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            },
            guid,
            modelDescriptionStr,
            lsDaeManifestStr,
            documentationDir,
            terminalsDir,
            simulationFlagsJson,
        )?;
    } else if FMI::isFMICSType(FMUType) {
        CodegenWasmJit::emitCsFmu(
            simCode.clone(),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*simCode.fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            },
            guid,
            modelDescriptionStr,
            lsDaeManifestStr,
            documentationDir,
            terminalsDir,
            simulationFlagsJson,
        )?;
    } else {
        CodegenWasmJit::emitMeFmu(
            simCode.clone(),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*simCode.fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            },
            guid,
            modelDescriptionStr,
            lsDaeManifestStr,
            documentationDir,
            terminalsDir,
            simulationFlagsJson,
        )?;
    }
    {
        let __v = None;
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

fn callTargetTemplatesFMU(
    mut inSimCode: metamodelica::Ref<SimCode::SimCode>,
    mut target: ArcStr,
    mut FMUVersion: ArcStr,
    mut FMUType: ArcStr,
    mut program: Absyn::Program,
    mut translateOnly: bool,
) -> Result<()> {
    let mut simCode: metamodelica::Ref<SimCode::SimCode> = SimCodeUtil::addFMI3Figures(inSimCode.clone(), &FMUVersion)?;
    let mut fmuTarget: ArcStr = if (metamodelica::stringEq(&target, &(literal!("wasm")))) {
        literal!("wasm-jit")
    } else {
        target.clone()
    };
    {
        let __v = Some(simCode.clone());
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    SimCodeFunctionUtil::setTrivialRecords(&simCode.recordDecls)?;
    let () = ({
        let mut needSundials: bool = false;
        (::match_deref::match_deref! { match &((simCode.clone(), fmuTarget)) {
            (Deref @ SimCode::SimCode { .. }, Deref @ "wasm-jit") => {
                if translateOnly {
                    CodegenWasmJit::translateFmu(simCode.clone(), FMUType.clone(), wasmFMUSimulationFlagsJson(&simCode)?)?;
                } else {
                    emitWasmFMU(simCode.clone(), &FMUVersion, &FMUType, program)?;
                }
                keepFmuTranslation(simCode.clone(), FMUVersion, &FMUType, simCode.modelInfo.name.clone())?;
                ()
            },
            (Deref @ SimCode::SimCode { .. }, Deref @ "C") => {
                let mut r#str: ArcStr;
                let mut newdir: ArcStr;
                let mut newpath: ArcStr;
                let mut resourcesDir: ArcStr;
                let mut dirname: ArcStr;
                let mut fileName: ArcStr;
                let mut fmutmp: ArcStr;
                let mut b: bool;
                let mut fileNamePrefixHash: ArcStr;
                let mut install_include_omc_dir: ArcStr;
                let mut install_include_omc_c_dir: ArcStr;
                let mut install_share_buildproject_dir: ArcStr;
                let mut install_fmu_sources_dir: ArcStr;
                let mut fmu_tmp_sources_dir: ArcStr;
                let mut cmakelistsStr: ArcStr;
                let mut needCvode: ArcStr;
                let mut cvodeDirectory: ArcStr;
                let mut modelDefinesHeaderStr: ArcStr;
                let mut fmu_dummy_include_defines: ArcStr;
                let mut needModelicaExternalC: ArcStr;
                let mut cmakeCode: ArcStr;
                let mut model_desc_src_files: metamodelica::List<ArcStr>;
                let mut fmi2HeaderFiles: metamodelica::List<ArcStr>;
                let mut modelica_standard_table_sources: metamodelica::List<ArcStr>;
                let mut dgesv_sources: metamodelica::List<ArcStr>;
                let mut cminpack_sources: metamodelica::List<ArcStr>;
                let mut simrt_c_sundials_sources: metamodelica::List<ArcStr>;
                let mut simrt_linear_solver_sources: metamodelica::List<ArcStr>;
                let mut simrt_non_linear_solver_sources: metamodelica::List<ArcStr>;
                let mut simrt_mixed_solver_sources: metamodelica::List<ArcStr>;
                let mut fmi_export_files: metamodelica::List<ArcStr>;
                let mut model_gen_files: metamodelica::List<ArcStr>;
                let mut model_all_gen_files: metamodelica::List<ArcStr>;
                let mut shared_source_files: metamodelica::List<ArcStr>;
                let mut rust_crate_roots: metamodelica::List<ArcStr>;
                let mut simrt_c_sources: metamodelica::List<ArcStr>;
                let mut rustRuntime: bool;
                let mut rustFmi: bool;
                let mut varInfo: SimCode::VarInfo;
                fileNamePrefixHash = Util::hashFileNamePrefix(&simCode.fileNamePrefix)?;
                fmutmp = { let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefixHash); __mm_s.push_str(&*literal!(".fmutmp")); ArcStr::from(__mm_s) };
                if System::directoryExists(fmutmp.clone()) {
                    if !(System::removeDirectory(fmutmp.clone())) {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*fmutmp); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                        return Err("fail");
                    }
                }
                Util::createDirectoryTree({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/include/")); ArcStr::from(__mm_s) });
                resourcesDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/resources/")); ArcStr::from(__mm_s) };
                Util::createDirectoryTree(resourcesDir.clone());
                for mut path in &*simCode.modelInfo.resourcePaths.clone() {
                    dirname = System::dirname(path.clone());
                    newpath = path.clone();
                    if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
                        dirname = System::stringReplace(dirname, literal!(":"), literal!(""))?;
                        newpath = System::stringReplace(newpath, literal!(":"), literal!(""))?;
                    }
                    newdir = { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*dirname); ArcStr::from(__mm_s) };
                    newpath = { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*newpath); ArcStr::from(__mm_s) };
                    if System::regularFileExists(newpath.clone()) || System::directoryExists(newpath.clone()) {
                        continue;
                    }
                    Util::createDirectoryTree(newdir);
                    if !(System::copyPath(path.clone(), newpath)) {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy path ")); __mm_s.push_str(&*path); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*dirname); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                    }
                }
                let () = (match simCode.fmiSimulationFlags.clone() {
            Some(SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS_FILE { path: mut pathToFlagsJson }) => {
                needSundials = true;
                if !(System::copyFile(pathToFlagsJson.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_flags.json")); ArcStr::from(__mm_s) })) {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy ")); __mm_s.push_str(&*pathToFlagsJson); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_flags.json")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                }
                ()
            },
            _ => {
                ()
            },
        });
                if Flags::isSet(Flags::VISUAL_XML.clone())? && System::regularFileExists({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_visual.xml")); ArcStr::from(__mm_s) }) {
                    if !(System::copyFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_visual.xml")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_visual.xml")); ArcStr::from(__mm_s) })) {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy ")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_visual.xml to ")); __mm_s.push_str(&*resourcesDir); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                    }
                    for mut cad in &*visualizationCadFiles({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_visual.xml")); ArcStr::from(__mm_s) })? {
                        if System::regularFileExists(cad.clone()) && !(System::copyFile(cad.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*System::basename(cad.clone())); ArcStr::from(__mm_s) })) {
                            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy CAD file ")); __mm_s.push_str(&*cad); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                        }
                    }
                }
                SerializeSparsityPattern::serialize(simCode.clone())?;
                for mut jac in &*simCode.jacobianMatrices.clone() {
                    if !((jac.sparsity).is_empty()) {
                        fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName); __mm_s.push_str(&*literal!(".bin")); ArcStr::from(__mm_s) };
                        if !(System::rename(fileName.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) })) {
                            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName); __mm_s.push_str(&*literal!(".bin file")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                        }
                    }
                }
                SerializeModelInfo::serialize(&simCode, Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/")); __mm_s.push_str(&*simCode.fileNamePrefix); ArcStr::from(__mm_s) };
                if metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) {
                    b = System::covertTextFileToCLiteral({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("_info.c")); ArcStr::from(__mm_s) }, Flags::getConfigString(Flags::TARGET.clone())?);
                    if !(b) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("System.covertTextFileToCLiteral failed. Could not write ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("_info.c\n")); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                    }
                } else {
                    if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_BLACKBOX.clone() && Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_PROTECTED.clone() {
                        fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) };
                        if !(System::rename(fileName.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) })) {
                            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_info.json file")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                        }
                    }
                }
                SimCodeCodegenUtil::resetFunctionIndex()?;
                varInfo = simCode.modelInfo.varInfo.clone();
                install_include_omc_dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/")); ArcStr::from(__mm_s) };
                install_include_omc_c_dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*install_include_omc_dir); __mm_s.push_str(&*literal!("c/")); ArcStr::from(__mm_s) };
                install_share_buildproject_dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/share/omc/runtime/c/fmi/buildproject/")); ArcStr::from(__mm_s) };
                install_fmu_sources_dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*arcstr::literal!(RuntimeSources::fmu_sources_dir)); ArcStr::from(__mm_s) };
                fmu_tmp_sources_dir = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) };
                rustRuntime = Config::simCodeRustRuntime()?;
                rustFmi = rustRuntime;
                simrt_c_sources = if (rustRuntime) {RuntimeSources::simrt_c_runtime_sources.clone()} else {RuntimeSources::simrt_c_sources.clone()};
                if rustRuntime && Flags::getConfigEnum(Flags::FMI_SOURCES.clone())? != Flags::FMI_SOURCES_NONE.clone() && Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_BLACKBOX.clone() {
                    copyFmuRustSources(&fmutmp)?;
                    rust_crate_roots = fmuRustCrateRoots(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) }), &(literal!("rust")))?;
                } else {
                    rust_crate_roots = metamodelica::nil();
                }
                copyFiles(&(RuntimeSources::simrt_c_headers.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                copyFiles(&simrt_c_sources, &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                if rustRuntime {
                    dgesv_sources = metamodelica::nil();
                    cminpack_sources = metamodelica::nil();
                } else {
                    copyFiles(&(RuntimeSources::dgesv_headers.clone()), &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                    copyFiles(&(RuntimeSources::dgesv_sources.clone()), &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                    dgesv_sources = RuntimeSources::dgesv_sources.clone();
                    copyFiles(&(RuntimeSources::cminpack_headers.clone()), &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                    copyFiles(&(RuntimeSources::cminpack_sources.clone()), &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                    cminpack_sources = RuntimeSources::cminpack_sources.clone();
                }
                if metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) {
                    if SimCodeUtil::cvodeFmiFlagIsSet(simCode.fmiSimulationFlags.clone())? {
                        Error::addCompilerWarning(literal!("OpenModelica exports FMI 1.0 as Model Exchange only (Co-Simulation export requires FMI 2.0). A model-exchange FMU is integrated by the importer, so the in-FMU CVODE integrator does not apply. The 's:cvode' simulation flag is ignored for this FMI 1.0 export."))?;
                    }
                    simrt_c_sundials_sources = metamodelica::nil();
                } else if !(rustFmi) && SimCodeUtil::cvodeFmiFlagIsSet(simCode.fmiSimulationFlags.clone())? {
                    copyFiles(&(RuntimeSources::sundials_headers.clone()), &install_include_omc_dir, &fmu_tmp_sources_dir)?;
                    copyFiles(&(RuntimeSources::simrt_c_sundials_sources.clone()), &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                    simrt_c_sundials_sources = RuntimeSources::simrt_c_sundials_sources.clone();
                } else {
                    simrt_c_sundials_sources = metamodelica::nil();
                }
                simrt_linear_solver_sources = if (varInfo.numLinearSystems.clone() > 0 && !(rustRuntime)) {RuntimeSources::simrt_linear_solver_sources.clone()} else {metamodelica::nil()};
                copyFiles(&simrt_linear_solver_sources, &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                simrt_non_linear_solver_sources = if (varInfo.numNonLinearSystems.clone() > 0 && !(rustRuntime)) {RuntimeSources::simrt_non_linear_solver_sources.clone()} else {metamodelica::nil()};
                copyFiles(&simrt_non_linear_solver_sources, &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                simrt_mixed_solver_sources = if (varInfo.numMixedSystems.clone() > 0 && !(rustRuntime)) {RuntimeSources::simrt_mixed_solver_sources.clone()} else {metamodelica::nil()};
                copyFiles(&simrt_mixed_solver_sources, &install_fmu_sources_dir, &fmu_tmp_sources_dir)?;
                if metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) && rustFmi {
                    copyFiles(&(RuntimeSources::fmi1_rust_headers.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    fmi_export_files = metamodelica::nil();
                } else if metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) {
                    copyFiles(&(RuntimeSources::fmi1Files.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    fmi_export_files = RuntimeSources::fmi1Files.clone();
                } else if metamodelica::stringEq(&FMUVersion, &(literal!("3.0"))) {
                    if !(rustFmi) {
                        copyFiles(&(RuntimeSources::fmi3_sources.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    }
                    copyFiles(&(RuntimeSources::fmi3_headers.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    fmi_export_files = if (rustFmi) {metamodelica::nil()} else {RuntimeSources::fmi3_sources.clone()};
                } else if rustFmi {
                    copyFiles(&(RuntimeSources::fmi2_headers.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    fmi_export_files = metamodelica::nil();
                } else {
                    copyFiles(&(RuntimeSources::fmi2_sources.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    copyFiles(&(RuntimeSources::fmi2_headers.clone()), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    fmi_export_files = RuntimeSources::fmi2_sources.clone();
                }
                fmi2HeaderFiles = list![literal!("fmi/fmi2Functions.h"), literal!("fmi/fmi2FunctionTypes.h"), literal!("fmi/fmi2TypesPlatform.h"), literal!("fmi/fmiModelFunctions.h"), literal!("fmi/fmiModelTypes.h")];
                copyFiles(&fmi2HeaderFiles, &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                if metamodelica::stringEq(&FMUVersion, &(literal!("3.0"))) {
                    copyFiles(&(list![literal!("fmi/fmi3Functions.h"), literal!("fmi/fmi3FunctionTypes.h"), literal!("fmi/fmi3PlatformTypes.h")]), &install_include_omc_c_dir, &fmu_tmp_sources_dir)?;
                    if !(((SimCodeCodegenUtil::getFMI3Terminals(&simCode)?)).is_empty()) {
                        Util::createDirectoryTree({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/terminalsAndIcons/")); ArcStr::from(__mm_s) });
                    }
                }
                copyFiles(&(RuntimeSources::modelica_external_c_sources.clone()), &install_include_omc_dir, &fmu_tmp_sources_dir)?;
                copyFiles(&(RuntimeSources::modelica_external_c_headers.clone()), &install_include_omc_dir, &fmu_tmp_sources_dir)?;
                modelica_standard_table_sources = RuntimeSources::modelica_external_c_sources.clone();
                System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/isfmi")); __mm_s.push_str(&*if (metamodelica::stringEq(&FMUVersion, &(literal!("1.0")))) {literal!("1")} else if (metamodelica::stringEq(&FMUVersion, &(literal!("3.0")))) {literal!("3")} else {literal!("2")}); ArcStr::from(__mm_s) }, literal!(""))?;
                model_gen_files = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut f in (RuntimeSources::defaultFileSuffixes.clone()).into_iter().cloned() {
                let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*f); ArcStr::from(__mm_s) };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                shared_source_files = List::flatten(list![fmi_export_files, simrt_c_sources, simrt_linear_solver_sources, simrt_non_linear_solver_sources, simrt_mixed_solver_sources])?;
                if Flags::getConfigEnum(Flags::FMI_SOURCES.clone())? == Flags::FMI_SOURCES_NONE.clone() || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone() {
                    model_desc_src_files = metamodelica::nil();
                } else {
                    model_desc_src_files = List::flatten(list![List::sort(model_gen_files.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(shared_source_files, (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(dgesv_sources, (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(cminpack_sources, (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(simrt_c_sundials_sources, (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(modelica_standard_table_sources, (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, rust_crate_roots])?;
                }
                Tpl::tplNoret({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new({ let __pe_b2 = FMUVersion.clone(); let __pe_b3 = FMUType.clone(); let __pe_b4 = model_desc_src_files; move |__pe_a0, __pe_a1| CodegenFMU::translateModel(__pe_a0, &__pe_a1, &__pe_b2, &__pe_b3, &__pe_b4) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU.translateModel needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMU.translateModel needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, simCode.clone())?;
                if metamodelica::stringEq(&FMUVersion, &(literal!("3.0"))) {
                    CevalScriptBackend::generateFMI3GraphicalRepresentation(simCode.modelInfo.name.clone(), &fmutmp, &(simCode.fileNamePrefix.clone()));
                }
                if writeFMUDocumentation(program, &simCode, &FMUVersion, &fmutmp)? {
                    ExecStat::execStat(&(literal!("FMU documentation")))?;
                }
                model_all_gen_files = listAppend(model_gen_files, SimCodeUtil::getFunctionIndex()?);
                System::copyFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*install_share_buildproject_dir); __mm_s.push_str(&*literal!("CMakeLists.txt.in")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) });
                cmakelistsStr = System::readFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) })?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMU_NAME_HASH_IN@"), fileNamePrefixHash)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMU_NAME_IN@"), simCode.fileNamePrefix.clone())?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMU_TARGET_NAME@"), simCode.fmuTargetName.clone())?;
                if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
                    cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@CMAKE_BUILD_TYPE@"), literal!("Debug"))?;
                } else {
                    cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@CMAKE_BUILD_TYPE@"), literal!("Release"))?;
                }
                let () = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?) {
            Deref @ "default" => {
                let mut cmakeVersion: SemanticVersion::Version;
                let mut minimumVersion: SemanticVersion::Version;
                cmakeVersion = SimCodeUtil::getCMakeVersion(&(arcstr::literal!(Autoconf::cmake)))?;
                minimumVersion = SemanticVersion::Version::SEMVER { major: 3, minor: 21, patch: 0, prerelease: metamodelica::nil(), meta: metamodelica::nil() };
                if SemanticVersion::compare(&minimumVersion, &cmakeVersion, true, false)? <= 0 {
                    cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUNTIME_DEPENDENCIES_LEVEL@"), literal!("\"modelica\""))?;
                } else {
                    cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUNTIME_DEPENDENCIES_LEVEL@"), literal!("\"none\""))?;
                }
                ()
            },
            Deref @ "none" => {
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUNTIME_DEPENDENCIES_LEVEL@"), literal!("\"none\""))?;
                ()
            },
            Deref @ "modelica" => {
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUNTIME_DEPENDENCIES_LEVEL@"), literal!("\"modelica\""))?;
                ()
            },
            Deref @ "all" => {
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUNTIME_DEPENDENCIES_LEVEL@"), literal!("\"all\""))?;
                ()
            },
            _ => {
                Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unsupported value ")); __mm_s.push_str(&*Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?); __mm_s.push_str(&*literal!("for compiler flag 'fmuRuntimeDepends'.")); ArcStr::from(__mm_s) })?;
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMI_INTERFACE_HEADER_FILES_DIRECTORY@"), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/c/fmi")); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) })?;
                if metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) || rustFmi {
                    (needCvode, cvodeDirectory) = (literal!("OFF"), literal!("\"\""));
                } else {
                    (needCvode, cvodeDirectory) = SimCodeUtil::getCmakeSundialsLinkCode(simCode.fmiSimulationFlags.clone())?;
                }
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@NEED_CVODE@"), needCvode)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@CVODE_DIRECTORY@"), cvodeDirectory)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@OMC_RUST_SIMULATION_RUNTIME@"), if (rustRuntime) {literal!("ON")} else {literal!("OFF")})?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@RUST_SIM_RUNTIME_LIBRARY@"), if (rustRuntime) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"${DOCKER_VOL_DIR}")); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/${OM_LIBRARY_ARCH}/omc/libSimulationRuntimeRust.a\"")); ArcStr::from(__mm_s) }} else {literal!("\"\"")})?;
                (needModelicaExternalC, cmakeCode) = SimCodeUtil::getCmakeLinkLibrariesCode(&simCode.makefileParams.libs)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@COMPILE_MODELICA_EXTERNAL_C@"), needModelicaExternalC)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMU_ADDITIONAL_LIBS@"), cmakeCode)?;
                cmakelistsStr = System::stringReplace(cmakelistsStr, literal!("@FMU_ADDITIONAL_INCLUDES@"), SimCodeUtil::make2CMakeInclude(&simCode.makefileParams.includes)?)?;
                System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }, cmakelistsStr)?;
                System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/.external_include_dirs")); ArcStr::from(__mm_s) }, stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut i in (simCode.makefileParams.includes.clone()).into_iter().cloned() {
                let __x = SimCodeUtil::stripIncludeFlag(i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!("\n")))?;
                if !metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) && !(rustFmi) {
                    fmu_dummy_include_defines = { let mut __mm_s = String::new(); __mm_s.push_str(&*if (metamodelica::stringEq(&FMUVersion, &(literal!("2.0")))) {literal!("fmu2")} else {literal!("fmu3")}); __mm_s.push_str(&*literal!("_dummy_model_defines.h")); ArcStr::from(__mm_s) };
                    for mut fmuModelInterfaceFile in &*if (metamodelica::stringEq(&FMUVersion, &(literal!("3.0")))) {list![literal!("fmi-export/fmu3_model_interface.c")]} else {list![literal!("fmi-export/fmu2_model_interface.c")]} {
                        modelDefinesHeaderStr = System::readFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir); __mm_s.push_str(&*fmuModelInterfaceFile); ArcStr::from(__mm_s) })?;
                        modelDefinesHeaderStr = System::stringReplace(modelDefinesHeaderStr, fmu_dummy_include_defines.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("../")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_FMU.h")); ArcStr::from(__mm_s) })?;
                        System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir); __mm_s.push_str(&*fmuModelInterfaceFile); ArcStr::from(__mm_s) }, modelDefinesHeaderStr)?;
                    }
                }
                Tpl::closeFile(Tpl::tplCallWithFailError({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| CodegenFMU::settingsfile(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU.settingsfile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMU.settingsfile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, simCode.clone(), Tpl::redirectToFile(Tpl::emptyTxt.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/sources/omc_simulation_settings.h")); ArcStr::from(__mm_s) })?)?)?;
                if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsicpp"))) {
                    runTpl({ #[cfg(feature = "cpp")] { (std::sync::Arc::new({ let __pe_b1 = simCode; let __pe_b2 = FMUVersion; let __pe_b3 = FMUType; move |__pe_a0| CodegenOMSICpp::translateModel(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "cpp"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenOMSICpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenOMSICpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } });
                }
                ()
            },
            (_, Deref @ "omsic") => {
                let mut guid: ArcStr;
                let mut fileprefix: ArcStr;
                guid = System::getUUIDStr();
                fileprefix = simCode.fileNamePrefix.clone();
                if System::directoryExists(simCode.fullPathPrefix.clone()) {
                    if !(System::removeDirectory(simCode.fullPathPrefix.clone())) {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*simCode.fullPathPrefix); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                        return Err("fail");
                    }
                }
                if !(System::createDirectory(simCode.fullPathPrefix.clone())) {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create tmp folder ")); __mm_s.push_str(&*simCode.fullPathPrefix); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"))?;
                    System::fflush();
                    return Err("fail");
                }
                SerializeInitXML::simulationInitFileReturnBool(&simCode, guid.clone())?;
                SerializeSparsityPattern::serialize(simCode.clone())?;
                SerializeModelInfo::serialize(&simCode, Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
                runTpl({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = guid; let __pe_b3 = FMUVersion; let __pe_b4 = FMUType; let __pe_b5 = metamodelica::nil(); let __pe_b6 = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("modelDescription.xml")); ArcStr::from(__mm_s) }; move |__pe_a0| CodegenOMSI_common::generateFMUModelDescriptionFile(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3, &__pe_b4, &__pe_b5, &__pe_b6) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenOMSI_common.generateFMUModelDescriptionFile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenOMSI_common.generateFMUModelDescriptionFile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } });
                runTplWriteFile({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = Config::simulationCodeTarget()?; let __pe_b3 = { let mut __mm_s = String::new(); __mm_s.push_str(&*fileprefix); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) }; move |__pe_a0| CodegenOMSIC::createMakefile(__pe_a0, &__pe_b1, &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenOMSIC.createMakefile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenOMSIC.createMakefile needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }, { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) });
                runTplWriteFile({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenOMSIC::generateOMSIC(__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenOMSIC.generateOMSIC needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenOMSIC.generateOMSIC needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } }, { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix); __mm_s.push_str(&*literal!("_omsic.c")); ArcStr::from(__mm_s) });
                runTpl({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new({ let __pe_b1 = simCode; let __pe_b2 = fileprefix; move |__pe_a0| CodegenOMSI_common::generateEquationsCode(__pe_a0, &__pe_b1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenOMSI_common.generateEquationsCode needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenOMSI_common.generateEquationsCode needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) } });
                ()
            },
            (_, Deref @ "Cpp") => {
                if Flags::isSet(Flags::HPCOM.clone())? {
                    Tpl::tplNoret3({ #[cfg(feature = "cpp")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>, __a2: ArcStr, __a3: ArcStr| CodegenFMUCppHpcom::translateModel(__a0, &__a1, &__a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "cpp"))] { (std::sync::Arc::new(|_a0, _a1, _a2, _a3| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMUCppHpcom.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMUCppHpcom.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>) } }, simCode, FMUVersion, FMUType)?;
                } else {
                    Tpl::tplNoret({ #[cfg(feature = "cpp")] { (std::sync::Arc::new({ let __pe_b2 = FMUVersion; let __pe_b3 = FMUType; let __pe_b4 = metamodelica::nil(); move |__pe_a0, __pe_a1| CodegenFMUCpp::translateModel(__pe_a0, &__pe_a1, &__pe_b2, &__pe_b3, &__pe_b4) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "cpp"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMUCpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")); return Err("CodegenFMUCpp.translateModel needs the 'cpp' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static>) } }, simCode.clone())?;
                    if !metamodelica::stringEq(&(Flags::getConfigString(Flags::FMI_EXTRA_ANNOTATIONS.clone())?), &(literal!(""))) {
                        System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_modelInstance.json")); ArcStr::from(__mm_s) }, ValuesUtil::extractValueString(&(NFApi::getModelInstance(simCode.modelInfo.name.clone(), simCode.modelInfo.name.clone(), &(literal!("")), true)?))?)?;
                    }
                }
                ()
            },
            _ => {
                let mut r#str: ArcStr;
                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown FMU template target: ")); __mm_s.push_str(&*target); ArcStr::from(__mm_s) };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    {
        let __v = None;
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

// `svgiconhead` is sized as the generated library documentation sizes it: a
// little larger than the class name it sits beside, not a banner.
pub(crate) const FMU_DOCUMENTATION_CSS: &'static str = "\n  :root { color-scheme: light dark; }\n  body { margin: 0 auto; padding: 1.5rem; max-width: 50rem; line-height: 1.5;\n         font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif; }\n  h1 { font-size: 1.5rem; margin: 0 0 .2rem; }\n  .svgiconhead { height: 32px; width: 32px; object-fit: contain; vertical-align: middle; }\n  .fmu-desc { margin: 0 0 1.5rem; font-style: italic; opacity: .8; }\n  img { max-width: 100%; height: auto; }\n  table { border-collapse: collapse; }\n  td, th { border: 1px solid; border-color: color-mix(in srgb, currentColor 30%, transparent);\n           padding: .2rem .5rem; }\n  pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; }\n";

fn writeFMUDocumentation(
    mut program: Absyn::Program,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
    mut FMUVersion: &ArcStr,
    mut fmutmp: &ArcStr,
) -> Result<bool> {
    let mut written: bool = false;
    let mut info: ArcStr;
    let mut revisions: ArcStr;
    let mut infoHeader: ArcStr;
    let mut docDir: ArcStr;
    let mut name: ArcStr;
    let mut page: ArcStr;
    let mut icon: ArcStr;
    (info, revisions, infoHeader) = ProgramUtil::getNamedAnnotationExp(
        simCode.modelInfo.name.clone(),
        program.clone(),
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("Documentation"),
        })),
        Some((literal!(""), literal!(""), literal!(""))),
        &Interactive::getDocumentationAnnotationString,
    )?;
    if stringEmpty(&info) && stringEmpty(&revisions) && stringEmpty(&infoHeader) {
        return Ok(written);
    }
    name = Util::escapeModelicaStringToXmlString(AbsynUtil::pathString(
        simCode.modelInfo.name.clone(),
        literal!("."),
        true,
        false,
    )?)?;
    icon = if (metamodelica::stringEq(&FMUVersion, &(literal!("3.0")))
        && System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("/terminalsAndIcons/icon.svg"));
            ArcStr::from(__mm_s)
        })) {
        literal!("<img class=\"svgiconhead\" src=\"../terminalsAndIcons/icon.svg\" alt=\"\"> ")
    } else {
        literal!("")
    };
    page = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n"
        ));
        __mm_s.push_str(&*literal!(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n"
        ));
        __mm_s.push_str(&*literal!("<title>"));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("</title>\n"));
        __mm_s.push_str(&*literal!("<style>"));
        __mm_s.push_str(&*arcstr::literal!(FMU_DOCUMENTATION_CSS));
        __mm_s.push_str(&*literal!("</style>\n"));
        __mm_s.push_str(&*literal!("</head>\n<body>\n"));
        __mm_s.push_str(&*literal!("<h1>"));
        __mm_s.push_str(&*icon);
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("</h1>\n"));
        __mm_s.push_str(&*if (stringEmpty(&simCode.modelInfo.description)) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("<p class=\"fmu-desc\">"));
                __mm_s.push_str(&*Util::escapeModelicaStringToXmlString(
                    simCode.modelInfo.description.clone(),
                )?);
                __mm_s.push_str(&*literal!("</p>\n"));
                ArcStr::from(__mm_s)
            }
        });
        __mm_s.push_str(&*fmuDocumentationSection(&(literal!("")), infoHeader)?);
        __mm_s.push_str(&*fmuDocumentationSection(&(literal!("Information")), info)?);
        __mm_s.push_str(&*fmuDocumentationSection(&(literal!("Revisions")), revisions)?);
        __mm_s.push_str(&*literal!("</body>\n</html>\n"));
        ArcStr::from(__mm_s)
    };
    docDir = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fmutmp);
        __mm_s.push_str(&*literal!("/documentation/"));
        ArcStr::from(__mm_s)
    };
    Util::createDirectoryTree(docDir.clone());
    page = copyFMUDocumentationResources(&program, page, &docDir)?;
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*docDir);
            __mm_s.push_str(&*if (metamodelica::stringEq(&FMUVersion, &(literal!("1.0")))) {
                literal!("_main.html")
            } else {
                literal!("index.html")
            });
            ArcStr::from(__mm_s)
        },
        page,
    )?;
    written = true;
    Ok(written)
}

fn fmuDocumentationSection(mut heading: &ArcStr, mut html: ArcStr) -> Result<ArcStr> {
    let mut section: ArcStr = literal!("");
    let mut body: ArcStr;
    let mut len: i32;
    body = System::trim(html, literal!(" \u{c}\n\r\t\u{b}"));
    if StringUtil::startsWith(System::tolower(body.clone()), literal!("<html>")) {
        len = ((body).len() as i32);
        body = if (len > 6) {
            substring(body, 7, len)?
        } else {
            literal!("")
        };
        len = ((body).len() as i32);
        if StringUtil::endsWith(System::tolower(body.clone()), literal!("</html>")) {
            body = if (len > 7) {
                substring(body, 1, len - 7)?
            } else {
                literal!("")
            };
        }
        body = System::trim(body, literal!(" \u{c}\n\r\t\u{b}"));
    }
    if stringEmpty(&body) {
        return Ok(section);
    }
    section = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*if (stringEmpty(&heading)) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("<h2>"));
                __mm_s.push_str(&*heading);
                __mm_s.push_str(&*literal!("</h2>\n"));
                ArcStr::from(__mm_s)
            }
        });
        __mm_s.push_str(&*body);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(section)
}

fn copyFMUDocumentationResources(
    mut program: &Absyn::Program,
    mut page: ArcStr,
    mut docDir: &ArcStr,
) -> Result<ArcStr> {
    let mut page: ArcStr = page;
    let mut uris: metamodelica::List<ArcStr>;
    let mut source: ArcStr;
    let mut classname: ArcStr;
    let mut relative: ArcStr;
    let mut target: ArcStr;
    uris = List::uniqueOnTrue(
        &({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut u in (System::strtok(page.clone(), literal!("\"'<> \t\n\r")))
                .into_iter()
                .cloned()
            {
                if !(StringUtil::startsWith(u.clone(), literal!("modelica://"))) {
                    continue;
                }
                let __x = u.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        &fnptr!(stringEq, ArcStr, ArcStr),
    )?;
    for mut uri in &*uris {
        if '__try0: {
            (_, classname, relative) = unwrap_break_err!(System::uriToClassAndPath(uri.clone()), '__try0);
            source = unwrap_break_err!(ProgramUtil::getFullPathFromUri(program, uri.clone(), false), '__try0);
            if System::regularFileExists(source.clone()) {
                target = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*classname);
                    __mm_s.push_str(&*relative);
                    ArcStr::from(__mm_s)
                };
                Util::createDirectoryTree({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*docDir);
                    __mm_s.push_str(&*System::dirname(target.clone()));
                    ArcStr::from(__mm_s)
                });
                if System::copyFile(source.clone(), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*docDir);
                    __mm_s.push_str(&*target);
                    ArcStr::from(__mm_s)
                }) {
                    page = unwrap_break_err!(System::stringReplace(page.clone(), uri.clone(), target.clone()), '__try0);
                }
            }
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    Ok(page)
}

fn callTargetTemplatesXML(mut simCode: metamodelica::Ref<SimCode::SimCode>, mut target: &ArcStr) -> Result<()> {
    Tpl::tplNoret(
        (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<SimCode::SimCode>| {
            CodegenXML::translateModel(__a0, &__a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<SimCode::SimCode>) -> Result<Tpl::Text> + 'static,
            >),
        simCode,
    )?;
    Ok(())
}

pub(crate) fn translateModel(
    mut kind: TranslateModelKind,
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut runBackend: bool,
    mut useDAEMode: bool,
    mut runSilent: bool,
    mut inSimSettingsOpt: Option<SimCode::SimulationSettings>,
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<(
    bool,
    FCore::Cache,
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
)> {
    let mut success: bool;
    let mut cache: FCore::Cache = cache;
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut inCache: FCore::Cache = cache.clone();
    let mut timeFrontend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut dae: DAE::DAElist;
    let mut env: FCore::Graph;
    let mut odae: Option<DAE::DAElist>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut allRoots: metamodelica::List<Option<i32>>;
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut funcTree: metamodelica::Ref<FunctionTreeImpl::Tree>;
    let mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<NFFunction::Function::Function>>,
    >;
    let mut dumpValidFlatModelicaNF: bool;
    let mut flatString: ArcStr = literal!("");
    let mut NFFlatString: ArcStr = literal!("");
    List::map_0(
        &(list![
            ClockIndexes::RT_CLOCK_FRONTEND.clone(),
            ClockIndexes::RT_CLOCK_BACKEND.clone(),
            ClockIndexes::RT_CLOCK_SIMCODE.clone(),
            ClockIndexes::RT_CLOCK_TEMPLATES.clone(),
            ClockIndexes::RT_CLOCK_BUILD_MODEL.clone(),
            ClockIndexes::RT_CLOCK_FMU_BACKEND.clone(),
            ClockIndexes::RT_CLOCK_FMU_SIMCODE.clone(),
            ClockIndexes::RT_CLOCK_FMU_TEMPLATES.clone()
        ]),
        &System::realtimeClear,
    )?;
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    outLibs = metamodelica::nil();
    outFileDir = literal!("");
    resultValues = metamodelica::nil();
    dumpValidFlatModelicaNF = !(runSilent) && Config::flatModelica()?;
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
            FlagsUtil::setConfigBool(Flags::SIM_CODE_SCALARIZE.clone(), false)?;
        }
        System::realtimeTick(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStatReset()?;
        (flatModel, funcTree, NFFlatString) =
            CevalScriptBackend::runFrontEndWorkNF(className.clone(), false, dumpValidFlatModelicaNF)?;
        timeFrontend = System::realtimeTock(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStat(&(literal!("FrontEnd")))?;
        if runBackend {
            funcMap = UnorderedMap::fromLists(
                &(FunctionTreeImpl::listKeys(&funcTree, metamodelica::nil())),
                FunctionTreeImpl::listValues(&funcTree, metamodelica::nil()),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Absyn::Path>,
                          __a1: metamodelica::Ref<Absyn::Path>|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Path>,
                                metamodelica::Ref<Absyn::Path>,
                            ) -> Result<bool>
                            + 'static,
                    >),
            )?;
            (outLibs, outFileDir, resultValues, funcs) = translateModelCallBackendNB(
                &flatModel,
                funcMap,
                className,
                inFileNamePrefix,
                inSimSettingsOpt,
                &kind,
            )?;
        } else {
            funcs = NFConvertDAE::convertFunctionTree(&funcTree)?;
        }
        if dumpValidFlatModelicaNF {
            flatString = NFFlatString;
        } else if !(runSilent) {
            dae = NFConvertDAE::convertModel(&flatModel)?;
            flatString = DAEDump::dumpStr(dae, &funcs)?;
        }
    } else {
        System::realtimeTick(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStatReset()?;
        (cache, env, odae, NFFlatString) = CevalScriptBackend::runFrontEnd(
            cache,
            inEnv.clone(),
            className.clone(),
            false,
            dumpValidFlatModelicaNF,
            false,
        )?;
        ExecStat::execStat(&(literal!("FrontEnd")))?;
        let __pa0 = ::match_deref::match_deref! { match &(odae) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        dae = metamodelica::Own::own(__pa0);
        if dumpValidFlatModelicaNF {
            flatString = NFFlatString;
        } else if !(runSilent) {
            funcs = FCore::getFunctionTree(&cache);
            flatString = DAEDump::dumpStr(dae.clone(), &funcs)?;
        }
        if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
            allRoots = metamodelica::nil();
            for mut i in 1..=300 {
                if '__try1: {
                    allRoots = metamodelica::cons(metamodelica::getGlobalRoot(i)?, allRoots.clone());
                    Ok::<(), &'static str>(())
                }
                .is_err()
                {}
            }
            serializeNotify(allRoots, literal!("All local+global roots (1:300)"))?;
            serializeNotify(dae.clone(), literal!("FrontEnd DAE"))?;
            serializeNotify(
                (env.clone(), inEnv.clone(), cache.clone(), inCache.clone()),
                literal!("FCore.Graph + Cache + Old graph + Old cache"),
            )?;
            serializeNotify(
                (
                    SymbolTable::get(),
                    dae.clone(),
                    env.clone(),
                    inEnv,
                    cache.clone(),
                    inCache,
                ),
                literal!("Symbol Table, DAE, Graph, OldGraph, Cache, OldCache"),
            )?;
            ExecStat::execStat(&(literal!("Serialize FrontEnd")))?;
        }
        timeFrontend = System::realtimeTock(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        if runBackend {
            if useDAEMode {
                (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOBDAEMode(
                    cache,
                    env,
                    dae,
                    className,
                    inFileNamePrefix,
                    inSimSettingsOpt,
                    args,
                    &kind,
                )?;
            } else {
                (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOB(
                    kind,
                    cache,
                    env,
                    dae,
                    className,
                    inFileNamePrefix,
                    inSimSettingsOpt,
                    args,
                )?;
            }
        }
    }
    resultValues = List::appendElt(
        (
            literal!("timeFrontend"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeFrontend }),
        ),
        resultValues,
    );
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), false)?;
    if !(stringEmpty(&flatString)) && runSilent {
        Error::addInternalError(
            literal!(
                "Flat model string generated but is not being dumped. Please make sure it is not generated if it is not shown."
            ),
            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
        )?;
    } else if stringEmpty(&flatString) && !(runSilent) {
        Error::addInternalError(
            literal!("Flat model string generated but is empty."),
            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
        )?;
    } else {
        metamodelica::print(flatString);
    }
    if Flags::isSet(Flags::EXEC_STAT.clone())? && System::realtimeNtick(ClockIndexes::RT_CLOCK_FMU_SIMCODE.clone())? > 0
    {
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("FMU-only work: backend "));
            __mm_s.push_str(&*fmuOverheadTime(ClockIndexes::RT_CLOCK_FMU_BACKEND.clone())?);
            __mm_s.push_str(&*literal!(", simCode "));
            __mm_s.push_str(&*fmuOverheadTime(ClockIndexes::RT_CLOCK_FMU_SIMCODE.clone())?);
            __mm_s.push_str(&*literal!(", templates "));
            __mm_s.push_str(&*fmuOverheadTime(ClockIndexes::RT_CLOCK_FMU_TEMPLATES.clone())?);
            ArcStr::from(__mm_s)
        })?;
    }
    success = true;
    Ok((success, cache, outLibs, outFileDir, resultValues))
}

fn fmuOverheadTime(mut clockIndex: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = System::snprintff(
        literal!("%.4g"),
        20,
        if (System::realtimeNtick(clockIndex)? > 0) {
            System::realtimeAccumulated(clockIndex)?
        } else {
            metamodelica::OrderedFloat(0.0_f64)
        },
    )?;
    Ok(r#str)
}

pub(crate) fn translateModelCallBackend(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut functions: &metamodelica::Ref<FunctionTreeImpl::Tree>,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileNamePrefix: ArcStr,
    mut useDAEMode: bool,
    mut simSettings: Option<SimCode::SimulationSettings>,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
)> {
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut func_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<NFFunction::Function::Function>>,
    >;
    let mut dae: DAE::DAElist;
    let mut dae_funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut env: FCore::Graph;
    let mut cache: FCore::Cache;
    let mut file_name_prefix: ArcStr;
    file_name_prefix = if (metamodelica::stringEq(&fileNamePrefix, &(literal!("<default>")))) {
        AbsynUtil::pathString(className.clone(), literal!("."), true, false)?
    } else {
        fileNamePrefix
    };
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
            FlagsUtil::setConfigBool(Flags::SIM_CODE_SCALARIZE.clone(), false)?;
        }
        func_map = UnorderedMap::fromLists(
            &(FunctionTreeImpl::listKeys(functions, metamodelica::nil())),
            FunctionTreeImpl::listValues(functions, metamodelica::nil()),
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Absyn::Path>,
                      __a1: metamodelica::Ref<Absyn::Path>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool>
                        + 'static,
                >),
        )?;
        (outLibs, outFileDir, resultValues, _) = translateModelCallBackendNB(
            flatModel,
            func_map,
            className,
            file_name_prefix,
            simSettings,
            &(crate::SimCodeMain::TranslateModelKind::NORMAL),
        )?;
    } else {
        dae = NFConvertDAE::convertModel(flatModel)?;
        dae_funcs = NFConvertDAE::convertFunctionTree(functions)?;
        env = FGraph::new(literal!("graph"), FCore::dummyTopModel.clone());
        cache = FCore::emptyCache();
        FCore::setCachedFunctionTree(&cache, dae_funcs);
        if useDAEMode {
            (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOBDAEMode(
                cache,
                env,
                dae,
                className,
                file_name_prefix,
                simSettings,
                &(Absyn::emptyFunctionArgs.clone()),
                &(crate::SimCodeMain::TranslateModelKind::NORMAL),
            )?;
        } else {
            (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOB(
                crate::SimCodeMain::TranslateModelKind::NORMAL,
                cache,
                env,
                dae,
                className,
                file_name_prefix,
                simSettings,
                &(Absyn::emptyFunctionArgs.clone()),
            )?;
        }
    }
    Ok((outLibs, outFileDir, resultValues))
}

fn simSettingsSimflags(mut inSimSettingsOpt: Option<SimCode::SimulationSettings>) -> Option<ArcStr> {
    let mut simflags: Option<ArcStr>;
    simflags = (match inSimSettingsOpt {
        Some(SimCode::SimulationSettings { simflags: mut s, .. }) => Some(s.clone()),
        _ => None,
    });
    simflags
}

fn translateModelCallBackendOB(
    mut kind: TranslateModelKind,
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inDae: DAE::DAElist,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut inSimSettingsOpt: Option<SimCode::SimulationSettings>,
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    (outLibs, outFileDir) = (match inEnv {
        mut graph => {
            let mut file_dir: ArcStr;
            let mut description: ArcStr;
            let mut fmuType: ArcStr;
            let mut libs: metamodelica::List<ArcStr>;
            let mut dae: DAE::DAElist;
            let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut initDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut initDAE_lambda0: Option<metamodelica::Ref<BackendDAE::BackendDAE>>;
            let mut inlineData: Option<BackendDAE::InlineData>;
            let mut removedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut strPreOptModules: Option<metamodelica::List<ArcStr>>;
            let mut isFMI2: bool;
            let mut fmiDer: metamodelica::List<(
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
            )>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(
                cache.clone(),
                graph.clone(),
                inDae.clone(),
                &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: DAE::DAElist| {
                    StateMachineFlatten::stateMachineToDataFlow(&__a0, &__a1, __a2)
                },
            )?;
            ExecStat::execStat(&(literal!("Transformations before backend")))?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), literal!("FrontEnd DAE after transformations"))?;
                serializeNotify(
                    (dae.clone(), inDae.clone()),
                    literal!("FrontEnd DAE before+after transformations"),
                )?;
                ExecStat::execStat(&(literal!("Serialize DAE (2)")))?;
            }
            GCExt::free(inDae);
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache)?;
            }
            description = DAEUtil::daeDescription(&dae);
            dlow = BackendDAECreate::lower(
                dae.clone(),
                cache.clone(),
                graph,
                BackendDAE::ExtraInfo {
                    description: description,
                    fileNamePrefix: inFileNamePrefix.clone(),
                    simflags: simSettingsSimflags(inSimSettingsOpt.clone()),
                },
            )?;
            GCExt::free(dae);
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), literal!("BackendDAECreate.lower"))?;
                ExecStat::execStat(&(literal!("Serialize dlow")))?;
            }
            (isFMI2, fmuType) = (match kind.clone() {
                TranslateModelKind::FMU {
                    kind: mut __esc_fmuType,
                    ..
                } => {
                    fmuType = __esc_fmuType.clone();
                    (
                        FMI::isFMIVersion20(&(FMI::getFMIVersionString()?))?
                            || FMI::isFMIVersion30(&(FMI::getFMIVersionString()?))?,
                        fmuType,
                    )
                }
                _ => (false, literal!("")),
            });
            strPreOptModules = if (isFMI2) {
                Some(metamodelica::cons(
                    literal!("introduceOutputAliases"),
                    BackendDAEUtil::getPreOptModulesString()?,
                ))
            } else {
                None
            };
            if isFMI2 && metamodelica::stringEq(&fmuType, &(literal!("cs"))) {
                strPreOptModules = Some(metamodelica::cons(
                    literal!("introduceOutputRealDerivatives"),
                    strPreOptModules.ok_or("pattern mismatch")?,
                ));
            }
            (dlow, initDAE, initDAE_lambda0, inlineData, removedInitialEquationLst) =
                BackendDAEUtil::getSolvedSystem(dlow, &inFileNamePrefix, strPreOptModules, None, None, None)?;
            if isFMI2 && !(Flags::isSet(Flags::FMI20_DEPENDENCIES.clone())?) {
                System::realtimeTick(ClockIndexes::RT_CLOCK_FMU_BACKEND.clone())?;
                (fmiDer, funcs) = SymbolicJacobian::createFMIModelDerivatives(&dlow)?;
                dlow = BackendDAEUtil::setFunctionTree(&dlow, funcs);
                System::realtimeAccumulate(ClockIndexes::RT_CLOCK_FMU_BACKEND.clone())?;
            } else {
                fmiDer = metamodelica::nil();
            }
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), literal!("BackendDAE (simulation)"))?;
                serializeNotify(initDAE.clone(), literal!("BackendDAE (initialization)"))?;
                serializeNotify(initDAE_lambda0.clone(), literal!("BackendDAE (lambda0)"))?;
                serializeNotify(
                    (
                        dlow.clone(),
                        initDAE.clone(),
                        initDAE_lambda0.clone(),
                        inlineData.clone(),
                        removedInitialEquationLst.clone(),
                    ),
                    literal!("BackendDAE (simulation+initialization+lambda0+inlineData+removedInitialEquationLst)"),
                )?;
                ExecStat::execStat(&(literal!("Serialize solved system")))?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = (match kind.clone() {
                TranslateModelKind::NORMAL => {
                    (libs, file_dir, timeSimCode, timeTemplates) = generateModelCode(
                        dlow,
                        &initDAE,
                        initDAE_lambda0,
                        inlineData,
                        removedInitialEquationLst,
                        SymbolTable::getAbsyn(),
                        className,
                        inFileNamePrefix,
                        inSimSettingsOpt,
                        args,
                        &fmiDer,
                    )?;
                    (libs, file_dir, timeSimCode, timeTemplates)
                }
                TranslateModelKind::FMU { .. } => {
                    (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeFMU(
                        dlow,
                        &initDAE,
                        initDAE_lambda0,
                        &fmiDer,
                        removedInitialEquationLst,
                        SymbolTable::getAbsyn(),
                        className,
                        FMI::getFMIVersionString()?,
                        var_field!(kind.kind, TranslateModelKind::FMU).clone(),
                        inFileNamePrefix,
                        var_field!(kind.targetName, TranslateModelKind::FMU).clone(),
                        inSimSettingsOpt,
                        var_field!(kind.translateOnly, TranslateModelKind::FMU).clone(),
                    )?;
                    (libs, file_dir, timeSimCode, timeTemplates)
                }
                TranslateModelKind::XML => {
                    (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeXML(
                        dlow,
                        &initDAE,
                        initDAE_lambda0,
                        removedInitialEquationLst,
                        SymbolTable::getAbsyn(),
                        className,
                        inFileNamePrefix,
                        inSimSettingsOpt,
                    )?;
                    (libs, file_dir, timeSimCode, timeTemplates)
                }
                _ => {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Unknown translateModel kind: "));
                            __mm_s.push_str(&*anyString(kind));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
                    )?;
                    return Err("fail");
                }
            });
            (libs, file_dir)
        }
    });
    if generateFunctions {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![
        (
            literal!("timeTemplates"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeTemplates })
        ),
        (
            literal!("timeSimCode"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeSimCode })
        ),
        (
            literal!("timeBackend"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeBackend })
        )
    ];
    Ok((cache, outLibs, outFileDir, resultValues))
}

pub(crate) fn translateModelCallBackendOBDAEMode(
    mut cache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inDae: DAE::DAElist,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut inSimSettingsOpt: Option<SimCode::SimulationSettings>,
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut kind: &TranslateModelKind,
) -> Result<(
    FCore::Cache,
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (outLibs, outFileDir) = 'mc: {
        let __mc_input = inEnv;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            let mut graph = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut file_dir: ArcStr;
            let mut description: ArcStr;
            let mut libs: metamodelica::List<ArcStr>;
            let mut dae: DAE::DAElist;
            let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut initDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut initDAE_lambda0_option: Option<metamodelica::Ref<BackendDAE::BackendDAE>>;
            let mut removedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut strPreOptModules: Option<metamodelica::List<ArcStr>>;
            let mut isFMI: bool;
            let mut cache: FCore::Cache = cache.clone();
            let mut generateFunctions: bool = generateFunctions.clone();
            let mut timeBackend: metamodelica::Real = timeBackend.clone();
            let mut timeSimCode: metamodelica::Real = timeSimCode.clone();
            let mut timeTemplates: metamodelica::Real = timeTemplates.clone();
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(
                cache.clone(),
                graph.clone(),
                inDae.clone(),
                &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: DAE::DAElist| {
                    StateMachineFlatten::stateMachineToDataFlow(&__a0, &__a1, __a2)
                },
            )?;
            ExecStat::execStat(&(literal!("Transformations before backend")))?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), literal!("dae2"))?;
                ExecStat::execStat(&(literal!("Serialize DAE (2)")))?;
            }
            GCExt::free(inDae.clone());
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache.clone())?;
            }
            description = DAEUtil::daeDescription(&dae);
            dlow = BackendDAECreate::lower(
                dae.clone(),
                cache.clone(),
                graph.clone(),
                BackendDAE::ExtraInfo {
                    description: description.clone(),
                    fileNamePrefix: inFileNamePrefix.clone(),
                    simflags: simSettingsSimflags(inSimSettingsOpt.clone()),
                },
            )?;
            GCExt::free(dae.clone());
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), literal!("dlow"))?;
                ExecStat::execStat(&(literal!("Serialize dlow")))?;
            }
            isFMI = (match kind.clone() {
                TranslateModelKind::FMU { .. } => {
                    FMI::isFMIVersion20(&(FMI::getFMIVersionString()?))?
                        || FMI::isFMIVersion30(&(FMI::getFMIVersionString()?))?
                }
                _ => false,
            });
            strPreOptModules = if (isFMI) {
                Some(metamodelica::cons(
                    literal!("introduceOutputAliases"),
                    BackendDAEUtil::getPreOptModulesString()?,
                ))
            } else {
                None
            };
            (dlow, initDAE, initDAE_lambda0_option, removedInitialEquationLst) = DAEMode::getEqSystemDAEmode(
                dlow.clone(),
                &inFileNamePrefix,
                strPreOptModules.clone(),
                None,
                None,
                None,
            )?;
            ExecStat::execStat(&(literal!("Backend")))?;
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), literal!("simDAE"))?;
                serializeNotify(initDAE.clone(), literal!("initDAE"))?;
                serializeNotify(removedInitialEquationLst.clone(), literal!("removedInitialEquationLst"))?;
                ExecStat::execStat(&(literal!("Serialize solved system")))?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeDAE(
                &dlow,
                &initDAE,
                initDAE_lambda0_option.clone(),
                removedInitialEquationLst.clone(),
                SymbolTable::getAbsyn(),
                className.clone(),
                inFileNamePrefix.clone(),
                inSimSettingsOpt.clone(),
                args,
                kind,
            )?;
            timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
            timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
            Ok((
                (libs.clone(), file_dir.clone()),
                cache.clone(),
                generateFunctions.clone(),
                timeBackend.clone(),
                timeSimCode.clone(),
                timeTemplates.clone(),
            ))
        })() {
            cache = __wb0;
            generateFunctions = __wb1;
            timeBackend = __wb2;
            timeSimCode = __wb3;
            timeTemplates = __wb4;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut resstr: ArcStr;
            resstr = AbsynUtil::pathStringNoQual(className.clone(), literal!("."), false, false)?;
            resstr = stringAppendList(list![
                literal!("SimCode DAEmode: The model "),
                resstr.clone(),
                literal!(" could not be translated")
            ]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![resstr.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    if generateFunctions {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![
        (
            literal!("timeTemplates"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeTemplates })
        ),
        (
            literal!("timeSimCode"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeSimCode })
        ),
        (
            literal!("timeBackend"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeBackend })
        )
    ];
    Ok((cache, outLibs, outFileDir, resultValues))
}

fn translateModelCallBackendNB(
    mut inFlatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<NFFunction::Function::Function>>,
    >,
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut inSimSettingsOpt: Option<SimCode::SimulationSettings>,
    mut kind: &TranslateModelKind,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut oldFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut bdae: metamodelica::Ref<NBackendDAE::NBackendDAE>;
    let mut nf_api: bool;
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    nf_api = FlagsUtil::set(Flags::NF_API.clone(), false)?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
    bdae = NBackendDAE::lower(inFlatModel, funcMap)?;
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        metamodelica::print(NBackendDAE::toString(&bdae, literal!("(After Lowering)"))?);
    }
    bdae = NBackendDAE::main(bdae)?;
    timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
    ExecStat::execStat(&(literal!("backend")))?;
    FlagsUtil::set(Flags::NF_API.clone(), nf_api)?;
    (outLibs, outFileDir, timeSimCode, timeTemplates, oldFunctionTree) =
        generateModelCodeNewBackend(&bdae, inClassName, inFileNamePrefix, inSimSettingsOpt, kind)?;
    resultValues = list![
        (
            literal!("timeTemplates"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeTemplates })
        ),
        (
            literal!("timeSimCode"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeSimCode })
        ),
        (
            literal!("timeBackend"),
            metamodelica::Ref::new(Values::Value::REAL { real: timeBackend })
        )
    ];
    Ok((outLibs, outFileDir, resultValues, oldFunctionTree))
}

fn generateModelCodeDAE(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inInitDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut initDAE_lambda0_option: Option<metamodelica::Ref<BackendDAE::BackendDAE>>,
    mut inRemovedInitialEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut p: Absyn::Program,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut filenamePrefix: ArcStr,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut kind: &TranslateModelKind,
) -> Result<(
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Real,
    metamodelica::Real,
)> {
    let mut libs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let debug: bool = false;
    let mut includes: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut includeDirs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut libPaths: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>> = metamodelica::nil();
    let mut simCode: metamodelica::Ref<SimCode::SimCode> =
        <metamodelica::Ref<SimCode::SimCode> as ::std::default::Default>::default();
    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration> = metamodelica::nil();
    let mut a_cref: metamodelica::Ref<Absyn::ComponentRef> = metamodelica::Ref::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (
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
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ) = (
        0,
        (
            Default::default(),
            (0, 0, Default::default()),
            0,
            (
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
                std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            ),
        ),
        metamodelica::nil(),
    );
    let mut lits: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut numCheckpoints: i32;
    let mut tempVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut emptyBDAE: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut initDAE_lambda0: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut modelInfo: SimCode::ModelInfo = <SimCode::ModelInfo as ::std::default::Default>::default();
    let mut extObjInfo: SimCode::ExtObjInfo = <SimCode::ExtObjInfo as ::std::default::Default>::default();
    let mut crefToSimVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    ) = (
        Default::default(),
        (0, 0, Default::default()),
        0,
        (
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
        ),
    );
    let mut makefileParams: SimCodeFunction::MakefileParams =
        <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
    let mut spatialInfo: SimCode::SpatialDistributionInfo =
        <SimCode::SpatialDistributionInfo as ::std::default::Default>::default();
    let mut delayedExps: metamodelica::List<(
        i32,
        (
            metamodelica::Ref<DAE::Exp>,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::Ref<DAE::Exp>,
        ),
    )> = metamodelica::nil();
    let mut maxDelayedExpIndex: i32 = 0;
    let mut uniqueEqIndex: i32 = 1;
    let mut tmpB: bool = false;
    let mut varToArrayIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
                )>,
            >,
        ),
        i32,
        (
            HashTableCrIListArray::FuncHashCref,
            HashTableCrIListArray::FuncCrefEqual,
            HashTableCrIListArray::FuncCrefStr,
            HashTableCrIListArray::FuncExpStr,
        ),
    ) = (
        Default::default(),
        (0, 0, Default::default()),
        0,
        (
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
        ),
    );
    let mut varToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    ) = (
        Default::default(),
        (0, 0, Default::default()),
        0,
        (
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
        ),
    );
    let mut crefToClockIndexHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    ) = (
        Default::default(),
        (0, 0, Default::default()),
        0,
        (
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_, _| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
            std::sync::Arc::new(|_| unreachable!("checkpoint placeholder")),
        ),
    );
    let mut discreteModelVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut timeEvents: metamodelica::List<BackendDAE::TimeEvent> = metamodelica::nil();
    let mut zeroCrossingsSet: BackendDAE::ZeroCrossingSet =
        <BackendDAE::ZeroCrossingSet as ::std::default::Default>::default();
    let mut sampleZCSet: BackendDAE::ZeroCrossingSet =
        <BackendDAE::ZeroCrossingSet as ::std::default::Default>::default();
    let mut de_relations: BackendDAE::ZeroCrossingSet =
        <BackendDAE::ZeroCrossingSet as ::std::default::Default>::default();
    let mut zeroCrossings: metamodelica::List<BackendDAE::ZeroCrossing> = metamodelica::nil();
    let mut sampleZC: metamodelica::List<BackendDAE::ZeroCrossing> = metamodelica::nil();
    let mut relations: metamodelica::List<BackendDAE::ZeroCrossing> = metamodelica::nil();
    let mut daeVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut resVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut algStateVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut auxVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut varsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut daeModeSP: Option<metamodelica::Ref<SimCode::JacobianMatrix>> = None;
    let mut daeModeData: Option<SimCode::DaeModeData> = None;
    let mut daeModeConf: SimCode::DaeModeConfig = SimCode::DaeModeConfig::ALL_EQUATIONS;
    let mut matrixnames: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut daeEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> =
        metamodelica::nil();
    let mut residualVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algebraicStateVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut auxiliaryVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut daeModeJacobian: (
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
    ) = (
        None,
        (
            metamodelica::nil(),
            metamodelica::nil(),
            (metamodelica::nil(), metamodelica::nil()),
            0,
        ),
        metamodelica::nil(),
        (
            metamodelica::nil(),
            metamodelica::nil(),
            (metamodelica::nil(), metamodelica::nil()),
            0,
        ),
    );
    let mut daeModeJac: Option<(
        metamodelica::Ref<BackendDAE::BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )> = None;
    let mut jacH: Option<metamodelica::Ref<BackendDAE::Jacobian>> = None;
    let mut daeModeSparsity: (
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
    ) = (
        metamodelica::nil(),
        metamodelica::nil(),
        (metamodelica::nil(), metamodelica::nil()),
        0,
    );
    let mut daeModeColoring: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    let mut nonlinearPattern: (
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
    ) = (
        metamodelica::nil(),
        metamodelica::nil(),
        (metamodelica::nil(), metamodelica::nil()),
        0,
    );
    let mut symDAESparsPattern: metamodelica::Ref<SimCode::JacobianMatrix> =
        <metamodelica::Ref<SimCode::JacobianMatrix> as ::std::default::Default>::default();
    let mut symJacs: metamodelica::List<metamodelica::Ref<SimCode::JacobianMatrix>> = metamodelica::nil();
    let mut SymbolicJacs: metamodelica::List<metamodelica::Ref<SimCode::JacobianMatrix>> = metamodelica::nil();
    let mut SymbolicJacsNLS: metamodelica::List<metamodelica::Ref<SimCode::JacobianMatrix>> = metamodelica::nil();
    let mut SymbolicJacsTemp: metamodelica::List<metamodelica::Ref<SimCode::JacobianMatrix>> = metamodelica::nil();
    let mut initialEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut initialEquations_lambda0: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut removedInitialEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut jacobianSimvars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut seedVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut startValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut maxValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut minValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut nominalValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut parameterEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut jacobianEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    let mut isFMU: bool = false;
    let mut translateOnly: bool = false;
    let mut FMUVersion: ArcStr = literal!("");
    let mut FMUType: ArcStr = literal!("");
    let mut fmuTargetName: ArcStr = literal!("");
    let mut fullPathPrefix: ArcStr = literal!("");
    let mut modelStructure: Option<SimCode::FmiModelStructure> = None;
    let mut fmiSimulationFlags: Option<SimCode::FmiSimulationFlags> = None;
    let () = (match kind.clone() {
        TranslateModelKind::FMU { .. } => {
            isFMU = true;
            FMUVersion = FMI::getFMIVersionString()?;
            FMUType = var_field!(kind.kind, TranslateModelKind::FMU).clone();
            fmuTargetName = var_field!(kind.targetName, TranslateModelKind::FMU).clone();
            translateOnly = var_field!(kind.translateOnly, TranslateModelKind::FMU).clone();
            fullPathPrefix = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Util::hashFileNamePrefix(&filenamePrefix)?);
                __mm_s.push_str(&*literal!(".fmutmp/sources/"));
                ArcStr::from(__mm_s)
            };
            ()
        }
        _ => (),
    });
    numCheckpoints = ErrorExt::getNumCheckpoints();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        StackOverflow::clearStacktraceMessages();
        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        a_cref = AbsynUtil::pathToCref(&className);
        fileDir = ProgramUtil::getFileDir(a_cref.clone(), p.clone());
        (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) =
            SimCodeUtilShared::createFunctions(&p, &inBackendDAE.shared.functionTree)?;
        extObjInfo = SimCodeUtil::createExtObjInfo(&inBackendDAE.shared)?;
        makefileParams = SimCodeFunctionUtil::createMakefileParams(
            includeDirs.clone(),
            libs.clone(),
            libPaths.clone(),
            false,
            false,
        )?;
        (delayedExps, maxDelayedExpIndex) = SimCodeUtil::extractDelayedExpressions(inBackendDAE)?;
        spatialInfo = SimCodeUtil::extractSpatialDistributionInfo(inBackendDAE)?;
        timeEvents = inBackendDAE.shared.eventInfo.timeEvents.clone();
        (zeroCrossings, relations, sampleZC) = (match inBackendDAE.shared.eventInfo.clone() {
            BackendDAE::EventInfo {
                zeroCrossings: mut __esc_zeroCrossingsSet,
                relations: mut __esc_de_relations,
                samples: mut __esc_sampleZCSet,
                ..
            } => {
                zeroCrossingsSet = __esc_zeroCrossingsSet.clone();
                de_relations = __esc_de_relations.clone();
                sampleZCSet = __esc_sampleZCSet.clone();
                (
                    ZeroCrossings::toList(&zeroCrossingsSet),
                    ZeroCrossings::toList(&de_relations),
                    ZeroCrossings::toList(&sampleZCSet),
                )
            }
        });
        (initialEquations, uniqueEqIndex, tempVars) =
            SimCodeUtil::createInitialEquations(inInitDAE, uniqueEqIndex, tempVars.clone())?;
        if (initDAE_lambda0_option).is_some() {
            let __pa1 = ::match_deref::match_deref! { match &(initDAE_lambda0_option.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            initDAE_lambda0 = metamodelica::Own::own(__pa1);
            (initialEquations_lambda0, uniqueEqIndex, tempVars) =
                SimCodeUtil::createInitialEquations_lambda0(&initDAE_lambda0, uniqueEqIndex, tempVars.clone())?;
        } else {
            initialEquations_lambda0 = metamodelica::nil();
        }
        let (__pa2, (__pa3, _), __pa4) = SimCodeUtil::createNonlinearResidualEquations(
            inRemovedInitialEquationLst.clone(),
            (uniqueEqIndex, 0),
            tempVars.clone(),
            &inBackendDAE.shared.functionTree,
        )?;
        removedInitialEquations = metamodelica::Own::own(__pa2);
        uniqueEqIndex = metamodelica::Own::own(__pa3);
        tempVars = metamodelica::Own::own(__pa4);
        ExecStat::execStat(&(literal!("simCode: created initialization part")))?;
        (uniqueEqIndex, startValueEquations, _) = BackendDAEUtil::foldEqSystem(
            inInitDAE,
            &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                   __a1: metamodelica::Ref<BackendDAE::Shared>,
                   __a2: (
                i32,
                metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
                BackendDAE::Variables,
            )| SimCodeUtil::createStartValueEquations(&__a0, &__a1, &__a2),
            (
                uniqueEqIndex,
                metamodelica::nil(),
                inBackendDAE.shared.globalKnownVars.clone(),
            ),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createStartValueEquations")))?;
        }
        (uniqueEqIndex, nominalValueEquations) = SimCodeUtil::createValueEquationsShared(
            &inBackendDAE.shared,
            (std::sync::Arc::new(fnptr!(
                SimCodeUtil::createInitialAssignmentsFromNominal,
                metamodelica::Ref<BackendDAE::Var>,
                (
                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                    BackendDAE::Variables
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            &((uniqueEqIndex, nominalValueEquations.clone())),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createNominalValueEquationsShared")))?;
        }
        (uniqueEqIndex, nominalValueEquations) = BackendDAEUtil::foldEqSystem(
            inBackendDAE,
            &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                   __a1: metamodelica::Ref<BackendDAE::Shared>,
                   __a2: (i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)| {
                SimCodeUtil::createNominalValueEquations(&__a0, &__a1, &__a2)
            },
            (uniqueEqIndex, nominalValueEquations.clone()),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createNominalValueEquations")))?;
        }
        (uniqueEqIndex, minValueEquations) = SimCodeUtil::createValueEquationsShared(
            &inBackendDAE.shared,
            (std::sync::Arc::new(fnptr!(
                SimCodeUtil::createInitialAssignmentsFromMin,
                metamodelica::Ref<BackendDAE::Var>,
                (
                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                    BackendDAE::Variables
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            &((uniqueEqIndex, minValueEquations.clone())),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createMinValueEquationsShared")))?;
        }
        (uniqueEqIndex, minValueEquations) = BackendDAEUtil::foldEqSystem(
            inBackendDAE,
            &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                   __a1: metamodelica::Ref<BackendDAE::Shared>,
                   __a2: (i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)| {
                SimCodeUtil::createMinValueEquations(&__a0, &__a1, &__a2)
            },
            (uniqueEqIndex, minValueEquations.clone()),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createMinValueEquations")))?;
        }
        (uniqueEqIndex, maxValueEquations) = SimCodeUtil::createValueEquationsShared(
            &inBackendDAE.shared,
            (std::sync::Arc::new(fnptr!(
                SimCodeUtil::createInitialAssignmentsFromMax,
                metamodelica::Ref<BackendDAE::Var>,
                (
                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                    BackendDAE::Variables
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            &((uniqueEqIndex, maxValueEquations.clone())),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createMaxValueEquationsShared")))?;
        }
        (uniqueEqIndex, maxValueEquations) = BackendDAEUtil::foldEqSystem(
            inBackendDAE,
            &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                   __a1: metamodelica::Ref<BackendDAE::Shared>,
                   __a2: (i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)| {
                SimCodeUtil::createMaxValueEquations(&__a0, &__a1, &__a2)
            },
            (uniqueEqIndex, maxValueEquations.clone()),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createMaxValueEquations")))?;
        }
        (uniqueEqIndex, parameterEquations, _) = SimCodeUtil::createParameterEquations(
            uniqueEqIndex,
            &parameterEquations,
            inBackendDAE.shared.globalKnownVars.clone(),
        )?;
        if debug {
            ExecStat::execStat(&(literal!("simCode: createParameterEquations")))?;
        }
        discreteModelVars = BackendDAEUtil::foldEqSystem(
            inBackendDAE,
            &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                   __a1: metamodelica::Ref<BackendDAE::Shared>,
                   __a2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| {
                SimCodeUtil::extractDiscreteModelVars(&__a0, &__a1, __a2)
            },
            metamodelica::nil(),
        )?;
        (daeEquations, uniqueEqIndex, tempVars) = SimCodeUtil::createEquationsfromBackendDAE(
            inBackendDAE,
            uniqueEqIndex,
            tempVars.clone(),
            true,
            true,
            false,
            false,
        )?;
        emptyBDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: metamodelica::cons(
                BackendDAEUtil::createEqSystem(
                    inBackendDAE
                        .shared
                        .daeModeData
                        .modelVars
                        .clone()
                        .ok_or("pattern mismatch")?,
                    BackendEquation::emptyEqns(),
                    metamodelica::nil(),
                    openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                    BackendEquation::emptyEqns(),
                ),
                metamodelica::nil(),
            ),
            shared: inBackendDAE.shared.clone(),
        });
        if metamodelica::stringEq(
            &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?),
            &(literal!("symbolic")),
        ) {
            (daeModeJac, daeModeSparsity, daeModeColoring, nonlinearPattern) =
                (inBackendDAE.shared.symjacs).get(BackendDAE::SymbolicJacobianAIndex.clone())?;
            if (inBackendDAE.shared.dataReconciliationData).is_some() {
                let BackendDAE::DATA_RECON {
                    symbolicJacobian: _,
                    setcVars: _,
                    datareconinputs: _,
                    setBVars: _,
                    symbolicJacobianH: __pa5,
                    ..
                } = inBackendDAE
                    .shared
                    .dataReconciliationData
                    .clone()
                    .ok_or("pattern mismatch")?;
                jacH = metamodelica::Own::own(__pa5);
                if (jacH).is_some() {
                    matrixnames = list![literal!("B"), literal!("C"), literal!("D"), literal!("ADJ")];
                } else {
                    matrixnames = list![
                        literal!("B"),
                        literal!("C"),
                        literal!("D"),
                        literal!("H"),
                        literal!("ADJ")
                    ];
                }
            } else {
                matrixnames = list![
                    literal!("B"),
                    literal!("C"),
                    literal!("D"),
                    literal!("F"),
                    literal!("H"),
                    literal!("ADJ")
                ];
            }
            (daeModeSP, uniqueEqIndex, tempVars) = SimCodeUtil::createSymbolicSimulationJacobian(
                &(metamodelica::Ref::new(BackendDAE::Jacobian::GENERIC_JACOBIAN {
                    jacobian: daeModeJac.clone(),
                    sparsePattern: daeModeSparsity.clone(),
                    coloring: daeModeColoring.clone(),
                    nonlinearPattern: nonlinearPattern.clone(),
                })),
                uniqueEqIndex,
                tempVars.clone(),
                false,
            )?;
            tmpB = FlagsUtil::set(Flags::NO_START_CALC.clone(), true)?;
            modelInfo = SimCodeUtil::createModelInfo(
                className.clone(),
                p.clone(),
                &emptyBDAE,
                inInitDAE,
                functions.clone(),
                metamodelica::nil(),
                0,
                spatialInfo.maxIndex.clone(),
                fileDir.clone(),
                0,
                &tempVars,
                None,
            )?;
            FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB)?;
            crefToSimVarHT = SimCodeCodegenUtil::createCrefToSimVarHT(&modelInfo)?;
            (symJacs, uniqueEqIndex) = SimCodeUtil::createSymbolicJacobianssSimCode(
                &(metamodelica::nil()),
                &crefToSimVarHT,
                uniqueEqIndex,
                &matrixnames,
                &(metamodelica::nil()),
            )?;
            symJacs = metamodelica::cons(daeModeSP.clone().ok_or("pattern mismatch")?, symJacs.clone()).reverse();
        } else {
            tmpB = FlagsUtil::set(Flags::NO_START_CALC.clone(), true)?;
            modelInfo = SimCodeUtil::createModelInfo(
                className.clone(),
                p.clone(),
                &emptyBDAE,
                inInitDAE,
                functions.clone(),
                metamodelica::nil(),
                0,
                spatialInfo.maxIndex.clone(),
                fileDir.clone(),
                0,
                &tempVars,
                None,
            )?;
            FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB)?;
            crefToSimVarHT = SimCodeCodegenUtil::createCrefToSimVarHT(&modelInfo)?;
            if (inBackendDAE.shared.dataReconciliationData).is_some() {
                let BackendDAE::DATA_RECON {
                    symbolicJacobian: _,
                    setcVars: _,
                    datareconinputs: _,
                    setBVars: _,
                    symbolicJacobianH: __pa6,
                    ..
                } = inBackendDAE
                    .shared
                    .dataReconciliationData
                    .clone()
                    .ok_or("pattern mismatch")?;
                jacH = metamodelica::Own::own(__pa6);
                if (jacH).is_some() {
                    matrixnames = list![
                        literal!("A"),
                        literal!("B"),
                        literal!("C"),
                        literal!("D"),
                        literal!("ADJ")
                    ];
                } else {
                    matrixnames = list![
                        literal!("A"),
                        literal!("B"),
                        literal!("C"),
                        literal!("D"),
                        literal!("H"),
                        literal!("ADJ")
                    ];
                }
            } else {
                matrixnames = list![
                    literal!("A"),
                    literal!("B"),
                    literal!("C"),
                    literal!("D"),
                    literal!("F"),
                    literal!("H"),
                    literal!("ADJ")
                ];
            }
            (symJacs, uniqueEqIndex) = SimCodeUtil::createSymbolicJacobianssSimCode(
                &(metamodelica::nil()),
                &crefToSimVarHT,
                uniqueEqIndex,
                &matrixnames,
                &(metamodelica::nil()),
            )?;
        }
        SymbolicJacsNLS = metamodelica::nil();
        (initialEquations, modelInfo, SymbolicJacsTemp) =
            SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations.clone(), modelInfo.clone(), true)?;
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (initialEquations_lambda0, modelInfo, SymbolicJacsTemp) =
            SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations_lambda0.clone(), modelInfo.clone(), true)?;
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (parameterEquations, modelInfo, SymbolicJacsTemp) =
            SimCodeUtil::addAlgebraicLoopsModelInfo(parameterEquations.clone(), modelInfo.clone(), true)?;
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (SymbolicJacs, modelInfo, SymbolicJacsTemp) =
            SimCodeUtil::addAlgebraicLoopsModelInfoSymJacs(symJacs.clone(), modelInfo.clone());
        jacobianEquations = SimCodeUtil::collectAllJacobianEquations(&SymbolicJacs);
        if debug {
            ExecStat::execStat(&(literal!("simCode: create Jacobian linear code")))?;
        }
        SymbolicJacs = listAppend(
            SymbolicJacsNLS.clone().reverse(),
            listAppend(SymbolicJacs.clone(), SymbolicJacsTemp.clone()),
        );
        jacobianSimvars = SimCodeUtil::collectAllJacobianVars(&SymbolicJacs);
        modelInfo = SimCodeUtil::setJacobianVars(jacobianSimvars.clone(), modelInfo.clone());
        crefToSimVarHT = List::fold(
            &jacobianSimvars,
            &HashTableCrefSimVar::addSimVarToHashTable,
            crefToSimVarHT.clone(),
        )?;
        seedVars = SimCodeUtil::collectAllSeedVars(&SymbolicJacs)?;
        modelInfo = SimCodeUtil::setSeedVars(seedVars.clone(), modelInfo.clone());
        crefToSimVarHT = List::fold(
            &seedVars,
            &HashTableCrefSimVar::addSimVarToHashTable,
            crefToSimVarHT.clone(),
        )?;
        varsLst = BackendVariable::equationSystemsVarsLst(&inBackendDAE.eqs)?;
        daeVars = BackendVariable::listVar(varsLst.clone())?;
        (_, resVars) = BackendVariable::traverseBackendDAEVars(
            daeVars.clone(),
            (std::sync::Arc::new(BackendVariable::collectVarKindVarinVariables)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            (
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isDAEmodeResVar(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
            ),
        )?;
        (residualVars, _) = BackendVariable::traverseBackendDAEVars(
            resVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>,
                      __a1: (
                    metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                    BackendDAE::Variables,
                )| SimCodeUtil::traversingdlowvarToSimvar(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            (
                metamodelica::nil(),
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
            ),
        )?;
        (residualVars, _) = SimCodeUtil::rewriteIndex(&residualVars, 0);
        (residualVars, _, _) = SimCodeCodegenUtil::setVariableIndexHelper(&residualVars, 0, 0)?;
        crefToSimVarHT = List::fold(
            &residualVars,
            &HashTableCrefSimVar::addSimVarToHashTable,
            crefToSimVarHT.clone(),
        )?;
        (_, auxVars) = BackendVariable::traverseBackendDAEVars(
            daeVars.clone(),
            (std::sync::Arc::new(BackendVariable::collectVarKindVarinVariables)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            (
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::isDAEmodeAuxVar(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static,
                    >),
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
            ),
        )?;
        (auxiliaryVars, _) = BackendVariable::traverseBackendDAEVars(
            auxVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>,
                      __a1: (
                    metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                    BackendDAE::Variables,
                )| SimCodeUtil::traversingdlowvarToSimvar(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            (
                metamodelica::nil(),
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
            ),
        )?;
        auxiliaryVars = List::sort(
            auxiliaryVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: metamodelica::Ref<SimCodeVar::SimVar>| {
                    SimCodeUtil::simVarCompareByCrefSubsAtEndlLexical(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<SimCodeVar::SimVar>,
                            metamodelica::Ref<SimCodeVar::SimVar>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        (auxiliaryVars, _) = SimCodeUtil::rewriteIndex(&auxiliaryVars, 0);
        (auxiliaryVars, _, _) = SimCodeCodegenUtil::setVariableIndexHelper(&auxiliaryVars, 0, 0)?;
        crefToSimVarHT = List::fold(
            &auxiliaryVars,
            &HashTableCrefSimVar::addSimVarToHashTable,
            crefToSimVarHT.clone(),
        )?;
        algStateVars = BackendVariable::listVar(inBackendDAE.shared.daeModeData.algStateVars.clone())?;
        (algebraicStateVars, _) = BackendVariable::traverseBackendDAEVars(
            algStateVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>,
                      __a1: (
                    metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                    BackendDAE::Variables,
                )| SimCodeUtil::traversingdlowvarToSimvar(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
                                BackendDAE::Variables,
                            ),
                        )> + 'static,
                >),
            (
                metamodelica::nil(),
                BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()),
            ),
        )?;
        algebraicStateVars = SimCodeUtil::sortSimVarsAndWriteIndex(algebraicStateVars.clone(), &crefToSimVarHT)?;
        if isFMU {
            modelInfo = SimCodeUtil::exportDaeAlgebraicStates(modelInfo.clone(), &algebraicStateVars)?;
        }
        daeModeJacobian = (inBackendDAE.shared.symjacs).get(BackendDAE::SymbolicJacobianAIndex.clone())?;
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(SimCodeUtil::createSymbolicJacobianssSimCode(&(list![daeModeJacobian.clone()]), &crefToSimVarHT, uniqueEqIndex, &(list![literal!("daeMode")]), &(metamodelica::nil()))?) {
            (Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil }, __pa8) => (__pa7.clone(), __pa8.clone()),
            _ => return Err("pattern mismatch"),
        } };
        symDAESparsPattern = metamodelica::Own::own(__pa7);
        uniqueEqIndex = metamodelica::Own::own(__pa8);
        daeModeSP = Some(symDAESparsPattern.clone());
        if metamodelica::stringEq(
            &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?),
            &(literal!("symbolic")),
        ) {
            SymbolicJacs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimCode::JacobianMatrix>> = metamodelica::nil();
                for mut symjac in (SymbolicJacs.clone()).into_iter().cloned() {
                    let __x = SimCodeUtil::syncDAEandSimJac(symjac.clone(), symDAESparsPattern.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
        daeModeConf = openmodelica_simcode_types::SimCode::DaeModeConfig::ALL_EQUATIONS;
        daeModeData = Some(SimCode::DaeModeData {
            daeEquations: daeEquations.clone(),
            sparsityPattern: daeModeSP.clone(),
            residualVars: residualVars.clone(),
            algebraicVars: algebraicStateVars.clone(),
            auxiliaryVars: auxiliaryVars.clone(),
            modeCreated: daeModeConf,
        });
        modelInfo = SimCodeUtil::addNumEqns(modelInfo.clone(), uniqueEqIndex - ((jacobianEquations).len() as i32));
        if isFMU {
            System::realtimeTick(ClockIndexes::RT_CLOCK_FMU_SIMCODE.clone())?;
            if FMI::isFMIVersion20(&FMUVersion)? || FMI::isFMIVersion30(&FMUVersion)? {
                (_, modelStructure, modelInfo, _, uniqueEqIndex, _) = SimCodeUtil::createFMIModelStructure(
                    &(metamodelica::nil()),
                    modelInfo.clone(),
                    uniqueEqIndex,
                    inInitDAE,
                    inBackendDAE,
                )?;
            }
            fmiSimulationFlags = SimCodeUtil::createFMISimulationFlags(true)?;
            System::realtimeAccumulate(ClockIndexes::RT_CLOCK_FMU_SIMCODE.clone())?;
        }
        if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
            (varToArrayIndexMapping, varToIndexMapping) = SimCodeUtilShared::createVarToArrayIndexMapping(&modelInfo)?;
            (crefToClockIndexHT, _) = List::fold(
                &(inBackendDAE.eqs.clone().reverse()),
                &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
                       __a1: (
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                        ),
                    ),
                    i32,
                )| SimCodeUtil::collectClockedVars(&__a0, __a1),
                (HashTable::emptyHashTable(), 1),
            )?;
        } else {
            varToArrayIndexMapping = HashTableCrIListArray::emptyHashTable();
            varToIndexMapping = HashTableCrILst::emptyHashTable();
            crefToClockIndexHT = HashTable::emptyHashTable();
        }
        simCode = metamodelica::Ref::new(SimCode::SimCode {
            modelInfo: modelInfo.clone(),
            literals: metamodelica::nil(),
            recordDecls: recordDecls.clone(),
            externalFunctionIncludes: includes.clone(),
            generic_loop_calls: metamodelica::nil(),
            localKnownVars: metamodelica::nil(),
            allEquations: metamodelica::nil(),
            odeEquations: metamodelica::nil(),
            algebraicEquations: metamodelica::nil(),
            clockedPartitions: metamodelica::nil(),
            initialEquations: initialEquations.clone(),
            initialEquations_lambda0: initialEquations_lambda0.clone(),
            removedInitialEquations: removedInitialEquations.clone(),
            startValueEquations: startValueEquations.clone(),
            nominalValueEquations: nominalValueEquations.clone(),
            minValueEquations: minValueEquations.clone(),
            maxValueEquations: maxValueEquations.clone(),
            parameterEquations: parameterEquations.clone(),
            removedEquations: metamodelica::nil(),
            algorithmAndEquationAsserts: metamodelica::nil(),
            equationsForZeroCrossings: metamodelica::nil(),
            jacobianEquations: jacobianEquations.clone(),
            stateSets: metamodelica::nil(),
            constraints: metamodelica::nil(),
            classAttributes: metamodelica::nil(),
            zeroCrossings: FindZeroCrossings::setOperatorZeroCrossingIndices(
                &(ZeroCrossings::updateIndices(&zeroCrossings)),
            )?,
            relations: ZeroCrossings::updateIndices(&relations),
            timeEvents: timeEvents.clone(),
            discreteModelVars: discreteModelVars.clone(),
            extObjInfo: extObjInfo.clone(),
            makefileParams: makefileParams.clone(),
            delayedExps: SimCode::DelayedExpression {
                delayedExps: delayedExps.clone(),
                maxDelayedIndex: maxDelayedExpIndex,
            },
            spatialInfo: spatialInfo.clone(),
            jacobianMatrices: SymbolicJacs.clone(),
            simulationSettingsOpt: simSettingsOpt.clone(),
            fileNamePrefix: filenamePrefix.clone(),
            fullPathPrefix: fullPathPrefix.clone(),
            fmuTargetName: fmuTargetName.clone(),
            hpcomData: HpcOmSimCode::emptyHpcomData().clone(),
            valueReferences: if (isFMU) {
                SimCodeUtil::getValueReferenceMapping(&modelInfo)?
            } else {
                openmodelica_simcode_types::AvlTreeCRToInt::Tree::interned_EMPTY()
            },
            varToArrayIndexMapping: varToArrayIndexMapping.clone(),
            varToIndexMapping: varToIndexMapping.clone(),
            crefToSimVarHT: crefToSimVarHT.clone(),
            crefToClockIndexHT: crefToClockIndexHT.clone(),
            backendMapping: None,
            modelStructure: modelStructure.clone(),
            fmiSimulationFlags: fmiSimulationFlags.clone(),
            partitionData: SimCode::emptyPartitionData.clone(),
            daeModeData: daeModeData.clone(),
            inlineEquations: metamodelica::nil(),
            omsiData: None,
            scalarized: true,
            fmiFigures: metamodelica::nil(),
        });
        (simCode, lits) = SimCodeUtil::findSimCodeLiterals(simCode.clone(), literals.clone())?;
        assign_field!(simCode.literals = lits.clone());
        timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
        ExecStat::execStat(&(literal!("SimCode")))?;
        if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
            serializeNotify(simCode.clone(), literal!("SimCode"))?;
            ExecStat::execStat(&(literal!("Serialize simCode")))?;
        }
        if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
            SimCodeUtil::dumpSimCodeDebug(&simCode)?;
        }
        System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        if isFMU {
            callTargetTemplatesFMU(
                simCode.clone(),
                Config::simCodeTarget()?,
                FMUVersion.clone(),
                FMUType.clone(),
                p.clone(),
                translateOnly,
            )?;
        } else {
            callTargetTemplates(simCode.clone(), &(Config::simCodeTarget()?))?;
        }
        timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
        ExecStat::execStat(&(literal!("Templates")))?;
        return Ok(true);
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok((
                    libs.clone(),
                    fileDir.clone(),
                    timeSimCode.clone(),
                    timeTemplates.clone(),
                ));
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            {
                let __v = None;
                openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Stack overflow in "));
                    __mm_s.push_str(&*literal!("SimCodeMain.generateModelCodeDAE"));
                    __mm_s.push_str(&*literal!("...\n"));
                    __mm_s.push_str(&*stringDelimitList(
                        StackOverflow::readableStacktraceMessages()?,
                        literal!("\n"),
                    ));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
            )?;
            StackOverflow::clearStacktraceMessages();
        }
    }
    return Err("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn serializeNotify<T: Clone + 'static + metamodelica::gc::MMTrace>(mut data: T, mut name: ArcStr) -> Result<()> {
    let mut sz: metamodelica::Real;
    let mut raw_sz: metamodelica::Real;
    let mut nonSharedStringSize: metamodelica::Real;
    (sz, raw_sz, nonSharedStringSize) = System::getSizeOfData(data);
    Error::addMessage(
        Error::SERIALIZED_SIZE.clone(),
        list![
            name,
            StringUtil::bytesToReadableUnit(sz, 4, metamodelica::OrderedFloat((500) as f64))?,
            StringUtil::bytesToReadableUnit(raw_sz, 4, metamodelica::OrderedFloat((500) as f64))?,
            StringUtil::bytesToReadableUnit(nonSharedStringSize, 4, metamodelica::OrderedFloat((500) as f64))?
        ],
    )?;
    Ok(())
}

fn copyFiles(mut files: &metamodelica::List<ArcStr>, mut source: &ArcStr, mut destination: &ArcStr) -> Result<()> {
    let mut f2: ArcStr;
    let mut d2: ArcStr;
    for mut f in &**files {
        f2 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*destination);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*f);
            ArcStr::from(__mm_s)
        };
        d2 = System::dirname(f2.clone());
        if !(System::directoryExists(d2.clone())) {
            Error::assertion(
                Util::createDirectoryTree(d2.clone()),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to create directory "));
                    __mm_s.push_str(&*d2);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")),
            )?;
        }
        Error::assertion(
            System::copyFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*source);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*f);
                    ArcStr::from(__mm_s)
                },
                f2,
            ),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to copy file "));
                __mm_s.push_str(&*f);
                __mm_s.push_str(&*literal!(" from "));
                __mm_s.push_str(&*source);
                __mm_s.push_str(&*literal!(" to "));
                __mm_s.push_str(&*destination);
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")),
        )?;
    }
    Ok(())
}

pub(crate) fn copyFmuRustSources(mut fmutmp: &ArcStr) -> Result<()> {
    let mut rust_sources_dir: ArcStr;
    let mut dest: ArcStr;
    let mut manifest: ArcStr;
    let mut vendor: ArcStr;
    let mut cache: ArcStr;
    let mut lock: ArcStr;
    rust_sources_dir = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*arcstr::literal!(RuntimeSources::fmu_rust_sources_dir));
        ArcStr::from(__mm_s)
    };
    if !(System::directoryExists(rust_sources_dir.clone())) {
        Error::addCompilerWarning({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "--fmiSources asked for the sources of a --simCodeTarget=C FMU, but "
            ));
            __mm_s.push_str(&*rust_sources_dir);
            __mm_s.push_str(&*literal!(
                " does not exist. The FMU carries its C sources only and cannot be rebuilt."
            ));
            ArcStr::from(__mm_s)
        })?;
        return Ok(());
    }
    dest = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fmutmp);
        __mm_s.push_str(&*literal!("/sources/rust"));
        ArcStr::from(__mm_s)
    };
    Error::assertion(
        Util::createDirectoryTree(dest.clone()),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to create directory "));
            __mm_s.push_str(&*dest);
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")),
    )?;
    if !(System::copyPath(rust_sources_dir, dest.clone())) {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to copy the Rust runtime sources into "));
                __mm_s.push_str(&*dest);
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
        )?;
        return Ok(());
    }
    manifest = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dest);
        __mm_s.push_str(&*literal!("/"));
        __mm_s.push_str(&*arcstr::literal!(RuntimeSources::fmu_rust_manifest));
        ArcStr::from(__mm_s)
    };
    writeFmuRustWorkspace(manifest.clone())?;
    if Flags::getConfigEnum(Flags::FMI_SOURCES.clone())? != Flags::FMI_SOURCES_FULL.clone() {
        return Ok(());
    }
    lock = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*System::dirname(manifest.clone()));
        __mm_s.push_str(&*literal!("/Cargo.lock"));
        ArcStr::from(__mm_s)
    };
    cache = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?));
        __mm_s.push_str(&*literal!("/.openmodelica/fmu-rust-vendor/"));
        __mm_s.push_str(&*intString(stringHashDjb2(
            &(if (System::regularFileExists(lock.clone())) {
                System::readFile(lock)?
            } else {
                literal!("")
            }),
        )));
        ArcStr::from(__mm_s)
    };
    if !(System::directoryExists(cache.clone())) {
        Error::assertion(
            Util::createDirectoryTree(cache.clone()),
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to create directory "));
                __mm_s.push_str(&*cache);
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")),
        )?;
        if 0 != System::systemCall(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("cargo vendor --versioned-dirs --manifest-path \""));
                __mm_s.push_str(&*manifest);
                __mm_s.push_str(&*literal!("\" \""));
                __mm_s.push_str(&*cache);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            },
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*fmutmp);
                __mm_s.push_str(&*literal!("/resources/cargo-vendor.log"));
                ArcStr::from(__mm_s)
            },
        ) {
            System::removeDirectory(cache);
            Error::addCompilerError({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "--fmiSources=full needs `cargo vendor` to collect the Rust dependencies, and it failed. "
                ));
                __mm_s.push_str(&*literal!("See "));
                __mm_s.push_str(&*fmutmp);
                __mm_s.push_str(&*literal!(
                    "/resources/cargo-vendor.log. Use --fmiSources=slim for an FMU whose rebuild fetches them instead."
                ));
                ArcStr::from(__mm_s)
            })?;
            return Ok(());
        }
    }
    vendor = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dest);
        __mm_s.push_str(&*literal!("/vendor"));
        ArcStr::from(__mm_s)
    };
    if !(System::copyPath(cache, vendor.clone())) {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to copy the vendored Rust dependencies into "));
                __mm_s.push_str(&*vendor);
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("SimCode/SimCodeMain.mo"),
        )?;
        return Ok(());
    }
    Error::assertion(
        Util::createDirectoryTree({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*dest);
            __mm_s.push_str(&*literal!("/.cargo"));
            ArcStr::from(__mm_s)
        }),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Failed to create directory "));
            __mm_s.push_str(&*dest);
            __mm_s.push_str(&*literal!("/.cargo"));
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("SimCode/SimCodeMain.mo")),
    )?;
    System::writeFile(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*dest);
            __mm_s.push_str(&*literal!("/.cargo/config.toml"));
            ArcStr::from(__mm_s)
        },
        literal!(
            "[source.crates-io]\nreplace-with = \"vendored-sources\"\n\n[source.vendored-sources]\ndirectory = \"vendor\"\n"
        ),
    )?;
    Ok(())
}

fn fmuRustCrateRoots(mut sources: &ArcStr, mut dir: &ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut roots: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut d in &*List::sort(
        System::subDirectories({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*sources);
            __mm_s.push_str(&*dir);
            ArcStr::from(__mm_s)
        }),
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?
    .reverse()
    {
        if !(listMember(
            d.clone(),
            list![
                literal!("vendor"),
                literal!("target"),
                literal!("src"),
                literal!(".cargo")
            ],
        )) {
            roots = listAppend(
                fmuRustCrateRoots(
                    sources,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*dir);
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*d);
                        ArcStr::from(__mm_s)
                    }),
                )?,
                roots,
            );
        }
    }
    if System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*sources);
        __mm_s.push_str(&*dir);
        __mm_s.push_str(&*literal!("/Cargo.toml"));
        ArcStr::from(__mm_s)
    }) && System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*sources);
        __mm_s.push_str(&*dir);
        __mm_s.push_str(&*literal!("/src/lib.rs"));
        ArcStr::from(__mm_s)
    }) {
        roots = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*literal!("/src/lib.rs"));
                ArcStr::from(__mm_s)
            },
            roots,
        );
    }
    Ok(roots)
}

fn writeFmuRustWorkspace(mut manifest: ArcStr) -> Result<()> {
    let mut dir: ArcStr;
    let mut excludes: ArcStr;
    let mut subdirs: metamodelica::List<ArcStr>;
    dir = System::dirname(manifest.clone());
    subdirs = List::sort(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut d in (System::subDirectories(dir.clone())).into_iter().cloned() {
                if !(System::regularFileExists({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*dir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*d);
                    __mm_s.push_str(&*literal!("/Cargo.toml"));
                    ArcStr::from(__mm_s)
                })) {
                    continue;
                }
                let __x = d.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?;
    excludes = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut d in (subdirs).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  \""));
                    __mm_s.push_str(&*d);
                    __mm_s.push_str(&*literal!("\","));
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        literal!("\n"),
    );
    System::writeFile(manifest, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "# Generated by OpenModelica: the workspace a source-code FMU builds its Rust\n"
        ));
        __mm_s.push_str(&*literal!("# simulation runtime from.\n"));
        __mm_s.push_str(&*literal!("[package]\n"));
        __mm_s.push_str(&*literal!("name = \"openmodelica_fmu_runtime\"\n"));
        __mm_s.push_str(&*literal!("version = \"0.1.0\"\n"));
        __mm_s.push_str(&*literal!("edition = \"2024\"\n"));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("[dependencies]\n"));
        __mm_s.push_str(&*literal!(
            "# default-features = false drops `standalone`, which an FMU has no use for:\n"
        ));
        __mm_s.push_str(&*literal!(
            "# the executable entry points, the result file, --variableFilter and -iif.\n"
        ));
        __mm_s.push_str(&*literal!("openmodelica_simulation_runtime = { path = \"openmodelica_simulation_runtime\", default-features = false, features = [\"fmu-lapack\", \"fmu-runtime\"] }\n"));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!(
            "# Reached as plain path dependencies; a member would have to belong to this\n"
        ));
        __mm_s.push_str(&*literal!("# workspace, and each of them names another root.\n"));
        __mm_s.push_str(&*literal!("[workspace]\n"));
        __mm_s.push_str(&*literal!("exclude = [\n"));
        __mm_s.push_str(&*excludes);
        __mm_s.push_str(&*literal!("\n]\n"));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn objectFilesOf(mut sourceFiles: &metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut objectFiles: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut ext: ArcStr;
    for mut f in &**sourceFiles {
        ext = literal!("");
        for mut part in &*System::strtok(f.clone(), literal!(".")) {
            ext = part.clone();
        }
        if metamodelica::stringEq(&ext, &(literal!("c"))) {
            objectFiles = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*substring(f.clone(), 1, ((f).len() as i32) - 1)?);
                    __mm_s.push_str(&*literal!("o"));
                    ArcStr::from(__mm_s)
                },
                objectFiles,
            );
        } else if metamodelica::stringEq(&ext, &(literal!("cpp")))
            || metamodelica::stringEq(&ext, &(literal!("cc")))
            || metamodelica::stringEq(&ext, &(literal!("cxx")))
            || metamodelica::stringEq(&ext, &(literal!("c++")))
        {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Leaving "));
                __mm_s.push_str(&*f);
                __mm_s.push_str(&*literal!(" out of the object files of the FMU makefile: "));
                __mm_s.push_str(&*literal!("the makefile has no rule for '."));
                __mm_s.push_str(&*ext);
                __mm_s.push_str(&*literal!("'."));
                ArcStr::from(__mm_s)
            })?;
        }
    }
    objectFiles = objectFiles.reverse();
    Ok(objectFiles)
}
