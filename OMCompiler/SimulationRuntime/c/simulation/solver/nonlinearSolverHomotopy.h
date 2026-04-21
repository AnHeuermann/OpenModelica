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

/*! \file nonlinearSolverHomotopy.h
 */

#ifndef _NONLINEARSOLVERHOMOTOPY_H_
#define _NONLINEARSOLVERHOMOTOPY_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "../../openmodelica_types.h"
#include "../../simulation_data.h"
#include "../../simulation_data.h"

typedef struct DATA_HOMOTOPY DATA_HOMOTOPY;

DATA_HOMOTOPY* allocateHomotopyData(size_t size, NLS_USERDATA* userData);
void freeHomotopyData(DATA_HOMOTOPY* homotopyData);
NLS_SOLVER_STATUS solveHomotopy(DATA *data, threadData_t *threadData, NONLINEAR_SYSTEM_DATA* nlsData);
double* getHomotopyJacobian(NONLINEAR_SYSTEM_DATA* nlsData);

#ifdef __cplusplus
};
#endif

#endif /* _NONLINEARSOLVERHOMOTOPY_H_ */
