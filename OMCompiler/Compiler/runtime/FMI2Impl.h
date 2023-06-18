/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S ACCEPTANCE
 * OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from OSMC, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or
 * http://www.openmodelica.org, and in the OpenModelica distribution.
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */

#ifndef __FMI2_IMPL_H
#define __FMI2_IMPL_H

#ifdef __cplusplus
  extern "C" {
#endif

#ifndef FMILIB_BUILDING_LIBRARY
#define FMILIB_BUILDING_LIBRARY
#endif
#include "fmilib.h"

void fmi2logger(fmi2_component_t c, fmi2_string_t instanceName, fmi2_status_t status, fmi2_string_t category, fmi2_string_t message, ...);
const char* getFMI2ModelVariableVariability(fmi2_import_variable_t* variable);
const char* getFMI2ModelVariableCausality(fmi2_import_variable_t* variable);
const char* getFMI2ModelVariableBaseType(fmi2_import_variable_t* variable);
char* getFMI2ModelVariableName(fmi2_import_variable_t* variable);
void* getFMI2ModelVariableStartValue(fmi2_import_variable_t* variable, int hasStartValue);
void FMIImpl__initializeFMI2Import(fmi2_import_t* fmi, void** fmiInfo, fmi_version_enu_t version, void** typeDefinitionsList, void** experimentAnnotation,
    void** modelVariablesInstance, void** modelVariablesList, int input_connectors, int output_connectors);

#ifdef __cplusplus
  }
#endif

#endif // __FMI2_IMPL_H
