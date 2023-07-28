cmake_minimum_required(VERSION 3.14)
set(CMAKE_VERBOSE_MAKEFILE ON)
message(STATUS "CMake version ${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION}.${CMAKE_PATCH_VERSION}")

project(OMSIBaseSimulationRuntime)

# enable warnings and use ANSI C compatible compiler
if(CMAKE_COMPILER_IS_GNUCXX)
    message(STATUS "GCC detected, adding compile flags")
    set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -Wall -Wextra -ansi -pedantic -g")
    if(NOT WIN32)
        set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fPIC")
    endif()
endif(CMAKE_COMPILER_IS_GNUCXX)
message(STATUS "Compiling with C flags: ${CMAKE_C_FLAGS}")

if(NOT PLATFORM OR PLATFORM STREQUAL "dynamic")
  set(BUILD_SHARED_LIBS ON)
elseif(PLATFORM STREQUAL "static")
  set(BUILD_SHARED_LIBS OFF)
else()
endif()

if(BUILD_SHARED_LIBS)
  set(LIBSUFFIX "")
else(BUILD_SHARED_LIBS)
  set(LIBSUFFIX "_static")
endif(BUILD_SHARED_LIBS)

if(MSVC)
  message(STATUS "MSVC")
  if(LIB_OMC)
    set(LIBINSTALLEXT "${LIB_OMC}/omsi/msvc" CACHE STRING "library directory" FORCE)
  else(LIB_OMC)
    set(LIBINSTALLEXT "omsi/msvc" CACHE STRING "library directory" FORCE)
  endif(LIB_OMC)
else(MSVC)
  if(LIB_OMC)
    set(LIBINSTALLEXT "${LIB_OMC}/omsi" CACHE STRING "library directory" FORCE)
  else(LIB_OMC)
    set(LIBINSTALLEXT "omsi" CACHE STRING "library directory" FORCE)
  endif(LIB_OMC)
endif(MSVC)
message(STATUS "Libs will be installed in ${CMAKE_INSTALL_PREFIX}/${LIBINSTALLEXT}")

set(OSUBaseName ${LIBPREFIX}OMSIBase${LIBSUFFIX})
set(OMSISolverName ${LIBPREFIX}OMSISolver${LIBSUFFIX})

include_directories ("${CMAKE_SOURCE_DIR}/include")
# omsi base simulation runtime
add_subdirectory(solver)
add_subdirectory(base)

#add_library(OMSI INTERFACE)
#add_library(omc::simrt::omsi ALIAS OMSI)

#target_link_libraries(OMSI omc::simrt::omsi::base INTERFACE)
#target_link_libraries(OMSI omc::simrt::omsi::solver INTERFACE)
#
#target_include_directories(OMSI PUBLIC include)
#target_include_directories(OMSI PUBLIC include/fmi2)
#target_include_directories(OMSI PRIVATE ${OMCTRUNCHOME}/OMCompiler/SimulationRuntime/fmi/export/fmi)


# TODO AHeu: Do this with some prefix instead of specifying DESTINATION?
install(FILES
  ${CMAKE_SOURCE_DIR}/include/omsi.h
  ${CMAKE_SOURCE_DIR}/include/omsi_callbacks.h
  ${CMAKE_SOURCE_DIR}/include/omsi_api_functions.h
  DESTINATION include/omc/omsi)

install(FILES
  ${CMAKE_SOURCE_DIR}/include/fmi2/omsi_fmi2_cs.h
  ${CMAKE_SOURCE_DIR}/include/fmi2/omsi_fmi2_me.h
  ${CMAKE_SOURCE_DIR}/include/fmi2/omsi_fmi2_wrapper.h
  ${CMAKE_SOURCE_DIR}/include/fmi2/fmi2Functions.h
  ${CMAKE_SOURCE_DIR}/include/fmi2/fmi2FunctionTypes.h
  ${CMAKE_SOURCE_DIR}/include/fmi2/fmi2TypesPlatform.h
  DESTINATION include/omc/omsi/fmi2/)

# uninstall target
if(NOT TARGET uninstall)
    configure_file(
        "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
        IMMEDIATE @ONLY)

    add_custom_target(uninstall
        COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()
