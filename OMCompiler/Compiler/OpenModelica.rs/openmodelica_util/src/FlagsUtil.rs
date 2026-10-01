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

use crate::Error;
use crate::Flags;
use crate::Global;
use crate::IOStream;
use crate::Print;
use crate::Settings;
use crate::StringUtil;
use crate::System;
use crate::Util;
use openmodelica_error::ErrorExt;
use openmodelica_util_datatypes_basic::List;

// This is a list of all debug flags, to keep track of which flags are used. A
// flag can not be used unless it's in this list, and the list is checked at
// initialization so that all flags are sorted by index (and thus have unique
// indices).
pub static allDebugFlags: std::sync::LazyLock<metamodelica::List<Flags::DebugFlag>> = std::sync::LazyLock::new(|| {
    list![
        Flags::FAILTRACE.clone(),
        Flags::CEVAL.clone(),
        Flags::CHECK_BACKEND_DAE.clone(),
        Flags::PTHREADS.clone(),
        Flags::EVENTS.clone(),
        Flags::DUMP_INLINE_SOLVER.clone(),
        Flags::EVAL_FUNC.clone(),
        Flags::GEN.clone(),
        Flags::DYN_LOAD.clone(),
        Flags::GENERATE_CODE_CHEAT.clone(),
        Flags::CGRAPH_GRAPHVIZ_FILE.clone(),
        Flags::CGRAPH_GRAPHVIZ_SHOW.clone(),
        Flags::GC_PROF.clone(),
        Flags::CHECK_DAE_CREF_TYPE.clone(),
        Flags::CHECK_ASUB.clone(),
        Flags::INSTANCE.clone(),
        Flags::CACHE.clone(),
        Flags::RML.clone(),
        Flags::TAIL.clone(),
        Flags::LOOKUP.clone(),
        Flags::PATTERNM_SKIP_FILTER_UNUSED_AS_BINDINGS.clone(),
        Flags::PATTERNM_ALL_INFO.clone(),
        Flags::PATTERNM_DCE.clone(),
        Flags::PATTERNM_MOVE_LAST_EXP.clone(),
        Flags::EXPERIMENTAL_REDUCTIONS.clone(),
        Flags::EVAL_PARAM.clone(),
        Flags::TYPES.clone(),
        Flags::SHOW_STATEMENT.clone(),
        Flags::DUMP.clone(),
        Flags::DUMP_GRAPHVIZ.clone(),
        Flags::EXEC_STAT.clone(),
        Flags::TRANSFORMS_BEFORE_DUMP.clone(),
        Flags::DAE_DUMP_GRAPHV.clone(),
        Flags::INTERACTIVE_TCP.clone(),
        Flags::INTERACTIVE_DUMP.clone(),
        Flags::RELIDX.clone(),
        Flags::DUMP_REPL.clone(),
        Flags::DUMP_FP_REPL.clone(),
        Flags::DUMP_PARAM_REPL.clone(),
        Flags::DUMP_PP_REPL.clone(),
        Flags::DUMP_EA_REPL.clone(),
        Flags::DEBUG_ALIAS.clone(),
        Flags::TEARING_DUMP.clone(),
        Flags::JAC_DUMP.clone(),
        Flags::JAC_DUMP2.clone(),
        Flags::DUMP_BINDINGS.clone(),
        Flags::DUMP_SORTING.clone(),
        Flags::DUMP_SPARSE.clone(),
        Flags::DUMP_SPARSE_VERBOSE.clone(),
        Flags::BLT_DUMP.clone(),
        Flags::DUMMY_SELECT.clone(),
        Flags::DUMP_DAE_LOW.clone(),
        Flags::DUMP_INDX_DAE.clone(),
        Flags::OPT_DAE_DUMP.clone(),
        Flags::EXEC_HASH.clone(),
        Flags::PARAM_DLOW_DUMP.clone(),
        Flags::DUMP_ENCAPSULATECONDITIONS.clone(),
        Flags::SHORT_OUTPUT.clone(),
        Flags::COUNT_OPERATIONS.clone(),
        Flags::CGRAPH.clone(),
        Flags::UPDMOD.clone(),
        Flags::STATIC.clone(),
        Flags::TPL_PERF_TIMES.clone(),
        Flags::CHECK_SIMPLIFY.clone(),
        Flags::SCODE_INST.clone(),
        Flags::WRITE_TO_BUFFER.clone(),
        Flags::DUMP_BACKENDDAE_INFO.clone(),
        Flags::GEN_DEBUG_SYMBOLS.clone(),
        Flags::DUMP_STATESELECTION_INFO.clone(),
        Flags::DUMP_EQNINORDER.clone(),
        Flags::SEMILINEAR.clone(),
        Flags::UNCERTAINTIES.clone(),
        Flags::SHOW_START_ORIGIN.clone(),
        Flags::DUMP_SIMCODE.clone(),
        Flags::DUMP_INITIAL_SYSTEM.clone(),
        Flags::GRAPH_INST.clone(),
        Flags::GRAPH_INST_RUN_DEP.clone(),
        Flags::GRAPH_INST_GEN_GRAPH.clone(),
        Flags::DUMP_CONST_REPL.clone(),
        Flags::SHOW_EQUATION_SOURCE.clone(),
        Flags::LS_ANALYTIC_JACOBIAN.clone(),
        Flags::NLS_ANALYTIC_JACOBIAN.clone(),
        Flags::INLINE_SOLVER.clone(),
        Flags::HPCOM.clone(),
        Flags::INITIALIZATION.clone(),
        Flags::INLINE_FUNCTIONS.clone(),
        Flags::DUMP_SCC_GRAPHML.clone(),
        Flags::TEARING_DUMPVERBOSE.clone(),
        Flags::DISABLE_SINGLE_FLOW_EQ.clone(),
        Flags::DUMP_DISCRETEVARS_INFO.clone(),
        Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone(),
        Flags::INFO_XML_OPERATIONS.clone(),
        Flags::HPCOM_DUMP.clone(),
        Flags::RESOLVE_LOOPS_DUMP.clone(),
        Flags::DISABLE_WINDOWS_PATH_CHECK_WARNING.clone(),
        Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone(),
        Flags::IMPL_ODE.clone(),
        Flags::EVAL_FUNC_DUMP.clone(),
        Flags::PRINT_STRUCTURAL.clone(),
        Flags::ITERATION_VARS.clone(),
        Flags::ALLOW_RECORD_TOO_MANY_FIELDS.clone(),
        Flags::HPCOM_MEMORY_OPT.clone(),
        Flags::DUMP_SYNCHRONOUS.clone(),
        Flags::STRIP_PREFIX.clone(),
        Flags::DO_SCODE_DEP.clone(),
        Flags::SHOW_INST_CACHE_INFO.clone(),
        Flags::DUMP_UNIT.clone(),
        Flags::DUMP_EQ_UNIT.clone(),
        Flags::DUMP_EQ_UNIT_STRUCT.clone(),
        Flags::SHOW_DAE_GENERATION.clone(),
        Flags::RESHUFFLE_POST.clone(),
        Flags::SHOW_EXPANDABLE_INFO.clone(),
        Flags::DUMP_HOMOTOPY.clone(),
        Flags::OMC_RELOCATABLE_FUNCTIONS.clone(),
        Flags::GRAPHML.clone(),
        Flags::USEMPI.clone(),
        Flags::DUMP_CSE.clone(),
        Flags::DUMP_CSE_VERBOSE.clone(),
        Flags::NO_START_CALC.clone(),
        Flags::CONSTJAC.clone(),
        Flags::VISUAL_XML.clone(),
        Flags::VECTORIZE.clone(),
        Flags::CHECK_EXT_LIBS.clone(),
        Flags::RUNTIME_STATIC_LINKING.clone(),
        Flags::SORT_EQNS_AND_VARS.clone(),
        Flags::DUMP_SIMPLIFY_LOOPS.clone(),
        Flags::DUMP_RTEARING.clone(),
        Flags::DIS_SYMJAC_FMI20.clone(),
        Flags::EVAL_OUTPUT_ONLY.clone(),
        Flags::HARDCODED_START_VALUES.clone(),
        Flags::DUMP_FUNCTIONS.clone(),
        Flags::DEBUG_DIFFERENTIATION.clone(),
        Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone(),
        Flags::FMU_EXPERIMENTAL.clone(),
        Flags::DUMP_DGESV.clone(),
        Flags::MULTIRATE_PARTITION.clone(),
        Flags::DUMP_EXCLUDED_EXP.clone(),
        Flags::DEBUG_ALGLOOP_JACOBIAN.clone(),
        Flags::DISABLE_JACSCC.clone(),
        Flags::FORCE_NLS_ANALYTIC_JACOBIAN.clone(),
        Flags::DUMP_LOOPS.clone(),
        Flags::DUMP_LOOPS_VERBOSE.clone(),
        Flags::SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR.clone(),
        Flags::OMC_RECORD_ALLOC_WORDS.clone(),
        Flags::TOTAL_TEARING_DUMP.clone(),
        Flags::TOTAL_TEARING_DUMPVERBOSE.clone(),
        Flags::PARALLEL_CODEGEN.clone(),
        Flags::SERIALIZED_SIZE.clone(),
        Flags::BACKEND_KEEP_ENV_GRAPH.clone(),
        Flags::DUMPBACKENDINLINE.clone(),
        Flags::DUMPBACKENDINLINE_VERBOSE.clone(),
        Flags::BLT_MATRIX_DUMP.clone(),
        Flags::LIST_REVERSE_WRONG_ORDER.clone(),
        Flags::PARTITION_INITIALIZATION.clone(),
        Flags::EVAL_PARAM_DUMP.clone(),
        Flags::NF_UNITCHECK.clone(),
        Flags::DISABLE_COLORING.clone(),
        Flags::MERGE_ALGORITHM_SECTIONS.clone(),
        Flags::WARN_NO_NOMINAL.clone(),
        Flags::REDUCE_DAE.clone(),
        Flags::IGNORE_CYCLES.clone(),
        Flags::ALIAS_CONFLICTS.clone(),
        Flags::SUSAN_MATCHCONTINUE_DEBUG.clone(),
        Flags::OLD_FE_UNITCHECK.clone(),
        Flags::EXEC_STAT_EXTRA_GC.clone(),
        Flags::DEBUG_DAEMODE.clone(),
        Flags::NF_SCALARIZE.clone(),
        Flags::NF_EVAL_CONST_ARG_FUNCS.clone(),
        Flags::NF_EXPAND_OPERATIONS.clone(),
        Flags::NF_API.clone(),
        Flags::NF_API_DYNAMIC_SELECT.clone(),
        Flags::NF_API_NOISE.clone(),
        Flags::FMI20_DEPENDENCIES.clone(),
        Flags::WARNING_MINMAX_ATTRIBUTES.clone(),
        Flags::NF_EXPAND_FUNC_ARGS.clone(),
        Flags::DUMP_JL.clone(),
        Flags::DUMP_ASSC.clone(),
        Flags::SPLIT_CONSTANT_PARTS_SYMJAC.clone(),
        Flags::DUMP_FORCE_FMI_ATTRIBUTES.clone(),
        Flags::DUMP_DATARECONCILIATION.clone(),
        Flags::ARRAY_CONNECT.clone(),
        Flags::COMBINE_SUBSCRIPTS.clone(),
        Flags::ZMQ_LISTEN_TO_ALL.clone(),
        Flags::DUMP_CONVERSION_RULES.clone(),
        Flags::PRINT_RECORD_TYPES.clone(),
        Flags::DUMP_SIMPLIFY.clone(),
        Flags::DUMP_BACKEND_CLOCKS.clone(),
        Flags::DUMP_SET_BASED_GRAPHS.clone(),
        Flags::MERGE_COMPONENTS.clone(),
        Flags::DUMP_SLICE.clone(),
        Flags::VECTORIZE_BINDINGS.clone(),
        Flags::DUMP_EVENTS.clone(),
        Flags::DUMP_RESIZABLE.clone(),
        Flags::DUMP_SOLVE.clone(),
        Flags::FORCE_SCALARIZE.clone(),
        Flags::DEBUG_ADJOINT.clone(),
        Flags::FLOW_ALIAS_ELIMINATION.clone(),
        Flags::DUMP_CHECK_MODEL.clone(),
        Flags::CHECK_DEF_USE.clone(),
        Flags::TEARING_COST.clone(),
        Flags::OMEDIT.clone()
    ]
});

