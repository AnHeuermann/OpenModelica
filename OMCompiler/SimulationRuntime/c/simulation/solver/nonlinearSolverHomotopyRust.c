/*
 * This file belongs to the OpenModelica Run-Time System
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC), c/o Linköpings
 * universitet, Department of Computer and Information Science, SE-58183 Linköping, Sweden. All rights
 * reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF THE BSD NEW LICENSE OR THE
 * AGPL VERSION 3 LICENSE OR THE OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8. ANY
 * USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S
 * ACCEPTANCE OF THE BSD NEW LICENSE OR THE OSMC PUBLIC LICENSE OR THE AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium) Public License
 * (OSMC-PL) are obtained from OSMC, either from the above address, from the URLs:
 * http://www.openmodelica.org or https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica, and in the OpenModelica distribution. GNU
 * AGPL version 3 is obtained from: https://www.gnu.org/licenses/licenses.html#GPL. The BSD NEW
 * License is obtained from: http://www.opensource.org/licenses/BSD-3-Clause.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY
 * SET FORTH IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF
 * OSMC-PL.
 *
 */

#if !defined(OMC_NUM_LINEAR_SYSYTEMS) || OMC_NUM_NONLINEAR_SYSTEMS>0

/*! \file nonlinearSolverHomotopyRust.c
 *
 * The homotopy solver of nls_solver/homotopy_nls_solver, a Rust port of
 * nonlinearSolverHomotopy.c, behind the functions of nonlinearSolverHomotopy.h.
 * This file connects the C interface of the Rust solver (homotopy_capi.h) to
 * the runtime: it evaluates the system through NONLINEAR_SYSTEM_DATA, catches
 * the asserts of the model, and handles the start values, the relations of
 * mixed systems and the flags.
 */

#include <stdlib.h>
#include <string.h>

#include "../options.h"
#include "../jacobian_util.h"
#include "../../util/omc_error.h"
#include "../../util/rtclock.h"
#include "model_help.h"
#include "nonlinearSystem.h"
#include "nonlinearSolverHomotopy.h"
#include "nonlinearSolverHybrd.h"

#include "homotopy_capi.h"

struct DATA_HOMOTOPY {
  size_t n;                     /* number of residuals and of unknowns without λ */
  NLS_USERDATA* userData;
  HomSolver* solver;

  size_t numUnknowns;           /* unknowns of the current solve, n or n+1 with λ */
  modelica_boolean casualTearingSet;
  /* A system with homotopy that is solved without it: the residual function
     reads λ = 1 after the n unknowns, so x is copied into a point with λ */
  modelica_boolean appendLambda;
  double* x;
  /* With an analytical Jacobian, the point of the latest successful residual
     evaluation. evalJacobian evaluates at the state of the model, so the
     residual is evaluated again if the Jacobian is wanted at another point. */
  modelica_boolean trackLastX;
  double* lastX;
  modelica_boolean lastXValid;
  double* f;                    /* residual buffer of the Jacobian callback */
  double* jac;                  /* dense Jacobian of evalJacobian */
  size_t jacRows;               /* row stride of jac */

  double* xStart;
  double* xSolution;
  modelica_boolean* relationsBackup;
  modelica_boolean backupRelations;

  long numberOfFunctionEvaluations;
  DATA_HYBRD* dataHybrid;
};

/**
 * @brief Evaluate the residual at x for the Rust solver.
 *
 * Runs the residual function under OMC_TRY_INTERNAL, so an assert of the
 * model returns HOM_EVAL_ASSERT instead of leaving with longjmp across the
 * Rust solver.
 */
static int residual(void* user, const double* x, double* f)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  DATA* data = homotopyData->userData->data;
  threadData_t* threadData = homotopyData->userData->threadData;
  NONLINEAR_SYSTEM_DATA* nlsData = homotopyData->userData->nlsData;
  RESIDUAL_USERDATA resUserData = {.data=data, .threadData=threadData, .solverData=NULL};
  int iflag = 0;
  const double* xModel = x;
  volatile int status = HOM_EVAL_ASSERT;

  if (homotopyData->appendLambda) {
    memcpy(homotopyData->x, x, homotopyData->n*sizeof(double));
    xModel = homotopyData->x;
  }
  homotopyData->lastXValid = FALSE;

