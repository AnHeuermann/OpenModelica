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

#include "context.h"
#include "../simulation_data.h"

const char *EVAL_CONTEXT_STRING[CONTEXT_MAX] = {
  "UNKNOWN",
  "ODE evaluation",
  "algebraic evaluation",
  "event search",
  "jacobian evaluation",
  "symbolica jacobian evaluation"
};

/**
 * @brief  Set current context in simulation info object
 *
 * @param data              Pointer to data struct.
 * @param currentTime       Current simulation time.
 * @param currentContext    Evaluation context.
 */
void setContext(DATA* data, double currentTime, EVAL_CONTEXT currentContext) {
  data->simulationInfo->currentContextOld = data->simulationInfo->currentContext;
  data->simulationInfo->currentContext = currentContext;
  infoStreamPrint(OMC_LOG_SOLVER_CONTEXT, 0, "+++ Set context %s +++ at time %f", EVAL_CONTEXT_STRING[currentContext], currentTime);
  if (currentContext == CONTEXT_JACOBIAN ||
      currentContext == CONTEXT_SYM_JACOBIAN)
  {
    data->simulationInfo->currentJacobianEval = 0;
  }
}

/**
 * @brief Restores previous context in simulation info object.
 *
 * @param data  Pointer to data struct.
 */
void unsetContext(DATA* data) {
  infoStreamPrint(OMC_LOG_SOLVER_CONTEXT, 0, "--- Unset context %s ---", EVAL_CONTEXT_STRING[data->simulationInfo->currentContext]);
  data->simulationInfo->currentContext = data->simulationInfo->currentContextOld;
}

/**
 * @brief Increase currentJacobianEval in (symbolic) Jacobian context.
 *
 * @param data  Pointer to data struct.
 */
void increaseJacContext(DATA* data) {
  EVAL_CONTEXT currentContext = data->simulationInfo->currentContext;
  if (currentContext == CONTEXT_JACOBIAN ||
      currentContext == CONTEXT_SYM_JACOBIAN)
  {
    data->simulationInfo->currentJacobianEval++;
    infoStreamPrint(OMC_LOG_SOLVER_CONTEXT, 0, "+++ Increase Jacobian column context %s +++ to %d", EVAL_CONTEXT_STRING[currentContext], data->simulationInfo->currentJacobianEval);
  }
}
