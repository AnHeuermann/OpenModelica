// This file defines templates for transforming Modelica/MetaModelica code to FMU
// code. They are used in the code generator phase of the compiler to write
// target code.
//
// There are one root template intended to be called from the code generator:
// translateModel. These template do not return any
// result but instead write the result to files. All other templates return
// text and are used by the root templates (most of them indirectly).
//
// To future maintainers of this file:
//
// - A line like this
//     # var = "" /*BUFD*/
//   declares a text buffer that you can later append text to. It can also be
//   passed to other templates that in turn can append text to it. In the new
//   version of Susan it should be written like this instead:
//     let &var = buffer ""
//
// - A line like this
//     ..., Text var /*BUFP*/, ...
//   declares that a template takes a text buffer as input parameter. In the
//   new version of Susan it should be written like this instead:
//     ..., Text &var, ...
//
// - A line like this:
//     ..., var /*BUFC*/, ...
//   passes a text buffer to a template. In the new version of Susan it should
//   be written like this instead:
//     ..., &var, ...
//
// - Style guidelines:
//
//   - Try (hard) to limit each row to 80 characters
//
//   - Code for a template should be indented with 2 spaces
//
//     - Exception to this rule is if you have only a single case, then that
//       single case can be written using no indentation
//
//       This single case can be seen as a clarification of the input to the
//       template
//
//   - Code after a case should be indented with 2 spaces if not written on the
//     same line

package CodegenFMU

import interface SimCodeTV;
import interface SimCodeBackendTV;
import CodegenUtil.*;
import CodegenUtilSimulation.*;
import CodegenC.*; //unqualified import, no need the CodegenC is optional when calling a template; or mandatory when the same named template exists in this package (name hiding)
import CodegenCFunctions.*;
import CodegenFMUCommon.*;
import CodegenFMU2.*;


template translateModel(SimCode simCode, String FMUVersion, String FMUType, list<String> sourceFiles)
 "Generates C code and Makefile for compiling a FMU of a
  Modelica model."
::=
match simCode
case sc as SIMCODE(modelInfo=modelInfo as MODELINFO(__)) then
  let guid = getUUIDStr()
  let target  = simulationCodeTarget()
  let fileNamePrefixTmpDir = '<%fileNamePrefix%>.fmutmp/sources/<%fileNamePrefix%>'
  let()= textFile(simulationLiteralsFile(fileNamePrefix, literals), '<%fileNamePrefixTmpDir%>_literals.h')
  let()= textFile(simulationFunctionsHeaderFile(fileNamePrefix, modelInfo.functions, recordDecls, sc.generic_loop_calls), '<%fileNamePrefixTmpDir%>_functions.h')
  let()= textFile(simulationFunctionsFile(fileNamePrefix, modelInfo.functions, generic_loop_calls), '<%fileNamePrefixTmpDir%>_functions.c')
  let()= textFile(externalFunctionIncludes(sc.externalFunctionIncludes), '<%fileNamePrefixTmpDir%>_includes.h')
  let()= textFile(recordsFile(fileNamePrefix, recordDecls, true /*isSimulation*/), '<%fileNamePrefixTmpDir%>_records.c')
  let()= textFile(simulationHeaderFile(simCode), '<%fileNamePrefixTmpDir%>_model.h')

  let _ = generateSimulationFiles(simCode,guid,fileNamePrefixTmpDir,FMUVersion)

  let()= textFile(simulationInitFunction(simCode,guid), '<%fileNamePrefixTmpDir%>_init_fmu.c')
  let()= textFile(fmumodel_identifierFile(simCode,guid,FMUVersion,FMUType), '<%fileNamePrefixTmpDir%>_FMU.c')

  /* Doesn't seem to work properly
  let &fmuModelDescription = buffer ""
  let &fmuModelDescription += redirectToFile('<%fileNamePrefix%>.fmutmp/modelDescription.xml')
  let &fmuModelDescription += fmuModelDescriptionFile(simCode,guid,FMUVersion,FMUType)
  let &fmuModelDescription += closeFile()
  */

  let()= textFile(fmuModelDescriptionFile(simCode, guid, FMUVersion, FMUType, sourceFiles), '<%fileNamePrefix%>.fmutmp/modelDescription.xml')

  // Generate optional <fmiPrefix>_flags.json
  let _ = match sc.fmiSimulationFlags
    case SOME(fmiSimFlags as FMI_SIMULATION_FLAGS(__)) then
      let() = textFile(fmuSimulationFlagsFile(fmiSimFlags), '<%fileNamePrefix%>.fmutmp/resources/<%fileNamePrefix%>_flags.json')
      ""
    else
      ""
    end match

  let()= textFile(fmuDeffile(simCode,FMUVersion), '<%fileNamePrefix%>.fmutmp/sources/<%fileNamePrefix%>.def')
  let()= textFile('# Dummy file so OMDEV Compile.bat works<%\n%>include Makefile<%\n%>', '<%fileNamePrefix%>.fmutmp/sources/<%fileNamePrefix%>.makefile')
  let()= textFile(fmuSourceMakefile(simCode,FMUVersion), '<%fileNamePrefix%>_FMU.makefile')
  "" // Return empty result since result written to files directly
end translateModel;

/* public */ template generateSimulationFiles(SimCode simCode, String guid, String modelNamePrefix, String fmuVersion)
 "Generates code in different C files for the simulation target.
  To make the compilation faster we split the simulation files into several
  used in Compiler/Template/CodegenFMU.tpl"
 ::=
  match simCode
    case simCode as SIMCODE(__) then
     // external objects
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_exo(simCode), '<%modelNamePrefix%>_01exo.c')
     // non-linear systems
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_nls(simCode), '<%modelNamePrefix%>_02nls.c')
     // linear systems
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_lsy(simCode), '<%modelNamePrefix%>_03lsy.c')
     // state set
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_set(simCode), '<%modelNamePrefix%>_04set.c')
     // events: sample, zero crossings, relations
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_evt(simCode), '<%modelNamePrefix%>_05evt.c')
     // initialization
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_inz(simCode), '<%modelNamePrefix%>_06inz.c')
     // delay
     let()= textFileConvertLines(simulationFile_dly(simCode), '<%modelNamePrefix%>_07dly.c')
     // update bound start values, update bound parameters
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_bnd(simCode), '<%modelNamePrefix%>_08bnd.c')
     // algebraic
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_alg(simCode), '<%modelNamePrefix%>_09alg.c')
     // asserts
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_asr(simCode), '<%modelNamePrefix%>_10asr.c')
     // mixed systems
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let &mixheader = buffer ""
     let()= textFileConvertLines(simulationFile_mix(simCode,&mixheader), '<%modelNamePrefix%>_11mix.c')
     let()= textFile(&mixheader, '<%modelNamePrefix%>_11mix.h')
     // jacobians
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_jac(simCode), '<%modelNamePrefix%>_12jac.c')
     let()= textFile(simulationFile_jac_header(simCode), '<%modelNamePrefix%>_12jac.h')
     // optimization
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_opt(simCode), '<%modelNamePrefix%>_13opt.c')
     let()= textFile(simulationFile_opt_header(simCode), '<%modelNamePrefix%>_13opt.h')
     // linearization
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_lnz(simCode), '<%modelNamePrefix%>_14lnz.c')
     // synchronous
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_syn(simCode), '<%modelNamePrefix%>_15syn.c')
     // residuals
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_dae(simCode), '<%modelNamePrefix%>_16dae.c')
     // inline solver
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_inl(simCode), '<%modelNamePrefix%>_17inl.c')
     // main file
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile_spd(simCode), '<%modelNamePrefix%>_18spd.c')
     // update bound start values, update bound parameters
     let()=tmpTickResetIndex(0, 0)
     let()=tmpTickResetIndex(0, 1)
     let()= textFileConvertLines(simulationFile(simCode,guid,fmuVersion), '<%modelNamePrefix%>.c')

     ""
  end match
end generateSimulationFiles;

