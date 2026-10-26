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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../options.h"
#include "../jacobian_util.h"
#include "../simulation_info_json.h"
#include "../../util/omc_error.h"
#include "../../util/omc_file.h"
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
  long numberOfIterations;
  DATA_HYBRD* dataHybrid;

  /* The homotopy paths of a solve, reported by the Rust solver */
  HomObserver observer;
  int runHomotopy;
  FILE* pathFile;               /* the path of OMC_LOG_INIT_HOMOTOPY */
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
 * @brief Write a message of the Rust solver to the stream of its target.
 */
static void logger(HomLogLevel level, const char* target, const char* message)
{
  int stream = OMC_LOG_NLS_V;
  if (strcmp(target, "NLS_HOMOTOPY") == 0) {
    stream = OMC_LOG_NLS_HOMOTOPY;
  } else if (strcmp(target, "DT") == 0) {
    stream = OMC_LOG_DT;
  }
  if (level <= 2) {
    warningStreamPrint(stream, 0, "%s", message);
  } else {
    infoStreamPrint(stream, 0, "%s", message);
  }
}

/**
 * @brief Open the CSV file of the path, like homotopyAlgorithm with
 * OMC_LOG_INIT_HOMOTOPY, and write its start point y.
 */
static void openPathFile(DATA_HOMOTOPY* homotopyData, int direction, const double* y)
{
#if !defined(OMC_NO_FILESYSTEM)
  DATA* data = homotopyData->userData->data;
  NONLINEAR_SYSTEM_DATA* nlsData = homotopyData->userData->nlsData;
  const char* sep = ",";
  const char* method = data->callback->homotopyMethod == GLOBAL_ADAPTIVE_HOMOTOPY ? "global" : "local";
  const char* sign = direction > 0 ? "pos" : "neg";
  char buffer[4096];
  size_t i;

  if (omc_flag[FLAG_OUTPUT_PATH]) {
    snprintf(buffer, sizeof(buffer), "%s/%s_nonlinsys%d_adaptive_%s_homotopy_%s.csv",
             omc_flagValue[FLAG_OUTPUT_PATH], data->modelData->modelFilePrefix,
             homotopyData->userData->sysNumber, method, sign);
  } else {
    snprintf(buffer, sizeof(buffer), "%s_nonlinsys%d_adaptive_%s_homotopy_%s.csv",
             data->modelData->modelFilePrefix, homotopyData->userData->sysNumber, method, sign);
  }
  infoStreamPrint(OMC_LOG_INIT_HOMOTOPY, 0, "The homotopy path will be exported to %s.", buffer);
  homotopyData->pathFile = omc_fopen(buffer, "wt");
  if (homotopyData->pathFile == NULL) {
    return;
  }
  fprintf(homotopyData->pathFile, "\"sep=%s\"\n%s", sep, "\"lambda\"");
  for (i = 0; i < homotopyData->n; i++) {
    fprintf(homotopyData->pathFile, "%s\"%s\"", sep,
            modelInfoGetEquation(&data->modelData->modelDataXml, nlsData->equationIndex).vars[i]);
  }
  fprintf(homotopyData->pathFile, "\n");
  fprintf(homotopyData->pathFile, "0.0");
  for (i = 0; i < homotopyData->n; i++) {
    fprintf(homotopyData->pathFile, "%s%.16g", sep, y[i]);
  }
  fprintf(homotopyData->pathFile, "\n");
#endif
}

static void closePathFile(DATA_HOMOTOPY* homotopyData)
{
  if (homotopyData->pathFile != NULL) {
    fclose(homotopyData->pathFile);
    homotopyData->pathFile = NULL;
  }
}

/**
 * @brief A homotopy path starts, log it like solveHomotopy.
 */