// This is a list of all configuration flags. A flag can not be used unless it's
// in this list, and the list is checked at initialization so that all flags are
// sorted by index (and thus have unique indices).
pub static allConfigFlags: std::sync::LazyLock<metamodelica::List<Flags::ConfigFlag>> =
    std::sync::LazyLock::new(|| {
        list![
            Flags::DEBUG.clone(),
            Flags::HELP.clone(),
            Flags::RUNNING_TESTSUITE.clone(),
            Flags::SHOW_VERSION.clone(),
            Flags::TARGET.clone(),
            Flags::GRAMMAR.clone(),
            Flags::ANNOTATION_VERSION.clone(),
            Flags::LANGUAGE_STANDARD.clone(),
            Flags::SHOW_ERROR_MESSAGES.clone(),
            Flags::SHOW_ANNOTATIONS.clone(),
            Flags::NO_SIMPLIFY.clone(),
            Flags::PRE_OPT_MODULES.clone(),
            Flags::CHEAPMATCHING_ALGORITHM.clone(),
            Flags::MATCHING_ALGORITHM.clone(),
            Flags::INDEX_REDUCTION_METHOD.clone(),
            Flags::POST_OPT_MODULES.clone(),
            Flags::SIMCODE_TARGET.clone(),
            Flags::ORDER_CONNECTIONS.clone(),
            Flags::TYPE_INFO.clone(),
            Flags::KEEP_ARRAYS.clone(),
            Flags::MODELICA_OUTPUT.clone(),
            Flags::SILENT.clone(),
            Flags::NUM_PROC.clone(),
            Flags::INST_CLASS.clone(),
            Flags::VECTORIZATION_LIMIT.clone(),
            Flags::SIMULATION_CG.clone(),
            Flags::EVAL_PARAMS_IN_ANNOTATIONS.clone(),
            Flags::CHECK_MODEL.clone(),
            Flags::CEVAL_EQUATION.clone(),
            Flags::UNIT_CHECKING.clone(),
            Flags::GENERATE_LABELED_SIMCODE.clone(),
            Flags::REDUCE_TERMS.clone(),
            Flags::REDUCTION_METHOD.clone(),
            Flags::DEMO_MODE.clone(),
            Flags::LOCALE_FLAG.clone(),
            Flags::DEFAULT_OPENCL_DEVICE.clone(),
            Flags::MAXTRAVERSALS.clone(),
            Flags::DUMP_TARGET.clone(),
            Flags::DELAY_BREAK_LOOP.clone(),
            Flags::TEARING_METHOD.clone(),
            Flags::TEARING_HEURISTIC.clone(),
            Flags::SCALARIZE_MINMAX.clone(),
            Flags::STRICT.clone(),
            Flags::SCALARIZE_BINDINGS.clone(),
            Flags::HPCOM_SCHEDULER.clone(),
            Flags::HPCOM_CODE.clone(),
            Flags::REWRITE_RULES_FILE.clone(),
            Flags::REPLACE_HOMOTOPY.clone(),
            Flags::GENERATE_DYNAMIC_JACOBIAN.clone(),
            Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(),
            Flags::INT_ENUM_CONVERSION.clone(),
            Flags::PROFILING_LEVEL.clone(),
            Flags::RESHUFFLE.clone(),
            Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(),
            Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone(),
            Flags::CPP_FLAGS.clone(),
            Flags::REMOVE_SIMPLE_EQUATIONS.clone(),
            Flags::DYNAMIC_TEARING.clone(),
            Flags::SYM_SOLVER.clone(),
            Flags::LOOP2CON.clone(),
            Flags::FORCE_TEARING.clone(),
            Flags::SIMPLIFY_LOOPS.clone(),
            Flags::RTEARING.clone(),
            Flags::FLOW_THRESHOLD.clone(),
            Flags::MATRIX_FORMAT.clone(),
            Flags::PARTLINTORN.clone(),
            Flags::INIT_OPT_MODULES.clone(),
            Flags::MAX_MIXED_DETERMINED_INDEX.clone(),
            Flags::USE_LOCAL_DIRECTION.clone(),
            Flags::DEFAULT_OPT_MODULES_ORDERING.clone(),
            Flags::PRE_OPT_MODULES_ADD.clone(),
            Flags::PRE_OPT_MODULES_SUB.clone(),
            Flags::POST_OPT_MODULES_ADD.clone(),
            Flags::POST_OPT_MODULES_SUB.clone(),
            Flags::INIT_OPT_MODULES_ADD.clone(),
            Flags::INIT_OPT_MODULES_SUB.clone(),
            Flags::PERMISSIVE.clone(),
            Flags::HETS.clone(),
            Flags::DEFAULT_CLOCK_PERIOD.clone(),
            Flags::INST_CACHE_SIZE.clone(),
            Flags::MAX_SIZE_LINEAR_TEARING.clone(),
            Flags::MAX_SIZE_NONLINEAR_TEARING.clone(),
            Flags::NO_TEARING_FOR_COMPONENT.clone(),
            Flags::CT_STATE_MACHINES.clone(),
            Flags::DAE_MODE.clone(),
            Flags::INLINE_METHOD.clone(),
            Flags::SET_TEARING_VARS.clone(),
            Flags::SET_RESIDUAL_EQNS.clone(),
            Flags::IGNORE_COMMAND_LINE_OPTIONS_ANNOTATION.clone(),
            Flags::CALCULATE_SENSITIVITIES.clone(),
            Flags::ALARM.clone(),
            Flags::TOTAL_TEARING.clone(),
            Flags::IGNORE_SIMULATION_FLAGS_ANNOTATION.clone(),
            Flags::DYNAMIC_TEARING_FOR_INITIALIZATION.clone(),
            Flags::PREFER_TVARS_WITH_START_VALUE.clone(),
            Flags::EQUATIONS_PER_FILE.clone(),
            Flags::EVALUATE_FINAL_PARAMS.clone(),
            Flags::EVALUATE_PROTECTED_PARAMS.clone(),
            Flags::REPLACE_EVALUATED_PARAMS.clone(),
            Flags::CONDENSE_ARRAYS.clone(),
            Flags::WFC_ADVANCED.clone(),
            Flags::GRAPHICS_EXP_MODE.clone(),
            Flags::TEARING_STRICTNESS.clone(),
            Flags::INTERACTIVE.clone(),
            Flags::ZEROMQ_FILE_SUFFIX.clone(),
            Flags::HOMOTOPY_APPROACH.clone(),
            Flags::IGNORE_REPLACEABLE.clone(),
            Flags::LABELED_REDUCTION.clone(),
            Flags::DISABLE_EXTRA_LABELING.clone(),
            Flags::LOAD_MSL_MODEL.clone(),
            Flags::LOAD_PACKAGE_FILE.clone(),
            Flags::BUILDING_FMU.clone(),
            Flags::BUILDING_MODEL.clone(),
            Flags::POST_OPT_MODULES_DAE.clone(),
            Flags::EVAL_LOOP_LIMIT.clone(),
            Flags::EVAL_RECURSION_LIMIT.clone(),
            Flags::SINGLE_INSTANCE_AGLSOLVER.clone(),
            Flags::SHOW_STRUCTURAL_ANNOTATIONS.clone(),
            Flags::INITIAL_STATE_SELECTION.clone(),
            Flags::LINEARIZATION_DUMP_LANGUAGE.clone(),
            Flags::NO_ASSC.clone(),
            Flags::FULL_ASSC.clone(),
            Flags::REAL_ASSC.clone(),
            Flags::INIT_ASSC.clone(),
            Flags::MAX_SIZE_ASSC.clone(),
            Flags::USE_ZEROMQ_IN_SIM.clone(),
            Flags::ZEROMQ_PUB_PORT.clone(),
            Flags::ZEROMQ_SUB_PORT.clone(),
            Flags::ZEROMQ_JOB_ID.clone(),
            Flags::ZEROMQ_SERVER_ID.clone(),
            Flags::ZEROMQ_CLIENT_ID.clone(),
            Flags::FMI_VERSION.clone(),
            Flags::BASE_MODELICA.clone(),
            Flags::FMI_FILTER.clone(),
            Flags::FMI_SOURCES.clone(),
            Flags::FMI_FLAGS.clone(),
            Flags::FMU_CMAKE_BUILD.clone(),
            Flags::NEW_BACKEND.clone(),
            Flags::PARMODAUTO.clone(),
            Flags::INTERACTIVE_PORT.clone(),
            Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
            Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone(),
            Flags::LINK_TYPE.clone(),
            Flags::TEARING_ALWAYS_DERIVATIVES.clone(),
            Flags::DUMP_FLAT_MODEL.clone(),
            Flags::SIMULATION.clone(),
            Flags::OBFUSCATE.clone(),
            Flags::FMU_RUNTIME_DEPENDS.clone(),
            Flags::FRONTEND_INLINE.clone(),
            Flags::EXPOSE_LOCAL_IOS.clone(),
            Flags::BASE_MODELICA_FORMAT.clone(),
            Flags::BASE_MODELICA_OPTIONS.clone(),
            Flags::DEBUG_FOLLOW_EQUATIONS.clone(),
            Flags::MAX_SIZE_LINEARIZATION.clone(),
            Flags::RESIZABLE_ARRAYS.clone(),
            Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone(),
            Flags::LOAD_MISSING_LIBRARIES.clone(),
            Flags::CAUSALIZE_DAE_MODE.clone(),
            Flags::SIM_CODE_SCALARIZE.clone(),
            Flags::EXECUTE_COMMAND.clone(),
            Flags::MOO_DYNAMIC_OPTIMIZATION.clone(),
            Flags::FMI_EXTRA_ANNOTATIONS.clone(),
            Flags::INTERACTIVE_DUMP_FORMAT.clone(),
            Flags::EXPORT_FMU.clone(),
            Flags::FMU_TYPE.clone(),
            Flags::FMU_PLATFORMS.clone(),
            Flags::FMU_VERSION.clone(),
            Flags::TEARING_COST_MARGIN.clone(),
            Flags::FMU_NATIVE_PLATFORMS.clone(),
            Flags::TPL_OUTPUT_DIR.clone(),
            Flags::FMU_DIRECTORY.clone(),
            Flags::TPL_INTERFACE_DIR.clone()
        ]
    });

pub fn new(mut inArgs: metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut outArgs: metamodelica::List<ArcStr>;
    loadFlags(true)?;
    outArgs = readArgs(inArgs)?;
    Ok(outArgs)
}

pub fn saveFlags(mut inFlags: Flags::Flag) -> () {
    {
        let __v = inFlags;
        crate::Globals::flagsIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub(crate) fn createConfigFlags() -> metamodelica::Array<Flags::FlagData> {
    let mut configFlags: metamodelica::Array<Flags::FlagData>;
    configFlags = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<Flags::FlagData> = metamodelica::nil();
            for mut flag in (allConfigFlags.clone()).into_iter().cloned() {
                let __x = flag.defaultValue.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    configFlags
}

pub(crate) fn createDebugFlags() -> metamodelica::Array<bool> {
    let mut debugFlags: metamodelica::Array<bool>;
    debugFlags = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut flag in (allDebugFlags.clone()).into_iter().cloned() {
                let __x = flag.default.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    debugFlags
}

pub fn loadFlags(mut initialize: bool) -> Result<Flags::Flag> {
    let mut flags: Flags::Flag;
    match '__try0: {
        flags = Flags::getFlags(true);
        Ok::<_, &'static str>((flags.clone(),))
    } {
        Ok((__try0_o0,)) => {
            flags = __try0_o0;
        }
        Err(_) => {
            if initialize {
                checkDebugFlags()?;
                checkConfigFlags()?;
                flags = Flags::Flag::FLAGS {
                    debugFlags: createDebugFlags(),
                    configFlags: createConfigFlags(),
                };
                saveFlags(flags.clone());
                if StringUtil::startsWith(System::openModelicaPlatform(), literal!("msvc")) {
                    setConfigString(Flags::TARGET.clone(), literal!("msvc"))?;
                }
            } else {
                metamodelica::print(literal!("Flag loading failed!\n"));
                flags = crate::Flags::Flag::NO_FLAGS;
            }
        }
    }
    Ok(flags)
}

pub fn backupFlags() -> Result<Flags::Flag> {
    let mut outFlags: Flags::Flag;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: __pa1,
    } = (loadFlags(true)?)
    else {
        return Err("pattern mismatch");
    };
    debug_flags = metamodelica::Own::own(__pa0);
    config_flags = metamodelica::Own::own(__pa1);
    outFlags = Flags::Flag::FLAGS {
        debugFlags: metamodelica::arrayFromVec(debug_flags.clone().borrow().clone()),
        configFlags: metamodelica::arrayFromVec(config_flags.clone().borrow().clone()),
    };
    Ok(outFlags)
}

pub fn resetDebugFlags() -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS {
        debugFlags: _,
        configFlags: __pa0,
    } = (loadFlags(true)?)
    else {
        return Err("pattern mismatch");
    };
    config_flags = metamodelica::Own::own(__pa0);
    debug_flags = createDebugFlags();
    saveFlags(Flags::Flag::FLAGS {
        debugFlags: debug_flags.clone(),
        configFlags: config_flags.clone(),
    });
    Ok(())
}

pub fn resetConfigFlags() -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: _,
    } = (loadFlags(true)?)
    else {
        return Err("pattern mismatch");
    };
    debug_flags = metamodelica::Own::own(__pa0);
    config_flags = createConfigFlags();
    saveFlags(Flags::Flag::FLAGS {
        debugFlags: debug_flags.clone(),
        configFlags: config_flags.clone(),
    });
    Ok(())
}

