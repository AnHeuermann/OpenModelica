CMAKE_MINIMUM_REQUIRED(VERSION 3.14)
SET(CMAKE_VERBOSE_MAKEFILE ON)
MESSAGE(STATUS "CMake version ${CMAKE_MAJOR_VERSION}.${CMAKE_MINOR_VERSION}.${CMAKE_PATCH_VERSION}")

PROJECT(OMSICSimulationRuntime)

# enable warnings and use ANSI C compatible compiler
if(CMAKE_COMPILER_IS_GNUCXX)
    message(STATUS "GCC detected, adding compile flags")
    set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -Wall -Wextra -ansi -pedantic -g")
    IF(NOT WIN32)
        SET(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fPIC")
    ENDIF()
endif(CMAKE_COMPILER_IS_GNUCXX)
message(STATUS "Compiling with C flags: ${CMAKE_C_FLAGS}")

if(NOT PLATFORM OR PLATFORM STREQUAL "dynamic")
  set(BUILD_SHARED_LIBS ON)
elseif(PLATFORM STREQUAL "static")
  set(BUILD_SHARED_LIBS OFF)
else()
endif()

IF(BUILD_SHARED_LIBS)
  SET(LIBSUFFIX "")
ELSE(BUILD_SHARED_LIBS)
  SET(LIBSUFFIX "_static")
ENDIF(BUILD_SHARED_LIBS)

IF(MSVC)
  MESSAGE(STATUS "MSVC")
  IF(LIB_OMC)
    SET(LIBINSTALLEXT "${LIB_OMC}/omsi/msvc" CACHE STRING "library directory" FORCE)
  ELSE(LIB_OMC)
    SET(LIBINSTALLEXT "omsi/msvc" CACHE STRING "library directory" FORCE)
  ENDIF(LIB_OMC)
ELSE(MSVC)
  IF(LIB_OMC)
    SET(LIBINSTALLEXT "${LIB_OMC}/omsi" CACHE STRING "library directory" FORCE)
  ELSE(LIB_OMC)
    SET(LIBINSTALLEXT "omsi" CACHE STRING "library directory" FORCE)
  ENDIF(LIB_OMC)
ENDIF(MSVC)
message(STATUS "Libs will be installed in ${CMAKE_INSTALL_PREFIX}/${LIBINSTALLEXT}")

SET(OMSI_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../OMSI)

#target_include_directories(OMSICSimulationRuntime PUBLIC include include/omsu)

# omsic simulation runtime
add_library(OMSIC)
add_library(omc::simrt::omsic ALIAS OMSIC)

target_sources(OMSIC PRIVATE
  src/omsu/omsu_helper.c
  src/omsu/omsu_initialization.c
  src/omsu/omsu_getters_and_setters.c
  src/omsu/omsu_event_simulation.c
  src/omsu/omsu_continuous_simulation.c
  src/fmi2/omsi_fmi2_wrapper.c
)

target_link_libraries(OMSIC PUBLIC omc::simrt::omsi)
target_link_libraries(OMSIC PRIVATE ${CMAKE_DL_LIBS})

target_include_directories(OMSIC PUBLIC include include/omsu)

install(TARGETS OMSIC DESTINATION ${LIBINSTALLEXT})

# TODO AHeu: Do this install differently (better)
install(FILES
  ${CMAKE_SOURCE_DIR}/include/omsic.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_common.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_helper.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_initialization.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_getters_and_setters.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_continuous_simulation.h
  ${CMAKE_SOURCE_DIR}/include/omsu/omsu_event_simulation.h
DESTINATION include/omc/omsic)

# uninstall target
if(NOT TARGET uninstall)
    configure_file(
        "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
        IMMEDIATE @ONLY)

    add_custom_target(uninstall
        COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()