static void pathStarted(void* user, int direction, const double* y, size_t size)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  NONLINEAR_SYSTEM_DATA* nlsData = homotopyData->userData->nlsData;
  (void) size;

  homotopyData->runHomotopy++;
  if (nlsData->initHomotopy) {
    if (homotopyData->runHomotopy == 1) {
      debugInt(OMC_LOG_INIT_HOMOTOPY, "Homotopy run: ", homotopyData->runHomotopy);
      debugDouble(OMC_LOG_INIT_HOMOTOPY, "startDirection = ", direction);
    } else {
      infoStreamPrint(OMC_LOG_ASSERT, 0, "The homotopy algorithm is started again with opposing start direction.");
      debugInt(OMC_LOG_INIT_HOMOTOPY, "Homotopy run: ", homotopyData->runHomotopy);
      debugDouble(OMC_LOG_INIT_HOMOTOPY, "Try again with startDirection = ", direction);
    }
    if (OMC_ACTIVE_STREAM(OMC_LOG_INIT_HOMOTOPY)) {
      openPathFile(homotopyData, direction, y);
    }
  } else {
    debugInt(OMC_LOG_NLS_HOMOTOPY, "Homotopy run: ", homotopyData->runHomotopy);
    if (homotopyData->runHomotopy <= 2) {
      debugDouble(OMC_LOG_NLS_HOMOTOPY, "STARTING NEWTON HOMOTOPY METHOD; startDirection = ", direction);
    } else {
      debugDouble(OMC_LOG_NLS_HOMOTOPY, "STARTING FIXPOINT HOMOTOPY METHOD = ", direction);
    }
  }
}

/**
 * @brief A try of a step starts, log λ like homotopyAlgorithm.
 */
static void stepStarted(void* user, const double* y, size_t size)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  int stream = homotopyData->userData->nlsData->initHomotopy ? OMC_LOG_INIT_HOMOTOPY : OMC_LOG_NLS_HOMOTOPY;
  infoStreamPrint(stream, 0, "homotopy parameter lambda = %g", y[size-1]);
}

/**
 * @brief A step was accepted, write it to the CSV file of the path.
 */
static void stepAccepted(void* user, const double* y, size_t size)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  size_t i;
  if (homotopyData->pathFile != NULL) {
    fprintf(homotopyData->pathFile, "%.16g", y[size-1]);
    for (i = 0; i < size-1; i++) {
      fprintf(homotopyData->pathFile, ",%.16g", y[i]);
    }
    fprintf(homotopyData->pathFile, "\n");
  }
}

/**
 * @brief The path reached λ = 1, count its steps like homotopyAlgorithm.
 */
static void pathFinished(void* user, const double* y, size_t size, size_t steps)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;
  DATA* data = homotopyData->userData->data;
  modelica_boolean initHomotopy = homotopyData->userData->nlsData->initHomotopy;

  infoStreamPrint(initHomotopy ? OMC_LOG_INIT_HOMOTOPY : OMC_LOG_NLS_HOMOTOPY, 0,
                  "homotopy parameter lambda = %g", y[size-1]);
  if (initHomotopy) {
    data->simulationInfo->homotopySteps += (int) steps;
    debugInt(OMC_LOG_INIT_HOMOTOPY, "Total number of lambda steps for this homotopy loop:", (int) steps);
  }
  closePathFile(homotopyData);
}

/**
 * @brief The path failed, warn like homotopyAlgorithm for the homotopy of
 * the initialization.
 */