template fmuModelDescriptionFile(SimCode simCode, String guid, String FMUVersion, String FMUType, list<String> sourceFiles)
 "Generates code for ModelDescription file for FMU target."
::=
match simCode
case SIMCODE(__) then
  let modelDescription =
    if isFMIVersion20(FMUVersion) then CodegenFMU2.fmiModelDescription(simCode, guid, FMUType, sourceFiles)
    else error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')

  <<
  <?xml version="1.0" encoding="UTF-8"?>
  <%modelDescription%>
  >>
end fmuModelDescriptionFile;

template fmuSimulationFlagsFile(FmiSimulationFlags fmiSimulationFlags)
  "Generates <fmiPrefix>_flags.json file for FMUs with custom simulation flags."
 ::=
  match fmiSimulationFlags
  case flags as FMI_SIMULATION_FLAGS(__) then
  let fileContent = (flags.nameValueTuples |> (name, value) =>
      '"<%name%>" : "<%value%>"'
      ;separator=",\n")
    <<
    {
      <%fileContent%>
    }
    >>
end fmuSimulationFlagsFile;

template VendorAnnotations(SimCode simCode)
 "Generates code for VendorAnnotations file for FMU target."
::=
match simCode
case SIMCODE(__) then
  <<
  <VendorAnnotations>
  </VendorAnnotations>
  >>
end VendorAnnotations;

template fmumodel_identifierFile(SimCode simCode, String guid, String FMUVersion, String FMUType)
 "Generates code for ModelDescription file for FMU target."
::=
match simCode
case SIMCODE(__) then

  let fmiHeaders =
    if isFMIVersion20(FMUVersion)
      then
        <<
        #define FMI2_FUNCTION_PREFIX <%modelNamePrefix(simCode)%>_
        #include "fmi2Functions.h"
        #include "fmi-export/fmu2_model_interface.h"
        #include "fmi-export/fmu_read_flags.h"
        >>
    else
      error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')

  let cIncludes =
    if isFMIVersion20(FMUVersion)
      then
        <<
        extern void <%symbolName(modelNamePrefix(simCode),"setupDataStruc")%>(DATA *data, threadData_t *threadData);
        #define fmu2_model_interface_setupDataStruc <%symbolName(modelNamePrefix(simCode),"setupDataStruc")%>
        #include "fmi-export/fmu2_model_interface.c.inc"
        #include "fmi-export/fmu_read_flags.c.inc"
        >>
    else
      error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')

  let fmiIdentifierFunctionsHeaders =
    if isFMIVersion20(FMUVersion) then CodegenFMU2.fmiIdentifierFunctionsHeaders()
    else error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')

  let fmiIdentifierFunctions =
    if isFMIVersion20(FMUVersion) then fmiIdentifierFunctions(simCode)
    else error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')

  <<
  // define class name and unique id
  #define MODEL_IDENTIFIER <%modelNamePrefix(simCode)%>
  #define MODEL_GUID "{<%guid%>}"

  // include fmu header files, typedefs and macros
  #include <stdio.h>
  #include <string.h>
  #include <assert.h>
  #include "openmodelica.h"
  #include "openmodelica_func.h"
  #include "simulation_data.h"
  #include "util/omc_error.h"
  #include "<%fileNamePrefix%>_functions.h"
  #include "simulation/solver/initialization/initialization.h"
  #include "simulation/solver/events.h"
  <%fmiHeaders%>

  #ifdef __cplusplus
  extern "C" {
  #endif

  void setStartValues(ModelInstance *comp);
  void setDefaultStartValues(ModelInstance *comp);
  <%fmiIdentifierFunctionsHeaders%>

  <%ModelDefineData(simCode, modelInfo)%>

  // implementation of the Model Exchange functions
  <%cIncludes%>

  <%setDefaultStartValues(modelInfo)%>
  <%setStartValues(modelInfo)%>
  <%fmiIdentifierFunctions%>

  #ifdef __cplusplus
  }
  #endif

  >>
end fmumodel_identifierFile;

template ModelDefineData(SimCode simCode, ModelInfo modelInfo)
 "Generates global data in simulation file."
::=
match modelInfo
case MODELINFO(varInfo=VARINFO(__), vars=SIMVARS(stateVars = listStates)) then
let numberOfReals = intAdd(intMul(varInfo.numStateVars,2),intAdd(varInfo.numDiscreteReal, intAdd(varInfo.numAlgVars,intAdd(varInfo.numParams,varInfo.numAlgAliasVars))))
let numberOfIntegers = intAdd(varInfo.numIntAlgVars,intAdd(varInfo.numIntParams,varInfo.numIntAliasVars))
let numberOfStrings = intAdd(varInfo.numStringAlgVars,intAdd(varInfo.numStringParamVars,varInfo.numStringAliasVars))
let numberOfBooleans = intAdd(varInfo.numBoolAlgVars,intAdd(varInfo.numBoolParams,varInfo.numBoolAliasVars))
let numberOfRealInputs = varInfo.numRealInputVars
  <<
  // define model size
  #define NUMBER_OF_STATES <%if intEq(varInfo.numStateVars,1) then statesnumwithDummy(listStates) else  varInfo.numStateVars%>
  #define NUMBER_OF_EVENT_INDICATORS <%varInfo.numZeroCrossings%>
  #define NUMBER_OF_REALS <%numberOfReals%>
  #define NUMBER_OF_REAL_INPUTS <%numberOfRealInputs%>
  #define NUMBER_OF_INTEGERS <%numberOfIntegers%>
  #define NUMBER_OF_STRINGS <%numberOfStrings%>
  #define NUMBER_OF_BOOLEANS <%numberOfBooleans%>
  #define NUMBER_OF_EXTERNALFUNCTIONS <%countDynamicExternalFunctions(functions)%>

  // define initial state vector as vector of value references
  #define STATES { <%vars.stateVars |> SIMVAR(__) => if stringEq(crefStr(name),"$dummy") then '' else lookupVR(name, simCode)  ;separator=", "%> }
  #define STATESDERIVATIVES { <%vars.derivativeVars |> SIMVAR(__) => if stringEq(crefStr(name),"der($dummy)") then '' else lookupVR(name, simCode)  ;separator=", "%> }

  <%System.tmpTickReset(0)%>
  <%(functions |> fn => defineExternalFunction(fn) ; separator="\n")%>
  >>
end ModelDefineData;

template dervativeNameCStyle(ComponentRef cr)
 "Generates the name of a derivative in c style, replaces ( with _"
::=
  match cr
  case CREF_QUAL(ident = "$DER") then 'der_<%crefStr(componentRef)%>_'
end dervativeNameCStyle;

template defineExternalFunction(Function fn)
 "Generates external function definitions."
::=
  match fn
    case EXTERNAL_FUNCTION(dynamicLoad=true) then
      let fname = extFunctionName(extName, language)
      <<
      #define $P<%fname%> <%System.tmpTick()%>
      >>
end defineExternalFunction;


template setDefaultStartValues(ModelInfo modelInfo)
 "Generates code in c file for function setStartValues() which will set start values for all variables."
::=
match modelInfo
case MODELINFO(varInfo=VARINFO(numStateVars=numStateVars, numAlgVars= numAlgVars),vars=SIMVARS(__)) then
  <<
  // Set values for all variables that define a start value
  OMC_DISABLE_OPT
  void setDefaultStartValues(ModelInstance *comp) {
    <%vars.stateVars |> var => initValsDefault(var,"realVars") ;separator="\n"%>
    <%vars.derivativeVars |> var => initValsDefault(var,"realVars") ;separator="\n"%>
    <%vars.algVars |> var => initValsDefault(var,"realVars") ;separator="\n"%>
    <%vars.discreteAlgVars |> var => initValsDefault(var, "realVars") ;separator="\n"%>
    <%vars.intAlgVars |> var => initValsDefault(var,"integerVars") ;separator="\n"%>
    <%vars.boolAlgVars |> var => initValsDefault(var,"booleanVars") ;separator="\n"%>
    <%vars.stringAlgVars |> var => initValsDefault(var,"stringVars") ;separator="\n"%>
    <%vars.paramVars |> var => initParamsDefault(var,"realParameter") ;separator="\n"%>
    <%vars.intParamVars |> var => initParamsDefault(var,"integerParameter") ;separator="\n"%>
    <%vars.boolParamVars |> var => initParamsDefault(var,"booleanParameter") ;separator="\n"%>
    <%vars.stringParamVars |> var => initParamsDefault(var,"stringParameter") ;separator="\n"%>
  }
  >>
