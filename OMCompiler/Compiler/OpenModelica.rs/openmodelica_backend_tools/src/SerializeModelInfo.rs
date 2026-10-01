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
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend::PrefixUtil;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference::writeCref;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr as expStr;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Autoconf;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::File::Escape::JSON;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn serialize(mut code: &metamodelica::Ref<SimCode::SimCode>, mut withOperations: bool) -> Result<ArcStr> {
    let mut fileName: ArcStr;
    let (true, __pa0) = (serializeWork(code, withOperations)?) else {
        return Err("pattern mismatch");
    };
    fileName = metamodelica::Own::own(__pa0);
    Ok(fileName)
}

fn serializeWork(mut code: &metamodelica::Ref<SimCode::SimCode>, mut withOperations: bool) -> Result<(bool, ArcStr)> {
    let mut success: bool;
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut file: File::File = File::File(File::noReference())?;
    (success, fileName) = 'mc: {
        let __mc_input = &**code;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCode::SimCode { modelInfo: mi @ SimCode::ModelInfo { .. }, .. } => {
                    let mut eqsName: ArcStr;
                    let mut eqsLst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
                    let mut fileName: ArcStr = fileName.clone();
                    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("omsic"))) {
                        fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*code.fullPathPrefix); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); __mm_s.push_str(&*code.fileNamePrefix); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) };
                    } else {
                        fileName = { let mut __mm_s = String::new(); __mm_s.push_str(&*code.fileNamePrefix); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) };
                    }
                    File::open(file.clone(), fileName.clone(), File::Mode::Write.clone());
                    File::write(file.clone(), literal!("{\"format\":\"Transformational debugger info\",\"version\":1,\n\"info\":{\"name\":"));
                    serializePath(file.clone(), mi.name.clone());
                    File::write(file.clone(), literal!(",\"description\":\""));
                    File::writeEscape(file.clone(), mi.description.clone(), JSON.clone());
                    File::write(file.clone(), literal!("\"},\n\"variables\":{\n"));
                    serializeVars(file.clone(), &mi.vars, withOperations)?;
                    File::write(file.clone(), literal!("\n},\n\"equations\":["));
                    File::write(file.clone(), literal!("{\"eqIndex\":0,\"tag\":\"dummy\"}"));
                    for mut tpl in &*list![(literal!("initial"), code.initialEquations.clone()), (literal!("initial-lambda0"), code.initialEquations_lambda0.clone()), (literal!("removed-initial"), code.removedInitialEquations.clone()), (literal!("regular"), code.allEquations.clone()), (literal!("synchronous"), SimCodeCodegenUtil::getClockedEquations(&(SimCodeCodegenUtil::getSubPartitions(code.clockedPartitions.clone())?))), (literal!("start"), code.startValueEquations.clone()), (literal!("nominal"), code.nominalValueEquations.clone()), (literal!("min"), code.minValueEquations.clone()), (literal!("max"), code.maxValueEquations.clone()), (literal!("parameter"), code.parameterEquations.clone()), (literal!("assertions"), code.algorithmAndEquationAsserts.clone()), (literal!("inline"), code.inlineEquations.clone()), (literal!("residuals"), List::flatten(SimCodeUtil::getSimCodeDAEModeDataEqns(code.daeModeData.clone())?)?), (literal!("jacobian"), code.jacobianEquations.clone())] {
                        (eqsName, eqsLst) = tpl.clone();
                        for mut eq in &*SimCodeCodegenUtil::sortEqSystems(eqsLst.clone())? {
                            match '__try0: {
                                        unwrap_break_err!(serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&eq), &eqsName, withOperations, 0, false, AssignType::NORMAL.clone()), '__try0);
                                        Ok::<(), &'static str>(())
                            } {
                                        Ok(()) => {}
                                        Err(__try0_err) => {
                                            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SerializeModelInfo.serializeWork failed for section=")); __mm_s.push_str(&*eqsName); __mm_s.push_str(&*literal!(" eqIndex=")); __mm_s.push_str(&*intString(SimCodeCodegenUtil::simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?)); ArcStr::from(__mm_s) }])?;
                                            return Err(__try0_err);
                                        }
                            }
                        }
                    }
                    File::write(file.clone(), literal!("\n],\n\"functions\":["));
                    serializeList(file.clone(), &mi.functions, &move |__a0: File::File, __a1: metamodelica::Ref<SimCodeFunction::Function::Function>| -> metamodelica::Result<_> { ::std::result::Result::Ok(serializeFunction(__a0, &__a1)) }, false, literal!(","))?;
                    File::write(file.clone(), literal!("\n]\n}"));
                    Ok(((true, fileName.clone()), fileName.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            fileName = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("SerializeModelInfo.serialize failed"), metamodelica::sourceInfo!("SimCode/SerializeModelInfo.mo"))?;
                    Ok((false, literal!("")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((success, fileName))
}

fn serializeVars(mut file: File::File, mut vars: &SimCodeVar::SimVars, mut withOperations: bool) -> Result<()> {
    let mut b: bool;
    b = serializeVarsHelp(file.clone(), &vars.stateVars, withOperations, true)?;
    b = serializeVarsHelp(file.clone(), &vars.derivativeVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.algVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.aliasVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.intAlgVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.boolAlgVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.inputVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.intAliasVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.boolAliasVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.paramVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.intParamVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.boolParamVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.stringAlgVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.stringAliasVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.extObjVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.constVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.intConstVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.boolConstVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.stringConstVars, withOperations, b)?;
    b = serializeVarsHelp(file.clone(), &vars.jacobianVars, withOperations, b)?;
    serializeVarsHelp(file, &vars.sensitivityVars, withOperations, b)?;
    Ok(())
}

fn serializeVarsHelp(
    mut file: File::File,
    mut vars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut withOperations: bool,
    mut inFirst: bool,
) -> Result<bool> {
    let mut outFirst: bool = inFirst && (vars).is_empty();
    serializeList(
        file,
        vars,
        &({
            let __pe_b2 = withOperations;
            move |__pe_a0, __pe_a1| serializeVar(__pe_a0, &__pe_a1, __pe_b2.clone())
        }),
        !(inFirst),
        literal!(",\n"),
    )?;
    Ok(outFirst)
}

fn serializeVar(
    mut file: File::File,
    mut var: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut withOperations: bool,
) -> Result<()> {
    File::write(file.clone(), literal!("\""));
    writeCref(file.clone(), var.name.clone(), JSON.clone())?;
    File::write(file.clone(), literal!("\":{\"comment\":\""));
    File::writeEscape(file.clone(), var.comment.clone(), JSON.clone());
    File::write(file.clone(), literal!("\",\"kind\":\""));
    File::write(file.clone(), varKindString(&var.varKind, var)?);
    File::write(file.clone(), literal!("\""));
    serializeTypeName(file.clone(), &var.type_);
    File::write(file.clone(), literal!(",\"unit\":\""));
    File::writeEscape(file.clone(), var.unit.clone(), JSON.clone());
    File::write(file.clone(), literal!("\",\"displayUnit\":\""));
    File::writeEscape(file.clone(), var.displayUnit.clone(), JSON.clone());
    File::write(file.clone(), literal!("\",\"source\":"));
    serializeSource(file.clone(), &var.source, withOperations)?;
    File::write(file.clone(), literal!(",\"index\":"));
    File::writeInt(file.clone(), var.index.clone(), literal!("%d"));
    let () = (match var.aliasvar.clone() {
        SimCodeVar::AliasVariable::ALIAS { varName: ref cr } => {
            File::write(file.clone(), literal!(",\"alias\":\""));
            writeCref(file.clone(), cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\""));
            ()
        }
        SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref cr } => {
            File::write(file.clone(), literal!(",\"alias\":\"-"));
            writeCref(file.clone(), cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\""));
            ()
        }
        _ => (),
    });
    File::write(file, literal!("}"));
    Ok(())
}

