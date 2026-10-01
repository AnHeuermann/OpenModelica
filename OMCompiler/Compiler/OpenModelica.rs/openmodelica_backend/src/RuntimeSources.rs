// Auto-generated from MetaModelica source
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

pub const fmu_sources_dir: &'static str = "/share/omc/sources/c";

// The Rust crates a --simCodeTarget=C source FMU carries, in the checkout's
// own layout: the manifests reach openmodelica_lapack by a relative path.
// Installed by SimulationRuntime/rust/CMakeLists.txt.
pub const fmu_rust_sources_dir: &'static str = "/share/omc/sources/rust";

pub const fmu_rust_manifest: &'static str = "SimulationRuntime/rust/Cargo.toml";

pub static simrt_c_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("gc/omc_alloc.c"),
        literal!("gc/omc_rc.c"),
        literal!("gc/omc_gc.c"),
        literal!("util/base_array.c"),
        literal!("util/boolean_array.c"),
        literal!("util/context.c"),
        literal!("util/division.c"),
        literal!("util/doubleEndedList.c"),
        literal!("util/generic_array.c"),
        literal!("util/index_spec.c"),
        literal!("util/integer_array.c"),
        literal!("util/list.c"),
        literal!("util/modelica_string_lit.c"),
        literal!("util/modelica_string.c"),
        literal!("util/ModelicaUtilities.c"),
        literal!("util/omc_error.c"),
        literal!("util/omc_file.c"),
        literal!("util/omc_init.c"),
        literal!("util/omc_mmap.c"),
        literal!("util/omc_msvc.c"),
        literal!("util/omc_numbers.c"),
        literal!("util/omc_box.c"),
        literal!("util/omc_str_utils.c"),
        literal!("util/omc_string.c"),
        literal!("util/rational.c"),
        literal!("util/real_array.c"),
        literal!("util/ringbuffer.c"),
        literal!("util/simulation_options.c"),
        literal!("util/string_array.c"),
        literal!("util/utility.c"),
        literal!("util/varinfo.c"),
        literal!("meta/meta_modelica_catch.c"),
        literal!("math-support/pivot.c"),
        literal!("simulation/arrayIndex.c"),
        literal!("simulation/eval_dep.c"),
        literal!("simulation/jacobian_util.c"),
        literal!("simulation/omc_simulation_util.c"),
        literal!("simulation/options.c"),
        literal!("simulation/simulation_info_json.c"),
        literal!("simulation/simulation_omc_assert.c"),
        literal!("simulation/solver/delay.c"),
        literal!("simulation/solver/discrete_changes.c"),
        literal!("simulation/solver/model_help.c"),
        literal!("simulation/solver/omc_math.c"),
        literal!("simulation/solver/spatialDistribution.c"),
        literal!("simulation/solver/stateset.c"),
        literal!("simulation/solver/synchronous.c"),
        literal!("simulation/solver/initialization/initialization.c")
    ]
});

// The libOpenModelicaRuntimeC half of simrt_c_sources: what a
// --simCodeTarget=C FMU still compiles from C, because the Rust runtime
// replaces only what libSimulationRuntimeC covers.
pub static simrt_c_runtime_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("gc/omc_alloc.c"),
        literal!("gc/omc_rc.c"),
        literal!("gc/omc_gc.c"),
        literal!("util/base_array.c"),
        literal!("util/boolean_array.c"),
        literal!("util/context.c"),
        literal!("util/division.c"),
        literal!("util/doubleEndedList.c"),
        literal!("util/generic_array.c"),
        literal!("util/index_spec.c"),
        literal!("util/integer_array.c"),
        literal!("util/list.c"),
        literal!("util/modelica_string_lit.c"),
        literal!("util/modelica_string.c"),
        literal!("util/ModelicaUtilities.c"),
        literal!("util/omc_error.c"),
        literal!("util/omc_file.c"),
        literal!("util/omc_init.c"),
        literal!("util/omc_mmap.c"),
        literal!("util/omc_msvc.c"),
        literal!("util/omc_numbers.c"),
        literal!("util/omc_box.c"),
        literal!("util/omc_str_utils.c"),
        literal!("util/omc_string.c"),
        literal!("util/rational.c"),
        literal!("util/real_array.c"),
        literal!("util/ringbuffer.c"),
        literal!("util/simulation_options.c"),
        literal!("util/string_array.c"),
        literal!("util/utility.c"),
        literal!("util/varinfo.c"),
        literal!("meta/meta_modelica_catch.c")
    ]
});