end setDefaultStartValues;

template setStartValues(ModelInfo modelInfo)
 "Generates code in c file for function setStartValues() which will set start values for all variables."
::=
match modelInfo
case MODELINFO(varInfo=VARINFO(numStateVars=numStateVars, numAlgVars= numAlgVars),vars=SIMVARS(__)) then
  <<
  // Set values for all variables that define a start value
  OMC_DISABLE_OPT
  void setStartValues(ModelInstance *comp) {
    <%vars.stateVars |> var => initVals(var,"realVars") ;separator="\n"%>
    <%vars.derivativeVars |> var => initVals(var,"realVars") ;separator="\n"%>
    <%vars.algVars |> var => initVals(var,"realVars") ;separator="\n"%>
    <%vars.discreteAlgVars |> var => initVals(var, "realVars") ;separator="\n"%>
    <%vars.intAlgVars |> var => initVals(var,"integerVars") ;separator="\n"%>
    <%vars.boolAlgVars |> var => initVals(var,"booleanVars") ;separator="\n"%>
    <%vars.stringAlgVars |> var => initVals(var,"stringVars") ;separator="\n"%>
    <%vars.paramVars |> var => initParams(var,"realParameter") ;separator="\n"%>
    <%vars.intParamVars |> var => initParams(var,"integerParameter") ;separator="\n"%>
    <%vars.boolParamVars |> var => initParams(var,"booleanParameter") ;separator="\n"%>
    <%vars.stringParamVars |> var => initParams(var,"stringParameter") ;separator="\n"%>
  }

  >>
end setStartValues;

template initializeFunction(list<SimEqSystem> allEquations)
  "Generates initialize function for c file."
::=
  let &sub = buffer ""
  let &varDecls = buffer "" /*BUFD*/
  let eqPart = ""/* (allEquations |> eq as SES_SIMPLE_ASSIGN(__) =>
      equation_(eq, contextOther, &varDecls)
    ;separator="\n") */
  <<
  // Used to set the first time event, if any.
  void initialize(ModelInstance* comp, fmiEventInfo* eventInfo) {

    <%varDecls%>

    <%eqPart%>
    <%allEquations |> SES_SIMPLE_ASSIGN(__) =>
      'if (sim_verbose) { printf("Setting variable start value: %s(start=%f)\n", "<%escapeModelicaStringToCString(crefStrNoUnderscore(cref))%>", <%cref(cref, &sub)%>); }'
    ;separator="\n"%>

  }
  >>
end initializeFunction;


template initVals(SimVar var, String arrayName) ::=
  match var
    case SIMVAR(__) then
    if stringEq(crefStr(name),"$dummy") then
      ''
    else if stringEq(crefStr(name),"der($dummy)") then
      ''
    else
    let str = 'comp->fmuData->modelData-><%arrayName%>Data[<%index%>].attribute.start'
      '<%str%> =  comp->fmuData->localData[0]-><%arrayName%>[<%index%>];'
end initVals;

template initParams(SimVar var, String arrayName) ::=
  match var
    case SIMVAR(__) then
    let str = 'comp->fmuData->modelData-><%arrayName%>Data[<%index%>].attribute.start'
      '<%str%> = comp->fmuData->simulationInfo-><%arrayName%>[<%index%>];'
end initParams;

template initValsDefault(SimVar var, String arrayName) ::=
  match var
    case SIMVAR(index=index, type_=type_) then
    let str = 'comp->fmuData->modelData-><%arrayName%>Data[<%index%>].attribute.start'
    '<%str%> = <%initValDefault(var)%>;'
end initValsDefault;

template initParamsDefault(SimVar var, String arrayName) ::=
  match var
    case SIMVAR(__) then
    let str = 'comp->fmuData->modelData-><%arrayName%>Data[<%index%>].attribute.start'
    match initialValue
      case SOME(v as SCONST(__)) then
      '<%str%> = mmc_mk_scon_persist(<%initVal(v)%>); /* TODO: these are not freed currently, see #6161 */'
      else
      '<%str%> = <%initValDefault(var)%>;'
end initParamsDefault;

template initValDefault(SimVar var) ::=
  match var
    case var as SIMVAR(__) then
    match var.initialValue
      case SOME(v as ICONST(__))
      case SOME(v as RCONST(__))
      case SOME(v as SCONST(__))
      case SOME(v as BCONST(__))
      case SOME(v as ENUM_LITERAL(__)) then initVal(v)
      else
        match var.type_
          case T_INTEGER(__)
          case T_REAL(__)
          case T_ENUMERATION(__)
          case T_BOOL(__) then '0'
          case T_STRING(__) then 'mmc_mk_scon("")'
          else error(sourceInfo(), 'Unknown type for initValDefault: <%unparseType(var.type_)%>')
end initValDefault;

template initVal(Exp initialValue)
::=
  match initialValue
  case ICONST(__) then integer
  case RCONST(__) then real
  case SCONST(__) then '"<%Util.escapeModelicaStringToXmlString(string)%>"'
  case BCONST(__) then if bool then "1" else "0"
  case ENUM_LITERAL(__) then '<%index%>'
  else error(sourceInfo(), 'initial value of unknown type: <%printExpStr(initialValue)%>')
end initVal;

template getPlatformString2(String modelNamePrefix, String platform, String fileNamePrefix, String fmuTargetName, String dirExtra, String libsPos1, String libsPos2, String omhome, String FMUVersion)
 "returns compilation commands for the platform. "
::=
let fmudirname = '<%fileNamePrefix%>.fmutmp'
match platform
  case "win32"
  case "win64" then
  <<
  <%fileNamePrefix%>_FMU: nozip
  <%\t%>cd .. && rm -f ../<%fileNamePrefix%>.fmu && zip -r ../<%fmuTargetName%>.fmu *
  nozip: <%fileNamePrefix%>_functions.h <%fileNamePrefix%>_literals.h $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)
  <%\t%>$(CXX) -shared -I. -o <%modelNamePrefix%>$(DLLEXT) $(RUNTIMEFILES) $(FMISUNDIALSFILES) $(OFILES) $(CPPFLAGS) <%dirExtra%> <%libsPos1%> <%libsPos2%> $(CFLAGS) $(LDFLAGS) -llis -Wl,--kill-at
  <%\t%>mkdir.exe -p ../binaries/<%platform%>
  <%\t%>dlltool -d <%fileNamePrefix%>.def --dllname <%fileNamePrefix%>$(DLLEXT) --output-lib <%fileNamePrefix%>.lib --kill-at
  <%\t%>cp <%fileNamePrefix%>$(DLLEXT) <%fileNamePrefix%>.lib <%fileNamePrefix%>_FMU.libs ../binaries/<%platform%>/
  <%\t%>rm -f *.o <%fileNamePrefix%>$(DLLEXT) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)
  <%\t%>cd .. && rm -f ../<%fileNamePrefix%>.fmu && zip -r ../<%fmuTargetName%>.fmu *

  >>
  else
  <<
  <%fileNamePrefix%>_FMU: nozip
  <%\t%>cd .. && rm -f ../<%fileNamePrefix%>.fmu && zip -r ../<%fmuTargetName%>.fmu *
  nozip: <%fileNamePrefix%>_functions.h <%fileNamePrefix%>_literals.h $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)
  <%\t%>mkdir -p ../binaries/$(FMIPLATFORM)
  ifeq (@LIBTYPE_DYNAMIC@,1)
  <%\t%>$(LD) -o <%modelNamePrefix%>$(DLLEXT) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES) <%dirExtra%> <%libsPos1%> <%libsPos2%> @BDYNAMIC@ $(LDFLAGS)
  <%\t%>cp <%fileNamePrefix%>$(DLLEXT) <%fileNamePrefix%>_FMU.libs ../binaries/$(FMIPLATFORM)/
  endif
  <%if intLt(Flags.getConfigEnum(Flags.FMI_FILTER), 4) then
  '<%\t%>head -n20 Makefile > ../resources/$(FMIPLATFORM).summary'
   %>
  ifeq (@LIBTYPE_STATIC@,1)
  <%\t%>rm -f <%modelNamePrefix%>.a
  <%\t%>$(AR) -rsu <%modelNamePrefix%>.a $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)
  <%\t%>cp <%fileNamePrefix%>.a <%fileNamePrefix%>_FMU.libs ../binaries/$(FMIPLATFORM)/
  endif
  <% if not Flags.isSet(Flags.GEN_DEBUG_SYMBOLS) then "\t$(MAKE) distclean" %>
  distclean: clean
  <%\t%>rm -f Makefile config.status config.log
  clean:
  <%\t%>rm -f <%fileNamePrefix%>.def <%fileNamePrefix%>.o <%fileNamePrefix%>.a <%fileNamePrefix%>$(DLLEXT) $(MAINOBJ) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)
  >>
