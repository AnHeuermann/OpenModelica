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

package CodegenFMU2

import interface SimCodeTV;
import interface SimCodeBackendTV;
import CodegenUtil.*;
import CodegenUtilSimulation.*;
import CodegenC.*; //unqualified import, no need the CodegenC is optional when calling a template; or mandatory when the same named template exists in this package (name hiding)
import CodegenFMUCommon.*;

// Code for generating modelDescription.xml file for FMI 2.0 ModelExchange.
template fmiModelDescription(SimCode simCode, String guid, String FMUType, list<String> sourceFiles)
  "Generates code for ModelDescription file for FMU target."
::=
//  <%UnitDefinitions(simCode)%>
//  <%VendorAnnotations(simCode)%>
match simCode
case SIMCODE(__) then
  <<
  <fmiModelDescription
    <%fmiModelDescriptionAttributes(simCode,guid)%>>
    <%if isFMIMEType(FMUType) then ModelExchange(simCode, sourceFiles)%>
    <%if isFMICSType(FMUType) then CoSimulation(simCode, sourceFiles)%>
    <%UnitDefinitions(simCode)%>
    <%fmiTypeDefinitions(simCode, "2.0")%>
    <% if Flags.isSet(Flags.FMU_EXPERIMENTAL) then
    <<
    <LogCategories>
      <Category name="logEvents" description="logEvents" />
      <Category name="logSingularLinearSystems" description="logSingularLinearSystems" />
      <Category name="logNonlinearSystems" description="logNonlinearSystems" />
      <Category name="logDynamicStateSelection" description="logDynamicStateSelection" />
      <Category name="logStatusWarning" description="logStatusWarning" />
      <Category name="logStatusDiscard" description="logStatusDiscard" />
      <Category name="logStatusError" description="logStatusError" />
      <Category name="logStatusFatal" description="logStatusFatal" />
      <Category name="logStatusPending" description="logStatusPending" />
      <Category name="logAll" description="logAll" />
      <Category name="logFmi2Call" description="logFmi2Call" />
    </LogCategories>
    >> else
    <<
    <LogCategories>
      <Category name="logEvents" />
      <Category name="logSingularLinearSystems" />
      <Category name="logNonlinearSystems" />
      <Category name="logDynamicStateSelection" />
      <Category name="logStatusWarning" />
      <Category name="logStatusDiscard" />
      <Category name="logStatusError" />
      <Category name="logStatusFatal" />
      <Category name="logStatusPending" />
      <Category name="logAll" />
      <Category name="logFmi2Call" />
    </LogCategories>
    >> %>
    <%DefaultExperiment(simulationSettingsOpt)%>
    <%fmiModelVariables(simCode, "2.0")%>
    <%ModelStructure(modelStructure)%>
  </fmiModelDescription>
  >>
end fmiModelDescription;

template fmiModelDescriptionAttributes(SimCode simCode, String guid)
  "Generates code for ModelDescription file for FMU target."
::=
match simCode
case SIMCODE(modelInfo = MODELINFO(varInfo = vi as VARINFO(__), vars = SIMVARS(stateVars = listStates))) then
  let fmiVersion = '2.0'
  let modelName = dotPath(modelInfo.name)
  let description = modelInfo.description
  let generationTool= 'OpenModelica Compiler <%getVersionNr()%>'
  let generationDateAndTime = xsdateTime(getCurrentDateTime())
  let variableNamingConvention = 'structured'
  let numberOfEventIndicators = getNumberOfEventIndicators(simCode)
  <<
  fmiVersion="<%fmiVersion%>"
  modelName="<%Util.escapeModelicaStringToXmlString(modelName)%>"
  guid="{<%guid%>}"
  description="<%Util.escapeModelicaStringToXmlString(description)%>"
  generationTool="<%Util.escapeModelicaStringToXmlString(generationTool)%>"
  generationDateAndTime="<%Util.escapeModelicaStringToXmlString(generationDateAndTime)%>"
  variableNamingConvention="<%variableNamingConvention%>"
  numberOfEventIndicators="<%numberOfEventIndicators%>"
  >>
end fmiModelDescriptionAttributes;

template CoSimulation(SimCode simCode, list<String> sourceFiles)
  "Generates CoSimulation code for ModelDescription file for FMU target."
::=
match simCode
case SIMCODE(__) then
  let modelIdentifier = modelNamePrefix(simCode)
  <<
  <CoSimulation
    modelIdentifier="<%Util.escapeModelicaStringToXmlString(modelIdentifier)%>"
    needsExecutionTool="false"
    canHandleVariableCommunicationStepSize="true"
    canInterpolateInputs="true"
    maxOutputDerivativeOrder="1"
    canRunAsynchronuously = "false"
    canBeInstantiatedOnlyOncePerProcess="false"
    canNotUseMemoryManagementFunctions="false"
    <% if Flags.isSet(FMU_EXPERIMENTAL) then 'canGetAndSetFMUstate="true"' else 'canGetAndSetFMUstate="false"'%>
    <% if Flags.isSet(FMU_EXPERIMENTAL) then 'canSerializeFMUstate="true"' else 'canSerializeFMUstate="false"'%>
    <% if Flags.isSet(FMU_EXPERIMENTAL) then 'providesDirectionalDerivative="true"' else 'providesDirectionalDerivative="false"'%>>
    <%SourceFiles(sourceFiles)%>
  </CoSimulation>
  >>
end CoSimulation;

template fmiIdentifierFunctionsHeaders()
  "FMI2 identifier function headers for template fmiIdentifierFunctions()"
::=
  <<
  void eventUpdate(ModelInstance* comp, fmi2EventInfo* eventInfo);
  fmi2Real getReal(ModelInstance* comp, const fmi2ValueReference vr);
  fmi2Status setReal(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Real value);
  fmi2Integer getInteger(ModelInstance* comp, const fmi2ValueReference vr);
  fmi2Status setInteger(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Integer value);
  fmi2Boolean getBoolean(ModelInstance* comp, const fmi2ValueReference vr);
  fmi2Status setBoolean(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Boolean value);
  fmi2String getString(ModelInstance* comp, const fmi2ValueReference vr);
  fmi2Status setString(ModelInstance* comp, const fmi2ValueReference vr, fmi2String value);
  fmi2Status setExternalFunction(ModelInstance* c, const fmi2ValueReference vr, const void* value);
  fmi2ValueReference mapInputReference2InputNumber(const fmi2ValueReference vr);
  fmi2ValueReference mapOutputReference2OutputNumber(const fmi2ValueReference vr);
  fmi2ValueReference mapOutputReference2RealOutputDerivatives(const fmi2ValueReference vr);
  fmi2ValueReference mapInitialUnknownsdependentIndex(const fmi2ValueReference vr);
  fmi2ValueReference mapInitialUnknownsIndependentIndex(const fmi2ValueReference vr);
  >>
