/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S ACCEPTANCE
 * OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from OSMC, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or
 * http://www.openmodelica.org, and in the OpenModelica distribution.
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */

#ifdef __cplusplus
extern "C" {
#endif

#include "omc_config.h"

#ifdef NO_FMIL
void FMIImpl__initializeFMI2Import(void* fmi, void** fmiInfo, int version, void** typeDefinitionsList, void** experimentAnnotation, void** modelVariablesInstance, void** modelVariablesList, int input_connectors, int output_connectors)
{
  MMC_THROW();
}
int FMIImpl__initializeFMIImport(const char* file_name, const char* working_directory, int fmi_log_level, int input_connectors, int output_connectors, void** fmiContext, void** fmiInstance, void** fmiInfo, void** typeDefinitionsList, void** experimentAnnotation, void** modelVariablesInstance, void** modelVariablesList)
{
  MMC_THROW();
}
void FMIImpl__releaseFMIImport(void *ptr1, void *ptr2, void *ptr3, const char* fmiVersion)
{
  MMC_THROW();
}
#else

#include <stdio.h>
#include <stdint.h>

#include "systemimpl.h"
#include "errorext.h"
#include "util/modelica_string.h"

#ifndef FMILIB_BUILDING_LIBRARY
#define FMILIB_BUILDING_LIBRARY
#endif
#include "fmilib.h"
#include "FMI2Impl.h"

#define mmc_mk_scon_check_null(s) s?mmc_mk_scon(s):mmc_mk_scon("")

static void importlogger(jm_callbacks* c, jm_string module, jm_log_level_enu_t log_level, jm_string message)
{
  const char* tokens[3] = {module,jm_log_level_to_string(log_level),message};
  switch (log_level) {
    case jm_log_level_fatal:
    case jm_log_level_error:
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_error, gettext("module = %s, log level = %s: %s"), tokens, 3);
      break;
    case jm_log_level_warning:
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_warning, gettext("module = %s, log level = %s: %s"), tokens, 3);
      break;
    case jm_log_level_info:
    case jm_log_level_verbose:
    case jm_log_level_debug:
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_notification, gettext("module = %s, log level = %s: %s"), tokens, 3);
      break;
    default:
      printf("module = %s, log level = %d: %s\n", module, log_level, message);fflush(NULL);
      break;
  }
}

/*
 * functions that replaces the given character old with the character new in a string
 */
void charReplace(char* variable_name, unsigned int len, char old, char new)
{
  char* res = NULL;
  res = strchr(variable_name, old);
  while (res != NULL) {
    *res = new;
    res = strchr(variable_name, old);
  }
  variable_name[len] = '\0';
}

/*
 * Makes the string safe by removing special characters. Returns a malloc'd string that should be
 * free'd.
 */
char* makeStringFMISafe(const char* str) {
  char* res = strdup(str);
  int length = strlen(res);

  charReplace(res, length, '.', '_');
  charReplace(res, length, '[', '_');
  charReplace(res, length, ']', '_');
  charReplace(res, length, ' ', '_');
  charReplace(res, length, ',', '_');
  charReplace(res, length, '(', '_');
  charReplace(res, length, ')', '_');
  return res;
}

/*
 * Initializes FMI Import.
 * Reads the Model Identifier name.
 * Reads the experiment annotation.
 * Reads the model variables.
 */
