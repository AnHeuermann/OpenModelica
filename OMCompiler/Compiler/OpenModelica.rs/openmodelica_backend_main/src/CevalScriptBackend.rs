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

use crate::CevalScript;
use crate::Interactive;
use crate::Interactive::Access;
use crate::InteractiveUtil;
use crate::NFApi;
use crate::Refactor;
use crate::SimCodeMain;
use crate::StaticScript;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_backend::BackendDAECreate;
use openmodelica_backend::BackendDAEOptimize;
use openmodelica_backend::BackendDAEUtil;
use openmodelica_backend::BackendDump;
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVariable;
use openmodelica_backend::FindZeroCrossings;
use openmodelica_backend::RewriteRules;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend::SymbolTable;
use openmodelica_backend::SymbolicJacobian;
use openmodelica_backend::XMLDump;
use openmodelica_backend_tools::AbsynToJulia;
use openmodelica_backend_tools::Binding;
use openmodelica_backend_tools::Conversion;
use openmodelica_backend_tools::DAEQuery;
use openmodelica_backend_tools::LexerModelicaDiff;
use openmodelica_backend_tools::Obfuscate;
use openmodelica_backend_tools::ReverseLookup;
use openmodelica_backend_tools::SimpleModelicaParser;
use openmodelica_backend_tools::TotalModelDebug;
use openmodelica_backend_tools::Uncertainties;
use openmodelica_backend_types::BackendDAE;
#[cfg(feature = "codegen_fmu_c")]
use openmodelica_codegen_fmu_c::CodegenFMU;
use openmodelica_codegen_wasm_jit::CodegenWasmJit;
use openmodelica_dump_extra::BlockCallRewrite;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::AbsynJLDumpTpl;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::FInst;
use openmodelica_frontend::Figaro;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::NFSCodeEnv;
use openmodelica_frontend::NFSCodeFlatten;
use openmodelica_frontend::NFSCodeLookup;
use openmodelica_frontend::StateMachineFlatten;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend::UnitAbsynBuilder;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_loader::Parser;
use openmodelica_nf_frontend::NFClassDiagram;
use openmodelica_nf_frontend::NFConvertDAE;
use openmodelica_nf_frontend::NFDefUseChains;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatModel;
use openmodelica_nf_frontend::NFFlatten;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFInst;
use openmodelica_nf_frontend::NFUsedElements;
use openmodelica_omgraphics::OMGraphics;
use openmodelica_program_util::ProgramUtil;
use openmodelica_script_util::PackageManagement;
use openmodelica_script_util::SimulationResults;
use openmodelica_script_util::UnitParserExt;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_tpl::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::ContainerImage;
use openmodelica_util::Debug;
use openmodelica_util::DiffAlgorithm;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::FMI;
use openmodelica_util::FMIExt;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Graph;
use openmodelica_util::Print;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::TaskGraphResults;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
thread_local! { static __simulationResultType_rtest_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationResult") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("resultFile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("simulationOptions"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("messages"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub(crate) fn simulationResultType_rtest() -> metamodelica::Ref<DAE::Type> {
    __simulationResultType_rtest_TLS.with(|__t| __t.clone())
}

thread_local! { static __simulationResultType_full_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationResult") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("resultFile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("simulationOptions"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("messages"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeFrontend"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeBackend"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeSimCode"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeTemplates"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeCompile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeSimulation"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("timeTotal"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub(crate) fn simulationResultType_full() -> metamodelica::Ref<DAE::Type> {
    __simulationResultType_full_TLS.with(|__t| __t.clone())
}

thread_local! { static __simulationResultType_drModelica_TLS: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationResult") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("messages"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("flatteningTime"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("simulationTime"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub(crate) fn simulationResultType_drModelica() -> metamodelica::Ref<DAE::Type> {
    __simulationResultType_drModelica_TLS.with(|__t| __t.clone())
}

//these are in reversed order than above
pub(crate) static zeroAdditionalSimulationResultValues: std::sync::LazyLock<
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
> = std::sync::LazyLock::new(|| {
    list![
        (
            literal!("timeTotal"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeSimulation"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeCompile"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeTemplates"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeSimCode"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeBackend"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeFrontend"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        )
    ]
});

// The build-phase times only (reversed order), for paths that skip translate/
// build (resimulateExecutable) but still run the model: createSimulationResult-
// FromcallModelExecutable adds the real timeTotal/timeSimulation, and these keep
// the result a complete SimulationResult record (0.0 for the phases not run)
// instead of a truncated one missing fields.
pub(crate) static zeroBuildPhaseResultValues: std::sync::LazyLock<
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
> = std::sync::LazyLock::new(|| {
    list![
        (
            literal!("timeCompile"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeTemplates"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeSimCode"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeBackend"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        ),
        (
            literal!("timeFrontend"),
            metamodelica::Ref::new(Values::Value::REAL {
                real: metamodelica::OrderedFloat(0.0_f64)
            })
        )
    ]
});

thread_local! { static __defaultStartTime_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }); }
pub(crate) fn defaultStartTime() -> metamodelica::Ref<DAE::Exp> {
    __defaultStartTime_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultStopTime_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }); }
pub(crate) fn defaultStopTime() -> metamodelica::Ref<DAE::Exp> {
    __defaultStopTime_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultNumberOfIntervals_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 500 }); }
pub(crate) fn defaultNumberOfIntervals() -> metamodelica::Ref<DAE::Exp> {
    __defaultNumberOfIntervals_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultStepSize_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.002_f64) }); }
pub(crate) fn defaultStepSize() -> metamodelica::Ref<DAE::Exp> {
    __defaultStepSize_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultTolerance_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-6_f64) }); }
pub(crate) fn defaultTolerance() -> metamodelica::Ref<DAE::Exp> {
    __defaultTolerance_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultMethod_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("dassl") }); }
pub(crate) fn defaultMethod() -> metamodelica::Ref<DAE::Exp> {
    __defaultMethod_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultFileNamePrefix_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }); }
pub(crate) fn defaultFileNamePrefix() -> metamodelica::Ref<DAE::Exp> {
    __defaultFileNamePrefix_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultOptions_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }); }
pub(crate) fn defaultOptions() -> metamodelica::Ref<DAE::Exp> {
    __defaultOptions_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultOutputFormat_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("mat") }); }
pub(crate) fn defaultOutputFormat() -> metamodelica::Ref<DAE::Exp> {
    __defaultOutputFormat_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultVariableFilter_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!(".*") }); }
pub(crate) fn defaultVariableFilter() -> metamodelica::Ref<DAE::Exp> {
    __defaultVariableFilter_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultCflags_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }); }
pub(crate) fn defaultCflags() -> metamodelica::Ref<DAE::Exp> {
    __defaultCflags_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultSimflags_TLS: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }); }
pub(crate) fn defaultSimflags() -> metamodelica::Ref<DAE::Exp> {
    __defaultSimflags_TLS.with(|__t| __t.clone())
}

thread_local! { static __defaultSimulationOptions_TLS: InteractiveTypes::SimulationOptions = InteractiveTypes::SimulationOptions { startTime: defaultStartTime().clone(), stopTime: defaultStopTime().clone(), numberOfIntervals: defaultNumberOfIntervals().clone(), stepSize: defaultStepSize().clone(), tolerance: defaultTolerance().clone(), method: defaultMethod().clone(), fileNamePrefix: defaultFileNamePrefix().clone(), options: defaultOptions().clone(), outputFormat: defaultOutputFormat().clone(), variableFilter: defaultVariableFilter().clone(), cflags: defaultCflags().clone(), simflags: defaultSimflags().clone() }; }
pub(crate) fn defaultSimulationOptions() -> InteractiveTypes::SimulationOptions {
    __defaultSimulationOptions_TLS.with(|__t| __t.clone())
}

pub(crate) static simulationOptionsNames: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("startTime"),
            literal!("stopTime"),
            literal!("numberOfIntervals"),
            literal!("tolerance"),
            literal!("method"),
            literal!("fileNamePrefix"),
            literal!("options"),
            literal!("outputFormat"),
            literal!("variableFilter"),
            literal!("cflags"),
            literal!("simflags")
        ]
    });

pub(crate) fn getSimulationResultType() -> Result<metamodelica::Ref<DAE::Type>> {
    let mut t: metamodelica::Ref<DAE::Type>;
    t = if (Testsuite::isRunning()?) {
        simulationResultType_rtest().clone()
    } else {
        simulationResultType_full().clone()
    };
    Ok(t)
}

pub(crate) fn getDrModelicaSimulationResultType() -> Result<metamodelica::Ref<DAE::Type>> {
    let mut t: metamodelica::Ref<DAE::Type>;
    t = if (Testsuite::isRunning()?) {
        simulationResultType_rtest().clone()
    } else {
        simulationResultType_drModelica().clone()
    };
    Ok(t)
}

pub(crate) fn createSimulationResult(
    mut resultFile: ArcStr,
    mut options: ArcStr,
    mut message: ArcStr,
    mut inAddResultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut fields: metamodelica::List<ArcStr>;
    let mut notest: bool;
    resultValues = inAddResultValues.reverse();
    notest = !(Testsuite::isRunning()?);
    fields = if (notest) {
        List::map(resultValues.clone(), &fnptr!(Util::tuple21, _))?
    } else {
        metamodelica::nil()
    };
    vals = if (notest) {
        List::map(resultValues, &fnptr!(Util::tuple22, _))?
    } else {
        metamodelica::nil()
    };
    res = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("SimulationResult"),
        }),
        orderd: metamodelica::cons(
            metamodelica::Ref::new(Values::Value::STRING { string: resultFile }),
            metamodelica::cons(
                metamodelica::Ref::new(Values::Value::STRING { string: options }),
                metamodelica::cons(metamodelica::Ref::new(Values::Value::STRING { string: message }), vals),
            ),
        ),
        comp: metamodelica::cons(
            literal!("resultFile"),
            metamodelica::cons(
                literal!("simulationOptions"),
                metamodelica::cons(literal!("messages"), fields),
            ),
        ),
        index: -1,
    });
    Ok(res)
}

pub(crate) fn createSimulationResultFailure(
    mut message: ArcStr,
    mut options: ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    res = createSimulationResult(
        literal!(""),
        options,
        message,
        zeroAdditionalSimulationResultValues.clone(),
    )?;
    Ok(res)
}

fn buildCurrentSimulationResultExp() -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    cref = ComponentReferenceBasics::makeCrefIdent(
        literal!("currentSimulationResult"),
        DAE::T_UNKNOWN_DEFAULT().clone(),
        metamodelica::nil(),
    );
    outExp = Expression::makeCrefExp(cref, DAE::T_UNKNOWN_DEFAULT().clone())?;
    Ok(outExp)
}

fn cevalCurrentSimulationResultExp(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inputFilename: ArcStr,
    mut msg: Absyn::Msg,
) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache;
    let mut filename: ArcStr;
    (outCache, filename) = (::match_deref::match_deref! { match &(inputFilename.clone()) {
        Deref @ "<default>" => {
            let mut cache = inCache.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache, env, buildCurrentSimulationResultExp()?, true, msg, 0)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            filename = metamodelica::Own::own(__pa1);
            (cache, filename)
        },
        _ => {
            (inCache, inputFilename)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, filename))
}

pub(crate) fn convertSimulationOptionsToSimCode(
    mut opts: &InteractiveTypes::SimulationOptions,
) -> Result<SimCode::SimulationSettings> {
    let mut settings: SimCode::SimulationSettings;
    settings = (::match_deref::match_deref! { match &(opts) {
        InteractiveTypes::SimulationOptions { startTime: Deref @ DAE::Exp::RCONST { real: startTime }, stopTime: Deref @ DAE::Exp::RCONST { real: stopTime }, numberOfIntervals: Deref @ DAE::Exp::ICONST { integer: nIntervals }, stepSize: Deref @ DAE::Exp::RCONST { real: stepSize }, tolerance: Deref @ DAE::Exp::RCONST { real: tolerance }, method: Deref @ DAE::Exp::SCONST { string: method }, fileNamePrefix: _, options: _, outputFormat: Deref @ DAE::Exp::SCONST { string: format }, variableFilter: Deref @ DAE::Exp::SCONST { string: varFilter }, cflags: Deref @ DAE::Exp::SCONST { string: cflags }, simflags: Deref @ DAE::Exp::SCONST { string: simflags } } => {
            let mut options: ArcStr;
            options = literal!("");
            SimCode::SimulationSettings { startTime: startTime.clone(), stopTime: stopTime.clone(), numberOfIntervals: nIntervals.clone(), stepSize: stepSize.clone(), tolerance: tolerance.clone(), method: method.clone(), options: options, outputFormat: format.clone(), variableFilter: varFilter.clone(), cflags: cflags.clone(), simflags: simflags.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(settings)
}

pub(crate) fn buildSimulationOptions(
    mut startTime: metamodelica::Ref<DAE::Exp>,
    mut stopTime: metamodelica::Ref<DAE::Exp>,
    mut numberOfIntervals: metamodelica::Ref<DAE::Exp>,
    mut stepSize: metamodelica::Ref<DAE::Exp>,
    mut tolerance: metamodelica::Ref<DAE::Exp>,
    mut method: metamodelica::Ref<DAE::Exp>,
    mut fileNamePrefix: metamodelica::Ref<DAE::Exp>,
    mut options: metamodelica::Ref<DAE::Exp>,
    mut outputFormat: metamodelica::Ref<DAE::Exp>,
    mut variableFilter: metamodelica::Ref<DAE::Exp>,
    mut cflags: metamodelica::Ref<DAE::Exp>,
    mut simflags: metamodelica::Ref<DAE::Exp>,
) -> InteractiveTypes::SimulationOptions {
    let mut outSimulationOptions: InteractiveTypes::SimulationOptions;
    outSimulationOptions = InteractiveTypes::SimulationOptions {
        startTime: startTime,
        stopTime: stopTime,
        numberOfIntervals: numberOfIntervals,
        stepSize: stepSize,
        tolerance: tolerance,
        method: method,
        fileNamePrefix: fileNamePrefix,
        options: options,
        outputFormat: outputFormat,
        variableFilter: variableFilter,
        cflags: cflags,
        simflags: simflags,
    };
    outSimulationOptions
}

pub(crate) fn getSimulationOption(
    mut inSimOpt: &InteractiveTypes::SimulationOptions,
    mut optionName: ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outOptionValue: metamodelica::Ref<DAE::Exp>;
    outOptionValue = (::match_deref::match_deref! { match &((inSimOpt, optionName)) {
        (InteractiveTypes::SimulationOptions { startTime: e, .. }, Deref @ "startTime") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { stopTime: e, .. }, Deref @ "stopTime") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { numberOfIntervals: e, .. }, Deref @ "numberOfIntervals") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { stepSize: e, .. }, Deref @ "stepSize") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { tolerance: e, .. }, Deref @ "tolerance") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { method: e, .. }, Deref @ "method") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { fileNamePrefix: e, .. }, Deref @ "fileNamePrefix") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { options: e, .. }, Deref @ "options") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { outputFormat: e, .. }, Deref @ "outputFormat") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { variableFilter: e, .. }, Deref @ "variableFilter") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { cflags: e, .. }, Deref @ "cflags") => {
            e.clone()
        },
        (InteractiveTypes::SimulationOptions { simflags: e, .. }, Deref @ "simflags") => {
            e.clone()
        },
        (_, name) => {
            let mut msg: ArcStr;
            msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown simulation option: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            Error::addCompilerWarning(msg)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptionValue)
}

pub(crate) fn buildSimulationOptionsFromModelExperimentAnnotation(
    mut inModelPath: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut defaultOption: Option<InteractiveTypes::SimulationOptions>,
) -> Result<InteractiveTypes::SimulationOptions> {
    let mut outSimOpt: InteractiveTypes::SimulationOptions;
    outSimOpt = 'mc: {
        let __mc_input = defaultOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut defaults: InteractiveTypes::SimulationOptions;
            let mut simOpt: InteractiveTypes::SimulationOptions;
            let mut experimentAnnotationStr: ArcStr;
            let mut named: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            let mut experiment_ann: Option<metamodelica::Ref<Absyn::Modification>>;
            loadProgram(&inModelPath)?;
            defaults = Util::getOptionOrDefault(
                defaultOption.clone(),
                setFileNamePrefixInSimulationOptions(defaultSimulationOptions().clone(), inFileNamePrefix.clone())?,
            );
            experiment_ann = InteractiveUtil::getInheritedAnnotation(
                inModelPath.clone(),
                literal!("experiment"),
                SymbolTable::getAbsyn(),
                true,
            )?;
            experimentAnnotationStr = Interactive::getExperimentAnnotationString(experiment_ann.clone())?;
            let false = (stringEq(&experimentAnnotationStr, &(literal!("{}")))) else {
                return Err("pattern mismatch");
            };
            experimentAnnotationStr =
                System::stringReplace(experimentAnnotationStr.clone(), literal!("{"), literal!(""))?;
            experimentAnnotationStr =
                System::stringReplace(experimentAnnotationStr.clone(), literal!("}"), literal!(""))?;
            let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("experiment(")); __mm_s.push_str(&*experimentAnnotationStr); __mm_s.push_str(&*literal!(");\n")); ArcStr::from(__mm_s) }, literal!("<experiment>"))?) {
                GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames: __pa0 }, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            named = metamodelica::Own::own(__pa0);
            simOpt = populateSimulationOptions(defaults.clone(), &named)?;
            Ok(simOpt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut defaults: InteractiveTypes::SimulationOptions;
            defaults =
                setFileNamePrefixInSimulationOptions(defaultSimulationOptions().clone(), inFileNamePrefix.clone())?;
            Ok(defaults.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSimOpt)
}

fn setFileNamePrefixInSimulationOptions(
    mut inSimOpt: InteractiveTypes::SimulationOptions,
    mut inFileNamePrefix: ArcStr,
) -> Result<InteractiveTypes::SimulationOptions> {
    let mut outSimOpt: InteractiveTypes::SimulationOptions;
    let mut startTime: metamodelica::Ref<DAE::Exp>;
    let mut stopTime: metamodelica::Ref<DAE::Exp>;
    let mut numberOfIntervals: metamodelica::Ref<DAE::Exp>;
    let mut stepSize: metamodelica::Ref<DAE::Exp>;
    let mut tolerance: metamodelica::Ref<DAE::Exp>;
    let mut method: metamodelica::Ref<DAE::Exp>;
    let mut options: metamodelica::Ref<DAE::Exp>;
    let mut outputFormat: metamodelica::Ref<DAE::Exp>;
    let mut variableFilter: metamodelica::Ref<DAE::Exp>;
    let mut cflags: metamodelica::Ref<DAE::Exp>;
    let mut simflags: metamodelica::Ref<DAE::Exp>;
    let mut UseOtimica: bool;
    UseOtimica =
        Config::acceptOptimicaGrammar()? || Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())?;
    let InteractiveTypes::SIMULATION_OPTIONS {
        startTime: __pa0,
        stopTime: __pa1,
        numberOfIntervals: __pa2,
        stepSize: __pa3,
        tolerance: __pa4,
        method: __pa5,
        fileNamePrefix: _,
        options: __pa6,
        outputFormat: __pa7,
        variableFilter: __pa8,
        cflags: __pa9,
        simflags: __pa10,
    } = inSimOpt;
    startTime = metamodelica::Own::own(__pa0);
    stopTime = metamodelica::Own::own(__pa1);
    numberOfIntervals = metamodelica::Own::own(__pa2);
    stepSize = metamodelica::Own::own(__pa3);
    tolerance = metamodelica::Own::own(__pa4);
    method = metamodelica::Own::own(__pa5);
    options = metamodelica::Own::own(__pa6);
    outputFormat = metamodelica::Own::own(__pa7);
    variableFilter = metamodelica::Own::own(__pa8);
    cflags = metamodelica::Own::own(__pa9);
    simflags = metamodelica::Own::own(__pa10);
    if UseOtimica {
        method = metamodelica::Ref::new(DAE::Exp::SCONST {
            string: literal!("optimization"),
        });
    } else if Flags::getConfigBool(Flags::DAE_MODE.clone())? {
        method = metamodelica::Ref::new(DAE::Exp::SCONST {
            string: literal!("ida"),
        });
    }
    numberOfIntervals = if (UseOtimica) {
        metamodelica::Ref::new(DAE::Exp::ICONST { integer: 50 })
    } else {
        numberOfIntervals
    };
    outSimOpt = InteractiveTypes::SimulationOptions {
        startTime: startTime,
        stopTime: stopTime,
        numberOfIntervals: numberOfIntervals,
        stepSize: stepSize,
        tolerance: tolerance,
        method: method,
        fileNamePrefix: metamodelica::Ref::new(DAE::Exp::SCONST {
            string: inFileNamePrefix,
        }),
        options: options,
        outputFormat: outputFormat,
        variableFilter: variableFilter,
        cflags: cflags,
        simflags: simflags,
    };
    Ok(outSimOpt)
}

fn getConst(
    mut inAbsynExp: &metamodelica::Ref<Absyn::Exp>,
    mut inExpType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&**inAbsynExp, &**inExpType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp }, _) => {
                    let mut i: i32;
                    let __pa0 = ::match_deref::match_deref! { match &(getConst(metamodelica::AsArg::as_arg(&exp), inExpType)?) {
                        Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i = metamodelica::Own::own(__pa0);
                    i = intNeg(i);
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp }, _) => {
                    let mut r: metamodelica::Real;
                    let __pa0 = ::match_deref::match_deref! { match &(getConst(metamodelica::AsArg::as_arg(&exp), inExpType)?) {
                        Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    r = -(r);
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: i }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { value: r#str }, Deref @ DAE::Type::T_REAL { .. }) => {
                    let mut r: metamodelica::Real;
                    r = stringReal(r#str.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: i }, Deref @ DAE::Type::T_REAL { .. }) => {
                    let mut r: metamodelica::Real;
                    r = intReal(i.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.getConst: experiment annotation contains unsupported expression: ")); __mm_s.push_str(&*Dump::printExpStr(inAbsynExp.clone())?); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(inExpType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Error::addCompilerError(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn populateSimulationOptions(
    mut options: InteractiveTypes::SimulationOptions,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<InteractiveTypes::SimulationOptions> {
    let mut options: InteractiveTypes::SimulationOptions = options;
    let mut name: ArcStr;
    let mut value: metamodelica::Ref<Absyn::Exp>;
    let mut interval: Option<metamodelica::Ref<DAE::Exp>> = None;
    for mut arg in &**args {
        let __arc2 = arg.clone();
        let Absyn::NAMEDARG {
            argName: __pa0,
            argValue: __pa1,
        } = &*__arc2;
        name = metamodelica::Own::own(__pa0);
        value = metamodelica::Own::own(__pa1);
        let () = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "Tolerance" => {
                options.tolerance = getConst(&value, &(DAE::T_REAL_DEFAULT().clone()))?;
                ()
            },
            Deref @ "StartTime" => {
                options.startTime = getConst(&value, &(DAE::T_REAL_DEFAULT().clone()))?;
                ()
            },
            Deref @ "StopTime" => {
                options.stopTime = getConst(&value, &(DAE::T_REAL_DEFAULT().clone()))?;
                ()
            },
            Deref @ "NumberOfIntervals" => {
                options.numberOfIntervals = getConst(&value, &(DAE::T_INTEGER_DEFAULT().clone()))?;
                ()
            },
            Deref @ "Interval" => {
                interval = Some(getConst(&value, &(DAE::T_REAL_DEFAULT().clone()))?);
                ()
            },
            _ => {
                if !(StringUtil::startsWith(name.clone(), literal!("__"))) {
                    Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Ignoring unknown experiment annotation option: ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Dump::printExpStr(value)?); ArcStr::from(__mm_s) })?;
                }
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if (interval).is_some() {
        options = setSimulationOptionsInterval(options, Expression::toReal(&(Util::getOption(interval)?))?)?;
    } else {
        options.stepSize = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::real_div_checked(
                (Expression::toReal(&options.stopTime)? - Expression::toReal(&options.startTime)?),
                metamodelica::OrderedFloat((500) as f64),
            )?,
        });
    }
    Ok(options)
}

fn setSimulationOptionsInterval(
    mut options: InteractiveTypes::SimulationOptions,
    mut interval: metamodelica::Real,
) -> Result<InteractiveTypes::SimulationOptions> {
    let mut options: InteractiveTypes::SimulationOptions = options;
    let mut start_time: metamodelica::Real;
    let mut stop_time: metamodelica::Real;
    start_time = Expression::toReal(&options.startTime)?;
    stop_time = Expression::toReal(&options.stopTime)?;
    options.stepSize = metamodelica::Ref::new(DAE::Exp::RCONST { real: interval });
    options.numberOfIntervals = metamodelica::Ref::new(DAE::Exp::ICONST {
        integer: ((metamodelica::real_div_checked((stop_time - start_time), interval)?
            + metamodelica::OrderedFloat(0.5_f64))
        .0
        .floor() as i32),
    });
    Ok(options)
}

fn simOptionsAsString(mut vals: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = &**vals;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: lst } => {
                    let mut simOptsValues: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr = r#str.clone();
                    simOptsValues = List::map(lst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesDump::valString(&__a0))?;
                    simOptsValues = List::map2(simOptsValues.clone(), &System::stringReplace, literal!("\""), literal!("'"))?;
                    r#str = Util::buildMapStr(&(simulationOptionsNames.clone()), &simOptsValues, literal!(" = "), literal!(", "))?;
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: lst } => {
                    let mut simOptsValues: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr = r#str.clone();
                    simOptsValues = List::map(lst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesDump::valString(&__a0))?;
                    simOptsValues = List::map2(simOptsValues.clone(), &System::stringReplace, literal!("\""), literal!("'"))?;
                    r#str = stringDelimitList(simOptsValues.clone(), literal!(", "));
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#str)
}

fn diffSanityCheckEqual(mut s1: ArcStr, mut s2: ArcStr) -> Result<bool> {
    use openmodelica_backend_tools::LexerModelicaDiff::Token;
    use openmodelica_backend_tools::LexerModelicaDiff::blockCommentCanonical;
    use openmodelica_backend_tools::LexerModelicaDiff::isBlockComment;
    use openmodelica_backend_tools::LexerModelicaDiff::isLineComment;
    use openmodelica_backend_tools::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use openmodelica_backend_tools::LexerModelicaDiff::scanString;
    use openmodelica_backend_tools::LexerModelicaDiff::tokenContent;
    let mut b: bool;
    let mut ts1: metamodelica::List<Token>;
    let mut ts2: metamodelica::List<Token>;
    let mut comments1: metamodelica::List<ArcStr>;
    let mut comments2: metamodelica::List<ArcStr>;
    (ts1, _) = scanString(s1, literal!("<StringSource>"))?;
    (ts2, _) = scanString(s2, literal!("<StringSource>"))?;
    if stringAppendList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (ts1.clone()).into_iter().cloned() {
                if !(!(modelicaDiffTokenWhitespace(t.clone()))) {
                    continue;
                }
                let __x = tokenContent(t.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    ) != stringAppendList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (ts2.clone()).into_iter().cloned() {
                if !(!(modelicaDiffTokenWhitespace(t.clone()))) {
                    continue;
                }
                let __x = tokenContent(t.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    ) {
        b = false;
        return Ok(b);
    }
    comments1 = List::sort(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (ts1).into_iter().cloned() {
                if !(isLineComment(&(t.clone())) || isBlockComment(&(t.clone()))) {
                    continue;
                }
                let __x = diffSanityCheckCommentStr(t.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?;
    comments2 = List::sort(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut t in (ts2).into_iter().cloned() {
                if !(isLineComment(&(t.clone())) || isBlockComment(&(t.clone()))) {
                    continue;
                }
                let __x = diffSanityCheckCommentStr(t.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
    )?;
    b = List::isEqualOnTrue(comments1, comments2, &fnptr!(stringEq, ArcStr, ArcStr))?;
    Ok(b)
}

fn diffSanityCheckCommentStr(mut t: LexerModelicaDiff::Token) -> Result<ArcStr> {
    use openmodelica_backend_tools::LexerModelicaDiff::blockCommentCanonical;
    use openmodelica_backend_tools::LexerModelicaDiff::isBlockComment;
    use openmodelica_backend_tools::LexerModelicaDiff::tokenContent;
    let mut s: ArcStr;
    s = if (isBlockComment(&t)) {
        stringDelimitList(blockCommentCanonical(t)?, literal!("\n"))
    } else {
        tokenContent(t)?
    };
    Ok(s)
}

pub(crate) fn cevalInteractiveFunctions3(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFunctionName: &ArcStr,
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut msg: Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    use openmodelica_backend_tools::LexerModelicaDiff::Token;
    use openmodelica_backend_tools::LexerModelicaDiff::TokenId;
    use openmodelica_backend_tools::LexerModelicaDiff::filterModelicaDiff;
    use openmodelica_backend_tools::LexerModelicaDiff::modelicaDiffTokenEq;
    use openmodelica_backend_tools::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use openmodelica_backend_tools::LexerModelicaDiff::reportErrors;
    use openmodelica_backend_tools::LexerModelicaDiff::scanString;
    use openmodelica_backend_tools::LexerModelicaDiff::tokenContent;
    use openmodelica_util::DiffAlgorithm::Diff;
    use openmodelica_util::DiffAlgorithm::diff;
    use openmodelica_util::DiffAlgorithm::printActual;
    use openmodelica_util::DiffAlgorithm::printDiffTerminalColor;
    use openmodelica_util::DiffAlgorithm::printDiffXml;
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inFunctionName.clone(), inVals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut blst: metamodelica::List<bool>;
                    let mut forkedSymbolTable: metamodelica::Ref<SymbolTable::SymbolTable>;
                    strs = List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    forkedSymbolTable = SymbolTable::get();
                    blst = System::launchParallelTasks(i.clone(), List::map1(strs.clone(), &fnptr!(Util::makeTuple, _, _), forkedSymbolTable.clone())?, (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::Ref<SymbolTable::SymbolTable>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(Interactive::evaluateFork(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<SymbolTable::SymbolTable>)) -> Result<bool> + 'static>))?;
                    v = ValuesMake::makeArray(List::map(blst.clone(), &fnptr!(ValuesMake::makeBoolean, bool))?);
                    SymbolTable::update(forkedSymbolTable.clone());
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut is: metamodelica::List<i32>;
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    strs = List::map1r(strs.clone(), &fnptr!(stringAppend, ArcStr, ArcStr), stringAppend(Settings::getInstallationDirectoryPath()?, literal!("/bin/omc ")))?;
                    is = System::systemCallParallel(strs.clone(), i.clone());
                    Ok(ValuesMake::makeArray(List::map(List::map1(is.clone(), &fnptr!(intEq, i32, i32), 0)?, &fnptr!(ValuesMake::makeBoolean, bool))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(ValuesMake::makeArray(List::fill(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), ((vals).len() as i32))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setClassComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::setClassComment(path.clone(), r#str.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isShortDefinition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = isShortDefinition(path.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUsedClassNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    sp = SymbolTable::getSCode()?;
                    (sp, _) = NFSCodeFlatten::flattenClassInProgram(path.clone(), sp.clone())?;
                    sp = SCodeUtil::removeBuiltinsFromTopScope(sp.clone())?;
                    paths = Interactive::getSCodeClassNamesRecursive(&sp)?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUsedClassNames", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let mut elem: metamodelica::Ref<Absyn::Element>;
                    elem = InteractiveUtil::getPathedElementInProgram(path.clone(), &(SymbolTable::getAbsyn()))?;
                    r#str = System::unescapedString(getClassElementComment(&elem));
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getPackages", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AllLoadedClasses" } } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    paths = Interactive::getTopPackages(&(SymbolTable::getAbsyn()))?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getPackages", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    paths = Interactive::getPackagesInPath(path.clone(), SymbolTable::getAbsyn());
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertUnits", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut offset: metamodelica::Real;
                    let mut offset1: metamodelica::Real;
                    let mut offset2: metamodelica::Real;
                    let mut scaleFactor: metamodelica::Real;
                    let mut scaleFactor1: metamodelica::Real;
                    let mut scaleFactor2: metamodelica::Real;
                    let mut b: bool;
                    let mut u1: UnitAbsyn::Unit;
                    let mut u2: UnitAbsyn::Unit;
                    Error::clearMessages();
                    UnitParserExt::initSIUnits();
                    (u1, scaleFactor1, offset1) = UnitAbsynBuilder::str2unitWithScaleFactor(str1.clone(), None)?;
                    (u2, scaleFactor2, offset2) = UnitAbsynBuilder::str2unitWithScaleFactor(str2.clone(), None)?;
                    b = u1.clone() == u2.clone();
                    scaleFactor = realDiv(scaleFactor2, scaleFactor1);
                    offset = realDiv((offset2) - (offset1), scaleFactor1);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: b }), metamodelica::Ref::new(Values::Value::REAL { real: scaleFactor }), metamodelica::Ref::new(Values::Value::REAL { real: offset })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertUnits", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) }), metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedUnits", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut u1: UnitAbsyn::Unit;
                    Error::clearMessages();
                    UnitParserExt::initSIUnits();
                    u1 = UnitAbsynBuilder::str2unit(str1.clone(), None)?;
                    strs = UnitAbsynBuilder::getDerivedUnits(u1.clone(), metamodelica::AsArg::as_arg(&str1))?;
                    Ok(ValuesMake::makeArray(List::map(strs.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedUnits", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassInformation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(getClassInformation(className.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassInformation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }), metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }), metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }), metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(className.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(getTransitions(className.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD { .. }, .. } } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = Interactive::addTransition(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str3.clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = Interactive::addTransitionWithAnnotation(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str3.clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: eltargs.clone() }), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str3.clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD { .. }, .. } } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b5 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str3.clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = Interactive::addTransition(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str4.clone(), b3.clone(), b4.clone(), b5.clone(), i1.clone(), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b5 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str3.clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = Interactive::addTransitionWithAnnotation(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), str1.clone(), str2.clone(), str4.clone(), b3.clone(), b4.clone(), b5.clone(), i1.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: eltargs.clone() }), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(className.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(getInitialStates(className.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD { .. }, .. } } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = addInitialState(classpath.clone(), str1.clone(), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = addInitialStateWithAnnotation(classpath.clone(), str1.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: eltargs.clone() }), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = deleteInitialState(classpath.clone(), str1.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD { .. }, .. } } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str: ArcStr;
                    let false = (Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    r#str = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = deleteInitialState(classpath.clone(), str1.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = addInitialState(classpath.clone(), str1.clone(), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: eltargs, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut bval: bool;
                    (bval, p) = deleteInitialState(classpath.clone(), str1.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = addInitialStateWithAnnotation(classpath.clone(), str1.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: eltargs.clone() }), p.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: bval }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "diffModelicaFileListings", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                            let mut s3: ArcStr;
                            let mut s4: ArcStr;
                            let mut s5: ArcStr;
                            let mut r#str: ArcStr;
                            let mut bom: ArcStr;
                            let mut sanityCheckFailed: bool;
                            let mut lineEndingIsCRLF: bool;
                            let mut tokens1: metamodelica::List<Token>;
                            let mut tokens2: metamodelica::List<Token>;
                            let mut errorTokens: metamodelica::List<Token>;
                            let mut parseTree1: metamodelica::List<metamodelica::Ref<SimpleModelicaParser::ParseTree>>;
                            let mut parseTree2: metamodelica::List<metamodelica::Ref<SimpleModelicaParser::ParseTree>>;
                            let mut treeDiffs: metamodelica::List<(Diff, metamodelica::List<metamodelica::Ref<SimpleModelicaParser::ParseTree>>)>;
                            let mut s1 = (*s1).clone();
                            let mut s2 = (*s2).clone();
                            ExecStat::execStatReset()?;
                            (s1, bom) = StringUtil::stripBOM(s1.clone())?;
                            lineEndingIsCRLF = !metamodelica::stringEq(&(s1.clone()), &(System::stringReplace(s1.clone(), literal!("\r\n"), literal!("\n"))?));
                            s1 = System::stringReplace(s1.clone(), literal!("\r\n"), literal!("\n"))?;
                            s1 = System::stringReplace(s1.clone(), literal!("\r"), literal!("\n"))?;
                            (tokens1, errorTokens) = scanString(s1.clone(), literal!("<StringSource>"))?;
                            reportErrors(&errorTokens)?;
                            if false && s1.clone() != stringAppendList(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut t in (tokens1.clone()).into_iter().cloned() {
                            let __x = tokenContent(t.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })) {
                                System::writeFile(literal!("string.before"), s1.clone())?;
                                System::writeFile(literal!("string.after"), stringAppendList(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut t in (tokens1.clone()).into_iter().cloned() {
                            let __x = tokenContent(t.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })))?;
                                Error::terminate(literal!("Lexed string does not match the original. See files string.before and string.after"), &(metamodelica::sourceInfo!("Script/CevalScriptBackend.mo")))?;
                                return Err("fail");
                            }
                            ExecStat::execStat(&(literal!("diffModelicaFileListings scan string 1")))?;
                            (_, parseTree1) = SimpleModelicaParser::stored_definition(tokens1.clone(), metamodelica::nil())?;
                            ExecStat::execStat(&(literal!("diffModelicaFileListings parse string 1")))?;
                            if false && !metamodelica::stringEq(&s1, &(SimpleModelicaParser::parseTreeStr(&parseTree1)?)) {
                                System::writeFile(literal!("string.before"), s1.clone())?;
                                System::writeFile(literal!("string.after"), SimpleModelicaParser::parseTreeStr(&parseTree1)?)?;
                                Error::terminate(literal!("Parsed string does not match the original. See files string.before and string.after"), &(metamodelica::sourceInfo!("Script/CevalScriptBackend.mo")))?;
                                return Err("fail");
                            }
                            (s2, bom) = StringUtil::stripBOM(s2.clone())?;
                            s2 = System::stringReplace(s2.clone(), literal!("\r\n"), literal!("\n"))?;
                            s2 = System::stringReplace(s2.clone(), literal!("\r"), literal!("\n"))?;
                            (tokens2, errorTokens) = scanString(s2.clone(), literal!("<StringSource>"))?;
                            reportErrors(&errorTokens)?;
                            ExecStat::execStat(&(literal!("diffModelicaFileListings scan string 2")))?;
                            (_, parseTree2) = SimpleModelicaParser::stored_definition(tokens2.clone(), metamodelica::nil())?;
                            ExecStat::execStat(&(literal!("diffModelicaFileListings parse string 2")))?;
                            if false && !metamodelica::stringEq(&s2, &(SimpleModelicaParser::parseTreeStr(&parseTree2)?)) {
                                System::writeFile(literal!("string.before"), s2.clone())?;
                                System::writeFile(literal!("string.after"), SimpleModelicaParser::parseTreeStr(&parseTree2)?)?;
                                Error::terminate(literal!("Parsed string does not match the original. See files string.before and string.after"), &(metamodelica::sourceInfo!("Script/CevalScriptBackend.mo")))?;
                                return Err("fail");
                            }
                            treeDiffs = SimpleModelicaParser::treeDiff(parseTree1.clone(), parseTree2.clone(), std::cmp::max(((tokens1).len() as i32), ((tokens2).len() as i32)))?;
                            ExecStat::execStat(&(literal!("treeDiff")))?;
                            sanityCheckFailed = false;
                            if true {
                                s3 = Dump::unparseStr(Parser::parsestring(s2.clone(), literal!("<interactive>"), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?, false, Dump::defaultDumpOptions.clone())?;
                                ExecStat::execStat(&(literal!("sanity parsestring(s2)")))?;
                                s5 = printActual(treeDiffs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SimpleModelicaParser::ParseTree>| SimpleModelicaParser::parseTreeNodeStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?;
                                match '__try0: {
                                    s4 = unwrap_break_err!(Dump::unparseStr(unwrap_break_err!(Parser::parsestring(s5.clone(), literal!("<interactive>"), unwrap_break_err!(Config::acceptedGrammar(), '__try0), unwrap_break_err!(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone()), '__try0), unwrap_break_err!(Flags::getConfigBool(Flags::STRICT.clone()), '__try0)), '__try0), false, Dump::defaultDumpOptions.clone()), '__try0);
                                    unwrap_break_err!(ExecStat::execStat(&(literal!("sanity parsestring(s5)"))), '__try0);
                                    Ok::<_, &'static str>((s4.clone(),))
                                } {
                                    Ok((__try0_o0,)) => {
                                                s4 = __try0_o0;
                                    }
                                    Err(__try0_err) => {
                                                System::writeFile(literal!("SanityCheckFail.mo"), s5.clone())?;
                                                Error::addInternalError(literal!("Failed to parse merged string (see generated file SanityCheckFail.mo)\n"), metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                                                return Err(__try0_err);
                                    }
                                }
                                if !(diffSanityCheckEqual(s3.clone(), s4.clone())?) {
                                    System::writeFile(literal!("SanityCheckFailBefore.mo"), s3.clone())?;
                                    System::writeFile(literal!("SanityCheckFailAfter.mo"), s4.clone())?;
                                    if b.clone() {
                                                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("After merging the strings, the semantics changed for some reason (see generated files SanityCheckFailBefore.mo SanityCheckFailAfter.mo). Will return the empty string:\ns1:\n")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\ns2:\n")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("\ns3:\n")); __mm_s.push_str(&*s3); __mm_s.push_str(&*literal!("\ns4:\n")); __mm_s.push_str(&*s4); __mm_s.push_str(&*literal!("\ns5:\n")); __mm_s.push_str(&*s5); __mm_s.push_str(&*literal!("\nparseTree2:")); __mm_s.push_str(&*SimpleModelicaParser::parseTreeStr(&parseTree2)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                                                return Err("fail");
                                    } else {
                                                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("After merging the strings, the semantics changed for some reason (see generated files SanityCheckFailBefore.mo SanityCheckFailAfter.mo). Will return s2:\ns1:\n")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!("\ns2:\n")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!("\ns3:\n")); __mm_s.push_str(&*s3); __mm_s.push_str(&*literal!("\ns4:\n")); __mm_s.push_str(&*s4); __mm_s.push_str(&*literal!("\ns5:\n")); __mm_s.push_str(&*s5); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                                    }
                                    sanityCheckFailed = true;
                                }
                            }
                            r#str = if (sanityCheckFailed) {s2.clone()} else {'mc: {
                let __mc_input = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path));
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                Deref @ "plain" => {
                                    Ok(printActual(treeDiffs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SimpleModelicaParser::ParseTree>| SimpleModelicaParser::parseTreeNodeStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                Deref @ "color" => {
                                    Ok(printDiffTerminalColor(treeDiffs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SimpleModelicaParser::ParseTree>| SimpleModelicaParser::parseTreeNodeStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                Deref @ "xml" => {
                                    Ok(printDiffXml(treeDiffs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SimpleModelicaParser::ParseTree>| SimpleModelicaParser::parseTreeNodeStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                _ => {
                                    Error::addInternalError(literal!("Unknown diffModelicaFileListings choice"), metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                                    Ok(return Err("fail"))
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            }};
                            r#str = if (lineEndingIsCRLF) {System::stringReplace(r#str.clone(), literal!("\n"), literal!("\r\n"))?} else {r#str.clone()};
                            Ok(metamodelica::Ref::new(Values::Value::STRING { string: { let mut __mm_s = String::new(); __mm_s.push_str(&*bom); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) } }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffModelicaFileListings", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportToFigaro", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    sp = SymbolTable::getSCode()?;
                    Figaro::run(sp.clone(), metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&s1), r#str.clone(), metamodelica::AsArg::as_arg(&str1), metamodelica::AsArg::as_arg(&str2), metamodelica::AsArg::as_arg(&str3))?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportToFigaro", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "inferBindings", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut pnew: Absyn::Program;
                    pnew = Binding::inferBindings(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "inferBindings", _) => {
                    metamodelica::print(literal!("failed inferBindings\n"));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateVerificationScenarios", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut pnew: Absyn::Program;
                    pnew = Binding::generateVerificationScenarios(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateVerificationScenarios", _) => {
                    metamodelica::print(literal!("failed to generateVerificationScenarios\n"));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "rewriteBlockCall", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut pnew: Absyn::Program;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    let mut within_: Absyn::Within;
                    let mut outCache: FCore::Cache = outCache.clone();
                    p = SymbolTable::getAbsyn();
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &p, false, false)?;
                    classes = list![absynClass.clone()];
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
                    within_ = ProgramUtil::buildWithin(classpath.clone())?;
                    pnew = BlockCallRewrite::rewriteBlockCall(Absyn::Program { classes: list![absynClass.clone()], within_: within_.clone() }, &(Absyn::Program { classes: classes.clone(), within_: within_.clone() }))?;
                    pnew = ProgramUtil::updateProgram(pnew.clone(), p.clone(), false, false)?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    outCache = FCore::emptyCache();
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "rewriteBlockCall", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "jacobian", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut res: ArcStr;
                    let mut filenameprefix: ArcStr;
                    let mut description: ArcStr;
                    let mut env: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut daelow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut jac: Option<metamodelica::List<(i32, i32, metamodelica::Ref<BackendDAE::Equation>)>>;
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(outCache.clone(), inEnv.clone(), path.clone(), true, false, true)?) {
                        (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    outCache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    dae = metamodelica::Own::own(__pa2);
                    filenameprefix = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    description = DAEUtil::daeDescription(&dae);
                    daelow = BackendDAECreate::lower(dae.clone(), outCache.clone(), env.clone(), BackendDAE::ExtraInfo { description: description.clone(), fileNamePrefix: filenameprefix.clone(), simflags: None })?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(BackendDAEUtil::preOptimizeBackendDAE(daelow.clone(), None)?) {
                        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa4 } => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    syst = metamodelica::Own::own(__pa3);
                    shared = metamodelica::Own::own(__pa4);
                    (syst, m, _) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(&shared))?;
                    vars = BackendVariable::daeVars(&syst);
                    eqnarr = BackendEquation::getEqnsFromEqSystem(&syst);
                    (jac, _) = SymbolicJacobian::calculateJacobian(vars.clone(), eqnarr.clone(), m.clone(), false, shared.clone());
                    res = BackendDump::dumpJacobianStr(jac.clone())?;
                    Ok((metamodelica::Ref::new(Values::Value::STRING { string: res.clone() }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModel", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } }) => {
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut b: bool;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                    (b, outCache, _, _, _) = translateModel(outCache.clone(), inEnv.clone(), className.clone(), filenameprefix.clone(), true, true, Some(simSettings.clone()))?;
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: b }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModel", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelEquationsUC", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: outputFile }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpExtractionSteps }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = Uncertainties::modelEquationsUC(outCache.clone(), inEnv.clone(), className.clone(), outputFile.clone(), dumpExtractionSteps.clone())?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelEquationsUC", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("There were errors during extraction of uncertainty equations. Use getErrorString() to see them.") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "translateModelFMU", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: _ } } } } }) => {
                            let mut b: bool;
                            let mut outCache: FCore::Cache = outCache.clone();
                            (b, outCache, _) = translateModelFMU(outCache.clone(), inEnv.clone(), className.clone(), str1.clone(), str2.clone(), filenameprefix.clone(), true, ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut vv in (cvars.clone()).into_iter().cloned() {
                            let __x = ValuesUtil::extractValueString(&(vv.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), None)?;
                            Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: b }), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModelFMU", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "buildModelFMU", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: _ } } } } } } }) => {
                            let mut ret_val: metamodelica::Ref<Values::Value>;
                            let mut simSettings: SimCode::SimulationSettings;
                            let mut simSettings: SimCode::SimulationSettings;
                            let mut outCache: FCore::Cache = outCache.clone();
                            simSettings = fmuSimulationSettings(className.clone(), filenameprefix.clone(), str3.clone())?;
                            (outCache, ret_val) = buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), str1.clone(), str2.clone(), filenameprefix.clone(), true, ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut vv in (cvars.clone()).into_iter().cloned() {
                            let __x = ValuesUtil::extractValueString(&(vv.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), Some(simSettings.clone()), str3.clone())?;
                            Ok((ret_val.clone(), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModelFMU", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildEncryptedPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut b1: bool;
                    p = SymbolTable::getAbsyn();
                    b1 = buildEncryptedPackage(className.clone(), b.clone(), &p)?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildEncryptedPackage", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModelXML", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    filenameprefix = Util::stringReplaceChar(filenameprefix.clone(), literal!("."), literal!("_"))?;
                    (outCache, ret_val) = translateModelXML(outCache.clone(), inEnv.clone(), className.clone(), filenameprefix.clone(), true, None)?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportDAEtoMatlab", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val, _) = getAdjacencyMatrix(outCache.clone(), inEnv.clone(), className.clone(), &msg, filenameprefix.clone())?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
                    (outCache, ret_val) = checkModel(outCache.clone(), inEnv.clone(), className.clone(), &msg)?;
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), false)?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkAllModelsRecursive", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: showProtected }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = checkAllModelsRecursive(outCache.clone(), inEnv.clone(), className.clone(), showProtected.clone(), msg.clone())?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateGraphics", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(translateGraphics(className.clone(), &msg))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLoadedLibraries", Deref @ metamodelica::ListNode::Nil) => {
                    let mut p: Absyn::Program;
                    p = SymbolTable::getAbsyn();
                    Ok(ValuesMake::makeArray(List::fold(&p.classes, &move |__a0: metamodelica::Ref<Absyn::Class>, __a1: metamodelica::List<metamodelica::Ref<Values::Value>>| makeLoadLibrariesEntryAbsyn(&__a0, __a1), metamodelica::nil())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelica_uriToFilename", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut res: ArcStr;
                    res = uriToFilename(s1.clone())?;
                    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following path is a loaded resource... ")); __mm_s.push_str(&*res); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        return Err("fail");
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: res.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelica_uriToFilename", _) => {
                    if !((!(Flags::getConfigBool(Flags::BUILDING_MODEL.clone())?))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationVersion", Deref @ metamodelica::ListNode::Nil) => {
                    let mut res: ArcStr;
                    res = Config::getAnnotationVersion()?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: res.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNoSimplify", Deref @ metamodelica::ListNode::Nil) => {
                    let mut b: bool;
                    b = Config::getNoSimplify()?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setNoSimplify", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Config::setNoSimplify(b.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getShowAnnotations", Deref @ metamodelica::ListNode::Nil) => {
                    let mut b: bool;
                    b = Config::showAnnotations()?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setShowAnnotations", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Config::setShowAnnotations(b.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVectorizationLimit", Deref @ metamodelica::ListNode::Nil) => {
                    let mut i: i32;
                    i = Config::vectorizationLimit()?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getOrderConnections", Deref @ metamodelica::ListNode::Nil) => {
                    let mut b: bool;
                    b = Config::orderConnections()?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModel", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut executable: ArcStr;
                    let mut initfilename: ArcStr;
                    let mut filenameprefix: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut b: bool;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    List::map_0(&(ClockIndexes::buildModelClocks.clone()), &System::realtimeClear)?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    if !(metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsic")))) {
                        (b, outCache, compileDir, executable, _, _, initfilename, _, _, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    } else {
                        filenameprefix = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                        match '__try0: {
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), literal!("2.0"), literal!("me"), literal!("<default>"), true, list![literal!("static")], None, literal!("<default>")), '__try0)) {
                                        (__pa1, Deref @ Values::Value::STRING { string: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                            } };
                            outCache = metamodelica::Own::own(__pa1);
                            r#str = metamodelica::Own::own(__pa2);
                            if stringEmpty(&r#str) {
                                        break '__try0 Err::<_, _>("fail");
                            }
                            b = true;
                            Ok::<_, &'static str>((b.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                        b = __try0_o0;
                            }
                            Err(_) => {
                                        b = false;
                            }
                        }
                        compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                        executable = { let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix); __mm_s.push_str(&*literal!("_me_FMU")); ArcStr::from(__mm_s) };
                        initfilename = { let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix); __mm_s.push_str(&*literal!("_init_xml")); ArcStr::from(__mm_s) };
                    }
                    executable = if (!(Testsuite::isRunning()?)) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*compileDir); __mm_s.push_str(&*executable); ArcStr::from(__mm_s) }} else {executable.clone()};
                    Ok((ValuesMake::makeArray(if (b) {list![metamodelica::Ref::new(Values::Value::STRING { string: executable.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: initfilename.clone() })]} else {list![metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })]}), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModel", _) => {
                    Ok(ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildLabel", vals) => {
                    let mut executable: ArcStr;
                    let mut initfilename: ArcStr;
                    let mut b: bool;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::GENERATE_LABELED_SIMCODE.clone(), true)?;
                    List::map_0(&(ClockIndexes::buildModelClocks.clone()), &System::realtimeClear)?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (b, outCache, _, executable, _, _, initfilename, _, _, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    Ok((ValuesMake::makeArray(if (b) {list![metamodelica::Ref::new(Values::Value::STRING { string: executable.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: initfilename.clone() })]} else {list![metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })]}), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reduceTerms", vals) => {
                    let mut executable: ArcStr;
                    let mut initfilename: ArcStr;
                    let mut b: bool;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::REDUCE_TERMS.clone(), true)?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_LABELED_SIMCODE.clone(), false)?;
                    FlagsUtil::disableDebug(Flags::WRITE_TO_BUFFER.clone())?;
                    List::map_0(&(ClockIndexes::buildModelClocks.clone()), &System::realtimeClear)?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    if ((vals).len() as i32) != 13 {
                        Error::addInternalError(literal!("reduceTerms expected 13 arguments"), metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                    }
                    (vals).get(13)?;
                    vals = listDelete(vals.clone(), 13)?;
                    (b, outCache, _, executable, _, _, initfilename, _, _, _, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    Ok((ValuesMake::makeArray(if (b) {list![metamodelica::Ref::new(Values::Value::STRING { string: executable.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: initfilename.clone() })]} else {list![metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })]}), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "simulate", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                            let mut simflags: ArcStr;
                            let mut r#str: ArcStr;
                            let mut executable: ArcStr;
                            let mut outputFormat_str: ArcStr;
                            let mut executableSuffixedExe: ArcStr;
                            let mut sim_call: ArcStr;
                            let mut result_file: ArcStr;
                            let mut filenameprefix: ArcStr;
                            let mut compileDir: ArcStr;
                            let mut exeDir: ArcStr;
                            let mut logFile: ArcStr;
                            let mut resimulateExecutable: ArcStr;
                            let mut simValue: metamodelica::Ref<Values::Value>;
                            let mut simSettings: SimCode::SimulationSettings;
                            let mut resI: i32;
                            let mut timeTotal: metamodelica::Real;
                            let mut timeSimulation: metamodelica::Real;
                            let mut b: bool;
                            let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
                            let mut simSettings: SimCode::SimulationSettings;
                            let mut vals = (*vals).clone();
                            let mut outCache: FCore::Cache = outCache.clone();
                            System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                            resimulateExecutable = (match &*(List::last(metamodelica::AsArg::as_arg(&vals))?) {
                Values::Value::STRING { string: __esc_str } => {
                            r#str = (*__esc_str).clone();
                            r#str.clone()
                },
                _ => literal!(""),
            });
                            vals = List::stripLast(vals.clone())?;
                            if !metamodelica::stringEq(&resimulateExecutable, &(literal!(""))) {
                                b = true;
                                executable = resimulateExecutable.clone();
                                compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                                simflags = (match &*(List::last(metamodelica::AsArg::as_arg(&vals))?) {
                Values::Value::STRING { string: __esc_str } => {
                            r#str = (*__esc_str).clone();
                            r#str.clone()
                },
                _ => literal!(""),
            });
                                resultValues = zeroBuildPhaseResultValues.clone();
                            } else if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsicpp"))) {
                                filenameprefix = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                                (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                                match '__try0: {
                                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), literal!("2.0"), literal!("me"), literal!("<default>"), true, list![literal!("static")], Some(simSettings.clone()), literal!("<default>")), '__try0)) {
                                                (__pa1, Deref @ Values::Value::STRING { string: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                                                _ => break '__try0 Err::<_, _>("pattern mismatch"),
                                    } };
                                    outCache = metamodelica::Own::own(__pa1);
                                    r#str = metamodelica::Own::own(__pa2);
                                    if stringEmpty(&r#str) {
                                                break '__try0 Err::<_, _>("fail");
                                    }
                                    b = true;
                                    Ok::<_, &'static str>((b.clone(),))
                                } {
                                    Ok((__try0_o0,)) => {
                                                b = __try0_o0;
                                    }
                                    Err(_) => {
                                                b = false;
                                    }
                                }
                                compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                                executable = filenameprefix.clone();
                                simflags = literal!("");
                                resultValues = zeroBuildPhaseResultValues.clone();
                            } else if !(metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsic")))) {
                                (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                            } else {
                                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![literal!("Can't simulate for SimCodeTarget=omsic!\n")])?;
                                return Err("fail");
                            }
                            if b {
                                exeDir = compileDir.clone();
                                (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                                let SimCode::SIMULATION_SETTINGS { outputFormat: __pa4, .. } = &simSettings;
                                outputFormat_str = metamodelica::Own::own(__pa4);
                                result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), compileDir.clone(), list![executable.clone(), literal!("_res."), outputFormat_str.clone()]));
                                result_file = selectResultFile(result_file.clone(), simflags.clone())?;
                                executableSuffixedExe = stringAppend(executable.clone(), getSimulationExtension(&(Config::simCodeTarget()?), &(arcstr::literal!(Autoconf::platform))));
                                logFile = stringAppend(executable.clone(), literal!(".log"));
                                if System::regularFileExists(logFile.clone()) {
                                    let 0 = (System::removeFile(logFile.clone())) else { return Err("pattern mismatch") };
                                }
                                sim_call = stringAppendList(list![literal!("\""), exeDir.clone(), executableSuffixedExe.clone(), literal!("\""), literal!(" "), simflags.clone()]);
                                System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                                SimulationResults::close();
                                if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                                    resI = CodegenWasmJit::runSimulation(executable.clone(), result_file.clone(), simflags.clone());
                                } else if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm"))) {
                                    resI = CodegenWasmJit::runSimulationWasmtime(executable.clone(), result_file.clone(), simflags.clone());
                                } else {
                                    resI = System::systemCallRestrictedEnv(sim_call.clone(), logFile.clone())?;
                                }
                                timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                            } else {
                                result_file = literal!("");
                                resI = 1;
                                timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                                logFile = literal!("");
                            }
                            timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                            (outCache, simValue) = createSimulationResultFromcallModelExecutable(b, resI, timeTotal, timeSimulation, resultValues.clone(), outCache.clone(), className.clone(), metamodelica::AsArg::as_arg(&vals), result_file.clone(), logFile.clone())?;
                            Ok((simValue.clone(), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "simulate", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    Settings::getInstallationDirectoryPath()?;
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to build model: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                    Ok(createSimulationResultFailure(res.clone(), simOptionsAsString(&(List::stripLast(vals.clone())?))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "simulate", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    Ok(createSimulationResultFailure({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Simulation failed for model: ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\nEnvironment variable OPENMODELICAHOME not set.")); ArcStr::from(__mm_s) }, simOptionsAsString(&(List::stripLast(vals.clone())?))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: direction }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = moveClass(metamodelica::AsArg::as_arg(&className), direction.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClass", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToTop", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = moveClassToTop(metamodelica::AsArg::as_arg(&className), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToTop", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToBottom", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = moveClassToBottom(metamodelica::AsArg::as_arg(&className), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToBottom", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "copyClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                            let mut p: Absyn::Program;
                            let mut absynClass: metamodelica::Ref<Absyn::Class>;
                            let mut within_: Absyn::Within;
                            p = SymbolTable::getAbsyn();
                            absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
                            within_ = InteractiveUtil::parseWithinPath(path.clone());
                            p = copyClass(absynClass.clone(), name.clone(), within_.clone(), classpath.clone(), p.clone())?;
                            absynClass = ProgramUtil::getPathedClassInProgram((match within_.clone() {
                Absyn::Within::WITHIN { .. } => ProgramUtil::joinPaths(name.clone(), var_field!(within_.path, Absyn::Within::WITHIN).clone())?,
                _ => metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            }), &p, false, false)?;
                            SymbolTable::setAbsynLoaded(p.clone(), Absyn::Program { classes: list![absynClass.clone()], within_: within_.clone() })?;
                            Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "copyClass", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut errMsg: ArcStr;
                    let false = (Interactive::existClass(className.clone(), &(SymbolTable::getAbsyn()))) else { return Err("pattern mismatch") };
                    errMsg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Linearization Failed. Model: ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" does not exist! Please load it first before linearization.")); ArcStr::from(__mm_s) };
                    Ok(createSimulationResultFailure(errMsg.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr;
                    let mut r#str: ArcStr;
                    let mut executable: ArcStr;
                    let mut outputFormat_str: ArcStr;
                    let mut executableSuffixedExe: ArcStr;
                    let mut sim_call: ArcStr;
                    let mut result_file: ArcStr;
                    let mut res: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut logFile: ArcStr;
                    let mut strlinearizeTime: ArcStr;
                    let mut simValue: metamodelica::Ref<Values::Value>;
                    let mut resI: i32;
                    let mut timeTotal: metamodelica::Real;
                    let mut timeSimulation: metamodelica::Real;
                    let mut linearizeTime: metamodelica::Real;
                    let mut b: bool;
                    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    r#str = Flags::getConfigString(Flags::LINEARIZATION_DUMP_LANGUAGE.clone())?;
                    if stringEq(&r#str, &(literal!("none"))) {
                        FlagsUtil::setConfigString(Flags::LINEARIZATION_DUMP_LANGUAGE.clone(), literal!("modelica"))?;
                    }
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    if b {
                        let __pa0 = ::match_deref::match_deref! { match &(getListNthShowError(vals.clone(), &(literal!("try to get stop time")), 0, 2)?) {
                            Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        linearizeTime = metamodelica::Own::own(__pa0);
                        executableSuffixedExe = stringAppend(executable.clone(), getSimulationExtension(&(Config::simCodeTarget()?), &(arcstr::literal!(Autoconf::platform))));
                        logFile = stringAppend(executable.clone(), literal!(".log"));
                        if System::regularFileExists(logFile.clone()) {
                            let 0 = (System::removeFile(logFile.clone())) else { return Err("pattern mismatch") };
                        }
                        strlinearizeTime = realString(linearizeTime);
                        simflags = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l=")); __mm_s.push_str(&*strlinearizeTime); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*simflags); ArcStr::from(__mm_s) };
                        sim_call = stringAppendList(list![literal!("\""), compileDir.clone(), executableSuffixedExe.clone(), literal!("\""), literal!(" "), simflags.clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                            result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), compileDir.clone(), list![executable.clone(), literal!("_res."), outputFormat_str.clone()]));
                            resI = CodegenWasmJit::runSimulation(executable.clone(), result_file.clone(), simflags.clone());
                        } else {
                            resI = System::systemCallRestrictedEnv(sim_call.clone(), logFile.clone())?;
                        }
                        if 0 == resI {
                            result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), compileDir.clone(), list![executable.clone(), literal!("_res."), outputFormat_str.clone()]));
                            timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                            timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                            simValue = createSimulationResult(result_file.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?, System::readFile(logFile.clone())?, metamodelica::cons((literal!("timeTotal"), metamodelica::Ref::new(Values::Value::REAL { real: timeTotal })), metamodelica::cons((literal!("timeSimulation"), metamodelica::Ref::new(Values::Value::REAL { real: timeSimulation })), resultValues.clone())))?;
                            SymbolTable::addVar(&(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("currentSimulationResult"), identType: DAE::T_STRING_DEFAULT().clone(), subscriptLst: metamodelica::nil() })), metamodelica::Ref::new(Values::Value::STRING { string: result_file.clone() }), &(FGraph::empty()))?;
                        } else {
                            res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Succeeding building the linearized executable, but failed to run the linearize command: ")); __mm_s.push_str(&*sim_call); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile(logFile.clone())?); ArcStr::from(__mm_s) };
                            simValue = createSimulationResultFailure(res.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?)?;
                        }
                    } else {
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                        simValue = createSimulationResult(literal!(""), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the linearize command: ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) }, metamodelica::cons((literal!("timeTotal"), metamodelica::Ref::new(Values::Value::REAL { real: timeTotal })), metamodelica::cons((literal!("timeSimulation"), metamodelica::Ref::new(Values::Value::REAL { real: timeSimulation })), resultValues.clone())))?;
                    }
                    Ok((simValue.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the linearize command: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                    Ok(createSimulationResultFailure(res.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "optimize", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr;
                    let mut executable: ArcStr;
                    let mut outputFormat_str: ArcStr;
                    let mut executableSuffixedExe: ArcStr;
                    let mut sim_call: ArcStr;
                    let mut result_file: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut exeDir: ArcStr;
                    let mut logFile: ArcStr;
                    let mut simValue: metamodelica::Ref<Values::Value>;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut resI: i32;
                    let mut timeTotal: metamodelica::Real;
                    let mut timeSimulation: metamodelica::Real;
                    let mut b: bool;
                    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
                    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(), true)?;
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    if b {
                        exeDir = compileDir.clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        let SimCode::SIMULATION_SETTINGS { outputFormat: __pa0, .. } = &simSettings;
                        outputFormat_str = metamodelica::Own::own(__pa0);
                        result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), compileDir.clone(), list![executable.clone(), literal!("_res."), outputFormat_str.clone()]));
                        executableSuffixedExe = stringAppend(executable.clone(), getSimulationExtension(&(Config::simCodeTarget()?), &(arcstr::literal!(Autoconf::platform))));
                        logFile = stringAppend(executable.clone(), literal!(".log"));
                        if System::regularFileExists(logFile.clone()) {
                            let 0 = (System::removeFile(logFile.clone())) else { return Err("pattern mismatch") };
                        }
                        sim_call = stringAppendList(list![literal!("\""), exeDir.clone(), executableSuffixedExe.clone(), literal!("\""), literal!(" "), simflags.clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                            resI = CodegenWasmJit::runSimulation(executable.clone(), result_file.clone(), simflags.clone());
                        } else if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm"))) {
                            resI = CodegenWasmJit::runSimulationWasmtime(executable.clone(), result_file.clone(), simflags.clone());
                        } else {
                            resI = System::systemCallRestrictedEnv(sim_call.clone(), logFile.clone())?;
                        }
                        timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                    } else {
                        result_file = literal!("");
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        resI = 1;
                        logFile = literal!("");
                    }
                    timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (outCache, simValue) = createSimulationResultFromcallModelExecutable(b, resI, timeTotal, timeSimulation, resultValues.clone(), outCache.clone(), className.clone(), metamodelica::AsArg::as_arg(&vals), result_file.clone(), logFile.clone())?;
                    Ok((simValue.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "optimize", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the optimize command: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                    Ok(createSimulationResultFailure(res.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "instantiateModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = instantiateModel(outCache.clone(), inEnv.clone(), className.clone())?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moo", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr;
                    let mut executable: ArcStr;
                    let mut outputFormat_str: ArcStr;
                    let mut executableSuffixedExe: ArcStr;
                    let mut sim_call: ArcStr;
                    let mut result_file: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut exeDir: ArcStr;
                    let mut logFile: ArcStr;
                    let mut simValue: metamodelica::Ref<Values::Value>;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut resI: i32;
                    let mut timeTotal: metamodelica::Real;
                    let mut timeSimulation: metamodelica::Real;
                    let mut b: bool;
                    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
                    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(), true)?;
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), &msg)?;
                    simflags = stringAppend(simflags.clone(), literal!(" -moo"));
                    if b {
                        exeDir = compileDir.clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        let SimCode::SIMULATION_SETTINGS { outputFormat: __pa0, .. } = &simSettings;
                        outputFormat_str = metamodelica::Own::own(__pa0);
                        result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), compileDir.clone(), list![executable.clone(), literal!("_res."), outputFormat_str.clone()]));
                        executableSuffixedExe = stringAppend(executable.clone(), getSimulationExtension(&(Config::simCodeTarget()?), &(arcstr::literal!(Autoconf::platform))));
                        logFile = stringAppend(executable.clone(), literal!(".log"));
                        if System::regularFileExists(logFile.clone()) {
                            let 0 = (System::removeFile(logFile.clone())) else { return Err("pattern mismatch") };
                        }
                        sim_call = stringAppendList(list![literal!("\""), exeDir.clone(), executableSuffixedExe.clone(), literal!("\""), literal!(" "), simflags.clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        resI = System::systemCallRestrictedEnv(sim_call.clone(), logFile.clone())?;
                        timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                    } else {
                        result_file = literal!("");
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        resI = 1;
                        logFile = literal!("");
                    }
                    timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (outCache, simValue) = createSimulationResultFromcallModelExecutable(b, resI, timeTotal, timeSimulation, resultValues.clone(), outCache.clone(), className.clone(), metamodelica::AsArg::as_arg(&vals), result_file.clone(), logFile.clone())?;
                    Ok((simValue.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moo", vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut res: ArcStr;
                    r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    res = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the moo command: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                    Ok(createSimulationResultFailure(res.clone(), simOptionsAsString(metamodelica::AsArg::as_arg(&vals))?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: fmiLogLevel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: inputConnectors }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: outputConnectors }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut pd: ArcStr;
                    let mut filename_1: ArcStr;
                    let mut name: ArcStr;
                    let mut outputFile: ArcStr;
                    let mut fmiContext: Option<i32>;
                    let mut fmiInstance: Option<i32>;
                    let mut fmiModelVariablesInstance: Option<i32>;
                    let mut fmiTypeDefinitionsList: metamodelica::List<FMI::TypeDefinitions>;
                    let mut fmiModelVariablesList: metamodelica::List<FMI::ModelVariables>;
                    let mut fmiExperimentAnnotation: FMI::ExperimentAnnotation;
                    let mut fmiInfo: FMI::Info;
                    let mut b: bool;
                    let mut workdir = (*workdir).clone();
                    Error::clearMessages();
                    let true = (System::regularFileExists(filename.clone())) else { return Err("pattern mismatch") };
                    workdir = if (System::directoryExists(workdir.clone())) {workdir.clone()} else {System::pwd()};
                    (b, fmiContext, fmiInstance, fmiInfo, fmiTypeDefinitionsList, fmiExperimentAnnotation, fmiModelVariablesInstance, fmiModelVariablesList) = FMIExt::initializeFMIImport(filename.clone(), workdir.clone(), fmiLogLevel.clone(), inputConnectors.clone(), outputConnectors.clone(), false)?;
                    let true = (b) else { return Err("pattern mismatch") };
                    fmiTypeDefinitionsList = fmiTypeDefinitionsList.clone().reverse();
                    fmiModelVariablesList = fmiModelVariablesList.clone().reverse();
                    s1 = System::tolower(arcstr::literal!(Autoconf::platform));
                    name = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    name = if (stringEq(&name, &(literal!("Default"))) || stringEq(&name, &(literal!("default")))) {literal!("")} else {name.clone()};
                    r#str = Tpl::tplString2({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: FMI::FmiImport, __a2: ArcStr| CodegenFMU::importFMUModelica(__a0, &__a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport, ArcStr) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0, _a1, _a2| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU.importFMUModelica needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("Script/CevalScriptBackend.mo")); return Err("CodegenFMU.importFMUModelica needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport, ArcStr) -> Result<Tpl::Text> + 'static>) } }, FMI::FmiImport { platform: s1.clone(), fmuFileName: filename.clone(), fmuWorkingDirectory: workdir.clone(), fmiLogLevel: fmiLogLevel.clone(), fmiDebugOutput: b2.clone(), fmiContext: fmiContext.clone(), fmiInstance: fmiInstance.clone(), fmiInfo: fmiInfo.clone(), fmiTypeDefinitionsList: fmiTypeDefinitionsList.clone(), fmiExperimentAnnotation: fmiExperimentAnnotation, fmiModelVariablesInstance: fmiModelVariablesInstance.clone(), fmiModelVariablesList: fmiModelVariablesList.clone(), generateInputConnectors: inputConnectors.clone(), generateOutputConnectors: outputConnectors.clone() }, name.clone())?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    str1 = FMI::getFMIModelIdentifier(&fmiInfo);
                    str2 = FMI::getFMIType(&fmiInfo);
                    str3 = FMI::getFMIVersion(&fmiInfo);
                    outputFile = if (stringEmpty(&name)) {stringAppendList(list![str1.clone(), literal!("_"), str2.clone(), literal!("_FMU.mo")])} else {stringAppendList(list![name.clone(), literal!(".mo")])};
                    filename_1 = if (b1.clone()) {stringAppendList(list![workdir.clone(), pd.clone(), outputFile.clone()])} else {outputFile.clone()};
                    System::writeFile(stringAppendList(list![workdir.clone(), pd.clone(), outputFile.clone()]), r#str.clone())?;
                    FMIExt::releaseFMIImport(fmiModelVariablesInstance.clone(), fmiInstance.clone(), fmiContext.clone(), str3.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: filename_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let false = (System::regularFileExists(filename.clone())) else { return Err("pattern mismatch") };
                    Error::clearMessages();
                    Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![filename.clone()])?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: fmiLogLevel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: inputConnectors }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: outputConnectors }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str3: ArcStr;
                    let mut pd: ArcStr;
                    let mut filename_1: ArcStr;
                    let mut outputFile: ArcStr;
                    let mut modeldescriptionfilename: ArcStr;
                    let mut tmpDir: ArcStr;
                    let mut tmpFile: ArcStr;
                    let mut fmiContext: Option<i32>;
                    let mut fmiInstance: Option<i32>;
                    let mut fmiModelVariablesInstance: Option<i32>;
                    let mut fmiTypeDefinitionsList: metamodelica::List<FMI::TypeDefinitions>;
                    let mut fmiModelVariablesList: metamodelica::List<FMI::ModelVariables>;
                    let mut fmiExperimentAnnotation: FMI::ExperimentAnnotation;
                    let mut fmiInfo: FMI::Info;
                    let mut b: bool;
                    let mut workdir = (*workdir).clone();
                    Error::clearMessages();
                    let true = (System::regularFileExists(filename.clone())) else { return Err("pattern mismatch") };
                    workdir = if (System::directoryExists(workdir.clone())) {workdir.clone()} else {System::pwd()};
                    tmpDir = System::createTemporaryDirectory({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getTempDirectoryPath()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("fmuTmp")); __mm_s.push_str(&*intString(System::intRand(1000))); ArcStr::from(__mm_s) })?;
                    tmpFile = { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpDir); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("modelDescription.xml")); ArcStr::from(__mm_s) };
                    System::systemCall({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -f ")); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*tmpFile); ArcStr::from(__mm_s) }, literal!(""));
                    modeldescriptionfilename = { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpDir); __mm_s.push_str(&*literal!("/modelDescription.fmu")); ArcStr::from(__mm_s) };
                    System::systemCall({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("zip -j ")); __mm_s.push_str(&*modeldescriptionfilename); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*tmpFile); ArcStr::from(__mm_s) }, literal!(""));
                    let true = (System::regularFileExists(modeldescriptionfilename.clone())) else { return Err("pattern mismatch") };
                    (b, fmiContext, fmiInstance, fmiInfo, fmiTypeDefinitionsList, fmiExperimentAnnotation, fmiModelVariablesInstance, fmiModelVariablesList) = FMIExt::initializeFMIImport(modeldescriptionfilename.clone(), tmpDir.clone(), fmiLogLevel.clone(), inputConnectors.clone(), outputConnectors.clone(), true)?;
                    let true = (b) else { return Err("pattern mismatch") };
                    fmiTypeDefinitionsList = fmiTypeDefinitionsList.clone().reverse();
                    fmiModelVariablesList = fmiModelVariablesList.clone().reverse();
                    s1 = System::tolower(arcstr::literal!(Autoconf::platform));
                    r#str = Tpl::tplString({ #[cfg(feature = "codegen_fmu_c")] { (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: FMI::FmiImport| CodegenFMU::importFMUModelDescription(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport) -> Result<Tpl::Text> + 'static>) } #[cfg(not(feature = "codegen_fmu_c"))] { (std::sync::Arc::new(|_a0, _a1| { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("CodegenFMU.importFMUModelDescription needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("Script/CevalScriptBackend.mo")); return Err("CodegenFMU.importFMUModelDescription needs the 'codegen_fmu_c' code generation target, which this OpenModelica build was compiled without") }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport) -> Result<Tpl::Text> + 'static>) } }, FMI::FmiImport { platform: s1.clone(), fmuFileName: modeldescriptionfilename.clone(), fmuWorkingDirectory: workdir.clone(), fmiLogLevel: fmiLogLevel.clone(), fmiDebugOutput: b2.clone(), fmiContext: fmiContext.clone(), fmiInstance: fmiInstance.clone(), fmiInfo: fmiInfo.clone(), fmiTypeDefinitionsList: fmiTypeDefinitionsList.clone(), fmiExperimentAnnotation: fmiExperimentAnnotation, fmiModelVariablesInstance: fmiModelVariablesInstance.clone(), fmiModelVariablesList: fmiModelVariablesList.clone(), generateInputConnectors: inputConnectors.clone(), generateOutputConnectors: outputConnectors.clone() })?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    str1 = FMI::getFMIModelIdentifier(&fmiInfo);
                    str3 = FMI::getFMIVersion(&fmiInfo);
                    outputFile = stringAppendList(list![workdir.clone(), pd.clone(), str1.clone(), literal!("_Input_Output_FMU.mo")]);
                    filename_1 = if (b1.clone()) {stringAppendList(list![workdir.clone(), pd.clone(), str1.clone(), literal!("_Input_Output_FMU.mo")])} else {stringAppendList(list![str1.clone(), literal!("_Input_Output_FMU.mo")])};
                    System::writeFile(outputFile.clone(), r#str.clone())?;
                    FMIExt::releaseFMIImport(fmiModelVariablesInstance.clone(), fmiInstance.clone(), fmiContext.clone(), str3.clone())?;
                    System::removeDirectory(tmpDir.clone());
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: filename_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    if !(System::regularFileExists(filename.clone())) {
                        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![filename.clone()])?;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getIndexReductionMethod", _) => {
                    let mut r#str: ArcStr;
                    r#str = Config::getIndexReductionMethod()?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableIndexReductionMethods", _) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(&(Flags::INDEX_REDUCTION_METHOD.clone()))?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ret_val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = cevalInteractiveFunctions4(inCache.clone(), inEnv.clone(), inFunctionName, inVals.clone(), msg.clone())?;
                    Ok((ret_val.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn cevalInteractiveFunctions4(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFunctionName: &ArcStr,
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut msg: Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    use openmodelica_backend_tools::LexerModelicaDiff::Token;
    use openmodelica_backend_tools::LexerModelicaDiff::TokenId;
    use openmodelica_backend_tools::LexerModelicaDiff::filterModelicaDiff;
    use openmodelica_backend_tools::LexerModelicaDiff::modelicaDiffTokenEq;
    use openmodelica_backend_tools::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use openmodelica_backend_tools::LexerModelicaDiff::reportErrors;
    use openmodelica_backend_tools::LexerModelicaDiff::scanString;
    use openmodelica_backend_tools::LexerModelicaDiff::tokenContent;
    use openmodelica_util::DiffAlgorithm::Diff;
    use openmodelica_util::DiffAlgorithm::diff;
    use openmodelica_util::DiffAlgorithm::printActual;
    use openmodelica_util::DiffAlgorithm::printDiffTerminalColor;
    use openmodelica_util::DiffAlgorithm::printDiffXml;
    let mut outCache: FCore::Cache = inCache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inFunctionName.clone(), inVals);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableIndexReductionMethods", _) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(&(Flags::INDEX_REDUCTION_METHOD.clone()))?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMatchingAlgorithm", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Config::getMatchingAlgorithm()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableMatchingAlgorithms", _) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(&(Flags::MATCHING_ALGORITHM.clone()))?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTearingMethod", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Config::getTearingMethod()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableTearingMethods", _) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut v2: metamodelica::Ref<Values::Value>;
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(&(Flags::TEARING_METHOD.clone()))?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut access: Access;
                    let mut b: bool;
                    b = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                        r#str = Dump::unparseStr(Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, true, Dump::defaultDumpOptions.clone())?;
                        if '__try0: {
                            unwrap_break_err!(System::writeFile(filename.clone(), r#str.clone()), '__try0);
                            b = true;
                            Ok::<(), &'static str>(())
                        }.is_err() {
                            Error::addMessage(Error::WRITING_FILE_ERROR.clone(), list![filename.clone()])?;
                        }
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "save", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let mut filename: ArcStr;
                    let mut newp: Absyn::Program;
                    let mut access: Access;
                    let mut b: bool;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        (newp, filename) = Interactive::getContainedClassAndFile(classpath.clone(), SymbolTable::getAbsyn())?;
                        r#str = Dump::unparseStr(newp.clone(), false, Dump::defaultDumpOptions.clone())?;
                        System::writeFile(filename.clone(), r#str.clone())?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "save", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveAll", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    r#str = Dump::unparseStr(SymbolTable::getAbsyn(), true, Dump::defaultDumpOptions.clone())?;
                    System::writeFile(filename.clone(), r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut cname: ArcStr;
                    cname = AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![cname.clone(), literal!("global")])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut s1: ArcStr;
                    let mut access: Access;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        (s1, _) = getTotalModel(classpath.clone(), b1.clone(), b2.clone(), b3.clone())?;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        s1 = literal!("");
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: s1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut access: Access;
                    let mut b: bool;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        saveTotalModel(filename.clone(), classpath.clone(), b1.clone(), b2.clone(), b3.clone(), false)?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "previous_saveTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut access: Access;
                    let mut b: bool;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        saveTotalModel(filename.clone(), classpath.clone(), b1.clone(), b2.clone(), b3.clone(), true)?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "previous_saveTotalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModelDebug", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut access: Access;
                    let mut b: bool;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        saveTotalModelDebug(filename.clone(), classpath.clone(), b1.clone(), b2.clone(), b3.clone())?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModelDebug", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDocumentationAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut access: Access;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access >= Access::documentation.clone() {
                        (str1, str2, str3) = ProgramUtil::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("Documentation") })), Some((literal!(""), literal!(""), literal!(""))), &Interactive::getDocumentationAnnotationString)?;
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        (str1, str2, str3) = (literal!(""), literal!(""), literal!(""));
                    }
                    Ok(ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::STRING { string: str1.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: str2.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: str3.clone() })]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    p = Interactive::addClassAnnotation(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath))), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: annlst, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    p = SymbolTable::getAbsyn();
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
                    absynClass = Interactive::addClassAnnotationToClass(absynClass.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = ProgramUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(metamodelica::AsArg::as_arg(&classpath))) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&classpath))? }} }, p.clone(), false, false)?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setDocumentationAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut aexp: metamodelica::Ref<Absyn::Exp>;
                    let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    p = SymbolTable::getAbsyn();
                    nargs = List::consOnTrue(!(stringEq(&str1, &(literal!("")))), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("info"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: System::escapedString(str1.clone(), false) }) }), metamodelica::nil());
                    nargs = List::consOnTrue(!(stringEq(&str2, &(literal!("")))), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("revisions"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: System::escapedString(str2.clone(), false) }) }), nargs.clone());
                    aexp = metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("Documentation"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: nargs.clone() }), typeVars: metamodelica::nil() });
                    p = Interactive::addClassAnnotation(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath))), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setDocumentationAnnotation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stat", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r1: metamodelica::Real;
                    let mut r2: metamodelica::Real;
                    let mut b: bool;
                    (b, r1, r2, _) = System::stat(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: b }), metamodelica::Ref::new(Values::Value::REAL { real: r1 }), metamodelica::Ref::new(Values::Value::REAL { real: r2 })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "regularFileExists", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut statFileType: System::StatFileType;
                    (_, _, _, statFileType) = System::stat(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: statFileType == System::StatFileType::RegularFile.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "directoryExists", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut statFileType: System::StatFileType;
                    (_, _, _, statFileType) = System::stat(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: statFileType == System::StatFileType::Directory.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelicaInternal_fullPathName", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::realpath(r#str.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isType", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isType(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isPackage(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isClass(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isRecord", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isRecord(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isBlock", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isBlock(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isFunction", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isFunction(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPartial", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isPartial(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isReplaceable", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isReplaceable(path.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isRedeclare", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isRedeclare(path.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isModel(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isConnector", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isConnector(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOptimization", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isOptimization(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isEnumeration", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isEnumeration(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperator", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isOperator(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperatorRecord", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isOperatorRecord(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperatorFunction", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut b: bool;
                    b = Interactive::isOperatorFunction(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isProtectedClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    b = Interactive::isProtectedClass(classpath.clone(), metamodelica::AsArg::as_arg(&name), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBuiltinType", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    (_, tp, _) = Lookup::lookupType(outCache.clone(), inEnv.clone(), classpath.clone(), Some(Absyn::dummyInfo.clone()))?;
                    r#str = TypesDump::unparseType(tp.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBuiltinType", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "extendsFrom", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    paths = Interactive::getAllInheritedClasses(classpath.clone(), SymbolTable::getAbsyn());
                    b = List::applyAndFold1(&paths, &fnptr!(boolOr, bool, bool), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathSuffixOfr(&__a0, &__a1)) }, baseClassPath.clone(), false)?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "extendsFrom", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isExperiment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: isExperiment(classpath.clone(), &(SymbolTable::getAbsyn())) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritedClasses", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    paths = Interactive::getInheritedClasses(classpath.clone())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritedClasses", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "getComponentsTest", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                            let mut genv: Interactive::GraphicEnvCache;
                            let mut absynClass: metamodelica::Ref<Absyn::Class>;
                            let mut valsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>>;
                            absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                            genv = Interactive::getClassEnv(SymbolTable::getAbsyn(), classpath.clone())?;
                            valsLst = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>> = metamodelica::nil();
                for mut c in (InteractiveUtil::getPublicComponentsInClass(&absynClass)).into_iter().cloned() {
                            let __x = getComponentInfo(&(c.clone()), genv.clone(), false)?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            valsLst = listAppend(({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Values::Value>>> = metamodelica::nil();
                for mut c in (InteractiveUtil::getProtectedComponentsInClass(&absynClass)).into_iter().cloned() {
                            let __x = getComponentInfo(&(c.clone()), genv.clone(), true)?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), valsLst.clone());
                            Ok(ValuesMake::makeArray(List::flatten(valsLst.clone())?))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentsTest", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getSimulationOptions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: startTime }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: tolerance }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: numberOfIntervals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: interval }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    let mut simOpt: InteractiveTypes::SimulationOptions;
                    let mut startTimeExp: metamodelica::Ref<DAE::Exp>;
                    let mut stopTimeExp: metamodelica::Ref<DAE::Exp>;
                    let mut toleranceExp: metamodelica::Ref<DAE::Exp>;
                    let mut intervalExp: metamodelica::Ref<DAE::Exp>;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut startTime = (*startTime).clone();
                    let mut stopTime = (*stopTime).clone();
                    let mut tolerance = (*tolerance).clone();
                    let mut numberOfIntervals = (*numberOfIntervals).clone();
                    let mut interval = (*interval).clone();
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath));
                    ErrorExt::setCheckpoint(literal!("getSimulationOptions"));
                    simOpt = InteractiveTypes::SimulationOptions { startTime: metamodelica::Ref::new(DAE::Exp::RCONST { real: startTime.clone() }), stopTime: metamodelica::Ref::new(DAE::Exp::RCONST { real: stopTime.clone() }), numberOfIntervals: metamodelica::Ref::new(DAE::Exp::ICONST { integer: numberOfIntervals.clone() }), stepSize: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), tolerance: metamodelica::Ref::new(DAE::Exp::RCONST { real: tolerance.clone() }), method: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), fileNamePrefix: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), options: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), outputFormat: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), variableFilter: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), cflags: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), simflags: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }) };
                    ErrorExt::rollBack(literal!("getSimulationOptions"));
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(StaticScript::getSimulationArguments(FCore::emptyCache(), FGraph::empty(), &(list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr.clone() })]), metamodelica::nil(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, literal!("getSimulationOptions"), Absyn::dummyInfo.clone(), Some(simOpt.clone()))?) {
                        (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: _ } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    startTimeExp = metamodelica::Own::own(__pa0);
                    stopTimeExp = metamodelica::Own::own(__pa1);
                    intervalExp = metamodelica::Own::own(__pa2);
                    toleranceExp = metamodelica::Own::own(__pa3);
                    startTime = ValuesUtil::valueReal(&(Util::makeValueOrDefault(&Ceval::cevalSimple, startTimeExp.clone(), metamodelica::Ref::new(Values::Value::REAL { real: startTime.clone() }))))?;
                    stopTime = ValuesUtil::valueReal(&(Util::makeValueOrDefault(&Ceval::cevalSimple, stopTimeExp.clone(), metamodelica::Ref::new(Values::Value::REAL { real: stopTime.clone() }))))?;
                    tolerance = ValuesUtil::valueReal(&(Util::makeValueOrDefault(&Ceval::cevalSimple, toleranceExp.clone(), metamodelica::Ref::new(Values::Value::REAL { real: tolerance.clone() }))))?;
                    let __pa5 = ::match_deref::match_deref! { match &(Util::makeValueOrDefault(&Ceval::cevalSimple, intervalExp.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }))) {
                        Deref @ Values::Value::INTEGER { integer: __pa5 } => __pa5.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    numberOfIntervals = metamodelica::Own::own(__pa5);
                    if numberOfIntervals.clone() == 0 && interval.clone() > metamodelica::OrderedFloat(0.0_f64) {
                        numberOfIntervals = (((metamodelica::real_div_checked((stopTime.clone() - startTime.clone()), interval.clone())?).ceil()).0.floor() as i32);
                    } else {
                        numberOfIntervals = std::cmp::max(numberOfIntervals.clone(), 1);
                        interval = metamodelica::real_div_checked((stopTime.clone() - startTime.clone()), metamodelica::OrderedFloat((numberOfIntervals.clone()) as f64))?;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: startTime.clone() }), metamodelica::Ref::new(Values::Value::REAL { real: stopTime.clone() }), metamodelica::Ref::new(Values::Value::REAL { real: tolerance.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }), metamodelica::Ref::new(Values::Value::REAL { real: interval.clone() })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelFigures", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(getModelFigures(classpath.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationNamedModifiers", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: annotationname }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(getAnnotationNamedModifiers(classpath.clone(), annotationname.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: annotationname }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: modifiername }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(getAnnotationModifierValue(classpath.clone(), annotationname.clone(), modifiername.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "searchClassNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    (_, paths) = ProgramUtil::getClassNamesRecursive(None, SymbolTable::getAbsyn(), false, false, metamodelica::nil())?;
                    paths = paths.clone().reverse();
                    vals = List::map(paths.clone(), &fnptr!(ValuesMake::makeCodeTypeName, metamodelica::Ref<Absyn::Path>))?;
                    vals = searchClassNames(&vals, r#str.clone(), b.clone(), SymbolTable::getAbsyn())?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableLibraries", Deref @ metamodelica::ListNode::Nil) => {
                    let mut files: metamodelica::List<ArcStr>;
                    PackageManagement::installCachedPackages()?;
                    files = PackageManagement::AvailableLibraries::listKeys(&(PackageManagement::getInstalledLibraries()?), metamodelica::nil());
                    Ok(ValuesMake::makeArray(List::map(files.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableLibraryVersions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut files: metamodelica::List<ArcStr>;
                    PackageManagement::installCachedPackages()?;
                    files = PackageManagement::getInstalledLibraryVersions(str1.clone())?;
                    Ok(ValuesMake::makeArray(List::map(files.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: PackageManagement::installPackage(str1.clone(), str2.clone(), b.clone(), false)? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: path @ Deref @ Absyn::Path::QUALIFIED { .. } } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Error::addMessage(Error::ERROR_PKG_NOT_IDENT.clone(), list![AbsynUtil::pathString(path.clone(), literal!("."), true, false)?])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updatePackageIndex", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: PackageManagement::updateIndex()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "upgradeInstalledPackages", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: PackageManagement::upgradeInstalledPackages(b.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "getAvailablePackageVersions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                            Ok(ValuesMake::makeArray(({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut s in (PackageManagement::versionsThatProvideTheWanted(str1.clone(), str2.clone(), true)).into_iter().cloned() {
                    let __x = ValuesMake::makeString(s.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageVersions", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsFrom", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeStringArray(PackageManagement::versionsThatConvertFromTheWanted(str1.clone(), str2.clone(), true))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsFrom", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsTo", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeStringArray(PackageManagement::versionsThatConvertToTheWanted(str1.clone(), str2.clone(), true))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsTo", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUses", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut uses: metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>;
                    let __arc1 = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    let __pa0 = (__arc1).clone();
                    let Absyn::CLASS { .. } = &*__arc1;
                    absynClass = metamodelica::Own::own(__pa0);
                    uses = Interactive::getUsesAnnotation(Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP })?;
                    Ok(ValuesMake::makeArray(List::map(uses.clone(), &move |__a0: (metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)| makeUsesArray(&__a0))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConversionsFromVersions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut withoutConversion: metamodelica::List<ArcStr>;
                    let mut withConversion: metamodelica::List<ArcStr>;
                    let __arc1 = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    let __pa0 = (__arc1).clone();
                    let Absyn::CLASS { .. } = &*__arc1;
                    absynClass = metamodelica::Own::own(__pa0);
                    (withoutConversion, withConversion) = Interactive::getConversionAnnotation(&absynClass);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(List::map(withoutConversion.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?), ValuesMake::makeArray(List::map(withConversion.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?)] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedClassModifierNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut args: metamodelica::List<ArcStr>;
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    args = Interactive::getDerivedClassModifierNames(&absynClass);
                    vals = List::map(args.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedClassModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = Interactive::getDerivedClassModifierValue(&absynClass, metamodelica::AsArg::as_arg(&path));
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResult", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: size }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::printCodeVariableName(&__a0))?;
                    filename = Util::absoluteOrRelative(filename.clone());
                    Ok(SimulationResults::readDataset(filename.clone(), vars_1.clone(), size.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResult", _) => {
                    Error::addMessage(Error::SCRIPT_READ_SIM_RES_ERROR.clone(), metamodelica::nil())?;
                    Ok(openmodelica_frontend_types::Values::Value::interned_META_FAIL())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResultSize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut i: i32;
                    let mut filename = (*filename).clone();
                    filename = Util::absoluteOrRelative(filename.clone());
                    i = SimulationResults::readSimulationResultSize(filename.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResultVars", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut args: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    filename = Util::absoluteOrRelative(filename.clone());
                    args = SimulationResults::readVariables(filename.clone(), b1.clone(), b2.clone())?;
                    vals = List::map(args.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareSimulationResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut strings: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    let mut filename2 = (*filename2).clone();
                    let mut cvars = (*cvars).clone();
                    Error::addMessage(Error::DEPRECATED_API_CALL.clone(), list![literal!("compareSimulationResults"), literal!("diffSimulationResults")])?;
                    filename = Util::absoluteOrRelative(filename.clone());
                    filename_1 = Testsuite::friendlyPath(filename_1.clone());
                    filename_1 = Util::absoluteOrRelative(filename_1.clone());
                    filename2 = Util::absoluteOrRelative(filename2.clone());
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    strings = SimulationResults::cmpSimulationResults(Testsuite::isRunning()?, filename.clone(), filename_1.clone(), filename2.clone(), x1.clone(), x2.clone(), vars_1.clone())?;
                    cvars = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(cvars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareSimulationResults", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("Error in compareSimulationResults") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deltaSimulationResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: method_str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut val: metamodelica::Real;
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    filename = Util::absoluteOrRelative(filename.clone());
                    filename_1 = Testsuite::friendlyPath(filename_1.clone());
                    filename_1 = Util::absoluteOrRelative(filename_1.clone());
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    val = SimulationResults::deltaSimulationResults(filename.clone(), filename_1.clone(), method_str.clone(), vars_1.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: val }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deltaSimulationResults", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("Error in deltaSimulationResults") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "filterSimulationResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: numberOfIntervals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: hintReadAllVars }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut b = (*b).clone();
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    b = SimulationResults::filterSimulationResults(filename.clone(), filename_1.clone(), vars_1.clone(), numberOfIntervals.clone(), b.clone(), hintReadAllVars.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "filterSimulationResults", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: reltol }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: reltolDiffMinMax }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rangeDelta }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut v1: metamodelica::Ref<Values::Value>;
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut strings: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    let mut filename2 = (*filename2).clone();
                    let mut cvars = (*cvars).clone();
                    let mut b = (*b).clone();
                    filename = Util::absoluteOrRelative(filename.clone());
                    filename_1 = Testsuite::friendlyPath(filename_1.clone());
                    filename_1 = Util::absoluteOrRelative(filename_1.clone());
                    filename2 = Util::absoluteOrRelative(filename2.clone());
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    (b, strings) = SimulationResults::diffSimulationResults(Testsuite::isRunning()?, filename.clone(), filename_1.clone(), filename2.clone(), reltol.clone(), reltolDiffMinMax.clone(), rangeDelta.clone(), vars_1.clone(), b.clone())?;
                    cvars = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    v1 = ValuesMake::makeArray(cvars.clone());
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() }), v1.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResults", _) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    v = ValuesMake::makeArray(metamodelica::nil());
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), v.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResultsHtml", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: reltol }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: reltolDiffMinMax }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rangeDelta }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    let mut r#str = (*r#str).clone();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    filename = Util::absoluteOrRelative(filename.clone());
                    filename_1 = Testsuite::friendlyPath(filename_1.clone());
                    filename_1 = Util::absoluteOrRelative(filename_1.clone());
                    r#str = SimulationResults::diffSimulationResultsHtml(Testsuite::isRunning()?, filename.clone(), filename_1.clone(), reltol.clone(), reltolDiffMinMax.clone(), rangeDelta.clone(), r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResultsHtml", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkTaskGraph", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut pd: ArcStr;
                    let mut pwd: ArcStr;
                    let mut cvars: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut strings: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    pwd = System::pwd();
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    filename = if (StringUtil::startsWith(filename.clone(), literal!("/"))) {filename.clone()} else {stringAppendList(list![pwd.clone(), pd.clone(), filename.clone()])};
                    filename_1 = if (StringUtil::startsWith(filename_1.clone(), literal!("/"))) {filename_1.clone()} else {stringAppendList(list![pwd.clone(), pd.clone(), filename_1.clone()])};
                    strings = TaskGraphResults::checkTaskGraph(filename.clone(), filename_1.clone())?;
                    cvars = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(cvars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkTaskGraph", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("Error in checkTaskGraph") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkCodeGraph", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut pd: ArcStr;
                    let mut pwd: ArcStr;
                    let mut cvars: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut strings: metamodelica::List<ArcStr>;
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    pwd = System::pwd();
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    filename = if (StringUtil::startsWith(filename.clone(), literal!("/"))) {filename.clone()} else {stringAppendList(list![pwd.clone(), pd.clone(), filename.clone()])};
                    filename_1 = if (StringUtil::startsWith(filename_1.clone(), literal!("/"))) {filename_1.clone()} else {stringAppendList(list![pwd.clone(), pd.clone(), filename_1.clone()])};
                    strings = TaskGraphResults::checkCodeGraph(filename.clone(), filename_1.clone())?;
                    cvars = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(cvars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkCodeGraph", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("Error in checkCodeGraph") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotAll", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut pd: ArcStr;
                    let mut call: ArcStr;
                    let mut omhome: ArcStr;
                    let mut logXStr: ArcStr;
                    let mut logYStr: ArcStr;
                    let mut x1Str: ArcStr;
                    let mut x2Str: ArcStr;
                    let mut y1Str: ArcStr;
                    let mut y2Str: ArcStr;
                    let mut curveWidthStr: ArcStr;
                    let mut curveStyleStr: ArcStr;
                    let mut autoScaleStr: ArcStr;
                    let mut b: bool;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    omhome = Settings::getInstallationDirectoryPath()?;
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), filename.clone(), msg.clone())?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    str1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd); __mm_s.push_str(&*filename); ArcStr::from(__mm_s) };
                    s1 = if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {literal!(".exe")} else {literal!("")};
                    filename = if (System::regularFileExists(str1.clone())) {str1.clone()} else {filename.clone()};
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b)) {
                        str2 = stringAppendList(list![omhome.clone(), pd.clone(), literal!("bin"), pd.clone(), literal!("OMPlot"), s1.clone()]);
                        str3 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr); __mm_s.push_str(&*literal!(" --plotAll --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); ArcStr::from(__mm_s) };
                        call = stringAppendList(list![literal!("\""), str2.clone(), literal!("\""), literal!(" "), str3.clone()]);
                        let 0 = (System::spawnCall(str2.clone(), call.clone())) else { return Err("pattern mismatch") };
                    } else if b {
                        logXStr = boolString(logX.clone());
                        logYStr = boolString(logY.clone());
                        x1Str = realString(x1.clone());
                        x2Str = realString(x2.clone());
                        y1Str = realString(y1.clone());
                        y2Str = realString(y2.clone());
                        curveWidthStr = realString(curveWidth.clone());
                        curveStyleStr = intString(curveStyle.clone());
                        autoScaleStr = boolString(autoScale.clone());
                        System::plotCallBack(externalWindow.clone(), filename.clone(), title.clone(), gridStr.clone(), literal!("plotall"), logXStr.clone(), logYStr.clone(), xLabel.clone(), yLabel.clone(), x1Str.clone(), x2Str.clone(), y1Str.clone(), y2Str.clone(), curveWidthStr.clone(), curveStyleStr.clone(), legendPosition.clone(), footer.clone(), autoScaleStr.clone(), literal!(""));
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotAll", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plot", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut pd: ArcStr;
                    let mut call: ArcStr;
                    let mut omhome: ArcStr;
                    let mut logXStr: ArcStr;
                    let mut logYStr: ArcStr;
                    let mut x1Str: ArcStr;
                    let mut x2Str: ArcStr;
                    let mut y1Str: ArcStr;
                    let mut y2Str: ArcStr;
                    let mut curveWidthStr: ArcStr;
                    let mut curveStyleStr: ArcStr;
                    let mut autoScaleStr: ArcStr;
                    let mut vars_1: metamodelica::List<ArcStr>;
                    let mut b: bool;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    vars_1 = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::printCodeVariableName(&__a0))?;
                    omhome = Settings::getInstallationDirectoryPath()?;
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), filename.clone(), msg.clone())?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    str1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd); __mm_s.push_str(&*filename); ArcStr::from(__mm_s) };
                    s1 = if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {literal!(".exe")} else {literal!("")};
                    filename = if (System::regularFileExists(str1.clone())) {str1.clone()} else {filename.clone()};
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b)) {
                        r#str = stringDelimitList(vars_1.clone(), literal!("' '"));
                        str2 = stringAppendList(list![omhome.clone(), pd.clone(), literal!("bin"), pd.clone(), literal!("OMPlot"), s1.clone()]);
                        str3 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr); __mm_s.push_str(&*literal!(" --plot --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); __mm_s.push_str(&*literal!(" '")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) };
                        call = stringAppendList(list![literal!("\""), str2.clone(), literal!("\""), literal!(" "), str3.clone()]);
                        let 0 = (System::spawnCall(str2.clone(), call.clone())) else { return Err("pattern mismatch") };
                    } else if b {
                        logXStr = boolString(logX.clone());
                        logYStr = boolString(logY.clone());
                        x1Str = realString(x1.clone());
                        x2Str = realString(x2.clone());
                        y1Str = realString(y1.clone());
                        y2Str = realString(y2.clone());
                        curveWidthStr = realString(curveWidth.clone());
                        curveStyleStr = intString(curveStyle.clone());
                        autoScaleStr = boolString(autoScale.clone());
                        r#str = stringDelimitList(vars_1.clone(), literal!(" "));
                        System::plotCallBack(externalWindow.clone(), filename.clone(), title.clone(), gridStr.clone(), literal!("plot"), logXStr.clone(), logYStr.clone(), xLabel.clone(), yLabel.clone(), x1Str.clone(), x2Str.clone(), y1Str.clone(), y2Str.clone(), curveWidthStr.clone(), curveStyleStr.clone(), legendPosition.clone(), footer.clone(), autoScaleStr.clone(), r#str.clone());
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plot", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "val", Deref @ metamodelica::ListNode::Cons { head: cvar, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: timeStamp }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "<default>" }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut filename: ArcStr;
                    let mut varNameStr: ArcStr;
                    let mut val: metamodelica::Real;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(outCache.clone(), inEnv.clone(), buildCurrentSimulationResultExp()?, true, msg.clone(), 0)?) {
                        (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    outCache = metamodelica::Own::own(__pa0);
                    filename = metamodelica::Own::own(__pa1);
                    varNameStr = ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar))?;
                    val = SimulationResults::val(filename.clone(), varNameStr.clone(), timeStamp.clone())?;
                    Ok((metamodelica::Ref::new(Values::Value::REAL { real: val }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "val", Deref @ metamodelica::ListNode::Cons { head: cvar, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: timeStamp }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut varNameStr: ArcStr;
                    let mut val: metamodelica::Real;
                    let false = (stringEq(&filename, &(literal!("<default>")))) else { return Err("pattern mismatch") };
                    varNameStr = ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar))?;
                    val = SimulationResults::val(filename.clone(), varNameStr.clone(), timeStamp.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: val }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "closeSimulationResultFile", _) => {
                    SimulationResults::close();
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getParameterNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut strings: metamodelica::List<ArcStr>;
                    strings = Interactive::getParameterNames(path.clone(), SymbolTable::getAbsyn());
                    vals = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getParameterValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut str2: ArcStr;
                    str2 = Interactive::getComponentBinding(path.clone(), metamodelica::AsArg::as_arg(&str1), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: str2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setParameterValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::setElementModifier(classpath.clone(), metamodelica::AsArg::as_arg(&path), &(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: aexp.clone(), info: Absyn::dummyInfo.clone() }) })), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut strings: metamodelica::List<ArcStr>;
                    strings = Interactive::getComponentModifierNames(path.clone(), str1.clone(), SymbolTable::getAbsyn());
                    vals = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    if AbsynUtil::crefIsIdent(&cr) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        s1 = metamodelica::Own::own(__pa0);
                        r#str = Interactive::getComponentBinding(classpath.clone(), &s1, &(SymbolTable::getAbsyn()));
                    } else {
                        s1 = AbsynUtil::crefFirstIdent(&cr)?;
                        cr = AbsynUtil::crefStripFirst(&cr)?;
                        r#str = Interactive::getComponentModifierValue(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath))), &(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: s1.clone(), subscripts: metamodelica::nil() })), &cr, &(SymbolTable::getAbsyn()));
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierValues", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    if AbsynUtil::crefIsIdent(&cr) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        s1 = metamodelica::Own::own(__pa0);
                        r#str = Interactive::getComponentBinding(classpath.clone(), &s1, &(SymbolTable::getAbsyn()));
                    } else {
                        s1 = AbsynUtil::crefFirstIdent(&cr)?;
                        cr = AbsynUtil::crefStripFirst(&cr)?;
                        r#str = Interactive::getComponentModifierValues(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: s1.clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn());
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::setElementModifier(classpath.clone(), metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&r#mod), SymbolTable::getAbsyn());
                    if b {
                        SymbolTable::setAbsynClass(p.clone(), ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?, metamodelica::AsArg::as_arg(&classpath))?;
                    } else {
                        SymbolTable::setAbsyn(p.clone())?;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExtendsModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(Interactive::getExtendsModifierValue(classpath.clone(), metamodelica::AsArg::as_arg(&baseClassPath), metamodelica::AsArg::as_arg(&path), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setExtendsModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::setExtendsModifier(classpath.clone(), metamodelica::AsArg::as_arg(&baseClassPath), metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&r#mod), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setExtendsModifier", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::setExtendsModifier(classpath.clone(), metamodelica::AsArg::as_arg(&baseClassPath), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("_") })), metamodelica::AsArg::as_arg(&r#mod), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isExtendsModifierFinal", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(Interactive::isExtendsModifierFinal(classpath.clone(), metamodelica::AsArg::as_arg(&baseClassPath), path.clone(), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeComponentModifiers", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::removeComponentModifiers(path.clone(), metamodelica::AsArg::as_arg(&str1), SymbolTable::getAbsyn(), keepRedeclares.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut strings: metamodelica::List<ArcStr>;
                    strings = InteractiveUtil::getElementModifierNames(path.clone(), str1.clone(), SymbolTable::getAbsyn());
                    vals = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierValue", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    if AbsynUtil::crefIsIdent(&cr) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        s1 = metamodelica::Own::own(__pa0);
                        r#str = InteractiveUtil::getElementBinding(classpath.clone(), &s1, &(SymbolTable::getAbsyn()));
                    } else {
                        s1 = AbsynUtil::crefFirstIdent(&cr)?;
                        cr = AbsynUtil::crefStripFirst(&cr)?;
                        r#str = InteractiveUtil::getElementModifierValue(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath))), &(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: s1.clone(), subscripts: metamodelica::nil() })), &cr, &(SymbolTable::getAbsyn()));
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierValues", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    cr = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    if AbsynUtil::crefIsIdent(&cr) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        s1 = metamodelica::Own::own(__pa0);
                        r#str = InteractiveUtil::getElementBinding(classpath.clone(), &s1, &(SymbolTable::getAbsyn()));
                    } else {
                        s1 = AbsynUtil::crefFirstIdent(&cr)?;
                        cr = AbsynUtil::crefStripFirst(&cr)?;
                        r#str = InteractiveUtil::getElementModifierValues(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath)), metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: s1.clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn());
                    }
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeElementModifiers", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::removeElementModifiers(path.clone(), metamodelica::AsArg::as_arg(&str1), SymbolTable::getAbsyn(), keepRedeclares.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeExtendsModifiers", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::removeExtendsModifiers(classpath.clone(), baseClassPath.clone(), SymbolTable::getAbsyn(), keepRedeclares.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstantiatedParametersAndValues", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut odae: Option<DAE::DAElist>;
                    let mut strings: metamodelica::List<ArcStr>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _, odae, _) = runFrontEnd(outCache.clone(), inEnv.clone(), classpath.clone(), true, false, false)?;
                    strings = Interactive::getInstantiatedParametersAndValues(odae.clone())?;
                    vals = List::map(strings.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?;
                    Ok((ValuesMake::makeArray(vals.clone()), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstantiatedParametersAndValues", _) => {
                    Error::addCompilerWarning(literal!("getInstantiatedParametersAndValues failed to instantiate the model."))?;
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    p = InteractiveUtil::updateConnectionAnnotation(&(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&classpath))), str1.clone(), str2.clone(), &(metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("annotate"), argValue: aexp.clone() }), metamodelica::nil())), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: annlst, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    p = SymbolTable::getAbsyn();
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
                    absynClass = InteractiveUtil::updateConnectionAnnotationInClass(absynClass.clone(), str1.clone(), str2.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = ProgramUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(metamodelica::AsArg::as_arg(&classpath))) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&classpath))? }} }, p.clone(), false, false)?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: annStr }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut aexp: metamodelica::Ref<Absyn::Exp>;
                    let mut istmts: GlobalScript::Statements;
                    let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut annlst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    istmts = Parser::parsestringexp({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("__dummy(")); __mm_s.push_str(&*annStr); __mm_s.push_str(&*literal!(");")); ArcStr::from(__mm_s) }, literal!("<interactive>"))?;
                    let __pa0 = ::match_deref::match_deref! { match &(istmts.clone()) {
                        GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    aexp = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(aexp.clone()) {
                        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: __pa2, .. }, .. } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    nargs = metamodelica::Own::own(__pa2);
                    let __pa4 = ::match_deref::match_deref! { match &((nargs).head().cloned()?) {
                        Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { elementArgLst: __pa4, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } } }, .. } => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    annlst = metamodelica::Own::own(__pa4);
                    p = SymbolTable::getAbsyn();
                    absynClass = ProgramUtil::getPathedClassInProgram(classpath.clone(), &p, false, false)?;
                    absynClass = InteractiveUtil::updateConnectionAnnotationInClass(absynClass.clone(), str1.clone(), str2.clone(), metamodelica::Ref::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = ProgramUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(metamodelica::AsArg::as_arg(&classpath))) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&classpath))? }} }, p.clone(), false, false)?;
                    SymbolTable::setAbsynClass(p.clone(), absynClass.clone(), metamodelica::AsArg::as_arg(&classpath))?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionAnnotation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (b, p) = InteractiveUtil::updateConnectionNames(classpath.clone(), str1.clone(), str2.clone(), str3.clone(), str4.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionNames", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    let mut access: Access;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access >= Access::diagram.clone() {
                        n = (((Interactive::getConnections(&absynClass))).len() as i32);
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        n = 0;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut access: Access;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access >= Access::diagram.clone() {
                        vals = Interactive::getNthConnection(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path)), SymbolTable::getAbsyn(), n.clone());
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        vals = metamodelica::nil();
                    }
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnection", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionList", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(getConnectionList(path.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = (((getAlgorithms(&absynClass)?)).len() as i32);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithm", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthAlgorithm(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithm", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = (((getInitialAlgorithms(&absynClass)?)).len() as i32);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithm", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthInitialAlgorithm(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithm", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmItemsCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getAlgorithmItemsCount(&absynClass)?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmItemsCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithmItem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthAlgorithmItem(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithmItem", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmItemsCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getInitialAlgorithmItemsCount(&absynClass)?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmItemsCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithmItem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthInitialAlgorithmItem(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithmItem", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = (((getEquations(&absynClass)?)).len() as i32);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthEquation(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = (((getInitialEquations(&absynClass)?)).len() as i32);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthInitialEquation(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationItemsCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getEquationItemsCount(&absynClass)?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationItemsCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquationItem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthEquationItem(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquationItem", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationItemsCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getInitialEquationItemsCount(&absynClass)?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationItemsCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquationItem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthInitialEquationItem(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquationItem", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getAnnotationCount(&absynClass)?;
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAnnotationString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut r#str: ArcStr;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    r#str = getNthAnnotationString(&absynClass, n.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAnnotationString", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut n: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    n = getImportCount(&absynClass);
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: n }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportCount", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthImport", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    absynClass = ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?;
                    vals = getNthImport(&absynClass, n.clone())?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthImport", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotParametric", Deref @ metamodelica::ListNode::Cons { head: cvar, tail: Deref @ metamodelica::ListNode::Cons { head: cvar2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut pd: ArcStr;
                    let mut call: ArcStr;
                    let mut omhome: ArcStr;
                    let mut logXStr: ArcStr;
                    let mut logYStr: ArcStr;
                    let mut x1Str: ArcStr;
                    let mut x2Str: ArcStr;
                    let mut y1Str: ArcStr;
                    let mut y2Str: ArcStr;
                    let mut curveWidthStr: ArcStr;
                    let mut curveStyleStr: ArcStr;
                    let mut autoScaleStr: ArcStr;
                    let mut b: bool;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    omhome = Settings::getInstallationDirectoryPath()?;
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), filename.clone(), msg.clone())?;
                    pd = arcstr::literal!(Autoconf::pathDelimiter);
                    str1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd); __mm_s.push_str(&*filename); ArcStr::from(__mm_s) };
                    s1 = if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {literal!(".exe")} else {literal!("")};
                    filename = if (System::regularFileExists(str1.clone())) {str1.clone()} else {filename.clone()};
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b)) {
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar))?); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar2))?); ArcStr::from(__mm_s) };
                        str2 = stringAppendList(list![omhome.clone(), pd.clone(), literal!("bin"), pd.clone(), literal!("OMPlot"), s1.clone()]);
                        str3 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr); __mm_s.push_str(&*literal!(" --plotParametric --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
                        call = stringAppendList(list![literal!("\""), str2.clone(), literal!("\""), literal!(" "), str3.clone()]);
                        let 0 = (System::spawnCall(str2.clone(), call.clone())) else { return Err("pattern mismatch") };
                    } else if b {
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(metamodelica::AsArg::as_arg(&cvar2))?); ArcStr::from(__mm_s) };
                        logXStr = boolString(logX.clone());
                        logYStr = boolString(logY.clone());
                        x1Str = realString(x1.clone());
                        x2Str = realString(x2.clone());
                        y1Str = realString(y1.clone());
                        y2Str = realString(y2.clone());
                        curveWidthStr = realString(curveWidth.clone());
                        curveStyleStr = intString(curveStyle.clone());
                        autoScaleStr = boolString(autoScale.clone());
                        System::plotCallBack(externalWindow.clone(), filename.clone(), title.clone(), gridStr.clone(), literal!("plotparametric"), logXStr.clone(), logYStr.clone(), xLabel.clone(), yLabel.clone(), x1Str.clone(), x2Str.clone(), y1Str.clone(), y2Str.clone(), curveWidthStr.clone(), curveStyleStr.clone(), legendPosition.clone(), footer.clone(), autoScaleStr.clone(), r#str.clone());
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotParametric", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dumpXMLDAE", vals) => {
                    let mut xml_filename: ArcStr;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, xml_filename) = dumpXMLDAE(outCache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&vals), &msg)?;
                    Ok((ValuesMake::makeTuple(list![metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), metamodelica::Ref::new(Values::Value::STRING { string: xml_filename.clone() })]), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dumpXMLDAE", _) => {
                    Ok(ValuesMake::makeTuple(list![metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "solveLinearSystem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut i: i32;
                    let mut realVals: metamodelica::List<metamodelica::Real>;
                    let mut v = (*v).clone();
                    (realVals, i) = System::dgesv(List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::arrayValueReals(&__a0))?, ValuesUtil::arrayValueReals(metamodelica::AsArg::as_arg(&v))?)?;
                    v = ValuesMake::makeArray(List::map(realVals.clone(), &fnptr!(ValuesMake::makeReal, metamodelica::Real))?);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: i })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "solveLinearSystem", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Unknown input to solveLinearSystem scripting function")])?;
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![v.clone(), metamodelica::Ref::new(Values::Value::INTEGER { integer: -1 })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "relocateFunctions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut b: bool;
                    let mut relocatableFunctionsTuple: metamodelica::List<(ArcStr, ArcStr)>;
                    relocatableFunctionsTuple = metamodelica::nil();
                    for mut varr in &*var_field!((**v).valueLst, Values::Value::ARRAY).clone() {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(varr.clone()) {
                            Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        s1 = metamodelica::Own::own(__pa0);
                        s2 = metamodelica::Own::own(__pa1);
                        relocatableFunctionsTuple = metamodelica::cons((s1.clone(), s2.clone()), relocatableFunctionsTuple.clone());
                    }
                    b = System::relocateFunctions(r#str.clone(), relocatableFunctionsTuple.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "toJulia", Deref @ metamodelica::ListNode::Nil) => {
                    let mut r#str: ArcStr;
                    r#str = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Program| AbsynToJulia::dumpProgram(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "interactiveDumpAbsynToJL", Deref @ metamodelica::ListNode::Nil) => {
                    let mut r#str: ArcStr;
                    r#str = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Program| AbsynJLDumpTpl::dump(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "relocateFunctions", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runConversionScript", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(runConversionScript(path.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertPackageToLibrary", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(convertPackageToLibrary(classpath.clone(), path.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstance", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(NFApi::getModelInstance(classpath.clone(), path.clone(), metamodelica::AsArg::as_arg(&r#str), b.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstanceAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(NFApi::getModelInstanceAnnotation(classpath.clone(), &(ValuesUtil::arrayValueStrings(metamodelica::AsArg::as_arg(&v))?), b.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstanceReference", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(NFApi::getModelInstanceReference(classpath.clone(), path.clone(), metamodelica::AsArg::as_arg(&r#str))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstanceAnnotationReference", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(NFApi::getModelInstanceAnnotationReference(classpath.clone(), &(ValuesUtil::arrayValueStrings(metamodelica::AsArg::as_arg(&v))?))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "releaseModelInstanceReference", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(NFApi::releaseModelInstanceReference(i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modifierToJSON", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(NFApi::modifierToJSON(metamodelica::AsArg::as_arg(&r#str), b.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "storeAST", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: SymbolTable::storeAST()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "restoreAST", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: SymbolTable::restoreAST(n.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "qualifyPath", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeCodeTypeName(NFApi::mkFullyQual(SymbolTable::getAbsyn(), classpath.clone(), path.clone(), false)?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: InteractiveUtil::getElementAnnotation(path.clone(), &(SymbolTable::getAbsyn())) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    (_, b) = InteractiveUtil::setElementAnnotation(metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&r#mod), SymbolTable::getAbsyn());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadClassContentString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: x }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: y }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = InteractiveUtil::loadClassContentString(r#str.clone(), metamodelica::AsArg::as_arg(&classpath), x.clone(), y.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementType", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    (_, b) = InteractiveUtil::setElementType(metamodelica::AsArg::as_arg(&path), cr.clone(), SymbolTable::getAbsyn());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExtendsModifierNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(InteractiveUtil::getExtendsModifierNames(classpath.clone(), metamodelica::AsArg::as_arg(&path), b.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPrimitive", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isPrimitive(classpath.clone(), SymbolTable::getAbsyn())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isParameter", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isParameter(path.clone(), classpath.clone(), &(SymbolTable::getAbsyn()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isConstant", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isConstant(path.clone(), classpath.clone(), &(SymbolTable::getAbsyn()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isProtected", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isProtected(metamodelica::AsArg::as_arg(&path), classpath.clone(), &(SymbolTable::getAbsyn()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setComponentDimensions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp @ Deref @ Absyn::Exp::ARRAY { .. } } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::setComponentDimensions(classpath.clone(), metamodelica::AsArg::as_arg(&path), var_field!((**aexp).arrayExp, Absyn::Exp::ARRAY), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "setComponentProperties", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                            let mut p: Absyn::Program;
                            let mut v: metamodelica::Ref<Values::Value>;
                            (p, v) = Interactive::setComponentProperties(metamodelica::AsArg::as_arg(&classpath), metamodelica::AsArg::as_arg(&name), &(({
                let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                for mut va in (vals.clone()).into_iter().cloned() {
                            let __x = ValuesUtil::valueBool(&(va.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), metamodelica::AsArg::as_arg(&s1), b1.clone(), b2.clone(), metamodelica::AsArg::as_arg(&s2), SymbolTable::getAbsyn());
                            SymbolTable::setAbsyn(p.clone())?;
                            Ok(v.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "createModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut p: Absyn::Program;
                    p = Interactive::createModel(metamodelica::AsArg::as_arg(&classpath), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "newModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    p = Interactive::newModel(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (b, p) = Interactive::deleteClass(classpath.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsynDeleted(p.clone(), metamodelica::AsArg::as_arg(&classpath))?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addComponent", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp3 } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::addComponent(name.clone(), path.clone(), classpath.clone(), aexp.clone(), r#mod.clone(), metamodelica::AsArg::as_arg(&aexp2), metamodelica::AsArg::as_arg(&aexp3), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateComponent", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp3 } }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::updateComponent(name.clone(), path.clone(), classpath.clone(), aexp.clone(), r#mod.clone(), metamodelica::AsArg::as_arg(&aexp2), metamodelica::AsArg::as_arg(&aexp3), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteComponent", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::deleteComponent(name.clone(), classpath.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeInteger(Interactive::getComponentCount(classpath.clone(), &(SymbolTable::getAbsyn()))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponent", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthComponent(classpath.clone(), &(SymbolTable::getAbsyn()), n.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponents", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getComponents(classpath.clone(), b.clone(), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElements", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getElements(classpath.clone(), b.clone(), SymbolTable::getAbsyn(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementsInfo", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getElementsInfo(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentAnnotations", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getComponentAnnotations(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementAnnotations", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getElementAnnotations(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthComponentAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentModification", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthComponentModification(classpath.clone(), n.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentCondition", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthComponentCondition(classpath.clone(), n.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritanceCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getInheritanceCount(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(NFApi::getNthInheritedClass(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setConnectionComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::setConnectionComment(metamodelica::AsArg::as_arg(&classpath), metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr2), metamodelica::AsArg::as_arg(&r#str), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::addConnection(metamodelica::AsArg::as_arg(&classpath), cr.clone(), cr2.clone(), metamodelica::AsArg::as_arg(&aexp), metamodelica::AsArg::as_arg(&aexp2), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::deleteConnection(metamodelica::AsArg::as_arg(&classpath), metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr2), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnectionAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthConnectionAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectorCount", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getConnectorCount(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnector", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthConnector(classpath.clone(), n.clone(), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnectorIconAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthConnectorIconAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getIconAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getIconAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDiagramAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getDiagramAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorIconAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::refactorIconAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorDiagramAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::refactorDiagramAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::refactorClass(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClassIconMapAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthInheritedClassIconMapAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClassDiagramMapAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNthInheritedClassDiagramMapAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNamedAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getNamedAnnotation(classpath.clone(), metamodelica::AsArg::as_arg(&path), SymbolTable::getAbsyn())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getShortDefinitionBaseClassInformation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getShortDefinitionBaseClassInformation(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExternalFunctionSpecification", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getExternalFunctionSpecification(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEnumerationLiterals", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getEnumerationLiterals(classpath.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "existClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::existClass(classpath.clone(), &(SymbolTable::getAbsyn()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(Interactive::getComponentComment(classpath.clone(), path.clone(), &(SymbolTable::getAbsyn()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setComponentComment", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut b: bool;
                    (p, b) = Interactive::setComponentComment(classpath.clone(), path.clone(), metamodelica::AsArg::as_arg(&r#str), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut v: metamodelica::Ref<Values::Value>;
                    (p, v) = Interactive::renameClass(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameComponent", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut v: metamodelica::Ref<Values::Value>;
                    (p, v) = Interactive::renameComponent(classpath.clone(), cr.clone(), cr2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameComponentInClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut p: Absyn::Program;
                    let mut v: metamodelica::Ref<Values::Value>;
                    (p, v) = Interactive::renameComponentOnlyInClass(classpath.clone(), cr.clone(), cr2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCrefInfo", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getCrefInfo(classpath.clone(), &(SymbolTable::getAbsyn())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultComponentName", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getDefaultComponentName(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultComponentPrefixes", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getDefaultComponentPrefixes(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefinitions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(Interactive::getDefinitions(SymbolTable::getAbsyn(), b.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultOpenCLDevice", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(ValuesMake::makeInteger(Config::getDefaultOpenCLDevice()?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefUseChains", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(ValuesMake::makeString(getDefUseChains(path.clone(), r#str.clone(), classpath.clone(), b.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDependencyGraph", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(ValuesMake::makeString(getDependencyGraph(classpath.clone(), r#str.clone(), b.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefinitionAt", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: x }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: y }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(ValuesMake::makeString(getDefinitionAt(r#str.clone(), x.clone(), y.clone(), b.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassDiagram", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
                    Ok(ValuesMake::makeString(getClassDiagram(classpath.clone(), r#str.clone(), metamodelica::AsArg::as_arg(&str1), i.clone(), ValuesUtil::arrayValueStrings(metamodelica::AsArg::as_arg(&v))?, b.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reverseLookup", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(ValuesMake::makeString(ReverseLookup::lookup(path.clone(), classpath.clone(), &(SymbolTable::getAbsyn()), b1.clone(), b2.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateResidualsDAE", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(NFApi::translateResidualsDAE(path.clone(), s1.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addEquation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::addEquation(metamodelica::AsArg::as_arg(&path), s1.clone(), b1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateEquation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::updateEquation(metamodelica::AsArg::as_arg(&path), s1.clone(), s2.clone(), b.clone(), b1.clone(), b2.clone(), b3.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn getSimulationExtension(mut inString: &ArcStr, mut inString2: &ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &((inString.clone(), inString2.clone())) {
        (Deref @ "C", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "C", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "Unix") => literal!(".sh"),
        (Deref @ "omsicpp", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "omsicpp", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "omsicpp", Deref @ "Unix") => literal!(".sh"),
        _ => arcstr::literal!(Autoconf::exeExt),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub(crate) fn getAdjacencyMatrix(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inMsg: &Absyn::Msg,
    mut filenameprefix: ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>, ArcStr)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut outString: ArcStr;
    (outCache, outValue, outString) = (match (inCache, inEnv) {
        (mut cache, mut env) => {
            let mut filename: ArcStr;
            let mut file_dir: ArcStr;
            let mut r#str: ArcStr;
            let mut dae: DAE::DAElist;
            let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
            let mut a_cref: metamodelica::Ref<Absyn::ComponentRef>;
            let mut flatModelicaStr: ArcStr;
            let mut description: ArcStr;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(cache, env, className.clone(), true, false, true)?) {
                (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            env = metamodelica::Own::own(__pa1);
            dae = metamodelica::Own::own(__pa2);
            description = DAEUtil::daeDescription(&dae);
            a_cref = AbsynUtil::pathToCref(&className);
            file_dir = ProgramUtil::getFileDir(a_cref, SymbolTable::getAbsyn());
            dlow = BackendDAECreate::lower(
                dae.clone(),
                cache.clone(),
                env,
                BackendDAE::ExtraInfo {
                    description: description,
                    fileNamePrefix: filenameprefix.clone(),
                    simflags: None,
                },
            )?;
            dlow = FindZeroCrossings::findZeroCrossings(&dlow)?;
            flatModelicaStr = DAEDump::dumpStr(dae, &(FCore::getFunctionTree(&cache)))?;
            flatModelicaStr = stringAppend(literal!("OldEqStr={'"), flatModelicaStr);
            flatModelicaStr = System::stringReplace(flatModelicaStr, literal!("\n"), literal!("%##%"))?;
            flatModelicaStr = System::stringReplace(flatModelicaStr, literal!("%##%"), literal!("','"))?;
            flatModelicaStr = stringAppend(flatModelicaStr, literal!("'};"));
            filename = DAEQuery::writeAdjacencyMatrix(&dlow, filenameprefix, flatModelicaStr)?;
            r#str = stringAppend(literal!("The equation system was dumped to Matlab file:"), filename);
            (
                cache,
                metamodelica::Ref::new(Values::Value::STRING { string: r#str }),
                file_dir,
            )
        }
    });
    Ok((outCache, outValue, outString))
}

/* -------------------------------------------------------------------
                      RUN FRONTEND
------------------------------------------------------------------- */
pub(crate) fn runFrontEnd(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut relaxedFrontEnd: bool,
    mut dumpFlat: bool,
    mut transform: bool,
) -> Result<(FCore::Cache, FCore::Graph, Option<DAE::DAElist>, ArcStr)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut odae: Option<DAE::DAElist> = None;
    let mut flatString: ArcStr = literal!("");
    let mut dae: DAE::DAElist;
    let mut b: bool;
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    if '__try0: {
        b = unwrap_break_err!(loadProgram(&className), '__try0);
        let true = (b) else { break '__try0 Err::<_, _>("pattern mismatch") };
        if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try0) {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), literal!("GC stats before front-end:"), literal!("\n  "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
        }
        unwrap_break_err!(ExecStat::execStat(&(literal!("FrontEnd - loaded program"))), '__try0);
        (cache, env, dae, flatString) = unwrap_break_err!(runFrontEndWork(cache.clone(), env.clone(), className.clone(), relaxedFrontEnd, dumpFlat), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try0) {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*GCExt::profStatsStr(GCExt::getProfStats(), literal!("GC stats after front-end:"), literal!("\n  "))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
        }
        unwrap_break_err!(ExecStat::execStat(&(literal!("FrontEnd - DAE generated"))), '__try0);
        if transform {
            dae = unwrap_break_err!(DAEUtil::transformationsBeforeBackend(cache.clone(), env.clone(), dae.clone(), &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: DAE::DAElist| StateMachineFlatten::stateMachineToDataFlow(&__a0, &__a1, __a2)), '__try0);
        }
        odae = Some(dae.clone());
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), false)?;
    Ok((cache, env, odae, flatString))
}

pub(crate) fn runFrontEndNF(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut relaxedFrontEnd: bool,
    mut dumpFlat: bool,
) -> Result<(
    metamodelica::Ref<NFFlatModel::NFFlatModel>,
    metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
    ArcStr,
)> {
    let mut flatModel: metamodelica::Ref<NFFlatModel::NFFlatModel>;
    let mut functions: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>;
    let mut flatString: ArcStr;
    let true = (loadProgram(&className)?) else {
        return Err("pattern mismatch");
    };
    (flatModel, functions, flatString) = runFrontEndWorkNF(className, relaxedFrontEnd, dumpFlat)?;
    Ok((flatModel, functions, flatString))
}

fn loadProgram(mut className: &metamodelica::Ref<Absyn::Path>) -> Result<bool> {
    let mut success: bool;
    let mut lib_name: ArcStr;
    let mut p: Absyn::Program;
    let mut b: bool;
    p = SymbolTable::getAbsyn();
    lib_name = AbsynUtil::pathFirstIdent(className);
    match '__try0: {
        unwrap_break_err!(ProgramUtil::getClassInProgram(&lib_name, &p), '__try0);
        success = true;
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            (p, b) = CevalScript::loadModel(
                &(list![(
                    metamodelica::Ref::new(Absyn::Path::IDENT { name: lib_name.clone() }),
                    literal!("the given model name to instantiate"),
                    list![literal!("default")],
                    false
                )]),
                Settings::getModelicaPath(Testsuite::isRunning()?)?,
                p.clone(),
                true,
                true,
                true,
                false,
                false,
                literal!(""),
            )?;
            Error::assertionOrAddSourceMessage(
                !(b),
                &(Error::NOTIFY_IMPLICIT_LOAD.clone()),
                list![lib_name.clone(), literal!("default")],
                &(Absyn::dummyInfo.clone()),
            )?;
            System::loadModelCallBack(lib_name.clone());
            SymbolTable::setAbsyn(p.clone())?;
            SymbolTable::clearSCode();
            success = true;
        }
    }
    Ok(success)
}

fn runFrontEndWork(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut relaxedFrontEnd: bool,
    mut dumpFlat: bool,
) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist, ArcStr)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut flatString: ArcStr = literal!("");
    let mut numError: i32 = Error::getNumErrorMessages();
    let mut graph_inst: bool;
    let mut nf_inst: bool;
    let mut nf_inst_actual: bool;
    let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    let mut flat_model: metamodelica::Ref<NFFlatModel::NFFlatModel> =
        <metamodelica::Ref<NFFlatModel::NFFlatModel> as ::std::default::Default>::default();
    let mut nf_funcs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree> =
        metamodelica::Ref::new(NFFlatten::FunctionTreeImpl::Tree::EMPTY);
    graph_inst = Flags::isSet(Flags::GRAPH_INST.clone())?;
    nf_inst = Flags::isSet(Flags::SCODE_INST.clone())?;
    nf_inst_actual = nf_inst;
    if nf_inst && Flags::getConfigEnum(Flags::GRAMMAR.clone())? == Flags::PDEMODELICA.clone() {
        nf_inst = false;
        FlagsUtil::set(Flags::SCODE_INST.clone(), false)?;
        Error::addMessage(Error::NF_PDE_NOT_IMPLEMENTED.clone(), metamodelica::nil())?;
    }
    (cache, env, dae) = 'mc: {
        let __mc_input = (graph_inst, nf_inst);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            let (false, true) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cache: FCore::Cache = cache.clone();
            let mut dae: DAE::DAElist = dae.clone();
            let mut env: FCore::Graph = env.clone();
            let mut flatString: ArcStr = flatString.clone();
            let mut flat_model: metamodelica::Ref<NFFlatModel::NFFlatModel> = flat_model.clone();
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree> = funcs.clone();
            let mut nf_funcs: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree> = nf_funcs.clone();
            (flat_model, nf_funcs, flatString) = runFrontEndWorkNF(className.clone(), relaxedFrontEnd, dumpFlat)?;
            (dae, funcs) = NFConvertDAE::convert(&flat_model, &nf_funcs)?;
            cache = FCore::emptyCache();
            FCore::setCachedFunctionTree(&cache, funcs.clone());
            env = FGraph::new(literal!("graph"), FCore::dummyTopModel.clone());
            Ok((
                (cache.clone(), env.clone(), dae.clone()),
                cache.clone(),
                dae.clone(),
                env.clone(),
                flatString.clone(),
                flat_model.clone(),
                funcs.clone(),
                nf_funcs.clone(),
            ))
        })() {
            cache = __wb0;
            dae = __wb1;
            env = __wb2;
            flatString = __wb3;
            flat_model = __wb4;
            funcs = __wb5;
            nf_funcs = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (true, false) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut dae: DAE::DAElist = dae.clone();
            System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
            dae = FInst::instPath(className.clone(), SymbolTable::getSCode()?);
            Ok(((cache.clone(), env.clone(), dae.clone()), dae.clone()))
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let (false, false) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cache: FCore::Cache = cache.clone();
            let mut dae: DAE::DAElist = dae.clone();
            let mut env: FCore::Graph = env.clone();
            let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>> = scodeP.clone();
            scodeP = SymbolTable::getSCode()?;
            ExecStat::execStat(&(literal!("FrontEnd - Absyn->SCode")))?;
            (cache, env, _, dae) = Inst::instantiateClass(
                cache.clone(),
                InnerOuter::emptyInstHierarchy().clone(),
                scodeP.clone(),
                className.clone(),
                true,
                relaxedFrontEnd,
                true,
            )?;
            dae = DAEUtil::mergeAlgorithmSections(dae.clone())?;
            DAEUtil::getFunctionList(&(FCore::getFunctionTree(&cache)), true)?;
            Ok((
                (cache.clone(), env.clone(), dae.clone()),
                cache.clone(),
                dae.clone(),
                env.clone(),
                scodeP.clone(),
            ))
        })() {
            cache = __wb0;
            dae = __wb1;
            env = __wb2;
            scodeP = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            if !(Error::getNumErrorMessages() == numError) {
                return Err("guard");
            }
            Error::checkCancel()?;
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Instantiation of "));
                    __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?);
                    __mm_s.push_str(&*literal!(" failed with no error message."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            FlagsUtil::set(Flags::SCODE_INST.clone(), nf_inst_actual)?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    FlagsUtil::set(Flags::SCODE_INST.clone(), nf_inst_actual)?;
    Ok((cache, env, dae, flatString))
}

pub(crate) fn runFrontEndWorkNF(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut relaxedFrontend: bool,
    mut dumpFlat: bool,
) -> Result<(
    metamodelica::Ref<NFFlatModel::NFFlatModel>,
    metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>,
    ArcStr,
)> {
    let mut flatModel: metamodelica::Ref<NFFlatModel::NFFlatModel>;
    let mut functions: metamodelica::Ref<NFFlatten::FunctionTreeImpl::Tree>;
    let mut flatString: ArcStr;
    let mut builtin_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut scode_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut annotation_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut nf_api: bool;
    let mut cls_name: metamodelica::Ref<Absyn::Path> = className;
    let mut obfuscate_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;
    let mut obfuscate_mode: ArcStr;
    (_, builtin_p) = FBuiltin::getInitialFunctions()?;
    scode_p = SymbolTable::getSCode()?;
    obfuscate_mode = Flags::getConfigString(Flags::OBFUSCATE.clone())?;
    if metamodelica::stringEq(&obfuscate_mode, &(literal!("none")))
        && Interactive::astContainsEncryptedClass(SymbolTable::getAbsyn())?
    {
        FlagsUtil::setConfigString(Flags::OBFUSCATE.clone(), literal!("encrypted"))?;
    }
    if metamodelica::stringEq(&obfuscate_mode, &(literal!("full"))) {
        (scode_p, cls_name, _, _, obfuscate_map) =
            Obfuscate::obfuscateProgram(scode_p, cls_name, SCode::noComment.clone())?;
    }
    scode_p = listAppend(builtin_p, scode_p);
    ExecStat::execStat(&(literal!("FrontEnd - Absyn->SCode")))?;
    annotation_p = AbsynToSCode::translateAbsyn2SCode(InteractiveUtil::modelicaAnnotationProgram(
        Config::getAnnotationVersion()?,
    )?)?;
    nf_api = FlagsUtil::set(Flags::NF_API.clone(), false)?;
    if let Ok((__pa0, __pa1, __pa2)) = NFInst::instClassInProgram(
        cls_name.clone(),
        scode_p.clone(),
        annotation_p.clone(),
        relaxedFrontend,
        dumpFlat,
    ) {
        flatModel = metamodelica::Own::own(__pa0);
        functions = metamodelica::Own::own(__pa1);
        flatString = metamodelica::Own::own(__pa2);
    } else {
        NFInst::clearCaches()?;
        FlagsUtil::set(Flags::NF_API.clone(), nf_api)?;
        return Err("fail");
    }
    FlagsUtil::set(Flags::NF_API.clone(), nf_api)?;
    Ok((flatModel, functions, flatString))
}

pub(crate) fn translateModel(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileNamePrefix: ArcStr,
    mut runBackend: bool,
    mut runSilent: bool,
    mut simSettingsOpt: Option<SimCode::SimulationSettings>,
) -> Result<(
    bool,
    FCore::Cache,
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
)> {
    let mut success: bool;
    let mut outCache: FCore::Cache;
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outFileDir: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>;
    let mut flags: Flags::Flag;
    let mut defaultSimOpt: InteractiveTypes::SimulationOptions;
    let mut simSettings: Option<SimCode::SimulationSettings>;
    if (simSettingsOpt).is_some() {
        simSettings = simSettingsOpt;
    } else {
        defaultSimOpt = buildSimulationOptionsFromModelExperimentAnnotation(
            className.clone(),
            fileNamePrefix.clone(),
            Some(defaultSimulationOptions().clone()),
        )?;
        simSettings = Some(convertSimulationOptionsToSimCode(&defaultSimOpt)?);
    }
    flags = loadCommandLineOptionsFromModel(className.clone())?;
    match '__try0: {
        (success, outCache, outLibs, outFileDir, resultValues) = unwrap_break_err!(SimCodeMain::translateModel(crate::SimCodeMain::TranslateModelKind::NORMAL, cache.clone(), env.clone(), className.clone(), fileNamePrefix.clone(), runBackend, unwrap_break_err!(Flags::getConfigBool(Flags::DAE_MODE.clone()), '__try0), runSilent, simSettings.clone(), &(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }))), '__try0);
        FlagsUtil::saveFlags(flags.clone());
        Ok::<_, &'static str>((
            outCache.clone(),
            outFileDir.clone(),
            outLibs.clone(),
            resultValues.clone(),
            success.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            outCache = __try0_o0;
            outFileDir = __try0_o1;
            outLibs = __try0_o2;
            resultValues = __try0_o3;
            success = __try0_o4;
        }
        Err(__try0_err) => {
            FlagsUtil::saveFlags(flags.clone());
            return Err(__try0_err);
        }
    }
    Ok((success, outCache, outLibs, outFileDir, resultValues))
}

fn getProcsStr(mut isMake: bool) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut n: i32;
    let mut sn: ArcStr;
    n = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    sn = intString(n);
    s = if (n == 0) {
        literal!("")
    } else {
        if (isMake) { sn } else { stringAppend(literal!("-j"), sn) }
    };
    Ok(s)
}

fn configureFMU_cmake(
    mut platform: ArcStr,
    mut fmutmp: &ArcStr,
    mut fmuTargetName: &ArcStr,
    mut logfile: ArcStr,
    mut externalLibLocations: &metamodelica::List<ArcStr>,
    mut isWindows: bool,
    mut needs3rdPartyLibs: bool,
) -> Result<()> {
    let mut fmuSourceDir: ArcStr;
    let mut CMAKE_GENERATOR: ArcStr = literal!("");
    let mut CMAKE_BUILD_TYPE: ArcStr;
    let mut quote: ArcStr;
    let mut dquote: ArcStr;
    let mut defaultFmiIncludeDirectoy: ArcStr;
    let mut CC: ArcStr;
    let mut makefileParams: SimCodeFunction::MakefileParams;
    let mut msvcEnv: ArcStr;
    let mut rmrf: ArcStr;
    makefileParams = SimCodeFunctionUtil::createMakefileParams(
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        false,
        true,
    )?;
    fmuSourceDir = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fmutmp);
        __mm_s.push_str(&*literal!("/sources/"));
        ArcStr::from(__mm_s)
    };
    quote = literal!("'");
    dquote = if (isWindows) { literal!("\"") } else { literal!("'") };
    msvcEnv = SimCodeUtil::msvcEnvironment()?;
    if !metamodelica::stringEq(&msvcEnv, &(literal!(""))) {
        CC = literal!("");
        CMAKE_GENERATOR = literal!("-G \"NMake Makefiles\" ");
        rmrf = literal!("rmdir /S /Q ");
    } else {
        CC = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-DCMAKE_C_COMPILER="));
            __mm_s.push_str(&*dquote);
            __mm_s.push_str(&*System::basename(makefileParams.ccompiler.clone()));
            __mm_s.push_str(&*dquote);
            ArcStr::from(__mm_s)
        };
        if isWindows {
            CMAKE_GENERATOR = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-G "));
                __mm_s.push_str(&*dquote);
                __mm_s.push_str(&*literal!("MSYS Makefiles"));
                __mm_s.push_str(&*dquote);
                __mm_s.push_str(&*literal!(" "));
                ArcStr::from(__mm_s)
            };
        }
        rmrf = literal!("rm -rf ");
    }
    defaultFmiIncludeDirectoy = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dquote);
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/include/omc/c/fmi"));
        __mm_s.push_str(&*dquote);
        ArcStr::from(__mm_s)
    };
    if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone()
        || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_PROTECTED.clone()
    {
        CMAKE_BUILD_TYPE = literal!("-DCMAKE_BUILD_TYPE=Release");
    } else if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
        CMAKE_BUILD_TYPE = literal!("-DCMAKE_BUILD_TYPE=Debug");
    } else {
        CMAKE_BUILD_TYPE = literal!("-DCMAKE_BUILD_TYPE=RelWithDebInfo");
    }
    if System::regularFileExists(logfile.clone()) {
        System::removeFile(logfile.clone());
    }
    let () = (::match_deref::match_deref! { match &(Util::stringSplitAtChar(platform.clone(), literal!(" "))?) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "dynamic", tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cmd: ArcStr;
            let mut cmakeCall: ArcStr;
            let mut buildDir: ArcStr;
            buildDir = literal!("build_cmake_dynamic");
            cmakeCall = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CMAKE_GENERATOR); __mm_s.push_str(&*CMAKE_BUILD_TYPE); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CC); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) };
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*msvcEnv); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*fmuSourceDir); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" --build . --parallel ")); __mm_s.push_str(&*getProcsStr(false)?); __mm_s.push_str(&*literal!(" --target install && ")); __mm_s.push_str(&*literal!("cd .. && ")); __mm_s.push_str(&*rmrf); __mm_s.push_str(&*buildDir); ArcStr::from(__mm_s) };
            if 0 != System::systemCallRestrictedEnv(cmd.clone(), logfile.clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmd: ")); __mm_s.push_str(&*cmd); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile(logfile)?); ArcStr::from(__mm_s) }])?;
                return Err("fail");
            }
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "static", tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cmd: ArcStr;
            let mut cmakeCall: ArcStr;
            let mut buildDir: ArcStr;
            buildDir = literal!("build_cmake_static");
            cmakeCall = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CMAKE_GENERATOR); __mm_s.push_str(&*CMAKE_BUILD_TYPE); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CC); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) };
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*msvcEnv); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*fmuSourceDir); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" --build . --parallel ")); __mm_s.push_str(&*getProcsStr(false)?); __mm_s.push_str(&*literal!(" --target install && ")); __mm_s.push_str(&*literal!("cd .. && ")); __mm_s.push_str(&*rmrf); __mm_s.push_str(&*buildDir); ArcStr::from(__mm_s) };
            if 0 != System::systemCallRestrictedEnv(cmd.clone(), logfile.clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmd: ")); __mm_s.push_str(&*cmd); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile(logfile)?); ArcStr::from(__mm_s) }])?;
                return Err("fail");
            }
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: crossTriple, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "docker", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "run", tail: dockerImgArgs } } } => {
            let mut cmd: ArcStr;
            let mut cmakeCall: ArcStr;
            let mut buildDir: ArcStr;
            let mut fmiTarget: ArcStr;
            let mut externalIncludeDirsFile: ArcStr;
            let mut dockerImage; // TODO: local with unresolved type
            let mut dockerArguments: metamodelica::List<ArcStr>;
            let mut dockerRunArgs: ArcStr;
            let mut isTrustedImage: bool;
            let mut uid: i32;
            let mut cidFile: ArcStr;
            let mut volumeID: ArcStr;
            let mut containerID: ArcStr;
            let mut userID: ArcStr;
            let mut dockerLogFile: ArcStr;
            let mut cmake_toolchain: ArcStr;
            let mut locations: metamodelica::List<ArcStr>;
            (dockerImage, dockerArguments) = ContainerImage::parseWithArgs(dockerImgArgs.clone())?;
            dockerImage = ContainerImage::getDigestSha(dockerImage)?;
            Error::addCompilerNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using docker image '")); __mm_s.push_str(&*ContainerImage::toString(&dockerImage, false)?); __mm_s.push_str(&*literal!("' for cross compilation.")); ArcStr::from(__mm_s) })?;
            (_, isTrustedImage) = ContainerImage::isTrustedOpenModelicaImage(&dockerImage)?;
            dockerRunArgs = { let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(dockerArguments, literal!(" "))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ContainerImage::toString(&dockerImage, false)?); ArcStr::from(__mm_s) };
            uid = System::getuid();
            cidFile = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!(".cidfile")); ArcStr::from(__mm_s) };
            dockerLogFile = { let mut __mm_s = String::new(); __mm_s.push_str(&*crossTriple); __mm_s.push_str(&*literal!(".tmp.log")); ArcStr::from(__mm_s) };
            if System::regularFileExists(dockerLogFile.clone()) {
                System::removeFile(dockerLogFile.clone());
            }
            if isTrustedImage && !(ContainerImage::isAvailableLocally(&dockerImage)?) {
                if ContainerImage::isCosignAvailable()? {
                    ContainerImage::assertSignature(&dockerImage)?;
                    ContainerImage::pull(&dockerImage)?;
                } else {
                    Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Refusing to download container image '")); __mm_s.push_str(&*ContainerImage::toString(&dockerImage, false)?); __mm_s.push_str(&*literal!("' without verifying its signature.")); ArcStr::from(__mm_s) })?;
                    Error::addCompilerNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Download the image manually with `")); __mm_s.push_str(&*ContainerImage::pullCommand(&dockerImage)?); __mm_s.push_str(&*literal!("` and run the FMU export again.")); ArcStr::from(__mm_s) })?;
                    return Err("fail");
                }
            }
            cmd = literal!("docker volume create");
            runDockerCmd(cmd, dockerLogFile.clone(), false, &(literal!("")), &(literal!("")))?;
            volumeID = List::last(&(System::strtok(System::readFile(dockerLogFile.clone())?, literal!("\n"))))?;
            if System::regularFileExists(cidFile.clone()) {
                System::removeFile(cidFile.clone());
            }
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run --cidfile ")); __mm_s.push_str(&*cidFile); __mm_s.push_str(&*literal!(" -v ")); __mm_s.push_str(&*volumeID); __mm_s.push_str(&*literal!(":/data busybox true")); ArcStr::from(__mm_s) };
            runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &(literal!("")))?;
            containerID = System::trim(System::readFile(cidFile.clone())?, literal!(" \u{c}\n\r\t\u{b}"));
            System::removeFile(cidFile);
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*literal!(":/data")); ArcStr::from(__mm_s) };
            runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*defaultFmiIncludeDirectoy); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*literal!(":/data/fmiInclude")); ArcStr::from(__mm_s) };
            runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
            (locations, _) = SimCodeUtil::getDirectoriesForDLLsFromLinkLibs(externalLibLocations)?;
            externalIncludeDirsFile = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/.external_include_dirs")); ArcStr::from(__mm_s) };
            if System::regularFileExists(externalIncludeDirsFile.clone()) {
                locations = listAppend(System::strtok(System::readFile(externalIncludeDirsFile)?, literal!("\n")), locations);
            }
            if needs3rdPartyLibs {
                locations = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }, locations);
            }
            for mut loc in &*locations {
                if System::directoryExists(loc.clone()) {
                    cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run --rm --hostname=")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*literal!(" --volume=")); __mm_s.push_str(&*volumeID); __mm_s.push_str(&*literal!(":/data busybox mkdir -p ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!("/data")); __mm_s.push_str(&*loc); __mm_s.push_str(&*dquote); ArcStr::from(__mm_s) };
                    runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
                    cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp -a -L ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*loc); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!(":/data")); __mm_s.push_str(&*System::dirname(loc.clone())); __mm_s.push_str(&*dquote); ArcStr::from(__mm_s) };
                    runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
                }
            }
            userID = if (uid != 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--user ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", uid))); ArcStr::from(__mm_s) }} else {literal!("")};
            buildDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("build_cmake_")); __mm_s.push_str(&*crossTriple); ArcStr::from(__mm_s) };
            if 0 != (System::regex(crossTriple.clone(), literal!("mingw"), 1, false, false)).0 {
                fmiTarget = literal!(" -DCMAKE_SYSTEM_NAME=Windows ");
            } else if 0 != (System::regex(crossTriple.clone(), literal!("apple"), 1, false, false)).0 {
                fmiTarget = literal!(" -DCMAKE_SYSTEM_NAME=Darwin ");
            } else {
                fmiTarget = literal!("");
            }
            if isTrustedImage {
                cmake_toolchain = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-DCMAKE_TOOLCHAIN_FILE=/opt/cmake/toolchain/")); __mm_s.push_str(&*crossTriple); __mm_s.push_str(&*literal!(".cmake ")); ArcStr::from(__mm_s) };
            } else {
                cmake_toolchain = literal!("");
            }
            cmakeCall = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmake ")); __mm_s.push_str(&*cmake_toolchain); __mm_s.push_str(&*literal!("-DFMI_INTERFACE_HEADER_FILES_DIRECTORY=/fmu/fmiInclude ")); __mm_s.push_str(&*literal!("-DDOCKER_VOL_DIR=/fmu ")); __mm_s.push_str(&*fmiTarget); __mm_s.push_str(&*CMAKE_BUILD_TYPE); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) };
            cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run ")); __mm_s.push_str(&*userID); __mm_s.push_str(&*literal!(" --rm -w /fmu -v ")); __mm_s.push_str(&*volumeID); __mm_s.push_str(&*literal!(":/fmu ")); __mm_s.push_str(&*dockerRunArgs); __mm_s.push_str(&*literal!(" sh -c ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!("/fmu/")); __mm_s.push_str(&*fmuSourceDir); __mm_s.push_str(&*dquote); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("cmake --build . --parallel ")); __mm_s.push_str(&*getProcsStr(false)?); __mm_s.push_str(&*literal!(" --target install && ")); __mm_s.push_str(&*literal!("cd .. && rm -rf ")); __mm_s.push_str(&*buildDir); __mm_s.push_str(&*dquote); ArcStr::from(__mm_s) };
            runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
            if isWindows {
                cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run ")); __mm_s.push_str(&*userID); __mm_s.push_str(&*literal!(" --rm -w /fmu -v ")); __mm_s.push_str(&*volumeID); __mm_s.push_str(&*literal!(":/fmu ")); __mm_s.push_str(&*dockerRunArgs); __mm_s.push_str(&*literal!(" tar -zcf comp-fmutmp.tar.gz ")); __mm_s.push_str(&*fmutmp); ArcStr::from(__mm_s) };
                runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
                cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*literal!(":/data/comp-fmutmp.tar.gz .")); ArcStr::from(__mm_s) };
                runDockerCmd(cmd, dockerLogFile.clone(), true, &volumeID, &containerID)?;
                if 0 != System::systemCall(literal!("tar zxf comp-fmutmp.tar.gz && rm comp-fmutmp.tar.gz"), dockerLogFile.clone()) {
                    Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to unpack comp-fmutmp.tar.gz:\n")); __mm_s.push_str(&*System::readFile(dockerLogFile.clone())?); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                }
            } else {
                cmd = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*containerID); __mm_s.push_str(&*literal!(":/data/")); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/ .")); ArcStr::from(__mm_s) };
                runDockerCmd(cmd, dockerLogFile.clone(), false, &volumeID, &containerID)?;
            }
            System::systemCall({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID); ArcStr::from(__mm_s) }, dockerLogFile.clone());
            System::systemCall({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID); ArcStr::from(__mm_s) }, dockerLogFile.clone());
            System::copyFile(dockerLogFile.clone(), logfile);
            System::removeFile(dockerLogFile);
            ()
        },
        _ => {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown/unsupported platform \"")); __mm_s.push_str(&*platform); __mm_s.push_str(&*literal!(" \" for CMake FMU build. ")); __mm_s.push_str(&*literal!("Use platforms={\"dynamic\"} for the default case.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn runDockerCmd(
    mut cmd: ArcStr,
    mut logfile: ArcStr,
    mut cleanup: bool,
    mut volumeID: &ArcStr,
    mut containerID: &ArcStr,
) -> Result<()> {
    let mut verbose: bool = false;
    System::appendFile(logfile.clone(), {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*cmd);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    })?;
    if 0 != System::systemCall(cmd.clone(), logfile.clone()) {
        Error::addMessage(
            Error::SIMULATOR_BUILD_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*cmd);
                __mm_s.push_str(&*literal!(" failed:\n"));
                __mm_s.push_str(&*System::readFile(logfile.clone())?);
                ArcStr::from(__mm_s)
            }],
        )?;
        if cleanup {
            if !(stringEqual(&containerID, &(literal!("")))) {
                System::systemCall(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("docker rm "));
                        __mm_s.push_str(&*containerID);
                        ArcStr::from(__mm_s)
                    },
                    logfile.clone(),
                );
            }
            if !(stringEqual(&volumeID, &(literal!("")))) {
                System::systemCall(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("docker volume rm "));
                        __mm_s.push_str(&*volumeID);
                        ArcStr::from(__mm_s)
                    },
                    logfile,
                );
            }
        }
        return Err("fail");
    } else if verbose {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::readFile(logfile)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn isDockerPlatform(mut platform: ArcStr) -> Result<bool> {
    let mut isDocker: bool;
    isDocker = (::match_deref::match_deref! { match &(Util::stringSplitAtChar(platform, literal!(" "))?) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "docker", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "run", tail: _ } } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isDocker)
}

fn translateModelFMU(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut FMUVersion: ArcStr,
    mut inFMUType: ArcStr,
    mut inFileNamePrefix: ArcStr,
    mut addDummy: bool,
    mut platforms: metamodelica::List<ArcStr>,
    mut inSimSettings: Option<SimCode::SimulationSettings>,
) -> Result<(bool, FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut success: bool = false;
    let mut cache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut flags: Flags::Flag;
    flags = loadCommandLineOptionsFromModel(className.clone())?;
    match '__try0: {
        (success, cache, outValue) = unwrap_break_err!(callTranslateModelFMU(inCache.clone(), inEnv.clone(), className.clone(), FMUVersion.clone(), inFMUType.clone(), inFileNamePrefix.clone(), addDummy, platforms.clone(), inSimSettings.clone()), '__try0);
        FlagsUtil::saveFlags(flags.clone());
        Ok::<_, &'static str>((cache.clone(), outValue.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            cache = __try0_o0;
            outValue = __try0_o1;
            success = __try0_o2;
        }
        Err(__try0_err) => {
            FlagsUtil::saveFlags(flags.clone());
            return Err(__try0_err);
        }
    }
    Ok((success, cache, outValue))
}

fn callTranslateModelFMU(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut FMUVersion: ArcStr,
    mut inFMUType: ArcStr,
    mut inFileNamePrefix: ArcStr,
    mut addDummy: bool,
    mut platforms: metamodelica::List<ArcStr>,
    mut inSimSettings: Option<SimCode::SimulationSettings>,
) -> Result<(bool, FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut success: bool;
    let mut cache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut filenameprefix: ArcStr;
    let mut fmuTargetName: ArcStr;
    let mut defaultSimOpt: InteractiveTypes::SimulationOptions;
    let mut simSettings: SimCode::SimulationSettings;
    let mut libs: metamodelica::List<ArcStr>;
    let mut FMUType: ArcStr = inFMUType;
    let mut isWasmFMU: bool = isWasmFMUExport(&FMUVersion, platforms.clone())?;
    cache = inCache;
    if !(FMI::checkFMIVersion(&FMUVersion)) {
        success = false;
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::UNKNOWN_FMU_VERSION.clone(), list![FMUVersion])?;
        return Ok((success, cache, outValue));
    } else if !(FMI::checkFMIType(&FMUType)) {
        success = false;
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::UNKNOWN_FMU_TYPE.clone(), list![FMUType])?;
        return Ok((success, cache, outValue));
    }
    if !(FMI::canExportFMU(&FMUVersion, &FMUType)) {
        success = false;
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::FMU_EXPORT_NOT_SUPPORTED.clone(), list![FMUType, FMUVersion])?;
        return Ok((success, cache, outValue));
    }
    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("Cpp"))) && FMI::isFMICSType(&FMUType) {
        Error::addMessage(Error::FMU_EXPORT_NOT_SUPPORTED_CPP.clone(), list![FMUType])?;
        FMUType = literal!("me");
    }
    if Flags::getConfigBool(Flags::DAE_MODE.clone())? && !(isWasmFMU) {
        success = false;
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        if FMI::isFMIMEType(&FMUType) {
            Error::addMessage(Error::FMU_EXPORT_DAE_MODE_ME.clone(), list![FMUType])?;
        } else {
            Error::addMessage(Error::FMU_EXPORT_DAE_MODE_C_CS.clone(), metamodelica::nil())?;
        }
        return Ok((success, cache, outValue));
    }
    filenameprefix = Util::stringReplaceChar(
        if (metamodelica::stringEq(&inFileNamePrefix, &(literal!("<default>")))) {
            AbsynUtil::pathLastIdent(&className)
        } else {
            inFileNamePrefix.clone()
        },
        literal!("."),
        literal!("_"),
    )?;
    fmuTargetName = if (metamodelica::stringEq(&FMUVersion, &(literal!("1.0")))) {
        filenameprefix.clone()
    } else {
        if (metamodelica::stringEq(&inFileNamePrefix, &(literal!("<default>")))) {
            AbsynUtil::pathLastIdent(&className)
        } else {
            inFileNamePrefix
        }
    };
    if (inSimSettings).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(inSimSettings) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        simSettings = metamodelica::Own::own(__pa0);
    } else {
        defaultSimOpt = buildSimulationOptionsFromModelExperimentAnnotation(
            className.clone(),
            filenameprefix.clone(),
            Some(defaultSimulationOptions().clone()),
        )?;
        simSettings = convertSimulationOptionsToSimCode(&defaultSimOpt)?;
    }
    if isWasmFMU {
        FlagsUtil::setConfigString(Flags::SIMCODE_TARGET.clone(), literal!("wasm-jit"))?;
        FlagsUtil::setConfigString(
            Flags::FMU_NATIVE_PLATFORMS.clone(),
            stringDelimitList(
                List::select(
                    platforms,
                    (std::sync::Arc::new(move |__a0: ArcStr| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isNotWasmPlatform(&__a0))
                    }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>),
                )?,
                literal!(","),
            ),
        )?;
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_FMU.clone(), true)?;
    FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), FMUVersion)?;
    match '__try1: {
        (success, cache, libs, _, _) = unwrap_break_err!(SimCodeMain::translateModel(SimCodeMain::TranslateModelKind::FMU { kind: FMUType.clone(), targetName: fmuTargetName.clone(), translateOnly: isWasmFMU }, cache.clone(), inEnv.clone(), className.clone(), filenameprefix.clone(), true, unwrap_break_err!(Flags::getConfigBool(Flags::DAE_MODE.clone()), '__try1), true, Some(simSettings.clone()), &(Absyn::emptyFunctionArgs.clone())), '__try1);
        if success && isWasmFMU {
            unwrap_break_err!(CodegenWasmJit::finishCompile(filenameprefix.clone()), '__try1);
        }
        outValue = metamodelica::Ref::new(Values::Value::STRING {
            string: {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*if (!(unwrap_break_err!(Testsuite::isRunning(), '__try1))) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*System::pwd());
                        __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    literal!("")
                });
                __mm_s.push_str(&*fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            },
        });
        Ok::<_, &'static str>((outValue.clone(), success.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            outValue = __try1_o0;
            success = __try1_o1;
        }
        Err(_) => {
            success = false;
            outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        }
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_FMU.clone(), false)?;
    FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), literal!(""))?;
    Ok((success, cache, outValue))
}

pub(crate) fn generateFMI3GraphicalRepresentation(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fmutmp: &ArcStr,
    mut modelIdentifier: &ArcStr,
) -> () {
    let mut handle: i32;
    let mut nConn: i32;
    let mut i: i32 = 0;
    let mut svg: ArcStr;
    let mut grepr: ArcStr;
    let mut modelName: ArcStr;
    let mut taiDir: ArcStr;
    let mut taiFile: ArcStr;
    let mut content: ArcStr;
    let mut info: ArcStr;
    let mut cname: ArcStr;
    let mut ibase: ArcStr;
    let mut sx1: ArcStr;
    let mut sy1: ArcStr;
    let mut sx2: ArcStr;
    let mut sy2: ArcStr;
    let mut csvg: ArcStr;
    let mut tgr: ArcStr;
    let mut parts: metamodelica::List<ArcStr>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(NFApi::getModelInstanceIconReference(className.clone()), '__try0)) {
            Deref @ Values::Value::INTEGER { integer: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        handle = metamodelica::Own::own(__pa1);
        if handle > 0 {
            match '__try2: {
                modelName = AbsynUtil::pathLastIdent(&className);
                taiDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp); __mm_s.push_str(&*literal!("/terminalsAndIcons/")); ArcStr::from(__mm_s) };
                taiFile = { let mut __mm_s = String::new(); __mm_s.push_str(&*taiDir); __mm_s.push_str(&*literal!("terminalsAndIcons.xml")); ArcStr::from(__mm_s) };
                svg = OMGraphics::iconSVGFromHandle(handle, modelName.clone());
                grepr = OMGraphics::graphicalRepresentationXMLFromHandle(handle, metamodelica::OrderedFloat(0.5_f64));
                nConn = OMGraphics::placedConnectorCount(handle);
                if !metamodelica::stringEq(&svg, &(literal!(""))) {
                    Util::createDirectoryTree(taiDir.clone());
                    if OMGraphics::writeIconPNGFromHandle(handle, modelName.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*taiDir); __mm_s.push_str(&*literal!("icon.png")); ArcStr::from(__mm_s) }) {
                        unwrap_break_err!(System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*taiDir); __mm_s.push_str(&*literal!("icon.svg")); ArcStr::from(__mm_s) }, svg.clone()), '__try2);
                    } else {
                        grepr = literal!("");
                    }
                } else {
                    grepr = literal!("");
                }
                if !metamodelica::stringEq(&grepr, &(literal!(""))) || nConn > 0 {
                    if System::regularFileExists(taiFile.clone()) {
                        content = unwrap_break_err!(System::readFile(taiFile.clone()), '__try2);
                    } else {
                        Util::createDirectoryTree(taiDir.clone());
                        content = literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<fmiTerminalsAndIcons fmiVersion=\"3.0\">\n</fmiTerminalsAndIcons>\n");
                    }
                    for mut i in 0..=nConn - 1 {
                        info = OMGraphics::placedConnectorInfo(handle, i);
                        parts = System::strtok(info.clone(), literal!("\t"));
                        if ((parts).len() as i32) == 6 {
                            cname = unwrap_break_err!((parts).get(1), '__try2);
                            ibase = unwrap_break_err!((parts).get(2), '__try2);
                            sx1 = unwrap_break_err!((parts).get(3), '__try2);
                            sy1 = unwrap_break_err!((parts).get(4), '__try2);
                            sx2 = unwrap_break_err!((parts).get(5), '__try2);
                            sy2 = unwrap_break_err!((parts).get(6), '__try2);
                            if unwrap_break_err!(System::stringFind(content.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<Terminal name=\"")); __mm_s.push_str(&*cname); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }), '__try2) >= 0 {
                                if OMGraphics::writePlacedConnectorIconPNG(handle, i, { let mut __mm_s = String::new(); __mm_s.push_str(&*taiDir); __mm_s.push_str(&*ibase); __mm_s.push_str(&*literal!(".png")); ArcStr::from(__mm_s) }) {
                                    csvg = OMGraphics::placedConnectorIconSVG(handle, i);
                                    if !metamodelica::stringEq(&csvg, &(literal!(""))) {
                                        unwrap_break_err!(System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*taiDir); __mm_s.push_str(&*ibase); __mm_s.push_str(&*literal!(".svg")); ArcStr::from(__mm_s) }, csvg.clone()), '__try2);
                                    }
                                    tgr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("      <TerminalGraphicalRepresentation x1=\"")); __mm_s.push_str(&*sx1); __mm_s.push_str(&*literal!("\" y1=\"")); __mm_s.push_str(&*sy1); __mm_s.push_str(&*literal!("\" x2=\"")); __mm_s.push_str(&*sx2); __mm_s.push_str(&*literal!("\" y2=\"")); __mm_s.push_str(&*sy2); __mm_s.push_str(&*literal!("\" iconBaseName=\"")); __mm_s.push_str(&*ibase); __mm_s.push_str(&*literal!(".png\"/>\n")); ArcStr::from(__mm_s) };
                                    content = unwrap_break_err!(insertBeforeTerminalClose(content.clone(), &cname, &tgr), '__try2);
                                }
                            }
                        }
                    }
                    if !metamodelica::stringEq(&grepr, &(literal!(""))) {
                        content = unwrap_break_err!(spliceGraphicalRepresentation(content.clone(), &grepr), '__try2);
                    }
                    unwrap_break_err!(System::writeFile(taiFile.clone(), content.clone()), '__try2);
                }
                Ok::<_, &'static str>((grepr.clone(), modelName.clone(), nConn.clone(), svg.clone(), taiDir.clone(), taiFile.clone()))
            } {
                Ok((__try2_o0, __try2_o1, __try2_o2, __try2_o3, __try2_o4, __try2_o5)) => {
                    grepr = __try2_o0;
                    modelName = __try2_o1;
                    nConn = __try2_o2;
                    svg = __try2_o3;
                    taiDir = __try2_o4;
                    taiFile = __try2_o5;
                }
                Err(_) => {
                    ::match_deref::match_deref! { match &(NFApi::releaseModelInstanceReference(handle)) {
                        Deref @ Values::Value::BOOL { boolean: _ } => (),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    break '__try0 Err::<_, _>("fail");
                }
            }
            ::match_deref::match_deref! { match &(NFApi::releaseModelInstanceReference(handle)) {
                Deref @ Values::Value::BOOL { boolean: _ } => (),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    ()
}

fn spliceGraphicalRepresentation(mut content: ArcStr, mut graphicalRepresentation: &ArcStr) -> Result<ArcStr> {
    let mut result: ArcStr;
    if System::stringFind(content.clone(), literal!("  <Terminals>"))? >= 0 {
        result = System::stringReplace(content, literal!("  <Terminals>"), {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*graphicalRepresentation);
            __mm_s.push_str(&*literal!("  <Terminals>"));
            ArcStr::from(__mm_s)
        })?;
    } else if System::stringFind(content.clone(), literal!("<Terminals>"))? >= 0 {
        result = System::stringReplace(content, literal!("<Terminals>"), {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*graphicalRepresentation);
            __mm_s.push_str(&*literal!("<Terminals>"));
            ArcStr::from(__mm_s)
        })?;
    } else {
        result = System::stringReplace(content, literal!("</fmiTerminalsAndIcons>"), {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*graphicalRepresentation);
            __mm_s.push_str(&*literal!("</fmiTerminalsAndIcons>"));
            ArcStr::from(__mm_s)
        })?;
    }
    Ok(result)
}

fn insertBeforeTerminalClose(mut content: ArcStr, mut name: &ArcStr, mut insertion: &ArcStr) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut marker: ArcStr;
    let mut tail: ArcStr;
    let mut p: i32;
    let mut r: i32;
    let mut k: i32;
    let mut len: i32;
    marker = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("<Terminal name=\""));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("\""));
        ArcStr::from(__mm_s)
    };
    p = System::stringFind(content.clone(), marker)?;
    len = ((content).len() as i32);
    if p < 0 {
        result = content;
    } else {
        tail = substring(content.clone(), p + 1, len)?;
        r = System::stringFind(tail, literal!("</Terminal>"))?;
        if r < 0 {
            result = content;
        } else {
            k = p + r;
            while k >= 1 && stringEq(&(substring(content.clone(), k, k)?), &(literal!(" "))) {
                k = k - 1;
            }
            result = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*substring(content.clone(), 1, k)?);
                __mm_s.push_str(&*insertion);
                __mm_s.push_str(&*substring(content, k + 1, len)?);
                ArcStr::from(__mm_s)
            };
        }
    }
    Ok(result)
}

fn fmuMethodToSimulationFlag(mut method: ArcStr, mut isWasmFMU: bool) -> Result<()> {
    let mut fmiFlags: metamodelica::List<ArcStr>;
    let mut accepted: metamodelica::List<ArcStr> = if (isWasmFMU) {
        CodegenWasmJit::fmuCsSolvers()
    } else {
        list![literal!("euler"), literal!("cvode")]
    };
    if metamodelica::stringEq(&method, &(literal!("<default>"))) || !(listMember(method.clone(), accepted)) {
        return Ok(());
    }
    fmiFlags = Flags::getConfigStringList(Flags::FMI_FLAGS.clone())?;
    for mut f in &*fmiFlags {
        if StringUtil::startsWith(f.clone(), literal!("s:")) {
            return Ok(());
        }
    }
    if !((fmiFlags).is_empty()) && !(stringEq(&((fmiFlags).head().cloned()?), &(literal!("default")))) {
        return Ok(());
    }
    FlagsUtil::setConfigStringList(
        Flags::FMI_FLAGS.clone(),
        list![{
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("s:"));
            __mm_s.push_str(&*method);
            ArcStr::from(__mm_s)
        }],
    )?;
    Ok(())
}

fn fmuAnnotationSimulationFlags(mut className: metamodelica::Ref<Absyn::Path>, mut isWasmFMU: bool) -> Result<()> {
    let mut fmiFlags: metamodelica::List<ArcStr>;
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut folded: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut name: ArcStr;
    let mut value: ArcStr;
    if !(isWasmFMU) || Flags::getConfigBool(Flags::IGNORE_SIMULATION_FLAGS_ANNOTATION.clone())? {
        return Ok(());
    }
    fmiFlags = Flags::getConfigStringList(Flags::FMI_FLAGS.clone())?;
    if ((fmiFlags).len() as i32) == 1 && stringEq(&((fmiFlags).head().cloned()?), &(literal!("default"))) {
        fmiFlags = metamodelica::nil();
    }
    for mut f in &*fmiFlags {
        if !(stringEq(&f, &(literal!("default"))))
            && !(((Util::stringSplitAtChar(f.clone(), literal!(":"))?).len() as i32) == 2)
        {
            return Ok(());
        }
        names = metamodelica::cons(
            (Util::stringSplitAtChar(f.clone(), literal!(":"))?).head().cloned()?,
            names,
        );
    }
    loadProgram(&className)?;
    r#mod = ProgramUtil::getNamedAnnotationExp(
        className,
        SymbolTable::getAbsyn(),
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("__OpenModelica_simulationFlags"),
        })),
        Some(None),
        &fnptr!(Util::id, _),
    )?;
    args = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: __esc_args, .. }) => {
            args = (*__esc_args).clone();
            args.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut arg in &*args {
        name = AbsynUtil::pathString(
            AbsynUtil::elementArgName(metamodelica::AsArg::as_arg(&arg))?,
            literal!("."),
            true,
            false,
        )?;
        value = fmuSimulationFlagValue(metamodelica::AsArg::as_arg(&arg))?;
        if !(listMember(name.clone(), names.clone())) {
            if CodegenWasmJit::fmuAcceptsFlag(name.clone(), value.clone()) {
                folded = metamodelica::cons(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*name);
                        __mm_s.push_str(&*literal!(":"));
                        __mm_s.push_str(&*value);
                        ArcStr::from(__mm_s)
                    },
                    folded,
                );
            } else {
                Error::addCompilerNotification({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Leaving the __OpenModelica_simulationFlags entry "));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!("=\""));
                    __mm_s.push_str(&*value);
                    __mm_s.push_str(&*literal!("\" out of the FMU: it cannot honour it."));
                    ArcStr::from(__mm_s)
                })?;
            }
        }
    }
    if !((folded).is_empty()) {
        FlagsUtil::setConfigStringList(Flags::FMI_FLAGS.clone(), listAppend(fmiFlags, folded.reverse()))?;
    }
    Ok(())
}

fn fmuSimulationFlagValue(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut value: ArcStr;
    value = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. } => {
            (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::STRING { value: Deref @ "()" } => literal!(""),
        Deref @ Absyn::Exp::STRING { value: __exp_value } => __exp_value.clone(),
        _ => Dump::printExpStr(exp.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn reportFMUPlatformsBuilt(mut platforms: &metamodelica::List<ArcStr>) -> Result<()> {
    let mut platformIndex: i32 = 0;
    let mut platformCount: i32 = ((platforms).len() as i32);
    let mut platformName: ArcStr;
    for mut platform in &**platforms {
        platformIndex = platformIndex + 1;
        platformName = (Util::stringSplitAtChar(platform.clone(), literal!(" "))?).get(1)?;
        System::reportProgress(intDiv((platformIndex - 1) * 1000, platformCount), 4);
        System::reportProgressMessage({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Building FMU for "));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        });
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Building FMU for platform '"));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!("' ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")."));
            ArcStr::from(__mm_s)
        })?;
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Finished FMU for platform '"));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!("' ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")."));
            ArcStr::from(__mm_s)
        })?;
    }
    System::reportProgress(1000, 4);
    Ok(())
}

fn fmuSimulationSettings(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inFileNamePrefix: ArcStr,
    mut method: ArcStr,
) -> Result<SimCode::SimulationSettings> {
    let mut simSettings: SimCode::SimulationSettings;
    let mut filenameprefix: ArcStr;
    filenameprefix = Util::stringReplaceChar(
        if (metamodelica::stringEq(&inFileNamePrefix, &(literal!("<default>")))) {
            AbsynUtil::pathLastIdent(&className)
        } else {
            inFileNamePrefix
        },
        literal!("."),
        literal!("_"),
    )?;
    simSettings = convertSimulationOptionsToSimCode(
        &(buildSimulationOptionsFromModelExperimentAnnotation(
            className,
            filenameprefix,
            Some(defaultSimulationOptions().clone()),
        )?),
    )?;
    if !metamodelica::stringEq(&method, &(literal!("<default>"))) {
        simSettings.method = method;
    }
    Ok(simSettings)
}

fn buildModelFMU(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut FMUVersion: ArcStr,
    mut inFMUType: ArcStr,
    mut inFileNamePrefix: ArcStr,
    mut addDummy: bool,
    mut platforms: metamodelica::List<ArcStr>,
    mut inSimSettings: Option<SimCode::SimulationSettings>,
    mut method: ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut flags: Flags::Flag;
    let mut fmiFlags: metamodelica::List<ArcStr>;
    if isProtectedContentAccess(&className)? {
        cache = inCache;
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
    } else {
        flags = loadCommandLineOptionsFromModel(className.clone())?;
        fmiFlags = Flags::getConfigStringList(Flags::FMI_FLAGS.clone())?;
        fmuMethodToSimulationFlag(method, isWasmFMUExport(&FMUVersion, platforms.clone())?)?;
        fmuAnnotationSimulationFlags(className.clone(), isWasmFMUExport(&FMUVersion, platforms.clone())?)?;
        match '__try0: {
            (cache, outValue) = unwrap_break_err!(callBuildModelFMU(inCache.clone(), inEnv.clone(), className.clone(), FMUVersion.clone(), inFMUType.clone(), inFileNamePrefix.clone(), addDummy, platforms.clone(), inSimSettings.clone()), '__try0);
            unwrap_break_err!(FlagsUtil::setConfigStringList(Flags::FMI_FLAGS.clone(), fmiFlags.clone()), '__try0);
            FlagsUtil::saveFlags(flags.clone());
            Ok::<_, &'static str>((cache.clone(), outValue.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                cache = __try0_o0;
                outValue = __try0_o1;
            }
            Err(__try0_err) => {
                FlagsUtil::setConfigStringList(Flags::FMI_FLAGS.clone(), fmiFlags.clone())?;
                FlagsUtil::saveFlags(flags.clone());
                return Err(__try0_err);
            }
        }
    }
    Ok((cache, outValue))
}

pub(crate) fn callBuildModelFMU(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut FMUVersion: ArcStr,
    mut inFMUType: ArcStr,
    mut inFileNamePrefix: ArcStr,
    mut addDummy: bool,
    mut platforms: metamodelica::List<ArcStr>,
    mut inSimSettings: Option<SimCode::SimulationSettings>,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut success: bool;
    let mut filenameprefix: ArcStr;
    let mut fmutmp: ArcStr;
    let mut logfile: ArcStr;
    let mut configureLogFile: ArcStr;
    let mut dir: ArcStr;
    let mut cmd: ArcStr;
    let mut msvcEnv: ArcStr;
    let mut fmuTargetName: ArcStr;
    let mut simSettings: SimCode::SimulationSettings;
    let mut libs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut isWindows: bool;
    let mut needs3rdPartyLibs: bool;
    let mut platformIndex: i32;
    let mut platformCount: i32;
    let mut platformName: ArcStr;
    let mut FMUType: ArcStr = inFMUType;
    let mut wasmRequested: bool = listMember(literal!("wasm"), platforms.clone());
    let mut wasmTarget: bool = metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit")))
        || metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm")));
    let mut isWasmFMU: bool = isWasmFMUExport(&FMUVersion, platforms.clone())?;
    let mut keptSimCode: Option<metamodelica::Ref<SimCode::SimCode>>;
    let mut keptTranslation: metamodelica::Ref<SimCode::SimCode>;
    let mut nativePlatforms: metamodelica::List<ArcStr> = if (wasmRequested) {
        List::select(
            platforms.clone(),
            (std::sync::Arc::new(move |__a0: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isNotWasmPlatform(&__a0))
            }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>),
        )?
    } else {
        list![literal!("native")]
    };
    cache = inCache;
    if !(FMI::checkFMIVersion(&FMUVersion)) {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::UNKNOWN_FMU_VERSION.clone(), list![FMUVersion])?;
        return Ok((cache, outValue));
    } else if !(FMI::checkFMIType(&FMUType)) {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::UNKNOWN_FMU_TYPE.clone(), list![FMUType])?;
        return Ok((cache, outValue));
    }
    if !(FMI::canExportFMU(&FMUVersion, &FMUType)) {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::FMU_EXPORT_NOT_SUPPORTED.clone(), list![FMUType, FMUVersion])?;
        return Ok((cache, outValue));
    }
    if wasmRequested && metamodelica::stringEq(&FMUVersion, &(literal!("1.0"))) {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        Error::addMessage(Error::FMU_EXPORT_WASM_FMI1.clone(), metamodelica::nil())?;
        return Ok((cache, outValue));
    }
    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("Cpp"))) && FMI::isFMICSType(&FMUType) {
        Error::addMessage(Error::FMU_EXPORT_NOT_SUPPORTED_CPP.clone(), list![FMUType])?;
        FMUType = literal!("me");
    }
    if Flags::getConfigBool(Flags::DAE_MODE.clone())? && !(isWasmFMU) {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        if FMI::isFMIMEType(&FMUType) {
            Error::addMessage(Error::FMU_EXPORT_DAE_MODE_ME.clone(), list![FMUType])?;
        } else {
            Error::addMessage(Error::FMU_EXPORT_DAE_MODE_C_CS.clone(), metamodelica::nil())?;
        }
        return Ok((cache, outValue));
    }
    filenameprefix = Util::stringReplaceChar(
        if (metamodelica::stringEq(&inFileNamePrefix, &(literal!("<default>")))) {
            AbsynUtil::pathLastIdent(&className)
        } else {
            inFileNamePrefix.clone()
        },
        literal!("."),
        literal!("_"),
    )?;
    fmuTargetName = if (metamodelica::stringEq(&FMUVersion, &(literal!("1.0")))) {
        filenameprefix.clone()
    } else {
        if (metamodelica::stringEq(&inFileNamePrefix, &(literal!("<default>")))) {
            AbsynUtil::pathLastIdent(&className)
        } else {
            inFileNamePrefix.clone()
        }
    };
    if (inSimSettings).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(inSimSettings) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        simSettings = metamodelica::Own::own(__pa0);
    } else {
        simSettings = fmuSimulationSettings(className.clone(), inFileNamePrefix, literal!("<default>"))?;
    }
    if isWasmFMU {
        FlagsUtil::setConfigString(Flags::SIMCODE_TARGET.clone(), literal!("wasm-jit"))?;
        FlagsUtil::setConfigString(
            Flags::FMU_NATIVE_PLATFORMS.clone(),
            stringDelimitList(nativePlatforms, literal!(",")),
        )?;
    } else if wasmTarget {
        FlagsUtil::setConfigString(Flags::SIMCODE_TARGET.clone(), literal!("C"))?;
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_FMU.clone(), true)?;
    FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), FMUVersion.clone())?;
    keptSimCode = if (isWasmFMU) {
        SimCodeMain::fmuTranslationFor(&FMUVersion, &FMUType, &className, Some(simSettings.clone()))?
    } else {
        None
    };
    match '__try1: {
        if (keptSimCode).is_some() {
            let __pa2 = ::match_deref::match_deref! { match &(keptSimCode.clone()) {
                Some(__pa2) => __pa2.clone(),
                _ => break '__try1 Err::<_, _>("pattern mismatch"),
            } };
            keptTranslation = metamodelica::Own::own(__pa2);
            unwrap_break_err!(Error::addCompilerNotification(literal!("Exporting the translation translateModelFMU already made; the model is not translated again.")), '__try1);
            unwrap_break_err!(SimCodeMain::emitWasmFMU(keptTranslation.clone(), &FMUVersion, &FMUType, SymbolTable::getAbsyn()), '__try1);
            success = true;
        } else {
            (success, cache, libs, _, _) = unwrap_break_err!(SimCodeMain::translateModel(SimCodeMain::TranslateModelKind::FMU { kind: FMUType.clone(), targetName: fmuTargetName.clone(), translateOnly: false }, cache.clone(), inEnv.clone(), className.clone(), filenameprefix.clone(), true, unwrap_break_err!(Flags::getConfigBool(Flags::DAE_MODE.clone()), '__try1), true, Some(simSettings.clone()), &(Absyn::emptyFunctionArgs.clone())), '__try1);
        }
        let true = (success) else {
            break '__try1 Err::<_, _>("pattern mismatch");
        };
        outValue = metamodelica::Ref::new(Values::Value::STRING {
            string: {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*if (!(unwrap_break_err!(Testsuite::isRunning(), '__try1))) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*System::pwd());
                        __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    literal!("")
                });
                __mm_s.push_str(&*fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            },
        });
        Ok::<_, &'static str>((outValue.clone(), success.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            outValue = __try1_o0;
            success = __try1_o1;
        }
        Err(_) => {
            outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
            FlagsUtil::setConfigBool(Flags::BUILDING_FMU.clone(), false)?;
            FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), literal!(""))?;
            return Ok((cache, outValue));
        }
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_FMU.clone(), false)?;
    FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), literal!(""))?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
    isWindows = metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")));
    fmutmp = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Util::hashFileNamePrefix(&filenameprefix)?);
        __mm_s.push_str(&*literal!(".fmutmp"));
        ArcStr::from(__mm_s)
    };
    logfile = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*filenameprefix);
        __mm_s.push_str(&*literal!(".log"));
        ArcStr::from(__mm_s)
    };
    dir = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fmutmp);
        __mm_s.push_str(&*literal!("/sources/"));
        ArcStr::from(__mm_s)
    };
    if isWasmFMU {
        if !(System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmuTargetName);
            __mm_s.push_str(&*literal!(".fmu"));
            ArcStr::from(__mm_s)
        }) || Flags::getConfigBool(Flags::FMU_DIRECTORY.clone())?
            && System::directoryExists({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu"));
                ArcStr::from(__mm_s)
            }))
        {
            Error::addMessage(
                Error::SIMULATOR_BUILD_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("wasm FMU export produced no "));
                    __mm_s.push_str(&*fmuTargetName);
                    __mm_s.push_str(&*literal!(".fmu"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
            return Ok((cache, outValue));
        }
        reportFMUPlatformsBuilt(&platforms)?;
        return Ok((cache, outValue));
    }
    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
        System::removeDirectory(literal!("binaries"));
        for mut platform in &*platforms {
            if metamodelica::stringEq(&platform, &(literal!("dynamic")))
                || metamodelica::stringEq(&platform, &(literal!("static")))
            {
                CevalScript::compileModel(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*filenameprefix);
                        __mm_s.push_str(&*literal!("_FMU"));
                        ArcStr::from(__mm_s)
                    },
                    libs.clone(),
                    literal!(""),
                    metamodelica::nil(),
                )?;
            } else {
                CevalScript::compileModel(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*filenameprefix);
                        __mm_s.push_str(&*literal!("_FMU"));
                        ArcStr::from(__mm_s)
                    },
                    libs.clone(),
                    literal!(""),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("TARGET_TRIPLET="));
                        __mm_s.push_str(&*platform);
                        ArcStr::from(__mm_s)
                    }],
                )?;
            }
            ExecStat::execStat(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("buildModelFMU: Generate C++ for platform "));
                    __mm_s.push_str(&*platform);
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
        if 0 != System::systemCallRestrictedEnv(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(Autoconf::make));
                __mm_s.push_str(&*literal!(" -f "));
                __mm_s.push_str(&*filenameprefix);
                __mm_s.push_str(&*literal!("_FMU.makefile clean"));
                ArcStr::from(__mm_s)
            },
            logfile,
        )? {}
        return Ok((cache, outValue));
    }
    if !(metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsic")))) {
        CevalScript::compileModel(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*filenameprefix);
                __mm_s.push_str(&*literal!("_FMU"));
                ArcStr::from(__mm_s)
            },
            libs.clone(),
            literal!(""),
            metamodelica::nil(),
        )?;
        ExecStat::execStat(&(literal!("buildModelFMU: Generate the FMI files")))?;
    } else {
        fmutmp = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
            ArcStr::from(__mm_s)
        };
        CevalScript::compileModel(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*filenameprefix);
                __mm_s.push_str(&*literal!("_FMU"));
                ArcStr::from(__mm_s)
            },
            libs,
            fmutmp,
            metamodelica::nil(),
        )?;
        return Ok((cache, outValue));
    }
    needs3rdPartyLibs = SimCodeUtil::cvodeFmiFlagIsSet(SimCodeUtil::createFMISimulationFlags(false)?)?;
    if Config::simCodeRustRuntime()?
        && !(System::directoryExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("/sources/rust"));
            ArcStr::from(__mm_s)
        }))
        && List::any(&platforms, &isDockerPlatform)?
    {
        SimCodeMain::copyFmuRustSources(&fmutmp)?;
    }
    platformCount = ((platforms).len() as i32);
    platformIndex = 0;
    for mut platform in &*platforms {
        platformIndex = platformIndex + 1;
        platformName = (Util::stringSplitAtChar(platform.clone(), literal!(" "))?).get(1)?;
        Error::checkCancel()?;
        System::reportProgress(intDiv((platformIndex - 1) * 1000, platformCount), 4);
        System::reportProgressMessage({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Building FMU for "));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        });
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Building FMU for platform '"));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!("' ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")."));
            ArcStr::from(__mm_s)
        })?;
        configureLogFile = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::realpath(fmutmp.clone())?);
            __mm_s.push_str(&*literal!("/resources/"));
            __mm_s.push_str(&*System::stringReplace(
                platformName.clone(),
                literal!("/"),
                literal!("-"),
            )?);
            __mm_s.push_str(&*literal!(".log"));
            ArcStr::from(__mm_s)
        };
        configureFMU_cmake(
            platform.clone(),
            &fmutmp,
            &filenameprefix,
            configureLogFile.clone(),
            &libs,
            isWindows,
            needs3rdPartyLibs,
        )?;
        if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone()
            || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_PROTECTED.clone()
        {
            System::removeFile(configureLogFile);
        }
        Error::addCompilerNotification({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Finished FMU for platform '"));
            __mm_s.push_str(&*platformName);
            __mm_s.push_str(&*literal!("' ("));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformIndex)));
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", platformCount)));
            __mm_s.push_str(&*literal!(")."));
            ArcStr::from(__mm_s)
        })?;
        ExecStat::execStat(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("buildModelFMU: Generate platform "));
                __mm_s.push_str(&*platform);
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    System::reportProgress(1000, 4);
    System::reportProgressMessage(literal!("Packing FMU"));
    Error::checkCancel()?;
    if Flags::getConfigEnum(Flags::FMI_SOURCES.clone())? == Flags::FMI_SOURCES_NONE.clone()
        || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone()
    {
        if !(System::removeDirectory({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("/sources/"));
            ArcStr::from(__mm_s)
        })) {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to remove directory: "));
                    __mm_s.push_str(&*fmutmp);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"),
            )?;
        }
    }
    msvcEnv = SimCodeUtil::msvcEnvironment()?;
    if !metamodelica::stringEq(&msvcEnv, &(literal!(""))) {
        cmd = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*msvcEnv);
            __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake));
            __mm_s.push_str(&*literal!(" -E rm -f \""));
            __mm_s.push_str(&*fmuTargetName);
            __mm_s.push_str(&*literal!(".fmu\" && cd \""));
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("\" && "));
            __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake));
            __mm_s.push_str(&*literal!(" -E tar cf \"../"));
            __mm_s.push_str(&*fmuTargetName);
            __mm_s.push_str(&*literal!(".fmu\" --format=zip ."));
            ArcStr::from(__mm_s)
        };
    } else {
        cmd = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("rm -f \""));
            __mm_s.push_str(&*fmuTargetName);
            __mm_s.push_str(&*literal!(".fmu\" && cd \""));
            __mm_s.push_str(&*fmutmp);
            __mm_s.push_str(&*literal!("\" && zip -r \"../"));
            __mm_s.push_str(&*fmuTargetName);
            __mm_s.push_str(&*literal!(".fmu\" *"));
            ArcStr::from(__mm_s)
        };
    }
    if 0 != System::systemCall(cmd.clone(), logfile.clone()) {
        Error::addMessage(
            Error::SIMULATOR_BUILD_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*cmd);
                __mm_s.push_str(&*literal!("\n\n"));
                __mm_s.push_str(&*System::readFile(logfile)?);
                ArcStr::from(__mm_s)
            }],
        )?;
        ExecStat::execStat(&(literal!("buildModelFMU failed")))?;
    }
    if !(System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*fmuTargetName);
        __mm_s.push_str(&*literal!(".fmu"));
        ArcStr::from(__mm_s)
    })) {
        Error::addMessage(
            Error::SIMULATOR_BUILD_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Build commands returned success, but "));
                __mm_s.push_str(&*fmuTargetName);
                __mm_s.push_str(&*literal!(".fmu does not exist"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    if !(Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?) {
        if !(System::removeDirectory(fmutmp.clone())) {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to remove directory: "));
                    __mm_s.push_str(&*fmutmp);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"),
            )?;
        }
    }
    Ok((cache, outValue))
}

fn isWasmFMUExport(mut FMUVersion: &ArcStr, mut platforms: metamodelica::List<ArcStr>) -> Result<bool> {
    let mut isWasm: bool;
    isWasm = (listMember(literal!("wasm"), platforms)
        || metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit")))
        || metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm"))))
        && !metamodelica::stringEq(&FMUVersion, &(literal!("1.0")));
    Ok(isWasm)
}

fn isNotWasmPlatform(mut platform: &ArcStr) -> bool {
    let mut keep: bool = !(metamodelica::stringEq(&platform, &(literal!("wasm")))
        || metamodelica::stringEq(&platform, &(literal!("static")))
        || metamodelica::stringEq(&platform, &(literal!("dynamic"))));
    keep
}

fn buildEncryptedPackage(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut encrypt: bool,
    mut inProgram: &Absyn::Program,
) -> Result<bool> {
    let mut success: bool = false;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut fileName: ArcStr;
    let mut logFile: ArcStr;
    let mut omhome: ArcStr;
    let mut pd: ArcStr;
    let mut ext: ArcStr;
    let mut packageTool: ArcStr;
    let mut packageToolArgs: ArcStr;
    let mut command: ArcStr;
    let mut runCommand: bool;
    let mut molName: ArcStr;
    let mut dirPath: ArcStr;
    let mut rmCommand: ArcStr;
    let mut cdCommand: ArcStr;
    let mut mvCommand: ArcStr;
    let mut dirOrFileName: ArcStr;
    let mut zipCommand: ArcStr;
    cls = ProgramUtil::getPathedClassInProgram(className.clone(), inProgram, false, false)?;
    fileName = AbsynUtil::classFilename(&cls)?;
    logFile = literal!("buildEncryptedPackage.log");
    runCommand = true;
    if System::regularFileExists(fileName.clone()) {
        omhome = Settings::getInstallationDirectoryPath()?;
        pd = arcstr::literal!(Autoconf::pathDelimiter);
        ext = if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {
            literal!(".exe")
        } else {
            literal!("")
        };
        if encrypt {
            packageTool = stringAppendList(list![
                omhome,
                pd.clone(),
                literal!("bin"),
                pd.clone(),
                literal!("omc-semla"),
                pd,
                literal!("packagetool"),
                ext
            ]);
            if System::regularFileExists(packageTool.clone()) {
                packageToolArgs = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-librarypath \""));
                    __mm_s.push_str(&*System::dirname(fileName));
                    __mm_s.push_str(&*literal!("\" -version \"1.0\" -language \"3.2\" -encrypt \""));
                    __mm_s.push_str(&*boolString(encrypt));
                    __mm_s.push_str(&*literal!("\""));
                    ArcStr::from(__mm_s)
                };
                command = stringAppendList(list![
                    literal!("\""),
                    packageTool,
                    literal!("\""),
                    literal!(" "),
                    packageToolArgs
                ]);
            } else {
                Error::addMessage(Error::ENCRYPTION_NOT_SUPPORTED.clone(), list![packageTool])?;
                runCommand = false;
                command = literal!("");
            }
        } else {
            molName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*AbsynUtil::pathString(className, literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(".mol"));
                ArcStr::from(__mm_s)
            };
            dirPath = System::dirname(fileName.clone());
            rmCommand = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("rm -f \""));
                __mm_s.push_str(&*molName);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            };
            cdCommand = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("cd \""));
                __mm_s.push_str(&*dirPath);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            };
            mvCommand = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("mv \""));
                __mm_s.push_str(&*molName);
                __mm_s.push_str(&*literal!("\" \""));
                __mm_s.push_str(&*System::pwd());
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            };
            if StringUtil::endsWith(fileName.clone(), literal!("package.mo")) {
                dirOrFileName = System::basename(dirPath);
                zipCommand = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("zip -r \""));
                    __mm_s.push_str(&*System::pwd());
                    __mm_s.push_str(&*pd);
                    __mm_s.push_str(&*molName);
                    __mm_s.push_str(&*literal!("\" \""));
                    __mm_s.push_str(&*dirOrFileName);
                    __mm_s.push_str(&*literal!("\""));
                    ArcStr::from(__mm_s)
                };
                command = stringAppendList(list![
                    rmCommand,
                    literal!(" && "),
                    cdCommand,
                    literal!(" && cd .. && "),
                    zipCommand
                ]);
            } else {
                dirOrFileName = System::basename(fileName);
                zipCommand = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("zip -r \""));
                    __mm_s.push_str(&*System::pwd());
                    __mm_s.push_str(&*pd);
                    __mm_s.push_str(&*molName);
                    __mm_s.push_str(&*literal!("\" \""));
                    __mm_s.push_str(&*dirOrFileName);
                    __mm_s.push_str(&*literal!("\""));
                    ArcStr::from(__mm_s)
                };
                command = stringAppendList(list![
                    rmCommand,
                    literal!(" && "),
                    cdCommand,
                    literal!(" && "),
                    zipCommand
                ]);
            }
        }
        if runCommand {
            if System::regularFileExists(logFile.clone()) {
                System::removeFile(logFile.clone());
            }
            success = 0 == System::systemCall(command.clone(), logFile);
            if !(success) {
                Error::addCompilerError({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Command failed: "));
                    __mm_s.push_str(&*command);
                    ArcStr::from(__mm_s)
                })?;
            }
        }
    } else {
        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![fileName])?;
    }
    Ok(success)
}

fn translateModelXML(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileNamePrefix: ArcStr,
    mut addDummy: bool,
    mut inSimSettingsOpt: Option<SimCode::SimulationSettings>,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut success: bool;
    if isProtectedContentAccess(&className)? {
        outValue = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
    } else {
        (success, cache, _, _, _) = SimCodeMain::translateModel(
            crate::SimCodeMain::TranslateModelKind::XML,
            cache,
            env,
            className,
            fileNamePrefix.clone(),
            true,
            false,
            true,
            inSimSettingsOpt,
            &(Absyn::emptyFunctionArgs.clone()),
        )?;
        outValue = metamodelica::Ref::new(Values::Value::STRING {
            string: if (success) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*if (!(Testsuite::isRunning()?)) {
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*System::pwd());
                            __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
                            ArcStr::from(__mm_s)
                        }
                    } else {
                        literal!("")
                    });
                    __mm_s.push_str(&*fileNamePrefix);
                    __mm_s.push_str(&*literal!(".xml"));
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("")
            },
        });
    }
    Ok((cache, outValue))
}

pub(crate) fn translateGraphics(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inMsg: &Absyn::Msg,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inMsg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut p: Absyn::Program;
            let mut retStr: ArcStr;
            let mut s1: ArcStr;
            let mut cls: metamodelica::Ref<Absyn::Class>;
            let mut refactoredClass: metamodelica::Ref<Absyn::Class>;
            let mut within_: Absyn::Within;
            p = SymbolTable::getAbsyn();
            cls = ProgramUtil::getPathedClassInProgram(className.clone(), &p, false, false)?;
            refactoredClass = Refactor::refactorGraphicalAnnotation(p.clone(), cls.clone())?;
            within_ = ProgramUtil::buildWithin(className.clone())?;
            SymbolTable::setAbsyn(ProgramUtil::updateProgram(
                Absyn::Program {
                    classes: list![refactoredClass.clone()],
                    within_: within_.clone(),
                },
                p.clone(),
                false,
                false,
            )?)?;
            s1 = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
            retStr = stringAppendList(list![
                literal!("Translation of "),
                s1.clone(),
                literal!(" successful.\n")
            ]);
            Ok(metamodelica::Ref::new(Values::Value::STRING { string: retStr.clone() }))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut errorMsg: ArcStr;
            let mut strEmpty: bool;
            errorMsg = Error::printMessagesStr(false);
            strEmpty = stringCompare(&(literal!("")), &errorMsg) == 0;
            errorMsg = if (strEmpty) {
                literal!("Internal error, translating graphics to new version")
            } else {
                errorMsg.clone()
            };
            Ok(metamodelica::Ref::new(Values::Value::STRING {
                string: errorMsg.clone(),
            }))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValue
}

fn calculateSimulationSettings(
    mut inCache: FCore::Cache,
    mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<(FCore::Cache, SimCode::SimulationSettings)> {
    let mut outCache: FCore::Cache;
    let mut outSimSettings: SimCode::SimulationSettings;
    (outCache, outSimSettings) = (::match_deref::match_deref! { match &(vals.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::ListNode::Cons { head: starttime_v, tail: Deref @ metamodelica::ListNode::Cons { head: stoptime_v, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: interval_i }, tail: Deref @ metamodelica::ListNode::Cons { head: tolerance_v, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: method_str }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: options_str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: outputFormat_str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: variableFilter_str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cflags }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: simflags }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } => {
            let mut cache = inCache;
            let mut starttime_r: metamodelica::Real;
            let mut stoptime_r: metamodelica::Real;
            let mut tolerance_r: metamodelica::Real;
            starttime_r = ValuesUtil::valueReal(metamodelica::AsArg::as_arg(&starttime_v))?;
            stoptime_r = ValuesUtil::valueReal(metamodelica::AsArg::as_arg(&stoptime_v))?;
            tolerance_r = ValuesUtil::valueReal(metamodelica::AsArg::as_arg(&tolerance_v))?;
            outSimSettings = SimCodeMain::createSimulationSettings(starttime_r, stoptime_r, interval_i.clone(), tolerance_r, method_str.clone(), options_str.clone(), outputFormat_str.clone(), variableFilter_str.clone(), cflags.clone(), simflags.clone())?;
            (cache, outSimSettings)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.calculateSimulationSettings failed: ")); __mm_s.push_str(&*ValuesDump::valString(&(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: vals })))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outSimSettings))
}

fn getListFirstShowError(
    mut inValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut errorMessage: ArcStr,
) -> Result<(
    metamodelica::Ref<Values::Value>,
    metamodelica::List<metamodelica::Ref<Values::Value>>,
)> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut restValues: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outValue, restValues) = (::match_deref::match_deref! { match inValues {
        Deref @ metamodelica::ListNode::Cons { head: v, tail: rest } => {
            (v.clone(), rest.clone())
        },
        Deref @ metamodelica::ListNode::Nil => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![errorMessage])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outValue, restValues))
}

fn getListNthShowError(
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut errorMessage: &ArcStr,
    mut currentElement: i32,
    mut nthElement: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inValues, currentElement, nthElement);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, i, n) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut rest: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let true = (i.clone() < n.clone()) else { return Err("pattern mismatch") };
                    (_, rest) = getListFirstShowError(metamodelica::AsArg::as_arg(&lst), errorMessage.clone())?;
                    v = getListNthShowError(rest.clone(), errorMessage, i.clone() + 1, n.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, _, _) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    (v, _) = getListFirstShowError(metamodelica::AsArg::as_arg(&lst), errorMessage.clone())?;
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValue)
}

fn moveClass(
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inOffset: i32,
    mut inProgram: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outSuccess: bool;
    let mut parent_cls: metamodelica::Ref<Absyn::Path>;
    let mut cls_name: ArcStr;
    if inOffset == 0 {
        outProgram = inProgram;
        outSuccess = true;
        return (outProgram, outSuccess);
    }
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName) {
            outProgram = unwrap_break_err!(moveClassInProgram(&(AbsynUtil::pathFirstIdent(inClassName)), inOffset, inProgram.clone()), '__try0);
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            parent_cls = metamodelica::Own::own(__pa1);
            cls_name = metamodelica::Own::own(__pa2);
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(&parent_cls, &inProgram, (std::sync::Arc::new({ let __pe_b0 = cls_name.clone(); let __pe_b1 = inOffset; move |__pe_a2| moveClassInClass(&__pe_b0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        }
        outSuccess = true;
        Ok::<_, &'static str>((outProgram.clone(), outSuccess.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outSuccess = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassToTop(
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut outSuccess: bool;
    let mut parent_cls: metamodelica::Ref<Absyn::Path>;
    let mut cls_name: ArcStr;
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName) {
            outProgram = (match outProgram.clone() {
                Absyn::Program { .. } => {
                    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    let mut cls: metamodelica::Ref<Absyn::Class>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue(AbsynUtil::pathFirstIdent(inClassName), outProgram.classes.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isClassNamed(&__a0, &__a1)) }), '__try0)) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    classes = metamodelica::Own::own(__pa0);
                    cls = metamodelica::Own::own(__pa1);
                    outProgram.classes = metamodelica::cons(cls.clone(), classes.clone());
                    outProgram.clone()
                }
            });
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            parent_cls = metamodelica::Own::own(__pa1);
            cls_name = metamodelica::Own::own(__pa2);
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(&parent_cls, &inProgram, (std::sync::Arc::new({ let __pe_b0 = cls_name.clone(); move |__pe_a1| moveClassToTopInClass(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        }
        outSuccess = true;
        Ok::<_, &'static str>((outSuccess.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSuccess = __try0_o0;
        }
        Err(_) => {
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassToBottom(
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut outSuccess: bool;
    let mut parent_cls: metamodelica::Ref<Absyn::Path>;
    let mut cls_name: ArcStr;
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName) {
            outProgram = (match outProgram.clone() {
                Absyn::Program { .. } => {
                    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    let mut cls: metamodelica::Ref<Absyn::Class>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue(AbsynUtil::pathFirstIdent(inClassName), outProgram.classes.clone(), &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isClassNamed(&__a0, &__a1)) }), '__try0)) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    classes = metamodelica::Own::own(__pa0);
                    cls = metamodelica::Own::own(__pa1);
                    outProgram.classes = listAppend(classes.clone(), list![cls.clone()]);
                    outProgram.clone()
                }
            });
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            parent_cls = metamodelica::Own::own(__pa1);
            cls_name = metamodelica::Own::own(__pa2);
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(&parent_cls, &inProgram, (std::sync::Arc::new({ let __pe_b0 = cls_name.clone(); move |__pe_a1| moveClassToBottomInClass(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> + 'static>)), '__try0);
        }
        outSuccess = true;
        Ok::<_, &'static str>((outSuccess.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSuccess = __try0_o0;
        }
        Err(_) => {
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassInProgram(mut inName: &ArcStr, mut inOffset: i32, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = inProgram;
    outProgram = (match outProgram.clone() {
        Absyn::Program { .. } => {
            outProgram.classes = moveClassInClassList(inName, inOffset, outProgram.classes.clone())?;
            outProgram
        }
    });
    Ok(outProgram)
}

fn moveClassInClassList(
    mut inName: &ArcStr,
    mut inOffset: i32,
    mut inClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Class>>> {
    let mut outClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut cls: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::Class>> = inClasses;
    let mut name: ArcStr;
    let mut offset: i32;
    loop {
        let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1 @ Deref @ Absyn::Class { name: __pa0, .. }, tail: __pa2 } => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        cls = metamodelica::Own::own(__pa1);
        rest = metamodelica::Own::own(__pa2);
        if metamodelica::stringEq(&name, &inName) {
            break;
        } else {
            acc = metamodelica::cons(cls.clone(), acc);
        }
    }
    if inOffset > 0 {
        offset = std::cmp::min(inOffset, ((rest).len() as i32));
        for mut i in 1..=offset {
            acc = metamodelica::cons((rest).head().cloned()?, acc);
            rest = (rest).rest()?;
        }
    } else {
        offset = std::cmp::max(inOffset, -((acc).len() as i32));
        for mut i in offset..=-1 {
            rest = metamodelica::cons((acc).head().cloned()?, rest);
            acc = (acc).rest()?;
        }
    }
    outClasses = List::append_reverse(&acc, metamodelica::cons(cls, rest));
    Ok(outClasses)
}

fn moveClassInClass(
    mut inName: &ArcStr,
    mut inOffset: i32,
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { body: __pa0, .. } = &*__arc1;
    body = metamodelica::Own::own(__pa0);
    body = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassInClassParts(inName, inOffset, __body_classParts.clone())?);
            body
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassInClassParts(inName, inOffset, __body_parts.clone())?);
            body
        }
        _ => return Err("match: no arm matched"),
    });
    outClass = AbsynUtil::setClassBody(inClass, body);
    Ok(outClass)
}

fn moveClassInClassParts(
    mut inName: &ArcStr,
    mut inOffset: i32,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = inClassParts.clone();
    let mut part: metamodelica::Ref<Absyn::ClassPart> =
        <metamodelica::Ref<Absyn::ClassPart> as ::std::default::Default>::default();
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = inClassParts.clone();
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut cls: Option<metamodelica::Ref<Absyn::ElementItem>> = None;
    let mut offset: i32 = 0;
    let mut is_public: bool = false;
    let mut is_empty: bool;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (part, cls, offset, is_public) = moveClassInClassPart(inName, inOffset, part)?;
        if (cls).is_some() {
            break;
        } else {
            acc = metamodelica::cons(part.clone(), acc);
        }
    }
    is_empty = AbsynUtil::isEmptyClassPart(&part);
    parts = if (offset > 0) { rest.clone() } else { acc.clone() };
    if (parts).is_empty() && offset != 0 {
        parts = moveClassInClassParts3(Util::getOption(cls)?, offset < 0, is_public, part, parts)?;
    } else {
        parts = moveClassInClassParts2(Util::getOption(cls)?, offset, is_public, parts)?;
        if !(is_empty) {
            parts = metamodelica::cons(part, parts);
        }
    }
    if offset > 0 {
        rest = parts;
    } else {
        acc = parts;
    }
    if is_empty && !((rest).is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa2);
        rest = metamodelica::Own::own(__pa3);
        acc = mergeClassPartWithList(part, acc);
    }
    outClassParts = List::append_reverse(&acc, rest);
    Ok(outClassParts)
}

fn mergeClassPartWithList(
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut part: metamodelica::Ref<Absyn::ClassPart>;
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outClassParts = (::match_deref::match_deref! { match &((inClassPart.clone(), inClassParts.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ metamodelica::ListNode::Cons { head: __esc_part @ Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: __esc_rest }) => {
            part = (*__esc_part).clone();
            rest = (*__esc_rest).clone();
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: listAppend(var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone()) }), rest.clone())
        },
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ metamodelica::ListNode::Cons { head: __esc_part @ Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: __esc_rest }) => {
            part = (*__esc_part).clone();
            rest = (*__esc_rest).clone();
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: listAppend(var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone()) }), rest.clone())
        },
        _ => metamodelica::cons(inClassPart, inClassParts),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClassParts
}

fn moveClassInClassParts2(
    mut inClass: metamodelica::Ref<Absyn::ElementItem>,
    mut inOffset: i32,
    mut inIsPublic: bool,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut part: metamodelica::Ref<Absyn::ClassPart>;
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = inClassParts;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut offset: i32 = inOffset;
    let mut moved: bool;
    while offset != 0 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (parts, offset, moved) = moveClassInClassPart3(inClass.clone(), offset, inIsPublic, part.clone())?;
        if (rest).is_empty() && !(moved) {
            acc = moveClassInClassParts3(inClass, inOffset > 0, inIsPublic, part, acc)?;
            break;
        } else if offset == 0 && !(moved) {
            acc = listAppend(parts, acc);
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            part = metamodelica::Own::own(__pa2);
            rest = metamodelica::Own::own(__pa3);
            acc = moveClassInClassParts3(inClass, inOffset > 0, inIsPublic, part, acc)?;
            break;
        }
        acc = listAppend(if (inOffset > 0) { parts } else { parts.reverse() }, acc);
    }
    outClassParts = List::append_reverse(&acc, rest);
    Ok(outClassParts)
}

fn moveClassInClassParts3(
    mut inClass: metamodelica::Ref<Absyn::ElementItem>,
    mut inPositiveOffset: bool,
    mut inIsPublic: bool,
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outClassParts = (::match_deref::match_deref! { match &((inPositiveOffset, inIsPublic, inClassPart.clone())) {
        (true, true, Deref @ Absyn::ClassPart::PUBLIC { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: metamodelica::cons(inClass, var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone()) }), inClassParts),
        (true, false, Deref @ Absyn::ClassPart::PROTECTED { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: metamodelica::cons(inClass, var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone()) }), inClassParts),
        (false, true, Deref @ Absyn::ClassPart::PUBLIC { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: listAppend(var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), list![inClass]) }), inClassParts),
        (false, false, Deref @ Absyn::ClassPart::PROTECTED { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: listAppend(var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), list![inClass]) }), inClassParts),
        (_, true, _) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![inClass] }), metamodelica::cons(inClassPart, inClassParts)),
        (_, false, _) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: list![inClass] }), metamodelica::cons(inClassPart, inClassParts)),
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClassParts)
}

fn moveClassInClassPart(
    mut inName: &ArcStr,
    mut inOffset: i32,
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<(
    metamodelica::Ref<Absyn::ClassPart>,
    Option<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
    bool,
)> {
    let mut outClassPart: metamodelica::Ref<Absyn::ClassPart> = inClassPart;
    let mut outClass: Option<metamodelica::Ref<Absyn::ElementItem>>;
    let mut outRemainingOffset: i32;
    let mut outIsPublic: bool;
    let mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    (outClassPart, outClass, outRemainingOffset, outIsPublic) = (match &*outClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __outClassPart_contents,
        } => {
            (elements, outClass, outRemainingOffset) =
                moveClassInClassPart2(inName, inOffset, __outClassPart_contents.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = elements);
            (outClassPart, outClass, outRemainingOffset, true)
        }
        Absyn::ClassPart::PROTECTED {
            contents: __outClassPart_contents,
        } => {
            (elements, outClass, outRemainingOffset) =
                moveClassInClassPart2(inName, inOffset, __outClassPart_contents.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = elements);
            (outClassPart, outClass, outRemainingOffset, false)
        }
        _ => (outClassPart, None, inOffset, false),
    });
    Ok((outClassPart, outClass, outRemainingOffset, outIsPublic))
}

fn moveClassInClassPart2(
    mut inName: &ArcStr,
    mut inOffset: i32,
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    Option<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut outClass: Option<metamodelica::Ref<Absyn::ElementItem>> = None;
    let mut outRemainingOffset: i32;
    let mut e: metamodelica::Ref<Absyn::ElementItem>;
    let mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = inElements.clone();
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    while !((elements).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elements) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        elements = metamodelica::Own::own(__pa1);
        if AbsynUtil::isElementItemClassNamed(inName, &e) {
            outClass = Some(e);
            break;
        } else {
            acc = metamodelica::cons(e, acc);
        }
    }
    if (outClass).is_none() {
        outElements = inElements;
        outRemainingOffset = inOffset;
        return Ok((outElements, outClass, outRemainingOffset));
    }
    (acc, elements, outRemainingOffset, _) = moveClassInSplitClassPart(inOffset, acc, elements)?;
    if outRemainingOffset == 0 {
        elements = metamodelica::cons(Util::getOption(outClass.clone())?, elements);
    }
    outElements = List::append_reverse(&acc, elements);
    Ok((outElements, outClass, outRemainingOffset))
}

fn makeClassPart(
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inPublic: bool,
) -> metamodelica::Ref<Absyn::ClassPart> {
    let mut outPart: metamodelica::Ref<Absyn::ClassPart> = if (inPublic) {
        metamodelica::Ref::new(Absyn::ClassPart::PUBLIC {
            contents: inElements.clone(),
        })
    } else {
        metamodelica::Ref::new(Absyn::ClassPart::PROTECTED {
            contents: inElements.clone(),
        })
    };
    outPart
}

fn moveClassInClassPart3(
    mut inClass: metamodelica::Ref<Absyn::ElementItem>,
    mut inOffset: i32,
    mut inIsPublic: bool,
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>, i32, bool)> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut outRemainingOffset: i32;
    let mut outMoved: bool = false;
    let mut same_part_type: bool;
    let mut reached_end: bool;
    let mut elems_before: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut elems_after: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    (elems, same_part_type) = (match &*inClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __inClassPart_contents,
        } => (__inClassPart_contents.clone(), inIsPublic),
        Absyn::ClassPart::PROTECTED {
            contents: __inClassPart_contents,
        } => (__inClassPart_contents.clone(), !(inIsPublic)),
        _ => return Err("match: no arm matched"),
    });
    if inOffset > 0 {
        elems_before = metamodelica::nil();
        elems_after = elems;
    } else {
        elems_before = elems;
        elems_after = metamodelica::nil();
    }
    (elems_before, elems_after, outRemainingOffset, reached_end) =
        moveClassInSplitClassPart(inOffset, elems_before.reverse(), elems_after)?;
    if outRemainingOffset == 0 {
        if same_part_type {
            elems = List::append_reverse(&elems_before, metamodelica::cons(inClass, elems_after));
            outClassParts = list![makeClassPart(elems, inIsPublic)];
            outMoved = true;
        } else if !(reached_end) {
            outClassParts = if ((elems_before).is_empty()) {
                metamodelica::nil()
            } else {
                list![makeClassPart(elems_before.reverse(), !(inIsPublic))]
            };
            outClassParts = metamodelica::cons(makeClassPart(list![inClass], inIsPublic), outClassParts);
            if !((elems_after).is_empty()) {
                outClassParts = metamodelica::cons(makeClassPart(elems_after, !(inIsPublic)), outClassParts);
            }
            outMoved = true;
        } else {
            outClassParts = list![inClassPart];
        }
    } else {
        outClassParts = list![inClassPart];
    }
    Ok((outClassParts, outRemainingOffset, outMoved))
}

fn moveClassInSplitClassPart(
    mut inOffset: i32,
    mut inElementsBefore: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inElementsAfter: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
    bool,
)> {
    let mut outElementsBefore: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = inElementsBefore;
    let mut outElementsAfter: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = inElementsAfter;
    let mut outRemainingOffset: i32 = inOffset;
    let mut outReachedEnd: bool;
    let mut e: metamodelica::Ref<Absyn::ElementItem>;
    if inOffset > 0 {
        while outRemainingOffset > 0 {
            if (outElementsAfter).is_empty() {
                break;
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outElementsAfter) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
                outElementsAfter = metamodelica::Own::own(__pa1);
                outElementsBefore = metamodelica::cons(e.clone(), outElementsBefore);
                if AbsynUtil::isElementItemClass(&e) {
                    outRemainingOffset = outRemainingOffset - 1;
                }
            }
        }
        outReachedEnd = (outElementsAfter).is_empty();
    } else {
        while outRemainingOffset < 0 {
            if (outElementsBefore).is_empty() {
                break;
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(outElementsBefore) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa2);
                outElementsBefore = metamodelica::Own::own(__pa3);
                outElementsAfter = metamodelica::cons(e.clone(), outElementsAfter);
                if AbsynUtil::isElementItemClass(&e) {
                    outRemainingOffset = outRemainingOffset + 1;
                }
            }
        }
        outReachedEnd = (outElementsBefore).is_empty();
    }
    Ok((outElementsBefore, outElementsAfter, outRemainingOffset, outReachedEnd))
}

fn deleteClassInClassPart(
    mut inName: ArcStr,
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<(
    metamodelica::Ref<Absyn::ClassPart>,
    Option<metamodelica::Ref<Absyn::ElementItem>>,
)> {
    let mut outClassPart: metamodelica::Ref<Absyn::ClassPart> = inClassPart;
    let mut outClass: Option<metamodelica::Ref<Absyn::ElementItem>>;
    let mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    (outClassPart, outClass) = (match &*outClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __outClassPart_contents,
        } => {
            (elements, outClass) = List::deleteMemberOnTrue(
                inName,
                __outClassPart_contents.clone(),
                &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::ElementItem>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::isElementItemClassNamed(&__a0, &__a1))
                },
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = elements);
            (outClassPart, outClass)
        }
        Absyn::ClassPart::PROTECTED {
            contents: __outClassPart_contents,
        } => {
            (elements, outClass) = List::deleteMemberOnTrue(
                inName,
                __outClassPart_contents.clone(),
                &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::ElementItem>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::isElementItemClassNamed(&__a0, &__a1))
                },
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = elements);
            (outClassPart, outClass)
        }
        _ => (outClassPart, None),
    });
    Ok((outClassPart, outClass))
}

fn moveClassToTopInClass(
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { body: __pa0, .. } = &*__arc1;
    body = metamodelica::Own::own(__pa0);
    body = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassToTopInClassParts(inName, __body_classParts.clone())?);
            body
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassToTopInClassParts(inName, __body_parts.clone())?);
            body
        }
        _ => return Err("match: no arm matched"),
    });
    outClass = AbsynUtil::setClassBody(inClass, body);
    Ok(outClass)
}

fn moveClassToTopInClassParts(
    mut inName: ArcStr,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut part: metamodelica::Ref<Absyn::ClassPart> =
        <metamodelica::Ref<Absyn::ClassPart> as ::std::default::Default>::default();
    let mut first: metamodelica::Ref<Absyn::ClassPart>;
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = inClassParts;
    let mut ocls: Option<metamodelica::Ref<Absyn::ElementItem>> = None;
    let mut cls: metamodelica::Ref<Absyn::ElementItem>;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (part, ocls) = deleteClassInClassPart(inName.clone(), part)?;
        if (ocls).is_some() {
            if !(AbsynUtil::isEmptyClassPart(&part)) || (acc).is_empty() || (rest).is_empty() {
                rest = metamodelica::cons(part.clone(), rest);
            }
            outClassParts = List::append_reverse(&acc, rest);
            break;
        } else {
            acc = metamodelica::cons(part.clone(), acc);
        }
    }
    let __pa2 = ::match_deref::match_deref! { match &(ocls) {
        Some(__pa2) => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cls = metamodelica::Own::own(__pa2);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(outClassParts) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    first = metamodelica::Own::own(__pa3);
    rest = metamodelica::Own::own(__pa4);
    outClassParts = (::match_deref::match_deref! { match &((first.clone(), part)) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
            assign_variant_field!(first => Absyn::ClassPart::PUBLIC; contents = metamodelica::cons(cls, var_field!((*first).contents, Absyn::ClassPart::PUBLIC).clone()));
            metamodelica::cons(first, rest)
        },
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
            assign_variant_field!(first => Absyn::ClassPart::PROTECTED; contents = metamodelica::cons(cls, var_field!((*first).contents, Absyn::ClassPart::PROTECTED).clone()));
            metamodelica::cons(first, rest)
        },
        (_, Deref @ Absyn::ClassPart::PUBLIC { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![cls] }), metamodelica::cons(first, rest)),
        (_, Deref @ Absyn::ClassPart::PROTECTED { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: list![cls] }), metamodelica::cons(first, rest)),
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClassParts)
}

fn moveClassToBottomInClass(
    mut inName: ArcStr,
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let __arc1 = inClass.clone();
    let Absyn::CLASS { body: __pa0, .. } = &*__arc1;
    body = metamodelica::Own::own(__pa0);
    body = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassToBottomInClassParts(inName, __body_classParts.clone())?);
            body
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassToBottomInClassParts(inName, __body_parts.clone())?);
            body
        }
        _ => return Err("match: no arm matched"),
    });
    outClass = AbsynUtil::setClassBody(inClass, body);
    Ok(outClass)
}

fn moveClassToBottomInClassParts(
    mut inName: ArcStr,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut part: metamodelica::Ref<Absyn::ClassPart> =
        <metamodelica::Ref<Absyn::ClassPart> as ::std::default::Default>::default();
    let mut last: metamodelica::Ref<Absyn::ClassPart>;
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = inClassParts;
    let mut ocls: Option<metamodelica::Ref<Absyn::ElementItem>> = None;
    let mut cls: metamodelica::Ref<Absyn::ElementItem>;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (part, ocls) = deleteClassInClassPart(inName.clone(), part)?;
        if (ocls).is_some() {
            break;
        } else {
            acc = metamodelica::cons(part.clone(), acc);
        }
    }
    let __pa2 = ::match_deref::match_deref! { match &(ocls) {
        Some(__pa2) => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cls = metamodelica::Own::own(__pa2);
    if !(AbsynUtil::isEmptyClassPart(&part)) || (rest).is_empty() {
        rest = metamodelica::cons(part.clone(), rest);
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    last = metamodelica::Own::own(__pa3);
    rest = metamodelica::Own::own(__pa4);
    rest = (::match_deref::match_deref! { match &((last.clone(), part)) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
            assign_variant_field!(last => Absyn::ClassPart::PUBLIC; contents = listAppend(var_field!((*last).contents, Absyn::ClassPart::PUBLIC).clone(), list![cls]));
            metamodelica::cons(last, rest)
        },
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
            assign_variant_field!(last => Absyn::ClassPart::PROTECTED; contents = listAppend(var_field!((*last).contents, Absyn::ClassPart::PROTECTED).clone(), list![cls]));
            metamodelica::cons(last, rest)
        },
        (_, Deref @ Absyn::ClassPart::PUBLIC { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![cls] }), metamodelica::cons(last, rest)),
        (_, Deref @ Absyn::ClassPart::PROTECTED { .. }) => metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: list![cls] }), metamodelica::cons(last, rest)),
        _ => return Err("match: no arm matched"),
    } });
    outClassParts = List::append_reverse(&acc, rest.reverse());
    Ok(outClassParts)
}

fn copyClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inName: ArcStr,
    mut inWithin: Absyn::Within,
    mut inClassPath: metamodelica::Ref<Absyn::Path>,
    mut inProg: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut outProg: Absyn::Program;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut orig_file: ArcStr;
    let mut dst_path: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    orig_file = metamodelica::Own::own(__pa0);
    dst_path = (match inWithin.clone() {
        Absyn::Within::TOP { .. } => literal!("<interactive>"),
        Absyn::Within::WITHIN { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(var_field!(inWithin.path, Absyn::Within::WITHIN).clone(), &inProg, false, false)?) {
                Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dst_path = metamodelica::Own::own(__pa0);
            dst_path
        }
    });
    cls = NFApi::updateMovedClassPaths(inClass, inClassPath, &inWithin)?;
    cls = moveClassInfo(cls, dst_path)?;
    cls = AbsynUtil::setClassName(cls, inName);
    outProg = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![cls],
            within_: inWithin,
        },
        inProg,
        false,
        false,
    )?;
    Ok(outProg)
}

fn moveSourceInfo(mut inInfo: SourceInfo, mut dstPath: ArcStr) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo = inInfo;
    let () = (match outInfo.clone() {
        SourceInfo { .. } => {
            outInfo.fileName = dstPath;
            outInfo.isReadOnly = false;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outInfo)
}

fn moveClassInfo(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass;
    let mut info: SourceInfo;
    let () = (match &*outClass.clone() {
        Absyn::Class {
            info: __esc_info @ SourceInfo { .. },
            ..
        } => {
            info = (*__esc_info).clone();
            assign_field!(
                outClass.body = moveClassDefInfo(outClass.body.clone(), dstPath.clone())?,
                outClass.info = moveSourceInfo(info.clone(), dstPath)?
            );
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outClass)
}

fn moveClassDefInfo(
    mut inClassDef: metamodelica::Ref<Absyn::ClassDef>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut outClassDef: metamodelica::Ref<Absyn::ClassDef> = inClassDef;
    let () = (match &*outClassDef {
        Absyn::ClassDef::PARTS {
            classParts: __outClassDef_classParts,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::PARTS;
                        classParts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut cp in (__outClassDef_classParts.clone()).into_iter().cloned() {
                    let __x = moveClassPartInfo(cp.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*outClassDef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = moveAnnotationInfo(a.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::DERIVED {
            arguments: __outClassDef_arguments,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::DERIVED;
                        arguments = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut e in (__outClassDef_arguments.clone()).into_iter().cloned() {
                    let __x = moveElementArgInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = moveCommentInfo(var_field!((*outClassDef).comment, Absyn::ClassDef::DERIVED).clone(), dstPath)?
                    );
            ()
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __outClassDef_comment,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::ENUMERATION; comment = moveCommentInfo(__outClassDef_comment.clone(), dstPath)?);
            ()
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __outClassDef_comment,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::OVERLOAD; comment = moveCommentInfo(__outClassDef_comment.clone(), dstPath)?);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            modifications: __outClassDef_modifications,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::CLASS_EXTENDS;
                        modifications = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut e in (__outClassDef_modifications.clone()).into_iter().cloned() {
                    let __x = moveElementArgInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        parts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut cp in (var_field!((*outClassDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = moveClassPartInfo(cp.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*outClassDef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = moveAnnotationInfo(a.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::PDER {
            comment: __outClassDef_comment,
            ..
        } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::PDER; comment = moveCommentInfo(__outClassDef_comment.clone(), dstPath)?);
            ()
        }
        _ => (),
    });
    Ok(outClassDef)
}

fn moveClassPartInfo(
    mut inPart: metamodelica::Ref<Absyn::ClassPart>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut outPart: metamodelica::Ref<Absyn::ClassPart>;
    outPart = (match &*inPart {
        Absyn::ClassPart::PUBLIC { contents: el } => metamodelica::Ref::new(Absyn::ClassPart::PUBLIC {
            contents: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut e in (el.clone()).into_iter().cloned() {
                    let __x = moveElementItemInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        Absyn::ClassPart::PROTECTED { contents: el } => metamodelica::Ref::new(Absyn::ClassPart::PROTECTED {
            contents: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut e in (el.clone()).into_iter().cloned() {
                    let __x = moveElementItemInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        Absyn::ClassPart::EQUATIONS { contents: eq } => metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS {
            contents: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
                for mut e in (eq.clone()).into_iter().cloned() {
                    let __x = moveEquationItemInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        Absyn::ClassPart::INITIALEQUATIONS { contents: eq } => {
            metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS {
                contents: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
                    for mut e in (eq.clone()).into_iter().cloned() {
                        let __x = moveEquationItemInfo(e.clone(), dstPath.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            })
        }
        Absyn::ClassPart::ALGORITHMS { contents: alg } => metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS {
            contents: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = metamodelica::nil();
                for mut e in (alg.clone()).into_iter().cloned() {
                    let __x = moveAlgorithmItemInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        Absyn::ClassPart::INITIALALGORITHMS { contents: alg } => {
            metamodelica::Ref::new(Absyn::ClassPart::INITIALALGORITHMS {
                contents: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = metamodelica::nil();
                    for mut e in (alg.clone()).into_iter().cloned() {
                        let __x = moveAlgorithmItemInfo(e.clone(), dstPath.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            })
        }
        Absyn::ClassPart::EXTERNAL {
            externalDecl: ext,
            annotation_: ann,
        } => {
            let mut ext = (*ext).clone();
            let mut ann = (*ann).clone();
            ext = moveExternalDeclInfo(ext.clone(), dstPath.clone())?;
            ann = moveAnnotationOptInfo(ann.clone(), dstPath)?;
            metamodelica::Ref::new(Absyn::ClassPart::EXTERNAL {
                externalDecl: ext.clone(),
                annotation_: ann.clone(),
            })
        }
        _ => inPart,
    });
    Ok(outPart)
}

fn moveAnnotationOptInfo(
    mut inAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>,
    mut dstPath: ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(inAnnotation.clone()) {
        Some(a) => {
            Some(moveAnnotationInfo(a.clone(), dstPath)?)
        },
        _ => {
            inAnnotation
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

fn moveAnnotationInfo(
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut outAnnotation: metamodelica::Ref<Absyn::Annotation> = inAnnotation;
    assign_field!(
        outAnnotation.elementArgs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
            for mut e in (outAnnotation.elementArgs.clone()).into_iter().cloned() {
                let __x = moveElementArgInfo(e.clone(), dstPath.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(outAnnotation)
}

fn moveElementItemInfo(
    mut inElement: metamodelica::Ref<Absyn::ElementItem>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outElement: metamodelica::Ref<Absyn::ElementItem>;
    outElement = (match &*inElement {
        Absyn::ElementItem::ELEMENTITEM {
            element: __inElement_element,
        } => metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM {
            element: moveElementInfo(__inElement_element.clone(), dstPath)?,
        }),
        _ => inElement,
    });
    Ok(outElement)
}

fn moveElementInfo(
    mut inElement: metamodelica::Ref<Absyn::Element>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut outElement: metamodelica::Ref<Absyn::Element> = inElement;
    let () = (match &*outElement {
        Absyn::Element::ELEMENT {
            specification: __outElement_specification,
            ..
        } => {
            assign_variant_field!(outElement => Absyn::Element::ELEMENT;
                specification = moveElementSpecInfo(__outElement_specification.clone(), dstPath.clone())?,
                constrainClass = moveConstrainClassInfo(var_field!((*outElement).constrainClass, Absyn::Element::ELEMENT).clone(), dstPath.clone())?,
                info = moveSourceInfo(var_field!((*outElement).info, Absyn::Element::ELEMENT).clone(), dstPath)?
            );
            ()
        }
        Absyn::Element::TEXT {
            info: __outElement_info,
            ..
        } => {
            assign_variant_field!(outElement => Absyn::Element::TEXT; info = moveSourceInfo(__outElement_info.clone(), dstPath)?);
            ()
        }
        _ => (),
    });
    Ok(outElement)
}

fn moveElementArgInfo(
    mut inArg: metamodelica::Ref<Absyn::ElementArg>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outArg: metamodelica::Ref<Absyn::ElementArg> = inArg;
    let () = (match &*outArg {
        Absyn::ElementArg::MODIFICATION {
            modification: __outArg_modification,
            ..
        } => {
            assign_variant_field!(outArg => Absyn::ElementArg::MODIFICATION;
                modification = moveModificationInfo(__outArg_modification.clone(), dstPath.clone())?,
                info = moveSourceInfo(var_field!((*outArg).info, Absyn::ElementArg::MODIFICATION).clone(), dstPath)?
            );
            ()
        }
        Absyn::ElementArg::REDECLARATION {
            elementSpec: __outArg_elementSpec,
            ..
        } => {
            assign_variant_field!(outArg => Absyn::ElementArg::REDECLARATION;
                elementSpec = moveElementSpecInfo(__outArg_elementSpec.clone(), dstPath.clone())?,
                constrainClass = moveConstrainClassInfo(var_field!((*outArg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), dstPath.clone())?,
                info = moveSourceInfo(var_field!((*outArg).info, Absyn::ElementArg::REDECLARATION).clone(), dstPath)?
            );
            ()
        }
        _ => (),
    });
    Ok(outArg)
}

fn moveModificationInfo(
    mut inMod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut dstPath: ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: el, eqMod: eq }) => {
            let mut el = (*el).clone();
            let mut eq = (*eq).clone();
            el = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), dstPath.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq = moveEqModInfo(eq.clone(), dstPath)?;
            Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: el.clone(), eqMod: eq.clone() }))
        },
        _ => {
            inMod
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn moveEqModInfo(
    mut inEqMod: metamodelica::Ref<Absyn::EqMod>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::EqMod>> {
    let mut outEqMod: metamodelica::Ref<Absyn::EqMod>;
    outEqMod = (match &*inEqMod {
        Absyn::EqMod::EQMOD {
            exp: __inEqMod_exp,
            info: __inEqMod_info,
        } => metamodelica::Ref::new(Absyn::EqMod::EQMOD {
            exp: __inEqMod_exp.clone(),
            info: moveSourceInfo(__inEqMod_info.clone(), dstPath)?,
        }),
        _ => inEqMod,
    });
    Ok(outEqMod)
}

fn moveConstrainClassInfo(
    mut inCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut dstPath: ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::ConstrainClass>>> {
    let mut outCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    outCC = (::match_deref::match_deref! { match &(inCC.clone()) {
        Some(Deref @ Absyn::ConstrainClass { elementSpec: spec, comment: cmt }) => {
            let mut spec = (*spec).clone();
            let mut cmt = (*cmt).clone();
            spec = moveElementSpecInfo(spec.clone(), dstPath.clone())?;
            cmt = moveCommentInfo(cmt.clone(), dstPath)?;
            Some(metamodelica::Ref::new(Absyn::ConstrainClass { elementSpec: spec.clone(), comment: cmt.clone() }))
        },
        _ => {
            inCC
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCC)
}

fn moveCommentInfo(
    mut inComment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut dstPath: ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut outComment: Option<metamodelica::Ref<Absyn::Comment>>;
    outComment = (::match_deref::match_deref! { match &(inComment.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: Some(a), comment: c }) => {
            let mut a = (*a).clone();
            a = moveAnnotationInfo(a.clone(), dstPath)?;
            Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(a.clone()), comment: c.clone() }))
        },
        _ => {
            inComment
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

fn moveEquationItemInfo(
    mut inEquation: metamodelica::Ref<Absyn::EquationItem>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::EquationItem>> {
    let mut outEquation: metamodelica::Ref<Absyn::EquationItem>;
    outEquation = (match &*inEquation {
        Absyn::EquationItem::EQUATIONITEM {
            equation_: eq,
            comment: cmt,
            info,
        } => {
            let mut cmt = (*cmt).clone();
            let mut info = (*info).clone();
            cmt = moveCommentInfo(cmt.clone(), dstPath.clone())?;
            info = moveSourceInfo(info.clone(), dstPath)?;
            metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM {
                equation_: eq.clone(),
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        _ => inEquation,
    });
    Ok(outEquation)
}

fn moveAlgorithmItemInfo(
    mut inAlgorithm: metamodelica::Ref<Absyn::AlgorithmItem>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::AlgorithmItem>> {
    let mut outAlgorithm: metamodelica::Ref<Absyn::AlgorithmItem>;
    outAlgorithm = (match &*inAlgorithm {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: alg,
            comment: cmt,
            info,
        } => {
            let mut cmt = (*cmt).clone();
            let mut info = (*info).clone();
            cmt = moveCommentInfo(cmt.clone(), dstPath.clone())?;
            info = moveSourceInfo(info.clone(), dstPath)?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM {
                algorithm_: alg.clone(),
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        _ => inAlgorithm,
    });
    Ok(outAlgorithm)
}

fn moveElementSpecInfo(
    mut inSpec: metamodelica::Ref<Absyn::ElementSpec>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outSpec: metamodelica::Ref<Absyn::ElementSpec> = inSpec;
    let () = (match &*outSpec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __outSpec_class_,
            ..
        } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::CLASSDEF; class_ = moveClassInfo(__outSpec_class_.clone(), dstPath)?);
            ()
        }
        Absyn::ElementSpec::EXTENDS {
            elementArg: __outSpec_elementArg,
            ..
        } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::EXTENDS;
                        elementArg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut e in (__outSpec_elementArg.clone()).into_iter().cloned() {
                    let __x = moveElementArgInfo(e.clone(), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        annotationOpt = moveAnnotationOptInfo(var_field!((*outSpec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), dstPath)?
                    );
            ()
        }
        Absyn::ElementSpec::IMPORT {
            comment: __outSpec_comment,
            ..
        } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::IMPORT;
                comment = moveCommentInfo(__outSpec_comment.clone(), dstPath.clone())?,
                info = moveSourceInfo(var_field!((*outSpec).info, Absyn::ElementSpec::IMPORT).clone(), dstPath)?
            );
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            components: __outSpec_components,
            ..
        } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::COMPONENTS; components = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
                for mut c in (__outSpec_components.clone()).into_iter().cloned() {
                    let __x = moveComponentItemInfo(&(c.clone()), dstPath.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(outSpec)
}

fn moveComponentItemInfo(
    mut inComponent: &metamodelica::Ref<Absyn::ComponentItem>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut outComponent: metamodelica::Ref<Absyn::ComponentItem>;
    let mut comp: Absyn::Component;
    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    let __arc3 = &(*inComponent);
    let Absyn::COMPONENTITEM {
        component: __pa0,
        condition: __pa1,
        comment: __pa2,
    } = &**__arc3;
    comp = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    cmt = metamodelica::Own::own(__pa2);
    comp = moveComponentInfo(comp, dstPath.clone())?;
    cmt = moveCommentInfo(cmt, dstPath)?;
    outComponent = metamodelica::Ref::new(Absyn::ComponentItem {
        component: comp,
        condition: cond,
        comment: cmt,
    });
    Ok(outComponent)
}

fn moveComponentInfo(mut inComponent: Absyn::Component, mut dstPath: ArcStr) -> Result<Absyn::Component> {
    let mut outComponent: Absyn::Component = inComponent;
    outComponent.modification = moveModificationInfo(outComponent.modification.clone(), dstPath)?;
    Ok(outComponent)
}

fn moveExternalDeclInfo(
    mut inExtDecl: metamodelica::Ref<Absyn::ExternalDecl>,
    mut dstPath: ArcStr,
) -> Result<metamodelica::Ref<Absyn::ExternalDecl>> {
    let mut outExtDecl: metamodelica::Ref<Absyn::ExternalDecl> = inExtDecl;
    assign_field!(outExtDecl.annotation_ = moveAnnotationOptInfo(outExtDecl.annotation_.clone(), dstPath)?);
    Ok(outExtDecl)
}

fn buildModel(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inMsg: &Absyn::Msg,
) -> Result<(
    bool,
    FCore::Cache,
    ArcStr,
    ArcStr,
    ArcStr,
    ArcStr,
    ArcStr,
    ArcStr,
    metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
    metamodelica::List<metamodelica::Ref<Values::Value>>,
    metamodelica::List<ArcStr>,
)> {
    let mut success: bool = false;
    let mut outCache: FCore::Cache;
    let mut compileDir: ArcStr = arcstr::literal!("");
    let mut outString1: ArcStr;
    let mut outString2: ArcStr;
    let mut outputFormat_str: ArcStr = arcstr::literal!("");
    let mut outInitFileName: ArcStr;
    let mut outSimFlags: ArcStr;
    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)> = metamodelica::nil();
    let mut outArgs: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut outLibsAndLibDirs: metamodelica::List<ArcStr>;
    (
        outCache,
        compileDir,
        outString1,
        outString2,
        outputFormat_str,
        outInitFileName,
        outSimFlags,
        resultValues,
        outArgs,
        outLibsAndLibDirs,
    ) = 'mc: {
        let __mc_input = (inCache, inEnv, inValues.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, vals) => {
                    let mut libsAndLibDirs: metamodelica::List<ArcStr>;
                    let mut file_dir: ArcStr;
                    let mut init_filename: ArcStr;
                    let mut method_str: ArcStr;
                    let mut filenameprefix: ArcStr;
                    let mut simflags: ArcStr;
                    let mut classname: metamodelica::Ref<Absyn::Path>;
                    let mut timeCompile: metamodelica::Real;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut simflags_mod: Option<metamodelica::Ref<Absyn::Modification>>;
                    let mut cache = (*cache).clone();
                    let mut vals = (*vals).clone();
                    let mut compileDir: ArcStr = compileDir.clone();
                    let mut outputFormat_str: ArcStr = outputFormat_str.clone();
                    let mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)> = resultValues.clone();
                    let mut success: bool = success.clone();
                    values = vals.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the className (1 arg) from the buildModel arguments"))?) {
                        (Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    classname = metamodelica::Own::own(__pa0);
                    vals = metamodelica::Own::own(__pa1);
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the startTime (2 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the stopTime (3 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the numberOfIntervals (4 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the tolerance (5 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the method (6 arg) from the buildModel arguments"))?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retreaving the fileNamePrefix (7 arg) from the buildModel arguments"))?) {
                        (Deref @ Values::Value::STRING { string: __pa3 }, __pa4) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    filenameprefix = metamodelica::Own::own(__pa3);
                    vals = metamodelica::Own::own(__pa4);
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the options (8 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the outputFormat (9 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the variableFilter (10 arg) from the buildModel arguments"))?;
                    (_, vals) = getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the cflags (11 arg) from the buildModel arguments"))?;
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(getListFirstShowError(metamodelica::AsArg::as_arg(&vals), literal!("while retrieving the simflags (12 arg) from the buildModel arguments"))?) {
                        (Deref @ Values::Value::STRING { string: __pa6 }, __pa7) => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    simflags = metamodelica::Own::own(__pa6);
                    vals = metamodelica::Own::own(__pa7);
                    Error::clearMessages();
                    if stringEmpty(&simflags) && !(Flags::getConfigBool(Flags::IGNORE_SIMULATION_FLAGS_ANNOTATION.clone())?) {
                        loadProgram(&classname)?;
                        simflags_mod = ProgramUtil::getNamedAnnotationExp(classname.clone(), SymbolTable::getAbsyn(), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("__OpenModelica_simulationFlags") })), Some(None), &fnptr!(Util::id, _))?;
                        simflags = formatSimulationFlagsString(simflags_mod.clone())?;
                        if !(stringEmpty(&simflags)) {
                            values = List::replaceAt(metamodelica::Ref::new(Values::Value::STRING { string: simflags.clone() }), 12, values.clone())?;
                        }
                    }
                    compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                    (cache, simSettings) = calculateSimulationSettings(cache.clone(), values.clone())?;
                    let SimCode::SIMULATION_SETTINGS { method: __pa9, outputFormat: __pa10, .. } = &simSettings;
                    method_str = metamodelica::Own::own(__pa9);
                    outputFormat_str = metamodelica::Own::own(__pa10);
                    (success, cache, libsAndLibDirs, file_dir, resultValues) = translateModel(cache.clone(), env.clone(), classname.clone(), filenameprefix.clone(), true, true, Some(simSettings.clone()))?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
                    init_filename = { let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) };
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildModel: about to compile model ")); __mm_s.push_str(&*filenameprefix); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*file_dir); ArcStr::from(__mm_s) })?;
                    }
                    if success {
                        if '__try11: {
                            if metamodelica::stringEq(&(unwrap_break_err!(Config::simCodeTarget(), '__try11)), &(literal!("wasm-jit"))) {
                                        unwrap_break_err!(CodegenWasmJit::finishCompile(filenameprefix.clone()), '__try11);
                            } else if metamodelica::stringEq(&(unwrap_break_err!(Config::simCodeTarget(), '__try11)), &(literal!("wasm"))) {
                            } else {
                                        unwrap_break_err!(CevalScript::compileModel(filenameprefix.clone(), libsAndLibDirs.clone(), literal!(""), metamodelica::nil()), '__try11);
                            }
                            Ok::<(), &'static str>(())
                        }.is_err() {
                            success = false;
                        }
                        timeCompile = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
                    } else {
                        timeCompile = metamodelica::OrderedFloat(0.0_f64);
                    }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::trace(literal!("buildModel: Compiling done.\n"))?;
                    }
                    resultValues = metamodelica::cons((literal!("timeCompile"), metamodelica::Ref::new(Values::Value::REAL { real: timeCompile })), resultValues.clone());
                    Ok(((cache.clone(), compileDir.clone(), filenameprefix.clone(), method_str.clone(), outputFormat_str.clone(), init_filename.clone(), simflags.clone(), resultValues.clone(), values.clone(), libsAndLibDirs.clone()), compileDir.clone(), outputFormat_str.clone(), resultValues.clone(), success.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            compileDir = __wb0;
            outputFormat_str = __wb1;
            resultValues = __wb2;
            success = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(((inValues).len() as i32) == 12, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildModel failure, length = ")); __mm_s.push_str(&*intString(((inValues).len() as i32))); ArcStr::from(__mm_s) }, &(Absyn::dummyInfo.clone()))?;
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
        success,
        outCache,
        compileDir,
        outString1,
        outString2,
        outputFormat_str,
        outInitFileName,
        outSimFlags,
        resultValues,
        outArgs,
        outLibsAndLibDirs,
    ))
}

fn formatSimulationFlagsString(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: args, .. }) => {
            List::toStringCustom(args.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementArg>| formatSimulationFlagString(&__a0), literal!(""), literal!("-"), literal!(" -"), literal!(""), false, 0)?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

fn formatSimulationFlagString(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: __arg_path, .. } => {
            (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::STRING { value: Deref @ "()" } => AbsynUtil::pathString(__arg_path.clone(), literal!("."), true, false)?,
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(__arg_path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Absyn::ElementArg::MODIFICATION { path: __arg_path, .. } => {
            AbsynUtil::pathString(__arg_path.clone(), literal!("."), true, false)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

fn createSimulationResultFromcallModelExecutable(
    mut buildSuccess: bool,
    mut callRet: i32,
    mut timeTotal: metamodelica::Real,
    mut timeSimulation: metamodelica::Real,
    mut resultValues: metamodelica::List<(ArcStr, metamodelica::Ref<Values::Value>)>,
    mut inCache: FCore::Cache,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inVals: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut result_file: ArcStr,
    mut logFile: ArcStr,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (buildSuccess, callRet);
        if let Ok(__v) = (|| -> Result<_> {
            let (false, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut simValue: metamodelica::Ref<Values::Value>;
            simValue = createSimulationResult(
                result_file.clone(),
                simOptionsAsString(inVals)?,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to build model: "));
                    __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::cons(
                    (
                        literal!("timeTotal"),
                        metamodelica::Ref::new(Values::Value::REAL { real: timeTotal }),
                    ),
                    metamodelica::cons(
                        (
                            literal!("timeSimulation"),
                            metamodelica::Ref::new(Values::Value::REAL { real: timeSimulation }),
                        ),
                        resultValues.clone(),
                    ),
                ),
            )?;
            Ok((inCache.clone(), simValue.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, 0) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut simValue: metamodelica::Ref<Values::Value>;
            simValue = createSimulationResult(
                result_file.clone(),
                simOptionsAsString(inVals)?,
                System::readFile(logFile.clone())?,
                metamodelica::cons(
                    (
                        literal!("timeTotal"),
                        metamodelica::Ref::new(Values::Value::REAL { real: timeTotal }),
                    ),
                    metamodelica::cons(
                        (
                            literal!("timeSimulation"),
                            metamodelica::Ref::new(Values::Value::REAL { real: timeSimulation }),
                        ),
                        resultValues.clone(),
                    ),
                ),
            )?;
            SymbolTable::addVar(
                &(metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: literal!("currentSimulationResult"),
                    identType: DAE::T_STRING_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                })),
                metamodelica::Ref::new(Values::Value::STRING {
                    string: result_file.clone(),
                }),
                &(FGraph::empty()),
            )?;
            Ok((inCache.clone(), simValue.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut res: ArcStr;
            let mut r#str: ArcStr;
            let mut simValue: metamodelica::Ref<Values::Value>;
            res = if (System::regularFileExists(logFile.clone())) {
                System::readFile(logFile.clone())?
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*logFile);
                    __mm_s.push_str(&*literal!(" does not exist"));
                    ArcStr::from(__mm_s)
                }
            };
            r#str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
            res = stringAppendList(list![
                literal!("Simulation execution failed for model: "),
                r#str.clone(),
                literal!("\n"),
                res.clone()
            ]);
            simValue = createSimulationResult(
                literal!(""),
                simOptionsAsString(inVals)?,
                res.clone(),
                metamodelica::cons(
                    (
                        literal!("timeTotal"),
                        metamodelica::Ref::new(Values::Value::REAL { real: timeTotal }),
                    ),
                    metamodelica::cons(
                        (
                            literal!("timeSimulation"),
                            metamodelica::Ref::new(Values::Value::REAL { real: timeSimulation }),
                        ),
                        resultValues.clone(),
                    ),
                ),
            )?;
            Ok((inCache.clone(), simValue.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn checkModel(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inMsg: &Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut odae: Option<DAE::DAElist>;
            let mut dae: DAE::DAElist;
            let mut eqnSize: i32;
            let mut varSize: i32;
            let mut simpleEqnSize: i32;
            let mut retStr: ArcStr;
            let mut classNameStr: ArcStr;
            let mut flags: Flags::Flag;
            let mut cache: FCore::Cache = cache.clone();
            ExecStat::execStatReset()?;
            flags = loadCommandLineOptionsFromModel(className.clone())?;
            match '__try0: {
                (cache, _, odae, _) = unwrap_break_err!(runFrontEnd(cache.clone(), env.clone(), className.clone(), false, false, false), '__try0);
                let __pa1 = ::match_deref::match_deref! { match &(odae.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                dae = metamodelica::Own::own(__pa1);
                (varSize, eqnSize, simpleEqnSize) = unwrap_break_err!(CheckModel::checkModel(dae.clone()), '__try0);
                FlagsUtil::saveFlags(flags.clone());
                Ok::<_, &'static str>((
                    cache.clone(),
                    dae.clone(),
                    eqnSize.clone(),
                    odae.clone(),
                    simpleEqnSize.clone(),
                    varSize.clone(),
                ))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
                    cache = __try0_o0;
                    dae = __try0_o1;
                    eqnSize = __try0_o2;
                    odae = __try0_o3;
                    simpleEqnSize = __try0_o4;
                    varSize = __try0_o5;
                }
                Err(__try0_err) => {
                    FlagsUtil::saveFlags(flags.clone());
                    return Err(__try0_err);
                }
            }
            classNameStr = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
            retStr = stringAppendList(list![
                literal!("Check of "),
                classNameStr.clone(),
                literal!(" completed successfully.\nClass "),
                classNameStr.clone(),
                literal!(" has "),
                ArcStr::from(::std::format!("{}", eqnSize)),
                literal!(" equation(s) and "),
                ArcStr::from(::std::format!("{}", varSize)),
                literal!(" variable(s).\n"),
                ArcStr::from(::std::format!("{}", simpleEqnSize)),
                literal!(" of these are trivial equation(s).")
            ]);
            Ok((
                metamodelica::Ref::new(Values::Value::STRING { string: retStr.clone() }),
                cache.clone(),
            ))
        })() {
            cache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let false = (Interactive::existClass(className.clone(), &(SymbolTable::getAbsyn()))) else {
                return Err("pattern mismatch");
            };
            Error::addMessage(
                Error::LOOKUP_ERROR.clone(),
                list![
                    AbsynUtil::pathString(className.clone(), literal!("."), true, false)?,
                    literal!("<TOP>")
                ],
            )?;
            Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            if Error::getNumMessages() == 0 {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Check of "));
                            __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?);
                            __mm_s.push_str(&*literal!(" failed with no error message"));
                            ArcStr::from(__mm_s)
                        },
                        literal!("<TOP>")
                    ],
                )?;
            }
            Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((cache, outValue))
}

fn getWithinStatement(mut ip: metamodelica::Ref<Absyn::Path>) -> Absyn::Within {
    let mut op: Absyn::Within;
    op = 'mc: {
        let __mc_input = ip;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                path => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&path))?;
                    Ok(Absyn::Within::WITHIN { path: path.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_ast::Absyn::Within::TOP)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    op
}

fn dumpXMLDAE(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut vals: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inMsg: &Absyn::Msg,
) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache;
    let mut xml_filename: ArcStr = arcstr::literal!("");
    (outCache, xml_filename) = 'mc: {
        let __mc_input = (inCache, inEnv, &**vals);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "flat" }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut cname_str: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut description: ArcStr;
                    let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dlow_1: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), rewriteRulesFile.clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) {
                        (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    dae = metamodelica::Own::own(__pa2);
                    description = DAEUtil::daeDescription(&dae);
                    compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                    cname_str = AbsynUtil::pathString(classname.clone(), literal!("."), true, false)?;
                    filenameprefix = if (metamodelica::stringEq(&filenameprefix, &(literal!("<default>")))) {cname_str.clone()} else {filenameprefix.clone()};
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: description.clone(), fileNamePrefix: filenameprefix.clone(), simflags: None })?;
                    dlow_1 = BackendDAEUtil::preOptimizeBackendDAE(dlow.clone(), None)?;
                    dlow_1 = FindZeroCrossings::findZeroCrossings(&dlow_1)?;
                    xml_filename = stringAppendList(list![filenameprefix.clone(), literal!(".xml")]);
                    dlow_1 = applyRewriteRulesOnBackend(dlow_1.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(&dlow_1, addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf(xml_filename.clone())?;
                    Print::clearBuf();
                    compileDir = if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()};
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), literal!(""))?;
                    RewriteRules::clearRules();
                    Ok(((cache.clone(), stringAppendList(list![compileDir.clone(), xml_filename.clone()])), xml_filename.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            xml_filename = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "optimiser" }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut cname_str: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut description: ArcStr;
                    let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dlow_1: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), rewriteRulesFile.clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) {
                        (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    dae = metamodelica::Own::own(__pa2);
                    description = DAEUtil::daeDescription(&dae);
                    compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                    cname_str = AbsynUtil::pathString(classname.clone(), literal!("."), true, false)?;
                    filenameprefix = if (metamodelica::stringEq(&filenameprefix, &(literal!("<default>")))) {cname_str.clone()} else {filenameprefix.clone()};
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: description.clone(), fileNamePrefix: filenameprefix.clone(), simflags: None })?;
                    dlow_1 = BackendDAEUtil::preOptimizeBackendDAE(dlow.clone(), None)?;
                    dlow_1 = BackendDAEUtil::transformBackendDAE(&dlow_1, None, None, None)?;
                    dlow_1 = FindZeroCrossings::findZeroCrossings(&dlow_1)?;
                    xml_filename = stringAppendList(list![filenameprefix.clone(), literal!(".xml")]);
                    dlow_1 = applyRewriteRulesOnBackend(dlow_1.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(&dlow_1, addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf(xml_filename.clone())?;
                    Print::clearBuf();
                    compileDir = if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()};
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), literal!(""))?;
                    RewriteRules::clearRules();
                    Ok(((cache.clone(), stringAppendList(list![compileDir.clone(), xml_filename.clone()])), xml_filename.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            xml_filename = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "backEnd" }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut cname_str: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut description: ArcStr;
                    let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut indexed_dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), rewriteRulesFile.clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) {
                        (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    dae = metamodelica::Own::own(__pa2);
                    description = DAEUtil::daeDescription(&dae);
                    compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                    cname_str = AbsynUtil::pathString(classname.clone(), literal!("."), true, false)?;
                    filenameprefix = if (metamodelica::stringEq(&filenameprefix, &(literal!("<default>")))) {cname_str.clone()} else {filenameprefix.clone()};
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: description.clone(), fileNamePrefix: filenameprefix.clone(), simflags: None })?;
                    (indexed_dlow, _, _, _, _) = BackendDAEUtil::getSolvedSystem(dlow.clone(), &(literal!("")), None, None, None, None)?;
                    xml_filename = stringAppendList(list![filenameprefix.clone(), literal!(".xml")]);
                    indexed_dlow = applyRewriteRulesOnBackend(indexed_dlow.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(&indexed_dlow, addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf(xml_filename.clone())?;
                    Print::clearBuf();
                    compileDir = if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()};
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), literal!(""))?;
                    RewriteRules::clearRules();
                    Ok(((cache.clone(), stringAppendList(list![compileDir.clone(), xml_filename.clone()])), xml_filename.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            xml_filename = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "stateSpace" }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
                    let mut cname_str: ArcStr;
                    let mut compileDir: ArcStr;
                    let mut description: ArcStr;
                    let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut indexed_dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), rewriteRulesFile.clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) {
                        (__pa0, __pa1, Some(__pa2), _) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    dae = metamodelica::Own::own(__pa2);
                    description = DAEUtil::daeDescription(&dae);
                    compileDir = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) };
                    cname_str = AbsynUtil::pathString(classname.clone(), literal!("."), true, false)?;
                    filenameprefix = if (metamodelica::stringEq(&filenameprefix, &(literal!("<default>")))) {cname_str.clone()} else {filenameprefix.clone()};
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: description.clone(), fileNamePrefix: filenameprefix.clone(), simflags: None })?;
                    (indexed_dlow, _, _, _, _) = BackendDAEUtil::getSolvedSystem(dlow.clone(), &(literal!("")), None, None, None, None)?;
                    xml_filename = stringAppendList(list![filenameprefix.clone(), literal!(".xml")]);
                    indexed_dlow = applyRewriteRulesOnBackend(indexed_dlow.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(&indexed_dlow, addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), true)?;
                    Print::writeBuf(xml_filename.clone())?;
                    Print::clearBuf();
                    compileDir = if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()};
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), literal!(""))?;
                    RewriteRules::clearRules();
                    Ok(((cache.clone(), stringAppendList(list![compileDir.clone(), xml_filename.clone()])), xml_filename.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            xml_filename = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), literal!(""))?;
                    RewriteRules::clearRules();
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, xml_filename))
}

fn applyRewriteRulesOnBackend(
    mut inBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outBackendDAE = if (RewriteRules::noRewriteRulesBackEnd()) {
        inBackendDAE
    } else {
        BackendDAEOptimize::applyRewriteRulesBackend(&inBackendDAE)?
    };
    Ok(outBackendDAE)
}

fn getClassnamesInClassList(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inShowProtected: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStrings: metamodelica::List<ArcStr>;
    outStrings = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = ProgramUtil::getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, false)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { functionNames: _, comment: _ }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: _, comment: _ }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = ProgramUtil::getClassnamesInParts(metamodelica::AsArg::as_arg(&parts), b, false)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { functionName: _, vars: _, comment: _ }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStrings)
}

fn joinPaths(mut child: ArcStr, mut parent: metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((child, parent)) {
        (c, r) => {
            let mut res: metamodelica::Ref<Absyn::Path>;
            res = AbsynUtil::joinPaths(r.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: c.clone() }))?;
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

fn getAllClassPathsRecursive(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inCheckProtected: bool,
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outPaths = 'mc: {
        let __mc_input = (inCheckProtected, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut b, mut p) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cdef: metamodelica::Ref<Absyn::Class>;
            let mut strlst: metamodelica::List<ArcStr>;
            let mut result_path_lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut result: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            cdef = ProgramUtil::getPathedClassInProgram(inPath.clone(), &(p.clone()), false, false)?;
            strlst = getClassnamesInClassList(&inPath, &(p.clone()), &cdef, b.clone())?;
            result_path_lst = List::map1(strlst.clone(), &joinPaths, inPath.clone())?;
            result = List::flatten(List::map2(
                result_path_lst.clone(),
                &getAllClassPathsRecursive,
                b.clone(),
                p.clone(),
            )?)?;
            Ok(metamodelica::cons(inPath.clone(), result.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut parent_string: ArcStr;
            let mut s: ArcStr;
            parent_string = AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?;
            s = Error::printMessagesStr(false);
            s = stringAppendList(list![
                parent_string.clone(),
                literal!("->"),
                literal!("PROBLEM GETTING CLASS PATHS: "),
                s.clone(),
                literal!("\n")
            ]);
            metamodelica::print(s.clone());
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPaths)
}

pub(crate) fn checkAllModelsRecursive(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut inCheckProtected: bool,
    mut inMsg: Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, inCheckProtected, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, mut b, mut msg) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut allClassPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut ret: ArcStr;
            let mut failed: i32;
            allClassPaths = getAllClassPathsRecursive(className.clone(), b.clone(), SymbolTable::getAbsyn())?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Number of classes to check: "));
                __mm_s.push_str(&*intString(((allClassPaths).len() as i32)));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            failed = checkAll(
                &(cache.clone()),
                &(env.clone()),
                &allClassPaths,
                &(msg.clone()),
                !(Testsuite::isRunning()?),
                0,
            )?;
            ret = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Number of classes checked / failed: "));
                __mm_s.push_str(&*intString(((allClassPaths).len() as i32)));
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*intString(failed));
                ArcStr::from(__mm_s)
            };
            Ok((
                cache.clone(),
                metamodelica::Ref::new(Values::Value::STRING { string: ret.clone() }),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, _, _, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut ret: ArcStr;
            ret = stringAppend(
                literal!("Error checking: "),
                AbsynUtil::pathString(className.clone(), literal!("."), true, false)?,
            );
            Ok((
                cache.clone(),
                metamodelica::Ref::new(Values::Value::STRING { string: ret.clone() }),
            ))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn failOrSuccess(mut inStr: ArcStr) -> (ArcStr, bool) {
    let mut outStr: ArcStr;
    let mut failed: bool = false;
    outStr = 'mc: {
        let __mc_input = inStr.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut res: i32;
            let mut failed: bool = failed.clone();
            res = System::stringFind(inStr.clone(), literal!("successfully"))?;
            let true = (res >= 0) else {
                return Err("pattern mismatch");
            };
            failed = false;
            Ok((literal!("OK"), failed.clone()))
        })() {
            failed = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut failed: bool = failed.clone();
            failed = true;
            Ok((literal!("FAILED!"), failed.clone()))
        })() {
            failed = __wb0;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outStr, failed)
}

pub(crate) fn checkAll(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut allClasses: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inMsg: &Absyn::Msg,
    mut reportTimes: bool,
    mut failed: i32,
) -> Result<i32> {
    let mut failed: i32 = failed;
    let mut p: Absyn::Program;
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut className: metamodelica::Ref<Absyn::Path>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    let mut smsg: ArcStr = arcstr::literal!("");
    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut elapsedTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut c: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut f: bool = false;
    p = SymbolTable::getAbsyn();
    let () = 'mc: {
        let __mc_input = &**allClasses;
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
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: className, tail: rest } => {
                    let mut c: metamodelica::Ref<Absyn::Class> = c.clone();
                    let mut elapsedTime: metamodelica::Real = elapsedTime.clone();
                    let mut f: bool = f.clone();
                    let mut failed: i32 = failed.clone();
                    let mut s: ArcStr = s.clone();
                    let mut smsg: ArcStr = smsg.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    let mut t1: metamodelica::Real = t1.clone();
                    let mut t2: metamodelica::Real = t2.clone();
                    c = ProgramUtil::getPathedClassInProgram(className.clone(), &p, false, false)?;
                    let false = (Interactive::isPackage(className.clone(), &p)) else { return Err("pattern mismatch") };
                    let false = (Interactive::isType(className.clone(), &p)) else { return Err("pattern mismatch") };
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Checking: ")); __mm_s.push_str(&*Dump::unparseClassAttributesStr(&c)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("... ")); ArcStr::from(__mm_s) });
                    t1 = clock();
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
                    let __pa0 = ::match_deref::match_deref! { match &(checkModel(FCore::emptyCache(), inEnv.clone(), className.clone(), inMsg)?) {
                        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), false)?;
                    t2 = clock();
                    elapsedTime = t2 - t1;
                    s = realString(elapsedTime);
                    (smsg, f) = failOrSuccess(r#str.clone());
                    failed = if (f) {failed + 1} else {failed};
                    if reportTimes {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" seconds -> ")); __mm_s.push_str(&*smsg); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    } else {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*smsg); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if !(stringEmpty(&r#str)) {
                        metamodelica::print(literal!("\t"));
                    }
                    metamodelica::print(System::stringReplace(r#str.clone(), literal!("\n"), literal!("\n\t"))?);
                    metamodelica::print(literal!("\n"));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error String:\n")); __mm_s.push_str(&*Print::getErrorString()?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error Buffer:\n")); __mm_s.push_str(&*ErrorExt::printMessagesStr(false)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*if (f) {literal!("[-]")} else {literal!("[+]")}); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*if (reportTimes) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*realString(elapsedTime)); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print(literal!("-------------------------------------------------------------------------\n"));
                    failed = checkAll(inCache, inEnv, metamodelica::AsArg::as_arg(&rest), inMsg, reportTimes, failed)?;
                    Ok(((), c.clone(), elapsedTime.clone(), f.clone(), failed.clone(), s.clone(), smsg.clone(), r#str.clone(), t1.clone(), t2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            c = __wb0;
            elapsedTime = __wb1;
            f = __wb2;
            failed = __wb3;
            s = __wb4;
            smsg = __wb5;
            r#str = __wb6;
            t1 = __wb7;
            t2 = __wb8;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: className, tail: rest } => {
                    let mut c: metamodelica::Ref<Absyn::Class> = c.clone();
                    let mut failed: i32 = failed.clone();
                    c = ProgramUtil::getPathedClassInProgram(className.clone(), &p, false, false)?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Checking skipped: ")); __mm_s.push_str(&*Dump::unparseClassAttributesStr(&c)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("...\n")); ArcStr::from(__mm_s) });
                    failed = checkAll(inCache, inEnv, metamodelica::AsArg::as_arg(&rest), inMsg, reportTimes, failed)?;
                    Ok(((), c.clone(), failed.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            c = __wb0;
            failed = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(failed)
}

fn getAlgorithms(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outList = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            algsList = getAlgorithmsInClassParts(metamodelica::AsArg::as_arg(&parts));
            algsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            algsList = getAlgorithmsInClassParts(metamodelica::AsArg::as_arg(&parts));
            algsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outList)
}

fn getAlgorithmsInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: cp @ Deref @ Absyn::ClassPart::ALGORITHMS { .. }, tail: xs } => {
                let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                algsList = getAlgorithmsInClassParts(xs);
                return metamodelica::cons(cp.clone(), algsList)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthAlgorithm(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    algsList = getAlgorithms(inClass)?;
    outString = getNthAlgorithmInClass(&((algsList).get(inInteger)?))?;
    Ok(outString)
}

fn getNthAlgorithmInClass(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClassPart {
        Absyn::ClassPart::ALGORITHMS { contents: algs } => {
            let mut r#str: ArcStr;
            r#str = Dump::unparseAlgorithmStrLst(algs.clone(), literal!("\n"))?;
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getInitialAlgorithms(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outList = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            algsList = getInitialAlgorithmsInClassParts(metamodelica::AsArg::as_arg(&parts));
            algsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            algsList = getInitialAlgorithmsInClassParts(metamodelica::AsArg::as_arg(&parts));
            algsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outList)
}

fn getInitialAlgorithmsInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: cp @ Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. }, tail: xs } => {
                let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                algsList = getInitialAlgorithmsInClassParts(xs);
                return metamodelica::cons(cp.clone(), algsList)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthInitialAlgorithm(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut algsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    algsList = getInitialAlgorithms(inClass)?;
    outString = getNthInitialAlgorithmInClass(&((algsList).get(inInteger)?))?;
    Ok(outString)
}

fn getNthInitialAlgorithmInClass(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClassPart {
        Absyn::ClassPart::INITIALALGORITHMS { contents: algs } => {
            let mut r#str: ArcStr;
            r#str = Dump::unparseAlgorithmStrLst(algs.clone(), literal!("\n"))?;
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getAlgorithmItemsCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32;
            count = getAlgorithmItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32;
            count = getAlgorithmItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getAlgorithmItemsCountInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: xs } => {
                let mut c1: i32;
                let mut c2: i32;
                c1 = getAlgorithmItemsCountInAlgorithmItems(metamodelica::AsArg::as_arg(&algs));
                c2 = getAlgorithmItemsCountInClassParts(xs);
                return c1 + c2
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getAlgorithmItemsCountInAlgorithmItems<'__b>(
    mut inAbsynAlgorithmItemLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynAlgorithmItemLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. }, tail: xs } => {
                let mut c1: i32;
                c1 = getAlgorithmItemsCountInAlgorithmItems(xs);
                return c1 + 1
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynAlgorithmItemLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthAlgorithmItem(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getNthAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&parts), inInteger)?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getNthAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&parts), inInteger)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getNthAlgorithmItemInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: _ }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthAlgorithmItemInAlgorithms(metamodelica::AsArg::as_arg(&algs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    let mut c1: i32;
                    let mut newn: i32;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(metamodelica::AsArg::as_arg(&algs));
                    newn = n.clone() - c1;
                    r#str = getNthAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&xs), newn)?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&xs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getNthAlgorithmItemInAlgorithms(
    mut inAbsynAlgorithmItemLst: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inAbsynAlgorithmItemLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, comment: cmt, info: inf }, tail: _ }, 1) => {
                    let mut r#str: ArcStr;
                    r#str = Dump::unparseAlgorithmStr(metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg.clone(), comment: cmt.clone(), info: inf.clone() }))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    let mut newn: i32;
                    newn = n.clone() - 1;
                    r#str = getNthAlgorithmItemInAlgorithms(metamodelica::AsArg::as_arg(&xs), newn)?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getInitialAlgorithmItemsCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32;
            count = getInitialAlgorithmItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32;
            count = getInitialAlgorithmItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getInitialAlgorithmItemsCountInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: xs } => {
                let mut c1: i32;
                let mut c2: i32;
                c1 = getAlgorithmItemsCountInAlgorithmItems(metamodelica::AsArg::as_arg(&algs));
                c2 = getInitialAlgorithmItemsCountInClassParts(xs);
                return c1 + c2
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthInitialAlgorithmItem(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut n = inInteger;
            let mut r#str: ArcStr;
            r#str = getNthInitialAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&parts), n)?;
            r#str
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut n = inInteger;
            let mut r#str: ArcStr;
            r#str = getNthInitialAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&parts), n)?;
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getNthInitialAlgorithmItemInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: _ }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthAlgorithmItemInAlgorithms(metamodelica::AsArg::as_arg(&algs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    let mut c1: i32;
                    let mut newn: i32;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(metamodelica::AsArg::as_arg(&algs));
                    newn = n.clone() - c1;
                    r#str = getNthInitialAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&xs), newn)?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthInitialAlgorithmItemInClassParts(metamodelica::AsArg::as_arg(&xs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getEquations(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outList = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqsList = getEquationsInClassParts(metamodelica::AsArg::as_arg(&parts));
            eqsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqsList = getEquationsInClassParts(metamodelica::AsArg::as_arg(&parts));
            eqsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outList)
}

fn getEquationsInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: cp @ Deref @ Absyn::ClassPart::EQUATIONS { .. }, tail: xs } => {
                let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                eqsList = getEquationsInClassParts(xs);
                return metamodelica::cons(cp.clone(), eqsList)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthEquation(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    eqsList = getEquations(inClass)?;
    outString = getNthEquationInClass(&((eqsList).get(inInteger)?))?;
    Ok(outString)
}

fn getNthEquationInClass(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClassPart {
        Absyn::ClassPart::EQUATIONS { contents: eqs } => {
            let mut r#str: ArcStr;
            r#str = Dump::unparseEquationItemStrLst(eqs.clone(), literal!("\n"))?;
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getInitialEquations(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outList = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqsList = getInitialEquationsInClassParts(metamodelica::AsArg::as_arg(&parts));
            eqsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqsList = getInitialEquationsInClassParts(metamodelica::AsArg::as_arg(&parts));
            eqsList
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outList)
}

fn getInitialEquationsInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: cp @ Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. }, tail: xs } => {
                let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                eqsList = getInitialEquationsInClassParts(xs);
                return metamodelica::cons(cp.clone(), eqsList)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthInitialEquation(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut eqsList: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    eqsList = getInitialEquations(inClass)?;
    outString = getNthInitialEquationInClass(&((eqsList).get(inInteger)?))?;
    Ok(outString)
}

fn getNthInitialEquationInClass(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClassPart {
        Absyn::ClassPart::INITIALEQUATIONS { contents: eqs } => {
            let mut r#str: ArcStr;
            r#str = Dump::unparseEquationItemStrLst(eqs.clone(), literal!("\n"))?;
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getEquationItemsCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32;
            count = getEquationItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32;
            count = getEquationItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getEquationItemsCountInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: xs } => {
                let mut c1: i32;
                let mut c2: i32;
                c1 = getEquationItemsCountInEquationItems(metamodelica::AsArg::as_arg(&eqs));
                c2 = getEquationItemsCountInClassParts(xs);
                return c1 + c2
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getEquationItemsCountInEquationItems<'__b>(
    mut inAbsynEquationItemLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynEquationItemLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, tail: xs } => {
                let mut c1: i32;
                c1 = getEquationItemsCountInEquationItems(xs);
                return c1 + 1
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynEquationItemLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthEquationItem(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut n = inInteger;
            let mut r#str: ArcStr;
            r#str = getNthEquationItemInClassParts(metamodelica::AsArg::as_arg(&parts), n)?;
            r#str
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut n = inInteger;
            let mut r#str: ArcStr;
            r#str = getNthEquationItemInClassParts(metamodelica::AsArg::as_arg(&parts), n)?;
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getNthEquationItemInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: _ }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthEquationItemInEquations(metamodelica::AsArg::as_arg(&eqs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    let mut c1: i32;
                    let mut newn: i32;
                    c1 = getEquationItemsCountInEquationItems(metamodelica::AsArg::as_arg(&eqs));
                    newn = n.clone() - c1;
                    r#str = getNthEquationItemInClassParts(metamodelica::AsArg::as_arg(&xs), newn)?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    r#str = getNthEquationItemInClassParts(metamodelica::AsArg::as_arg(&xs), n.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getNthEquationItemInEquations(
    mut inAbsynEquationItemLst: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (&**inAbsynEquationItemLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, .. }, tail: _ }, 1) => {
                    let mut r#str: ArcStr;
                    r#str = Dump::unparseEquationStr(eq.clone())?;
                    r#str = stringAppend(r#str.clone(), literal!(";"));
                    r#str = System::trim(r#str.clone(), literal!(" "));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr;
                    let mut newn: i32;
                    newn = n.clone() - 1;
                    r#str = getNthEquationItemInEquations(metamodelica::AsArg::as_arg(&xs), newn)?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getInitialEquationItemsCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32;
            count = getInitialEquationItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32;
            count = getInitialEquationItemsCountInClassParts(metamodelica::AsArg::as_arg(&parts));
            count
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getInitialEquationItemsCountInClassParts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: xs } => {
                let mut c1: i32;
                let mut c2: i32;
                c1 = getEquationItemsCountInEquationItems(metamodelica::AsArg::as_arg(&eqs));
                c2 = getInitialEquationItemsCountInClassParts(xs);
                return c1 + c2
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut res: i32;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getNthInitialEquationItem(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getNthInitialEquationItemInClassParts(metamodelica::AsArg::as_arg(&parts), inInteger)?
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
            parts = (*__esc_parts).clone();
            getNthInitialEquationItemInClassParts(metamodelica::AsArg::as_arg(&parts), inInteger)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getNthInitialEquationItemInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inAbsynClassPartLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: _ } => {
                    Ok(getNthEquationItemInEquations(metamodelica::AsArg::as_arg(&eqs), inInteger)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: xs } => {
                    let mut c1: i32;
                    let mut newn: i32;
                    c1 = getEquationItemsCountInEquationItems(metamodelica::AsArg::as_arg(&eqs));
                    newn = inInteger - c1;
                    Ok(getNthInitialEquationItemInClassParts(metamodelica::AsArg::as_arg(&xs), newn)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    Ok(getNthInitialEquationItemInClassParts(metamodelica::AsArg::as_arg(&xs), inInteger)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn getAnnotationCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann, .. }, .. } => {
            ((ann).len() as i32)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann, .. }, .. } => {
            ((ann).len() as i32)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getNthAnnotationString(mut inClass: &metamodelica::Ref<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann: anns, .. }, .. } => {
            let mut n = inInteger;
            let mut ann: metamodelica::Ref<Absyn::Annotation>;
            let mut r#str: ArcStr;
            ann = (anns).get(n)?;
            r#str = Dump::unparseAnnotation(ann)?;
            r#str = stringAppend(r#str, literal!(";"));
            r#str = System::trim(r#str, literal!(" "));
            r#str
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann: anns, .. }, .. } => {
            let mut n = inInteger;
            let mut ann: metamodelica::Ref<Absyn::Annotation>;
            let mut r#str: ArcStr;
            ann = (anns).get(n)?;
            r#str = Dump::unparseAnnotation(ann)?;
            r#str = stringAppend(r#str, literal!(";"));
            r#str = System::trim(r#str, literal!(" "));
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getImportCount(mut inClass: &metamodelica::Ref<Absyn::Class>) -> i32 {
    let mut outInteger: i32;
    let mut pub_imports_list: metamodelica::List<Absyn::Import>;
    let mut pro_imports_list: metamodelica::List<Absyn::Import>;
    (pub_imports_list, pro_imports_list) =
        CevalScript::getImportList(inClass, metamodelica::nil(), metamodelica::nil());
    outInteger = ((pub_imports_list).len() as i32) + ((pro_imports_list).len() as i32);
    outInteger
}

fn getNthImport(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inInteger: i32,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValue: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut pub_imports_list: metamodelica::List<Absyn::Import>;
    let mut pro_imports_list: metamodelica::List<Absyn::Import>;
    (pub_imports_list, pro_imports_list) =
        CevalScript::getImportList(inClass, metamodelica::nil(), metamodelica::nil());
    outValue = unparseNthImport(&((pub_imports_list).get(inInteger)?))?;
    Ok(outValue)
}

fn unparseNthImport(mut inImport: &Absyn::Import) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValue: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValue = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT {
            name: mut id,
            path: mut path,
        } => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut path_str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            vals = list![
                metamodelica::Ref::new(Values::Value::STRING { string: path_str }),
                metamodelica::Ref::new(Values::Value::STRING { string: id.clone() }),
                metamodelica::Ref::new(Values::Value::STRING {
                    string: literal!("named")
                })
            ];
            vals
        }
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut path_str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            vals = list![
                metamodelica::Ref::new(Values::Value::STRING { string: path_str }),
                metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }),
                metamodelica::Ref::new(Values::Value::STRING {
                    string: literal!("qualified")
                })
            ];
            vals
        }
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut path_str: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            path_str = stringAppendList(list![path_str, literal!(".*")]);
            vals = list![
                metamodelica::Ref::new(Values::Value::STRING { string: path_str }),
                metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }),
                metamodelica::Ref::new(Values::Value::STRING {
                    string: literal!("unqualified")
                })
            ];
            vals
        }
        Absyn::Import::GROUP_IMPORT {
            prefix: ref path,
            groups: ref gi,
        } => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut path_str: ArcStr;
            let mut id: ArcStr;
            path_str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
            id = stringDelimitList(unparseGroupImport(metamodelica::AsArg::as_arg(&gi)), literal!(","));
            path_str = stringAppendList(list![path_str, literal!(".{"), id, literal!("}")]);
            vals = list![
                metamodelica::Ref::new(Values::Value::STRING { string: path_str }),
                metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }),
                metamodelica::Ref::new(Values::Value::STRING {
                    string: literal!("multiple")
                })
            ];
            vals
        }
    });
    Ok(outValue)
}

fn unparseGroupImport<'__b>(
    mut inAbsynGroupImportLst: &'__b metamodelica::List<Absyn::GroupImport>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynGroupImportLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Absyn::GroupImport::GROUP_IMPORT_NAME { name: r#str }, tail: rest } => {
                let mut lst: metamodelica::List<ArcStr>;
                lst = unparseGroupImport(rest);
                return metamodelica::cons(r#str.clone(), lst)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut lst: metamodelica::List<ArcStr>;
                { inAbsynGroupImportLst = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn isShortDefinition(mut inPath: metamodelica::Ref<Absyn::Path>, mut inProgram: &Absyn::Program) -> bool {
    let mut outBoolean: bool;
    match '__try0: {
        ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getPathedClassInProgram(inPath.clone(), inProgram, false, false), '__try0)) {
            Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outBoolean = true;
        Ok::<_, &'static str>((outBoolean.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBoolean = __try0_o0;
        }
        Err(_) => {
            outBoolean = false;
        }
    }
    outBoolean
}

fn isExperiment(mut path: metamodelica::Ref<Absyn::Path>, mut program: &Absyn::Program) -> bool {
    let mut res: bool;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        cdef = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0);
        let false = (AbsynUtil::isPartial(&cdef)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        let true = (AbsynUtil::isModel(&cdef) || AbsynUtil::isBlock(&cdef)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        let __pa1 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(&cdef, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("experiment") })), &hasStopTime)) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        res = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

fn hasStopTime(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            List::any(metamodelica::AsArg::as_arg(&arglst), &move |__a0: metamodelica::Ref<Absyn::ElementArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(hasStopTime2(&__a0)) })?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(b)
}

fn hasStopTime2(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "StopTime" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn searchClassNames(
    mut inVals: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inSearchText: ArcStr,
    mut inFindInText: bool,
    mut inProgram: Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outVals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outVals = 'mc: {
        let __mc_input = (&**inVals, inSearchText, inFindInText, inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: val @ Deref @ Values::Value::CODE { A: _ }, tail: xs }, str1, true, p) => {
                    let mut valsList: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut r#str: ArcStr;
                    let mut p1: Absyn::Program;
                    let mut absynClass: metamodelica::Ref<Absyn::Class>;
                    let mut position: i32;
                    absynClass = ProgramUtil::getPathedClassInProgram(ValuesUtil::getPath(metamodelica::AsArg::as_arg(&val))?, metamodelica::AsArg::as_arg(&p), false, false)?;
                    p1 = Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP };
                    let false = (Interactive::isPackage(ValuesUtil::getPath(metamodelica::AsArg::as_arg(&val))?, &inProgram)) else { return Err("pattern mismatch") };
                    r#str = Dump::unparseStr(p1.clone(), false, Dump::defaultDumpOptions.clone())?;
                    position = System::stringFind(System::tolower(r#str.clone()), System::tolower(str1.clone()))?;
                    let true = (position > -1) else { return Err("pattern mismatch") };
                    valsList = searchClassNames(metamodelica::AsArg::as_arg(&xs), str1.clone(), true, p.clone())?;
                    Ok(metamodelica::cons(val.clone(), valsList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: val @ Deref @ Values::Value::CODE { A: _ }, tail: xs }, str1, b, p) => {
                    let mut valsList: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut r#str: ArcStr;
                    let mut position: i32;
                    r#str = ValuesDump::valString(metamodelica::AsArg::as_arg(&val))?;
                    position = System::stringFind(System::tolower(r#str.clone()), System::tolower(str1.clone()))?;
                    let true = (position > -1) else { return Err("pattern mismatch") };
                    valsList = searchClassNames(metamodelica::AsArg::as_arg(&xs), str1.clone(), b.clone(), p.clone())?;
                    Ok(metamodelica::cons(val.clone(), valsList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, str1, b, p) => {
                    let mut valsList: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    valsList = searchClassNames(metamodelica::AsArg::as_arg(&xs), str1.clone(), b.clone(), p.clone())?;
                    Ok(valsList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVals)
}

fn makeUsesArray(
    mut inTpl: &(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool),
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = (::match_deref::match_deref! { match &(inTpl) {
        (p, _, Deref @ metamodelica::ListNode::Cons { head: ver, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            let mut pstr: ArcStr;
            pstr = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
            ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::STRING { string: pstr }), metamodelica::Ref::new(Values::Value::STRING { string: ver.clone() })])
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("makeUsesArray failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(v)
}

fn saveTotalModel(
    mut filename: ArcStr,
    mut classpath: metamodelica::Ref<Absyn::Path>,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
    mut previous: bool,
) -> Result<()> {
    let mut result: ArcStr;
    let mut obfuscate_map: ArcStr;
    if previous {
        (result, obfuscate_map) = previousGetTotalModel(classpath, stripAnnotations, stripComments, obfuscate)?;
    } else {
        (result, obfuscate_map) = getTotalModel(classpath, stripAnnotations, stripComments, obfuscate)?;
    }
    if obfuscate {
        System::writeFile(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::stripFileExtension(filename.clone())?);
                __mm_s.push_str(&*literal!("_mapping.json"));
                ArcStr::from(__mm_s)
            },
            obfuscate_map,
        )?;
    }
    System::writeFile(filename, result)?;
    Ok(())
}

fn getTotalModel(
    mut classpath: metamodelica::Ref<Absyn::Path>,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<(ArcStr, ArcStr)> {
    let mut result: ArcStr;
    let mut obfuscate_map: ArcStr = literal!("");
    let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut str3: ArcStr;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut cls_path: metamodelica::Ref<Absyn::Path> = classpath;
    let mut extendable: bool;
    loadProgram(&cls_path)?;
    (scodeP, cls) = getTotalProgramNF(cls_path.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { cmt: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cmt = metamodelica::Own::own(__pa0);
    extendable = !(SCodeUtil::isPackage(&cls) || SCodeUtil::isFunction(&cls));
    scodeP = SCodeUtil::removeBuiltinsFromTopScope(scodeP)?;
    if stripAnnotations || stripComments {
        scodeP = SCodeUtil::stripCommentsFromProgram(scodeP, stripAnnotations, stripComments)?;
    }
    if obfuscate {
        (scodeP, cls_path, cmt, obfuscate_map, _) = Obfuscate::obfuscateProgram(scodeP, cls_path, cmt)?;
    }
    result = SCodeDump::programStr(scodeP, SCodeDump::defaultOptions.clone())?;
    if extendable {
        str1 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathLastIdent(&cls_path));
            __mm_s.push_str(&*literal!("_total"));
            ArcStr::from(__mm_s)
        };
        str2 = if (stripComments) {
            literal!("")
        } else {
            SCodeDump::printCommentStr(&cmt, SCodeDump::defaultOptions.clone())?
        };
        str2 = if (stringEq(&str2, &(literal!("")))) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*str2);
                ArcStr::from(__mm_s)
            }
        };
        str3 = if (stripAnnotations) {
            literal!("")
        } else {
            SCodeDump::printAnnotationStr(&cmt, SCodeDump::defaultOptions.clone())?
        };
        str3 = if (stringEq(&str3, &(literal!("")))) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*str3);
                __mm_s.push_str(&*literal!(";\n"));
                ArcStr::from(__mm_s)
            }
        };
        result = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*result);
            __mm_s.push_str(&*literal!("\nmodel "));
            __mm_s.push_str(&*str1);
            __mm_s.push_str(&*str2);
            __mm_s.push_str(&*literal!("\n  extends "));
            __mm_s.push_str(&*AbsynUtil::pathString(cls_path, literal!("."), true, false)?);
            __mm_s.push_str(&*literal!(";\n"));
            __mm_s.push_str(&*str3);
            __mm_s.push_str(&*literal!("end "));
            __mm_s.push_str(&*str1);
            __mm_s.push_str(&*literal!(";\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok((result, obfuscate_map))
}

fn previousGetTotalModel(
    mut classpath: metamodelica::Ref<Absyn::Path>,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<(ArcStr, ArcStr)> {
    let mut result: ArcStr;
    let mut obfuscate_map: ArcStr = literal!("");
    let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut r#str: ArcStr;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut str3: ArcStr;
    let mut env: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut cls_path: metamodelica::Ref<Absyn::Path> = classpath;
    loadProgram(&cls_path)?;
    scodeP = SymbolTable::getSCode()?;
    (scodeP, env) = NFSCodeFlatten::flattenClassInProgram(cls_path.clone(), scodeP)?;
    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupClassName(cls_path.clone(), env, &(Absyn::dummyInfo.clone()))?) {
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { cmt: __pa0, .. }, .. }, _, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cmt = metamodelica::Own::own(__pa0);
    scodeP = SCodeUtil::removeBuiltinsFromTopScope(scodeP)?;
    if stripAnnotations || stripComments {
        scodeP = SCodeUtil::stripCommentsFromProgram(scodeP, stripAnnotations, stripComments)?;
    }
    if obfuscate {
        (scodeP, cls_path, cmt, obfuscate_map, _) = Obfuscate::obfuscateProgram(scodeP, cls_path, cmt)?;
    }
    r#str = SCodeDump::programStr(scodeP, SCodeDump::defaultOptions.clone())?;
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*AbsynUtil::pathLastIdent(&cls_path));
        __mm_s.push_str(&*literal!("_total"));
        ArcStr::from(__mm_s)
    };
    str2 = if (stripComments) {
        literal!("")
    } else {
        SCodeDump::printCommentStr(&cmt, SCodeDump::defaultOptions.clone())?
    };
    str2 = if (stringEq(&str2, &(literal!("")))) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*str2);
            ArcStr::from(__mm_s)
        }
    };
    str3 = if (stripAnnotations) {
        literal!("")
    } else {
        SCodeDump::printAnnotationStr(&cmt, SCodeDump::defaultOptions.clone())?
    };
    str3 = if (stringEq(&str3, &(literal!("")))) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*str3);
            __mm_s.push_str(&*literal!(";\n"));
            ArcStr::from(__mm_s)
        }
    };
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nmodel "));
        __mm_s.push_str(&*str1);
        __mm_s.push_str(&*str2);
        __mm_s.push_str(&*literal!("\n  extends "));
        __mm_s.push_str(&*AbsynUtil::pathString(cls_path, literal!("."), true, false)?);
        __mm_s.push_str(&*literal!(";\n"));
        __mm_s.push_str(&*str3);
        __mm_s.push_str(&*literal!("end "));
        __mm_s.push_str(&*str1);
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    };
    result = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*str1);
        ArcStr::from(__mm_s)
    };
    Ok((result, obfuscate_map))
}

fn getTotalProgramNF(
    mut classPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::Ref<SCode::Element>,
)> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;
    let mut nf_inst: bool;
    let mut builtin_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut annotation_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    cls = InteractiveUtil::getPathedSCodeElementInProgram(classPath.clone(), SymbolTable::getSCode()?)?;
    ErrorExt::setCheckpoint(literal!("CevalScriptBackend.getTotalProgramNF"));
    nf_inst = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    match '__try0: {
        (_, builtin_p) = unwrap_break_err!(FBuiltin::getInitialFunctions(), '__try0);
        annotation_p = unwrap_break_err!(AbsynToSCode::translateAbsyn2SCode(unwrap_break_err!(InteractiveUtil::modelicaAnnotationProgram(unwrap_break_err!(Config::getAnnotationVersion(), '__try0)), '__try0)), '__try0);
        used = unwrap_break_err!(NFUsedElements::collect(&(metamodelica::cons(classPath.clone(), unwrap_break_err!(getNestedClassPaths(&cls, &classPath, metamodelica::nil()), '__try0))), listAppend(builtin_p.clone(), unwrap_break_err!(SymbolTable::getSCode(), '__try0)), annotation_p.clone()), '__try0);
        Ok::<_, &'static str>((used.clone(),))
    } {
        Ok((__try0_o0,)) => {
            used = __try0_o0;
        }
        Err(_) => {
            used = UnorderedSet::new(
                (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
                (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
                13,
            );
        }
    }
    FlagsUtil::set(Flags::SCODE_INST.clone(), nf_inst)?;
    ErrorExt::rollBack(literal!("CevalScriptBackend.getTotalProgramNF"));
    markElementUsed(&cls, used.clone())?;
    program = SymbolTable::getSCode()?;
    program = filterUsedClasses(&(program.clone()), used, program, false)?;
    cls = InteractiveUtil::getPathedSCodeElementInProgram(classPath, program.clone())?;
    Ok((program, cls))
}

fn getDefUseChains(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileName: ArcStr,
    mut scope: metamodelica::Ref<Absyn::Path>,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut cls_path: metamodelica::Ref<Absyn::Path> = className.clone();
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cls: metamodelica::Ref<SCode::Element>;
    let mut defs: metamodelica::List<NFUsedElements::Definition>;
    let mut uses: metamodelica::List<NFUsedElements::Use>;
    let mut unresolved: metamodelica::List<NFUsedElements::Use>;
    program = SymbolTable::getSCode()?;
    if '__try0: {
        cls = unwrap_break_err!(InteractiveUtil::getPathedSCodeElementInProgram(className.clone(), program.clone()), '__try0);
        if !(SCodeUtil::elementIsClass(&cls)) {
            cls_path = unwrap_break_err!(AbsynUtil::stripLast(&className), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        cls_path = AbsynUtil::stripLast(&className)?;
    }
    paths = scopeClassPaths(scope.clone(), program.clone())?;
    if !(isAllLoadedClasses(&scope)) && !(AbsynUtil::pathPrefixOf(scope.clone(), cls_path.clone())) {
        if '__try1: {
            cls = unwrap_break_err!(InteractiveUtil::getPathedSCodeElementInProgram(cls_path.clone(), program.clone()), '__try1);
            paths = unwrap_break_err!(getNestedClassPaths(&cls, &(cls_path.clone()), metamodelica::cons(cls_path.clone(), paths.clone())), '__try1);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    (defs, uses, unresolved) = collectDefUse(paths, program)?;
    result = NFDefUseChains::toJSON(
        AbsynUtil::pathString(className, literal!("."), true, false)?,
        defs,
        uses,
        unresolved,
        &(if (isAllLoadedClasses(&scope)) {
            literal!("")
        } else {
            AbsynUtil::pathString(scope, literal!("."), true, false)?
        }),
        prettyPrint,
    )?;
    if !(stringEmpty(&fileName)) {
        System::writeFile(fileName.clone(), result)?;
        result = fileName;
    }
    Ok(result)
}

fn getClassDiagram(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut fileName: ArcStr,
    mut format: &ArcStr,
    mut depth: i32,
    mut exclude: metamodelica::List<ArcStr>,
    mut showModifiers: bool,
) -> Result<ArcStr> {
    let mut result: ArcStr = literal!("");
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut builtin_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut annotation_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut nf_inst: bool;
    if !metamodelica::stringEq(&format, &(literal!("plantuml")))
        && !metamodelica::stringEq(&format, &(literal!("mermaid")))
        && !metamodelica::stringEq(&format, &(literal!("drawio")))
    {
        Error::addCompilerError({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("getClassDiagram: unknown format "));
            __mm_s.push_str(&*format);
            __mm_s.push_str(&*literal!(", expected plantuml, mermaid or drawio."));
            ArcStr::from(__mm_s)
        })?;
        return Ok(result);
    }
    program = SymbolTable::getSCode()?;
    if let Ok(_) = InteractiveUtil::getPathedSCodeElementInProgram(className.clone(), program.clone()) {
    } else {
        Error::addMessage(
            Error::LOOKUP_ERROR.clone(),
            list![
                AbsynUtil::pathString(className.clone(), literal!("."), true, false)?,
                literal!("<TOP>")
            ],
        )?;
        return Ok(result);
    }
    ErrorExt::setCheckpoint(literal!("CevalScriptBackend.getClassDiagram"));
    nf_inst = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    if '__try0: {
        (_, builtin_p) = unwrap_break_err!(FBuiltin::getInitialFunctions(), '__try0);
        annotation_p = unwrap_break_err!(AbsynToSCode::translateAbsyn2SCode(unwrap_break_err!(InteractiveUtil::modelicaAnnotationProgram(unwrap_break_err!(Config::getAnnotationVersion(), '__try0)), '__try0)), '__try0);
        result = unwrap_break_err!(NFClassDiagram::generate(className.clone(), listAppend(builtin_p.clone(), program.clone()), annotation_p.clone(), format, depth, exclude.clone(), showModifiers), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    FlagsUtil::set(Flags::SCODE_INST.clone(), nf_inst)?;
    ErrorExt::rollBack(literal!("CevalScriptBackend.getClassDiagram"));
    if !(stringEmpty(&fileName)) && !(stringEmpty(&result)) {
        System::writeFile(fileName.clone(), result)?;
        result = fileName;
    }
    Ok(result)
}

fn getDependencyGraph(
    mut scope: metamodelica::Ref<Absyn::Path>,
    mut fileName: ArcStr,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut defs: metamodelica::List<NFUsedElements::Definition>;
    let mut uses: metamodelica::List<NFUsedElements::Use>;
    program = SymbolTable::getSCode()?;
    (defs, uses, _) = collectDefUse(scopeClassPaths(scope.clone(), program.clone())?, program)?;
    result = NFDefUseChains::dependencyGraphJSON(
        AbsynUtil::pathString(scope, literal!("."), true, false)?,
        &defs,
        &uses,
        prettyPrint,
    )?;
    if !(stringEmpty(&fileName)) {
        System::writeFile(fileName.clone(), result)?;
        result = fileName;
    }
    Ok(result)
}

fn getDefinitionAt(mut fileName: ArcStr, mut line: i32, mut column: i32, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut found: Option<(metamodelica::Ref<Absyn::Path>, ArcStr)> = None;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut file: ArcStr = fileName.clone();
    let mut real_path: ArcStr;
    let mut defs: metamodelica::List<NFUsedElements::Definition> = metamodelica::nil();
    let mut uses: metamodelica::List<NFUsedElements::Use> = metamodelica::nil();
    program = SymbolTable::getSCode()?;
    real_path = System::realpath(fileName.clone())?;
    for mut c in &*program {
        found = findClassAt(
            metamodelica::AsArg::as_arg(&c),
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?,
            }),
            fileName.clone(),
            real_path.clone(),
            line,
            column,
            found,
        )?;
    }
    if (found).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(found) {
            Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        path = metamodelica::Own::own(__pa0);
        file = metamodelica::Own::own(__pa1);
        (defs, uses, _) = collectDefUse(
            metamodelica::cons(path.clone(), enclosingReplacingClasses(path, program.clone())?),
            program,
        )?;
    }
    result = NFDefUseChains::definitionAtJSON(file, line, column, &defs, uses, prettyPrint)?;
    Ok(result)
}

fn enclosingReplacingClasses(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut p: metamodelica::Ref<Absyn::Path> = path;
    let mut cls: metamodelica::Ref<SCode::Element>;
    while AbsynUtil::pathIsQual(&p) {
        p = AbsynUtil::stripLast(&p)?;
        if '__try0: {
            cls =
                unwrap_break_err!(InteractiveUtil::getPathedSCodeElementInProgram(p.clone(), program.clone()), '__try0);
            for mut e in &*SCodeUtil::getClassElements(&cls) {
                if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
                    && (SCodeUtil::isClassExtends(metamodelica::AsArg::as_arg(&e))
                        || unwrap_break_err!(SCodeUtil::isElementRedeclare(metamodelica::AsArg::as_arg(&e)), '__try0))
                {
                    paths = metamodelica::cons(
                        AbsynUtil::suffixPath(
                            &p,
                            &(unwrap_break_err!(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e)), '__try0)),
                        ),
                        paths.clone(),
                    );
                }
            }
            paths = metamodelica::cons(p.clone(), paths.clone());
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    Ok(paths)
}

fn findClassAt(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut fileName: ArcStr,
    mut realPath: ArcStr,
    mut line: i32,
    mut column: i32,
    mut found: Option<(metamodelica::Ref<Absyn::Path>, ArcStr)>,
) -> Result<Option<(metamodelica::Ref<Absyn::Path>, ArcStr)>> {
    let mut found: Option<(metamodelica::Ref<Absyn::Path>, ArcStr)> = found;
    let mut info: SourceInfo;
    if !(SCodeUtil::elementIsClass(element)) {
        return Ok(found);
    }
    info = SCodeUtil::elementInfo(element);
    if (info.fileName.clone() == fileName.clone() || info.fileName.clone() == realPath.clone())
        && (line > info.lineNumberStart.clone()
            || line == info.lineNumberStart.clone() && column >= info.columnNumberStart.clone())
        && (line < info.lineNumberEnd.clone()
            || line == info.lineNumberEnd.clone() && column <= info.columnNumberEnd.clone())
    {
        found = Some((path.clone(), info.fileName.clone()));
    }
    for mut e in &*SCodeUtil::getClassElements(element) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e)) {
            found = findClassAt(
                metamodelica::AsArg::as_arg(&e),
                AbsynUtil::suffixPath(&path, &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?)),
                fileName.clone(),
                realPath.clone(),
                line,
                column,
                found,
            )?;
        }
    }
    Ok(found)
}

fn isAllLoadedClasses(mut path: &metamodelica::Ref<Absyn::Path>) -> bool {
    let mut res: bool = AbsynUtil::pathEqual(
        path,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("AllLoadedClasses"),
        })),
    );
    res
}

fn scopeClassPaths(
    mut scope: metamodelica::Ref<Absyn::Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
    let mut cls: metamodelica::Ref<SCode::Element>;
    if isAllLoadedClasses(&scope) {
        for mut c in &*program {
            if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&c))
                && !metamodelica::stringEq(
                    &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?),
                    &(literal!("OpenModelica")),
                )
            {
                paths = getNestedClassPaths(
                    metamodelica::AsArg::as_arg(&c),
                    &(metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?,
                    })),
                    metamodelica::cons(
                        metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: SCodeUtil::elementName(metamodelica::AsArg::as_arg(&c))?,
                        }),
                        paths,
                    ),
                )?;
            }
        }
    } else {
        if '__try0: {
            cls = unwrap_break_err!(InteractiveUtil::getPathedSCodeElementInProgram(scope.clone(), program.clone()), '__try0);
            paths = unwrap_break_err!(getNestedClassPaths(&cls, &(scope.clone()), list![scope.clone()]), '__try0);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok(paths)
}

fn collectDefUse(
    mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<NFUsedElements::Definition>,
    metamodelica::List<NFUsedElements::Use>,
    metamodelica::List<NFUsedElements::Use>,
)> {
    let mut defs: metamodelica::List<NFUsedElements::Definition> = metamodelica::nil();
    let mut uses: metamodelica::List<NFUsedElements::Use> = metamodelica::nil();
    let mut unresolved: metamodelica::List<NFUsedElements::Use> = metamodelica::nil();
    let mut builtin_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut annotation_p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut nf_inst: bool;
    ErrorExt::setCheckpoint(literal!("CevalScriptBackend.collectDefUse"));
    nf_inst = FlagsUtil::set(Flags::SCODE_INST.clone(), true)?;
    if '__try0: {
        (_, builtin_p) = unwrap_break_err!(FBuiltin::getInitialFunctions(), '__try0);
        annotation_p = unwrap_break_err!(AbsynToSCode::translateAbsyn2SCode(unwrap_break_err!(InteractiveUtil::modelicaAnnotationProgram(unwrap_break_err!(Config::getAnnotationVersion(), '__try0)), '__try0)), '__try0);
        (defs, uses, unresolved) = unwrap_break_err!(NFUsedElements::collectUses(&(paths.clone().reverse()), listAppend(builtin_p.clone(), program.clone()), annotation_p.clone()), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    FlagsUtil::set(Flags::SCODE_INST.clone(), nf_inst)?;
    ErrorExt::rollBack(literal!("CevalScriptBackend.collectDefUse"));
    Ok((defs, uses, unresolved))
}

fn getNestedClassPaths(
    mut cls: &metamodelica::Ref<SCode::Element>,
    mut clsPath: &metamodelica::Ref<Absyn::Path>,
    mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = paths;
    let mut path: metamodelica::Ref<Absyn::Path>;
    for mut e in &*SCodeUtil::getClassElements(cls) {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e)) {
            path = AbsynUtil::suffixPath(clsPath, &(SCodeUtil::elementName(metamodelica::AsArg::as_arg(&e))?));
            paths = getNestedClassPaths(
                metamodelica::AsArg::as_arg(&e),
                &(path.clone()),
                metamodelica::cons(path, paths),
            )?;
        }
    }
    Ok(paths)
}

fn markElementUsed(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
) -> Result<()> {
    UnorderedSet::add(NFUsedElements::elementKey(element)?, used.clone())?;
    if SCodeUtil::elementIsClass(element) {
        for mut e in &*SCodeUtil::getClassElements(element) {
            if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e))
                || SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e))
            {
                markElementUsed(metamodelica::AsArg::as_arg(&e), used.clone())?;
            }
        }
    }
    Ok(())
}

fn filterUsedClasses(
    mut elements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPackage: bool,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    for mut e in &**elements {
        if SCodeUtil::elementIsClass(metamodelica::AsArg::as_arg(&e)) {
            if UnorderedSet::contains(
                NFUsedElements::elementKey(metamodelica::AsArg::as_arg(&e))?,
                used.clone(),
            )? {
                outElements =
                    metamodelica::cons(filterUsedNestedClasses(e.clone(), used.clone(), &program)?, outElements);
            }
        } else if inPackage && SCodeUtil::isComponent(metamodelica::AsArg::as_arg(&e)) {
            if UnorderedSet::contains(
                NFUsedElements::elementKey(metamodelica::AsArg::as_arg(&e))?,
                used.clone(),
            )? {
                outElements = metamodelica::cons(e.clone(), outElements);
            }
        } else if !(isUnusedImport(metamodelica::AsArg::as_arg(&e), used.clone(), program.clone())) {
            outElements = metamodelica::cons(e.clone(), outElements);
        }
    }
    outElements = outElements.reverse();
    Ok(outElements)
}

fn filterUsedNestedClasses(
    mut cls: metamodelica::Ref<SCode::Element>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    mut program: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cls: metamodelica::Ref<SCode::Element> = cls;
    let () = (match &*cls {
        SCode::Element::CLASS {
            classDef: __cls_classDef,
            ..
        } => {
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = filterUsedClassDef(__cls_classDef.clone(), used, program, SCodeUtil::isPackage(&cls))?);
            ()
        }
        _ => (),
    });
    Ok(cls)
}

fn filterUsedClassDef(
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    mut program: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPackage: bool,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut classDef: metamodelica::Ref<SCode::ClassDef> = classDef;
    let () = (match &*classDef {
        SCode::ClassDef::PARTS {
            elementLst: __classDef_elementLst,
            ..
        } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = filterUsedClasses(metamodelica::AsArg::as_arg(&__classDef_elementLst), used, program.clone(), inPackage)?);
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __classDef_composition,
            ..
        } => {
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = filterUsedClassDef(__classDef_composition.clone(), used, program, inPackage)?);
            ()
        }
        _ => (),
    });
    Ok(classDef)
}

fn isUnusedImport(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> bool {
    let mut unused: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    unused = (match &**element {
        SCode::Element::IMPORT {
            imp: Absyn::Import::NAMED_IMPORT { path: __esc_path, .. },
            ..
        } => {
            path = (*__esc_path).clone();
            isUnusedElementPath(path.clone(), used, program)
        }
        SCode::Element::IMPORT {
            imp: Absyn::Import::QUAL_IMPORT { path: __esc_path },
            ..
        } => {
            path = (*__esc_path).clone();
            isUnusedElementPath(path.clone(), used, program)
        }
        _ => false,
    });
    unused
}

fn isUnusedElementPath(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut used: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> bool {
    let mut unused: bool;
    match '__try0: {
        unused = !(unwrap_break_err!(UnorderedSet::contains(unwrap_break_err!(NFUsedElements::elementKey(&(unwrap_break_err!(InteractiveUtil::getPathedSCodeElementInProgram(path.clone(), program.clone()), '__try0))), '__try0), used.clone()), '__try0));
        Ok::<_, &'static str>((unused.clone(),))
    } {
        Ok((__try0_o0,)) => {
            unused = __try0_o0;
        }
        Err(_) => {
            unused = false;
        }
    }
    unused
}

fn saveTotalModelDebug(
    mut filename: ArcStr,
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<()> {
    let mut prog: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut r#str: ArcStr;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut str3: ArcStr;
    let mut cls_path: metamodelica::Ref<Absyn::Path> = classPath;
    let mut ocmt: Option<metamodelica::Ref<SCode::Comment>>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    loadProgram(&cls_path)?;
    prog = SymbolTable::getSCode()?;
    prog = TotalModelDebug::getTotalModel(prog, &cls_path)?;
    prog = SCodeUtil::removeBuiltinsFromTopScope(prog)?;
    ocmt = SCodeUtil::getElementComment(
        &(InteractiveUtil::getPathedSCodeElementInProgram(cls_path.clone(), prog.clone())?),
    );
    cmt = if ((ocmt).is_some()) {
        Util::getOption(ocmt)?
    } else {
        SCode::noComment.clone()
    };
    if stripAnnotations || stripComments {
        prog = SCodeUtil::stripCommentsFromProgram(prog, stripAnnotations, stripComments)?;
    }
    if obfuscate {
        (prog, cls_path, cmt, _, _) = Obfuscate::obfuscateProgram(prog, cls_path, cmt)?;
    }
    r#str = SCodeDump::programStr(prog, SCodeDump::defaultOptions.clone())?;
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*AbsynUtil::pathLastIdent(&cls_path));
        __mm_s.push_str(&*literal!("_total"));
        ArcStr::from(__mm_s)
    };
    str2 = if (stripComments) {
        literal!("")
    } else {
        SCodeDump::printCommentStr(&cmt, SCodeDump::defaultOptions.clone())?
    };
    str2 = if (stringEq(&str2, &(literal!("")))) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*str2);
            ArcStr::from(__mm_s)
        }
    };
    str3 = if (stripAnnotations) {
        literal!("")
    } else {
        SCodeDump::printAnnotationStr(&cmt, SCodeDump::defaultOptions.clone())?
    };
    str3 = if (stringEq(&str3, &(literal!("")))) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*str3);
            __mm_s.push_str(&*literal!(";\n"));
            ArcStr::from(__mm_s)
        }
    };
    str1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nmodel "));
        __mm_s.push_str(&*str1);
        __mm_s.push_str(&*str2);
        __mm_s.push_str(&*literal!("\n  extends "));
        __mm_s.push_str(&*AbsynUtil::pathString(cls_path, literal!("."), true, false)?);
        __mm_s.push_str(&*literal!(";\n"));
        __mm_s.push_str(&*str3);
        __mm_s.push_str(&*literal!("end "));
        __mm_s.push_str(&*str1);
        __mm_s.push_str(&*literal!(";\n"));
        ArcStr::from(__mm_s)
    };
    System::writeFile(filename, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*str1);
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

fn getDymolaStateAnnotation(mut className: metamodelica::Ref<Absyn::Path>, mut p: Absyn::Program) -> Result<bool> {
    let mut isState: bool;
    isState = (match p.clone() {
        _ => {
            let mut stateStr: ArcStr;
            stateStr = ProgramUtil::getNamedAnnotationExp(
                className,
                p,
                &(metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("__Dymola_state"),
                })),
                Some(literal!("false")),
                &fnptr!(
                    getDymolaStateAnnotationModStr,
                    Option<metamodelica::Ref<Absyn::Modification>>
                ),
            )?;
            stringEq(&stateStr, &(literal!("true")))
        }
    });
    Ok(isState)
}

fn getDymolaStateAnnotationModStr(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> ArcStr {
    let mut stateStr: ArcStr = arcstr::literal!("");
    stateStr = 'mc: {
        let __mc_input = r#mod;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut stateStr: ArcStr = stateStr.clone();
                    stateStr = Dump::printExpStr(e.clone())?;
                    Ok((stateStr.clone(), stateStr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            stateStr = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("false"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    stateStr
}

fn getClassInformation(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut p: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res_1: metamodelica::Ref<Values::Value>;
    let mut name: ArcStr;
    let mut file: ArcStr;
    let mut res: ArcStr;
    let mut cmt: ArcStr;
    let mut version: ArcStr;
    let mut preferredView: ArcStr;
    let mut access: ArcStr;
    let mut versionDate: ArcStr;
    let mut versionBuild: ArcStr;
    let mut dateModified: ArcStr;
    let mut revisionId: ArcStr;
    let mut lastIdent: ArcStr;
    let mut partialPrefix: bool;
    let mut finalPrefix: bool;
    let mut encapsulatedPrefix: bool;
    let mut isReadOnly: bool;
    let mut isProtectedClass: bool;
    let mut isDocClass: bool;
    let mut isState: bool;
    let mut restr: Absyn::Restriction;
    let mut cdef: metamodelica::Ref<Absyn::ClassDef>;
    let mut sl: i32;
    let mut sc: i32;
    let mut el: i32;
    let mut ec: i32;
    let mut classPath: metamodelica::Ref<Absyn::Path>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(path.clone(), &p, false, false)?) {
        Deref @ Absyn::Class { name: __pa0, partialPrefix: __pa1, finalPrefix: __pa2, encapsulatedPrefix: __pa3, restriction: __pa4, body: __pa5, commentsBeforeClass: _, commentsBeforeEnd: _, commentsAfterEnd: _, info: SourceInfo { fileName: __pa6, isReadOnly: __pa7, lineNumberStart: __pa8, columnNumberStart: __pa9, lineNumberEnd: __pa10, columnNumberEnd: __pa11, lastModification: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    partialPrefix = metamodelica::Own::own(__pa1);
    finalPrefix = metamodelica::Own::own(__pa2);
    encapsulatedPrefix = metamodelica::Own::own(__pa3);
    restr = metamodelica::Own::own(__pa4);
    cdef = metamodelica::Own::own(__pa5);
    file = metamodelica::Own::own(__pa6);
    isReadOnly = metamodelica::Own::own(__pa7);
    sl = metamodelica::Own::own(__pa8);
    sc = metamodelica::Own::own(__pa9);
    el = metamodelica::Own::own(__pa10);
    ec = metamodelica::Own::own(__pa11);
    res = Dump::unparseRestrictionStr(restr)?;
    cmt = getClassDefComment(&cdef);
    file = Testsuite::friendly(file)?;
    if AbsynUtil::pathIsIdent(&(AbsynUtil::makeNotFullyQualified(path.clone()))) {
        isProtectedClass = false;
    } else {
        lastIdent = AbsynUtil::pathLastIdent(&(AbsynUtil::makeNotFullyQualified(path.clone())));
        classPath = AbsynUtil::stripLast(&path)?;
        isProtectedClass = Interactive::isProtectedClass(classPath, &lastIdent, &p);
    }
    isDocClass = Interactive::getDocumentationClassAnnotation(path.clone(), p.clone())?;
    version = CevalScript::getPackageVersion(path.clone(), p.clone())?;
    preferredView = Interactive::getStringNamedAnnotation(
        path.clone(),
        p.clone(),
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("preferredView"),
        })),
    );
    isState = getDymolaStateAnnotation(path.clone(), p.clone())?;
    access = Interactive::getAccessAnnotation(path.clone(), p.clone())?;
    versionDate = Interactive::getStringNamedAnnotation(
        path.clone(),
        p.clone(),
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("versionDate"),
        })),
    );
    versionBuild = Interactive::getIntegerNamedAnnotation(
        path.clone(),
        &p,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("versionBuild"),
        })),
    );
    dateModified = Interactive::getStringNamedAnnotation(
        path.clone(),
        p.clone(),
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("dateModified"),
        })),
    );
    revisionId = Interactive::getStringNamedAnnotation(
        path,
        p,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("revisionId"),
        })),
    );
    res_1 = metamodelica::Ref::new(Values::Value::TUPLE {
        valueLst: list![
            metamodelica::Ref::new(Values::Value::STRING { string: res }),
            metamodelica::Ref::new(Values::Value::STRING { string: cmt }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: partialPrefix }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: finalPrefix }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: encapsulatedPrefix
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: file }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isReadOnly }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: sl }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: sc }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: el }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: ec }),
            getClassDimensions(&cdef)?,
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: isProtectedClass
            }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isDocClass }),
            metamodelica::Ref::new(Values::Value::STRING { string: version }),
            metamodelica::Ref::new(Values::Value::STRING { string: preferredView }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isState }),
            metamodelica::Ref::new(Values::Value::STRING { string: access }),
            metamodelica::Ref::new(Values::Value::STRING { string: versionDate }),
            metamodelica::Ref::new(Values::Value::STRING { string: versionBuild }),
            metamodelica::Ref::new(Values::Value::STRING { string: dateModified }),
            metamodelica::Ref::new(Values::Value::STRING { string: revisionId })
        ],
    });
    Ok(res_1)
}

fn getClassDimensions(mut cdef: &metamodelica::Ref<Absyn::ClassDef>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = (::match_deref::match_deref! { match cdef {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(ad), .. }, .. } => {
            ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut d in (ad.clone()).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: Dump::printSubscriptStr(&(d.clone()))? });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        _ => {
            ValuesMake::makeArray(metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(v)
}

fn getClassElementComment(mut element: &metamodelica::Ref<Absyn::Element>) -> ArcStr {
    let mut commentStr: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    commentStr = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls, .. }, constrainClass: __element_constrainClass, .. } => {
            cls = (*__esc_cls).clone();
            commentStr = InteractiveUtil::getConstrainingClassComment(__element_constrainClass.clone());
            if stringEmpty(&commentStr) {
                commentStr = getClassDefComment(&cls.body);
            }
            commentStr
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    commentStr
}

fn getClassDefComment(mut inClassDef: &metamodelica::Ref<Absyn::ClassDef>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inClassDef {
        Absyn::ClassDef::PARTS {
            comment: Some(r#str), ..
        } => r#str.clone(),
        Absyn::ClassDef::DERIVED { comment: cmt, .. } => Interactive::getStringComment(cmt.clone()),
        Absyn::ClassDef::ENUMERATION { comment: cmt, .. } => Interactive::getStringComment(cmt.clone()),
        Absyn::ClassDef::ENUMERATION { comment: cmt, .. } => Interactive::getStringComment(cmt.clone()),
        Absyn::ClassDef::OVERLOAD { comment: cmt, .. } => Interactive::getStringComment(cmt.clone()),
        Absyn::ClassDef::CLASS_EXTENDS {
            comment: Some(r#str), ..
        } => r#str.clone(),
        _ => {
            literal!("")
        }
    });
    outString
}

fn getAnnotationInEquation(mut inEquationItem: &metamodelica::Ref<Absyn::EquationItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inEquationItem {
        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annotations }), comment: _ }), .. } => {
            let mut annotationStr: ArcStr;
            let mut annotationList: metamodelica::List<ArcStr>;
            annotationList = getAnnotationInEquationElArgs(metamodelica::AsArg::as_arg(&annotations))?;
            annotationStr = stringDelimitList(annotationList, literal!(", "));
            annotationStr
        },
        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: None, .. } => {
            literal!("")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getAnnotationInEquationElArgs(
    mut inElArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = &**inElArgLst;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: _ }), .. }, tail: rest } => {
                    let mut fargs: metamodelica::Ref<Absyn::FunctionArgs>;
                    let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut env: FCore::Graph;
                    let mut newexp: metamodelica::Ref<DAE::Exp>;
                    let mut gexpstr: ArcStr;
                    let mut res: metamodelica::List<ArcStr>;
                    let mut cache: FCore::Cache;
                    let mut prop: DAE::Properties;
                    let mut lineProgram: Absyn::Program;
                    lineProgram = InteractiveUtil::modelicaAnnotationProgram(Config::getAnnotationVersion()?)?;
                    fargs = Interactive::createFuncargsFromElementargs(metamodelica::AsArg::as_arg(&r#mod))?;
                    p_1 = AbsynToSCode::translateAbsyn2SCode(lineProgram.clone())?;
                    (cache, env) = Inst::makeEnvFromProgram(&p_1)?;
                    (_, newexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: annName.clone(), subscripts: metamodelica::nil() }), functionArgs: fargs.clone(), typeVars: metamodelica::nil() }), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                    (cache, newexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), newexp.clone(), prop.clone(), false, metamodelica::sourceInfo!("Script/CevalScriptBackend.mo"))?;
                    Print::clearErrorBuf();
                    gexpstr = ExpressionBasics::printExpStr(newexp.clone())?;
                    res = getAnnotationInEquationElArgs(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(metamodelica::cons(gexpstr.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: annName }, modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), .. }, tail: rest } => {
                    let mut gexpstr_1: ArcStr;
                    let mut res: metamodelica::List<ArcStr>;
                    gexpstr_1 = stringAppendList(list![annName.clone(), literal!("(error)")]);
                    res = getAnnotationInEquationElArgs(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(metamodelica::cons(gexpstr_1.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

fn getTransitions(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut p: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut transitions: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    cdef = ProgramUtil::getPathedClassInProgram(path, p, false, false)?;
    transitions = getTransitionsInClass(&cdef)?.reverse();
    res = ValuesMake::makeArray(List::map(transitions, &ValuesMake::makeStringArray)?);
    Ok(res)
}

fn getTransitionsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut outTransitions: metamodelica::List<metamodelica::List<ArcStr>>;
    outTransitions = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut transitions: metamodelica::List<metamodelica::List<ArcStr>>;
            transitions = getTransitionsInClassParts(metamodelica::AsArg::as_arg(&parts));
            transitions
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut transitions: metamodelica::List<metamodelica::List<ArcStr>>;
            transitions = getTransitionsInClassParts(metamodelica::AsArg::as_arg(&parts));
            transitions
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTransitions)
}

fn getTransitionsInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::List<ArcStr>> {
    let mut outTransitions: metamodelica::List<metamodelica::List<ArcStr>>;
    outTransitions = 'mc: {
        let __mc_input = &**inAbsynClassPartLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist }, tail: xs } => {
                    let mut transitions1: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut transitions2: metamodelica::List<metamodelica::List<ArcStr>>;
                    transitions1 = getTransitionsInEquations(eqlist.clone(), metamodelica::nil())?;
                    transitions2 = getTransitionsInClassParts(metamodelica::AsArg::as_arg(&xs));
                    Ok(listAppend(transitions1.clone(), transitions2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut transitions1: metamodelica::List<metamodelica::List<ArcStr>>;
                    transitions1 = getTransitionsInClassParts(metamodelica::AsArg::as_arg(&xs));
                    Ok(transitions1.clone())
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
        panic!("matchcontinue: no arm matched")
    };
    outTransitions
}

fn getTransitionsInEquations(
    mut inAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inTransitions: metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynEquationItemLst, inTransitions.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: eqItem @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq @ Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "transition", .. }, .. }, .. }, tail: xs }, transitions) => {
                let mut transition: metamodelica::List<ArcStr>;
                let mut transitions = (*transitions).clone();
                transition = getTransitionInEquation(metamodelica::AsArg::as_arg(&eq))?;
                transition = List::insert(transition.clone(), ((transition).len() as i32) + 1, getAnnotationInEquation(metamodelica::AsArg::as_arg(&eqItem))?)?;
                transitions = listAppend(list![transition], transitions.clone());
                { (inAbsynEquationItemLst, inTransitions) = (xs.clone(), transitions.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, _) => {
                { (inAbsynEquationItemLst, inTransitions) = (xs.clone(), inTransitions); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inTransitions)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getTransitionInEquation(mut inEquation: &metamodelica::Ref<Absyn::Equation>) -> Result<metamodelica::List<ArcStr>> {
    let mut outTransition: metamodelica::List<ArcStr>;
    outTransition = (::match_deref::match_deref! { match inEquation {
        Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, argNames: namedArgs }, .. } => {
            let mut transition: metamodelica::List<ArcStr>;
            transition = List::map(expArgs.clone(), &Dump::printExpStr)?;
            transition = Interactive::addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("immediate")), &(literal!("true")), transition, 4)?;
            transition = Interactive::addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("reset")), &(literal!("true")), transition, 5)?;
            transition = Interactive::addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("synchronize")), &(literal!("false")), transition, 6)?;
            transition = Interactive::addOrUpdateNamedArg(metamodelica::AsArg::as_arg(&namedArgs), &(literal!("priority")), &(literal!("1")), transition, 7)?;
            transition
        },
        _ => {
            list![literal!(""), literal!(""), literal!(""), literal!("true"), literal!("true"), literal!("false"), literal!("1")]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTransition)
}

fn getInitialStates(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut p: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut initialStates: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    cdef = ProgramUtil::getPathedClassInProgram(path, p, false, false)?;
    initialStates = getInitialStatesInClass(&cdef)?.reverse();
    res = ValuesMake::makeArray(List::map(initialStates, &ValuesMake::makeStringArray)?);
    Ok(res)
}

fn getInitialStatesInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut outInitialStates: metamodelica::List<metamodelica::List<ArcStr>>;
    outInitialStates = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut initialStates: metamodelica::List<metamodelica::List<ArcStr>>;
            initialStates = getInitialStatesInClassParts(metamodelica::AsArg::as_arg(&parts));
            initialStates
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut initialStates: metamodelica::List<metamodelica::List<ArcStr>>;
            initialStates = getInitialStatesInClassParts(metamodelica::AsArg::as_arg(&parts));
            initialStates
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInitialStates)
}

fn getInitialStatesInClassParts(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::List<ArcStr>> {
    let mut outInitialStates: metamodelica::List<metamodelica::List<ArcStr>>;
    outInitialStates = 'mc: {
        let __mc_input = &**inAbsynClassPartLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist }, tail: xs } => {
                    let mut initialStates1: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut initialStates2: metamodelica::List<metamodelica::List<ArcStr>>;
                    initialStates1 = getInitialStatesInEquations(eqlist.clone(), metamodelica::nil())?;
                    initialStates2 = getInitialStatesInClassParts(metamodelica::AsArg::as_arg(&xs));
                    Ok(listAppend(initialStates1.clone(), initialStates2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut initialStates1: metamodelica::List<metamodelica::List<ArcStr>>;
                    initialStates1 = getInitialStatesInClassParts(metamodelica::AsArg::as_arg(&xs));
                    Ok(initialStates1.clone())
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
        panic!("matchcontinue: no arm matched")
    };
    outInitialStates
}

fn getInitialStatesInEquations(
    mut inAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut inInitialStates: metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynEquationItemLst, inInitialStates.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: eqItem @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq @ Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, .. }, .. }, tail: xs }, initialStates) => {
                let mut initialState: metamodelica::List<ArcStr>;
                let mut initialStates = (*initialStates).clone();
                initialState = getInitialStateInEquation(metamodelica::AsArg::as_arg(&eq))?;
                initialState = List::insert(initialState.clone(), ((initialState).len() as i32) + 1, getAnnotationInEquation(metamodelica::AsArg::as_arg(&eqItem))?)?;
                initialStates = listAppend(list![initialState], initialStates.clone());
                { (inAbsynEquationItemLst, inInitialStates) = (xs.clone(), initialStates.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, _) => {
                { (inAbsynEquationItemLst, inInitialStates) = (xs.clone(), inInitialStates); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inInitialStates)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getInitialStateInEquation(
    mut inEquation: &metamodelica::Ref<Absyn::Equation>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outInitialState: metamodelica::List<ArcStr>;
    outInitialState = (::match_deref::match_deref! { match inEquation {
        Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, .. }, .. } => {
            let mut initialState: metamodelica::List<ArcStr>;
            initialState = List::map(expArgs.clone(), &Dump::printExpStr)?;
            initialState
        },
        _ => {
            list![literal!("")]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInitialState)
}

fn addInitialState(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut state: ArcStr,
    mut inAbsynNamedArgLst: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut b: bool;
    let mut outProgram: Absyn::Program;
    (b, outProgram) = addInitialStateWithAnnotation(
        inPath,
        state,
        InteractiveUtil::annotationListToAbsyn(inAbsynNamedArgLst)?,
        inProgram,
    );
    Ok((b, outProgram))
}

fn addInitialStateWithAnnotation(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut state: ArcStr,
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut inProgram: Absyn::Program,
) -> (bool, Absyn::Program) {
    let mut b: bool;
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut package_: metamodelica::Ref<Absyn::Path>;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    let mut newcdef: metamodelica::Ref<Absyn::Class>;
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    match '__try0: {
        cdef =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(inPath.clone(), &inProgram, false, false), '__try0);
        cmt = Some(metamodelica::Ref::new(Absyn::Comment {
            annotation_: Some(inAnnotation.clone()),
            comment: None,
        }));
        newcdef = unwrap_break_err!(InteractiveUtil::addToEquation(cdef.clone(), metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL { functionName: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("initialState"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: state.clone(), subscripts: metamodelica::nil() }) })], argNames: metamodelica::nil() }) }), comment: cmt.clone(), info: Absyn::dummyInfo.clone() })), '__try0);
        if AbsynUtil::pathIsIdent(&(AbsynUtil::makeNotFullyQualified(inPath.clone()))) {
            outProgram = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: inProgram.within_.clone() }, inProgram.clone(), false, false), '__try0);
        } else {
            package_ = unwrap_break_err!(AbsynUtil::stripLast(&inPath), '__try0);
            outProgram = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: package_.clone() } }, inProgram.clone(), false, false), '__try0);
        }
        b = true;
        Ok::<_, &'static str>((b.clone(),))
    } {
        Ok((__try0_o0,)) => {
            b = __try0_o0;
        }
        Err(_) => {
            b = false;
        }
    }
    (b, outProgram)
}

fn deleteInitialState(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut state: ArcStr,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut b: bool;
    let mut outProgram: Absyn::Program;
    (b, outProgram) = 'mc: {
        let __mc_input = (inPath, state, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, state_, p @ Absyn::Program { .. }) => {
                    let mut modelwithin: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newcdef: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    cdef = ProgramUtil::getPathedClassInProgram(modelpath.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    newcdef = deleteInitialStateInClass(cdef.clone(), state_.clone())?;
                    if AbsynUtil::pathIsIdent(&(AbsynUtil::makeNotFullyQualified(modelpath.clone()))) {
                        newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false, false)?;
                    } else {
                        modelwithin = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&modelpath))?;
                        newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false, false)?;
                    }
                    Ok((true, newp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((b, outProgram))
}

fn deleteInitialStateInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut state: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. } => {
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = InteractiveUtil::getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = deleteInitialStateInEqlist(eqlst, state)?;
            parts2 = InteractiveUtil::replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. } => {
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = InteractiveUtil::getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = deleteInitialStateInEqlist(eqlst, state)?;
            parts2 = InteractiveUtil::replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn deleteInitialStateInEqlist(
    mut inEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut state: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    fn is_matching_initial_state(
        mut item: &metamodelica::Ref<Absyn::EquationItem>,
        mut state: &ArcStr,
    ) -> Result<bool> {
        let mut isMatch: bool;
        let mut name: metamodelica::Ref<Absyn::ComponentRef>;
        let mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
        isMatch = (::match_deref::match_deref! { match item {
            Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __esc_args, .. } }, .. } if (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&name), &(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("initialState"), subscripts: metamodelica::nil() })))?) => {
                args = (*__esc_args).clone();
                !((args).is_empty()) && metamodelica::stringEq(&state, &(Dump::printExpStr((args).head().cloned()?)?))
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(isMatch)
    }

    let mut outEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    outEqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
        for mut e in (inEqs).into_iter().cloned() {
            if !(!(is_matching_initial_state(&(e.clone()), &state)?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEqs)
}

fn getComponentInfo(
    mut comp: &metamodelica::Ref<Absyn::Element>,
    mut inEnv: Interactive::GraphicEnvCache,
    mut isProtected: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut vs: metamodelica::List<metamodelica::Ref<Values::Value>>;
    vs = (::match_deref::match_deref! { match comp {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: _ }, .. }, finalPrefix: __comp_finalPrefix, innerOuter: __comp_innerOuter, redeclareKeywords: __comp_redeclareKeywords, .. } => {
            let mut p_1: metamodelica::Ref<Absyn::Path> = <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
            let mut typename: ArcStr;
            let mut inout_str: ArcStr;
            let mut variability_str: ArcStr;
            let mut dir_str: ArcStr;
            let mut name: ArcStr;
            let mut comment: ArcStr;
            let mut r_1: bool;
            let mut dims: metamodelica::List<ArcStr>;
            let mut dims1: metamodelica::List<ArcStr>;
            let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            typename = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut p_1: metamodelica::Ref<Absyn::Path>;
            (_, p_1) = Interactive::mkFullyQual(inEnv.clone(), p.clone(), false)?;
            Ok(AbsynUtil::pathString(p_1.clone(), literal!("."), true, false)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?)
        })() { break 'mc __v; }
        return Err("matchcontinue: no arm matched")
    };
            vs = metamodelica::nil();
            dims1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut sub in (attr.arrayDim.clone()).into_iter().cloned() {
            let __x = Dump::printSubscriptStr(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            r_1 = Interactive::keywordReplaceable(__comp_redeclareKeywords.clone());
            inout_str = AbsynUtil::innerOuterStr(__comp_innerOuter.clone());
            variability_str = attrVariabilityStr(metamodelica::AsArg::as_arg(&attr))?;
            dir_str = attrDirectionStr(metamodelica::AsArg::as_arg(&attr))?;
            for mut ci in &*var_field!((**spec).components, Absyn::ElementSpec::COMPONENTS).clone() {
                (name, comment) = getComponentitemsName(metamodelica::AsArg::as_arg(&ci))?;
                let __arc1 = ci.clone();
                let Absyn::COMPONENTITEM { component: Absyn::COMPONENT { arrayDim: __pa0, .. }, .. } = &*__arc1;
                subs = metamodelica::Own::own(__pa0);
                dims = listAppend(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut sub in (subs).into_iter().cloned() {
            let __x = Dump::printSubscriptStr(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), dims1.clone());
                vs = metamodelica::cons(makeGetComponentsRecord(typename.clone(), name, comment, isProtected, __comp_finalPrefix.clone(), attr.flowPrefix.clone(), attr.streamPrefix.clone(), r_1, variability_str.clone(), inout_str.clone(), dir_str.clone(), dims), vs);
            }
            vs
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(vs)
}

fn makeGetComponentsRecord(
    mut className: ArcStr,
    mut name: ArcStr,
    mut comment: ArcStr,
    mut isProtected: bool,
    mut isFinal: bool,
    mut isFlow: bool,
    mut isStream: bool,
    mut isReplaceable: bool,
    mut variability: ArcStr,
    mut innerOuter: ArcStr,
    mut inputOutput: ArcStr,
    mut dimensions: metamodelica::List<ArcStr>,
) -> metamodelica::Ref<Values::Value> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("OpenModelica"),
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("Scripting"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("getComponentsTest"),
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("Component"),
                    }),
                }),
            }),
        }),
        orderd: list![
            metamodelica::Ref::new(Values::Value::STRING { string: className }),
            metamodelica::Ref::new(Values::Value::STRING { string: name }),
            metamodelica::Ref::new(Values::Value::STRING { string: comment }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isProtected }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isFinal }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isFlow }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isStream }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: isReplaceable }),
            metamodelica::Ref::new(Values::Value::STRING { string: variability }),
            metamodelica::Ref::new(Values::Value::STRING { string: innerOuter }),
            metamodelica::Ref::new(Values::Value::STRING { string: inputOutput }),
            ValuesMake::makeArray(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                    for mut s in (dimensions).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(Values::Value::STRING { string: s.clone() });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            )
        ],
        comp: list![
            literal!("className"),
            literal!("name"),
            literal!("comment"),
            literal!("isProtected"),
            literal!("isFinal"),
            literal!("isFlow"),
            literal!("isStream"),
            literal!("isReplaceable"),
            literal!("variability"),
            literal!("innerOuter"),
            literal!("inputOutput"),
            literal!("dimensions")
        ],
        index: -1,
    });
    v
}

fn attrVariabilityStr(mut inElementAttributes: &Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            variability: Absyn::Variability::VAR { .. },
            ..
        } => literal!(""),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::DISCRETE { .. },
            ..
        } => literal!("discrete"),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::PARAM { .. },
            ..
        } => literal!("parameter"),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::CONST { .. },
            ..
        } => literal!("constant"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn attrDirectionStr(mut inElementAttributes: &Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            direction: Absyn::Direction::INPUT { .. },
            ..
        } => literal!("input"),
        Absyn::ElementAttributes {
            direction: Absyn::Direction::OUTPUT { .. },
            ..
        } => literal!("output"),
        Absyn::ElementAttributes {
            direction: Absyn::Direction::BIDIR { .. },
            ..
        } => literal!(""),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn getComponentitemsName(mut ci: &metamodelica::Ref<Absyn::ComponentItem>) -> Result<(ArcStr, ArcStr)> {
    let mut name: ArcStr;
    let mut comment: ArcStr;
    (name, comment) = (::match_deref::match_deref! { match ci {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: c1, .. }, comment: Some(Deref @ Absyn::Comment { annotation_: _, comment: Some(s2) }), .. } => {
            (c1.clone(), s2.clone())
        },
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: c1, .. }, comment: Some(Deref @ Absyn::Comment { annotation_: _, comment: _ }), .. } => {
            (c1.clone(), literal!(""))
        },
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: c1, .. }, comment: None, .. } => {
            (c1.clone(), literal!(""))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((name, comment))
}

fn getModelFigures(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut ofigs: Option<metamodelica::List<metamodelica::Ref<Values::Value>>>;
    let mut figs: metamodelica::List<metamodelica::Ref<Values::Value>>;
    cls = ProgramUtil::getPathedClassInProgram(classPath, program, false, false)?;
    ofigs = AbsynUtil::getNamedAnnotationInClass(
        &cls,
        &(metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Documentation"),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("figures"),
            }),
        })),
        &figuresFromMod,
    );
    figs = (::match_deref::match_deref! { match &(ofigs) {
        Some(__esc_figs) => {
            figs = (*__esc_figs).clone();
            figs.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = ValuesMake::makeArray(figs);
    Ok(result)
}

fn figuresFromMod(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut figures: metamodelica::List<metamodelica::Ref<Values::Value>>;
    figures = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }) => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut e in (figureExpElements(exp.clone())).into_iter().cloned() {
            let __x = figureValue(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(figures)
}

fn figureExpElements(mut exp: metamodelica::Ref<Absyn::Exp>) -> metamodelica::List<metamodelica::Ref<Absyn::Exp>> {
    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    exps = (match &*exp {
        Absyn::Exp::ARRAY {
            arrayExp: __exp_arrayExp,
        } => __exp_arrayExp.clone(),
        _ => list![exp],
    });
    exps
}

fn figureValue(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>;
    let mut plots: metamodelica::List<metamodelica::Ref<Values::Value>>;
    args = figureArgs(
        exp,
        list![
            literal!("title"),
            literal!("identifier"),
            literal!("group"),
            literal!("preferred"),
            literal!("plots"),
            literal!("caption")
        ],
    )?;
    plots = (::match_deref::match_deref! { match &(figureArgExp(&args, &(literal!("plots")))) {
        Some(e) => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut p in (figureExpElements(e.clone())).into_iter().cloned() {
            let __x = plotValue(&(p.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.Figure"),
        }),
        orderd: list![
            figureStringArg(&args, &(literal!("title")), literal!("")),
            figureStringArg(&args, &(literal!("identifier")), literal!("")),
            figureStringArg(&args, &(literal!("group")), literal!("")),
            figureBoolArg(&args, &(literal!("preferred")), false),
            ValuesMake::makeArray(plots),
            figureStringArg(&args, &(literal!("caption")), literal!(""))
        ],
        comp: list![
            literal!("title"),
            literal!("identifier"),
            literal!("group"),
            literal!("preferred"),
            literal!("plots"),
            literal!("caption")
        ],
        index: -1,
    });
    Ok(value)
}

fn plotValue(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>;
    let mut curves: metamodelica::List<metamodelica::Ref<Values::Value>>;
    args = figureArgs(
        exp,
        list![
            literal!("title"),
            literal!("identifier"),
            literal!("curves"),
            literal!("x"),
            literal!("y")
        ],
    )?;
    curves = (::match_deref::match_deref! { match &(figureArgExp(&args, &(literal!("curves")))) {
        Some(e) => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut c in (figureExpElements(e.clone())).into_iter().cloned() {
            let __x = curveValue(&(c.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.Plot"),
        }),
        orderd: list![
            figureStringArg(&args, &(literal!("title")), literal!("")),
            figureStringArg(&args, &(literal!("identifier")), literal!("")),
            ValuesMake::makeArray(curves),
            axisValue(figureArgExp(&args, &(literal!("x"))))?,
            axisValue(figureArgExp(&args, &(literal!("y"))))?
        ],
        comp: list![
            literal!("title"),
            literal!("identifier"),
            literal!("curves"),
            literal!("x"),
            literal!("y")
        ],
        index: -1,
    });
    Ok(value)
}

fn curveValue(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>;
    args = figureArgs(
        exp,
        list![literal!("x"), literal!("y"), literal!("legend"), literal!("zOrder")],
    )?;
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.Curve"),
        }),
        orderd: list![
            figureRefArg(&args, &(literal!("x")), literal!("time"))?,
            figureRefArg(&args, &(literal!("y")), literal!(""))?,
            figureStringArg(&args, &(literal!("legend")), literal!("")),
            figureIntArg2(&args, &(literal!("zOrder")), 0)
        ],
        comp: list![literal!("x"), literal!("y"), literal!("legend"), literal!("zOrder")],
        index: -1,
    });
    Ok(value)
}

fn axisValue(mut oexp: Option<metamodelica::Ref<Absyn::Exp>>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>;
    args = (::match_deref::match_deref! { match &(oexp.clone()) {
        Some(_) => figureArgs(&(Util::getOption(oexp)?), list![literal!("min"), literal!("max"), literal!("unit"), literal!("label"), literal!("scale")])?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.Axis"),
        }),
        orderd: list![
            figureRealBoundArg(&args, &(literal!("min")))?,
            figureRealBoundArg(&args, &(literal!("max")))?,
            figureStringArg(&args, &(literal!("unit")), literal!("")),
            figureStringArg(&args, &(literal!("label")), literal!("")),
            axisScaleValue(figureArgExp(&args, &(literal!("scale"))))?
        ],
        comp: list![
            literal!("min"),
            literal!("max"),
            literal!("unit"),
            literal!("label"),
            literal!("scale")
        ],
        index: -1,
    });
    Ok(value)
}

fn axisScaleValue(mut oexp: Option<metamodelica::Ref<Absyn::Exp>>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut scaleType: ArcStr = literal!("Linear");
    let mut base: i32 = 10;
    let _ = (::match_deref::match_deref! { match &(oexp) {
        Some(Deref @ Absyn::Exp::CALL { function_: cr, functionArgs: fargs, .. }) => {
            scaleType = AbsynUtil::crefIdent(metamodelica::AsArg::as_arg(&cr))?;
            base = figureIntArg(&(figureArgs(&(metamodelica::Ref::new(Absyn::Exp::CALL { function_: cr.clone(), functionArgs: fargs.clone(), typeVars: metamodelica::nil() })), list![literal!("base")])?), &(literal!("base")), 10);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.AxisScale"),
        }),
        orderd: list![
            metamodelica::Ref::new(Values::Value::STRING { string: scaleType }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: base })
        ],
        comp: list![literal!("scaleType"), literal!("base")],
        index: -1,
    });
    Ok(value)
}

fn figureArgs(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
    mut fieldNames: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>> {
    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)> = metamodelica::nil();
    let mut pos: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut named: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    let mut names: metamodelica::List<ArcStr> = fieldNames;
    let mut name: ArcStr;
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __esc_pos, argNames: __esc_named }, .. } => {
            pos = (*__esc_pos).clone();
            named = (*__esc_named).clone();
            for mut e in &*pos.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                name = metamodelica::Own::own(__pa0);
                names = metamodelica::Own::own(__pa1);
                args = metamodelica::cons((name, e.clone()), args);
            }
            for mut na in &*named.clone() {
                args = metamodelica::cons((na.argName.clone(), na.argValue.clone()), args);
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(args)
}

fn figureArgExp(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut oexp: Option<metamodelica::Ref<Absyn::Exp>> = None;
    let mut n: ArcStr;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    for mut a in &**args {
        (n, e) = a.clone();
        if metamodelica::stringEq(&n, &name) {
            oexp = Some(e);
            return oexp;
        }
    }
    oexp
}

fn figureStringArg(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: ArcStr,
) -> metamodelica::Ref<Values::Value> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(Deref @ Absyn::Exp::STRING { .. }) => metamodelica::Ref::new(Values::Value::STRING { string: figureArgString(args, name, default) }),
        _ => metamodelica::Ref::new(Values::Value::STRING { string: default }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

fn figureArgString(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: ArcStr,
) -> ArcStr {
    let mut value: ArcStr;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(Deref @ Absyn::Exp::STRING { value: s }) => {
            s.clone()
        },
        _ => {
            default
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

fn figureRefArg(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(e) => {
            metamodelica::Ref::new(Values::Value::STRING { string: Dump::printExpStr(e.clone())? })
        },
        _ => {
            metamodelica::Ref::new(Values::Value::STRING { string: default })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn figureBoolArg(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: bool,
) -> metamodelica::Ref<Values::Value> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(Deref @ Absyn::Exp::BOOL { value: b }) => {
            metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() })
        },
        _ => {
            metamodelica::Ref::new(Values::Value::BOOL { boolean: default })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

fn figureIntArg(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: i32,
) -> i32 {
    let mut value: i32;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(Deref @ Absyn::Exp::INTEGER { value: i }) => {
            i.clone()
        },
        _ => {
            default
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

fn figureRealBoundArg(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (::match_deref::match_deref! { match &(figureArgExp(args, name)) {
        Some(e) => {
            ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::REAL { real: figureExpReal(metamodelica::AsArg::as_arg(&e))? })])
        },
        _ => {
            ValuesMake::makeArray(metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn figureExpReal(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (match &**exp {
        Absyn::Exp::INTEGER { value: i } => intReal(i.clone()),
        Absyn::Exp::REAL { value: s } => stringReal(s.clone())?,
        Absyn::Exp::UNARY {
            op: Absyn::Operator::UMINUS { .. },
            exp: __exp_exp,
        } => -(figureExpReal(metamodelica::AsArg::as_arg(&__exp_exp))?),
        _ => metamodelica::OrderedFloat(0.0_f64),
    });
    Ok(value)
}

fn figureIntArg2(
    mut args: &metamodelica::List<(ArcStr, metamodelica::Ref<Absyn::Exp>)>,
    mut name: &ArcStr,
    mut default: i32,
) -> metamodelica::Ref<Values::Value> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = metamodelica::Ref::new(Values::Value::INTEGER {
        integer: figureIntArg(args, name, default),
    });
    value
}

fn getAnnotationNamedModifiers(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut annotationName: ArcStr,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    fn get_names(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> Result<metamodelica::List<ArcStr>> {
        let mut names: metamodelica::List<ArcStr>;
        let mut m: metamodelica::Ref<Absyn::Modification>;
        let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
        names = (::match_deref::match_deref! { match &(r#mod) {
            Some(__esc_m) => {
                m = (*__esc_m).clone();
                paths = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
            for mut a in (m.elementArgLst.clone()).into_iter().cloned() {
                let __x = AbsynUtil::elementArgName(&(a.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut p in (paths).into_iter().cloned() {
                if !(AbsynUtil::pathIsIdent(&(p.clone()))) { continue; }
                let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
            },
            _ => metamodelica::nil(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(names)
    }

    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut names: metamodelica::List<ArcStr>;
    cls = ProgramUtil::getPathedClassInProgram(classPath, program, false, false)?;
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(&cls, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: annotationName })), &get_names)) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    names = metamodelica::Own::own(__pa0);
    result = ValuesMake::makeStringArray(names)?;
    Ok(result)
}

fn getOptModifierValue(
    mut modifier: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(modifier) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#mod = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &(r#mod.eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { exp: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa1);
    value = ValuesUtil::absynExpValue(&exp)?;
    Ok(value)
}

fn getAnnotationModifierValue(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut annotationName: ArcStr,
    mut modifierName: ArcStr,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    cls = ProgramUtil::getPathedClassInProgram(classPath, program, false, false)?;
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(&cls, &(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: annotationName, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: modifierName }) })), &getOptModifierValue)) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    result = metamodelica::Own::own(__pa0);
    Ok(result)
}

fn makeLoadLibrariesEntryAbsyn(
    mut cl: &metamodelica::Ref<Absyn::Class>,
    mut acc: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut out: metamodelica::List<metamodelica::Ref<Values::Value>>;
    out = (::match_deref::match_deref! { match cl {
        Deref @ Absyn::Class { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. } => {
            acc
        },
        Deref @ Absyn::Class { name, info: SourceInfo { fileName, .. }, .. } => {
            let mut dir: ArcStr;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut b: bool;
            let mut fileName = (*fileName).clone();
            dir = System::dirname(fileName.clone());
            fileName = System::basename(fileName.clone());
            v = ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::STRING { string: name.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: dir.clone() })]);
            b = stringEq(&fileName, &(literal!("ModelicaBuiltin.mo"))) || stringEq(&fileName, &(literal!("MetaModelicaBuiltin.mo"))) || stringEq(&dir, &(literal!(".")));
            List::consOnTrue(!(b), v, acc)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

fn selectResultFile(mut resultFile: ArcStr, mut simflags: ArcStr) -> Result<ArcStr> {
    let mut resultFile: ArcStr = resultFile;
    let mut nm: i32;
    let mut f: ArcStr = literal!("");
    if System::stringFind(simflags.clone(), literal!("-r"))? < 0 {
        return Ok(resultFile);
    }
    if '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r=\"(.*?)\""), 2, true, false)) {
            (__pa1, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa1);
        f = metamodelica::Own::own(__pa2);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r='(.*?)'"), 2, true, false)) {
            (__pa4, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa4.clone(), __pa5.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa4);
        f = metamodelica::Own::own(__pa5);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r[ ]*\"(.*?)\""), 2, true, false)) {
            (__pa7, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa7.clone(), __pa8.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa7);
        f = metamodelica::Own::own(__pa8);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r[ ]*'(.*?)'"), 2, true, false)) {
            (__pa10, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa10.clone(), __pa11.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa10);
        f = metamodelica::Own::own(__pa11);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        let (__pa13, __pa14) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r=([^ ]*)"), 2, true, false)) {
            (__pa13, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa14, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa13.clone(), __pa14.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa13);
        f = metamodelica::Own::own(__pa14);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        let (__pa16, __pa17) = ::match_deref::match_deref! { match &(System::regex(simflags.clone(), literal!("-r[ ]*([^ ]*)"), 2, true, false)) {
            (__pa16, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa17, tail: Deref @ metamodelica::ListNode::Nil } }) => (__pa16.clone(), __pa17.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        nm = metamodelica::Own::own(__pa16);
        f = metamodelica::Own::own(__pa17);
        if nm == 2 {
            resultFile = f.clone();
            return Ok(resultFile);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(resultFile)
}

fn instantiateModel(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: metamodelica::Ref<Values::Value>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut odae: Option<DAE::DAElist> = None;
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    if isProtectedContentAccess(&path)? {
        result = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
        return Ok((cache, result));
    }
    r#str = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut cache: FCore::Cache = cache.clone();
            let mut flags: Flags::Flag = flags.clone();
            let mut odae: Option<DAE::DAElist> = odae.clone();
            let mut r#str: ArcStr = r#str.clone();
            ExecStat::execStatReset()?;
            flags = loadCommandLineOptionsFromModel(path.clone())?;
            match '__try0: {
                (cache, _, odae, r#str) = unwrap_break_err!(runFrontEnd(cache.clone(), env.clone(), path.clone(), false, unwrap_break_err!(Config::flatModelica(), '__try0) && !(unwrap_break_err!(Config::silent(), '__try0)), false), '__try0);
                unwrap_break_err!(ExecStat::execStat(&(literal!("runFrontEnd"))), '__try0);
                if !(stringEmpty(&r#str)) {
                } else if (odae).is_none() {
                    r#str = literal!("");
                } else if unwrap_break_err!(Config::silent(), '__try0) {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("model "));
                        __mm_s.push_str(&*unwrap_break_err!(AbsynUtil::pathString(path.clone(), literal!("."), true, false), '__try0));
                        __mm_s.push_str(&*literal!("\n  /* Silent mode */\nend"));
                        __mm_s.push_str(&*unwrap_break_err!(AbsynUtil::pathString(path.clone(), literal!("."), true, false), '__try0));
                        __mm_s.push_str(&*literal!(";\n"));
                        ArcStr::from(__mm_s)
                    };
                } else {
                    r#str = unwrap_break_err!(DAEDump::dumpStr(unwrap_break_err!(Util::getOption(odae.clone()), '__try0), &(FCore::getFunctionTree(&cache))), '__try0);
                    unwrap_break_err!(ExecStat::execStat(&(literal!("DAEDump.dumpStr"))), '__try0);
                }
                FlagsUtil::saveFlags(flags.clone());
                Ok::<_, &'static str>((cache.clone(), odae.clone(), r#str.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    cache = __try0_o0;
                    odae = __try0_o1;
                    r#str = __try0_o2;
                }
                Err(__try0_err) => {
                    FlagsUtil::saveFlags(flags.clone());
                    return Err(__try0_err);
                }
            }
            Ok((r#str.clone(), cache.clone(), flags.clone(), odae.clone(), r#str.clone()))
        })() {
            cache = __wb0;
            flags = __wb1;
            odae = __wb2;
            r#str = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let false = (Interactive::existClass(path.clone(), &(SymbolTable::getAbsyn()))) else {
                return Err("pattern mismatch");
            };
            Error::addMessage(
                Error::LOOKUP_ERROR.clone(),
                list![
                    AbsynUtil::pathString(path.clone(), literal!("."), true, false)?,
                    literal!("<TOP>")
                ],
            )?;
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#str: ArcStr = r#str.clone();
            if Error::getNumMessages() == 0 {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Instantiation of "));
                    __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                    __mm_s.push_str(&*literal!(" failed with no error message"));
                    ArcStr::from(__mm_s)
                };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone(), literal!("<TOP>")])?;
            }
            Ok((literal!(""), r#str.clone()))
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    result = metamodelica::Ref::new(Values::Value::STRING { string: r#str });
    Ok((cache, result))
}

fn getConnectionList(mut className: metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut valList: metamodelica::Ref<Values::Value>;
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut annotation_sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut connList: metamodelica::List<metamodelica::List<ArcStr>>;
    annotation_sp = AbsynToSCode::translateAbsyn2SCode(InteractiveUtil::modelicaAnnotationProgram(
        Config::getAnnotationVersion()?,
    )?)?;
    (_, sp) = FBuiltin::getInitialFunctions()?;
    sp = listAppend(SymbolTable::getSCode()?, sp);
    connList = NFInst::instClassForConnection(className, sp, annotation_sp)?;
    valList = ValuesMake::makeArray(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
            for mut conn in (connList).into_iter().cloned() {
                let __x = ValuesMake::makeArray(List::map(conn.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    Ok(valList)
}

fn runConversionScript(
    mut clsPath: metamodelica::Ref<Absyn::Path>,
    mut scriptFile: ArcStr,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut p: Absyn::Program;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut wi: Absyn::Within;
    match '__try0: {
        p = SymbolTable::getAbsyn();
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(clsPath.clone(), &p, false, true), '__try0);
        cls = unwrap_break_err!(Conversion::convertPackage(cls.clone(), scriptFile.clone()), '__try0);
        wi = unwrap_break_err!(ProgramUtil::buildWithin(clsPath.clone()), '__try0);
        p = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: wi.clone() }, p.clone(), false, false), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(p.clone()), '__try0);
        res = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn convertPackageToLibrary(
    mut clsPath: metamodelica::Ref<Absyn::Path>,
    mut libPath: metamodelica::Ref<Absyn::Path>,
    mut libVersion: ArcStr,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut p: Absyn::Program;
    let mut lib_program: Absyn::Program;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut lib_cls: metamodelica::Ref<Absyn::Class>;
    let mut wi: Absyn::Within;
    let mut uses_version: Option<ArcStr>;
    let mut lib_version: SemanticVersion::Version;
    let mut lib_version_used: SemanticVersion::Version;
    let mut conversions: metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>;
    let mut scripts: metamodelica::List<ArcStr>;
    let mut lib_name: ArcStr;
    match '__try0: {
        p = SymbolTable::getAbsyn();
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(clsPath.clone(), &p, false, true), '__try0);
        uses_version = unwrap_break_err!(Interactive::getUsedVersion(cls.clone(), &libPath), '__try0);
        if (uses_version).is_some() {
            lib_version_used = unwrap_break_err!(SemanticVersion::parse(unwrap_break_err!(Util::getOption(uses_version.clone()), '__try0), true), '__try0);
        } else {
            unwrap_break_err!(Error::addMessage(Error::CONVERSION_MISSING_USES.clone(), list![unwrap_break_err!(AbsynUtil::pathString(clsPath.clone(), literal!("."), true, false), '__try0), unwrap_break_err!(AbsynUtil::pathString(libPath.clone(), literal!("."), true, false), '__try0)]), '__try0);
            break '__try0 Err::<_, _>("fail");
        }
        lib_name = AbsynUtil::pathFirstIdent(&libPath);
        lib_version = unwrap_break_err!(SemanticVersion::parse(unwrap_break_err!(CevalScript::getPackageVersion(libPath.clone(), p.clone()), '__try0), false), '__try0);
        if unwrap_break_err!(SemanticVersion::compare(&lib_version, &(unwrap_break_err!(SemanticVersion::parse(libVersion.clone(), false), '__try0)), true, false), '__try0)
            != 0
        {
            if metamodelica::stringEq(&lib_name, &(literal!("Modelica"))) {
                unwrap_break_err!(Config::setLanguageStandardFromMSL({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Modelica ")); __mm_s.push_str(&*libVersion); ArcStr::from(__mm_s) }, true), '__try0);
            }
            let (__pa1, true) = (unwrap_break_err!(CevalScript::loadModel(&(list![(libPath.clone(), lib_name.clone(), list![libVersion.clone()], false)]), unwrap_break_err!(Settings::getModelicaPath(unwrap_break_err!(Testsuite::isRunning(), '__try0)), '__try0), p.clone(), true, true, false, true, false, literal!("")), '__try0))
            else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            lib_program = metamodelica::Own::own(__pa1);
            unwrap_break_err!(SymbolTable::setAbsyn(lib_program.clone()), '__try0);
        } else {
            lib_program = p.clone();
        }
        lib_version = unwrap_break_err!(SemanticVersion::parse(unwrap_break_err!(CevalScript::getPackageVersion(libPath.clone(), lib_program.clone()), '__try0), false), '__try0);
        lib_cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(libPath.clone(), &lib_program, false, true), '__try0);
        conversions = Interactive::getConversionsInClass(&lib_cls);
        scripts = unwrap_break_err!(findConversionPaths(&conversions, &lib_version, &lib_version_used, 0), '__try0);
        if (scripts).is_empty() {
            unwrap_break_err!(Error::addMessage(Error::CONVERSION_NO_COMPATIBLE_SCRIPT_FOUND.clone(), list![unwrap_break_err!(AbsynUtil::pathString(libPath.clone(), literal!("."), true, false), '__try0), SemanticVersion::toString(&lib_version_used), SemanticVersion::toString(&lib_version)]), '__try0);
            break '__try0 Err::<_, _>("fail");
        }
        for mut script in &*scripts {
            let mut script = script.clone();
            script = unwrap_break_err!(uriToFilename(script.clone()), '__try0);
            cls = unwrap_break_err!(Conversion::convertPackage(cls.clone(), script.clone()), '__try0);
        }
        cls = unwrap_break_err!(Interactive::updateUsedVersion(cls.clone(), libPath.clone(), &(SemanticVersion::toString(&lib_version))), '__try0);
        wi = unwrap_break_err!(ProgramUtil::buildWithin(clsPath.clone()), '__try0);
        lib_program = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: wi.clone() }, lib_program.clone(), false, false), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(lib_program.clone()), '__try0);
        res = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn findConversionPaths(
    mut conversions: &metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>,
    mut libVersion: &SemanticVersion::Version,
    mut libVersionUsed: &SemanticVersion::Version,
    mut depth: i32,
) -> Result<metamodelica::List<ArcStr>> {
    let mut scripts: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut paths: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut path_len: i32;
    let mut path_min: i32 = 100;
    if depth > 100 {
        return Ok(scripts);
    }
    for mut c in &**conversions {
        paths = metamodelica::cons(
            findConversionPath(&(c.clone()), libVersion, libVersionUsed, conversions, depth)?,
            paths,
        );
    }
    for mut p in &*paths {
        path_len = ((p).len() as i32);
        if path_len > 0 && path_len < path_min {
            scripts = p.clone();
            path_min = path_len;
        }
    }
    Ok(scripts)
}

fn findConversionPath(
    mut conversion: &(ArcStr, Option<ArcStr>, Option<ArcStr>),
    mut libVersion: &SemanticVersion::Version,
    mut libVersionUsed: &SemanticVersion::Version,
    mut conversions: &metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>,
    mut depth: i32,
) -> Result<metamodelica::List<ArcStr>> {
    let mut scripts: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut from: ArcStr;
    let mut to: Option<ArcStr>;
    let mut script: Option<ArcStr>;
    let mut from_version: SemanticVersion::Version;
    let mut to_version: SemanticVersion::Version;
    (from, to, script) = conversion.clone();
    if (script).is_none() {
        return Ok(scripts);
    }
    from_version = SemanticVersion::parse(from, true)?;
    if SemanticVersion::compare(libVersionUsed, &from_version, true, false)? == 0 {
        if (to).is_some() {
            to_version = SemanticVersion::parse(Util::getOption(to)?, true)?;
            if SemanticVersion::compare(libVersion, &to_version, true, false)? != 0 {
                scripts = findConversionPaths(conversions, libVersion, &to_version, depth + 1)?;
            }
        }
        scripts = metamodelica::cons(Util::getOption(script)?, scripts);
    }
    Ok(scripts)
}

pub(crate) fn loadCommandLineOptionsFromModel(mut className: metamodelica::Ref<Absyn::Path>) -> Result<Flags::Flag> {
    let mut oldFlags: Flags::Flag;
    let mut opts: ArcStr;
    let mut args: metamodelica::List<ArcStr>;
    if Config::ignoreCommandLineOptionsAnnotation()? {
        oldFlags = FlagsUtil::loadFlags(true)?;
        return Ok(oldFlags);
    }
    loadProgram(&className)?;
    let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getNamedAnnotationExp(className, SymbolTable::getAbsyn(), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("__OpenModelica_commandLineOptions") })), Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })), &Interactive::getAnnotationExp)?) {
        Deref @ Absyn::Exp::STRING { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    opts = metamodelica::Own::own(__pa0);
    if !(stringEmpty(&opts)) {
        oldFlags = FlagsUtil::backupFlags()?;
        args = System::strtok(opts, literal!(" "));
        FlagsUtil::readArgs(args)?;
    } else {
        oldFlags = FlagsUtil::loadFlags(true)?;
    }
    Ok(oldFlags)
}

pub(crate) fn isProtectedContentAccess(mut className: &metamodelica::Ref<Absyn::Path>) -> Result<bool> {
    let mut restricted: bool;
    loadProgram(className)?;
    restricted = Interactive::astContainsEncryptedClass(SymbolTable::getAbsyn())?;
    if restricted {
        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
    }
    Ok(restricted)
}