fn checkDebugFlags() -> Result<()> {
    let mut index: i32 = 0;
    let mut err_str: ArcStr;
    for mut flag in &*allDebugFlags.clone() {
        index = index + 1;
        if flag.index.clone() != index {
            err_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Invalid flag '"));
                __mm_s.push_str(&*flag.name);
                __mm_s.push_str(&*literal!("' with index "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", flag.index.clone())));
                __mm_s.push_str(&*literal!(" (expected "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                __mm_s.push_str(&*literal!(
                    ") in Flags.allDebugFlags. Make sure that all flags are present and ordered correctly!"
                ));
                ArcStr::from(__mm_s)
            };
            Error::terminateError(err_str, &(metamodelica::sourceInfo!("Util/FlagsUtil.mo")))?;
            unreachable!("Error.terminateError always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

fn checkConfigFlags() -> Result<()> {
    let mut index: i32 = 0;
    let mut err_str: ArcStr;
    for mut flag in &*allConfigFlags.clone() {
        index = index + 1;
        if flag.index.clone() != index {
            err_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Invalid flag '"));
                __mm_s.push_str(&*flag.name);
                __mm_s.push_str(&*literal!("' with index "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", flag.index.clone())));
                __mm_s.push_str(&*literal!(" (expected "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                __mm_s.push_str(&*literal!(
                    ") in Flags.allConfigFlags. Make sure that all flags are present and ordered correctly!"
                ));
                ArcStr::from(__mm_s)
            };
            Error::terminateError(err_str, &(metamodelica::sourceInfo!("Util/FlagsUtil.mo")))?;
            unreachable!("Error.terminateError always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

pub fn set(mut inFlag: Flags::DebugFlag, mut inValue: bool) -> Result<bool> {
    let mut outOldValue: bool;
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: __pa1,
    } = (loadFlags(true)?)
    else {
        return Err("pattern mismatch");
    };
    debug_flags = metamodelica::Own::own(__pa0);
    config_flags = metamodelica::Own::own(__pa1);
    (debug_flags, outOldValue) = updateDebugFlagArray(debug_flags.clone(), inValue, inFlag)?;
    saveFlags(Flags::Flag::FLAGS {
        debugFlags: debug_flags.clone(),
        configFlags: config_flags.clone(),
    });
    Ok(outOldValue)
}

pub fn enableDebug(mut inFlag: Flags::DebugFlag) -> Result<bool> {
    let mut outOldValue: bool;
    outOldValue = set(inFlag, true)?;
    Ok(outOldValue)
}

pub fn disableDebug(mut inFlag: Flags::DebugFlag) -> Result<bool> {
    let mut outOldValue: bool;
    outOldValue = set(inFlag, false)?;
    Ok(outOldValue)
}

pub fn getConfigOptionsStringList(
    mut inFlag: &Flags::ConfigFlag,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outOptions: metamodelica::List<ArcStr>;
    let mut outComments: metamodelica::List<ArcStr>;
    (outOptions, outComments) = (match inFlag.clone() {
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: mut options }),
            ..
        } => (
            List::map(options.clone(), &fnptr!(Util::tuple21, _))?,
            List::map(options.clone(), &fnptr!(Util::tuple22, _))?,
        ),
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref flags }),
            ..
        } => (flags.clone(), List::fill(literal!(""), ((flags).len() as i32))),
        _ => return Err("match: no arm matched"),
    });
    Ok((outOptions, outComments))
}

fn updateDebugFlagArray(
    mut inFlags: metamodelica::Array<bool>,
    mut inValue: bool,
    mut inFlag: Flags::DebugFlag,
) -> Result<(metamodelica::Array<bool>, bool)> {
    let mut outFlags: metamodelica::Array<bool>;
    let mut outOldValue: bool;
    let mut index: i32;
    let Flags::DEBUG_FLAG { index: __pa0, .. } = inFlag;
    index = metamodelica::Own::own(__pa0);
    outOldValue = metamodelica::arrayGet(inFlags.clone(), index)?;
    outFlags = metamodelica::arrayUpdate(inFlags.clone(), index, inValue)?;
    Ok((outFlags, outOldValue))
}

fn updateConfigFlagArray(
    mut inFlags: metamodelica::Array<Flags::FlagData>,
    mut inValue: Flags::FlagData,
    mut inFlag: Flags::ConfigFlag,
) -> Result<metamodelica::Array<Flags::FlagData>> {
    let mut outFlags: metamodelica::Array<Flags::FlagData>;
    let mut index: i32;
    let Flags::CONFIG_FLAG { index: __pa0, .. } = &inFlag;
    index = metamodelica::Own::own(__pa0);
    outFlags = metamodelica::arrayUpdate(inFlags.clone(), index, inValue.clone())?;
    applySideEffects(inFlag, inValue);
    Ok(outFlags)
}

pub fn readArgs(mut inArgs: metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut outArgs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut flags: Flags::Flag;
    let mut numError: i32;
    let mut arg: ArcStr;
    let mut rest_args: metamodelica::List<ArcStr> = inArgs;
    numError = Error::getNumErrorMessages();
    flags = loadFlags(true)?;
    while !((rest_args).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        if metamodelica::stringEq(&arg, &(literal!("--"))) {
            break;
        } else {
            (rest_args, outArgs) = readArg(arg, &flags, rest_args, outArgs)?;
        }
    }
    outArgs = List::append_reverse(&outArgs, rest_args);
    List::map2(
        outArgs.clone(),
        &fnptr!(System::iconv, ArcStr, ArcStr, ArcStr),
        literal!("UTF-8"),
        literal!("UTF-8"),
    )?;
    Error::assertionOrAddSourceMessage(
        numError == Error::getNumErrorMessages(),
        &(Error::UTF8_COMMAND_LINE_ARGS.clone()),
        metamodelica::nil(),
        &(Util::dummyInfo.clone()),
    )?;
    saveFlags(flags);
    handleDeprecatedFlags()?;
    Ok(outArgs)
}

fn readArg(
    mut inArg: ArcStr,
    mut inFlags: &Flags::Flag,
    mut restArgs: metamodelica::List<ArcStr>,
    mut nonFlags: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut restArgs: metamodelica::List<ArcStr> = restArgs;
    let mut nonFlags: metamodelica::List<ArcStr> = nonFlags;
    let mut flagtype: ArcStr;
    let mut len: i32;
    flagtype = stringGetStringChar(inArg.clone(), 1)?;
    len = ((inArg).len() as i32);
    if metamodelica::stringEq(&flagtype, &(literal!("+"))) {
        if len == 1 {
            parseFlag(
                inArg,
                &(crate::Flags::Flag::NO_FLAGS),
                restArgs.clone(),
                &(literal!("")),
            )?;
        } else {
            restArgs = parseFlag(substring(inArg, 2, len)?, inFlags, restArgs, &flagtype)?;
        }
    } else if metamodelica::stringEq(&flagtype, &(literal!("-"))) {
        if len == 1 {
            parseFlag(
                inArg,
                &(crate::Flags::Flag::NO_FLAGS),
                restArgs.clone(),
                &(literal!("")),
            )?;
        } else if len == 2 {
            restArgs = parseFlag(substring(inArg, 2, 2)?, inFlags, restArgs, &flagtype)?;
        } else if metamodelica::stringEq(&(stringGetStringChar(inArg.clone(), 2)?), &(literal!("-"))) {
            if len < 4 || metamodelica::stringEq(&(stringGetStringChar(inArg.clone(), 4)?), &(literal!("="))) {
                parseFlag(
                    inArg,
                    &(crate::Flags::Flag::NO_FLAGS),
                    restArgs.clone(),
                    &(literal!("")),
                )?;
            } else {
                restArgs = parseFlag(substring(inArg, 3, len)?, inFlags, restArgs, &(literal!("--")))?;
            }
        } else {
            if metamodelica::stringEq(&(stringGetStringChar(inArg.clone(), 3)?), &(literal!("="))) {
                restArgs = parseFlag(substring(inArg, 2, len)?, inFlags, restArgs, &flagtype)?;
            } else {
                parseFlag(
                    inArg,
                    &(crate::Flags::Flag::NO_FLAGS),
                    restArgs.clone(),
                    &(literal!("")),
                )?;
            }
        }
    } else {
        nonFlags = metamodelica::cons(inArg, nonFlags);
    }
    Ok((restArgs, nonFlags))
}

fn parseFlag(
    mut inFlag: ArcStr,
    mut inFlags: &Flags::Flag,
    mut restArgs: metamodelica::List<ArcStr>,
    mut inFlagPrefix: &ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut restArgs: metamodelica::List<ArcStr> = restArgs;
    let mut flag: ArcStr;
    let mut values: metamodelica::List<ArcStr>;
    let mut value: ArcStr;
    let mut missing_value: bool;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok(inFlag.clone(), literal!("="))) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    flag = metamodelica::Own::own(__pa0);
    values = metamodelica::Own::own(__pa1);
    value = stringAppendList(values);
    missing_value = stringEmpty(&value) && !(StringUtil::endsWith(inFlag, literal!("=")));
    restArgs = parseConfigFlag(flag, value, inFlags, restArgs, inFlagPrefix, missing_value)?;
    Ok(restArgs)
}

fn parseConfigFlag(
    mut inFlag: ArcStr,
    mut inValue: ArcStr,
    mut inFlags: &Flags::Flag,
    mut restArgs: metamodelica::List<ArcStr>,
    mut inFlagPrefix: &ArcStr,
    mut missingValue: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut restArgs: metamodelica::List<ArcStr> = restArgs;
    let mut config_flag: Flags::ConfigFlag;
    let mut value: ArcStr;
    config_flag = lookupConfigFlag(inFlag, inFlagPrefix)?;
    if missingValue
        && flagRequiresValue(&config_flag)
        && !((restArgs).is_empty())
        && !(StringUtil::startsWith((restArgs).head().cloned()?, literal!("-")))
    {
        value = (restArgs).head().cloned()?;
        restArgs = (restArgs).rest()?;
    } else {
        value = inValue;
    }
    evaluateConfigFlag(config_flag, value, inFlags)?;
    Ok(restArgs)
}

fn lookupConfigFlag(mut inFlag: ArcStr, mut inFlagPrefix: &ArcStr) -> Result<Flags::ConfigFlag> {
    let mut outFlag: Flags::ConfigFlag;
    if let Ok(__iflet0) = List::getMemberOnTrue(
        inFlag.clone(),
        &(allConfigFlags.clone()),
        &fnptr!(matchConfigFlag, ArcStr, Flags::ConfigFlag),
    ) {
        outFlag = __iflet0;
    } else {
        Error::addMessage(
            Error::UNKNOWN_OPTION.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inFlagPrefix);
                __mm_s.push_str(&*inFlag);
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(outFlag)
}

fn configFlagEq(mut inFlag1: &Flags::ConfigFlag, mut inFlag2: &Flags::ConfigFlag) -> Result<bool> {
    let mut eq: bool;
    eq = (match (inFlag1.clone(), inFlag2.clone()) {
        (Flags::ConfigFlag { index: mut index1, .. }, Flags::ConfigFlag { index: mut index2, .. }) => {
            index1.clone() == index2.clone()
        }
    });
    Ok(eq)
}

fn flagRequiresValue(mut flag: &Flags::ConfigFlag) -> bool {
    let mut requiresValue: bool;
    requiresValue = (match flag.clone() {
        Flags::ConfigFlag {
            defaultValue: Flags::FlagData::BOOL_FLAG { .. },
            ..
        } => false,
        _ => true,
    });
    requiresValue
}

fn setAdditionalOptModules(
    mut inFlag: Flags::ConfigFlag,
    mut inOppositeFlag: Flags::ConfigFlag,
    mut inValues: &metamodelica::List<ArcStr>,
) -> Result<()> {
    let mut values: metamodelica::List<ArcStr>;
    for mut value in &**inValues {
        values = Flags::getConfigStringList(inOppositeFlag.clone())?;
        values = List::removeOnTrue(value.clone(), &fnptr!(stringEq, ArcStr, ArcStr), values)?;
        setConfigStringList(inOppositeFlag.clone(), values)?;
        values = Flags::getConfigStringList(inFlag.clone())?;
        values = List::removeOnTrue(value.clone(), &fnptr!(stringEq, ArcStr, ArcStr), values)?;
        setConfigStringList(inFlag.clone(), metamodelica::cons(value.clone(), values))?;
    }
    Ok(())
}

fn evaluateConfigFlag(mut inFlag: Flags::ConfigFlag, mut inValue: ArcStr, mut inFlags: &Flags::Flag) -> Result<()> {
    let () = (match (inFlag.clone(), inFlags.clone()) {
        (
            Flags::ConfigFlag { index: 1, .. },
            Flags::Flag::FLAGS {
                debugFlags: mut debug_flags,
                ..
            },
        ) => {
            List::map1_0(&(splitCSV(inValue)), &setDebugFlag, debug_flags.clone())?;
            ()
        }
        (Flags::ConfigFlag { index: 2, .. }, _) => {
            let mut values: metamodelica::List<ArcStr>;
            values = splitCSV(System::tolower(inValue));
            metamodelica::print(printHelp(&values)?);
            setConfigString(Flags::HELP.clone(), literal!("omc"))?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::PRE_OPT_MODULES_ADD.clone()))?) => {
            setAdditionalOptModules(
                Flags::PRE_OPT_MODULES_ADD.clone(),
                Flags::PRE_OPT_MODULES_SUB.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::PRE_OPT_MODULES_SUB.clone()))?) => {
            setAdditionalOptModules(
                Flags::PRE_OPT_MODULES_SUB.clone(),
                Flags::PRE_OPT_MODULES_ADD.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::POST_OPT_MODULES_ADD.clone()))?) => {
            setAdditionalOptModules(
                Flags::POST_OPT_MODULES_ADD.clone(),
                Flags::POST_OPT_MODULES_SUB.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::POST_OPT_MODULES_SUB.clone()))?) => {
            setAdditionalOptModules(
                Flags::POST_OPT_MODULES_SUB.clone(),
                Flags::POST_OPT_MODULES_ADD.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::INIT_OPT_MODULES_ADD.clone()))?) => {
            setAdditionalOptModules(
                Flags::INIT_OPT_MODULES_ADD.clone(),
                Flags::INIT_OPT_MODULES_SUB.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (_, _) if (configFlagEq(&inFlag, &(Flags::INIT_OPT_MODULES_SUB.clone()))?) => {
            setAdditionalOptModules(
                Flags::INIT_OPT_MODULES_SUB.clone(),
                Flags::INIT_OPT_MODULES_ADD.clone(),
                &(splitCSV(inValue)),
            )?;
            ()
        }
        (
            _,
            Flags::Flag::FLAGS {
                configFlags: mut config_flags,
                ..
            },
        ) => {
            setConfigFlag(inFlag.clone(), config_flags.clone(), inValue)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn setDebugFlag(mut inFlag: ArcStr, mut inFlags: metamodelica::Array<bool>) -> Result<()> {
    let mut negated: bool;
    let mut neg1: bool;
    let mut neg2: bool;
    let mut flag_str: ArcStr;
    neg1 = stringEq(&(stringGetStringChar(inFlag.clone(), 1)?), &(literal!("-")));
    neg2 = System::strncmp(literal!("no"), inFlag.clone(), 2) == 0;
    negated = neg1 || neg2;
    flag_str = if (negated) { StringUtil::rest(inFlag)? } else { inFlag };
    flag_str = if (neg2) { StringUtil::rest(flag_str)? } else { flag_str };
    setDebugFlag2(flag_str, !(negated), inFlags.clone())?;
    Ok(())
}

fn setDebugFlag2(mut inFlag: ArcStr, mut inValue: bool, mut inFlags: metamodelica::Array<bool>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inFlags.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut flag: Flags::DebugFlag;
            flag = List::getMemberOnTrue(
                inFlag.clone(),
                &(allDebugFlags.clone()),
                &move |__a0: ArcStr, __a1: Flags::DebugFlag| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(matchDebugFlag(&__a0, __a1))
                },
            )?;
            updateDebugFlagArray(inFlags.clone(), inValue, flag.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addMessage(Error::UNKNOWN_DEBUG_FLAG.clone(), list![inFlag.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn matchDebugFlag(mut inFlagName: &ArcStr, mut inFlag: Flags::DebugFlag) -> bool {
    let mut outMatches: bool;
    let mut name: ArcStr;
    let Flags::DEBUG_FLAG { name: __pa0, .. } = inFlag;
    name = metamodelica::Own::own(__pa0);
    outMatches = stringEq(&inFlagName, &name);
    outMatches
}

fn matchConfigFlag(mut inFlagName: ArcStr, mut inFlag: Flags::ConfigFlag) -> bool {
    let mut outMatches: bool;
    let mut opt_shortname: Option<ArcStr>;
    let mut name: ArcStr;
    let mut shortname: ArcStr;
    let Flags::CONFIG_FLAG {
        name: __pa0,
        shortname: __pa1,
        ..
    } = inFlag;
    name = metamodelica::Own::own(__pa0);
    opt_shortname = metamodelica::Own::own(__pa1);
    shortname = opt_shortname.unwrap_or(literal!(""));
    outMatches =
        stringEq(&inFlagName, &shortname) || stringEq(&(System::tolower(inFlagName)), &(System::tolower(name)));
    outMatches
}

fn setConfigFlag(
    mut inFlag: Flags::ConfigFlag,
    mut inConfigData: metamodelica::Array<Flags::FlagData>,
    mut inValue: ArcStr,
) -> Result<()> {
    let mut data: Flags::FlagData;
    let mut default_value: Flags::FlagData;
    let mut name: ArcStr;
    let mut validOptions: Option<Flags::ValidOptions>;
    let Flags::CONFIG_FLAG {
        name: __pa0,
        defaultValue: __pa1,
        validOptions: __pa2,
        ..
    } = &inFlag;
    name = metamodelica::Own::own(__pa0);
    default_value = metamodelica::Own::own(__pa1);
    validOptions = metamodelica::Own::own(__pa2);
    data = stringFlagData(inValue, &default_value, validOptions, name)?;
    updateConfigFlagArray(inConfigData.clone(), data, inFlag)?;
    Ok(())
}

fn stringFlagData(
    mut inValue: ArcStr,
    mut inExpectedType: &Flags::FlagData,
    mut validOptions: Option<Flags::ValidOptions>,
    mut inName: ArcStr,
) -> Result<Flags::FlagData> {
    let mut outValue: Flags::FlagData;
    outValue = 'mc: {
        let __mc_input = (inValue.clone(), inExpectedType, validOptions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", Flags::FlagData::BOOL_FLAG { .. }, _) => {
                    Ok(Flags::FlagData::BOOL_FLAG { data: true })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::BOOL_FLAG { .. }, _) => {
                    let mut b: bool;
                    b = Util::stringBool(inValue.clone())?;
                    Ok(Flags::FlagData::BOOL_FLAG { data: b })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::INT_FLAG { .. }, _) => {
                    let mut i: i32;
                    i = stringInt(inValue.clone())?;
                    let true = (stringEq(&(intString(i)), &inValue)) else { return Err("pattern mismatch") };
                    Ok(Flags::FlagData::INT_FLAG { data: i })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (_, Flags::FlagData::INT_LIST_FLAG { .. }, _) => {
                            let mut ilst: metamodelica::List<i32>;
                            ilst = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut v in (splitCSV(inValue.clone())).into_iter().cloned() {
                            let __x = stringInt(v.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(Flags::FlagData::INT_LIST_FLAG { data: ilst.clone() })
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::REAL_FLAG { .. }, _) => {
                    Ok(Flags::FlagData::REAL_FLAG { data: stringReal(inValue.clone())? })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_FLAG { .. }, Some(options)) => {
                    let mut flags: metamodelica::List<ArcStr>;
                    flags = getValidStringOptions(metamodelica::AsArg::as_arg(&options))?;
                    let true = (listMember(inValue.clone(), flags.clone())) else { return Err("pattern mismatch") };
                    Ok(Flags::FlagData::STRING_FLAG { data: inValue.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_FLAG { .. }, None) => {
                    if !((!(stringEmpty(&inValue)))) { return Err("guard") }
                    Ok(Flags::FlagData::STRING_FLAG { data: inValue.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::STRING_LIST_FLAG { .. }, _) => {
                    Ok(Flags::FlagData::STRING_LIST_FLAG { data: splitCSV(inValue.clone()) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", Flags::FlagData::ENUM_FLAG { validValues: enums, .. }, _) => {
                    let mut i: i32;
                    i = Util::assoc(literal!("true"), enums.clone())?;
                    Ok(Flags::FlagData::ENUM_FLAG { data: i, validValues: enums.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Flags::FlagData::ENUM_FLAG { validValues: enums, .. }, _) => {
                    let mut i: i32;
                    i = Util::assoc(inValue.clone(), enums.clone())?;
                    Ok(Flags::FlagData::ENUM_FLAG { data: i, validValues: enums.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None) => {
                    let mut et: ArcStr;
                    let mut at: ArcStr;
                    et = printExpectedTypeStr(inExpectedType)?;
                    at = printActualTypeStr(inValue.clone());
                    Error::addMessage(Error::INVALID_FLAG_TYPE.clone(), list![inName.clone(), et.clone(), at.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(options)) => {
                    let mut et: ArcStr;
                    let mut at: ArcStr;
                    let mut flags: metamodelica::List<ArcStr>;
                    flags = getValidStringOptions(metamodelica::AsArg::as_arg(&options))?;
                    et = stringDelimitList(flags.clone(), literal!(", "));
                    at = printActualTypeStr(inValue.clone());
                    Error::addMessage(Error::INVALID_FLAG_TYPE_STRINGS.clone(), list![inName.clone(), et.clone(), at.clone()])?;
                    Ok(return Err("fail"))
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

fn printExpectedTypeStr(mut inType: &Flags::FlagData) -> Result<ArcStr> {
    let mut outTypeStr: ArcStr;
    outTypeStr = (match inType.clone() {
        Flags::FlagData::BOOL_FLAG { .. } => {
            literal!("a boolean value")
        }
        Flags::FlagData::INT_FLAG { .. } => {
            literal!("an integer value")
        }
        Flags::FlagData::REAL_FLAG { .. } => {
            literal!("a floating-point value")
        }
        Flags::FlagData::STRING_FLAG { .. } => {
            literal!("a string")
        }
        Flags::FlagData::STRING_LIST_FLAG { .. } => {
            literal!("a comma-separated list of strings")
        }
        Flags::FlagData::ENUM_FLAG {
            validValues: ref enums, ..
        } => {
            let mut enum_strs: metamodelica::List<ArcStr>;
            enum_strs = List::map(enums.clone(), &fnptr!(Util::tuple21, _))?;
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("one of the values {"));
                __mm_s.push_str(&*stringDelimitList(enum_strs, literal!(", ")));
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outTypeStr)
}

fn printActualTypeStr(mut inType: ArcStr) -> ArcStr {
    let mut outTypeStr: ArcStr;
    outTypeStr = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Ok(literal!("nothing"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Util::stringBool(inType.clone())?;
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the boolean value ")); __mm_s.push_str(&*inType); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i: i32;
                    i = stringInt(inType.clone())?;
                    let true = (stringEq(&(intString(i)), &inType)) else { return Err("pattern mismatch") };
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number ")); __mm_s.push_str(&*intString(i)); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the string \"")); __mm_s.push_str(&*inType); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTypeStr
}

fn configFlagsIsEqualIndex(mut inFlag1: Flags::ConfigFlag, mut inFlag2: Flags::ConfigFlag) -> bool {
    let mut outEqualIndex: bool;
    let mut index1: i32;
    let mut index2: i32;
    let Flags::CONFIG_FLAG { index: __pa0, .. } = inFlag1;
    index1 = metamodelica::Own::own(__pa0);
    let Flags::CONFIG_FLAG { index: __pa1, .. } = inFlag2;
    index2 = metamodelica::Own::own(__pa1);
    outEqualIndex = intEq(index1, index2);
    outEqualIndex
}

fn handleDeprecatedFlags() -> Result<()> {
    let mut remaining_flags: metamodelica::List<ArcStr>;
    if Flags::isSet(Flags::NF_UNITCHECK.clone())? {
        disableDebug(Flags::NF_UNITCHECK.clone())?;
        setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
        Error::addMessage(
            Error::DEPRECATED_FLAG.clone(),
            list![literal!("-d=frontEndUnitCheck"), literal!("--unitChecking")],
        )?;
    }
    if Flags::isSet(Flags::OLD_FE_UNITCHECK.clone())? {
        disableDebug(Flags::OLD_FE_UNITCHECK.clone())?;
        setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
        Error::addMessage(
            Error::DEPRECATED_FLAG.clone(),
            list![literal!("-d=oldFrontEndUnitCheck"), literal!("--unitChecking")],
        )?;
    }
    if Flags::isSet(Flags::INTERACTIVE_TCP.clone())? {
        disableDebug(Flags::INTERACTIVE_TCP.clone())?;
        setConfigString(Flags::INTERACTIVE.clone(), literal!("tcp"))?;
        Error::addMessage(
            Error::DEPRECATED_FLAG.clone(),
            list![literal!("-d=interactive"), literal!("--interactive=tcp")],
        )?;
        metamodelica::print(literal!(
            "The flag -d=interactive is depreciated. Please use --interactive=tcp instead.\n"
        ));
    }
    if metamodelica::stringEq(
        &(Flags::getConfigString(Flags::TEARING_METHOD.clone())?),
        &(literal!("noTearing")),
    ) {
        setConfigString(Flags::TEARING_METHOD.clone(), literal!("minimalTearing"))?;
        Error::addMessage(
            Error::DEPRECATED_FLAG.clone(),
            list![
                literal!("--tearingMethod=noTearing"),
                literal!("--tearingMethod=minimalTearing")
            ],
        )?;
    }
    remaining_flags = metamodelica::nil();
    for mut flag in &*Flags::getConfigStringList(Flags::PRE_OPT_MODULES.clone())? {
        if metamodelica::stringEq(&flag, &(literal!("unitChecking"))) {
            setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
            Error::addMessage(
                Error::DEPRECATED_FLAG.clone(),
                list![literal!("--preOptModules=unitChecking"), literal!("--unitChecking")],
            )?;
        } else {
            remaining_flags = metamodelica::cons(flag.clone(), remaining_flags);
        }
    }
    setConfigStringList(Flags::PRE_OPT_MODULES.clone(), remaining_flags.reverse())?;
    remaining_flags = metamodelica::nil();
    for mut flag in &*Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())? {
        if metamodelica::stringEq(&flag, &(literal!("unitChecking"))) {
            setConfigBool(Flags::UNIT_CHECKING.clone(), true)?;
            Error::addMessage(
                Error::DEPRECATED_FLAG.clone(),
                list![literal!("--preOptModules+=unitChecking"), literal!("--unitChecking")],
            )?;
        } else {
            remaining_flags = metamodelica::cons(flag.clone(), remaining_flags);
        }
    }
    setConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone(), remaining_flags.reverse())?;
    Ok(())
}

fn applySideEffects(mut inFlag: Flags::ConfigFlag, mut inValue: Flags::FlagData) -> () {
    let () = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut value: bool;
            let true = (configFlagsIsEqualIndex(inFlag.clone(), Flags::SHOW_ERROR_MESSAGES.clone())) else {
                return Err("pattern mismatch");
            };
            let Flags::BOOL_FLAG { data: __pa0 } = (inValue.clone()) else {
                return Err("pattern mismatch");
            };
            value = metamodelica::Own::own(__pa0);
            ErrorExt::setShowErrorMessages(value);
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

pub(crate) fn setConfigValue(mut inFlag: Flags::ConfigFlag, mut inValue: Flags::FlagData) -> Result<()> {
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let mut flags: Flags::Flag;
    flags = loadFlags(true)?;
    let Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: __pa1,
    } = (flags)
    else {
        return Err("pattern mismatch");
    };
    debug_flags = metamodelica::Own::own(__pa0);
    config_flags = metamodelica::Own::own(__pa1);
    config_flags = updateConfigFlagArray(config_flags.clone(), inValue, inFlag)?;
    saveFlags(Flags::Flag::FLAGS {
        debugFlags: debug_flags.clone(),
        configFlags: config_flags.clone(),
    });
    Ok(())
}

pub fn setConfigBool(mut inFlag: Flags::ConfigFlag, mut inValue: bool) -> Result<()> {
    setConfigValue(inFlag, Flags::FlagData::BOOL_FLAG { data: inValue })?;
    Ok(())
}

pub fn setConfigInt(mut inFlag: Flags::ConfigFlag, mut inValue: i32) -> Result<()> {
    setConfigValue(inFlag, Flags::FlagData::INT_FLAG { data: inValue })?;
    Ok(())
}

pub(crate) fn setConfigReal(mut inFlag: Flags::ConfigFlag, mut inValue: metamodelica::Real) -> Result<()> {
    setConfigValue(inFlag, Flags::FlagData::REAL_FLAG { data: inValue })?;
    Ok(())
}

pub fn setConfigString(mut inFlag: Flags::ConfigFlag, mut inValue: ArcStr) -> Result<()> {
    setConfigValue(inFlag, Flags::FlagData::STRING_FLAG { data: inValue })?;
    Ok(())
}

pub fn setConfigStringList(mut inFlag: Flags::ConfigFlag, mut inValue: metamodelica::List<ArcStr>) -> Result<()> {
    setConfigValue(inFlag, Flags::FlagData::STRING_LIST_FLAG { data: inValue })?;
    Ok(())
}

pub fn appendConfigStringList(mut flag: Flags::ConfigFlag, mut value: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut oldValues: metamodelica::List<ArcStr>;
    oldValues = Flags::getConfigStringList(flag.clone())?;
    if !(listMember(value.clone(), oldValues.clone())) {
        setConfigStringList(flag, metamodelica::cons(value, oldValues.clone()))?;
    }
    Ok(oldValues)
}

pub fn setConfigEnum(mut inFlag: Flags::ConfigFlag, mut inValue: i32) -> Result<()> {
    let mut valid_values: metamodelica::List<(ArcStr, i32)>;
    let Flags::CONFIG_FLAG {
        defaultValue: Flags::ENUM_FLAG { validValues: __pa0, .. },
        ..
    } = (inFlag.clone())
    else {
        return Err("pattern mismatch");
    };
    valid_values = metamodelica::Own::own(__pa0);
    setConfigValue(
        inFlag,
        Flags::FlagData::ENUM_FLAG {
            data: inValue,
            validValues: valid_values,
        },
    )?;
    Ok(())
}

// Used by the print functions below to indent descriptions.
pub(crate) const descriptionIndent: &'static str = "                            ";

pub fn printHelp(mut inTopics: &metamodelica::List<ArcStr>) -> Result<ArcStr> {
    let mut help: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    help = 'mc: {
        let __mc_input = &**inTopics;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(printUsage()?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "omc", tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(printUsage()?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "omcall-sphinxoutput", tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(printUsageSphinxAll()?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "topics", tail: Deref @ metamodelica::ListNode::Nil } => {
                            let mut topics: metamodelica::List<(ArcStr, ArcStr)>;
                            let mut s: IOStream::IOStream = s.clone();
                            s = IOStream::create(literal!("topics"), crate::IOStream::IOStreamType::LIST)?;
                            s = IOStream::append(s.clone(), literal!("The available topics (help(\"topics\")) are as follows:\n"))?;
                            topics = list![(literal!("omc"), literal!("The command-line options available for omc.")), (literal!("debug"), literal!("Flags that enable debugging, diagnostics, and research prototypes.")), (literal!("optmodules"), literal!("Flags that determine which symbolic methods are used to produce the causalized equation system.")), (literal!("simulation"), literal!("The command-line options available for simulation executables generated by OpenModelica.")), (literal!("<flagname>"), literal!("Displays option descriptions for flag <flagname>.")), (literal!("topics"), literal!("This help-text."))];
                            s = IOStream::append(s.clone(), stringDelimitList(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut t in (topics.clone()).into_iter().cloned() {
                            let __x = makeTopicString(&(t.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), literal!("\n")))?;
                            s = IOStream::append(s.clone(), literal!("\n"))?;
                            Ok((IOStream::string(&s)?, s.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            s = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "simulation", tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The simulation executable takes the following flags:\n\n")); __mm_s.push_str(&*System::getSimulationHelpText(true, false)); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "simulation-sphinxoutput", tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The simulation executable takes the following flags:\n\n")); __mm_s.push_str(&*System::getSimulationHelpText(true, true)); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "debug", tail: Deref @ metamodelica::ListNode::Nil } => {
                            let mut s: IOStream::IOStream = s.clone();
                            s = IOStream::create(literal!("debug"), crate::IOStream::IOStreamType::LIST)?;
                            s = IOStream::append(s.clone(), literal!("The debug flag takes a comma-separated list of flags which are used by the\ncompiler for debugging or experimental purposes.\nFlags prefixed with \"-\" or \"no\" will be disabled.\n"))?;
                            s = IOStream::append(s.clone(), literal!("The available flags are (+ are enabled by default, - are disabled):\n\n"))?;
                            s = IOStream::appendList(s.clone(), &(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut flag in (List::sort(allDebugFlags.clone(), (std::sync::Arc::new(fnptr!(compareDebugFlags, Flags::DebugFlag, Flags::DebugFlag)) as std::sync::Arc<dyn ::std::ops::Fn(Flags::DebugFlag, Flags::DebugFlag) -> Result<bool> + 'static>))?).into_iter().cloned() {
                            let __x = printDebugFlag(flag.clone(), false)?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })))?;
                            Ok((IOStream::string(&s)?, s.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            s = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "optmodules", tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut data: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut s: IOStream::IOStream = s.clone();
                    s = IOStream::create(literal!("optmodules"), crate::IOStream::IOStreamType::LIST)?;
                    s = IOStream::append(s.clone(), wrapToTerminal(literal!("The --preOptModules flag sets the optimization modules which are used before the\nmatching and index reduction in the back end. These modules are specified as a comma-separated list."))?)?;
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa0 }, .. } = (Flags::PRE_OPT_MODULES.clone()) else { return Err("pattern mismatch") };
                    data = metamodelica::Own::own(__pa0);
                    s = IOStream::append(s.clone(), literal!("\n\nThe modules used by default are:\n--preOptModules="))?;
                    s = IOStream::append(s.clone(), stringDelimitList(data.clone(), literal!(",")))?;
                    s = IOStream::append(s.clone(), literal!("\n\nThe valid modules are:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&(Flags::PRE_OPT_MODULES.clone()))?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    s = IOStream::append(s.clone(), wrapToTerminal(literal!("\nThe --matchingAlgorithm sets the method that is used for the matching algorithm, after the pre optimization modules."))?)?;
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_FLAG { data: __pa1 }, .. } = (Flags::MATCHING_ALGORITHM.clone()) else { return Err("pattern mismatch") };
                    r#str = metamodelica::Own::own(__pa1);
                    s = IOStream::append(s.clone(), literal!("\n\nThe method used by default is:\n--matchingAlgorithm="))?;
                    s = IOStream::append(s.clone(), r#str.clone())?;
                    s = IOStream::append(s.clone(), literal!("\n\nThe valid methods are:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&(Flags::MATCHING_ALGORITHM.clone()))?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    s = IOStream::append(s.clone(), wrapToTerminal(literal!("The --indexReductionMethod sets the method that is used for the index reduction, after the pre optimization modules."))?)?;
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_FLAG { data: __pa2 }, .. } = (Flags::INDEX_REDUCTION_METHOD.clone()) else { return Err("pattern mismatch") };
                    r#str = metamodelica::Own::own(__pa2);
                    s = IOStream::append(s.clone(), literal!("\n\nThe method used by default is:\n--indexReductionMethod="))?;
                    s = IOStream::append(s.clone(), r#str.clone())?;
                    s = IOStream::append(s.clone(), literal!("\n\nThe valid methods are:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&(Flags::INDEX_REDUCTION_METHOD.clone()))?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    s = IOStream::append(s.clone(), wrapToTerminal(literal!("The --initOptModules then sets the optimization modules which are used after the index reduction to optimize the system for initialization, specified as a comma-separated list."))?)?;
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa3 }, .. } = (Flags::INIT_OPT_MODULES.clone()) else { return Err("pattern mismatch") };
                    data = metamodelica::Own::own(__pa3);
                    s = IOStream::append(s.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nThe modules used by default are:\n--initOptModules=")); __mm_s.push_str(&*stringDelimitList(data.clone(), literal!(","))); ArcStr::from(__mm_s) })?;
                    s = IOStream::append(s.clone(), literal!("\n\nThe valid modules are:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&(Flags::INIT_OPT_MODULES.clone()))?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    s = IOStream::append(s.clone(), wrapToTerminal(literal!("The --postOptModules then sets the optimization modules which are used after the index reduction to optimize the system for simulation, specified as a comma-separated list."))?)?;
                    let Flags::CONFIG_FLAG { defaultValue: Flags::STRING_LIST_FLAG { data: __pa4 }, .. } = (Flags::POST_OPT_MODULES.clone()) else { return Err("pattern mismatch") };
                    data = metamodelica::Own::own(__pa4);
                    s = IOStream::append(s.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nThe modules used by default are:\n--postOptModules=")); __mm_s.push_str(&*stringDelimitList(data.clone(), literal!(","))); ArcStr::from(__mm_s) })?;
                    s = IOStream::append(s.clone(), literal!("\n\nThe valid modules are:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&(Flags::POST_OPT_MODULES.clone()))?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    Ok((IOStream::string(&s)?, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            s = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut desc: ArcStr;
                    let mut name: ArcStr;
                    let mut config_flag: Flags::ConfigFlag;
                    let mut short_name: Option<ArcStr>;
                    let mut s: IOStream::IOStream = s.clone();
                    s = IOStream::create(literal!("flag"), crate::IOStream::IOStreamType::LIST)?;
                    let ref __pa3 @ Flags::CONFIG_FLAG { name: ref __pa0, shortname: ref __pa1, description: ref __pa2, .. } = List::getMemberOnTrue(r#str.clone(), &(allConfigFlags.clone()), &fnptr!(matchConfigFlag, ArcStr, Flags::ConfigFlag))?;
                    name = metamodelica::Own::own(__pa0);
                    short_name = metamodelica::Own::own(__pa1);
                    desc = metamodelica::Own::own(__pa2);
                    config_flag = metamodelica::Own::own(__pa3);
                    if (short_name).is_some() {
                        s = IOStream::append(s.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*short_name.clone().ok_or("pattern mismatch")?); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) })?;
                    }
                    s = IOStream::append(s.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    s = IOStream::append(s.clone(), wrapToTerminal(desc.clone())?)?;
                    s = IOStream::append(s.clone(), literal!("\n\n"))?;
                    s = IOStream::append(s.clone(), literal!("Valid arguments:\n"))?;
                    s = IOStream::append(s.clone(), printFlagValidOptionsDesc(&config_flag)?)?;
                    s = IOStream::append(s.clone(), literal!("\n"))?;
                    Ok((IOStream::string(&s)?, s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            s = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("I'm sorry, I don't know what ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" is.\n")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#str, tail: rest_topics @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
                    let mut r#str = (*r#str).clone();
                    let mut help: ArcStr = help.clone();
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*printHelp(&(list![r#str.clone()]))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    help = printHelp(metamodelica::AsArg::as_arg(&rest_topics))?;
                    Ok(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*help); ArcStr::from(__mm_s) }, help.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            help = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(help)
}

pub fn getValidOptionsAndDescription(
    mut flagName: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, ArcStr, metamodelica::List<ArcStr>)> {
    let mut validStrings: metamodelica::List<ArcStr>;
    let mut mainDescriptionStr: ArcStr;
    let mut descriptions: metamodelica::List<ArcStr>;
    let mut validOptions: Flags::ValidOptions;
    let mut mainDescription: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::getMemberOnTrue(flagName, &(allConfigFlags.clone()), &fnptr!(matchConfigFlag, ArcStr, Flags::ConfigFlag))?) {
        Flags::ConfigFlag { description: __pa0, validOptions: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    mainDescription = metamodelica::Own::own(__pa0);
    validOptions = metamodelica::Own::own(__pa1);
    mainDescriptionStr = mainDescription;
    (validStrings, descriptions) = getValidOptionsAndDescription2(&validOptions)?;
    Ok((validStrings, mainDescriptionStr, descriptions))
}

fn getValidOptionsAndDescription2(
    mut validOptions: &Flags::ValidOptions,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut validStrings: metamodelica::List<ArcStr>;
    let mut descriptions: metamodelica::List<ArcStr>;
    (validStrings, descriptions) = (match validOptions.clone() {
        Flags::ValidOptions::STRING_OPTION {
            options: ref __esc_validStrings,
        } => {
            validStrings = __esc_validStrings.clone();
            (validStrings.clone(), metamodelica::nil())
        }
        Flags::ValidOptions::STRING_DESC_OPTION { options: mut options } => {
            validStrings = List::map(options.clone(), &fnptr!(Util::tuple21, _))?;
            descriptions = List::map(options.clone(), &fnptr!(Util::tuple22, _))?;
            (validStrings, descriptions)
        }
    });
    Ok((validStrings, descriptions))
}

fn compareDebugFlags(mut flag1: Flags::DebugFlag, mut flag2: Flags::DebugFlag) -> bool {
    let mut b: bool;
    let mut name1: ArcStr;
    let mut name2: ArcStr;
    let Flags::DEBUG_FLAG { name: __pa0, .. } = flag1;
    name1 = metamodelica::Own::own(__pa0);
    let Flags::DEBUG_FLAG { name: __pa1, .. } = flag2;
    name2 = metamodelica::Own::own(__pa1);
    b = stringCompare(&name1, &name2) > 0;
    b
}

fn makeTopicString(mut topic: &(ArcStr, ArcStr)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    (str1, str2) = topic.clone();
    str1 = Util::stringPadRight(str1, 13, literal!(" "));
    r#str = stringAppendList(StringUtil::wordWrap(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*str1);
            __mm_s.push_str(&*str2);
            ArcStr::from(__mm_s)
        },
        System::getTerminalWidth(),
        literal!("\n               "),
        metamodelica::OrderedFloat(0.3_f64),
    )?);
    Ok(r#str)
}

pub fn printUsage() -> Result<ArcStr> {
    let mut usage: ArcStr;
    Print::clearBuf();
    Print::printBuf(literal!("OpenModelica Compiler "))?;
    Print::printBuf(Settings::getVersionNr())?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(literal!("Copyright © 2019 Open Source Modelica Consortium (OSMC)\n"))?;
    Print::printBuf(literal!(
        "Distributed under OSMC-PL and AGPL3, see www.openmodelica.org\n\n"
    ))?;
    Print::printBuf(literal!(
        "Usage: omc [Options] (Model.mo | Script.mos) [Libraries | .mo-files]\n* Libraries: Fully qualified names of libraries to load before processing Model or Script.\n             The libraries should be separated by spaces: Lib1 Lib2 ... LibN.\n"
    ))?;
    Print::printBuf(literal!("\n* Options:\n"))?;
    Print::printBuf(printAllConfigFlags()?)?;
    Print::printBuf(literal!(
        "\nFor more details on a specific topic, use --help=topics or help(\"topics\")\n\n"
    ))?;
    Print::printBuf(literal!("* Examples:\n"))?;
    Print::printBuf(literal!(
        "  omc Model.mo             will produce flattened Model on standard output.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc -s Model.mo          will produce simulation code for the model:\n"
    ))?;
    Print::printBuf(literal!(
        "                            * Model.c           The model C code.\n"
    ))?;
    Print::printBuf(literal!(
        "                            * Model_functions.c The model functions C code.\n"
    ))?;
    Print::printBuf(literal!(
        "                            * Model.makefile    The makefile to compile the model.\n"
    ))?;
    Print::printBuf(literal!(
        "                            * Model_init.xml    The initial values.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc Script.mos           will run the commands from Script.mos.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc Model.mo Modelica    will first load the Modelica library and then produce\n                            flattened Model on standard output.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc Model1.mo Model2.mo  will load both Model1.mo and Model2.mo, and produce\n                            flattened Model1 on standard output.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc --export-fmu -i MyPackage.Examples.Hello --fmiVersion=2.0\n                           ./MyPackage/package.mo\n                           will load the local package and export the model as an FMU.\n"
    ))?;
    Print::printBuf(literal!(
        "  omc --export-fmu -i MyModel --fmiVersion=2.0 MyModel.mo Modelica\n                           will load MyModel.mo and the Modelica Standard Library,\n                           then export MyModel as an FMU.\n"
    ))?;
    Print::printBuf(literal!("  *.mo (Modelica files)\n"))?;
    Print::printBuf(literal!("  *.mos (Modelica Script files)\n\n"))?;
    Print::printBuf(literal!("For available simulation flags, use --help=simulation.\n\n"))?;
    Print::printBuf(literal!(
        "Documentation is available in the built-in package OpenModelica.Scripting or\nonline <https://build.openmodelica.org/Documentation/OpenModelica.Scripting.html>.\n"
    ))?;
    usage = Print::getString()?;
    Print::clearBuf();
    Ok(usage)
}

pub(crate) fn printUsageSphinxAll() -> Result<ArcStr> {
    let mut usage: ArcStr;
    let mut s: ArcStr;
    Print::clearBuf();
    s = literal!("OpenModelica Compiler Flags");
    Print::printBuf(literal!("\n.. _openmodelica-compiler-flags :\n\n"))?;
    Print::printBuf(s.clone())?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(
        ({
            let mut __acc = String::new();
            for mut e in (1..=((s).len() as i32)).into_iter() {
                let __x = literal!("=");
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone(),
    )?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(literal!(
        "Usage: omc [Options] (Model.mo | Script.mos) [Libraries | .mo-files]\n\n* Libraries: Fully qualified names of libraries to load before processing Model or Script.\n  The libraries should be separated by spaces: Lib1 Lib2 ... LibN.\n\n"
    ))?;
    Print::printBuf(literal!("\n.. _omcflags-options :\n\n"))?;
    s = literal!("Options");
    Print::printBuf(s.clone())?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(
        ({
            let mut __acc = String::new();
            for mut e in (1..=((s).len() as i32)).into_iter() {
                let __x = literal!("-");
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone(),
    )?;
    Print::printBuf(literal!("\n\n"))?;
    for mut flag in &*allConfigFlags.clone() {
        Print::printBuf(printConfigFlagSphinx(metamodelica::AsArg::as_arg(&flag))?)?;
    }
    Print::printBuf(literal!("\n.. _omcflag-debug-section:\n\n"))?;
    s = literal!("Debug flags");
    Print::printBuf(s.clone())?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(
        ({
            let mut __acc = String::new();
            for mut e in (1..=((s).len() as i32)).into_iter() {
                let __x = literal!("-");
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone(),
    )?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "The debug flag takes a comma-separated list of flags which are used by the\ncompiler for debugging or experimental purposes.\nFlags prefixed with \"-\" or \"no\" will be disabled.\n"
    ))?;
    Print::printBuf(literal!(
        "The available flags are (+ are enabled by default, - are disabled):\n\n"
    ))?;
    for mut flag in &*List::sort(
        allDebugFlags.clone(),
        (std::sync::Arc::new(fnptr!(compareDebugFlags, Flags::DebugFlag, Flags::DebugFlag))
            as std::sync::Arc<dyn ::std::ops::Fn(Flags::DebugFlag, Flags::DebugFlag) -> Result<bool> + 'static>),
    )? {
        Print::printBuf(printDebugFlag(flag.clone(), true)?)?;
    }
    Print::printBuf(literal!("\n.. _omcflag-optmodules-section:\n\n"))?;
    s = literal!("Flags for Optimization Modules");
    Print::printBuf(s.clone())?;
    Print::printBuf(literal!("\n"))?;
    Print::printBuf(
        ({
            let mut __acc = String::new();
            for mut e in (1..=((s).len() as i32)).into_iter() {
                let __x = literal!("-");
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone(),
    )?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "Flags that determine which symbolic methods are used to produce the causalized equation system.\n\n"
    ))?;
    Print::printBuf(literal!(
        "The :ref:`--preOptModules <omcflag-preOptModules>` flag sets the optimization modules which are used before the\nmatching and index reduction in the back end. These modules are specified as a comma-separated list."
    ))?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "The :ref:`--matchingAlgorithm <omcflag-matchingAlgorithm>` sets the method that is used for the matching algorithm, after the pre optimization modules."
    ))?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "The :ref:`--indexReductionMethod <omcflag-indexReductionMethod>` sets the method that is used for the index reduction, after the pre optimization modules."
    ))?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "The :ref:`--initOptModules <omcflag-initOptModules>` then sets the optimization modules which are used after the index reduction to optimize the system for initialization, specified as a comma-separated list."
    ))?;
    Print::printBuf(literal!("\n\n"))?;
    Print::printBuf(literal!(
        "The :ref:`--postOptModules <omcflag-postOptModules>` then sets the optimization modules which are used after the index reduction to optimize the system for simulation, specified as a comma-separated list."
    ))?;
    Print::printBuf(literal!("\n\n"))?;
    usage = Print::getString()?;
    Print::clearBuf();
    Ok(usage)
}

pub(crate) fn printAllConfigFlags() -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringAppendList(List::map(allConfigFlags.clone(), &move |__a0: Flags::ConfigFlag| {
        printConfigFlag(&__a0)
    })?);
    Ok(outString)
}

fn printConfigFlag(mut inFlag: &Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inFlag.clone() {
        Flags::ConfigFlag {
            visibility: Flags::FlagVisibility::INTERNAL { .. },
            ..
        } => {
            literal!("")
        }
        Flags::ConfigFlag {
            description: mut desc, ..
        } => {
            let mut name: ArcStr;
            let mut desc_str: ArcStr;
            let mut flag_str: ArcStr;
            let mut delim_str: ArcStr;
            let mut opt_str: ArcStr;
            let mut wrapped_str: metamodelica::List<ArcStr>;
            desc_str = desc.clone();
            name = Util::stringPadRight((printConfigFlagName(inFlag, false)?).0, 28, literal!(" "));
            flag_str = stringAppendList(list![name, literal!(" "), desc_str]);
            delim_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                __mm_s.push_str(&*literal!("  "));
                ArcStr::from(__mm_s)
            };
            wrapped_str = StringUtil::wordWrap(
                flag_str,
                System::getTerminalWidth(),
                delim_str,
                metamodelica::OrderedFloat(0.3_f64),
            )?;
            opt_str = printValidOptions(inFlag)?;
            flag_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(wrapped_str, literal!("\n")));
                __mm_s.push_str(&*opt_str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            flag_str
        }
    });
    Ok(outString)
}

fn printConfigFlagSphinx(mut inFlag: &Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inFlag.clone() {
        Flags::ConfigFlag {
            visibility: Flags::FlagVisibility::INTERNAL { .. },
            ..
        } => {
            literal!("")
        }
        Flags::ConfigFlag {
            description: mut desc, ..
        } => {
            let mut name: ArcStr;
            let mut longName: ArcStr;
            let mut desc_str: ArcStr;
            let mut flag_str: ArcStr;
            let mut opt_str: ArcStr;
            desc_str = desc.clone();
            desc_str = System::stringReplace(
                desc_str,
                literal!("--help=debug"),
                literal!(":ref:`--help=debug <omcflag-debug-section>`"),
            )?;
            desc_str = System::stringReplace(
                desc_str,
                literal!("--help=optmodules"),
                literal!(":ref:`--help=optmodules <omcflag-optmodules-section>`"),
            )?;
            (name, longName) = printConfigFlagName(inFlag, true)?;
            opt_str = printValidOptionsSphinx(inFlag)?;
            flag_str = stringAppendList(list![
                literal!(".. _omcflag-"),
                longName.clone(),
                literal!(":\n\n:ref:`"),
                name,
                literal!("<omcflag-"),
                longName,
                literal!(">`\n\n"),
                desc_str,
                literal!("\n"),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*opt_str);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                }
            ]);
            flag_str
        }
    });
    Ok(outString)
}

fn printConfigFlagName(mut inFlag: &Flags::ConfigFlag, mut sphinx: bool) -> Result<(ArcStr, ArcStr)> {
    let mut outString: ArcStr;
    let mut longName: ArcStr;
    (outString, longName) = (match inFlag.clone() {
        Flags::ConfigFlag {
            name: mut name,
            shortname: Some(mut shortname),
            ..
        } => {
            shortname = if (sphinx) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-"));
                    __mm_s.push_str(&*shortname);
                    ArcStr::from(__mm_s)
                }
            } else {
                Util::stringPadLeft(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("-"));
                        __mm_s.push_str(&*shortname);
                        ArcStr::from(__mm_s)
                    },
                    4,
                    literal!(" "),
                )
            };
            (
                stringAppendList(list![shortname.clone(), literal!(", --"), name.clone()]),
                name.clone(),
            )
        }
        Flags::ConfigFlag {
            name: mut name,
            shortname: None,
            ..
        } => (
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*if (sphinx) { literal!("--") } else { literal!("      --") });
                __mm_s.push_str(&*name);
                ArcStr::from(__mm_s)
            },
            name.clone(),
        ),
        _ => return Err("match: no arm matched"),
    });
    Ok((outString, longName))
}

fn printValidOptions(mut inFlag: &Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inFlag.clone() {
        Flags::ConfigFlag { validOptions: None, .. } => {
            literal!("")
        }
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref strl }),
            ..
        } => {
            let mut opt_str: ArcStr;
            let mut strl = strl.clone();
            opt_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                __mm_s.push_str(&*literal!("   "));
                __mm_s.push_str(&*literal!("Valid options:"));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*stringDelimitList(strl.clone(), literal!(", ")));
                ArcStr::from(__mm_s)
            };
            strl = StringUtil::wordWrap(
                opt_str,
                System::getTerminalWidth(),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                    __mm_s.push_str(&*literal!("     "));
                    ArcStr::from(__mm_s)
                },
                metamodelica::OrderedFloat(0.3_f64),
            )?;
            opt_str = stringDelimitList(strl.clone(), literal!("\n"));
            opt_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*opt_str);
                ArcStr::from(__mm_s)
            };
            opt_str
        }
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: ref descl }),
            ..
        } => {
            let mut opt_str: ArcStr;
            opt_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                __mm_s.push_str(&*literal!("   "));
                __mm_s.push_str(&*literal!("Valid options:"));
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*stringAppendList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut d in (descl.clone()).into_iter().cloned() {
                            let __x = printFlagOptionDescShort(&(d.clone()), false);
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                ));
                ArcStr::from(__mm_s)
            };
            opt_str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn printValidOptionsSphinx(mut inFlag: &Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inFlag.clone() {
        Flags::ConfigFlag { validOptions: None, .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone()));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_OPTION { options: ref strl }),
            ..
        } => {
            let mut opt_str: ArcStr;
            opt_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone()));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*literal!("Valid options"));
                __mm_s.push_str(&*literal!(":\n\n"));
                __mm_s.push_str(
                    &*({
                        let mut __acc = String::new();
                        for mut s in (strl.clone()).into_iter().cloned() {
                            let __x = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("* "));
                                __mm_s.push_str(&*s);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            };
                            __acc.push_str(&__x);
                        }
                        ArcStr::from(__acc)
                    }),
                );
                ArcStr::from(__mm_s)
            };
            opt_str
        }
        Flags::ConfigFlag {
            validOptions: Some(Flags::ValidOptions::STRING_DESC_OPTION { options: ref descl }),
            ..
        } => {
            let mut opt_str: ArcStr;
            opt_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*defaultFlagSphinx(inFlag.defaultValue.clone()));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*literal!("Valid options"));
                __mm_s.push_str(&*literal!(":\n\n"));
                __mm_s.push_str(
                    &*({
                        let mut __acc = String::new();
                        for mut s in (descl.clone()).into_iter().cloned() {
                            let __x = printFlagOptionDesc(&(s.clone()), true)?;
                            __acc.push_str(&__x);
                        }
                        ArcStr::from(__acc)
                    }),
                );
                ArcStr::from(__mm_s)
            };
            opt_str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn defaultFlagSphinx(mut flag: Flags::FlagData) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(flag.clone()) {
        Flags::FlagData::BOOL_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Boolean (default")); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*boolString(var_field!(flag.data, Flags::FlagData::BOOL_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::INT_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Integer (default")); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*intString(var_field!(flag.data, Flags::FlagData::INT_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::REAL_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Real (default")); __mm_s.push_str(&*literal!(" ``")); __mm_s.push_str(&*realString(var_field!(flag.data, Flags::FlagData::REAL_FLAG).clone())); __mm_s.push_str(&*literal!("``).")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::STRING_FLAG { data: Deref @ "" } => {
            literal!("String (default *empty*).")
        },
        Flags::FlagData::STRING_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("String (default")); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*var_field!(flag.data, Flags::FlagData::STRING_FLAG)); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::STRING_LIST_FLAG { data: Deref @ metamodelica::ListNode::Nil } => {
            literal!("String list (default *empty*).")
        },
        Flags::FlagData::STRING_LIST_FLAG { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("String list (default")); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(var_field!(flag.data, Flags::FlagData::STRING_LIST_FLAG).clone(), literal!(","))); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }
        },
        Flags::FlagData::ENUM_FLAG { .. } => {
            let mut i: i32;
            for mut f in &*var_field!(flag.validValues, Flags::FlagData::ENUM_FLAG).clone() {
                (r#str, i) = f.clone();
                if i == var_field!(flag.data, Flags::FlagData::ENUM_FLAG).clone() {
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("String (default ")); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) };
                    return r#str;
                }
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#ENUM_FLAG Failed#")); __mm_s.push_str(&*anyString(flag)); ArcStr::from(__mm_s) }
        },
        _ => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown default value")); __mm_s.push_str(&*anyString(flag)); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#str
}

fn printFlagOptionDescShort(mut inOption: &(ArcStr, ArcStr), mut sphinx: bool) -> ArcStr {
    let mut outString: ArcStr;
    let mut name: ArcStr;
    (name, _) = inOption.clone();
    outString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*if (sphinx) {
            literal!("* ")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                __mm_s.push_str(&*literal!("    * "));
                ArcStr::from(__mm_s)
            }
        });
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    outString
}

fn printFlagValidOptionsDesc(mut inFlag: &Flags::ConfigFlag) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut desc_options: metamodelica::List<(ArcStr, ArcStr)>;
    let mut str_options: metamodelica::List<ArcStr>;
    let mut enum_options: metamodelica::List<(ArcStr, i32)>;
    outString = (match inFlag.clone() {
        Flags::ConfigFlag {
            validOptions:
                Some(Flags::ValidOptions::STRING_DESC_OPTION {
                    options: ref __esc_desc_options,
                }),
            ..
        } => {
            desc_options = __esc_desc_options.clone();
            stringAppendList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut o in (desc_options.clone()).into_iter().cloned() {
                        let __x = printFlagOptionDesc(&(o.clone()), false)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )
        }
        Flags::ConfigFlag {
            validOptions:
                Some(Flags::ValidOptions::STRING_OPTION {
                    options: ref __esc_str_options,
                }),
            ..
        } => {
            str_options = __esc_str_options.clone();
            stringDelimitList(str_options.clone(), literal!(", "))
        }
        Flags::ConfigFlag {
            defaultValue:
                Flags::FlagData::ENUM_FLAG {
                    validValues: ref __esc_enum_options,
                    ..
                },
            ..
        } => {
            enum_options = __esc_enum_options.clone();
            stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut v in (enum_options.clone()).into_iter().cloned() {
                        let __x = Util::tuple21(v.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            )
        }
        _ => {
            (match inFlag.defaultValue.clone() {
                Flags::FlagData::BOOL_FLAG { .. } => literal!("false, true"),
                Flags::FlagData::INT_FLAG { .. } => literal!("An Integer value."),
                Flags::FlagData::INT_LIST_FLAG { .. } => literal!("A comma-separated list of Integer values."),
                Flags::FlagData::REAL_FLAG { .. } => literal!("A Real value."),
                Flags::FlagData::STRING_FLAG { .. } => literal!("A String value"),
                Flags::FlagData::STRING_LIST_FLAG { .. } => literal!("A comma-separated list of String values."),
                _ => literal!("Unknown"),
            })
        }
    });
    Ok(outString)
}

fn sphinxMathMode(mut s: ArcStr) -> Result<ArcStr> {
    let mut o: ArcStr = s;
    let mut i: i32;
    let mut strs: metamodelica::List<ArcStr>;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    (i, strs) = System::regex(o.clone(), literal!("^(.*)[$]([^$]*)[$](.*)$"), 4, true, false);
    if i == 4 {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(strs) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: _ } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        s1 = metamodelica::Own::own(__pa0);
        s2 = metamodelica::Own::own(__pa1);
        s3 = metamodelica::Own::own(__pa2);
        o = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*s1);
            __mm_s.push_str(&*literal!(" :math:`"));
            __mm_s.push_str(&*s2);
            __mm_s.push_str(&*literal!("` "));
            __mm_s.push_str(&*s3);
            ArcStr::from(__mm_s)
        };
    }
    Ok(o)
}

fn removeSphinxMathMode(mut s: ArcStr) -> Result<ArcStr> {
    let mut o: ArcStr = s;
    let mut i: i32;
    let mut strs: metamodelica::List<ArcStr>;
    (i, strs) = System::regex(o.clone(), literal!("^(.*):math:`([^`]*)[`](.*)$"), 4, true, false);
    if i == 4 {
        o = removeSphinxMathMode(stringAppendList((strs).rest()?))?;
    }
    Ok(o)
}

fn printFlagOptionDesc(mut inOption: &(ArcStr, ArcStr), mut sphinx: bool) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut desc: ArcStr;
    let mut name: ArcStr;
    let mut desc_str: ArcStr;
    let mut r#str: ArcStr;
    (name, desc) = inOption.clone();
    desc_str = desc;
    if sphinx {
        desc_str = ({
            let mut __acc = String::new();
            for mut s in (System::strtok(desc_str, literal!("\n"))).into_iter().cloned() {
                let __x = System::trim(s.clone(), literal!(" \u{c}\n\r\t\u{b}"));
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone();
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("* "));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*desc_str);
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        };
    } else {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Util::stringPadRight(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" * "));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(" "));
                    ArcStr::from(__mm_s)
                },
                30,
                literal!(" "),
            ));
            __mm_s.push_str(&*removeSphinxMathMode(desc_str)?);
            ArcStr::from(__mm_s)
        };
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                StringUtil::wordWrap(
                    r#str,
                    System::getTerminalWidth(),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*arcstr::literal!(descriptionIndent));
                        __mm_s.push_str(&*literal!("    "));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::OrderedFloat(0.3_f64),
                )?,
                literal!("\n"),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