end fmiIdentifierFunctionsHeaders;

template fmiIdentifierFunctions(SimCode simCode)
  "FMI2 identifier functions to e.g. get/set variables."
::=
  match simCode
  case SIMCODE(__) then
    <<
    <%eventUpdateFunction()%>
    <%getRealFunction(simCode, modelInfo)%>
    <%setRealFunction(simCode, modelInfo)%>
    <%getIntegerFunction(simCode, modelInfo)%>
    <%setIntegerFunction(simCode, modelInfo)%>
    <%getBooleanFunction(simCode, modelInfo)%>
    <%setBooleanFunction(simCode, modelInfo)%>
    <%getStringFunction(simCode, modelInfo)%>
    <%setStringFunction(simCode, modelInfo)%>
    <%setExternalFunction(modelInfo)%>
    <%mapInputAndOutputs(simCode)%>
    <%mapRealOutputDerivatives(simCode)%>
    <%mapInitialUnknownsdependentCrefs(simCode)%>
    <%mapInitialUnknownsIndependentCrefs(simCode)%>
    >>
end fmiIdentifierFunctions;

template eventUpdateFunction()
  "Generates event update function for c file."
::=
  <<
  // Used to set the next time event, if any.
  void eventUpdate(ModelInstance* comp, fmi2EventInfo* eventInfo) {
  }

  >>
end eventUpdateFunction;


template getRealFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates getReal function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__),varInfo=VARINFO(numAlgAliasVars=numAlgAliasVars, numParams=numParams, numStateVars=numStateVars, numAlgVars= numAlgVars, numDiscreteReal=numDiscreteReal)) then
  let ixFirstParam = intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal))
  let ixFirstAlias = intAdd(numParams, intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal)))
  let ixEnd = intAdd(numAlgAliasVars,intAdd(numParams, intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal))))
  <<
  <%if numAlgAliasVars then
  <<
  static const int realAliasIndexes[<%numAlgAliasVars%>] = {
    <%vars.aliasVars |> v as SIMVAR(__) => aliasSetVR(simCode, aliasvar) ; separator=", "; align=20; alignSeparator=",\n" %>
  };

  >>
  %>
  fmi2Real getReal(ModelInstance* comp, const fmi2ValueReference vr) {
    if (vr < <%ixFirstParam%>) {
      return comp->fmuData->localData[0]->realVars[vr];
    }
    if (vr < <%ixFirstAlias%>) {
      return comp->fmuData->simulationInfo->realParameter[vr-<%ixFirstParam%>];
    }
    <%if numAlgAliasVars then
    <<
    if (vr < <%ixEnd%>) {
      int ix = realAliasIndexes[vr-<%ixFirstAlias%>];
      return ix>=0 ? getReal(comp, ix) : -getReal(comp, -(ix+1));
    }
    >>
    %>
    return NAN;
  }

  >>
end getRealFunction;


template setRealFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates setReal function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__),varInfo=VARINFO(numAlgAliasVars=numAlgAliasVars, numParams=numParams, numStateVars=numStateVars, numAlgVars= numAlgVars, numDiscreteReal=numDiscreteReal)) then
  let ixFirstParam = intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal))
  let ixFirstAlias = intAdd(numParams, intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal)))
  let ixEnd = intAdd(numAlgAliasVars,intAdd(numParams, intAdd(intMul(2,numStateVars),intAdd(numAlgVars,numDiscreteReal))))
  <<
  fmi2Status setReal(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Real value) {
    // set start value attribute for all variable that has start value, till initialization mode
    if (vr < <%ixFirstParam%> && (comp->state == model_state_instantiated || comp->state == model_state_initialization_mode)) {
      comp->fmuData->modelData->realVarsData[vr].attribute.start = value;
    }
    if (vr < <%ixFirstParam%>) {
      comp->fmuData->localData[0]->realVars[vr] = value;
      return fmi2OK;
    }
    if (vr < <%ixFirstAlias%>) {
      comp->fmuData->simulationInfo->realParameter[vr-<%ixFirstParam%>] = value;
      return fmi2OK;
    }
    <%if numAlgAliasVars then
    <<
    if (vr < <%ixEnd%>) {
      int ix = realAliasIndexes[vr-<%ixFirstAlias%>];
      return ix >= 0 ? setReal(comp, ix, value) : setReal(comp, -(ix+1), -value);
    }
    >>
    %>
    return fmi2Error;
  }

  >>
end setRealFunction;


template getIntegerFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates setInteger function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__),varInfo=VARINFO(numIntAliasVars=numAliasVars, numIntParams=numParams, numIntAlgVars=numAlgVars)) then
  let ixFirstParam = numAlgVars
  let ixFirstAlias = intAdd(numParams, numAlgVars)
  let ixEnd = intAdd(numAliasVars,intAdd(numParams, numAlgVars))
  <<
  <% if numAliasVars then
  <<
  static const int intAliasIndexes[<%numAliasVars%>] = {
    <%vars.intAliasVars |> v as SIMVAR(__) => aliasSetVR(simCode, aliasvar) ; separator=", "; align=20; alignSeparator=",\n" %>
  };

  >>
  %>
  fmi2Integer getInteger(ModelInstance* comp, const fmi2ValueReference vr) {
    if (vr < <%ixFirstParam%>) {
      return comp->fmuData->localData[0]->integerVars[vr];
    }
    if (vr < <%ixFirstAlias%>) {
      return comp->fmuData->simulationInfo->integerParameter[vr-<%ixFirstParam%>];
    }
    <% if numAliasVars then
    <<
    if (vr < <%ixEnd%>) {
      int ix = intAliasIndexes[vr-<%ixFirstAlias%>];
      return ix>=0 ? getInteger(comp, ix) : -getInteger(comp, -(ix+1));
    }
    >>
    %>
    return 0;
  }

  >>
end getIntegerFunction;

template setIntegerFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates getInteger function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__),varInfo=VARINFO(numIntAliasVars=numAliasVars, numIntParams=numParams, numIntAlgVars=numAlgVars)) then
  let ixFirstParam = numAlgVars
  let ixFirstAlias = intAdd(numParams, numAlgVars)
  let ixEnd = intAdd(numAliasVars,intAdd(numParams, numAlgVars))
  <<
  fmi2Status setInteger(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Integer value) {
    // set start value attribute for all variable that has start value, till initialization mode
    if (vr < <%ixFirstParam%> && (comp->state == model_state_instantiated || comp->state == model_state_initialization_mode)) {
      comp->fmuData->modelData->integerVarsData[vr].attribute.start = value;
    }
    if (vr < <%ixFirstParam%>) {
      comp->fmuData->localData[0]->integerVars[vr] = value;
      return fmi2OK;
    }
    if (vr < <%ixFirstAlias%>) {
      comp->fmuData->simulationInfo->integerParameter[vr-<%ixFirstParam%>] = value;
      return fmi2OK;
    }
    <% if numAliasVars then
    <<
    if (vr < <%ixEnd%>) {
      int ix = intAliasIndexes[vr-<%ixFirstAlias%>];
      return ix >= 0 ? setInteger(comp, ix, value) : setInteger(comp, -(ix+1), -value);
    }
    >>
    %>
    return fmi2Error;
  }
  >>