int FMIImpl__initializeFMIImport(const char* file_name, const char* working_directory, int fmi_log_level, int input_connectors, int output_connectors, int isModelDescriptionImport,
    void** fmiContext, void** fmiInstance, void** fmiInfo, void** typeDefinitionsList, void** experimentAnnotation, void** modelVariablesInstance, void** modelVariablesList)
{
  // JM callbacks
  static jm_callbacks callbacks;
  static int init_jm_callbacks = 0;
  fmi_import_context_t* context;
  fmi_version_enu_t version;
  *fmiContext = mmc_mk_some(0);
  *fmiInstance = mmc_mk_some(0);
  *fmiInfo = NULL;
  *typeDefinitionsList = NULL;
  *experimentAnnotation = NULL;
  *modelVariablesInstance = mmc_mk_some(0);
  *modelVariablesList = NULL;
  if (!init_jm_callbacks) {
    init_jm_callbacks = 1;
    callbacks.malloc = malloc;
    callbacks.calloc = calloc;
    callbacks.realloc = realloc;
    callbacks.free = free;
    callbacks.logger = importlogger;
    callbacks.log_level = fmi_log_level;
    callbacks.context = 0;
  }
  context = fmi_import_allocate_context(&callbacks);
  *fmiContext = mmc_mk_some(context);
  // extract the fmu file and read the version
  version = fmi_import_get_fmi_version(context, file_name, working_directory);
  if ((version <= fmi_version_unknown_enu) || (version >= fmi_version_unsupported_enu) || (version == fmi_version_1_enu)) {
    const char* tokens[1] = {fmi_version_to_string(version)};
    fmi_import_free_context(context);
    c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_error, gettext("The FMU version is %s. Unknown/Unsupported FMU version."), tokens, 1);
    return 0;
  }
  if (version == fmi_version_2_0_enu) {
    static int init_fmi2_callback_functions = 0;
    // FMI callback functions
    static fmi2_callback_functions_t fmi2_callback_functions;
    fmi2_import_t* fmi;
    fmi2_fmu_kind_enu_t fmiType;
    if (!init_fmi2_callback_functions) {
      init_fmi2_callback_functions = 1;
      fmi2_callback_functions.logger = fmi2logger;
      fmi2_callback_functions.allocateMemory = calloc;
      fmi2_callback_functions.freeMemory = free;
    }
    // parse the xml file
    fmi = fmi2_import_parse_xml(context, working_directory, NULL);
    if(!fmi) {
      fmi_import_free_context(context);
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_error, gettext("Error parsing the modelDescription.xml file."), NULL, 0);
      return 0;
    }
    /* remove the following block once we have support for FMI 2.0 CS. */
    fmiType = fmi2_import_get_fmu_kind(fmi);
    if (!isModelDescriptionImport && (fmiType == fmi2_fmu_kind_cs)) {
      const char* tokens[1] = {fmi2_fmu_kind_to_string(fmiType)};
      fmi2_import_free(fmi);
      fmi_import_free_context(context);
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_error, gettext("The FMU version is 2.0 and FMU type is %s. Unsupported FMU type. Only FMI 2.0 ModelExchange is supported."), tokens, 1);
      return 0;
    }
    *fmiInstance = mmc_mk_some(fmi);
    /* Loading the binary (dll/so) can mess up the compiler, and the information is unused in the compiler */
#if 0
    jm_status_enu_t status;
    status = fmi2_import_create_dllfmu(fmi, fmi2_fmu_kind_me, &fmi2_callback_functions);
    if (status == jm_status_error) {
      fmi2_import_free(fmi);
      fmi_import_free_context(context);
      c_add_message(NULL,-1, ErrorType_scripting, ErrorLevel_error, gettext("Loading of FMU dynamic link library failed."), NULL, 0);
      return 0;
    }
#endif
    FMIImpl__initializeFMI2Import(fmi, fmiInfo, version, typeDefinitionsList, experimentAnnotation, modelVariablesInstance, modelVariablesList, input_connectors, output_connectors);
  }
  /* everything is OK return success */
  return 1;
}

/*
 * Releases all the instances of FMI Import.
 * From FMIL docs; Free a variable list. Note that variable lists are allocated dynamically and must be freed when not needed any longer.
 */
void FMIImpl__releaseFMIImport(void *ptr1, void *ptr2, void *ptr3, const char* fmiVersion)
{
  intptr_t fmiModeVariablesInstance = (intptr_t)MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(ptr1),1));
  intptr_t fmiInstance = (intptr_t)MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(ptr2),1));
  intptr_t fmiContext = (intptr_t)MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(ptr3),1));
  if (strcmp(fmiVersion, "2.0") == 0) {
    fmi2_import_t* fmi = (fmi2_import_t*)fmiInstance;
    free((fmi2_import_variable_list_t*)fmiModeVariablesInstance);
    fmi2_import_free(fmi);
  } else {
    c_add_message(NULL, -1, ErrorType_scripting, ErrorLevel_error, gettext("The FMU version is %s. Unknown/Unsupported FMU version."), &fmiVersion, 1);
    return;
  }
  fmi_import_free_context((fmi_import_context_t*)fmiContext);
}

#endif

#ifdef __cplusplus
}
#endif