end getPlatformString2;

template settingsfile(SimCode simCode)
"Generates content of omc_simulation_settings.h"
::=
  match simCode
  case SIMCODE(modelInfo=MODELINFO(varInfo=varInfo as VARINFO(__)), delayedExps=DELAYED_EXPRESSIONS(maxDelayedIndex=maxDelayedIndex), fmiSimulationFlags=fmiSimulationFlags) then
  <<
  #if !defined(OMC_SIM_SETTINGS_CMDLINE)
  #define OMC_SIM_SETTINGS_CMDLINE
  #define OMC_NUM_LINEAR_SYSTEMS <%varInfo.numLinearSystems%>
  #define OMC_NUM_NONLINEAR_SYSTEMS <%varInfo.numNonLinearSystems%>
  #define OMC_NUM_MIXED_SYSTEMS <%varInfo.numMixedSystems%>
  #define OMC_NDELAY_EXPRESSIONS <%maxDelayedIndex%>
  #define OMC_NVAR_STRING <%varInfo.numStringAlgVars%>
  <% if Flags.isSet(Flags.FMU_EXPERIMENTAL) then '#define FMU_EXPERIMENTAL 1'%>
  #define OMC_MODEL_PREFIX "<%modelNamePrefix(simCode)%>"
  #define OMC_MINIMAL_RUNTIME 1
  #define OMC_FMI_RUNTIME 1
  #endif
 >>
end settingsfile;

template fmuMakefile(String target, SimCode simCode, String FMUVersion, list<String> sourceFiles, list<String> runtimeObjectFiles, list<String> dgesvObjectFiles, list<String> cminpackObjectFiles, list <String> sundialsObjectFiles)
 "Generates the contents of the makefile for the simulation case. Copy libexpat & correct linux fmu"