#ifndef OMC_EMCC
  OMC_TRY_INTERNAL(simulationJumpBuffer)
#endif
  if (homotopyData->casualTearingSet) {
    status = nlsData->residualFuncConstraints(&resUserData, xModel, f, &iflag)
             ? HOM_EVAL_CONSTRAINT_VIOLATED : HOM_EVAL_OK;
  } else {
    nlsData->residualFunc(&resUserData, xModel, f, &iflag);
    status = HOM_EVAL_OK;
  }
  if (OMC_ERROR_RAISED()) {
    OMC_ERROR_CLEAR();
    status = HOM_EVAL_ASSERT;
  }
#ifndef OMC_EMCC
  OMC_CATCH_INTERNAL(simulationJumpBuffer)
#endif

  homotopyData->numberOfFunctionEvaluations++;
  if (status == HOM_EVAL_OK) {
    if (homotopyData->trackLastX) {
      memcpy(homotopyData->lastX, x, homotopyData->numUnknowns*sizeof(double));
      homotopyData->lastXValid = TRUE;
    }
    /* Like solveHomotopy, the relations at the start point, to compare a
       solution of a mixed system with */
    if (homotopyData->backupRelations) {
      memcpy(homotopyData->relationsBackup, data->simulationInfo->relations,
             data->modelData->nRelations*sizeof(modelica_boolean));
      homotopyData->backupRelations = FALSE;
      /* Newton's method and the homotopies run with the relations kept */
      data->simulationInfo->solveContinuous = 1;
    }
  }
  return status;
}

/**
 * @brief Evaluate the analytical Jacobian at x for the Rust solver.
 *
 * evalJacobian evaluates at the state of the model, so the residual is
 * evaluated first unless x is the point of its latest evaluation. The columns
 * of the solver's unknowns are copied from the possibly larger dense Jacobian.
 */
static int jacobian(void* user, const double* x, const double* f, double* jac)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  DATA* data = homotopyData->userData->data;
  threadData_t* threadData = homotopyData->userData->threadData;
  NONLINEAR_SYSTEM_DATA* nlsData = homotopyData->userData->nlsData;
  JACOBIAN* analyticJacobian = homotopyData->userData->analyticJacobian;
  size_t n = homotopyData->n;
  size_t i, j;
  volatile int status = HOM_EVAL_ASSERT;
  (void) f;

  if (!homotopyData->lastXValid ||
      memcmp(homotopyData->lastX, x, homotopyData->numUnknowns*sizeof(double)) != 0) {
    int residualStatus = residual(user, x, homotopyData->f);
    if (residualStatus != HOM_EVAL_OK) {
      return residualStatus;
    }
  }

  rt_ext_tp_tick(&nlsData->jacobianTimeClock);
#ifndef OMC_EMCC
  OMC_TRY_INTERNAL(simulationJumpBuffer)
#endif
  evalJacobian(data, threadData, analyticJacobian, NULL, homotopyData->jac, TRUE);
  status = HOM_EVAL_OK;
  if (OMC_ERROR_RAISED()) {
    OMC_ERROR_CLEAR();
    status = HOM_EVAL_ASSERT;
  }
#ifndef OMC_EMCC
  OMC_CATCH_INTERNAL(simulationJumpBuffer)
#endif
  nlsData->jacobianTime += rt_ext_tp_tock(&nlsData->jacobianTimeClock);
  nlsData->numberOfJEval++;

  if (status == HOM_EVAL_OK) {
    for (j = 0; j < homotopyData->numUnknowns; j++) {
      for (i = 0; i < n; i++) {
        jac[j*n + i] = homotopyData->jac[j*homotopyData->jacRows + i];
      }
    }
  }
  return status;
}

