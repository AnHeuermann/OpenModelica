# libhomotopy_capi.a, the homotopy solver of the submodule
# simulation/solver/nls_solver/homotopy_nls_solver, which replaces
# nonlinearSolverHomotopy.c through solver/nonlinearSolverHomotopyRust.c.
# Defines the IMPORTED target omc::homotopy_capi.

find_program(CARGO_EXECUTABLE cargo)
if(NOT CARGO_EXECUTABLE)
  message(FATAL_ERROR "cargo was not found; the homotopy solver needs a stable Rust toolchain.")
endif()

set(_hom_workspace ${CMAKE_CURRENT_LIST_DIR}/../simulation/solver/nls_solver/homotopy_nls_solver)
if(NOT EXISTS ${_hom_workspace}/Cargo.toml)
  message(FATAL_ERROR "${_hom_workspace} is empty, run `git submodule update --init`.")
endif()
set(_hom_target_dir ${CMAKE_BINARY_DIR}/homotopy-nls-solver-target)
set(_hom_lib ${_hom_target_dir}/release/${CMAKE_STATIC_LIBRARY_PREFIX}homotopy_capi${CMAKE_STATIC_LIBRARY_SUFFIX})

file(GLOB_RECURSE _hom_srcs CONFIGURE_DEPENDS
     ${_hom_workspace}/Cargo.toml ${_hom_workspace}/Cargo.lock
     ${_hom_workspace}/*/Cargo.toml ${_hom_workspace}/*/src/*.rs
     ${_hom_workspace}/homotopy_capi/build.rs ${_hom_workspace}/homotopy_capi/cbindgen.toml)
add_custom_command(
  OUTPUT ${_hom_lib}
  WORKING_DIRECTORY ${_hom_workspace}
  COMMAND ${CARGO_EXECUTABLE} build --release --target-dir ${_hom_target_dir} -p homotopy_capi
  DEPENDS ${_hom_srcs}
  COMMENT "Rust: building libhomotopy_capi"
  VERBATIM)
add_custom_target(rust_homotopy_capi DEPENDS ${_hom_lib})

# The system libraries are the ones `cargo rustc -p homotopy_capi --lib
# --crate-type staticlib -- --print native-static-libs` reports.
find_package(Threads REQUIRED)
add_library(omc::homotopy_capi STATIC IMPORTED GLOBAL)
set_target_properties(omc::homotopy_capi PROPERTIES
  IMPORTED_LOCATION ${_hom_lib}
  INTERFACE_INCLUDE_DIRECTORIES ${_hom_workspace}/homotopy_capi/include
  INTERFACE_LINK_LIBRARIES "Threads::Threads;${CMAKE_DL_LIBS};m")
add_dependencies(omc::homotopy_capi rust_homotopy_capi)