fn printDebugFlag(mut inFlag: Flags::DebugFlag, mut sphinx: bool) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut desc: ArcStr;
    let mut name: ArcStr;
    let mut desc_str: ArcStr;
    let mut default: bool;
    let Flags::DEBUG_FLAG {
        default: __pa0,
        name: __pa1,
        description: __pa2,
        ..
    } = inFlag;
    default = metamodelica::Own::own(__pa0);
    name = metamodelica::Own::own(__pa1);
    desc = metamodelica::Own::own(__pa2);
    desc_str = desc;
    if sphinx {
        desc_str = stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut s in (System::strtok(desc_str, literal!("\n"))).into_iter().cloned() {
                    let __x = System::trim(s.clone(), literal!(" \u{c}\n\r\t\u{b}"));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!("\n  "),
        );
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n.. _omcflag-debug-"));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(":\n\n"));
            __mm_s.push_str(&*literal!(":ref:`"));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" <omcflag-debug-"));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(">`"));
            __mm_s.push_str(&*literal!(" (default: "));
            __mm_s.push_str(&*if (default) { literal!("on") } else { literal!("off") });
            __mm_s.push_str(&*literal!(")\n  "));
            __mm_s.push_str(&*desc_str);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    } else {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Util::stringPadRight(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*if (default) { literal!(" + ") } else { literal!(" - ") });
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(" "));
                    ArcStr::from(__mm_s)
                },
                26,
                literal!(" "),
            ));
            __mm_s.push_str(&*removeSphinxMathMode(desc_str)?);
            ArcStr::from(__mm_s)
        };
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*stringDelimitList(
                StringUtil::wordWrap(
                    outString,
                    System::getTerminalWidth(),
                    arcstr::literal!(descriptionIndent),
                    metamodelica::OrderedFloat(0.3_f64),
                )?,
                literal!("\n"),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(outString)
}