::=
  let common =
    match simCode
    case SIMCODE(modelInfo=MODELINFO(__), makefileParams=MAKEFILE_PARAMS(__), simulationSettingsOpt = sopt) then
    <<
    CFILES = <%sourceFiles ; separator=" \\\n         "%>
    OFILES=$(CFILES:.c=.o)

    RUNTIMEDIR=.
    ifneq ($(NEED_DGESV),)
    DGESV_OBJS = <%dgesvObjectFiles ; separator = " "%>
    endif
    ifneq ($(NEED_CMINPACK),)
    CMINPACK_OBJS=<%cminpackObjectFiles ; separator = " "%>
    endif
    ifneq ($(NEED_RUNTIME),)
    RUNTIMEFILES=<%runtimeObjectFiles ; separator = " "%> $(DGESV_OBJS) $(CMINPACK_OBJS)
    endif
    ifneq ($(NEED_SUNDIALS),)
    FMISUNDIALSFILES=<%sundialsObjectFiles ; separator = " "%>
    LDFLAGS+=-Wl,-Bstatic -lsundials_cvode -lsundials_nvecserial -Wl,-Bdynamic
    endif
    >>

  match getGeneralTarget(target)
  case "msvc" then
    match simCode
    case SIMCODE(modelInfo=MODELINFO(__), makefileParams=MAKEFILE_PARAMS(__), simulationSettingsOpt = sopt) then
      let fmiIncludeFlag =
        if isFMIVersion20(FMUVersion)
          then '/I"<%makefileParams.omhome%>/include/omc/c/fmi2"'
        else
          error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
      let dirExtra = if modelInfo.directory then '/LIBPATH:"<%modelInfo.directory%>"' //else ""
      let libsStr = (makefileParams.libs |> lib => lib ;separator=" ")
      let libsPos1 = if not dirExtra then libsStr //else ""
      let libsPos2 = if dirExtra then libsStr // else ""
      let fmudirname = '<%fileNamePrefix%>.fmutmp'
      let compilecmds = getPlatformString2(modelNamePrefix(simCode), makefileParams.platform, fileNamePrefix, fmuTargetName, dirExtra, libsPos1, libsPos2, makefileParams.omhome, FMUVersion)
      let mkdir = match makefileParams.platform case "win32" case "win64" then '"mkdir.exe"' else 'mkdir'
      <<
      # Makefile generated by OpenModelica

      # Simulations use -O3 by default
      SIM_OR_DYNLOAD_OPT_LEVEL=
      MODELICAUSERCFLAGS=
      CXX=cl
      EXEEXT=.exe
      DLLEXT=.dll
      FMUEXT=.fmu
      PLATWIN32 = win32

      # /Od - Optimization disabled
      # /EHa enable C++ EH (w/ SEH exceptions)
      # /fp:except - consider floating-point exceptions when generating code
      # /arch:SSE2 - enable use of instructions available with SSE2 enabled CPUs
      # /I - Include Directories
      # /DNOMINMAX - Define NOMINMAX (does what it says)
      # /TP - Use C++ Compiler
      CFLAGS=/MP /Od /ZI /EHa /fp:except /I"<%makefileParams.omhome%>/include/omc/c" /I"<%makefileParams.omhome%>/include/omc/msvc/" <%fmiIncludeFlag%> /I. /DNOMINMAX /TP /DNO_INTERACTIVE_DEPENDENCY  <% if Flags.isSet(Flags.FMU_EXPERIMENTAL) then '/DFMU_EXPERIMENTAL'%>

      # /ZI enable Edit and Continue debug info
      CDFLAGS=/ZI

      # /MD - link with MSVCRT.LIB
      # /link - [linker options and libraries]
      # /LIBPATH: - Directories where libs can be found
      LDFLAGS=/MD /link /dll /debug /pdb:"<%fileNamePrefix%>.pdb" /LIBPATH:"<%makefileParams.omhome%>/lib/<%Autoconf.triple%>/omc/msvc/" /LIBPATH:"<%makefileParams.omhome%>/lib/<%Autoconf.triple%>/omc/msvc/release/" <%dirExtra%> <%libsPos1%> <%libsPos2%> f2c.lib initialization.lib libexpat.lib math-support.lib meta.lib results.lib simulation.lib solver.lib sundials_kinsol.lib sundials_nvecserial.lib util.lib lapack_win32_MT.lib lis.lib  omcgc.lib user32.lib pthreadVC2.lib wsock32.lib cminpack.lib umfpack.lib amd.lib

      # /MDd link with MSVCRTD.LIB debug lib
      # lib names should not be appended with a d just switch to lib/omc/msvc/debug


      <%common%>

      <%fileNamePrefix%>$(FMUEXT): <%fileNamePrefix%>$(DLLEXT) modelDescription.xml
          if not exist <%fmudirname%>\binaries\$(PLATWIN32) <%mkdir%> <%fmudirname%>\binaries\$(PLATWIN32)
          if not exist <%fmudirname%>\sources <%mkdir%> <%fmudirname%>\sources

          copy <%fileNamePrefix%>.dll <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%fileNamePrefix%>.lib <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%fileNamePrefix%>.pdb <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%fileNamePrefix%>.c <%fmudirname%>\sources\<%fileNamePrefix%>.c
          copy <%fileNamePrefix%>_model.h <%fmudirname%>\sources\<%fileNamePrefix%>_model.h
          copy <%fileNamePrefix%>_FMU.c <%fmudirname%>\sources\<%fileNamePrefix%>_FMU.c
          copy <%fileNamePrefix%>_info.c <%fmudirname%>\sources\<%fileNamePrefix%>_info.c
          copy <%fileNamePrefix%>_init_fmu.c <%fmudirname%>\sources\<%fileNamePrefix%>_init_fmu.c
          copy <%fileNamePrefix%>_functions.c <%fmudirname%>\sources\<%fileNamePrefix%>_functions.c
          copy <%fileNamePrefix%>_functions.h <%fmudirname%>\sources\<%fileNamePrefix%>_functions.h
          copy <%fileNamePrefix%>_records.c <%fmudirname%>\sources\<%fileNamePrefix%>_records.c
          copy modelDescription.xml <%fmudirname%>\modelDescription.xml
          copy <%stringReplace(makefileParams.omhome,"/","\\")%>\bin\SUNDIALS_CVODE.DLL <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%stringReplace(makefileParams.omhome,"/","\\")%>\bin\SUNDIALS_KINSOL.DLL <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%stringReplace(makefileParams.omhome,"/","\\")%>\bin\SUNDIALS_NVECSERIAL.DLL <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%stringReplace(makefileParams.omhome,"/","\\")%>\bin\LAPACK_WIN32_MT.DLL <%fmudirname%>\binaries\$(PLATWIN32)
          copy <%stringReplace(makefileParams.omhome,"/","\\")%>\bin\pthreadVC2.dll <%fmudirname%>\binaries\$(PLATWIN32)
          cd <%fmudirname%>
          "zip.exe" -r ../<%fmuTargetName%>.fmu *
          cd ..
          rm -rf <%fmudirname%>

      <%fileNamePrefix%>$(DLLEXT): $(MAINOBJ) $(CFILES)
          $(CXX) /Fe<%fileNamePrefix%>$(DLLEXT) <%fileNamePrefix%>_FMU.c <%fileNamePrefix%>_FMU.c $(CFILES) $(CFLAGS) $(LDFLAGS)
      >>
    end match
  case "gcc" then
    match simCode
    case SIMCODE(modelInfo=MODELINFO(varInfo=varInfo as VARINFO(__)), delayedExps=DELAYED_EXPRESSIONS(maxDelayedIndex=maxDelayedIndex), makefileParams=MAKEFILE_PARAMS(__), simulationSettingsOpt = sopt, fmiSimulationFlags = fmiSimulationFlags) then
      let dirExtra = if modelInfo.directory then '-L"<%modelInfo.directory%>"' //else ""
      let libsStr = (makefileParams.libs |> lib => lib ;separator=" ")
      let libsPos1 = if not dirExtra then libsStr //else ""
      let libsPos2 = if dirExtra then libsStr // else ""
      let compilecmds = getPlatformString2(modelNamePrefix(simCode), makefileParams.platform, fileNamePrefix, fmuTargetName, dirExtra, libsPos1, libsPos2, makefileParams.omhome, FMUVersion)
      let platformstr = makefileParams.platform
      let thirdPartyInclude = match fmiSimulationFlags case SOME(__) then "-Isundials/ -I/util" else ""
      let fmiAdditionalCppFlags =
        if isFMIVersion20(FMUVersion) then "-DFMI2_OVERRIDE_FUNCTION_PREFIX"
        else ""
      <<
      # Makefile generated by OpenModelica
      CC=@CC@
      AR=@AR@
      CFLAGS=@CFLAGS@
      LD=$(CC) -shared
      # define OMC_LDFLAGS_LINK_TYPE env variable to override this
      OMC_LDFLAGS_LINK_TYPE=static
      LDFLAGS=@LDFLAGS@ @LIBS@
      DLLEXT=@DLLEXT@
      NEED_RUNTIME=@NEED_RUNTIME@
      NEED_DGESV=@NEED_DGESV@
      NEED_CMINPACK=@NEED_CMINPACK@
      NEED_SUNDIALS=@NEED_SUNDIALS@
      FMIPLATFORM=@FMIPLATFORM@
      # Note: Simulation of the fmu with dymola does not work with -finline-small-functions (enabled by most optimization levels)
      CPPFLAGS=@CPPFLAGS@
      override CPPFLAGS += <%fmiAdditionalCppFlags%>

      override CPPFLAGS += <%makefileParams.includes ; separator=" "%>

      <%common%>

      PHONY: <%fileNamePrefix%>_FMU
      <%compilecmds%>
      >>
    end match
  else
    error(sourceInfo(), 'target <%target%> is not handled!')
end fmuMakefile;


template fmuSourceMakefile(SimCode simCode, String FMUVersion)
 "Generates the contents of the makefile for the simulation case. Copy libexpat & correct linux fmu"
::=
  match simCode
  case SIMCODE(modelInfo=modelInfo as MODELINFO(__), makefileParams=MAKEFILE_PARAMS(__), simulationSettingsOpt = sopt) then
  let includedir = '<%fileNamePrefix%>.fmutmp/sources/'
  let mkdir = match makefileParams.platform case "win32" case "win64" then '"mkdir.exe"' else 'mkdir'
  <<
  # FIXME: before you push into master...
  RUNTIMEDIR=<%makefileParams.omhome%>/include/omc/c/
  #COPY_RUNTIMEFILES=$(FMI_ME_OBJS:%= && (OMCFILE=% && cp $(RUNTIMEDIR)/$$OMCFILE.c $$OMCFILE.c))

  fmu:
  <%\t%>rm -f <%fileNamePrefix%>.fmutmp/sources/<%fileNamePrefix%>_init.xml<%/*Already translated to .c*/%>
  <%\t%>cp -a "<%makefileParams.omhome%>/share/omc/runtime/c/fmi/buildproject/"* <%fileNamePrefix%>.fmutmp/sources
  <%\t%>cp -a <%fileNamePrefix%>_FMU.libs <%fileNamePrefix%>.fmutmp/sources/
  <%if boolNot(boolOr(stringEq(makefileParams.platform, "win32"),stringEq(makefileParams.platform, "win64"))) then
     match  Config.simCodeTarget()
     case "omsicpp" then
     <<
     <%\t%>chmod +x <%dotPath(modelInfo.name)%>.sh
     >>
     end match
  %>
  <%\n%>
  >>
end fmuSourceMakefile;

template fmuDeffile(SimCode simCode, String FMUVersion)
  "Generates the def file of the fmu."
::=
  if isFMIVersion20(FMUVersion) then CodegenFMU2.fmuDeffile(simCode)
  else error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end fmuDeffile;

template importFMUModelica(FmiImport fmi, String name)
 "Generates the Modelica code depending on the FMU type."
::=
match fmi
case FMIIMPORT(__) then
  match fmiInfo
    case (INFO(fmiVersion = "2.0", fmiType = 1)) then
      CodegenFMU2.importFMU2ModelExchange(fmi, name)
    else
      error(sourceInfo(), 'Unknown/unsupported FMI version')
end importFMUModelica;

template dumpFMITypeDefinitions(list<TypeDefinitions> fmiTypeDefinitionsList)
 "Generates the Type Definitions code."
::=
  <<
  <%fmiTypeDefinitionsList |> fmiTypeDefinition => dumpFMITypeDefinition(fmiTypeDefinition) ;separator="\n"%>
  >>
end dumpFMITypeDefinitions;

