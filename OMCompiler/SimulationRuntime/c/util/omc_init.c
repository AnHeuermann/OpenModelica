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

#include <stdio.h>
#include "omc_init.h"
#include "../meta/meta_modelica_segv.h"

#if defined(OM_HAVE_PTHREADS)
pthread_key_t mmc_thread_data_key = 0;
pthread_once_t mmc_init_once = PTHREAD_ONCE_INIT;
#else
threadData_t *OMC_MAIN_THREADDATA_NAME = 0;
#endif

void mmc_init_nogc()
{
#if defined(OM_HAVE_PTHREADS)
  pthread_key_create(&mmc_thread_data_key,NULL);
#endif
#if !defined(OMC_MINIMAL_RUNTIME)
  /* Stack overflow detection is too expensive and fun for small targets
   * C-code is usually not generated for stack overflow detection anyway... */
  init_metamodelica_segv_handler();
#endif
}

#if defined(OMC_MINIMAL_RUNTIME)
void mmc_init()
{
  fprintf(stderr, "Error: called mmc_init (requesting garbage collection) when OMC was compiled with a minimal runtime system.");
  exit(1);
}
#else
#include "../gc/omc_gc.h"
void mmc_init()
{
  mmc_init_nogc();
  mmc_GC_init();
}
#endif