end setIntegerFunction;

template getBooleanFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates setBoolean function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__)) then
  <<
  fmi2Boolean getBoolean(ModelInstance* comp, const fmi2ValueReference vr) {
    switch (vr) {
      <%vars.boolAlgVars |> var => SwitchVars(simCode, var, "booleanVars") ;separator="\n"%>
      <%vars.boolParamVars |> var => SwitchParameters(simCode, var, "booleanParameter") ;separator="\n"%>
      <%vars.boolAliasVars |> var => SwitchAliasVars(simCode, var, "Boolean", "!") ;separator="\n"%>
      default:
        return fmi2False;
    }
  }

  >>
end getBooleanFunction;

template setBooleanFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates getBoolean function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__)) then
  <<
  fmi2Status setBoolean(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Boolean value) {
    switch (vr) {
      <%vars.boolAlgVars |> var => SwitchVarsSet(simCode, var, "booleanVars") ;separator="\n"%>
      <%vars.boolParamVars |> var => SwitchParametersSet(simCode, var, "booleanParameter") ;separator="\n"%>
      <%vars.boolAliasVars |> var => SwitchAliasVarsSet(simCode, var, "Boolean", "!") ;separator="\n"%>
      default:
        return fmi2Error;
    }
    return fmi2OK;
  }

  >>
end setBooleanFunction;

template getStringFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates getString function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__)) then
  <<
  fmi2String getString(ModelInstance* comp, const fmi2ValueReference vr) {
    switch (vr) {
      <%vars.stringAlgVars |> var => SwitchVars(simCode, var, "stringVars") ;separator="\n"%>
      <%vars.stringParamVars |> var => SwitchParameters(simCode, var, "stringParameter") ;separator="\n"%>
      <%vars.stringAliasVars |> var => SwitchAliasVars(simCode, var, "String", "") ;separator="\n"%>
      default:
        return "";
    }
  }

  >>
end getStringFunction;

template setStringFunction(SimCode simCode, ModelInfo modelInfo)
  "Generates setString function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__)) then
  <<
  fmi2Status setString(ModelInstance* comp, const fmi2ValueReference vr, fmi2String value) {
    switch (vr) {
      <%vars.stringAlgVars |> var => SwitchVarsSet(simCode, var, "stringVars") ;separator="\n"%>
      <%vars.stringParamVars |> var => SwitchParametersSet(simCode, var, "stringParameter") ;separator="\n"%>
      <%vars.stringAliasVars |> var => SwitchAliasVarsSet(simCode, var, "String", "") ;separator="\n"%>
      default:
        return fmi2Error;
    }
    return fmi2OK;
  }

  >>
end setStringFunction;

template setExternalFunction(ModelInfo modelInfo)
  "Generates setExternal function for c file."
::=
match modelInfo
case MODELINFO(vars=SIMVARS(__)) then
  let externalFuncs = setExternalFunctionsSwitch(functions)
  <<
  fmi2Status setExternalFunction(ModelInstance* c, const fmi2ValueReference vr, const void* value){
    switch (vr) {
      <%externalFuncs%>
      default:
        return fmi2Error;
    }
    return fmi2OK;
  }

  >>
end setExternalFunction;


template mapInputAndOutputs(SimCode simCode)
::=
match simCode
case SIMCODE(modelInfo=MODELINFO(vars=SIMVARS(inputVars=inputVars, outputVars=outputVars))) then
    <<
    /* function maps input references to a input index used in partialDerivatives */
    fmi2ValueReference mapInputReference2InputNumber(const fmi2ValueReference vr) {
        switch (vr) {
          <%inputVars |> var hasindex index0 =>  match var case SIMVAR(name=name, type_=T_REAL()) then
          'case <%lookupVR(name, simCode)%>: return <%index0%>; break;' ;separator="\n"%>
          default:
            return -1;
        }
    }
    /* function maps output references to a input index used in partialDerivatives */
    fmi2ValueReference mapOutputReference2OutputNumber(const fmi2ValueReference vr) {
        switch (vr) {
          <%outputVars |> var hasindex index0 =>  match var case SIMVAR(name=name, type_=T_REAL()) then
          'case <%lookupVR(name, simCode)%>: return <%index0%>; break;' ;separator="\n"%>
          default:
            return -1;
        }
    }
    >>
end match
end mapInputAndOutputs;

template mapRealOutputDerivatives(SimCode simCode)
::=
  match simCode
  case SIMCODE(modelInfo=MODELINFO(vars=SIMVARS(outputVars=outputVars))) then
    <<
    /* function maps output references to an internal output Real derivatives */
    fmi2ValueReference mapOutputReference2RealOutputDerivatives(const fmi2ValueReference vr) {
        switch (vr) {
          <%outputVars |> var =>  match var case SIMVAR(name=name, type_=T_REAL()) then
          'case <%lookupVR(name, simCode)%>: return <%lookupVRForRealOutputDerivative(name, simCode, "2.0")%>; break;' ;separator="\n"%>
          default:
            return -1;
        }
    }
    >>
end match
end mapRealOutputDerivatives;

template mapInitialUnknownsdependentCrefs(SimCode simCode)
::=
  match simCode
  case SIMCODE(modelStructure=SOME(FMIMODELSTRUCTURE(fmiInitialUnknowns=FMIINITIALUNKNOWNS(sortedUnknownCrefs=sortedUnknownCrefs)))) then
    <<
    /* function maps initialUnknowns UnknownVars ValueReferences to an internal partial derivatives index */
    fmi2ValueReference mapInitialUnknownsdependentIndex(const fmi2ValueReference vr) {
        switch (vr) {
          <%sortedUnknownCrefs |> (index, cref) =>
          'case <%lookupVR(cref, simCode)%>: return <%index%>; break;' ;separator="\n"%>
          default:
            return -1;
        }
    }
    >>
  end match
end mapInitialUnknownsdependentCrefs;