/**
 * @brief Allocate the homotopy solver for a system of size unknowns.
 *
 * The settings are the newton* and hom* flags. The hybrd data is allocated
 * like in nonlinearSolverHomotopy.c, it owns userData.
 */
DATA_HOMOTOPY* allocateHomotopyData(size_t size, NLS_USERDATA* userData)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) calloc(1, sizeof(DATA_HOMOTOPY));
  DATA* data = userData->data;
  JACOBIAN* analyticJacobian = userData->analyticJacobian;
  size_t jacRows = size;
  size_t jacCols = size + 1;
  HomSettings settings = {0};

  assertStreamPrint(NULL, NULL != homotopyData, "allocateHomotopyData() failed!");
  if (homBacktraceStrategy != 1) {
    warningStreamPrint(OMC_LOG_STDOUT, 0,
      "The Rust homotopy solver only implements homBacktraceStrategy=1, using it instead of %d.",
      homBacktraceStrategy);
  }

  settings.newton_x_tol = newtonXTol;
  settings.newton_f_tol = newtonFTol;
  settings.tau_start = homTauStart;
  settings.tau_min = homTauMin;
  settings.tau_max = homTauMax;
  settings.tau_decreasing_factor = homTauDecreasingFactor;
  settings.tau_decreasing_factor_predictor = homTauDecreasingFactorPredictor;
  settings.tau_increasing_factor = homTauIncreasingFactor;
  settings.tau_increasing_threshold = homTauIncreasingThreshold;
  settings.adapt_bend = homAdaptBend;
  settings.h_eps = homHEps;
  settings.max_corrector_steps = homMaxNewtonSteps;
  settings.max_tries = homMaxTries;
  settings.max_lambda_steps = homMaxLambdaSteps; /* 0 is the default of 100·size */

  homotopyData->solver = hom_solver_new(size, &settings);
  assertStreamPrint(NULL, NULL != homotopyData->solver,
                    "allocateHomotopyData(): invalid homotopy settings");

  /* The analytical Jacobian may have more columns than unknowns, and
     evalJacobian writes min(sizeRows, sizeCols) rows */
  if (analyticJacobian != NULL) {
    if ((size_t) analyticJacobian->sizeCols > jacCols) {
      jacCols = (size_t) analyticJacobian->sizeCols;
    }
    jacRows = (size_t) (analyticJacobian->sizeRows < analyticJacobian->sizeCols
                        ? analyticJacobian->sizeRows : analyticJacobian->sizeCols);
    if (jacRows < size) {
      jacRows = size;
    }
  }

  homotopyData->n = size;
  homotopyData->userData = userData;
  homotopyData->x = (double*) calloc(size+1, sizeof(double));
  homotopyData->x[size] = 1.0; /* λ, see appendLambda */
  homotopyData->lastX = (double*) calloc(size+1, sizeof(double));
  homotopyData->f = (double*) calloc(size, sizeof(double));
  homotopyData->jac = (double*) calloc(jacRows*jacCols, sizeof(double));
  homotopyData->jacRows = jacRows;
  homotopyData->xStart = (double*) calloc(size, sizeof(double));
  homotopyData->xSolution = (double*) calloc(size, sizeof(double));
  homotopyData->relationsBackup = (modelica_boolean*) calloc(
    data->modelData->nRelations > 0 ? data->modelData->nRelations : 1, sizeof(modelica_boolean));
  homotopyData->dataHybrid = allocateHybrdData(size, userData);

  return homotopyData;
}

/**
 * @brief Free the homotopy solver.
 */
void freeHomotopyData(DATA_HOMOTOPY* homotopyData)
{
  hom_solver_free(homotopyData->solver);
  free(homotopyData->x);
  free(homotopyData->lastX);
  free(homotopyData->f);
  free(homotopyData->jac);
  free(homotopyData->xStart);
  free(homotopyData->xSolution);
  free(homotopyData->relationsBackup);
  /* Don't free userData here, it's done in freeHybrdData */
  freeHybrdData(homotopyData->dataHybrid);
  free(homotopyData);
}

/**
 * @brief Solve the system once with the Rust solver from xStart.
 */