template dumpFMITypeDefinition(TypeDefinitions fmiTypeDefinition)
 "Generates the Type code."
::=
match fmiTypeDefinition
case ENUMERATIONTYPE(__) then
  <<
  type <%name%> = enumeration(
    <%dumpFMITypeDefinitionsItems(items)%>);
  >>
end dumpFMITypeDefinition;

template dumpFMITypeDefinitionsItems(list<EnumerationItem> items)
 "Generates the Enumeration Type items code."
::=
  <<
  <%items |> item => dumpFMITypeDefinitionsItem(item) ;separator=",\n"%>
  >>
end dumpFMITypeDefinitionsItems;

template dumpFMITypeDefinitionsItem(EnumerationItem item)
 "Generates the Enumeration Type item name."
::=
match item
case ENUMERATIONITEM(__) then
  <<
  <%name%>
  >>
end dumpFMITypeDefinitionsItem;

template dumpFMITypeDefinitionsMappingFunctions(list<TypeDefinitions> fmiTypeDefinitionsList)
 "Generates the mapping functions for all enumeration types."
::=
  <<
  <%fmiTypeDefinitionsList |> fmiTypeDefinition => dumpFMITypeDefinitionMappingFunction(fmiTypeDefinition) ;separator="\n"%>
  >>
end dumpFMITypeDefinitionsMappingFunctions;

template dumpFMITypeDefinitionMappingFunction(TypeDefinitions fmiTypeDefinition)
 "Generates the mapping function from integer to enumeration type."
::=
match fmiTypeDefinition
case ENUMERATIONTYPE(__) then
  <<
  function map_<%name%>_from_integer
    input Integer i;
    output <%name%> outType;
  algorithm
    <%items |> item hasindex i0 fromindex 1 => dumpFMITypeDefinitionMappingFunctionItems(item, name, i0) ;separator="\n"%>
    <%if intGt(listLength(items), 1) then "end if;"%>
  end map_<%name%>_from_integer;
  >>
end dumpFMITypeDefinitionMappingFunction;

template dumpFMITypeDefinitionMappingFunctionItems(EnumerationItem item, String typeName, Integer i)
 "Dumps the mapping function conditions. This is closely related to dumpFMITypeDefinitionMappingFunction."
::=
match item
case ENUMERATIONITEM(__) then
  if intEq(i, 1) then
  <<
  if i == <%i%> then outType := <%typeName%>.<%name%>;
  >>
  else
  <<
  elseif i == <%i%> then outType := <%typeName%>.<%name%>;
  >>
end dumpFMITypeDefinitionMappingFunctionItems;

template dumpFMITypeDefinitionsArrayMappingFunctions(list<TypeDefinitions> fmiTypeDefinitionsList)
 "Generates the array mapping functions for all enumeration types."
::=
  <<
  <%fmiTypeDefinitionsList |> fmiTypeDefinition => dumpFMITypeDefinitionsArrayMappingFunction(fmiTypeDefinition) ;separator="\n"%>
  >>
end dumpFMITypeDefinitionsArrayMappingFunctions;

template dumpFMITypeDefinitionsArrayMappingFunction(TypeDefinitions fmiTypeDefinition)
 "Generates the mapping function from integer to enumeration type."
::=
match fmiTypeDefinition
case ENUMERATIONTYPE(__) then
  <<
  function map_<%name%>_from_integers
    input Integer fromInt[size(fromInt, 1)];
    output <%name%> toEnum[size(fromInt, 1)];
  protected
    Integer n = size(fromInt, 1);
  algorithm
    for i in 1:n loop
      toEnum[i] := map_<%name%>_from_integer(fromInt[i]);
    end for;
  end map_<%name%>_from_integers;
  >>
end dumpFMITypeDefinitionsArrayMappingFunction;

template dumpFMIModelVariablesList(String FMUVersion, list<ModelVariables> fmiModelVariablesList, list<TypeDefinitions> fmiTypeDefinitionsList, Boolean generateInputConnectors, Boolean generateOutputConnectors)
 "Generates the Model Variables code."
::=
  <<
  <%fmiModelVariablesList |> fmiModelVariable => dumpFMIModelVariable(FMUVersion, fmiModelVariable, fmiTypeDefinitionsList, generateInputConnectors, generateOutputConnectors) ;separator="\n"%>
  >>
end dumpFMIModelVariablesList;

template dumpFMIModelVariable(String FMUVersion, ModelVariables fmiModelVariable, list<TypeDefinitions> fmiTypeDefinitionsList, Boolean generateInputConnectors, Boolean generateOutputConnectors)
::=
  if isFMIVersion20(FMUVersion) then
    match fmiModelVariable
      case REALVARIABLE(__) then
        <<
        <%dumpFMIModelVariableVariability(variability)%><%dumpFMIModelVariableCausalityAndBaseType(causality, baseType, generateInputConnectors, generateOutputConnectors)%> <%name%><%dumpFMIRealModelVariableStartValue(FMUVersion, causality, hasStartValue, startValue, isFixed)%><%dumpFMIModelVariableDescription(description)%><%dumpFMIModelVariablePlacementAnnotation(x1Placement, x2Placement, y1Placement, y2Placement, generateInputConnectors, generateOutputConnectors, causality)%>;
        >>
      case INTEGERVARIABLE(__) then
        <<
        <%dumpFMIModelVariableVariability(variability)%><%dumpFMIModelVariableCausalityAndBaseType(causality, baseType, generateInputConnectors, generateOutputConnectors)%> <%name%><%dumpFMIIntegerModelVariableStartValue(FMUVersion, causality, hasStartValue, startValue, isFixed)%><%dumpFMIModelVariableDescription(description)%><%dumpFMIModelVariablePlacementAnnotation(x1Placement, x2Placement, y1Placement, y2Placement, generateInputConnectors, generateOutputConnectors, causality)%>;
        >>
      case BOOLEANVARIABLE(__) then
        <<
        <%dumpFMIModelVariableVariability(variability)%><%dumpFMIModelVariableCausalityAndBaseType(causality, baseType, generateInputConnectors, generateOutputConnectors)%> <%name%><%dumpFMIBooleanModelVariableStartValue(FMUVersion, causality, hasStartValue, startValue, isFixed)%><%dumpFMIModelVariableDescription(description)%><%dumpFMIModelVariablePlacementAnnotation(x1Placement, x2Placement, y1Placement, y2Placement, generateInputConnectors, generateOutputConnectors, causality)%>;
        >>
      case STRINGVARIABLE(__) then
        <<
        <%dumpFMIModelVariableVariability(variability)%><%dumpFMIModelVariableCausalityAndBaseType(causality, baseType, generateInputConnectors, generateOutputConnectors)%> <%name%><%dumpFMIStringModelVariableStartValue(FMUVersion, causality, hasStartValue, startValue, isFixed)%><%dumpFMIModelVariableDescription(description)%><%dumpFMIModelVariablePlacementAnnotation(x1Placement, x2Placement, y1Placement, y2Placement, generateInputConnectors, generateOutputConnectors, causality)%>;
        >>
      case ENUMERATIONVARIABLE(__) then
        <<
        <%dumpFMIModelVariableVariability(variability)%><%dumpFMIModelVariableCausalityAndBaseType(causality, baseType, generateInputConnectors, generateOutputConnectors)%> <%name%><%dumpFMIEnumerationModelVariableStartValue(fmiTypeDefinitionsList, baseType, hasStartValue, startValue, isFixed)%><%dumpFMIModelVariableDescription(description)%><%dumpFMIModelVariablePlacementAnnotation(x1Placement, x2Placement, y1Placement, y2Placement, generateInputConnectors, generateOutputConnectors, causality)%>;
        >>
    end match
  else error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end dumpFMIModelVariable;

template dumpFMIModelVariableVariability(String variability)
::=
  <<
  <%if stringEq(variability, "") then "" else variability+" "%>
  >>
end dumpFMIModelVariableVariability;

