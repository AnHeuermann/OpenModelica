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

#ifndef __OMC_EMBEDDED_SERVER_H
#define __OMC_EMBEDDED_SERVER_H

#include "simulation_data.h"

#if defined(__cplusplus)
extern "C" {
#endif

extern void* (*embedded_server_init)(DATA *data, double tout, double step, const char *argv_0, void (*omc_real_time_sync_update)(DATA *data, double scaling), int port);
extern void (*wait_for_step)(void *handle);
extern void (*embedded_server_deinit)(void *handle);
/* Tells the embedded server that a simulation step has passed; the server
 * can read/write values from/to the simulator
 */
extern int (*embedded_server_update)(void *handle, double tout, int *terminate);
/* Give the filename or generic name to use for loading an embedded server */
extern void* embedded_server_load_functions(const char *name);
extern void embedded_server_unload_functions(void *dllHandle);

#if defined(__cplusplus)
}
#endif

#endif