static HomStatus solveOnce(DATA_HOMOTOPY* homotopyData, NONLINEAR_SYSTEM_DATA* nlsData,
                           HomSolvedBy* solvedBy)
{
  HomModel model = {0};

  model.user = homotopyData;
  model.num_residuals = homotopyData->n;
  model.num_unknowns = homotopyData->numUnknowns;
  model.nominal = nlsData->nominal;
  model.min = nlsData->min;
  model.max = nlsData->max;
  model.casual_tearing_set = homotopyData->casualTearingSet;
  model.residual = residual;
  model.jacobian = homotopyData->trackLastX ? jacobian : NULL;

  memcpy(homotopyData->xSolution, homotopyData->xStart, homotopyData->n*sizeof(double));
  homotopyData->lastXValid = FALSE;
  homotopyData->backupRelations = TRUE;

  if (nlsData->initHomotopy) {
    return hom_solve_own_homotopy(homotopyData->solver, &model, homotopyData->xSolution,
                                  omc_flag[FLAG_HOMOTOPY_NEG_START_DIR], solvedBy);
  }
  return hom_solve(homotopyData->solver, &model, homotopyData->xSolution, solvedBy);
}

/**
 * @brief Solve the nonlinear system with the Rust homotopy solver.
 *
 * Like solveHomotopy of nonlinearSolverHomotopy.c: the start point is the
 * current value in a discrete call and the extrapolation otherwise. A
 * solution of a mixed system in a discrete call is checked once against the
 * relations at the start point, and solved again if they changed.
 */
NLS_SOLVER_STATUS solveHomotopy(DATA *data, threadData_t *threadData, NONLINEAR_SYSTEM_DATA* nlsData)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) nlsData->solverData;
  size_t n = homotopyData->n;
  HomSolvedBy solvedBy;
  HomStatus status;
  (void) threadData;

  homotopyData->casualTearingSet = nlsData->strictTearingFunctionCall != NULL;
  homotopyData->numUnknowns = nlsData->initHomotopy ? n + 1 : n;
  homotopyData->appendLambda = nlsData->homotopySupport && !nlsData->initHomotopy &&
                               nlsData->size > n;
  homotopyData->trackLastX = nlsData->jacobianIndex != -1 &&
                             homotopyData->userData->analyticJacobian != NULL;

  memcpy(homotopyData->xStart,
         data->simulationInfo->discreteCall ? nlsData->nlsx : nlsData->nlsxExtrapolation,
         n*sizeof(double));

  if (data->simulationInfo->discreteCall) {
    data->simulationInfo->solveContinuous = 0;
  }
  status = solveOnce(homotopyData, nlsData, &solvedBy);

  if (status == HOM_STATUS_SOLVED && nlsData->mixedSystem && data->simulationInfo->discreteCall &&
      !nlsData->initHomotopy) {
    data->simulationInfo->solveContinuous = 0;
    if (residual(homotopyData, homotopyData->xSolution, homotopyData->f) != HOM_EVAL_OK) {
      status = HOM_STATUS_NOT_SOLVED;
    } else if (memcmp(data->simulationInfo->relations, homotopyData->relationsBackup,
                      data->modelData->nRelations*sizeof(modelica_boolean)) != 0) {
      /* The relations changed, solve once more from the start point */
      status = solveOnce(homotopyData, nlsData, &solvedBy);
    }
  }

  if (status == HOM_STATUS_SOLVED) {
    memcpy(nlsData->nlsx, homotopyData->xSolution, n*sizeof(double));
  }
  data->simulationInfo->solveContinuous = 0;
  nlsData->numberOfFEval = homotopyData->numberOfFunctionEvaluations;

  return status == HOM_STATUS_SOLVED ? NLS_SOLVED : NLS_FAILED;
}

/**
 * @brief Return the dense Jacobian of the latest analytical evaluation.
 */
double* getHomotopyJacobian(NONLINEAR_SYSTEM_DATA* nlsData)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*)(nlsData->solverData);
  return homotopyData->jac;
}

#endif