template dumpFMIModelVariableCausalityAndBaseType(String causality, String baseType, Boolean generateInputConnectors, Boolean generateOutputConnectors)
::=
  if boolAnd(generateInputConnectors, boolAnd(stringEq(causality, "input"),stringEq(baseType, "Real"))) then "Modelica.Blocks.Interfaces.RealInput"
  else if boolAnd(generateInputConnectors, boolAnd(stringEq(causality, "input"),stringEq(baseType, "Integer"))) then "Modelica.Blocks.Interfaces.IntegerInput"
  else if boolAnd(generateInputConnectors, boolAnd(stringEq(causality, "input"),stringEq(baseType, "Boolean"))) then "Modelica.Blocks.Interfaces.BooleanInput"
  else if boolAnd(generateOutputConnectors, boolAnd(stringEq(causality, "output"),stringEq(baseType, "Real"))) then "Modelica.Blocks.Interfaces.RealOutput"
  else if boolAnd(generateOutputConnectors, boolAnd(stringEq(causality, "output"),stringEq(baseType, "Integer"))) then "Modelica.Blocks.Interfaces.IntegerOutput"
  else if boolAnd(generateOutputConnectors, boolAnd(stringEq(causality, "output"),stringEq(baseType, "Boolean"))) then "Modelica.Blocks.Interfaces.BooleanOutput"
  else if stringEq(causality, "") then baseType else causality+" "+baseType
end dumpFMIModelVariableCausalityAndBaseType;

template dumpFMIModelVariableCausality(String causality)
::=
  <<
  <%if stringEq(causality, "") then "" else causality+" "%>
  >>
end dumpFMIModelVariableCausality;

// TODO AHeu: Unify with dumpFMIRealModelVariableStartValue, dumpFMIBooleanModelVariableStartValue
template dumpFMIRealModelVariableStartValue(String FMUVersion, String variabilityCausality, Boolean hasStartValue, Real startValue, Boolean isFixed)
::=
  if isFMIVersion20(FMUVersion) then
    match variabilityCausality
      case "parameter" then
        if boolAnd(hasStartValue,isFixed) then " = "+startValue
        else if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
        else if boolAnd(boolNot(hasStartValue),isFixed) then "(fixed=true)"
        else if boolAnd(boolNot(hasStartValue),boolNot(isFixed)) then "(fixed=false)"
      else
        if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
  else
    error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end dumpFMIRealModelVariableStartValue;

template dumpFMIIntegerModelVariableStartValue(String FMUVersion, String variabilityCausality, Boolean hasStartValue, Integer startValue, Boolean isFixed)
::=
  if isFMIVersion20(FMUVersion) then
    match variabilityCausality
      case "parameter" then
        if boolAnd(hasStartValue,isFixed) then " = "+startValue
        else if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
        else if boolAnd(boolNot(hasStartValue),isFixed) then "(fixed=true)"
        else if boolAnd(boolNot(hasStartValue),boolNot(isFixed)) then "(fixed=false)"
      else
        if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
  else
    error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end dumpFMIIntegerModelVariableStartValue;

template dumpFMIBooleanModelVariableStartValue(String FMUVersion, String variabilityCausality, Boolean hasStartValue, Boolean startValue, Boolean isFixed)
::=
  if isFMIVersion20(FMUVersion) then
    match variabilityCausality
      case "parameter" then
        if boolAnd(hasStartValue,isFixed) then " = "+startValue
        else if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
        else if boolAnd(boolNot(hasStartValue),isFixed) then "(fixed=true)"
        else if boolAnd(boolNot(hasStartValue),boolNot(isFixed)) then "(fixed=false)"
      else
        if boolAnd(hasStartValue,boolNot(isFixed)) then "(start="+startValue+",fixed=false)"
  else
    error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end dumpFMIBooleanModelVariableStartValue;

template dumpFMIStringModelVariableStartValue(String FMUVersion, String variabilityCausality, Boolean hasStartValue, String startValue, Boolean isFixed)
::=
  if isFMIVersion20(FMUVersion) then
    match variabilityCausality
      case "parameter" then
        if boolAnd(hasStartValue,isFixed) then " = \""+startValue+"\""
        else if boolAnd(hasStartValue,boolNot(isFixed)) then "(start=\""+startValue+"\",fixed=false)"
        else if boolAnd(boolNot(hasStartValue),isFixed) then "(fixed=true)"
        else if boolAnd(boolNot(hasStartValue),boolNot(isFixed)) then "(fixed=false)"
      else
        if boolAnd(hasStartValue,boolNot(isFixed)) then "(start=\""+startValue+"\",fixed=false)"
  else
    error(sourceInfo(), 'Unknown/unsupported FMI version: <%FMUVersion%>')
end dumpFMIStringModelVariableStartValue;

template dumpFMIEnumerationModelVariableStartValue(list<TypeDefinitions> fmiTypeDefinitionsList, String baseType, Boolean hasStartValue, Integer startValue, Boolean isFixed)
::=
  <<
  <%if hasStartValue then " = map_" + getEnumerationTypeFromTypes(fmiTypeDefinitionsList, baseType) + "_from_integer(" + startValue + ")"%>
  >>
end dumpFMIEnumerationModelVariableStartValue;

template dumpFMIModelVariableDescription(String description)
::=
  <<
  <%if stringEq(description, "") then "" else " \""+description+"\""%>
  >>
end dumpFMIModelVariableDescription;

template dumpFMIModelVariablePlacementAnnotation(Integer x1Placement, Integer x2Placement, Integer y1Placement, Integer y2Placement, Boolean generateInputConnectors, Boolean generateOutputConnectors, String causality)
::=
  if boolAnd(generateInputConnectors, stringEq(causality, "input")) then " annotation(Placement(transformation(extent={{"+x1Placement+","+y1Placement+"},{"+x2Placement+","+y2Placement+"}})))"
  else if boolAnd(generateOutputConnectors, stringEq(causality, "output")) then " annotation(Placement(transformation(extent={{"+x1Placement+","+y1Placement+"},{"+x2Placement+","+y2Placement+"}})))"
end dumpFMIModelVariablePlacementAnnotation;

template dumpVariables(list<ModelVariables> fmiModelVariablesList, String type, String variabilityCausality, Boolean dependent, Integer what, String fmiVersion)
::=
  <<
  <%fmiModelVariablesList |> fmiModelVariable => dumpVariable(fmiModelVariable, type, variabilityCausality, dependent, what, fmiVersion) ;separator=", "%>
  >>
end dumpVariables;

template dumpVariable(ModelVariables fmiModelVariable, String type, String variabilityCausality, Boolean dependent, Integer what, String fmiVersion)
::=
  if isFMIVersion20(fmiVersion) then
    dumpFMI2Variable(fmiModelVariable, type, variabilityCausality, dependent, what)
  else
    error(sourceInfo(), 'Unknown/unsupported FMI version: <%fmiVersion%>')
end dumpVariable;

template dumpOutputGetEnumerationVariables(list<ModelVariables> fmiModelVariablesList, list<TypeDefinitions> fmiTypeDefinitionsList, String fmiGetFunction, String fmiType)
::=
  <<
  <%fmiModelVariablesList |> fmiModelVariable => dumpOutputGetEnumerationVariable(fmiModelVariable, fmiTypeDefinitionsList, fmiGetFunction, fmiType)%>
  >>
end dumpOutputGetEnumerationVariables;

template dumpOutputGetEnumerationVariable(ModelVariables fmiModelVariable, list<TypeDefinitions> fmiTypeDefinitionsList, String fmiGetFunction, String fmiType)
::=
match fmiModelVariable
case ENUMERATIONVARIABLE(variability = "",causality="") then
  <<
  {<%name%>} = map_<%getEnumerationTypeFromTypes(fmiTypeDefinitionsList, baseType)%>_from_integers(<%fmiGetFunction%>(<%fmiType%>, {<%valueReference%>}, flowStatesInputs));<%\n%>
  >>