pub static simrt_c_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("omc_dll.h"),
        literal!("omc_inline.h"),
        literal!("openmodelica_func.h"),
        literal!("openmodelica.h"),
        literal!("omc_simulation_settings.h"),
        literal!("openmodelica_types.h"),
        literal!("simulation_data.h"),
        literal!("ModelicaUtilities.h"),
        literal!("linearization/linearize.h"),
        literal!("simulation/arrayIndex.h"),
        literal!("simulation/eval_dep.h"),
        literal!("simulation/jacobian_colpack.h"),
        literal!("simulation/jacobian_util.h"),
        literal!("simulation/modelinfo.h"),
        literal!("simulation/options.h"),
        literal!("simulation/simulation_info_json.h"),
        literal!("simulation/simulation_input_xml.h"),
        literal!("simulation/simulation_omc_assert.h"),
        literal!("simulation/simulation_runtime.h"),
        literal!("simulation/omc_simulation_util.h"),
        literal!("simulation/results/simulation_result.h"),
        literal!("simulation/solver/cvode_solver.h"),
        literal!("simulation/solver/dae_mode.h"),
        literal!("simulation/solver/dassl.h"),
        literal!("simulation/solver/delay.h"),
        literal!("simulation/solver/embedded_server.h"),
        literal!("simulation/solver/epsilon.h"),
        literal!("simulation/solver/events.h"),
        literal!("simulation/solver/external_input.h"),
        literal!("simulation/solver/discrete_changes.h"),
        literal!("simulation/solver/ida_solver.h"),
        literal!("simulation/solver/linearSolverLapack.h"),
        literal!("simulation/solver/linearSolverTotalPivot.h"),
        literal!("simulation/solver/linearSystem.h"),
        literal!("simulation/solver/mixedSearchSolver.h"),
        literal!("simulation/solver/mixedSystem.h"),
        literal!("simulation/solver/model_help.h"),
        literal!("simulation/solver/nonlinearSolverHomotopy.h"),
        literal!("simulation/solver/nonlinearSolverHybrd.h"),
        literal!("simulation/solver/nonlinearSystem.h"),
        literal!("simulation/solver/nonlinearValuesList.h"),
        literal!("simulation/solver/omc_math.h"),
        literal!("simulation/solver/perform_qss_simulation.c.inc"),
        literal!("simulation/solver/perform_simulation.c.inc"),
        literal!("simulation/solver/real_time_sync.h"),
        literal!("simulation/solver/solver_main.h"),
        literal!("simulation/solver/spatialDistribution.h"),
        literal!("simulation/solver/stateset.h"),
        literal!("simulation/solver/sundials_error.h"),
        literal!("simulation/solver/sundials_util.h"),
        literal!("simulation/solver/synchronous.h"),
        literal!("simulation/solver/initialization/initialization.h"),
        literal!("meta/meta_modelica_builtin_boxptr.h"),
        literal!("meta/meta_modelica_builtin_boxvar.h"),
        literal!("meta/meta_modelica_builtin.h"),
        literal!("meta/meta_modelica.h"),
        literal!("meta/meta_modelica_data.h"),
        literal!("meta/meta_modelica_mk_box.h"),
        literal!("meta/meta_modelica_segv.h"),
        literal!("gc/omc_alloc.h"),
        literal!("gc/omc_gc.h"),
        literal!("gc/omc_rc.h"),
        literal!("util/base_array.h"),
        literal!("util/boolean_array.h"),
        literal!("util/context.h"),
        literal!("util/division.h"),
        literal!("util/generic_array.h"),
        literal!("util/index_spec.h"),
        literal!("util/integer_array.h"),
        literal!("meta/java_interface.h"),
        literal!("util/ModelicaUtilitiesExtra.h"),
        literal!("util/modelica.h"),
        literal!("util/modelica_string.h"),
        literal!("util/omc_error.h"),
        literal!("util/omc_file.h"),
        literal!("util/omc_mmap.h"),
        literal!("util/omc_msvc.h"),
        literal!("util/omc_numbers.h"),
        literal!("util/omc_box.h"),
        literal!("util/omc_stackoverflow.h"),
        literal!("util/omc_str_utils.h"),
        literal!("util/omc_string.h"),
        literal!("util/omc_spinlock.h"),
        literal!("util/omc_strdup.h"),
        literal!("util/read_matlab4.h"),
        literal!("util/read_csv.h"),
        literal!("util/libcsv.h"),
        literal!("util/read_write.h"),
        literal!("util/real_array.h"),
        literal!("util/ringbuffer.h"),
        literal!("util/rtclock.h"),
        literal!("util/simulation_options.h"),
        literal!("util/string_array.h"),
        literal!("util/uthash.h"),
        literal!("util/utility.h"),
        literal!("util/varinfo.h"),
        literal!("util/list.h"),
        literal!("util/doubleEndedList.h"),
        literal!("util/rational.h"),
        literal!("util/modelica_string_lit.h"),
        literal!("util/omc_init.h"),
        literal!("dataReconciliation/dataReconciliation.h")
    ]
});