template mapInitialUnknownsIndependentCrefs(SimCode simCode)
::=
match simCode
case SIMCODE(modelStructure=SOME(FMIMODELSTRUCTURE(fmiInitialUnknowns=FMIINITIALUNKNOWNS(sortedknownCrefs=sortedknownCrefs)))) then
    <<
    /* function maps initialUnknowns knownVars ValueReferences to an internal partial derivatives index */
    fmi2ValueReference mapInitialUnknownsIndependentIndex(const fmi2ValueReference vr) {
        switch (vr) {
          <%sortedknownCrefs |> (index, cref) =>
          'case <%lookupVR(cref, simCode)%>: return <%index%>; break;' ;separator="\n"%>
          default:
            return -1;
        }
    }
    >>
end match
end mapInitialUnknownsIndependentCrefs;

template fmuDeffile(SimCode simCode)
  "Generates the def file of 2 FMUs."
::=
  match simCode
  case SIMCODE(modelInfo=MODELINFO(__), makefileParams=MAKEFILE_PARAMS(__), simulationSettingsOpt = sopt) then
  <<
  EXPORTS
    ;***************************************************
    ;Common Functions
    ;****************************************************
    <%fileNamePrefix%>_fmiGetTypesPlatform @1
    <%fileNamePrefix%>_fmiGetVersion @2
    <%fileNamePrefix%>_fmiSetDebugLogging @3
    <%fileNamePrefix%>_fmiInstantiate @4
    <%fileNamePrefix%>_fmiFreeInstance @5
    <%fileNamePrefix%>_fmiSetupExperiment @6
    <%fileNamePrefix%>_fmiEnterInitializationMode @7
    <%fileNamePrefix%>_fmiExitInitializationMode @8
    <%fileNamePrefix%>_fmiTerminate @9
    <%fileNamePrefix%>_fmiReset @10
    <%fileNamePrefix%>_fmiGetReal @11
    <%fileNamePrefix%>_fmiGetInteger @12
    <%fileNamePrefix%>_fmiGetBoolean @13
    <%fileNamePrefix%>_fmiGetString @14
    <%fileNamePrefix%>_fmiSetReal @15
    <%fileNamePrefix%>_fmiSetInteger @16
    <%fileNamePrefix%>_fmiSetBoolean @17
    <%fileNamePrefix%>_fmiSetString @18
    <%fileNamePrefix%>_fmiGetFMUstate @19
    <%fileNamePrefix%>_fmiSetFMUstate @20
    <%fileNamePrefix%>_fmiFreeFMUstate @21
    <%fileNamePrefix%>_fmiSerializedFMUstateSize @22
    <%fileNamePrefix%>_fmiSerializeFMUstate @23
    <%fileNamePrefix%>_fmiDeSerializeFMUstate @24
    <%fileNamePrefix%>_fmiGetDirectionalDerivative @25
    ;***************************************************
    ;Functions for FMI for Model Exchange
    ;****************************************************
    <%fileNamePrefix%>_fmiEnterEventMode @26
    <%fileNamePrefix%>_fmiNewDiscreteStates @27
    <%fileNamePrefix%>_fmiEnterContinuousTimeMode @28
    <%fileNamePrefix%>_fmiCompletedIntegratorStep @29
    <%fileNamePrefix%>_fmiSetTime @30
    <%fileNamePrefix%>_fmiSetContinuousStates @31
    <%fileNamePrefix%>_fmiGetDerivatives @32
    <%fileNamePrefix%>_fmiGetEventIndicators @33
    <%fileNamePrefix%>_fmiGetContinuousStates @34
    <%fileNamePrefix%>_fmiGetNominalsOfContinuousStates @35
    ;***************************************************
    ;Functions for FMI for Co-Simulation
    ;****************************************************
    <%fileNamePrefix%>_fmiSetRealInputDerivatives @36
    <%fileNamePrefix%>_fmiGetRealOutputDerivatives @37
    <%fileNamePrefix%>_fmiDoStep @38
    <%fileNamePrefix%>_fmiCancelStep @39
    <%fileNamePrefix%>_fmiGetStatus @40
    <%fileNamePrefix%>_fmiGetRealStatus @41
    <%fileNamePrefix%>_fmiGetIntegerStatus @42
    <%fileNamePrefix%>_fmiGetBooleanStatus @43
    <%fileNamePrefix%>_fmiGetStringStatus @44
    <% if Flags.isSet(Flags.FMU_EXPERIMENTAL) then
    <<
    ;***************************************************
    ; Experimetnal function for FMI for ModelExchange
    ;****************************************************
    <%fileNamePrefix%>_fmiGetSpecificDerivatives @45
    >> %>
  >>
end fmuDeffile;

template importFMU2ModelExchange(FmiImport fmi, String name)
  "Generates Modelica code for FMI Model Exchange version 2.0"
