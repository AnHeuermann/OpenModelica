// Auto-generated from MetaModelica source
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
#![allow(warnings)]
#![allow(
    unreachable_patterns,
    unreachable_code,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports,
    unused_variables,
    non_upper_case_globals,
    unused_mut
)]

use arcstr::{ArcStr, format, literal};
use const_str;
use loop_unwrap::unwrap_break_err;
use metamodelica::Result;
use metamodelica::*; // Built-in types and functions
use std::sync::Arc;

use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE::VarKind;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend_base::ComponentReference as CR;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Exp;
use openmodelica_frontend_types::DAE::Type;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCode::ModelInfo;
use openmodelica_simcode_types::SimCode::SimCode as SIMCODE;
use openmodelica_simcode_types::SimCode::SimulationSettings;
use openmodelica_simcode_types::SimCode::VarInfo;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_types::SimCodeVar::AliasVariable;
use openmodelica_simcode_types::SimCodeVar::Causality;
use openmodelica_simcode_types::SimCodeVar::SimVar;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::File::Escape::XML;
use openmodelica_util::Settings;
use openmodelica_util::Util;

pub fn simulationInitFile(mut simCode: &metamodelica::Ref<SIMCODE>, mut guid: ArcStr) -> Result<()> {
    let true = (simulationInitFileReturnBool(simCode, guid)?) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

pub fn simulationInitFileReturnBool(mut simCode: &metamodelica::Ref<SIMCODE>, mut guid: ArcStr) -> Result<bool> {
    let mut success: bool = false;
    let mut vi: VarInfo;
    let mut s: SimulationSettings;
    let mut file: File::File = File::File(File::noReference())?;
    let mut fileName: ArcStr;
    let mut FMUType: ArcStr;
    if '__try0: {
        fileName = (::match_deref::match_deref! { match &(unwrap_break_err!(Config::simCodeTarget(), '__try0)) {
        Deref @ "omsic" => { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) },
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        File::open(file.clone(), fileName.clone(), File::Mode::Write.clone());
        vi = simCode.modelInfo.varInfo.clone();
        let __pa1 = ::match_deref::match_deref! { match &(simCode.simulationSettingsOpt.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        s = metamodelica::Own::own(__pa1);
        FMUType = (::match_deref::match_deref! { match &(unwrap_break_err!(Config::simCodeTarget(), '__try0)) {
        Deref @ "omsic" => literal!("2.0"),
        Deref @ "omsicpp" => literal!("2.0"),
        _ => literal!("1.0"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        File::write(file.clone(), literal!("<?xml version = \"1.0\" encoding=\"UTF-8\"?>\n\n"));
        File::write(file.clone(), literal!("<!-- description of the model interface using an extention of the FMI standard -->\n"));
        File::write(file.clone(), literal!("<fmiModelDescription\n"));
        File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  fmiVersion                          = \"")); __mm_s.push_str(&*FMUType); __mm_s.push_str(&*literal!("\"\n\n")); ArcStr::from(__mm_s) });
        File::write(file.clone(), literal!("  modelName                           = \""));
        unwrap_break_err!(Dump::writePath(file.clone(), simCode.modelInfo.name.clone(), File::Escape::None.clone(), literal!("."), false), '__try0);
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("  modelIdentifier                     = \""));
        unwrap_break_err!(Dump::writePath(file.clone(), simCode.modelInfo.name.clone(), File::Escape::None.clone(), literal!("_"), false), '__try0);
        File::write(file.clone(), literal!("\"\n\n"));
        File::write(file.clone(), literal!("  OPENMODELICAHOME                    = \""));
        File::write(file.clone(), simCode.makefileParams.omhome.clone());
        File::write(file.clone(), literal!("\"\n\n"));
        File::write(file.clone(), literal!("  guid                                = \"{"));
        File::write(file.clone(), guid.clone());
        File::write(file.clone(), literal!("}\"\n\n"));
        File::write(file.clone(), literal!("  description                         = \""));
        File::writeEscape(file.clone(), simCode.modelInfo.description.clone(), XML.clone());
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("  generationTool                      = \"OpenModelica Compiler "));
        File::write(file.clone(), Settings::getVersionNr());
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("  generationDateAndTime               = \""));
        xsdateTime(file.clone(), Util::getCurrentDateTime());
        File::write(file.clone(), literal!("\"\n\n"));
        File::write(file.clone(), literal!("  variableNamingConvention            = \"structured\"\n\n"));
        File::write(file.clone(), literal!("  numberOfEventIndicators             = \""));
        File::writeInt(file.clone(), vi.numZeroCrossings.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfEventIndicators             = \"NG:       number of zero crossings,                           FMI\"\n"));
        File::write(file.clone(), literal!("  numberOfTimeEvents                  = \""));
        File::writeInt(file.clone(), vi.numTimeEvents.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfTimeEvents                  = \"NG_SAM:   number of zero crossings that are samples,          OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfInputVariables              = \""));
        File::writeInt(file.clone(), vi.numInVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfInputVariables              = \"NI:       number of inputvar on topmodel,                     OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfOutputVariables             = \""));
        File::writeInt(file.clone(), vi.numOutVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfOutputVariables             = \"NO:       number of outputvar on topmodel,                    OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfExternalObjects             = \""));
        File::writeInt(file.clone(), vi.numExternalObjects.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfExternalObjects             = \"NEXT:     number of external objects,                         OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfFunctions                   = \""));
        File::writeInt(file.clone(), ((simCode.modelInfo.functions).len() as i32), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfFunctions                   = \"NFUNC:    number of functions used by the simulation,         OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfContinuousStates            = \""));
        File::writeInt(file.clone(), vi.numStateVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfContinuousStates            = \"NX:       number of states,                                   FMI\"\n"));
        File::write(file.clone(), literal!("  numberOfRealAlgebraicVariables      = \""));
        File::writeInt(file.clone(), vi.numAlgVars.clone() + vi.numDiscreteReal.clone() + vi.numOptimizeConstraints.clone() + vi.numOptimizeFinalConstraints.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfRealAlgebraicVariables      = \"NY:       number of real variables,                           OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfRealAlgebraicAliasVariables = \""));
        File::writeInt(file.clone(), vi.numAlgAliasVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfRealAlgebraicAliasVariables = \"NA:       number of alias variables,                          OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfRealParameters              = \""));
        File::writeInt(file.clone(), vi.numParams.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfRealParameters              = \"NP:       number of parameters,                               OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfIntegerAlgebraicVariables   = \""));
        File::writeInt(file.clone(), vi.numIntAlgVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfIntegerAlgebraicVariables   = \"NYINT:    number of alg. int variables,                       OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfIntegerAliasVariables       = \""));
        File::writeInt(file.clone(), vi.numIntAliasVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfIntegerAliasVariables       = \"NAINT:    number of alias int variables,                      OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfIntegerParameters           = \""));
        File::writeInt(file.clone(), vi.numIntParams.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfIntegerParameters           = \"NPINT:    number of int parameters,                           OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfStringAlgebraicVariables    = \""));
        File::writeInt(file.clone(), vi.numStringAlgVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfStringAlgebraicVariables    = \"NYSTR:    number of alg. string variables,                    OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfStringAliasVariables        = \""));
        File::writeInt(file.clone(), vi.numStringAliasVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfStringAliasVariables        = \"NASTR:    number of alias string variables,                   OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfStringParameters            = \""));
        File::writeInt(file.clone(), vi.numStringParamVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfStringParameters            = \"NPSTR:    number of string parameters,                        OMC\"\n\n"));
        File::write(file.clone(), literal!("  numberOfBooleanAlgebraicVariables   = \""));
        File::writeInt(file.clone(), vi.numBoolAlgVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfBooleanAlgebraicVariables   = \"NYBOOL:   number of alg. bool variables,                      OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfBooleanAliasVariables       = \""));
        File::writeInt(file.clone(), vi.numBoolAliasVars.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfBooleanAliasVariables       = \"NABOOL:   number of alias bool variables,                     OMC\"\n"));
        File::write(file.clone(), literal!("  numberOfBooleanParameters           = \""));
        File::writeInt(file.clone(), vi.numBoolParams.clone(), literal!("%d"));
        File::write(file.clone(), literal!("\"  cmt_numberOfBooleanParameters           = \"NPBOOL:   number of bool parameters,                          OMC\" >\n\n\n"));
        File::write(file.clone(), literal!("  <!-- startTime, stopTime, tolerance are FMI specific, all others are OMC specific -->\n"));
        File::write(file.clone(), literal!("  <DefaultExperiment\n"));
        File::write(file.clone(), literal!("    startTime      = \""));
        File::writeReal(file.clone(), s.startTime.clone(), literal!("%.15g"));
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    stopTime       = \""));
        File::writeReal(file.clone(), s.stopTime.clone(), literal!("%.15g"));
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    stepSize       = \""));
        File::writeReal(file.clone(), s.stepSize.clone(), literal!("%.15g"));
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    tolerance      = \""));
        File::writeReal(file.clone(), s.tolerance.clone(), literal!("%.15g"));
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    solver         = \""));
        File::write(file.clone(), s.method.clone());
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    outputFormat   = \""));
        File::write(file.clone(), s.outputFormat.clone());
        File::write(file.clone(), literal!("\"\n"));
        File::write(file.clone(), literal!("    variableFilter = \""));
        File::write(file.clone(), s.variableFilter.clone());
        File::write(file.clone(), literal!("\" />\n\n"));
        File::write(file.clone(), literal!("  <!-- variables in the model -->\n"));
        File::write(file.clone(), literal!("  <ModelVariables>\n\n"));
        unwrap_break_err!(modelVariables(file.clone(), &simCode.modelInfo.vars), '__try0);
        File::write(file.clone(), literal!("\n\n\n  </ModelVariables>\n\n"));
        File::write(file.clone(), literal!("\n</fmiModelDescription>\n\n"));
        success = true;
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(success)
}

