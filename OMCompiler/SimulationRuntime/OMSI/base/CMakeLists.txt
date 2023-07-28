CMAKE_MINIMUM_REQUIRED(VERSION 3.14)
#OMSI base simulation runtime
PROJECT(OMSIBase)
SET(CMAKE_VERBOSE_MAKEFILE ON)

add_library(OMSIBase)
add_library(omc::simrt::omsi::base ALIAS OMSIBase)

target_sources(OMSIBase PRIVATE
  src/omsi_event_helper.c
  src/omsi_getters_and_setters.c
  src/omsi_initialization.c
  src/omsi_input_json.c
  src/omsi_input_model_variables.c
  src/omsi_input_sim_data.c
  src/omsi_input_xml.c
  src/omsi_mmap.c
  src/omsi_posix_func.c
  src/omsi_solve_alg_system.c
  src/omsi_utils.c)

target_include_directories(OMSIBase PUBLIC include ../include)

# TODO AHeu: Ask Mahder about mac magic
# Set RPATH for mac to use lib from build tree and install location
SET(CMAKE_SKIP_BUILD_RPATH  FALSE)
SET(CMAKE_BUILD_WITH_INSTALL_RPATH FALSE)
SET(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")
SET(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)
LIST(FIND CMAKE_PLATFORM_IMPLICIT_LINK_DIRECTORIES "${CMAKE_INSTALL_PREFIX}/lib" isSystemDir)
IF("${isSystemDir}" STREQUAL "-1")
   SET(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")
ENDIF("${isSystemDir}" STREQUAL "-1")

target_link_libraries(OMSIBase PUBLIC omc::3rd::FMIL::expat)
target_link_libraries(OMSIBase PUBLIC omc::simrt::omsi::solver)
target_link_libraries(OMSIBase PRIVATE ${CMAKE_DL_LIBS})
IF(WIN32)
  target_link_libraries(OMSIBase PRIVATE wsock32 ws2_32 )
ENDIF(WIN32)

install(TARGETS OMSIBase DESTINATION ${LIBINSTALLEXT})

# TODO AHeu: Install include DESTINATION
#DESTINATION include/omc/omsi/base
