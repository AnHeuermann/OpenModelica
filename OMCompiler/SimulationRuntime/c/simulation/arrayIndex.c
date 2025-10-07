/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF THE BSD NEW LICENSE OR THE
 * GPL VERSION 3 LICENSE OR THE OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3,
 * ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs: http://www.openmodelica.org or
 * http://www.ida.liu.se/projects/OpenModelica, and in the OpenModelica
 * distribution. GNU version 3 is obtained from:
 * http://www.gnu.org/copyleft/gpl.html. The New BSD License is obtained from:
 * http://www.opensource.org/licenses/BSD-3-Clause.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without even the implied
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, EXCEPT AS
 * EXPRESSLY SET FORTH IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE
 * CONDITIONS OF OSMC-PL.
 *
 */

/*! \file arrayIndex.c
 *
 * Handling of Index mapping between array variables and scalar representation
 * in simulation data.
 */

#include "arrayIndex.h"
#include "omc_error.h"

/**
 * @brief Allocate memory for index maps.
 *
 * @param simulationInfo
 * @param modelData
 * @param threadData
 */
void allocateArrayIndexMaps(SIMULATION_INFO *simulationInfo, MODEL_DATA *modelData, threadData_t *threadData)
{
  simulationInfo->realVarsIndex = (size_t *)calloc(modelData->nVariablesRealArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, NULL != simulationInfo->realVarsIndex, "Out of memory");

  simulationInfo->integerVarsIndex = (size_t *)calloc(modelData->nVariablesIntegerArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, NULL != simulationInfo->integerVarsIndex, "Out of memory");

  simulationInfo->booleanVarsIndex = (size_t *)calloc(modelData->nVariablesBooleanArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, NULL != simulationInfo->booleanVarsIndex, "Out of memory");

  simulationInfo->stringVarsIndex = (size_t *)calloc(modelData->nVariablesStringArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, NULL != simulationInfo->stringVarsIndex, "Out of memory");
}

void freeArrayIndexMaps(SIMULATION_INFO *simulationInfo)
{
  free(simulationInfo->realVarsIndex);
  free(simulationInfo->integerVarsIndex);
  free(simulationInfo->booleanVarsIndex);
  free(simulationInfo->stringVarsIndex);
}

/**
 * @brief Calculate length of multi-dimensional array.
 *
 * Example Tensor T[2][3][4]:
 *   <dimension start=2>
 *   <dimension start=3>
 *   <dimension start=4>
 * will result in length 2*3*4 = 24
 *
 * A scalar variable with no dimension info will always be size 1.
 *
 * @param dimensionInfo
 * @return size_t
 */
size_t calculateLength(DIMENSION_INFO *dimensionInfo)
{
  size_t length = 1;
  size_t dim_idx;
  DIMENSION_ATTRIBUTE *dimensionAttribute;

  if (dimensionInfo == NULL || dimensionInfo->numberOfDimensions == 0 || dimensionInfo->dimensions == NULL)
  {
    return length;
  }

  for (dim_idx = 0; dim_idx < dimensionInfo->numberOfDimensions; dim_idx++)
  {
    dimensionAttribute = &dimensionInfo->dimensions[dim_idx];
    assertStreamPrint(NULL, dimensionAttribute != NULL, "DIMENSION_ATTRIBUTE is NULL");

    switch (dimensionAttribute->type)
    {
    case DIMENSION_BY_START:
      length = length * dimensionAttribute->start;
      break;

    case DIMENSION_BY_VALUE_REFERENCE:
      throwStreamPrint(NULL, "collectRealArrayVariableSizes: Not implemented yet!");
      break;

    default:
      throwStreamPrint(NULL, "collectArrayVariableSizes: Illegal dimension attribute type case!");
      break;
    }
  }

  return length;
}

/**
 * @brief
 *
 * @param variableData
 * @param type
 * @param num_variables
 * @param num_array_variables
 * @param total_array_size
 */
size_t collectArrayVariableSizes(void *variableData, enum var_type type, size_t num_variables)
{
  unsigned int i;

  DIMENSION_INFO *dimensionInfo;
  modelica_integer numberOfDimensions;

  size_t num_array_variables = 0;
  size_t total_array_size = 0;

  for (i = 0; i < num_variables; i++)
  {

    switch (type)
    {
    case T_REAL:
      dimensionInfo = &((STATIC_REAL_DATA *)variableData)[i].dimension;
      break;
    case T_INTEGER:
      dimensionInfo = &((STATIC_INTEGER_DATA *)variableData)[i].dimension;
      break;
    case T_BOOLEAN:
      dimensionInfo = &((STATIC_BOOLEAN_DATA *)variableData)[i].dimension;
      break;
    case T_STRING:
      dimensionInfo = &((STATIC_STRING_DATA *)variableData)[i].dimension;
      break;
    default:
      throwStreamPrint(NULL, "collectArrayVariableSizes: Illegal variable type case.");
    }

    if (dimensionInfo->numberOfDimensions > 0)
    {
      num_array_variables++;
      total_array_size += calculateLength(dimensionInfo);
    }
  }

  return num_array_variables;
}

/**
 * @brief Iterate over model variables and count number of array variables and dimensions
 *
 */
void allocIndexMaps(MODEL_DATA *modelData, SIMULATION_INFO *simulationInfo, threadData_t *threadData)
{

  size_t total_array_size;

  modelData->nVariablesRealArray = collectArrayVariableSizes(modelData->realVarsData, T_REAL, modelData->nVariablesReal);
  simulationInfo->realVarsIndex = (size_t *)calloc(modelData->nVariablesRealArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, simulationInfo->realVarsIndex != NULL, "Out of memory");

  modelData->nVariablesIntegerArray = collectArrayVariableSizes(modelData->integerVarsData, T_INTEGER, modelData->nVariablesInteger);
  simulationInfo->integerVarsIndex = (size_t *)calloc(modelData->nVariablesIntegerArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, simulationInfo->integerVarsIndex != NULL, "Out of memory");

  modelData->nVariablesBooleanArray = collectArrayVariableSizes(modelData->booleanVarsData, T_BOOLEAN, modelData->nVariablesBoolean);
  simulationInfo->booleanVarsIndex = (size_t *)calloc(modelData->nVariablesBooleanArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, simulationInfo->booleanVarsIndex != NULL, "Out of memory");

  modelData->nVariablesStringArray = collectArrayVariableSizes(modelData->stringVarsData, T_STRING, modelData->nVariablesString);
  simulationInfo->stringVarsIndex = (size_t *)calloc(modelData->nVariablesStringArray + 1, sizeof(size_t));
  assertStreamPrint(threadData, simulationInfo->stringVarsIndex != NULL, "Out of memory");
}

// todo: Add freeIndexMaps function

void computeVarIndices()
{
}
