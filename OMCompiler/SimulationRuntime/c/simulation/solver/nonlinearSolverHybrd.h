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

/*! \file nonlinearSolverHybrd.h
 */

#ifndef _NONLINEARSOLVERHYBRD_H_
#define _NONLINEARSOLVERHYBRD_H_

#include "../../openmodelica_types.h"
#include "../../simulation_data.h"
#include "nonlinearSystem.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct DATA_HYBRD
{
  modelica_boolean initialized;
  double* resScaling;
  int useXScaling;
  double* xScalefactors;
  double* fvecScaled;

  integer n;
  double* x;
  double* xSave;
  double* xScaled;
  double* fvec;
  double* fvecSave;
  double xtol;
  integer maxfev;
  int ml;
  int mu;
  double epsfcn;
  double* diag;
  double* diagres;
  integer mode;
  double factor;
  integer nprint;
  integer info;
  integer nfev;
  integer njev;
  double* fjac;         /* Jacobian matrix in row-major order */
  double* fjacobian;
  integer ldfjac;
  double* r__;
  integer lr;
  double* qtf;
  double* wa1;
  double* wa2;
  double* wa3;
  double* wa4;

  unsigned int numberOfIterations; /* over the whole simulation time */
  unsigned int numberOfFunctionEvaluations; /* over the whole simulation time */

  NLS_USERDATA* userData;        /* User data provided to KINSOL */

} DATA_HYBRD;

extern
void hybrj_( void(*) (const integer*, const double*, double*, double *, const integer*, const integer*, void*),  const integer *n, double *x, double *fvec, double *fjac, const integer *ldfjac,
  const double *xtol, const integer *axfev, double *diag, const integer *mode,
  const double *factor, const integer *nprint, integer *info, integer *nfev, integer *njev,
  double *r, integer *lr, double *qtf, double *wa1, double *wa2,
  double *wa3, double *wa4, void* user_data);

DATA_HYBRD* allocateHybrdData(size_t size, NLS_USERDATA* userData);
void freeHybrdData(DATA_HYBRD* hybrdData);
NLS_SOLVER_STATUS solveHybrd(DATA *data, threadData_t *threadData, NONLINEAR_SYSTEM_DATA* nlsData);

#ifdef __cplusplus
}
#endif

#endif