case ENUMERATIONVARIABLE(variability = "",causality="output") then
  <<
  {<%name%>} = map_<%getEnumerationTypeFromTypes(fmiTypeDefinitionsList, baseType)%>_from_integers(<%fmiGetFunction%>(<%fmiType%>, {<%valueReference%>}, flowStatesInputs));<%\n%>
  >>
end dumpOutputGetEnumerationVariable;

/* public */ template simulationInitFunction(SimCode simCode, String guid)
 "Generates the contents of the makefile for the simulation case.
  used in Compiler/Template/CodegenFMU.tpl"
::=
match simCode
case SIMCODE(modelInfo = MODELINFO(functions = functions, varInfo = vi as VARINFO(__), vars = vars as SIMVARS(__)),
             simulationSettingsOpt = SOME(s as SIMULATION_SETTINGS(__)), makefileParams = makefileParams as MAKEFILE_PARAMS(__))
  then
  <<
  #include "simulation_data.h"

  OMC_DISABLE_OPT<%/* This function is very simple and doesn't need to be optimized. GCC/clang spend way too much time looking at it. */%>
  void <%symbolName(modelNamePrefix(simCode),"read_input_fmu")%>(MODEL_DATA* modelData, SIMULATION_INFO* simulationInfo)
  {
    simulationInfo->startTime = <%s.startTime%>;
    simulationInfo->stopTime = <%s.stopTime%>;
    simulationInfo->stepSize = <%s.stepSize%>;
    simulationInfo->tolerance = <%s.tolerance%>;
    simulationInfo->solverMethod = "<%s.method%>";
    simulationInfo->outputFormat = "<%s.outputFormat%>";
    simulationInfo->variableFilter = "<%s.variableFilter%>";
    simulationInfo->OPENMODELICAHOME = "<%makefileParams.omhome%>";
    <%System.tmpTickReset(1000)%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.stateVars       |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%vars.derivativeVars  |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%vars.algVars         |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%vars.discreteAlgVars |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%vars.realOptimizeConstraintsVars
                           |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%vars.realOptimizeFinalConstraintsVars
                           |> var => ScalarVariableFMU(var,"realVarsData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.paramVars       |> var => ScalarVariableFMU(var,"realParameterData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.intAlgVars      |> var => ScalarVariableFMU(var,"integerVarsData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.intParamVars    |> var => ScalarVariableFMU(var,"integerParameterData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.boolAlgVars     |> var => ScalarVariableFMU(var,"booleanVarsData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.boolParamVars   |> var => ScalarVariableFMU(var,"booleanParameterData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.stringAlgVars   |> var => ScalarVariableFMU(var,"stringVarsData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.stringParamVars |> var => ScalarVariableFMU(var,"stringParameterData") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%
    /* Skip these; shouldn't be needed to look at in the FMU
    <%vars.aliasVars       |> var => ScalarVariableFMU(var,"realAlias") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.intAliasVars    |> var => ScalarVariableFMU(var,"integerAlias") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.boolAliasVars   |> var => ScalarVariableFMU(var,"booleanAlias") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    <%vars.stringAliasVars |> var => ScalarVariableFMU(var,"stringAlias") ;separator="\n";empty%>
    <%System.tmpTickResetIndex(0,2)%>
    */
    %>
  }
  >>
end simulationInitFunction;

template getInfoArgsFMU(String str, builtin.SourceInfo info)
::=
  match info
    case SOURCEINFO(__) then
      <<
      <%str%>.filename = "<%Util.escapeModelicaStringToCString(fileName)%>";
      <%str%>.lineStart = <%lineNumberStart%>;
      <%str%>.colStart = <%columnNumberStart%>;
      <%str%>.lineEnd = <%lineNumberEnd%>;
      <%str%>.colEnd = <%columnNumberEnd%>;
      <%str%>.readonly = <%if isReadOnly then 1 else 0%>;
      >>
end getInfoArgsFMU;

template ScalarVariableFMU(SimVar simVar, String classType)
 "Generates code for ScalarVariable file for FMU target."
::=
  match simVar
    case SIMVAR(source = SOURCE(info = info)) then
      let valueReference = System.tmpTick()
      let ci = System.tmpTickIndex(2)
      let description = if comment then Util.escapeModelicaStringToCString(comment)
      let infostr = 'modelData-><%classType%>[<%ci%>].info'
      let attrstr = 'modelData-><%classType%>[<%ci%>].attribute'
      <<
      <%infostr%>.id = <%valueReference%>;
      <%infostr%>.name = "<%Util.escapeModelicaStringToCString(crefStrNoUnderscore(name))%>";
      <%infostr%>.comment = "<%description%>";
      <%getInfoArgsFMU(infostr+".info", info)%>
      <%ScalarVariableTypeFMU(attrstr, unit, displayUnit, minValue, maxValue, initialValue, nominalValue, isFixed, type_)%>
      >>
end ScalarVariableFMU;

template optInitValFMU(Option<Exp> exp, String default)
::=
  match exp
  case SOME(e) then
  (
  match e
  case ICONST(__) then integer
  case RCONST(__) then real
  case SCONST(__) then 'mmc_mk_scon("<%Util.escapeModelicaStringToCString(string)%>")'
  case BCONST(__) then if bool then 1 else 0
  case ENUM_LITERAL(__) then '<%index%>'
  else default // error(sourceInfo(), 'initial value of unknown type: <%printExpStr(e)%>')
  )
  else default
end optInitValFMU;

template ScalarVariableTypeFMU(String attrstr, String unit, String displayUnit, Option<DAE.Exp> minValue, Option<DAE.Exp> maxValue, Option<DAE.Exp> startValue, Option<DAE.Exp> nominalValue, Boolean isFixed, DAE.Type type_)
 "Generates code for ScalarVariable Type file for FMU target."
::=
  match type_
    case T_INTEGER(__) then
      <<
      <%attrstr%>.min = <%optInitValFMU(minValue,"-LONG_MAX")%>;
      <%attrstr%>.max = <%optInitValFMU(maxValue,"LONG_MAX")%>;
      <%attrstr%>.fixed = <%if isFixed then 1 else 0%>;
      <%attrstr%>.start = <%optInitValFMU(startValue,"0")%>;
      >>
    case T_REAL(__) then
      <<
      <%attrstr%>.unit = "<%Util.escapeModelicaStringToCString(unit)%>";
      <%attrstr%>.displayUnit = "<%Util.escapeModelicaStringToCString(displayUnit)%>";
      <%attrstr%>.min = <%optInitValFMU(minValue,"-DBL_MAX")%>;
      <%attrstr%>.max = <%optInitValFMU(maxValue,"DBL_MAX")%>;
      <%attrstr%>.fixed = <%if isFixed then 1 else 0%>;
      <%attrstr%>.useNominal = <%if nominalValue then 1 else 0%>;
      <%attrstr%>.nominal = <%optInitValFMU(nominalValue,"1.0")%>;
      <%attrstr%>.start = <%optInitValFMU(startValue,"0.0")%>;
      >>
    case T_BOOL(__) then
      <<
      <%attrstr%>.fixed = <%if isFixed then 1 else 0%>;
      <%attrstr%>.start = <%optInitValFMU(startValue,"0")%>;
      >>
    case T_STRING(__) then
      <<
      <%attrstr%>.start = <%optInitValFMU(startValue,"mmc_mk_scon(\"\")")%>;
      >>
    case T_ENUMERATION(__) then
      <<
      <%attrstr%>.min = <%optInitValFMU(minValue,"1")%>;
      <%attrstr%>.max = <%optInitValFMU(maxValue,listLength(names))%>;
      <%attrstr%>.fixed = <%if isFixed then 1 else 0%>;
      <%attrstr%>.start = <%optInitValFMU(startValue,"0")%>;
      >>
    else error(sourceInfo(), 'ScalarVariableTypeFMU: <%unparseType(type_)%>')
end ScalarVariableTypeFMU;

annotation(__OpenModelica_Interface="backend");
end CodegenFMU;
