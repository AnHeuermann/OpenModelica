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

use crate::BackendInterfaceImplementation;
use crate::CevalScript;
use crate::CevalScriptBackend;
use crate::Interactive;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_backend::SymbolTable;
use openmodelica_dump_extra::DumpGraphviz;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::AbsynJLDumpTpl;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::FCore;
use openmodelica_loader::Parser;
use openmodelica_program_util::ProgramUtil;
#[cfg(feature = "susan")]
use openmodelica_susan::TplMain;
use openmodelica_tpl::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::Socket;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util::ZeroMQ;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

fn makeDebugResult(mut inFlag: Flags::DebugFlag, mut res: ArcStr) -> ArcStr {
    let mut res_1: ArcStr;
    res_1 = 'mc: {
        let __mc_input = inFlag.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let Flags::DebugFlag { name: mut flagstr, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut debugstr: ArcStr;
            let mut res_with_debug: ArcStr;
            let true = (Flags::isSet(inFlag.clone())?) else {
                return Err("pattern mismatch");
            };
            debugstr = Print::getString()?;
            res_with_debug = stringAppendList(list![
                res.clone(),
                literal!("\n---DEBUG("),
                flagstr.clone(),
                literal!(")---\n"),
                debugstr.clone(),
                literal!("\n---/DEBUG("),
                flagstr.clone(),
                literal!(")---\n")
            ]);
            Ok(res_with_debug.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(res.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res_1
}

fn parseCommand(mut inCommand: ArcStr) -> (Option<GlobalScript::Statements>, Option<Absyn::Program>) {
    let mut outStatements: Option<GlobalScript::Statements>;
    let mut outProgram: Option<Absyn::Program>;
    (outStatements, outProgram) = 'mc: {
        let __mc_input = inCommand.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut stmts: GlobalScript::Statements;
            ErrorExt::setCheckpoint(literal!("parsestring"));
            stmts = Parser::parsestringexp(inCommand.clone(), literal!("<interactive>"))?;
            ErrorExt::delCheckpoint(literal!("parsestring"));
            Ok((Some(stmts.clone()), None))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut prog: Absyn::Program;
            ErrorExt::rollBack(literal!("parsestring"));
            prog = Parser::parsestring(
                inCommand.clone(),
                literal!("<interactive>"),
                Config::acceptedGrammar()?,
                Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
                Flags::getConfigBool(Flags::STRICT.clone())?,
            )?;
            Ok((None, Some(prog.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((None, None))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outStatements, outProgram)
}

pub(crate) fn handleCommand(mut inCommand: ArcStr) -> Result<(bool, ArcStr)> {
    let mut outContinue: bool;
    let mut outResult: ArcStr;
    let mut stmts: Option<GlobalScript::Statements>;
    let mut prog: Option<Absyn::Program>;
    Print::clearBuf();
    if Util::strncmp(literal!("quit()"), inCommand.clone(), 6) {
        outContinue = false;
        outResult = literal!("Ok\n");
    } else {
        outContinue = true;
        (stmts, prog) = parseCommand(inCommand.clone());
        outResult = handleCommand2(stmts, prog, inCommand)?;
        outResult = makeDebugResult(Flags::DUMP.clone(), outResult);
        outResult = makeDebugResult(Flags::DUMP_GRAPHVIZ.clone(), outResult);
    }
    System::reportProgress(-1, 0);
    Ok((outContinue, outResult))
}

fn handleCommand2(
    mut inStatements: Option<GlobalScript::Statements>,
    mut inProgram: Option<Absyn::Program>,
    mut inCommand: ArcStr,
) -> Result<ArcStr> {
    let mut outResult: ArcStr;
    outResult = 'mc: {
        let __mc_input = (inStatements.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(mut stmts), None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut result: ArcStr;
            result = Interactive::evaluate(&(stmts.clone()), false)?;
            Ok(result.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(mut prog)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut prog2: Absyn::Program;
            let mut ast: Absyn::Program;
            let mut result: ArcStr;
            let mut vars: metamodelica::List<InteractiveTypes::Variable>;
            let mut table: metamodelica::Ref<SymbolTable::SymbolTable>;
            table = SymbolTable::get();
            ast = table.ast.clone();
            vars = table.vars.clone();
            prog2 = Interactive::addScope(prog.clone(), vars.clone());
            prog2 = ProgramUtil::updateProgram(prog2.clone(), ast.clone(), false, false)?;
            if Flags::isSet(Flags::DUMP.clone())? {
                Debug::trace(literal!("\n--------------- Parsed program ---------------\n"))?;
                Print::printBuf(Dump::unparseStr(
                    prog2.clone(),
                    false,
                    Dump::defaultDumpOptions.clone(),
                )?)?;
            }
            if Flags::isSet(Flags::DUMP_GRAPHVIZ.clone())? {
                DumpGraphviz::dump(&prog2)?;
            }
            result = makeClassDefResult(&(prog.clone()))?;
            SymbolTable::setAbsyn(prog2.clone())?;
            Ok(result.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut result: ArcStr;
            Print::printBuf(literal!("Error occurred building AST\n"))?;
            result = Print::getString()?;
            result = stringAppend(result.clone(), literal!("Syntax Error\n"));
            result = stringAppend(result.clone(), Error::printMessagesStr(false));
            Ok(result.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut result: ArcStr;
            let true = ((inStatements).is_some() || (inProgram).is_some()) else {
                return Err("pattern mismatch");
            };
            result = Error::printMessagesStr(false);
            Ok(result.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = ((inStatements).is_some() || (inProgram).is_some()) else {
                return Err("pattern mismatch");
            };
            Error::addMessage(Error::STACK_OVERFLOW.clone(), list![inCommand.clone()])?;
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outResult)
}

fn makeClassDefResult(mut p: &Absyn::Program) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (match p.clone() {
        Absyn::Program {
            classes: ref cls,
            within_: Absyn::Within::WITHIN { path: ref scope },
        } => {
            let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            names = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut c in (cls.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: AbsynUtil::className(&(c.clone())),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            names = List::map1(names, &AbsynUtil::joinPaths, scope.clone())?;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut n in (names).into_iter().cloned() {
                            let __x = AbsynUtil::pathString(n.clone(), literal!("."), true, false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            res
        }
        Absyn::Program {
            classes: ref cls,
            within_: Absyn::Within::TOP { .. },
        } => {
            let mut names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            names = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut c in (cls.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: AbsynUtil::className(&(c.clone())),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut n in (names).into_iter().cloned() {
                            let __x = AbsynUtil::pathString(n.clone(), literal!("."), true, false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("}\n"));
                ArcStr::from(__mm_s)
            };
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(res)
}

fn isModelicaFile(mut inFilename: ArcStr) -> Result<bool> {
    let mut outIsModelicaFile: bool;
    let mut lst: metamodelica::List<ArcStr>;
    let mut file_ext: ArcStr;
    lst = System::strtok(inFilename, literal!("."));
    if (lst).is_empty() {
        outIsModelicaFile = false;
    } else {
        file_ext = List::last(&lst)?;
        outIsModelicaFile = metamodelica::stringEq(&file_ext, &(literal!("mo")))
            || metamodelica::stringEq(&file_ext, &(literal!("mof")));
    }
    Ok(outIsModelicaFile)
}

fn isEmptyOrFirstIsModelicaFile(mut libs: &metamodelica::List<ArcStr>) -> Result<()> {
    let () = (::match_deref::match_deref! { match libs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: f, tail: _ } => {
            let true = (isModelicaFile(f.clone())?) else { return Err("pattern mismatch") };
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isFlatModelicaFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: metamodelica::List<ArcStr>;
    let mut last: ArcStr;
    lst = System::strtok(filename, literal!("."));
    let __pa0 = ::match_deref::match_deref! { match &(lst.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    last = metamodelica::Own::own(__pa0);
    let true = (stringEq(&last, &(literal!("mof")))) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

fn isModelicaScriptFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: metamodelica::List<ArcStr>;
    let mut last: ArcStr;
    let true = (System::regularFileExists(filename.clone())) else {
        return Err("pattern mismatch");
    };
    lst = System::strtok(filename, literal!("."));
    let __pa0 = ::match_deref::match_deref! { match &(lst.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    last = metamodelica::Own::own(__pa0);
    let true = (stringEq(&last, &(literal!("mos")))) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

fn isCodegenTemplateFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: metamodelica::List<ArcStr>;
    let mut last: ArcStr;
    lst = System::strtok(filename, literal!("."));
    let __pa0 = ::match_deref::match_deref! { match &(lst.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    last = metamodelica::Own::own(__pa0);
    let true = (stringEq(&last, &(literal!("tpl")))) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

fn showErrors(mut errorString: ArcStr, mut errorMessages: ArcStr) -> () {
    if !metamodelica::stringEq(&errorString, &(literal!(""))) {
        System::fflush();
        System::fputs(errorString, System::StreamType::STDERR.clone());
        System::fputs(literal!("\n"), System::StreamType::STDERR.clone());
        System::fflush();
    }
    if !metamodelica::stringEq(&errorMessages, &(literal!(""))) {
        System::fflush();
        System::fputs(errorMessages, System::StreamType::STDERR.clone());
        System::fputs(literal!("\n"), System::StreamType::STDERR.clone());
        System::fflush();
    }
    ()
}

fn loadLib(mut inLib: ArcStr) -> Result<()> {
    let mut is_modelica_file: bool;
    is_modelica_file = isModelicaFile(inLib.clone())?;
    let () = 'mc: {
        let __mc_input = is_modelica_file;
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut pnew: Absyn::Program;
            let mut p: Absyn::Program;
            p = SymbolTable::getAbsyn();
            pnew = CevalScript::loadFile(inLib.clone(), literal!("UTF-8"), p.clone(), true, true, false, true)?;
            SymbolTable::setAbsyn(pnew.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut mp: ArcStr;
            let mut pnew: Absyn::Program;
            let mut p: Absyn::Program;
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = AbsynUtil::stringPath(inLib.clone())?;
            mp = Settings::getModelicaPath(Testsuite::isRunning()?)?;
            p = SymbolTable::getAbsyn();
            let (__pa0, true) = (CevalScript::loadModel(
                &(list![(
                    path.clone(),
                    literal!("command-line argument"),
                    list![literal!("default")],
                    false
                )]),
                mp.clone(),
                p.clone(),
                true,
                true,
                true,
                false,
                false,
                literal!(""),
            )?) else {
                return Err("pattern mismatch");
            };
            pnew = metamodelica::Own::own(__pa0);
            SymbolTable::setAbsyn(pnew.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else {
                return Err("nomatch");
            };
            Print::printErrorBuf({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to load library: "));
                __mm_s.push_str(&*inLib);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else {
                return Err("nomatch");
            };
            Print::printErrorBuf({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Failed to parse file: "));
                __mm_s.push_str(&*inLib);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn translateFile(mut inStringLst: metamodelica::List<ArcStr>) -> Result<()> {
    let mut f: ArcStr;
    let mut libs: metamodelica::List<ArcStr>;
    let mut cname: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut runBackend: bool = false;
    let mut runSilent: bool = false;
    let mut stmts: GlobalScript::Statements = <GlobalScript::Statements as ::std::default::Default>::default();
    let mut cls: ArcStr = arcstr::literal!("");
    let mut fileNamePrefix: ArcStr = arcstr::literal!("");
    let mut fmuVersion: ArcStr = arcstr::literal!("");
    let mut fmuType: ArcStr = arcstr::literal!("");
    let mut fmuPlatformsStr: ArcStr = arcstr::literal!("");
    let mut fmuVersionInt: i32 = 0;
    let mut fmuPlatforms: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut fmuTypeList: metamodelica::List<ArcStr> = metamodelica::nil();
    if !(stringEmpty(&(Flags::getConfigString(Flags::EXECUTE_COMMAND.clone())?))) {
        stmts = Parser::parsestringexp(
            Flags::getConfigString(Flags::EXECUTE_COMMAND.clone())?,
            literal!("<interactive>"),
        )?;
        showErrors(Print::getErrorString()?, ErrorExt::printMessagesStr(false));
        Interactive::evaluateToStdOut(&stmts, true)?;
        if (inStringLst).is_empty() && stringEmpty(&(Config::classToInstantiate()?)) {
            return Ok(());
        }
    }
    let () = 'mc: {
        let __mc_input = inStringLst;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            libs => {
                                let mut cls: ArcStr = cls.clone();
                                let mut cname: metamodelica::Ref<Absyn::Path> = cname.clone();
                                let mut fileNamePrefix: ArcStr = fileNamePrefix.clone();
                                let mut fmuPlatforms: metamodelica::List<ArcStr> = fmuPlatforms.clone();
                                let mut fmuPlatformsStr: ArcStr = fmuPlatformsStr.clone();
                                let mut fmuType: ArcStr = fmuType.clone();
                                let mut fmuTypeList: metamodelica::List<ArcStr> = fmuTypeList.clone();
                                let mut fmuVersion: ArcStr = fmuVersion.clone();
                                let mut fmuVersionInt: i32 = fmuVersionInt.clone();
                                let mut runBackend: bool = runBackend.clone();
                                let mut runSilent: bool = runSilent.clone();
                                isEmptyOrFirstIsModelicaFile(metamodelica::AsArg::as_arg(&libs))?;
                                execStatReset()?;
                                for mut lib in &*libs.clone() {
                                    loadLib(lib.clone())?;
                                }
                                if Flags::isSet(Flags::DUMP.clone())? {
                                    Debug::trace(literal!("\n--------------- Parsed program ---------------\n"))?;
                                    Dump::unparseStr(SymbolTable::getAbsyn(), false, Dump::defaultDumpOptions.clone())?;
                                    metamodelica::print(Print::getString()?);
                                }
                                if Flags::isSet(Flags::DUMP_JL.clone())? {
                                    Debug::trace(literal!("\n--------------- Julia representation of the parsed program ---------------\n"))?;
                                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: Absyn::Program| AbsynJLDumpTpl::dump(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                }
                                if Flags::isSet(Flags::DUMP_GRAPHVIZ.clone())? {
                                    DumpGraphviz::dump(&(SymbolTable::getAbsyn()))?;
                                }
                                execStat(&(literal!("Parsed file")))?;
                                cls = Config::classToInstantiate()?;
                                cname = if (stringEmpty(&cls)) {AbsynUtil::lastClassname(SymbolTable::getAbsyn())?} else {AbsynUtil::stringPath(cls.clone())?};
                                fileNamePrefix = Util::stringReplaceChar(AbsynUtil::pathString(cname.clone(), literal!("."), true, false)?, literal!("."), literal!("_"))?;
                                if Flags::getConfigBool(Flags::EXPORT_FMU.clone())? {
                                    fmuVersionInt = Flags::getConfigEnum(Flags::FMU_VERSION.clone())?;
                                    fmuVersion = (match fmuVersionInt {
                    10 => literal!("1.0"),
                    20 => literal!("2.0"),
                    30 => literal!("3.0"),
                    _ => return Err("match: no arm matched"),
                });
                                    FlagsUtil::setConfigString(Flags::FMI_VERSION.clone(), fmuVersion.clone())?;
                                    fmuTypeList = Flags::getConfigStringList(Flags::FMU_TYPE.clone())?;
                                    fmuType = stringDelimitList(fmuTypeList.clone(), literal!("_"));
                                    fmuPlatformsStr = Flags::getConfigString(Flags::FMU_PLATFORMS.clone())?;
                                    fmuPlatforms = Util::stringSplitAtChar(fmuPlatformsStr.clone(), literal!(","))?;
                                    CevalScriptBackend::callBuildModelFMU(FCore::emptyCache(), FGraph::empty(), cname.clone(), fmuVersion.clone(), fmuType.clone(), fileNamePrefix.clone(), true, fmuPlatforms.clone(), None)?;
                                    showErrors(Print::getErrorString()?, ErrorExt::printMessagesStr(false));
                                    System::fflush();
                                } else {
                                    runBackend = Config::simulationCg()? || Config::simulation()?;
                                    runSilent = Config::silent()?;
                                    CevalScriptBackend::translateModel(FCore::emptyCache(), FGraph::empty(), cname.clone(), fileNamePrefix.clone(), runBackend, runSilent, None)?;
                                    showErrors(Print::getErrorString()?, ErrorExt::printMessagesStr(false));
                                }
                                Ok(((), cls.clone(), cname.clone(), fileNamePrefix.clone(), fmuPlatforms.clone(), fmuPlatformsStr.clone(), fmuType.clone(), fmuTypeList.clone(), fmuVersion.clone(), fmuVersionInt.clone(), runBackend.clone(), runSilent.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })()
        {
            cls = __wb0;
            cname = __wb1;
            fileNamePrefix = __wb2;
            fmuPlatforms = __wb3;
            fmuPlatformsStr = __wb4;
            fmuType = __wb5;
            fmuTypeList = __wb6;
            fmuVersion = __wb7;
            fmuVersionInt = __wb8;
            runBackend = __wb9;
            runSilent = __wb10;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: f, tail: libs } => {
                    let mut stmts: GlobalScript::Statements = stmts.clone();
                    isModelicaScriptFile(f.clone())?;
                    for mut lib in &*libs.clone() {
                        loadLib(lib.clone())?;
                    }
                    stmts = Parser::parseexp(f.clone())?;
                    showErrors(Print::getErrorString()?, ErrorExt::printMessagesStr(false));
                    Interactive::evaluateToStdOut(&stmts, true)?;
                    Ok(((), stmts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            stmts = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: f, tail: Deref @ metamodelica::ListNode::Nil } => {
                    isCodegenTemplateFile(f.clone())?;
                    { #[cfg(feature = "susan")] { TplMain::main(f.clone(), &(Flags::getConfigString(Flags::TPL_OUTPUT_DIR.clone())?), literal!(""))? } #[cfg(not(feature = "susan"))] { { let _ = ::openmodelica_util::Error::addInternalError(::arcstr::literal!("TplMain.main needs the 'susan' code generation target, which this OpenModelica build was compiled without"), metamodelica::sourceInfo!("Main/Main.mo")); return Err("TplMain.main needs the 'susan' code generation target, which this OpenModelica build was compiled without") } } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: f, tail: _ } => {
                    if System::regularFileExists(f.clone()) {
                        metamodelica::print(literal!("Error processing file: "));
                    } else {
                        metamodelica::print(literal!("File does not exist: "));
                    }
                    metamodelica::print(f.clone());
                    metamodelica::print(literal!("\n"));
                    System::fflush();
                    showErrors(Print::getErrorString()?, ErrorExt::printMessagesStr(false));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn interactivemode() -> Result<()> {
    let mut shandle: i32;
    let mut b: bool;
    let mut r#str: ArcStr;
    let mut replystr: ArcStr;
    shandle = Socket::waitforconnect(29500);
    if shandle == -1 {
        return Err("fail");
    }
    loop {
        r#str = Socket::handlerequest(shandle);
        if Flags::isSet(Flags::INTERACTIVE_DUMP.clone())? {
            Debug::trace(literal!("------- Recieved Data from client -----\n"))?;
            Debug::trace(r#str.clone())?;
            Debug::trace(literal!("------- End recieved Data-----\n"))?;
        }
        (b, replystr) = handleCommand(r#str)?;
        replystr = if (b) {
            replystr
        } else {
            literal!("quit requested, shutting server down\n")
        };
        Socket::sendreply(shandle, replystr);
        if !(b) {
            Socket::close(shandle);
            Socket::cleanup();
            break;
        }
    }
    Ok(())
}

fn interactivemodeZMQ() -> Result<()> {
    let mut zmqSocket: Option<i32>;
    let mut b: bool;
    let mut r#str: ArcStr;
    let mut replystr: ArcStr;
    let mut suffix: ArcStr;
    suffix = Flags::getConfigString(Flags::ZEROMQ_FILE_SUFFIX.clone())?;
    zmqSocket = ZeroMQ::initialize(
        if (metamodelica::stringEq(&suffix, &(literal!("")))) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*suffix);
                ArcStr::from(__mm_s)
            }
        },
        Flags::isSet(Flags::ZMQ_LISTEN_TO_ALL.clone())?,
        Flags::getConfigInt(Flags::INTERACTIVE_PORT.clone())?,
    );
    let false = (Some(0) == zmqSocket.clone()) else {
        return Err("pattern mismatch");
    };
    loop {
        r#str = ZeroMQ::handleRequest(zmqSocket.clone());
        if Flags::isSet(Flags::INTERACTIVE_DUMP.clone())? {
            Debug::trace(literal!("------- Recieved Data from client -----\n"))?;
            Debug::trace(r#str.clone())?;
            Debug::trace(literal!("------- End recieved Data-----\n"))?;
        }
        (b, replystr) = handleCommand(r#str)?;
        replystr = if (b) {
            replystr
        } else {
            literal!("quit requested, shutting server down\n")
        };
        ZeroMQ::sendReply(zmqSocket.clone(), replystr);
        if !(b) {
            ZeroMQ::close(zmqSocket);
            break;
        }
    }
    Ok(())
}

pub(crate) fn readSettings(mut inArguments: metamodelica::List<ArcStr>) -> Result<()> {
    let mut settings_file: ArcStr;
    settings_file = Util::flagValue(&(literal!("-s")), inArguments)?;
    if !metamodelica::stringEq(&settings_file, &(literal!(""))) {
        settings_file = System::trim(settings_file, literal!(" \""));
        readSettingsFile(settings_file)?;
    }
    Ok(())
}

fn readSettingsFile(mut filePath: ArcStr) -> Result<()> {
    let mut command: ArcStr;
    if System::regularFileExists(filePath.clone()) {
        command = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("runScript(\""));
            __mm_s.push_str(&*filePath);
            __mm_s.push_str(&*literal!("\")"));
            ArcStr::from(__mm_s)
        };
        handleCommand(command)?;
    }
    Ok(())
}

pub(crate) fn setWindowsPaths(mut inOMHome: ArcStr) -> Result<()> {
    let () = (match inOMHome {
        mut omHome => {
            let mut oldPath: ArcStr;
            let mut newPath: ArcStr;
            let mut omdevPath: ArcStr;
            let mut msysPath: ArcStr;
            let mut mingwDir: ArcStr;
            let mut binDir: ArcStr;
            let mut libBinDir: ArcStr;
            let mut msysBinDir: ArcStr;
            let mut omLibDir: ArcStr;
            let mut hasBinDir: bool;
            let mut hasLibBinDir: bool;
            let mut isMSVC: bool;
            System::setEnv(literal!("OPENMODELICAHOME"), omHome.clone(), true);
            omdevPath = Util::makeValueOrDefault(&System::readEnv, literal!("OMDEV"), literal!(""));
            if stringEq(&omdevPath, &(literal!(""))) {
                omdevPath = omHome.clone();
            }
            msysPath = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*omdevPath);
                __mm_s.push_str(&*literal!("\\tools\\msys"));
                ArcStr::from(__mm_s)
            };
            mingwDir = System::openModelicaPlatform();
            msysBinDir = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*msysPath);
                __mm_s.push_str(&*literal!("\\usr\\bin"));
                ArcStr::from(__mm_s)
            };
            binDir = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*msysPath);
                __mm_s.push_str(&*literal!("\\"));
                __mm_s.push_str(&*mingwDir);
                __mm_s.push_str(&*literal!("\\bin"));
                ArcStr::from(__mm_s)
            };
            isMSVC = 0 == System::stringFind(mingwDir.clone(), literal!("msvc"))?;
            if isMSVC {
                libBinDir = binDir.clone();
            } else if metamodelica::stringEq(&(System::getCCompiler()), &(literal!("gcc"))) {
                libBinDir = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*msysPath);
                    __mm_s.push_str(&*literal!("\\"));
                    __mm_s.push_str(&*mingwDir);
                    __mm_s.push_str(&*literal!("\\lib\\gcc\\"));
                    __mm_s.push_str(&*System::gccDumpMachine());
                    __mm_s.push_str(&*literal!("\\"));
                    __mm_s.push_str(&*System::gccVersion());
                    ArcStr::from(__mm_s)
                };
            } else {
                libBinDir = binDir.clone();
            }
            hasBinDir = System::directoryExists(binDir.clone());
            hasLibBinDir = System::directoryExists(libBinDir.clone());
            omLibDir = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*omHome);
                __mm_s.push_str(&*literal!("\\lib\\"));
                __mm_s.push_str(&*arcstr::literal!(Autoconf::triple));
                __mm_s.push_str(&*literal!("\\omc"));
                ArcStr::from(__mm_s)
            };
            if isMSVC {
                oldPath = System::readEnv(literal!("PATH"))?;
                newPath = stringAppendList(list![omHome, literal!("\\bin;"), omLibDir, literal!(";")]);
                newPath = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*System::stringReplace(newPath, literal!("/"), literal!("\\"))?);
                    __mm_s.push_str(&*oldPath);
                    ArcStr::from(__mm_s)
                };
                System::setEnv(literal!("PATH"), newPath, true);
            } else if hasBinDir && hasLibBinDir {
                oldPath = System::readEnv(literal!("PATH"))?;
                newPath = stringAppendList(list![
                    omHome,
                    literal!("\\bin;"),
                    omLibDir,
                    literal!(";"),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*binDir);
                        __mm_s.push_str(&*literal!(";"));
                        ArcStr::from(__mm_s)
                    },
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*libBinDir);
                        __mm_s.push_str(&*literal!(";"));
                        ArcStr::from(__mm_s)
                    },
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*msysBinDir);
                        __mm_s.push_str(&*literal!(";"));
                        ArcStr::from(__mm_s)
                    }
                ]);
                newPath = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*System::stringReplace(newPath, literal!("/"), literal!("\\"))?);
                    __mm_s.push_str(&*oldPath);
                    ArcStr::from(__mm_s)
                };
                System::setEnv(literal!("PATH"), newPath, true);
            } else {
                if !(Flags::isSet(Flags::DISABLE_WINDOWS_PATH_CHECK_WARNING.clone())?) {
                    metamodelica::print(literal!(
                        "We could not find some needed MINGW paths in $OPENMODELICAHOME or $OMDEV. Searched for paths:\n"
                    ));
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("\t"));
                        __mm_s.push_str(&*binDir);
                        __mm_s.push_str(&*if (hasBinDir) {
                            literal!(" [found] ")
                        } else {
                            literal!(" [not found] ")
                        });
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("\t"));
                        __mm_s.push_str(&*libBinDir);
                        __mm_s.push_str(&*if (hasLibBinDir) {
                            literal!(" [found] ")
                        } else {
                            literal!(" [not found] ")
                        });
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            ()
        }
    });
    Ok(())
}

fn setDefaultCC() -> () {
    if '__try0: {
        System::setCCompiler(unwrap_break_err!(System::readEnv(literal!("CC")), '__try0));
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ()
}

pub(crate) fn init(mut args: metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut args_1: metamodelica::List<ArcStr>;
    System::setEnv(literal!("G_SLICE"), literal!("always-malloc"), true);
    System::initGarbageCollector();
    if true {
        GCExt::setForceUnmapOnGcollect(metamodelica::stringEq(
            &arcstr::literal!(Autoconf::os),
            &(literal!("Windows_NT")),
        ));
    } else {
        GCExt::expandHeap(metamodelica::OrderedFloat(
            (if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {
                1024 * 1024 * 150
            } else {
                1024 * 1024 * 300
            }) as f64,
        ));
    }
    Global::initialize();
    ErrorExt::registerModelicaFormatError();
    ErrorExt::initAssertionFunctions();
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
    args_1 = FlagsUtil::new(args)?;
    FlagsUtil::applyNumProcEnvironment()?;
    setDefaultCC();
    SymbolTable::reset()?;
    BackendInterfaceImplementation::initializeBackendInterface();
    Ok(args_1)
}

pub fn main(mut args: metamodelica::List<ArcStr>) -> Result<()> {
    let mut args_1: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut seconds: i32 = 0;
    execStatReset()?;
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        match '__try1: {
            args_1 = unwrap_break_err!(init(args.clone()), '__try1);
            if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try1) {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*GCExt::profStatsStr(
                        GCExt::getProfStats(),
                        literal!("GC stats after initialization:"),
                        literal!("\n  "),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            seconds = unwrap_break_err!(Flags::getConfigInt(Flags::ALARM.clone()), '__try1);
            if seconds > 0 {
                System::alarm(seconds);
            }
            unwrap_break_err!(main2(args_1.clone()), '__try1);
            Ok::<_, &'static str>((args_1.clone(), seconds.clone()))
        } {
            Ok((__try1_o0, __try1_o1)) => {
                args_1 = __try1_o0;
                seconds = __try1_o1;
            }
            Err(__try1_err) => {
                ErrorExt::clearMessages();
                if '__try2: {
                    unwrap_break_err!(FlagsUtil::new(args.clone()), '__try2);
                    Ok::<(), &'static str>(())
                }
                .is_ok()
                {
                    return Err("failure(): body succeeded");
                }
                metamodelica::print(ErrorExt::printMessagesStr(false));
                metamodelica::print(literal!("\n"));
                return Err(__try1_err);
            }
        }
        if Flags::isSet(Flags::GC_PROF.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*GCExt::profStatsStr(
                    GCExt::getProfStats(),
                    literal!("GC stats at end of program:"),
                    literal!("\n  "),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok(());
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Stack overflow detected and was not caught.\n"));
                __mm_s.push_str(&*literal!(
                    "Send us a bug report at https://trac.openmodelica.org/OpenModelica/newticket\n"
                ));
                __mm_s.push_str(&*literal!("    Include the following trace:\n"));
                ArcStr::from(__mm_s)
            });
            for mut s in &*StackOverflow::readableStacktraceMessages()? {
                metamodelica::print(s.clone());
                metamodelica::print(literal!("\n"));
            }
        }
    }
    Ok(())
}

fn main2(mut args: metamodelica::List<ArcStr>) -> Result<()> {
    let mut interactiveMode: ArcStr;
    if Config::versionRequest()? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Settings::getVersionNr());
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        return Ok(());
    }
    interactiveMode = Flags::getConfigString(Flags::INTERACTIVE.clone())?;
    if System::userIsRoot()
        && (metamodelica::stringEq(&interactiveMode, &(literal!("tcp")))
            || metamodelica::stringEq(&interactiveMode, &(literal!("zmq"))))
    {
        Error::addMessage(Error::ROOT_USER_INTERACTIVE.clone(), metamodelica::nil())?;
        metamodelica::print(ErrorExt::printMessagesStr(false));
        return Err("fail");
    }
    if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
        setWindowsPaths(Settings::getInstallationDirectoryPath()?)?;
    }
    match '__try0: {
        unwrap_break_err!(Settings::getInstallationDirectoryPath(), '__try0);
        unwrap_break_err!(readSettings(args.clone()), '__try0);
        if metamodelica::stringEq(&interactiveMode, &(literal!("tcp"))) {
            unwrap_break_err!(interactivemode(), '__try0);
        } else if metamodelica::stringEq(&interactiveMode, &(literal!("zmq"))) {
            unwrap_break_err!(interactivemodeZMQ(), '__try0);
        } else {
            unwrap_break_err!(translateFile(args.clone()), '__try0);
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            if (args).is_empty() && metamodelica::stringEq(&(Config::classToInstantiate()?), &(literal!(""))) {
                if !(Config::helpRequest()?) {
                    metamodelica::print(FlagsUtil::printUsage()?);
                    System::fflush();
                }
                return Ok(());
            }
            if '__try1: {
                unwrap_break_err!(Settings::getInstallationDirectoryPath(), '__try1);
                metamodelica::print(literal!("# Error encountered! Exiting...\n"));
                System::fflush();
                metamodelica::print(literal!("# Please check the error message and the flags.\n"));
                System::fflush();
                unwrap_break_err!(Print::printBuf(literal!("\n\n----\n\nError buffer:\n\n")), '__try1);
                System::fflush();
                metamodelica::print(unwrap_break_err!(Print::getErrorString(), '__try1));
                System::fflush();
                metamodelica::print(ErrorExt::printMessagesStr(false));
                System::fflush();
                metamodelica::print(literal!("\n"));
                System::fflush();
                Ok::<(), &'static str>(())
            }
            .is_err()
            {
                metamodelica::print(literal!("Error: Failed to retrieve the installation directory path!\n"));
                System::fflush();
            }
            return Err(__try0_err);
        }
    }
    Ok(())
}