pub fn debugFlagName(mut inFlag: Flags::DebugFlag) -> ArcStr {
    let mut name: ArcStr;
    let Flags::DEBUG_FLAG { name: __pa0, .. } = inFlag;
    name = metamodelica::Own::own(__pa0);
    name
}

pub fn configFlagName(mut inFlag: Flags::ConfigFlag) -> ArcStr {
    let mut name: ArcStr;
    let Flags::CONFIG_FLAG { name: __pa0, .. } = inFlag;
    name = metamodelica::Own::own(__pa0);
    name
}

fn getValidStringOptions(mut inOptions: &Flags::ValidOptions) -> Result<metamodelica::List<ArcStr>> {
    let mut validOptions: metamodelica::List<ArcStr>;
    validOptions = (match inOptions.clone() {
        Flags::ValidOptions::STRING_OPTION {
            options: ref __esc_validOptions,
        } => {
            validOptions = __esc_validOptions.clone();
            validOptions.clone()
        }
        Flags::ValidOptions::STRING_DESC_OPTION { options: mut options } => {
            List::map(options.clone(), &fnptr!(Util::tuple21, _))?
        }
    });
    Ok(validOptions)
}

pub(crate) fn flagDataEq(mut data1: &Flags::FlagData, mut data2: &Flags::FlagData) -> Result<bool> {
    let mut eq: bool;
    eq = (match (data1.clone(), data2.clone()) {
        (Flags::FlagData::EMPTY_FLAG { .. }, Flags::FlagData::EMPTY_FLAG { .. }) => true,
        (Flags::FlagData::BOOL_FLAG { .. }, Flags::FlagData::BOOL_FLAG { .. }) => {
            var_field!(data1.data, Flags::FlagData::BOOL_FLAG).clone()
                == var_field!(data2.data, Flags::FlagData::BOOL_FLAG).clone()
        }
        (Flags::FlagData::INT_FLAG { .. }, Flags::FlagData::INT_FLAG { .. }) => {
            var_field!(data1.data, Flags::FlagData::INT_FLAG).clone()
                == var_field!(data2.data, Flags::FlagData::INT_FLAG).clone()
        }
        (Flags::FlagData::INT_LIST_FLAG { .. }, Flags::FlagData::INT_LIST_FLAG { .. }) => List::isEqualOnTrue(
            var_field!(data1.data, Flags::FlagData::INT_LIST_FLAG).clone(),
            var_field!(data2.data, Flags::FlagData::INT_LIST_FLAG).clone(),
            &fnptr!(intEq, i32, i32),
        )?,
        (Flags::FlagData::REAL_FLAG { .. }, Flags::FlagData::REAL_FLAG { .. }) => {
            var_field!(data1.data, Flags::FlagData::REAL_FLAG).clone()
                == var_field!(data2.data, Flags::FlagData::REAL_FLAG).clone()
        }
        (Flags::FlagData::STRING_FLAG { .. }, Flags::FlagData::STRING_FLAG { .. }) => metamodelica::stringEq(
            &var_field!(data1.data, Flags::FlagData::STRING_FLAG),
            &var_field!(data2.data, Flags::FlagData::STRING_FLAG),
        ),
        (Flags::FlagData::STRING_LIST_FLAG { .. }, Flags::FlagData::STRING_LIST_FLAG { .. }) => List::isEqualOnTrue(
            var_field!(data1.data, Flags::FlagData::STRING_LIST_FLAG).clone(),
            var_field!(data2.data, Flags::FlagData::STRING_LIST_FLAG).clone(),
            &fnptr!(stringEq, ArcStr, ArcStr),
        )?,
        (Flags::FlagData::ENUM_FLAG { .. }, Flags::FlagData::ENUM_FLAG { .. }) => {
            metamodelica::ReferenceEq::reference_eq(
                &(var_field!(data1.validValues, Flags::FlagData::ENUM_FLAG).clone()),
                &(var_field!(data2.validValues, Flags::FlagData::ENUM_FLAG).clone()),
            ) && var_field!(data1.data, Flags::FlagData::ENUM_FLAG).clone()
                == var_field!(data2.data, Flags::FlagData::ENUM_FLAG).clone()
        }
        _ => false,
    });
    Ok(eq)
}