::=
match fmi
case FMIIMPORT(fmiInfo=INFO(__),fmiExperimentAnnotation=EXPERIMENTANNOTATION(__)) then
  /* Get Real parameters and their value references */

  let realParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "real", "parameter", false, 1)
  let realParametersNames = dumpFMI2Variable(fmiModelVariablesList, "real", "parameter", false, 2)
  /* Get Integer parameters and their value references */
  let integerParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "integer", "parameter", false, 1)
  let integerParametersNames = dumpFMI2Variable(fmiModelVariablesList, "integer", "parameter", false, 2)
  /* Get Boolean parameters and their value references */
  let booleanParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "boolean", "parameter", false, 1)
  let booleanParametersNames = dumpFMI2Variable(fmiModelVariablesList, "boolean", "parameter", false, 2)
  /* Get String parameters and their value references */
  let stringParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "string", "parameter", false, 1)
  let stringParametersNames = dumpFMI2Variable(fmiModelVariablesList, "string", "parameter", false, 2)
  /* Get dependent Real parameters and their value references */
  let realDependentParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "real", "parameter", true, 1)
  let realDependentParametersNames = dumpFMI2Variable(fmiModelVariablesList, "real", "parameter", true, 2)
  /* Get dependent Integer parameters and their value references */
  let integerDependentParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "integer", "parameter", true, 1)
  let integerDependentParametersNames = dumpFMI2Variable(fmiModelVariablesList, "integer", "parameter", true, 2)
  /* Get dependent Boolean parameters and their value references */
  let booleanDependentParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "boolean", "parameter", true, 1)
  let booleanDependentParametersNames = dumpFMI2Variable(fmiModelVariablesList, "boolean", "parameter", true, 2)
  /* Get dependent String parameters and their value references */
  let stringDependentParametersVRs = dumpFMI2Variable(fmiModelVariablesList, "string", "parameter", true, 1)
  let stringDependentParametersNames = dumpFMI2Variable(fmiModelVariablesList, "string", "parameter", true, 2)
  /* Get input Real varibales and their value references */
  let nRealInputVariables = listLength(filterModelVariables(fmiModelVariablesList, "real", "input"))
  let realInputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "real", "input", false, 1)
  let realInputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "real", "input", false, 2)
  let realInputVariablesReturnNames = dumpFMI2Variable(fmiModelVariablesList, "real", "input", false, 3)
  /* Get input Integer varibales and their value references */
  let nIntegerInputVariables = listLength(filterModelVariables(fmiModelVariablesList, "integer", "input"))
  let integerInputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "integer", "input", false, 1)
  let integerInputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "integer", "input", false, 2)
  let integerInputVariablesReturnNames = dumpFMI2Variable(fmiModelVariablesList, "integer", "input", false, 3)
  /* Get input Boolean varibales and their value references */
  let nBooleanInputVariables = listLength(filterModelVariables(fmiModelVariablesList, "boolean", "input"))
  let booleanInputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "boolean", "input", false, 1)
  let booleanInputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "boolean", "input", false, 2)
  let booleanInputVariablesReturnNames = dumpFMI2Variable(fmiModelVariablesList, "boolean", "input", false, 3)
  /* Get input String varibales and their value references */
  let nStringInputVariables = listLength(filterModelVariables(fmiModelVariablesList, "string", "input"))
  let stringInputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "string", "input", false, 1)
  let stringStartVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "string", "input", false, 2)
  let stringInputVariablesReturnNames = dumpFMI2Variable(fmiModelVariablesList, "string", "input", false, 3)
  /* Get output Real varibales and their value references */
  let realOutputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "real", "output", false, 1)
  let realOutputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "real", "output", false, 2)
  /* Get output Integer varibales and their value references */
  let integerOutputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "integer", "output", false, 1)
  let integerOutputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "integer", "output", false, 2)
  /* Get output Boolean varibales and their value references */
  let booleanOutputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "boolean", "output", false, 1)
  let booleanOutputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "boolean", "output", false, 2)
  /* Get output String varibales and their value references */
  let stringOutputVariablesVRs = dumpFMI2Variable(fmiModelVariablesList, "string", "output", false, 1)
  let stringOutputVariablesNames = dumpFMI2Variable(fmiModelVariablesList, "string", "output", false, 2)
  <<
  model <%if stringEq(name, "") then fmiInfo.fmiModelIdentifier+"_"+getFMIType(fmiInfo)+"_FMU" else name%><%if stringEq(fmiInfo.fmiDescription, "") then "" else " \""+fmiInfo.fmiDescription+"\""%>
    <%dumpFMITypeDefinitions(fmiTypeDefinitionsList)%>
    constant String fmuWorkingDir = "<%fmuWorkingDirectory%>";
    parameter Integer logLevel = <%fmiLogLevel%> "log level used during the loading of FMU" annotation (Dialog(tab="FMI", group="Enable logging"));
    parameter Boolean debugLogging = <%fmiDebugOutput%> "enables the FMU simulation logging" annotation (Dialog(tab="FMI", group="Enable logging"));
    <%dumpFMIModelVariablesList("2.0", fmiModelVariablesList, fmiTypeDefinitionsList, generateInputConnectors, generateOutputConnectors)%>
  protected
    FMI2ModelExchange fmi2me = FMI2ModelExchange(logLevel, fmuWorkingDir, "<%fmiInfo.fmiModelIdentifier%>", debugLogging);
    constant Integer numberOfContinuousStates = <%listLength(fmiInfo.fmiNumberOfContinuousStates)%>;
    Real fmi_x[numberOfContinuousStates] "States";
    Real fmi_x_new[numberOfContinuousStates](each fixed=true) "New States";
    constant Integer numberOfEventIndicators = <%listLength(fmiInfo.fmiNumberOfEventIndicators)%>;
    Real fmi_z[numberOfEventIndicators] "Events Indicators";
    Boolean fmi_z_positive[numberOfEventIndicators](each fixed=true);
    parameter Real flowStartTime(fixed=false);
    Real flowTime;
    parameter Real flowEnterInitialization(fixed=false);
    parameter Real flowInitialized(fixed=false);
    parameter Real flowParamsStart(fixed=false);
    parameter Real flowInitInputs(fixed=false);
    Real flowStatesInputs;
    <%if not stringEq(realInputVariablesVRs, "") then "Real realInputVariables["+nRealInputVariables+"];"%>
    <%if not stringEq(realInputVariablesVRs, "") then "Real "+realInputVariablesReturnNames+";"%>
    <%if not stringEq(integerInputVariablesVRs, "") then "Integer integerInputVariables["+nIntegerInputVariables+"];"%>
    <%if not stringEq(integerInputVariablesVRs, "") then "Integer "+integerInputVariablesReturnNames+";"%>
    <%if not stringEq(booleanInputVariablesVRs, "") then "Boolean booleanInputVariables["+nBooleanInputVariables+"];"%>
    <%if not stringEq(booleanInputVariablesVRs, "") then "Boolean "+booleanInputVariablesReturnNames+";"%>
    <%if not stringEq(stringInputVariablesVRs, "") then "String stringInputVariables["+nStringInputVariables+"];"%>
    <%if not stringEq(stringInputVariablesVRs, "") then "String "+stringInputVariablesReturnNames+";"%>
    Boolean callEventUpdate;
    Boolean newStatesAvailable(fixed = true);
    Real triggerDSSEvent;
    Real nextEventTime(fixed = true);
  initial equation
    <%if intGt(listLength(fmiInfo.fmiNumberOfContinuousStates), 0) then
    <<
    fmi_x = fmi2Functions.fmi2GetContinuousStates(fmi2me, numberOfContinuousStates, flowParamsStart+flowInitialized);
    >>
    %>
  initial algorithm
    flowParamsStart := 1;
    flowInitInputs := 1;
    flowStartTime := fmi2Functions.fmi2SetupExperiment(fmi2me, false, 0.0, time, false, 0.0, flowParamsStart+flowInitInputs);
    flowEnterInitialization := fmi2Functions.fmi2EnterInitialization(fmi2me, flowParamsStart+flowInitInputs+flowStartTime);
    flowInitialized := fmi2Functions.fmi2ExitInitialization(fmi2me, flowParamsStart+flowInitInputs+flowStartTime+flowEnterInitialization);
    <%if not stringEq(realParametersVRs, "") then "flowParamsStart := fmi2Functions.fmi2SetRealParameter(fmi2me, {"+realParametersVRs+"}, {"+realParametersNames+"});"%>
    <%if not stringEq(integerParametersVRs, "") then "flowParamsStart := fmi2Functions.fmi2SetIntegerParameter(fmi2me, {"+integerParametersVRs+"}, {"+integerParametersNames+"});"%>
    <%if not stringEq(booleanParametersVRs, "") then "flowParamsStart := fmi2Functions.fmi2SetBooleanParameter(fmi2me, {"+booleanParametersVRs+"}, {"+booleanParametersNames+"});"%>
    <%if not stringEq(stringParametersVRs, "") then "flowParamsStart := fmi2Functions.fmi2SetStringParameter(fmi2me, {"+stringParametersVRs+"}, {"+stringParametersNames+"});"%>
  initial equation
    <%if not stringEq(realDependentParametersVRs, "") then "{"+realDependentParametersNames+"} = fmi2Functions.fmi2GetReal(fmi2me, {"+realDependentParametersVRs+"}, flowInitialized);"%>
    <%if not stringEq(integerDependentParametersVRs, "") then "{"+integerDependentParametersNames+"} = fmi2Functions.fmi2GetInteger(fmi2me, {"+integerDependentParametersVRs+"}, flowInitialized);"%>
    <%if not stringEq(booleanDependentParametersVRs, "") then "{"+booleanDependentParametersNames+"} = fmi2Functions.fmi2GetBoolean(fmi2me, {"+booleanDependentParametersVRs+"}, flowInitialized);"%>
    <%if not stringEq(stringDependentParametersVRs, "") then "{"+stringDependentParametersNames+"} = fmi2Functions.fmi2GetString(fmi2me, {"+stringDependentParametersVRs+"}, flowInitialized);"%>
  algorithm
    flowTime := if not initial() then fmi2Functions.fmi2SetTime(fmi2me, time, flowInitialized) else time;
    /* algorithm section ensures that inputs to fmi (if any) are set directly after the new time is set */
    <%if not stringEq(realInputVariablesVRs, "") then "realInputVariables := fmi2Functions.fmi2SetReal(fmi2me, {"+realInputVariablesVRs+"}, {"+realInputVariablesNames+"});"%>
    <%if not stringEq(integerInputVariablesVRs, "") then "integerInputVariables := fmi2Functions.fmi2SetInteger(fmi2me, {"+integerInputVariablesVRs+"}, {"+integerInputVariablesNames+"});"%>
    <%if not stringEq(booleanInputVariablesVRs, "") then "booleanInputVariables := fmi2Functions.fmi2SetBoolean(fmi2me, {"+booleanInputVariablesVRs+"}, {"+booleanInputVariablesNames+"});"%>
    <%if not stringEq(stringInputVariablesVRs, "") then "stringInputVariables := fmi2Functions.fmi2SetString(fmi2me, {"+stringInputVariablesVRs+"}, {"+stringStartVariablesNames+"});"%>
  equation
    <%if not stringEq(realInputVariablesVRs, "") then "{"+realInputVariablesReturnNames+"} = realInputVariables;"%>
    <%if not stringEq(integerInputVariablesVRs, "") then "{"+integerInputVariablesReturnNames+"} = integerInputVariables;"%>
    <%if not stringEq(booleanInputVariablesVRs, "") then "{"+booleanInputVariablesReturnNames+"} = booleanInputVariables;"%>
    <%if not stringEq(stringInputVariablesVRs, "") then "{"+stringInputVariablesReturnNames+"} = stringInputVariables;"%>
    flowStatesInputs = fmi2Functions.fmi2SetContinuousStates(fmi2me, fmi_x, flowParamsStart + flowTime);
    der(fmi_x) = fmi2Functions.fmi2GetDerivatives(fmi2me, numberOfContinuousStates, flowStatesInputs);
    fmi_z  = fmi2Functions.fmi2GetEventIndicators(fmi2me, numberOfEventIndicators, flowStatesInputs);
    for i in 1:size(fmi_z,1) loop
      fmi_z_positive[i] = if not terminal() then fmi_z[i] > 0 else pre(fmi_z_positive[i]);
    end for;

    triggerDSSEvent = noEvent(if callEventUpdate then flowStatesInputs+1.0 else flowStatesInputs-1.0);

    <%if not boolAnd(stringEq(realOutputVariablesNames, ""), stringEq(realOutputVariablesVRs, "")) then "{"+realOutputVariablesNames+"} = fmi2Functions.fmi2GetReal(fmi2me, {"+realOutputVariablesVRs+"}, flowStatesInputs);"%>
    <%if not boolAnd(stringEq(integerOutputVariablesNames, ""), stringEq(integerOutputVariablesVRs, "")) then "{"+integerOutputVariablesNames+"} = fmi2Functions.fmi2GetInteger(fmi2me, {"+integerOutputVariablesVRs+"}, flowStatesInputs);"%>
    <%if not boolAnd(stringEq(booleanOutputVariablesNames, ""), stringEq(booleanOutputVariablesVRs, "")) then "{"+booleanOutputVariablesNames+"} = fmi2Functions.fmi2GetBoolean(fmi2me, {"+booleanOutputVariablesVRs+"}, flowStatesInputs);"%>
    <%if not boolAnd(stringEq(stringOutputVariablesNames, ""), stringEq(stringOutputVariablesVRs, "")) then "{"+stringOutputVariablesNames+"} = fmi2Functions.fmi2GetString(fmi2me, {"+stringOutputVariablesVRs+"}, flowStatesInputs);"%>
    <%dumpOutputGetEnumerationVariables(fmiModelVariablesList, fmiTypeDefinitionsList, "fmi2Functions.fmi2GetInteger", "fmi2me")%>
    callEventUpdate = fmi2Functions.fmi2CompletedIntegratorStep(fmi2me, flowStatesInputs+flowTime);
  algorithm
  <%if intGt(listLength(fmiInfo.fmiNumberOfEventIndicators), 0) then
  <<
    when {<%fmiInfo.fmiNumberOfEventIndicators |> eventIndicator =>  "change(fmi_z_positive["+eventIndicator+"])" ;separator=" or "%>, triggerDSSEvent > flowStatesInputs, pre(nextEventTime) < time, terminal()} then
  >>
  else
  <<
    when {triggerDSSEvent > flowStatesInputs, pre(nextEventTime) < time, terminal()} then
  >>
  %>
      newStatesAvailable := fmi2Functions.fmi2EventUpdate(fmi2me);
      nextEventTime := fmi2Functions.fmi2nextEventTime(fmi2me, flowStatesInputs);
  <%if intGt(listLength(fmiInfo.fmiNumberOfContinuousStates), 0) then
  <<
      if newStatesAvailable then
        fmi_x_new := fmi2Functions.fmi2GetContinuousStates(fmi2me, numberOfContinuousStates, flowStatesInputs);
        <%fmiInfo.fmiNumberOfContinuousStates |> continuousStates =>  "reinit(fmi_x["+continuousStates+"], fmi_x_new["+continuousStates+"]);" ;separator="\n"%>
      end if;
  >>
  %>
    end when;
    annotation(experiment(StartTime=<%fmiExperimentAnnotation.fmiExperimentStartTime%>, StopTime=<%fmiExperimentAnnotation.fmiExperimentStopTime%>, Tolerance=<%fmiExperimentAnnotation.fmiExperimentTolerance%>));
    annotation (Icon(graphics={
        Rectangle(
          extent={{-100,100},{100,-100}},
          lineColor={0,0,0},
          fillColor={240,240,240},
          fillPattern=FillPattern.Solid,
          lineThickness=0.5),
        Text(
          extent={{-100,40},{100,0}},
          lineColor={0,0,0},
          textString="%name"),
        Text(
          extent={{-100,-50},{100,-90}},
          lineColor={0,0,0},
          textString="V2.0")}));
  protected
    class FMI2ModelExchange
      extends ExternalObject;
        function constructor
          input Integer logLevel;
          input String workingDirectory;
          input String instanceName;
          input Boolean debugLogging;
          output FMI2ModelExchange fmi2me;
          external "C" fmi2me = FMI2ModelExchangeConstructor_OMC(logLevel, workingDirectory, instanceName, debugLogging) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
        end constructor;

        function destructor
          input FMI2ModelExchange fmi2me;
          external "C" FMI2ModelExchangeDestructor_OMC(fmi2me) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
        end destructor;
    end FMI2ModelExchange;

    <%dumpFMITypeDefinitionsMappingFunctions(fmiTypeDefinitionsList)%>

    <%dumpFMITypeDefinitionsArrayMappingFunctions(fmiTypeDefinitionsList)%>

    package fmi2Functions
      function fmi2SetupExperiment
        input FMI2ModelExchange fmi2me;
        input Boolean inToleranceDefined;
        input Real inTolerance;
        input Real inStartTime;
        input Boolean inStopTimeDefined;
        input Real inStopTime;
        input Real inFlow;
        output Real outFlow = inFlow;
        external "C" fmi2SetupExperiment_OMC(fmi2me, inToleranceDefined, inTolerance, inStartTime, inStopTimeDefined, inStopTime) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetupExperiment;

      function fmi2SetTime
        input FMI2ModelExchange fmi2me;
        input Real inTime;
        input Real inFlow;
        output Real outFlow = inFlow;
        external "C" fmi2SetTime_OMC(fmi2me, inTime) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetTime;

      function fmi2EnterInitialization
        input FMI2ModelExchange fmi2me;
        input Real inFlowVariable;
        output Real outFlowVariable = inFlowVariable;
        external "C" fmi2EnterInitializationModel_OMC(fmi2me) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2EnterInitialization;

      function fmi2ExitInitialization
        input FMI2ModelExchange fmi2me;
        input Real inFlowVariable;
        output Real outFlowVariable = inFlowVariable;
        external "C" fmi2ExitInitializationModel_OMC(fmi2me) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2ExitInitialization;

      function fmi2GetContinuousStates
        input FMI2ModelExchange fmi2me;
        input Integer numberOfContinuousStates;
        input Real inFlowParams;
        output Real fmi_x[numberOfContinuousStates];
        external "C" fmi2GetContinuousStates_OMC(fmi2me, numberOfContinuousStates, inFlowParams, fmi_x) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetContinuousStates;

      function fmi2SetContinuousStates
        input FMI2ModelExchange fmi2me;
        input Real fmi_x[:];
        input Real inFlowParams;
        output Real outFlowStates;
        external "C" outFlowStates = fmi2SetContinuousStates_OMC(fmi2me, size(fmi_x, 1), inFlowParams, fmi_x) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetContinuousStates;

      function fmi2GetDerivatives
        input FMI2ModelExchange fmi2me;
        input Integer numberOfContinuousStates;
        input Real inFlowStates;
        output Real fmi_x[numberOfContinuousStates];
        external "C" fmi2GetDerivatives_OMC(fmi2me, numberOfContinuousStates, inFlowStates, fmi_x) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetDerivatives;

      function fmi2GetEventIndicators
        input FMI2ModelExchange fmi2me;
        input Integer numberOfEventIndicators;
        input Real inFlowStates;
        output Real fmi_z[numberOfEventIndicators];
        external "C" fmi2GetEventIndicators_OMC(fmi2me, numberOfEventIndicators, inFlowStates, fmi_z) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetEventIndicators;

      function fmi2GetReal
        input FMI2ModelExchange fmi2me;
        input Real realValuesReferences[:];
        input Real inFlowStatesInput;
        output Real realValues[size(realValuesReferences, 1)];
        external "C" fmi2GetReal_OMC(fmi2me, size(realValuesReferences, 1), realValuesReferences, inFlowStatesInput, realValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetReal;

      function fmi2SetReal
        input FMI2ModelExchange fmi2me;
        input Real realValueReferences[:];
        input Real realValues[size(realValueReferences, 1)];
        output Real outValues[size(realValueReferences, 1)] = realValues;
        external "C" fmi2SetReal_OMC(fmi2me, size(realValueReferences, 1), realValueReferences, realValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetReal;

      function fmi2SetRealParameter
        input FMI2ModelExchange fmi2me;
        input Real realValueReferences[:];
        input Real realValues[size(realValueReferences, 1)];
        output Real out_Value = 1;
        external "C" fmi2SetReal_OMC(fmi2me, size(realValueReferences, 1), realValueReferences, realValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetRealParameter;

      function fmi2GetInteger
        input FMI2ModelExchange fmi2me;
        input Real integerValueReferences[:];
        input Real inFlowStatesInput;
        output Integer integerValues[size(integerValueReferences, 1)];
        external "C" fmi2GetInteger_OMC(fmi2me, size(integerValueReferences, 1), integerValueReferences, inFlowStatesInput, integerValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetInteger;

      function fmi2SetInteger
        input FMI2ModelExchange fmi2me;
        input Real integerValuesReferences[:];
        input Integer integerValues[size(integerValuesReferences, 1)];
        output Integer outValues[size(integerValuesReferences, 1)] = integerValues;
        external "C" fmi2SetInteger_OMC(fmi2me, size(integerValuesReferences, 1), integerValuesReferences, integerValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetInteger;

      function fmi2SetIntegerParameter
        input FMI2ModelExchange fmi2me;
        input Real integerValuesReferences[:];
        input Integer integerValues[size(integerValuesReferences, 1)];
        output Real out_Value = 1;
        external "C" fmi2SetInteger_OMC(fmi2me, size(integerValuesReferences, 1), integerValuesReferences, integerValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetIntegerParameter;

      function fmi2GetBoolean
        input FMI2ModelExchange fmi2me;
        input Real booleanValuesReferences[:];
        input Real inFlowStatesInput;
        output Boolean booleanValues[size(booleanValuesReferences, 1)];
        external "C" fmi2GetBoolean_OMC(fmi2me, size(booleanValuesReferences, 1), booleanValuesReferences, inFlowStatesInput, booleanValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetBoolean;

      function fmi2SetBoolean
        input FMI2ModelExchange fmi2me;
        input Real booleanValueReferences[:];
        input Boolean booleanValues[size(booleanValueReferences, 1)];
        output Boolean outValues[size(booleanValueReferences, 1)] = booleanValues;
        external "C" fmi2SetBoolean_OMC(fmi2me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetBoolean;

      function fmi2SetBooleanParameter
        input FMI2ModelExchange fmi2me;
        input Real booleanValueReferences[:];
        input Boolean booleanValues[size(booleanValueReferences, 1)];
        output Real out_Value = 1;
        external "C" fmi2SetBoolean_OMC(fmi2me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetBooleanParameter;

      function fmi2GetString
        input FMI2ModelExchange fmi2me;
        input Real stringValuesReferences[:];
        input Real inFlowStatesInput;
        output String stringValues[size(stringValuesReferences, 1)];
        external "C" fmi2GetString_OMC(fmi2me, size(stringValuesReferences, 1), stringValuesReferences, inFlowStatesInput, stringValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2GetString;

      function fmi2SetString
        input FMI2ModelExchange fmi2me;
        input Real stringValueReferences[:];
        input String stringValues[size(stringValueReferences, 1)];
        output String outValues[size(stringValueReferences, 1)] = stringValues;
        external "C" fmi2SetString_OMC(fmi2me, size(stringValueReferences, 1), stringValueReferences, stringValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetString;

      function fmi2SetStringParameter
        input FMI2ModelExchange fmi2me;
        input Real stringValueReferences[:];
        input String stringValues[size(stringValueReferences, 1)];
        output Real out_Value = 1;
        external "C" fmi2SetString_OMC(fmi2me, size(stringValueReferences, 1), stringValueReferences, stringValues, 1) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2SetStringParameter;

      function fmi2EventUpdate
        input FMI2ModelExchange fmi2me;
        output Boolean outNewStatesAvailable;
        external "C" outNewStatesAvailable = fmi2EventUpdate_OMC(fmi2me) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2EventUpdate;

      function fmi2nextEventTime
        input FMI2ModelExchange fmi2me;
        input Real inFlowStates;
        output Real outNewnextTime;
        external "C" outNewnextTime = fmi2nextEventTime_OMC(fmi2me, inFlowStates) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2nextEventTime;

      function fmi2CompletedIntegratorStep
        input FMI2ModelExchange fmi2me;
        input Real inFlowStates;
        output Boolean outCallEventUpdate;
        external "C" outCallEventUpdate = fmi2CompletedIntegratorStep_OMC(fmi2me, inFlowStates) annotation(Library = {"OpenModelicaFMIRuntimeC", "fmilib"});
      end fmi2CompletedIntegratorStep;
    end fmi2Functions;
  end <%if stringEq(name, "") then fmiInfo.fmiModelIdentifier+"_"+getFMIType(fmiInfo)+"_FMU" else name%>;
  >>
end importFMU2ModelExchange;

// TODO AHeu: Refactor this function!
template dumpFMI2Variable(ModelVariables fmiModelVariable, String type, String variabilityCausality, Boolean dependent, Integer what)
::=
  if boolAnd(stringEq(type, "real"), (boolAnd(stringEq(variabilityCausality, "parameter"), boolNot(dependent)))) then
    match fmiModelVariable
      case REALVARIABLE(causality="parameter", hasStartValue=true) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "integer"), (boolAnd(stringEq(variabilityCausality, "parameter"), boolNot(dependent)))) then
    match fmiModelVariable
      case INTEGERVARIABLE(causality="parameter", hasStartValue=true) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "boolean"), (boolAnd(stringEq(variabilityCausality, "parameter"), boolNot(dependent)))) then
    match fmiModelVariable
      case BOOLEANVARIABLE(causality="parameter", hasStartValue=true) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "string"), (boolAnd(stringEq(variabilityCausality, "parameter"), boolNot(dependent)))) then
    match fmiModelVariable
      case STRINGVARIABLE(causality="parameter", hasStartValue=true) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "real"), (boolAnd(stringEq(variabilityCausality, "parameter"), dependent))) then
    match fmiModelVariable
      case REALVARIABLE(causality="parameter", hasStartValue=false, isFixed=false) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "integer"), (boolAnd(stringEq(variabilityCausality, "parameter"), dependent))) then
    match fmiModelVariable
      case INTEGERVARIABLE(causality="parameter", hasStartValue=false, isFixed=false) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "boolean"), (boolAnd(stringEq(variabilityCausality, "parameter"), dependent))) then
    match fmiModelVariable
      case BOOLEANVARIABLE(causality="parameter", hasStartValue=false, isFixed=false) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "string"), (boolAnd(stringEq(variabilityCausality, "parameter"), dependent))) then
    match fmiModelVariable
      case STRINGVARIABLE(causality="parameter", hasStartValue=false, isFixed=false) then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "real"), stringEq(variabilityCausality, "input")) then
    match fmiModelVariable
      case REALVARIABLE(causality="input") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name else if intEq(what,3) then "fmi_input_"+name

  else if boolAnd(stringEq(type, "integer"), stringEq(variabilityCausality, "input")) then
    match fmiModelVariable
      case INTEGERVARIABLE(causality="input") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name else if intEq(what,3) then "fmi_input_"+name

  else if boolAnd(stringEq(type, "boolean"), stringEq(variabilityCausality, "input")) then
    match fmiModelVariable
      case BOOLEANVARIABLE(causality="input") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name else if intEq(what,3) then "fmi_input_"+name

  else if boolAnd(stringEq(type, "string"), stringEq(variabilityCausality, "input")) then
    match fmiModelVariable
      case STRINGVARIABLE(causality="input") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name else if intEq(what,3) then "fmi_input_"+name

  else if boolAnd(stringEq(type, "real"), stringEq(variabilityCausality, "output")) then
    match fmiModelVariable
      case REALVARIABLE(variability = "",causality="") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name
      case REALVARIABLE(variability = "",causality="output") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "integer"), stringEq(variabilityCausality, "output")) then
    match fmiModelVariable
      case INTEGERVARIABLE(variability = "",causality="") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name
      case INTEGERVARIABLE(variability = "",causality="output") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "boolean"), stringEq(variabilityCausality, "output")) then
    match fmiModelVariable
      case BOOLEANVARIABLE(variability = "",causality="") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name
      case BOOLEANVARIABLE(variability = "",causality="output") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

  else if boolAnd(stringEq(type, "string"), stringEq(variabilityCausality, "output")) then
    match fmiModelVariable
      case STRINGVARIABLE(variability = "",causality="") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name
      case STRINGVARIABLE(variability = "",causality="output") then
        if intEq(what,1) then valueReference else if intEq(what,2) then name

end dumpFMI2Variable;

annotation(__OpenModelica_Interface="backend");
end CodegenFMU2;