pub static fmi1Files: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fmi-export/fmu1_model_interface.c.inc"),
        literal!("fmi-export/fmu1_model_interface.h")
    ]
});

pub static fmi1_rust_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fmi-export/fmu1_model_interface.h"),
        literal!("fmi-export/fmu1_rust_interface.c.inc")
    ]
});

pub static fmi2_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fmi-export/fmu2_model_interface.h"),
        literal!("fmi-export/fmu2_rust_interface.c.inc"),
        literal!("fmi-export/fmu_read_flags.h")
    ]
});

pub static fmi2_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fmi-export/fmu2_model_interface.c"),
        literal!("fmi-export/fmu_read_flags.c")
    ]
});

// FMI 3.0 export reuses the FMI 2.0 ModelInstance (fmu2_model_interface.h) and
// the generated per-base-type get/set helpers, so the FMI 2.0 header is also
// required when building an FMI 3.0 FMU.
pub static fmi3_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fmi-export/fmu3_model_interface.h"),
        literal!("fmi-export/fmu3_rust_interface.c.inc")
    ]
});

pub static fmi3_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| list![literal!("fmi-export/fmu3_model_interface.c")]);

pub static defaultFileSuffixes: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!(".c"),
        literal!("_functions.c"),
        literal!("_records.c"),
        literal!("_01exo.c"),
        literal!("_02nls.c"),
        literal!("_03lsy.c"),
        literal!("_04set.c"),
        literal!("_05evt.c"),
        literal!("_06inz.c"),
        literal!("_07dly.c"),
        literal!("_08bnd.c"),
        literal!("_09alg.c"),
        literal!("_10asr.c"),
        literal!("_11mix.c"),
        literal!("_12jac.c"),
        literal!("_13opt.c"),
        literal!("_14lnz.c"),
        literal!("_15syn.c"),
        literal!("_16dae.c"),
        literal!("_17inl.c"),
        literal!("_18spd.c"),
        literal!("_init_fmu.c"),
        literal!("_FMU.c")
    ]
});

pub static sundials_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("sundials/cvode/cvode.h"),
        literal!("sundials/cvode/cvode_ls.h"),
        literal!("sundials/cvode/cvode_proj.h"),
        literal!("sundials/nvector/nvector_serial.h"),
        literal!("sundials/sundials/priv/sundials_context_impl.h"),
        literal!("sundials/sundials/priv/sundials_errors_impl.h"),
        literal!("sundials/sundials/sundials_adaptcontroller.h"),
        literal!("sundials/sundials/sundials_adjointcheckpointscheme.h"),
        literal!("sundials/sundials/sundials_adjointstepper.h"),
        literal!("sundials/sundials/sundials_config.h"),
        literal!("sundials/sundials/sundials_context.h"),
        literal!("sundials/sundials/sundials_core.h"),
        literal!("sundials/sundials/sundials_dense.h"),
        literal!("sundials/sundials/sundials_direct.h"),
        literal!("sundials/sundials/sundials_domeigestimator.h"),
        literal!("sundials/sundials/sundials_errors.h"),
        literal!("sundials/sundials/sundials_export.h"),
        literal!("sundials/sundials/sundials_iterative.h"),
        literal!("sundials/sundials/sundials_linearsolver.h"),
        literal!("sundials/sundials/sundials_logger.h"),
        literal!("sundials/sundials/sundials_math.h"),
        literal!("sundials/sundials/sundials_matrix.h"),
        literal!("sundials/sundials/sundials_memory.h"),
        literal!("sundials/sundials/sundials_nonlinearsolver.h"),
        literal!("sundials/sundials/sundials_nvector.h"),
        literal!("sundials/sundials/sundials_profiler.h"),
        literal!("sundials/sundials/sundials_stepper.h"),
        literal!("sundials/sundials/sundials_types.h"),
        literal!("sundials/sundials/sundials_version.h"),
        literal!("sundials/sunlinsol/sunlinsol_dense.h"),
        literal!("sundials/sunmatrix/sunmatrix_dense.h"),
        literal!("sundials/sunnonlinsol/sunnonlinsol_fixedpoint.h")
    ]
});

