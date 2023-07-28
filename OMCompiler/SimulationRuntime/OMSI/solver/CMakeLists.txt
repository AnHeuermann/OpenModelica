CMAKE_MINIMUM_REQUIRED(VERSION 3.14)
#OpenModelica Simulation Interface solver
PROJECT(OMSISolver)
SET(CMAKE_VERBOSE_MAKEFILE ON)

############################
# Find BLAS and LAPACK
############################
if(MSVC)
  #workaround  because CMake does not find the lapack libraries for Visual Studio 10
  SET(LAPACK_MSVC_10  $ENV{OMDEV}/lib/3rdParty/Lapack/Lib/lapack_win32.lib )
  SET(BLAS_MSVC_10 $ENV{OMDEV}/lib/3rdParty/Lapack/Lib/blas_win32.lib )
  SET(LAPACK_LIBRARIES  ${LAPACK_MSVC_10} ${BLAS_MSVC_10})
else(MSVC)
  find_package(LAPACK REQUIRED)
endif(MSVC)
if (NOT LAPACK_LIBRARIES)
  MESSAGE(FATAL_ERROR "Error: Lapack Libraries not found!")
else()
  MESSAGE(STATUS "Lapack Libraries: ${LAPACK_LIBRARIES}")
endif()

####################################
# OMSISolver library
####################################
add_library(OMSISolver)
add_library(omc::simrt::omsi::solver ALIAS OMSISolver)

target_sources(OMSISolver PRIVATE src/solver_api.c
                                  src/solver_helper.c
                                  src/solver_kinsol.c
                                  src/solver_lapack.c)

set_property(TARGET OMSISolver PROPERTY C_STANDARD 99) # snprintf is C99

set(CMAKE_MACOSX_RPATH 1)   # Handle rpath on mac in makefiles

target_link_libraries(OMSISolver PUBLIC omc::3rd::sundials::kinsol)
target_link_libraries(OMSISolver PUBLIC omc::3rd::sundials::sunlinsollapackdense)
target_link_libraries(OMSISolver PUBLIC ${LAPACK_LIBRARIES})
target_link_libraries(OMSISolver PRIVATE ${CMAKE_DL_LIBS})

target_include_directories(OMSISolver PUBLIC include)
# TODO AHeu: DESTINATION include/omc/omsi/solver

install(TARGETS OMSISolver DESTINATION ${LIBINSTALLEXT})