pub(crate) fn flagDataString(mut flagData: &Flags::FlagData) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match flagData.clone() {
        Flags::FlagData::BOOL_FLAG { .. } => boolString(var_field!(flagData.data, Flags::FlagData::BOOL_FLAG).clone()),
        Flags::FlagData::INT_FLAG { .. } => intString(var_field!(flagData.data, Flags::FlagData::INT_FLAG).clone()),
        Flags::FlagData::INT_LIST_FLAG { .. } => List::toStringCustom(
            var_field!(flagData.data, Flags::FlagData::INT_LIST_FLAG).clone(),
            &fnptr!(intString, i32),
            literal!(""),
            literal!(""),
            literal!(","),
            literal!(""),
            false,
            0,
        )?,
        Flags::FlagData::REAL_FLAG { .. } => realString(var_field!(flagData.data, Flags::FlagData::REAL_FLAG).clone()),
        Flags::FlagData::STRING_FLAG { .. } => var_field!(flagData.data, Flags::FlagData::STRING_FLAG).clone(),
        Flags::FlagData::STRING_LIST_FLAG { .. } => stringDelimitList(
            var_field!(flagData.data, Flags::FlagData::STRING_LIST_FLAG).clone(),
            literal!(","),
        ),
        Flags::FlagData::ENUM_FLAG { .. } => {
            let mut v: i32;
            for mut vt in &*var_field!(flagData.validValues, Flags::FlagData::ENUM_FLAG).clone() {
                (r#str, v) = vt.clone();
                if v == var_field!(flagData.data, Flags::FlagData::ENUM_FLAG).clone() {
                    return Ok(r#str);
                }
            }
            literal!("")
        }
        _ => {
            literal!("")
        }
    });
    Ok(r#str)
}

pub fn unparseFlags() -> Result<metamodelica::List<ArcStr>> {
    let mut flagStrings: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut debug_flags: metamodelica::Array<bool>;
    let mut config_flags: metamodelica::Array<Flags::FlagData>;
    let mut name: ArcStr;
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fvalue: bool;
    if let Ok(Flags::FLAGS {
        debugFlags: __pa0,
        configFlags: __pa1,
    }) = loadFlags(false)
    {
        debug_flags = metamodelica::Own::own(__pa0);
        config_flags = metamodelica::Own::own(__pa1);
    } else {
        return Ok(flagStrings);
    }
    for mut f in &*allConfigFlags.clone() {
        if !(flagDataEq(
            &f.defaultValue,
            &({
                let __elt = (*metamodelica::index_checked(&config_flags.borrow(), f.index.clone())?).clone();
                __elt
            }),
        )?) {
            name = (match f.shortname.clone() {
                Some(mut __esc_name) => {
                    name = __esc_name.clone();
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("-"));
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    }
                }
                _ => {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("--"));
                    __mm_s.push_str(&*f.name);
                    ArcStr::from(__mm_s)
                }
            });
            flagStrings = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!("="));
                    __mm_s.push_str(&*flagDataString(
                        &({
                            let __elt =
                                (*metamodelica::index_checked(&config_flags.borrow(), f.index.clone())?).clone();
                            __elt
                        }),
                    )?);
                    ArcStr::from(__mm_s)
                },
                flagStrings,
            );
        }
    }
    for mut f in &*allDebugFlags.clone() {
        fvalue = ({
            let __elt = (*metamodelica::index_checked(&debug_flags.borrow(), f.index.clone())?).clone();
            __elt
        });
        if f.default.clone() != fvalue {
            name = if (fvalue) {
                f.name.clone()
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("no"));
                    __mm_s.push_str(&*f.name);
                    ArcStr::from(__mm_s)
                }
            };
            strl = metamodelica::cons(name, strl);
        }
    }
    if !((strl).is_empty()) {
        flagStrings = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-d="));
                __mm_s.push_str(&*stringDelimitList(strl, literal!(",")));
                ArcStr::from(__mm_s)
            },
            flagStrings,
        );
    }
    Ok(flagStrings)
}

pub(crate) fn splitCSV(mut value: ArcStr) -> metamodelica::List<ArcStr> {
    let mut outValues: metamodelica::List<ArcStr> = System::strtok(value.clone(), literal!(","));
    outValues
}

pub(crate) fn wrapToTerminal(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr = stringAppendList(StringUtil::wordWrap(
        r#str.clone(),
        System::getTerminalWidth(),
        literal!("\n"),
        metamodelica::OrderedFloat(0.3_f64),
    )?);
    Ok(outStr)
}

pub fn applyNumProcEnvironment() -> Result<()> {
    if Flags::getConfigInt(Flags::NUM_PROC.clone())? == 1 {
        System::setEnv(literal!("OPENBLAS_NUM_THREADS"), literal!("1"), false);
        System::setEnv(literal!("OMP_NUM_THREADS"), literal!("1"), false);
    }
    Ok(())
}
