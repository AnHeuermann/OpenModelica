/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Linkoping University,
 * Department of Computer and Information Science,
 * SE-58183 Linkoping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 
 * AND THIS OSMC PUBLIC LICENSE (OSMC-PL). 
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S  
 * ACCEPTANCE OF THE OSMC PUBLIC LICENSE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from Linkoping University, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or  
 * http://www.openmodelica.org, and in the OpenModelica distribution. 
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS
 * OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */

#ifndef MODELICA_PARSER_COMMON_H
#define MODELICA_PARSER_COMMON_H

#ifdef __cplusplus
extern "C" {
#endif

#include "systemimpl.h"

extern int ModelicaParser_flags;
extern int ModelicaParser_readonly;
extern void *ModelicaParser_filename_RML;
extern const char *ModelicaParser_filename_C;
extern const char *ModelicaParser_filename_C_testsuiteFriendly;
extern int ModelicaParser_lexerError;
extern const char *ModelicaParser_encoding;

#define PARSE_MODELICA        0
#define PARSE_FLAT            1<<0
#define PARSE_META_MODELICA   1<<1
#define PARSE_EXPRESSION      1<<2
#define PARSE_CODE_EXPRESSION 1<<3
#define PARSE_PARMODELICA     1<<4
#define PARSE_OPTIMICA        1<<5
#define metamodelica_enabled() (ModelicaParser_flags&PARSE_META_MODELICA)
#define parmodelica_enabled() (ModelicaParser_flags&PARSE_PARMODELICA)
#define optimica_enabled() (ModelicaParser_flags&PARSE_OPTIMICA)
#define code_expressions_enabled() (ModelicaParser_flags&PARSE_CODE_EXPRESSION)
#define flat_modelica_enabled() (ModelicaParser_flags&PARSE_FLAT)

#ifdef __cplusplus
}
#endif

#endif