static void pathFailed(void* user, HomPathFailure failure, double value)
{
  DATA_HOMOTOPY* homotopyData = (DATA_HOMOTOPY*) user;

  closePathFile(homotopyData);
  if (!homotopyData->userData->nlsData->initHomotopy) {
    debugDouble(OMC_LOG_NLS_HOMOTOPY, "Homotopy algorithm did not converge:", value);
    return;
  }
  switch (failure) {
  case HOM_PATH_FAILURE_TAU_AT_MINIMUM:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nNo solution for current step size tau found and tau cannot be decreased any further.\nYou can set the minimum step size tau with:\n\t-homTauMin=<value>\nYou can also try to allow more newton steps in the corrector step with:\n\t-homMaxNewtonSteps=<value>\nor change the tolerance for the solution with:\n\t-homHEps=<value>\nYou can also try to use another backtrace stategy in the corrector step with:\n\t-homBacktraceStrategy=<fix|orthogonal>\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.");
    break;
  case HOM_PATH_FAILURE_TOO_MANY_TRIES:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nThe maximum number of tries for one lambda is reached (%d).\nYou can change the number of tries with:\n\t-homMaxTries=<value>\nYou can also try to allow more newton steps in the corrector step with:\n\t-homMaxNewtonSteps=<value>\nor change the tolerance for the solution with:\n\t-homHEps=<value>\nYou can also try to use another backtrace stategy in the corrector step with:\n\t-homBacktraceStrategy=<fix|orthogonal>\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.", (int) value);
    break;
  case HOM_PATH_FAILURE_LAMBDA_BELOW_MINUS_ONE:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nlambda is smaller than -1: lambda=%g\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.", value);
    break;
  case HOM_PATH_FAILURE_TOO_MANY_STEPS:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nThe maximum number of lambda steps is reached (%d).\nYou can change the maximum number of lambda steps with:\n\t-homMaxLambdaSteps=<value>\nYou can also try to influence the step size tau with the following flags:\n\t-homTauDecFac=<value>\n\t-homTauDecFacPredictor=<value>\n\t-homTauIncFac=<value>\n\t-homTauIncThreshold=<value>\n\t-homTauMax=<value>\n\t-homTauMin=<value>\n\t-homTauStart=<value>\nor you can also set the threshold for accepting the current bending with:\n\t-homAdaptBend=<value>\nYou can also try to use another backtrace stategy in the corrector step with:\n\t-homBacktraceStrategy=<fix|orthogonal>\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.", (int) value);
    break;
  case HOM_PATH_FAILURE_MODEL_ERROR:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nIt was not possible to calculate the jacobian.\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.");
    break;
  case HOM_PATH_FAILURE_SINGULAR:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nThe system is singular and not solvable.\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.");
    break;
  case HOM_PATH_FAILURE_PREDICTOR_BACKED_OFF:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nThe step size tau cannot be decreased anymore and current tau=%g already failed.\nYou can influence the calculation of tau with the following flags:\n\t-homTauDecFac=<value>\n\t-homTauDecFacPredictor=<value>\n\t-homTauIncFac=<value>\n\t-homTauIncThreshold=<value>\n\t-homTauMax=<value>\n\t-homTauMin=<value>\n\t-homTauStart=<value>\nYou can also set the threshold for accepting the current bending with:\n\t-homAdaptBend=<value>\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.", value);
    break;
  case HOM_PATH_FAILURE_CORRECTOR_STUCK:
    warningStreamPrint(OMC_LOG_ASSERT, 0, "Homotopy algorithm did not converge.\nThe value specifying the bending of the homotopy curve is smaller than DBL_EPSILON (increment zero).\nYou can use -lv=LOG_INIT_HOMOTOPY,LOG_NLS_HOMOTOPY to get more information.");
    break;
  }
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

  homotopyData->observer.user = homotopyData;
  homotopyData->observer.path_started = pathStarted;
  homotopyData->observer.step_started = stepStarted;
  homotopyData->observer.step_accepted = stepAccepted;
  homotopyData->observer.path_finished = pathFinished;
  homotopyData->observer.path_failed = pathFailed;

  /* Format the trace messages of the solver only if they are logged */
  hom_set_logger(logger, OMC_ACTIVE_STREAM(OMC_LOG_NLS_V) || OMC_ACTIVE_STREAM(OMC_LOG_NLS_HOMOTOPY) ||
                         OMC_ACTIVE_STREAM(OMC_LOG_DT) ? 5 : 2);

  return homotopyData;
}

/**
 * @brief Free the homotopy solver.
 */
void freeHomotopyData(DATA_HOMOTOPY* homotopyData)
{
  closePathFile(homotopyData);
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
  HomStatus status;

  model.user = homotopyData;
  model.num_residuals = homotopyData->n;
  model.num_unknowns = homotopyData->numUnknowns;
  model.nominal = nlsData->nominal;
  model.min = nlsData->min;
  model.max = nlsData->max;
  model.casual_tearing_set = homotopyData->casualTearingSet;
  model.residual = residual;
  model.jacobian = homotopyData->trackLastX ? jacobian : NULL;
  model.observer = &homotopyData->observer;

  memcpy(homotopyData->xSolution, homotopyData->xStart, homotopyData->n*sizeof(double));
  homotopyData->lastXValid = FALSE;
  homotopyData->backupRelations = TRUE;
  homotopyData->runHomotopy = 0;

  if (nlsData->initHomotopy) {
    status = hom_solve_own_homotopy(homotopyData->solver, &model, homotopyData->xSolution,
                                    omc_flag[FLAG_HOMOTOPY_NEG_START_DIR], solvedBy);
  } else {
    status = hom_solve(homotopyData->solver, &model, homotopyData->xSolution, solvedBy);
  }
  homotopyData->numberOfIterations += (long) hom_solver_statistics(homotopyData->solver).newton_iterations;
  return status;
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
  nlsData->numberOfIterations = homotopyData->numberOfIterations;

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