pub static simrt_c_sundials_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("simulation/solver/cvode_solver.c"),
        literal!("simulation/solver/sundials_error.c")
    ]
});

pub static modelica_external_c_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("ModelicaExternalC/ModelicaStandardTables.c"),
            literal!("ModelicaExternalC/ModelicaMatIO.c"),
            literal!("ModelicaExternalC/ModelicaIO.c"),
            literal!("ModelicaExternalC/ModelicaStandardTablesDummyUsertab.c"),
            literal!("ModelicaExternalC/snprintf.c")
        ]
    });

pub static modelica_external_c_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("ModelicaExternalC/ModelicaStandardTables.h"),
            literal!("ModelicaExternalC/ModelicaMatIO.h"),
            literal!("ModelicaExternalC/ModelicaIO.h"),
            literal!("ModelicaExternalC/safe-math.h"),
            literal!("ModelicaExternalC/read_data_impl.h")
        ]
    });

pub static dgesv_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("./external_solvers/blaswrap.h"),
        literal!("./external_solvers/clapack.h"),
        literal!("./external_solvers/f2c.h")
    ]
});

pub static dgesv_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("external_solvers/dgemm.c"),
        literal!("external_solvers/dgemv.c"),
        literal!("external_solvers/dger.c"),
        literal!("external_solvers/dscal.c"),
        literal!("external_solvers/dswap.c"),
        literal!("external_solvers/dtrmm.c"),
        literal!("external_solvers/dtrmv.c"),
        literal!("external_solvers/dtrsm.c"),
        literal!("external_solvers/idamax.c"),
        literal!("external_solvers/lsame.c"),
        literal!("external_solvers/dgesv.c"),
        literal!("external_solvers/dgetf2.c"),
        literal!("external_solvers/dgetrf.c"),
        literal!("external_solvers/dgetri.c"),
        literal!("external_solvers/dgetrs.c"),
        literal!("external_solvers/dlamch.c"),
        literal!("external_solvers/dlaswp.c"),
        literal!("external_solvers/dtrti2.c"),
        literal!("external_solvers/dtrtri.c"),
        literal!("external_solvers/ieeeck.c"),
        literal!("external_solvers/ilaenv.c"),
        literal!("external_solvers/iparmq.c"),
        literal!("external_solvers/xerbla.c"),
        literal!("external_solvers/F77_aloc.c"),
        literal!("external_solvers/exit_.c"),
        literal!("external_solvers/i_nint.c"),
        literal!("external_solvers/pow_di.c"),
        literal!("external_solvers/s_cat.c"),
        literal!("external_solvers/s_cmp.c"),
        literal!("external_solvers/s_copy.c")
    ]
});

pub static cminpack_headers: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("./external_solvers/cminpack.h"),
        literal!("./external_solvers/minpack.h"),
        literal!("./external_solvers/minpackP.h")
    ]
});

pub static cminpack_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("external_solvers/enorm_.c"),
        literal!("external_solvers/hybrj_.c"),
        literal!("external_solvers/dpmpar_.c"),
        literal!("external_solvers/qrfac_.c"),
        literal!("external_solvers/qform_.c"),
        literal!("external_solvers/dogleg_.c"),
        literal!("external_solvers/r1updt_.c"),
        literal!("external_solvers/r1mpyq_.c")
    ]
});

pub static simrt_linear_solver_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("simulation/solver/linearSolverLapack.c"),
            literal!("simulation/solver/linearSolverTotalPivot.c"),
            literal!("simulation/solver/linearSystem.c")
        ]
    });

pub static simrt_non_linear_solver_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("simulation/solver/nonlinearSolverHybrd.c"),
            literal!("simulation/solver/nonlinearSystem.c"),
            literal!("simulation/solver/nonlinearValuesList.c"),
            literal!("simulation/solver/nonlinearSolverHomotopy.c")
        ]
    });

pub static simrt_mixed_solver_sources: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("simulation/solver/mixedSystem.c"),
            literal!("simulation/solver/mixedSearchSolver.c")
        ]
    });