fn modelVariables(mut file: File::File, mut vars: &SimCodeVar::SimVars) -> Result<()> {
    let mut vr: i32;
    let mut ix: i32 = 0;
    vr = (::match_deref::match_deref! { match &(Config::simCodeTarget()?) {
        Deref @ "omsic" => 0,
        Deref @ "omsicpp" => 0,
        _ => 1000,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (vr, _) = scalarVariables(file.clone(), &vars.stateVars, literal!("rSta"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.derivativeVars, literal!("rDer"), vr, 0)?;
    (vr, ix) = scalarVariables(file.clone(), &vars.algVars, literal!("rAlg"), vr, ix)?;
    (vr, ix) = scalarVariables(file.clone(), &vars.discreteAlgVars, literal!("rAlg"), vr, ix)?;
    (vr, ix) = scalarVariables(
        file.clone(),
        &vars.realOptimizeConstraintsVars,
        literal!("rAlg"),
        vr,
        ix,
    )?;
    (vr, ix) = scalarVariables(
        file.clone(),
        &vars.realOptimizeFinalConstraintsVars,
        literal!("rAlg"),
        vr,
        ix,
    )?;
    (vr, _) = scalarVariables(file.clone(), &vars.paramVars, literal!("rPar"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.aliasVars, literal!("rAli"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.intAlgVars, literal!("iAlg"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.intParamVars, literal!("iPar"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.intAliasVars, literal!("iAli"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.boolAlgVars, literal!("bAlg"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.boolParamVars, literal!("bPar"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.boolAliasVars, literal!("bAli"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.stringAlgVars, literal!("sAlg"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.stringParamVars, literal!("sPar"), vr, 0)?;
    (vr, _) = scalarVariables(file.clone(), &vars.stringAliasVars, literal!("sAli"), vr, 0)?;
    (vr, _) = scalarVariables(file, &vars.sensitivityVars, literal!("rSen"), vr, 0)?;
    Ok(())
}

fn scalarVariables(
    mut file: File::File,
    mut vars: &metamodelica::List<metamodelica::Ref<SimVar>>,
    mut classType: ArcStr,
    mut valueReference: i32,
    mut index: i32,
) -> Result<(i32, i32)> {
    let mut valueReference: i32 = valueReference;
    let mut index: i32 = index;
    for mut var in &**vars {
        scalarVariable(file.clone(), var.clone(), classType.clone(), valueReference, index)?;
        index = index + 1;
        valueReference = valueReference + 1;
    }
    Ok((valueReference, index))
}

fn scalarVariable(
    mut file: File::File,
    mut var: metamodelica::Ref<SimVar>,
    mut classType: ArcStr,
    mut valueReference: i32,
    mut classIndex: i32,
) -> Result<()> {
    let mut type_name: ArcStr = if (DAEUtil::expTypeArray(&var.type_)) {
        literal!("ArrayVariable")
    } else {
        literal!("ScalarVariable")
    };
    File::write(file.clone(), {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  <"));
        __mm_s.push_str(&*type_name);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    scalarVariableAttribute(file.clone(), var.clone(), classType, valueReference, classIndex)?;
    File::write(file.clone(), literal!("    "));
    scalarVariableType(file.clone(), &var)?;
    File::write(file, {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n  </"));
        __mm_s.push_str(&*type_name);
        __mm_s.push_str(&*literal!(">\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn scalarVariableAttribute(
    mut file: File::File,
    mut simVar: metamodelica::Ref<SimVar>,
    mut classType: ArcStr,
    mut valueReference: i32,
    mut classIndex: i32,
) -> Result<()> {
    let mut inputIndex: i32 = SimCodeCodegenUtil::getInputIndex(&simVar)?;
    let mut info: SourceInfo = simVar.source.info.clone();
    File::write(file.clone(), literal!("    name = \""));
    CR::writeCref(file.clone(), simVar.name.clone(), XML.clone())?;
    File::write(file.clone(), literal!("\"\n"));
    File::write(file.clone(), literal!("    valueReference = \""));
    File::writeInt(file.clone(), valueReference, literal!("%d"));
    File::write(file.clone(), literal!("\"\n"));
    if !metamodelica::stringEq(&simVar.comment, &(literal!(""))) {
        File::write(file.clone(), literal!("    description = \""));
        File::writeEscape(file.clone(), simVar.comment.clone(), XML.clone());
        File::write(file.clone(), literal!("\"\n"));
    }
    File::write(file.clone(), literal!("    variability = \""));
    File::write(file.clone(), getVariablity(&simVar.varKind));
    File::write(file.clone(), literal!("\" isDiscrete = \""));
    File::write(
        file.clone(),
        ArcStr::from(::std::format!("{}", simVar.isDiscrete.clone())),
    );
    File::write(file.clone(), literal!("\"\n"));
    File::write(file.clone(), literal!("    causality = \""));
    File::write(file.clone(), getCausality(simVar.causality.clone()));
    File::write(file.clone(), literal!("\" isValueChangeable = \""));
    File::write(
        file.clone(),
        ArcStr::from(::std::format!("{}", simVar.isValueChangeable.clone())),
    );
    File::write(file.clone(), literal!("\"\n"));
    if inputIndex != -1 {
        File::write(file.clone(), literal!("    inputIndex = \""));
        File::writeInt(file.clone(), inputIndex, literal!("%d"));
        File::write(file.clone(), literal!("\"\n"));
    }
    File::write(file.clone(), literal!("    alias = "));
    getAliasVar(file.clone(), simVar.clone())?;
    File::write(file.clone(), literal!("\n"));
    File::write(file.clone(), literal!("    classIndex = \""));
    File::writeInt(file.clone(), classIndex, literal!("%d"));
    File::write(file.clone(), literal!("\" classType = \""));
    File::write(file.clone(), classType);
    File::write(file.clone(), literal!("\"\n"));
    File::write(file.clone(), literal!("    isProtected = \""));
    File::write(
        file.clone(),
        ArcStr::from(::std::format!("{}", simVar.isProtected.clone())),
    );
    File::write(file.clone(), literal!("\" hideResult = \""));
    File::write(
        file.clone(),
        Util::applyOptionOrDefault(simVar.hideResult.clone(), &fnptr!(boolString, bool), literal!(""))?,
    );
    File::write(file.clone(), literal!("\" isEncrypted = \""));
    File::write(file.clone(), boolString(simVar.isEncrypted.clone()));
    File::write(file.clone(), literal!("\" initNonlinear = \""));
    File::write(file.clone(), boolString(simVar.initNonlinear.clone()));
    File::write(file.clone(), literal!("\"\n"));
    File::write(file.clone(), literal!("    fileName = \""));
    File::writeEscape(file.clone(), info.fileName.clone(), XML.clone());
    File::write(file.clone(), literal!("\" startLine = \""));
    File::writeInt(file.clone(), info.lineNumberStart.clone(), literal!("%d"));
    File::write(file.clone(), literal!("\" startColumn = \""));
    File::writeInt(file.clone(), info.columnNumberStart.clone(), literal!("%d"));
    File::write(file.clone(), literal!("\" endLine = \""));
    File::writeInt(file.clone(), info.lineNumberEnd.clone(), literal!("%d"));
    File::write(file.clone(), literal!("\" endColumn = \""));
    File::writeInt(file.clone(), info.columnNumberEnd.clone(), literal!("%d"));
    File::write(file.clone(), literal!("\" fileWritable = \""));
    File::write(
        file.clone(),
        ArcStr::from(::std::format!("{}", !(info.isReadOnly.clone()))),
    );
    File::write(file.clone(), literal!("\">\n"));
    for mut dim in &*Expression::arrayDimension(&simVar.type_) {
        File::write(file.clone(), {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("    <Dimension start=\""));
            __mm_s.push_str(&*intString(Expression::dimensionSize(metamodelica::AsArg::as_arg(
                &dim,
            ))?));
            __mm_s.push_str(&*literal!("\"/>\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn scalarVariableType(mut file: File::File, mut v: &metamodelica::Ref<SimVar>) -> Result<()> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let () = (match &*(Types::arrayElementType(&v.type_)) {
        Type::T_INTEGER { .. } => {
            File::write(file.clone(), literal!("<Integer"));
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), literal!("start"));
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeAttribute(file.clone(), v.minValue.clone(), literal!("min"));
            scalarVariableTypeAttribute(file.clone(), v.maxValue.clone(), literal!("max"));
            scalarVariableTypeStringAttribute(file.clone(), v.unit.clone(), literal!("unit"));
            scalarVariableTypeStringAttribute(file.clone(), v.displayUnit.clone(), literal!("displayUnit"));
            File::write(file, literal!(" />"));
            ()
        }
        Type::T_REAL { .. } => {
            File::write(file.clone(), literal!("<Real"));
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), literal!("start"));
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeUseAttribute(
                file.clone(),
                v.nominalValue.clone(),
                literal!("useNominal"),
                literal!("nominal"),
            );
            scalarVariableTypeAttribute(file.clone(), v.minValue.clone(), literal!("min"));
            scalarVariableTypeAttribute(file.clone(), v.maxValue.clone(), literal!("max"));
            scalarVariableTypeStringAttribute(file.clone(), v.unit.clone(), literal!("unit"));
            scalarVariableTypeStringAttribute(file.clone(), v.displayUnit.clone(), literal!("displayUnit"));
            if v.relativeQuantity.clone() {
                File::write(file.clone(), literal!(" relativeQuantity=\"true\""));
            }
            File::write(file, literal!(" />"));
            ()
        }
        Type::T_BOOL { .. } => {
            File::write(file.clone(), literal!("<Boolean"));
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), literal!("start"));
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), v.unit.clone(), literal!("unit"));
            scalarVariableTypeStringAttribute(file.clone(), v.displayUnit.clone(), literal!("displayUnit"));
            File::write(file, literal!(" />"));
            ()
        }
        Type::T_STRING { .. } => {
            File::write(file.clone(), literal!("<String"));
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), literal!("start"));
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), v.unit.clone(), literal!("unit"));
            scalarVariableTypeStringAttribute(file.clone(), v.displayUnit.clone(), literal!("displayUnit"));
            File::write(file, literal!(" />"));
            ()
        }
        Type::T_ENUMERATION { .. } => {
            File::write(file.clone(), literal!("<Integer"));
            scalarVariableTypeAttribute(file.clone(), v.initialValue.clone(), literal!("start"));
            scalarVariableTypeFixedAttribute(file.clone(), v.isFixed.clone());
            scalarVariableTypeStringAttribute(file.clone(), v.unit.clone(), literal!("unit"));
            scalarVariableTypeStringAttribute(file.clone(), v.displayUnit.clone(), literal!("displayUnit"));
            File::write(file, literal!(" />"));
            ()
        }
        Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { path: __esc_path },
            ..
        } => {
            path = (*__esc_path).clone();
            File::write(file.clone(), literal!("<ExternalObject path=\""));
            Dump::writePath(file.clone(), path.clone(), XML.clone(), literal!("."), true)?;
            File::write(file, literal!("\" />"));
            ()
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("SerializeInitXML.scalarVariableType"));
                    __mm_s.push_str(&*literal!(": "));
                    __mm_s.push_str(&*TypesDump::unparseType(v.type_.clone())?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("SimCode/SerializeInitXML.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

fn scalarVariableTypeUseAttribute(
    mut file: File::File,
    mut attr: Option<metamodelica::Ref<Exp>>,
    mut r#use: ArcStr,
    mut name: ArcStr,
) -> () {
    File::write(file.clone(), literal!(" "));
    File::write(file.clone(), r#use);
    File::write(file.clone(), literal!("=\""));
    File::write(file.clone(), ArcStr::from(::std::format!("{}", (attr).is_some())));
    File::write(file.clone(), literal!("\""));
    scalarVariableTypeAttribute(file, attr, name);
    ()
}

fn scalarVariableTypeFixedAttribute(mut file: File::File, mut isFixed: bool) -> () {
    File::write(file.clone(), literal!(" fixed=\""));
    File::write(file.clone(), ArcStr::from(::std::format!("{}", isFixed)));
    File::write(file, literal!("\""));
    ()
}

fn scalarVariableTypeAttribute(mut file: File::File, mut attr: Option<metamodelica::Ref<Exp>>, mut name: ArcStr) -> () {
    let mut expStr: ArcStr;
    if '__try0: {
        expStr = unwrap_break_err!(expString(&(unwrap_break_err!(attr.clone().ok_or("pattern mismatch"), '__try0))), '__try0);
        File::write(file.clone(), literal!(" "));
        File::write(file.clone(), name.clone());
        File::write(file.clone(), literal!("=\""));
        File::write(file.clone(), expStr.clone());
        File::write(file.clone(), literal!("\""));
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ()
}

fn scalarVariableTypeStringAttribute(mut file: File::File, mut attr: ArcStr, mut name: ArcStr) -> () {
    if metamodelica::stringEq(&attr, &(literal!(""))) {
        return ();
    }
    File::write(file.clone(), literal!(" "));
    File::write(file.clone(), name);
    File::write(file.clone(), literal!("=\""));
    File::writeEscape(file.clone(), attr, XML.clone());
    File::write(file, literal!("\""));
    ()
}

fn getCausality(mut c: Option<Causality>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match c {
        Some(SimCodeVar::Causality::NONECAUS) => literal!("none"),
        Some(SimCodeVar::Causality::OUTPUT) => literal!("output"),
        Some(SimCodeVar::Causality::INPUT) => literal!("input"),
        Some(SimCodeVar::Causality::LOCAL) => literal!("local"),
        Some(SimCodeVar::Causality::PARAMETER) => literal!("parameter"),
        Some(SimCodeVar::Causality::CALCULATED_PARAMETER) => literal!("calculatedParameter"),
        _ => literal!("local"),
    });
    r#str
}

fn getVariablity(mut varKind: &VarKind) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match varKind.clone() {
        VarKind::DISCRETE => literal!("discrete"),
        VarKind::PARAM => literal!("parameter"),
        VarKind::CONST => literal!("constant"),
        _ => literal!("continuous"),
    });
    r#str
}

fn getAliasVar(mut file: File::File, mut simVar: metamodelica::Ref<SimVar>) -> Result<()> {
    let () = (match &*simVar {
        SimVar {
            aliasvar: aliasvar @ SimCodeVar::AliasVariable::ALIAS { .. },
            ..
        } => {
            File::write(file.clone(), literal!("\"alias\" aliasVariable=\""));
            CR::writeCref(
                file.clone(),
                var_field!(aliasvar.varName, AliasVariable::ALIAS).clone(),
                XML.clone(),
            )?;
            File::write(file.clone(), literal!("\" aliasVariableId=\""));
            File::write(
                file.clone(),
                SimCodeCodegenUtil::getValueReference(simVar, &(SimCodeCodegenUtil::getSimCode()?), true)?,
            );
            File::write(file, literal!("\""));
            ()
        }
        SimVar {
            aliasvar: aliasvar @ SimCodeVar::AliasVariable::NEGATEDALIAS { .. },
            ..
        } => {
            File::write(file.clone(), literal!("\"negatedAlias\" aliasVariable=\""));
            CR::writeCref(
                file.clone(),
                var_field!(aliasvar.varName, AliasVariable::NEGATEDALIAS).clone(),
                XML.clone(),
            )?;
            File::write(file.clone(), literal!("\" aliasVariableId=\""));
            File::write(
                file.clone(),
                SimCodeCodegenUtil::getValueReference(simVar, &(SimCodeCodegenUtil::getSimCode()?), true)?,
            );
            File::write(file, literal!("\""));
            ()
        }
        _ => {
            File::write(file, literal!("\"noAlias\""));
            ()
        }
    });
    Ok(())
}

fn xsdateTime(mut file: File::File, mut dt: Util::DateTime) -> () {
    File::writeInt(file.clone(), dt.year.clone(), literal!("%d"));
    File::writeInt(file.clone(), dt.mon.clone(), literal!("-%02d"));
    File::writeInt(file.clone(), dt.mday.clone(), literal!("-%02d"));
    File::writeInt(file.clone(), dt.hour.clone(), literal!("T%02d"));
    File::writeInt(file.clone(), dt.min.clone(), literal!(":%02d"));
    File::writeInt(file, dt.sec.clone(), literal!(":%02dZ"));
    ()
}

fn expString<'__b>(mut exp: &'__b metamodelica::Ref<Exp>) -> Result<ArcStr> {
    '__tco: loop {
        match &**exp {
            Exp::ICONST { .. } => return Ok(intString(var_field!((**exp).integer, Exp::ICONST).clone())),
            Exp::RCONST { .. } => return Ok(realString(var_field!((**exp).real, Exp::RCONST).clone())),
            Exp::SCONST { .. } => {
                return Ok(Util::escapeModelicaStringToXmlString(
                    var_field!((**exp).string, Exp::SCONST).clone(),
                )?);
            }
            Exp::BCONST { .. } => return Ok(boolString(var_field!((**exp).bool, Exp::BCONST).clone())),
            Exp::ENUM_LITERAL { .. } => return Ok(intString(var_field!((**exp).index, Exp::ENUM_LITERAL).clone())),
            Exp::ARRAY { .. } if (Expression::isSimpleLiteralValue(exp, true)?) => {
                return Ok(stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut e in (var_field!((**exp).array, Exp::ARRAY).clone()).into_iter().cloned() {
                            let __x = arrayElementString(&(e.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(" "),
                ));
            }
            Exp::REDUCTION { .. } => {
                exp = var_field!((**exp).expr, Exp::REDUCTION);
                continue '__tco;
            }
            _ => return Ok(return Err("fail")),
        }
    }
}

fn arrayElementString(mut exp: &metamodelica::Ref<Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**exp {
        Exp::SCONST { string: __exp_string } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("&quot;"));
            __mm_s.push_str(&*Util::escapeModelicaStringToXmlString(__exp_string.clone())?);
            __mm_s.push_str(&*literal!("&quot;"));
            ArcStr::from(__mm_s)
        }
        Exp::ARRAY { array: __exp_array, .. } if (Expression::isSimpleLiteralValue(exp, true)?) => stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut e in (__exp_array.clone()).into_iter().cloned() {
                    let __x = arrayElementString(&(e.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!(" "),
        ),
        _ => expString(exp)?,
    });
    Ok(r#str)
}