fn serializeTypeName(mut file: File::File, mut ty: &metamodelica::Ref<DAE::Type>) -> () {
    let () = (match &**ty {
        DAE::Type::T_REAL { .. } => {
            File::write(file, literal!(",\"type\":\"Real\""));
            ()
        }
        DAE::Type::T_INTEGER { .. } => {
            File::write(file, literal!(",\"type\":\"Integer\""));
            ()
        }
        DAE::Type::T_BOOL { .. } => {
            File::write(file, literal!(",\"type\":\"Boolean\""));
            ()
        }
        DAE::Type::T_STRING { .. } => {
            File::write(file, literal!(",\"type\":\"String\""));
            ()
        }
        DAE::Type::T_ENUMERATION { .. } => {
            File::write(file, literal!(",\"type\":\"Enumeration\""));
            ()
        }
        _ => (),
    });
    ()
}

fn serializeSource(
    mut file: File::File,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut withOperations: bool,
) -> Result<()> {
    let mut info: SourceInfo;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut typeLst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut partOfLst: metamodelica::List<Absyn::Within>;
    let mut instance: metamodelica::Ref<DAE::ComponentPrefix>;
    let mut operations: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let __arc5 = &(*source);
    let DAE::SOURCE {
        typeLst: __pa0,
        info: __pa1,
        instance: __pa2,
        partOfLst: __pa3,
        operations: __pa4,
        ..
    } = &**__arc5;
    typeLst = metamodelica::Own::own(__pa0);
    info = metamodelica::Own::own(__pa1);
    instance = metamodelica::Own::own(__pa2);
    partOfLst = metamodelica::Own::own(__pa3);
    operations = metamodelica::Own::own(__pa4);
    File::write(file.clone(), literal!("{"));
    serializeInfo(file.clone(), &info);
    if !((partOfLst).is_empty()) {
        paths = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
            for mut w in (partOfLst).into_iter().cloned() {
                if !(match w.clone() {
                    Absyn::Within::TOP { .. } => false,
                    _ => true,
                }) {
                    continue;
                }
                let __x = (match w.clone() {
                    Absyn::Within::WITHIN { .. } => var_field!(w.path, Absyn::Within::WITHIN).clone(),
                    _ => return Err("match: no arm matched"),
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        File::write(file.clone(), literal!(",\"within\":["));
        serializeList(
            file.clone(),
            &paths,
            &fnptr!(serializePath, File::File, metamodelica::Ref<Absyn::Path>),
            false,
            literal!(","),
        )?;
        File::write(file.clone(), literal!("]"));
    }
    let () = (match &*instance {
        DAE::ComponentPrefix::NOCOMPPRE { .. } => (),
        DAE::ComponentPrefix::PRE { .. } => {
            File::write(file.clone(), literal!(",\"instance\":\""));
            PrefixUtil::writeComponentPrefix(file.clone(), &instance, JSON.clone())?;
            File::write(file.clone(), literal!("\""));
            ()
        }
    });
    if !((typeLst).is_empty()) {
        File::write(file.clone(), literal!(",\"typeLst\":["));
        serializeList(
            file.clone(),
            &typeLst,
            &fnptr!(serializePath, File::File, metamodelica::Ref<Absyn::Path>),
            false,
            literal!(","),
        )?;
        File::write(file.clone(), literal!("]"));
    }
    if withOperations && !((operations).is_empty()) {
        File::write(file.clone(), literal!(",\"operations\":["));
        serializeList(file.clone(), &operations, &serializeOperation, false, literal!(","))?;
        File::write(file.clone(), literal!("]"));
    }
    File::write(file, literal!("}"));
    Ok(())
}

fn serializeInfo(mut file: File::File, mut info: &SourceInfo) -> () {
    File::write(file.clone(), literal!("\"info\":{\"file\":\""));
    File::writeEscape(file.clone(), info.fileName.clone(), JSON.clone());
    File::write(file.clone(), literal!("\",\"lineStart\":"));
    File::writeInt(file.clone(), info.lineNumberStart.clone(), literal!("%d"));
    File::write(file.clone(), literal!(",\"lineEnd\":"));
    File::writeInt(file.clone(), info.lineNumberEnd.clone(), literal!("%d"));
    File::write(file.clone(), literal!(",\"colStart\":"));
    File::writeInt(file.clone(), info.columnNumberStart.clone(), literal!("%d"));
    File::write(file.clone(), literal!(",\"colEnd\":"));
    File::writeInt(file.clone(), info.columnNumberEnd.clone(), literal!("%d"));
    File::write(file, literal!("}"));
    ()
}

fn serializeOperation(mut file: File::File, mut op: metamodelica::Ref<DAE::SymbolicOperation>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(op.clone()) {
        Deref @ DAE::SymbolicOperation::FLATTEN { dae: Some(elt), scode: __op_scode } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"flattening\",\"data\":[\""));
            File::writeEscape(file.clone(), System::trim(SCodeDump::equationStr(__op_scode.clone(), SCodeDump::defaultOptions.clone())?, literal!(" \u{c}\n\r\t\u{b}")), JSON.clone());
            File::write(file.clone(), literal!("\",\""));
            File::writeEscape(file.clone(), System::trim(DAEDump::dumpEquationStr(metamodelica::AsArg::as_arg(&elt)), literal!(" \u{c}\n\r\t\u{b}")), JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::FLATTEN { scode: __op_scode, .. } => {
            File::write(file.clone(), literal!("{\"op\":\"info\",\"display\":\"scode\",\"data\":[\""));
            File::writeEscape(file.clone(), System::trim(SCodeDump::equationStr(__op_scode.clone(), SCodeDump::defaultOptions.clone())?, literal!(" \u{c}\n\r\t\u{b}")), JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::SIMPLIFY { after: __op_after, before: __op_before } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"simplify\",\"data\":[\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_before))?;
            File::write(file.clone(), literal!("\",\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_after))?;
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_INLINE { after: __op_after, before: __op_before } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"inline\",\"data\":[\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_before))?;
            File::write(file.clone(), literal!("\",\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_after))?;
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVE { assertConds: Deref @ metamodelica::ListNode::Nil, cr: __op_cr, exp1: __op_exp1, exp2: __op_exp2, res: __op_res } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"solved\",\"data\":[\""));
            File::writeEscape(file.clone(), expStr(__op_exp1.clone())?, JSON.clone());
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_exp2.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\",\""));
            writeCref(file.clone(), __op_cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_res.clone())?, JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVE { assertConds: __op_assertConds, cr: __op_cr, exp1: __op_exp1, exp2: __op_exp2, res: __op_res } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after-assert\",\"display\":\"solved\",\"data\":[\""));
            File::writeEscape(file.clone(), expStr(__op_exp1.clone())?, JSON.clone());
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_exp2.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\",\""));
            writeCref(file.clone(), __op_cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_res.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\""));
            serializeList(file.clone(), metamodelica::AsArg::as_arg(&__op_assertConds), &serializeExp, true, literal!(","))?;
            File::write(file, literal!("]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_RESIDUAL { e: __op_e, e1: __op_e1, e2: __op_e2 } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"residual\",\"data\":["));
            File::writeEscape(file.clone(), expStr(__op_e1.clone())?, JSON.clone());
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_e2.clone())?, JSON.clone());
            File::write(file.clone(), literal!(",\"0 = "));
            File::writeEscape(file.clone(), expStr(__op_e.clone())?, JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::SUBSTITUTION { source: __op_source, substitutions: __op_substitutions } => {
            File::write(file.clone(), literal!("{\"op\":\"chain\",\"display\":\"substitution\",\"data\":[\""));
            File::writeEscape(file.clone(), expStr(__op_source.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\""));
            serializeList(file.clone(), metamodelica::AsArg::as_arg(&__op_substitutions), &serializeExp, true, literal!(","))?;
            File::write(file, literal!("]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::SOLVED { cr: __op_cr, exp: __op_exp } => {
            File::write(file.clone(), literal!("{\"op\":\"info\",\"display\":\"solved\",\"data\":[\""));
            writeCref(file.clone(), __op_cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file.clone(), expStr(__op_exp.clone())?, JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_DIFFERENTIATE { after: __op_after, before: __op_before, cr: __op_cr } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"differentiate d/d"));
            writeCref(file.clone(), __op_cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\",\"data\":[\""));
            File::writeEscape(file.clone(), expStr(__op_before.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\",\""));
            File::writeEscape(file.clone(), expStr(__op_after.clone())?, JSON.clone());
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::OP_SCALARIZE { after: __op_after, before: __op_before, index: __op_index } => {
            File::write(file.clone(), literal!("{\"op\":\"before-after\",\"display\":\"scalarize ["));
            File::write(file.clone(), intString(__op_index.clone()));
            File::write(file.clone(), literal!("]\",\"data\":[\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_before))?;
            File::write(file.clone(), literal!("\",\""));
            writeEqExpStr(file.clone(), metamodelica::AsArg::as_arg(&__op_after))?;
            File::write(file, literal!("\"]}"));
            ()
        },
        Deref @ DAE::SymbolicOperation::NEW_DUMMY_DER { candidates: __op_candidates, chosen: __op_chosen } => {
            File::write(file.clone(), literal!("{\"op\":\"dummy-der\",\"display\":\"dummy derivative"));
            File::write(file.clone(), literal!("\",\"data\":[\""));
            writeCref(file.clone(), __op_chosen.clone(), File::Escape::None.clone())?;
            File::write(file.clone(), literal!("\""));
            serializeList(file.clone(), metamodelica::AsArg::as_arg(&__op_candidates), &serializeCref, true, literal!(","))?;
            File::write(file, literal!("]}"));
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeOperation failed: ")); __mm_s.push_str(&*anyString(op)); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SerializeModelInfo.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum AssignType {
    NORMAL = 1,
    TORN = 2,
    JACOBIAN = 3,
}
impl PartialOrd for AssignType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AssignType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for AssignType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

fn tagFromAssignType(mut assignType: AssignType) -> Result<ArcStr> {
    let mut tag: ArcStr;
    tag = (match assignType {
        AssignType::NORMAL => literal!("assign"),
        AssignType::TORN => literal!("torn"),
        AssignType::JACOBIAN => literal!("jacobian"),
    });
    Ok(tag)
}

fn serializeEquation(
    mut file: File::File,
    mut eq: &metamodelica::Ref<SimCode::SimEqSystem>,
    mut section: &ArcStr,
    mut withOperations: bool,
    mut parent: i32,
    mut first: bool,
    mut assign_type: AssignType,
) -> Result<()> {
    if !(first) {
        File::write(file.clone(), literal!(","));
    }
    let () = (::match_deref::match_deref! { match eq {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"residual\",\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"residual\",\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"residual\",\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { exp: __eq_exp, index: __eq_index, lhs: __eq_lhs, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            writeCref(file.clone(), Expression::expCref(metamodelica::AsArg::as_arg(&__eq_lhs))?, JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, alternativeTearing: None, .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            i = ((lSystem.beqs).len() as i32);
            j = ((lSystem.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(lSystem.residual.clone())?;
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if (eqs).is_empty() && (jeqs).is_empty() {
                File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            } else {
                File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            }
            File::writeInt(file.clone(), lSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList(file.clone(), &lSystem.simJac, &({ let __pe_b2 = withOperations; move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, &__pe_a1, __pe_b2.clone()) }), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &lSystem.beqs, &serializeExp, false, literal!(","))?;
            File::write(file, literal!("]}]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, alternativeTearing: Some(atL @ Deref @ SimCode::LinearSystem { .. }), .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            i = ((lSystem.beqs).len() as i32);
            j = ((lSystem.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(lSystem.residual.clone())?;
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if (eqs).is_empty() && (jeqs).is_empty() {
                File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            } else {
                File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            }
            File::writeInt(file.clone(), lSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList(file.clone(), &lSystem.simJac, &({ let __pe_b2 = withOperations; move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, &__pe_a1, __pe_b2.clone()) }), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &lSystem.beqs, &serializeExp, false, literal!(","))?;
            File::write(file.clone(), literal!("]}]},"));
            i = ((atL.beqs).len() as i32);
            j = ((atL.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(atL.residual.clone())?;
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, atL.index.clone(), true, if (atL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atL.index.clone(), false, if (atL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(atL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, atL.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atL.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            if (eqs).is_empty() && (jeqs).is_empty() {
                File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            } else {
                File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            }
            File::writeInt(file.clone(), atL.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if atL.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (atL.vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList(file.clone(), &atL.simJac, &({ let __pe_b2 = withOperations; move |__pe_a0, __pe_a1| serializeLinearCell(__pe_a0, &__pe_a1, __pe_b2.clone()) }), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &atL.beqs, &serializeExp, false, literal!(","))?;
            File::write(file, literal!("]}]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: stmt @ Deref @ DAE::Statement::STMT_ASSIGN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, index: __eq_index, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*section); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            writeCref(file.clone(), Expression::expCref(var_field!((**stmt).exp1, DAE::Statement::STMT_ASSIGN))?, JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!((**stmt).exp, DAE::Statement::STMT_ASSIGN).clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeList(file.clone(), var_field!((**eq).statements, SimCode::SimEqSystem::SES_ALGORITHM), &fnptr!(serializeStatement, File::File, metamodelica::Ref<DAE::Statement>), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), &(Algorithm::getStatementSource(metamodelica::AsArg::as_arg(&stmt))?), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: _ }, index: __eq_index, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*section); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"equation\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), var_field!((**eq).statements, SimCode::SimEqSystem::SES_ALGORITHM), &fnptr!(serializeStatement, File::File, metamodelica::Ref<DAE::Statement>), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), &(Algorithm::getStatementSource(metamodelica::AsArg::as_arg(&stmt))?), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: _ }, index: __eq_index, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*section); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"equation\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), var_field!((**eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM), &fnptr!(serializeStatement, File::File, metamodelica::Ref<DAE::Statement>), false, literal!(","))?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), &(Algorithm::getStatementSource(metamodelica::AsArg::as_arg(&stmt))?), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, alternativeTearing: None, .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            eqs = SimCodeCodegenUtil::sortEqSystems(nlSystem.eqs.clone())?;
            for mut e in &*eqs {
                match '__try0: {
                    unwrap_break_err!(serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), unwrap_break_err!(SimCodeCodegenUtil::simEqSystemIndex(metamodelica::AsArg::as_arg(&e)), '__try0) == unwrap_break_err!(SimCodeCodegenUtil::simEqSystemIndex(&(unwrap_break_err!((eqs).head().cloned(), '__try0))), '__try0), if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()}), '__try0);
                    Ok::<(), &'static str>(())
                } {
                    Ok(()) => {}
                    Err(__try0_err) => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SerializeModelInfo inner eq failed in NLS ")); __mm_s.push_str(&*intString(nlSystem.index.clone())); __mm_s.push_str(&*literal!(" for inner eqIndex=")); __mm_s.push_str(&*intString(SimCodeCodegenUtil::simEqSystemIndex(metamodelica::AsArg::as_arg(&e))?)); ArcStr::from(__mm_s) }])?;
                        return Err(__try0_err);
                    }
                }
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), nlSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &nlSystem.crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file, literal!("]]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, alternativeTearing: Some(atNL @ Deref @ SimCode::NonlinearSystem { .. }), .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            eqs = SimCodeCodegenUtil::sortEqSystems(nlSystem.eqs.clone())?;
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), nlSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &nlSystem.crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file.clone(), literal!("]]},"));
            eqs = SimCodeCodegenUtil::sortEqSystems(atNL.eqs.clone())?;
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, atNL.index.clone(), true, if (atNL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atNL.index.clone(), false, if (atNL.tornSystem.clone()) {AssignType::TORN.clone()} else {AssignType::NORMAL.clone()})?;
            }
            jeqs = (::match_deref::match_deref! { match &(atNL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, atNL.index.clone(), true, AssignType::JACOBIAN.clone())?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atNL.index.clone(), false, AssignType::JACOBIAN.clone())?;
                }
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), atNL.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if atNL.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atNL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeList(file.clone(), &atNL.crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1), false, literal!(","))?;
            File::write(file, literal!("]]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { elsebranch: __eq_elsebranch, ifbranches: __eq_ifbranches, index: __eq_index, .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            eqs = listAppend(List::flatten(({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> = metamodelica::nil();
        for mut e in (__eq_ifbranches.clone()).into_iter().cloned() {
            let __x = Util::tuple22(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, __eq_elsebranch.clone());
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, 0, true, AssignType::NORMAL.clone())?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, AssignType::NORMAL.clone())?;
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"if-equation\",\"display\":\"if-equation\",\"equation\":["));
            serializeList(file.clone(), metamodelica::AsArg::as_arg(&__eq_ifbranches), &move |__a0: File::File, __a1: (metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)| serializeIfBranch(__a0, &__a1), false, literal!(","))?;
            File::write(file.clone(), literal!(","));
            serializeIfBranch(file.clone(), &((metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }), __eq_elsebranch.clone())))?;
            File::write(file, literal!("]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { cont: __eq_cont, discEqs: __eq_discEqs, discVars: __eq_discVars, index: __eq_index, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&__eq_cont), section, withOperations, 0, true, AssignType::NORMAL.clone())?;
            for mut e in &*__eq_discEqs.clone() {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, AssignType::NORMAL.clone())?;
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"container\",\"display\":\"mixed\",\"defines\":["));
            serializeList(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (__eq_discVars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"uses\":["));
            crefs = metamodelica::nil();
            serializeList(file.clone(), &crefs, &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeEquationIndex(file.clone(), metamodelica::AsArg::as_arg(&__eq_cont))?;
            for mut e1 in &*__eq_discEqs.clone() {
                File::write(file.clone(), literal!(","));
                serializeEquationIndex(file.clone(), metamodelica::AsArg::as_arg(&e1))?;
            }
            File::write(file, literal!("]}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: __eq_conditions, elseWhen: __eq_elseWhen, index: __eq_index, source: __eq_source, whenStmtLst: __eq_whenStmtLst, .. } => {
            let mut whenOp: BackendDAE::WhenOperator;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            for mut whenOps in &*__eq_whenStmtLst.clone() {
                let () = (match whenOps.clone() {
        mut __esc_whenOp @ BackendDAE::WhenOperator::ASSIGN { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\",\"defines\":["));
            serializeExp(file.clone(), var_field!(whenOp.left, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeList(file.clone(), &(getWhenUses(__eq_conditions.clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::REINIT { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\",\"defines\":["));
            serializeCref(file.clone(), var_field!(whenOp.stateVar, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeList(file.clone(), &(getWhenUses(__eq_conditions.clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::ASSERT { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            crefs = Expression::extractCrefsFromExpDerPreStart(var_field!(whenOp.condition, BackendDAE::WhenOperator::ASSERT).clone(), true)?;
            serializeList(file.clone(), &(getWhenUses(crefs, var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::TERMINATE { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            serializeList(file.clone(), &(getWhenUses(__eq_conditions.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::NORETCALL { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            serializeList(file.clone(), &(getWhenUses(__eq_conditions.clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file.clone(), literal!("}"));
            ()
        },
    });
            }
            let () = (::match_deref::match_deref! { match &(__eq_elseWhen.clone()) {
        Some(e) => {
            if SimCodeCodegenUtil::simEqSystemIndex(metamodelica::AsArg::as_arg(&e))? != 0 {
                serializeEquation(file, metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, AssignType::NORMAL.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\",\"tag\":\"")); __mm_s.push_str(&*tagFromAssignType(assign_type)?); __mm_s.push_str(&*literal!("\",\"defines\":[\"")); ArcStr::from(__mm_s) });
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeList(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?), &serializeCref, false, literal!(","))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: __eq_aliasOf, index: __eq_index } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            File::write(file.clone(), literal!(",\"tag\":\"alias\",\"equation\":["));
            File::writeInt(file.clone(), __eq_aliasOf.clone(), literal!("%d"));
            File::write(file.clone(), literal!("],\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file, literal!("\"}"));
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeEquation failed: ")); __mm_s.push_str(&*anyString(eq.clone())); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SerializeModelInfo.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeLinearCell(
    mut file: File::File,
    mut cell: &(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>),
    mut withOperations: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cell) {
        (i, j, eq @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. }) => {
            File::write(file.clone(), literal!("{\"row\":"));
            File::write(file.clone(), intString(i.clone()));
            File::write(file.clone(), literal!(",\"column\":"));
            File::write(file.clone(), intString(j.clone()));
            File::write(file.clone(), literal!(",\"exp\":\""));
            File::writeEscape(file.clone(), expStr(var_field!((**eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone())?, JSON.clone());
            File::write(file.clone(), literal!("\",\"source\":"));
            serializeSource(file.clone(), var_field!((**eq).source, SimCode::SimEqSystem::SES_RESIDUAL), withOperations)?;
            File::write(file, literal!("}"));
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("SerializeModelInfo.serializeLinearCell failed. Expected only SES_RESIDUAL as input.")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn varKindString(mut varKind: &BackendDAE::VarKind, mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match varKind.clone() {
        BackendDAE::VarKind::VARIABLE { .. } => literal!("variable"),
        BackendDAE::VarKind::STATE { .. } => literal!("state"),
        BackendDAE::VarKind::STATE_DER { .. } => literal!("derivative"),
        BackendDAE::VarKind::DUMMY_DER { .. } => literal!("dummy derivative"),
        BackendDAE::VarKind::DUMMY_STATE { .. } => literal!("dummy state"),
        BackendDAE::VarKind::CLOCKED_STATE { .. } => literal!("clocked state"),
        BackendDAE::VarKind::DISCRETE { .. } => literal!("discrete"),
        BackendDAE::VarKind::PARAM { .. } => literal!("parameter"),
        BackendDAE::VarKind::CONST { .. } => literal!("constant"),
        BackendDAE::VarKind::EXTOBJ { .. } => literal!("external object"),
        BackendDAE::VarKind::JAC_VAR { .. } => literal!("jacobian variable"),
        BackendDAE::VarKind::JAC_TMP_VAR { .. } => literal!("jacobian differentiated variable"),
        BackendDAE::VarKind::OPT_CONSTR { .. } => literal!("constraint"),
        BackendDAE::VarKind::OPT_FCONSTR { .. } => literal!("final constraint"),
        BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. } => literal!("use derivation of input"),
        BackendDAE::VarKind::OPT_INPUT_DER { .. } => literal!("derivation of input"),
        BackendDAE::VarKind::OPT_TGRID { .. } => literal!("time grid for optimization"),
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => literal!("variable for transform loop in constraint"),
        BackendDAE::VarKind::ALG_STATE { .. } => literal!("helper variable transform ode for symSolver"),
        BackendDAE::VarKind::ALG_STATE_OLD { .. } => literal!("helper variable transform ode for symSolver"),
        BackendDAE::VarKind::LOOP_ITERATION { .. } => literal!("iteration variable for solving an algebraic loop"),
        BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. } => literal!("residual variable for dae mode"),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("SerializeModelInfo.varKindString"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*SimCodeCodegenUtil::simVarString(var)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(r#str)
}

fn getWhenUses(
    mut conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut value: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut uses: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    uses = listAppend(conditions, Expression::extractCrefsFromExpDerPreStart(value, true)?);
    uses = UnorderedSet::unique_list(
        uses,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::hashComponentRef(&__a0)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(uses)
}

fn serializeStatement(mut file: File::File, mut stmt: metamodelica::Ref<DAE::Statement>) -> () {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(
        file.clone(),
        System::trim(DAEDump::ppStatementStr(stmt), literal!(" \u{c}\n\r\t\u{b}")),
        JSON.clone(),
    );
    File::write(file, literal!("\""));
    ()
}

fn serializeList<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut file: File::File,
    mut lst: &metamodelica::List<ArgType>,
    mut func: &dyn ::std::ops::Fn(File::File, ArgType) -> Result<()>,
    mut append: bool,
    mut sep: ArcStr,
) -> Result<()> {
    pub type FuncType<ArgType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(File::File, ArgType) -> Result<()> + 'static>;

    if !((lst).is_empty()) {
        if append {
            File::write(file.clone(), sep.clone());
        }
        func(file.clone(), (lst).head().cloned()?)?;
        for mut a in &*(lst).rest()? {
            File::write(file.clone(), sep.clone());
            func(file.clone(), a.clone())?;
        }
    }
    Ok(())
}

fn serializeExp(mut file: File::File, mut exp: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(file.clone(), expStr(exp)?, JSON.clone());
    File::write(file, literal!("\""));
    Ok(())
}

fn serializeCref(mut file: File::File, mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<()> {
    File::write(file.clone(), literal!("\""));
    writeCref(file.clone(), cr, JSON.clone())?;
    File::write(file, literal!("\""));
    Ok(())
}

fn serializeString(mut file: File::File, mut string: ArcStr) -> () {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(file.clone(), string, JSON.clone());
    File::write(file, literal!("\""));
    ()
}

fn serializePath(mut file: File::File, mut path: metamodelica::Ref<Absyn::Path>) -> () {
    let mut p: metamodelica::Ref<Absyn::Path> = path;
    let mut b: bool = true;
    File::write(file.clone(), literal!("\""));
    while b {
        (p, b) = (match &*p {
            Absyn::Path::IDENT { name: __p_name } => {
                File::writeEscape(file.clone(), __p_name.clone(), JSON.clone());
                (p, false)
            }
            Absyn::Path::QUALIFIED {
                name: __p_name,
                path: __p_path,
            } => {
                File::writeEscape(file.clone(), __p_name.clone(), JSON.clone());
                File::write(file.clone(), literal!("."));
                (__p_path.clone(), true)
            }
            Absyn::Path::FULLYQUALIFIED { path: __p_path } => (__p_path.clone(), true),
        });
    }
    File::write(file, literal!("\""));
    ()
}

fn serializeEquationIndex(mut file: File::File, mut eq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<()> {
    File::writeInt(file, SimCodeCodegenUtil::simEqSystemIndex(eq)?, literal!("%d"));
    Ok(())
}

fn serializeIfBranch(
    mut file: File::File,
    mut branch: &(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    ),
) -> Result<()> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    (exp, eqs) = branch.clone();
    File::write(file.clone(), literal!("["));
    serializeExp(file.clone(), exp)?;
    serializeList(
        file.clone(),
        &eqs,
        &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1),
        true,
        literal!(","),
    )?;
    File::write(file, literal!("]"));
    Ok(())
}

fn writeEqExpStr(mut file: File::File, mut eqExp: &metamodelica::Ref<DAE::EquationExp>) -> Result<()> {
    let () = (match &**eqExp {
        DAE::EquationExp::PARTIAL_EQUATION { exp: __eqExp_exp } => {
            File::writeEscape(file, expStr(__eqExp_exp.clone())?, JSON.clone());
            ()
        }
        DAE::EquationExp::RESIDUAL_EXP { exp: __eqExp_exp } => {
            File::write(file.clone(), literal!("0 = "));
            File::writeEscape(file, expStr(__eqExp_exp.clone())?, JSON.clone());
            ()
        }
        DAE::EquationExp::EQUALITY_EXPS {
            lhs: __eqExp_lhs,
            rhs: __eqExp_rhs,
        } => {
            File::writeEscape(file.clone(), expStr(__eqExp_lhs.clone())?, JSON.clone());
            File::write(file.clone(), literal!(" = "));
            File::writeEscape(file, expStr(__eqExp_rhs.clone())?, JSON.clone());
            ()
        }
    });
    Ok(())
}

fn serializeFunction(mut file: File::File, mut func: &metamodelica::Ref<SimCodeFunction::Function::Function>) -> () {
    File::write(file.clone(), literal!("\n"));
    serializePath(file, SimCodeUtil::functionPath(func));
    ()
}
