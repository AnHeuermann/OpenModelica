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

#ifndef MODEL_CONTEXT_H
#define MODEL_CONTEXT_H

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declaration of DATA */
struct DATA;
typedef struct DATA DATA;

/**
 * @brief Describe in what context a function is evaluated.
 *
 * Set by ODE integrators.
 */
typedef enum EVAL_CONTEXT
{
  CONTEXT_UNKNOWN = 0,    /* Unknown evaluation context */

  CONTEXT_ODE,            /* Doing an ODE integrator step */
  CONTEXT_ALGEBRAIC,      /* Solving (non-)linear system  */
  CONTEXT_EVENTS,         /* Doing a event search */
  CONTEXT_JACOBIAN,       /* Evaluating (numeric) Jacobian */
  CONTEXT_SYM_JACOBIAN,   /* Evaluating symbolic Jacobian */

  CONTEXT_MAX
} EVAL_CONTEXT;

void setContext(DATA* data, double currentTime, EVAL_CONTEXT currentContext);
void unsetContext(DATA* data);
void increaseJacContext(DATA* data);

#ifdef __cplusplus
}
#endif

#endif  // MODEL_CONTEXT_H
