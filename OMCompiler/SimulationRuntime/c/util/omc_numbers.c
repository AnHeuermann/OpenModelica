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

/* strtod_l is considered a GNU extension.
 * It will fail if the correct prototype does not exist. */
#define _GNU_SOURCE 1

#include <stdlib.h>

#if (defined(_MSC_VER) && _MSC_VER >= 1400)

#include <locale.h>


_locale_t getCLocale()
{
  static int init = 0;
  static _locale_t loc;
  if (!init) {
    loc = _create_locale(LC_NUMERIC, "C");
    init = 1;
  }
  return loc;
}

double om_strtod(const char *nptr, char **endptr)
{
  return _strtod_l(nptr, endptr, getCLocale());
}

#elif (defined(__GLIBC__) && defined(__GLIBC_MINOR__) && ((__GLIBC__ << 16) + __GLIBC_MINOR__ >= (2 << 16) + 3)) || defined(__APPLE_CC__)

#if defined(__GLIBC__)
#include <locale.h>
#elif defined(__APPLE_CC__)
#include <xlocale.h>
#include <locale.h>
#endif

locale_t getCLocale()
{
  static int init = 0;
  static locale_t loc;
  if (!init) {
    loc = newlocale(LC_NUMERIC, "C", NULL);
    init = 1;
  }
  return loc;
}

double om_strtod(const char *nptr, char **endptr)
{
  return strtod_l(nptr, endptr, getCLocale());
}

#else

double om_strtod(const char *nptr, char **endptr)
{
  /* Default to just assuming we have the correct locale */
  return strtod(nptr, endptr);
}

#endif
