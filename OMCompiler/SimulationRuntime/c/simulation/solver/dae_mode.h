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

#ifndef DAE_MODE_H
#define DAE_MODE_H

#include "simulation_data.h"

/* EVAL_DYNAMIC = 1000 */
extern const int EVAL_DYNAMIC;
/* EVAL_ALGEBRAIC = 0100 */
extern const int EVAL_ALGEBRAIC;
/* EVAL_ZEROCROSS = 0010 */
extern const int EVAL_ZEROCROSS;
/* EVAL_DISCRETE = 0001 */
extern const int EVAL_DISCRETE;
/* EVAL_ALL = 1111 */
extern const int EVAL_ALL;

#ifdef __cplusplus
extern "C" {
#endif

int evaluateDAEResiduals_wrapperEventUpdate(DATA* data, threadData_t* threadData);
int evaluateDAEResiduals_wrapperZeroCrossingsEquations(DATA* data, threadData_t* threadData);

void getAlgebraicDAEVarNominals(DATA*, double*);
void getAlgebraicDAEVars(DATA*, double*);
void setAlgebraicDAEVars(DATA*, double*);


#ifdef __cplusplus
}
#endif

#endif
