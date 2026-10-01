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

use crate::CevalScriptBackend;
use crate::Interactive;
use crate::Interactive::Access;
use crate::InteractiveUtil;
use crate::NFApi;
use openmodelica_ast::Absyn;
use openmodelica_backend::SymbolTable;
use openmodelica_backend_tools::CevalScriptOMSimulator;
use openmodelica_backend_tools::DAEToMid;
use openmodelica_backend_tools::GenerateAPIFunctionsTpl;
use openmodelica_backend_tools::Unparsing;
use openmodelica_codegen::CodegenMidToC;
use openmodelica_codegen_cfunctions::CodegenCFunctions;
use openmodelica_codegen_util::MidCode;
use openmodelica_codegen_wasm_jit::CodegenWasmJitFunctions;
use openmodelica_error::ErrorExt;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::CevalFunction;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::FNode;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InstFunction;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Static;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_loader::ClassLoader;
use openmodelica_loader::Parser;
use openmodelica_program_util::ProgramUtil;
use openmodelica_script_util::DynLoad;
use openmodelica_script_util::PackageManagement;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_tpl::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::HashSetString;
use openmodelica_util::Print;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
// protected imports
pub(crate) fn ceval(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inBoolean: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    pub type ReductionOperator = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Values::Value>,
                metamodelica::Ref<Values::Value>,
            ) -> Result<metamodelica::Ref<Values::Value>>
            + 'static,
    >;

    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inBoolean, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { path: funcpath, expLst: expl, .. }, r#impl, msg) => {
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    let false = (stringEq(&(literal!("Connection.isRoot")), &(AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?))) else { return Err("pattern mismatch") };
                    (cache, vallst) = Ceval::cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter)?;
                    (cache, newval) = cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { .. }, true, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, value) = cevalInteractiveFunctions(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&e), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, r#impl, msg) => {
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, value) = Ceval::ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn isCompleteFunction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFuncPath: metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut isComplete: bool;
    isComplete = 'mc: {
        let __mc_input = (inCache, inEnv, inFuncPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, fpath) => {
                    ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&fpath), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(_), .. }, .. }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (System::getPartialInstantiation()) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, fpath) => {
                    ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&fpath), None)?) {
                        (_, Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL { .. }, .. }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isComplete
}

pub(crate) fn compileModel(
    mut fileprefix: ArcStr,
    mut libs: metamodelica::List<ArcStr>,
    mut workingDir: ArcStr,
    mut makeVars: metamodelica::List<ArcStr>,
) -> Result<()> {
    let mut omhome: ArcStr = Settings::getInstallationDirectoryPath()?;
    let mut omhome_1: ArcStr = System::stringReplace(omhome.clone(), literal!("\""), literal!(""))?;
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    let mut cdWorkingDir: ArcStr;
    let mut setMakeVars: ArcStr;
    let mut libsfilename: ArcStr;
    let mut libs_str: ArcStr;
    let mut s_call: ArcStr;
    let mut winCompileMode: ArcStr;
    let mut workDir: ArcStr = if (stringEq(&workingDir, &(literal!("")))) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*workingDir);
            __mm_s.push_str(&*pd);
            ArcStr::from(__mm_s)
        }
    };
    let mut linkType: ArcStr = literal!("dynamic");
    let mut fileDLL: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workDir);
        __mm_s.push_str(&*fileprefix);
        __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt));
        ArcStr::from(__mm_s)
    };
    let mut fileEXE: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workDir);
        __mm_s.push_str(&*fileprefix);
        __mm_s.push_str(&*arcstr::literal!(Autoconf::exeExt));
        ArcStr::from(__mm_s)
    };
    let mut fileLOG: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workDir);
        __mm_s.push_str(&*fileprefix);
        __mm_s.push_str(&*literal!(".log"));
        ArcStr::from(__mm_s)
    };
    let mut numParallel: i32;
    let mut isWindows: bool = metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")));
    let mut makeVarsNoBinding: metamodelica::List<ArcStr>;
    libsfilename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*workDir);
        __mm_s.push_str(&*fileprefix);
        __mm_s.push_str(&*literal!(".libs"));
        ArcStr::from(__mm_s)
    };
    libs_str = stringDelimitList(libs, literal!(" "));
    makeVarsNoBinding = makeVars;
    System::writeFile(libsfilename, libs_str)?;
    if isWindows {
        omhome = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("set OPENMODELICAHOME="));
            __mm_s.push_str(&*System::stringReplace(
                omhome_1.clone(),
                literal!("/"),
                literal!("\\"),
            )?);
            __mm_s.push_str(&*literal!("&& "));
            ArcStr::from(__mm_s)
        };
        setMakeVars = ({
            let mut __acc = String::new();
            for mut var in (makeVarsNoBinding).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("set "));
                    __mm_s.push_str(&*var);
                    __mm_s.push_str(&*literal!("&& "));
                    ArcStr::from(__mm_s)
                };
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone();
        cdWorkingDir = if (stringEmpty(&workingDir)) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("cd \""));
                __mm_s.push_str(&*workingDir);
                __mm_s.push_str(&*literal!("\"&& "));
                ArcStr::from(__mm_s)
            }
        };
        winCompileMode = if (Testsuite::isRunning()?) {
            literal!("serial")
        } else {
            literal!("parallel")
        };
        if Flags::getConfigEnum(Flags::LINK_TYPE.clone())? == 1 {
            linkType = literal!("static");
        } else if Flags::getConfigEnum(Flags::LINK_TYPE.clone())? == 2 {
            linkType = literal!("dynamic");
        }
        linkType = if (Testsuite::isRunning()?) {
            literal!("dynamic")
        } else {
            linkType
        };
        s_call = stringAppendList(list![
            omhome,
            cdWorkingDir,
            setMakeVars,
            literal!("\""),
            omhome_1.clone(),
            pd.clone(),
            literal!("share"),
            pd.clone(),
            literal!("omc"),
            pd.clone(),
            literal!("scripts"),
            pd.clone(),
            literal!("Compile"),
            literal!("\""),
            literal!(" "),
            fileprefix.clone(),
            literal!(" "),
            Config::simulationCodeTarget()?,
            literal!(" "),
            System::openModelicaPlatform(),
            literal!(" "),
            winCompileMode,
            literal!(" "),
            linkType
        ]);
    } else {
        numParallel = if (Testsuite::isRunning()?) {
            1
        } else {
            Config::noProc()?
        };
        cdWorkingDir = if (stringEmpty(&workingDir)) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" -C \""));
                __mm_s.push_str(&*workingDir);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            }
        };
        setMakeVars = ({
            let mut __acc = String::new();
            for mut var in (makeVarsNoBinding).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*var);
                    ArcStr::from(__mm_s)
                };
                __acc.push_str(&__x);
            }
            ArcStr::from(__acc)
        })
        .clone();
        s_call = stringAppendList(list![
            arcstr::literal!(Autoconf::make),
            literal!(" -j"),
            intString(numParallel),
            cdWorkingDir,
            literal!(" -f "),
            fileprefix.clone(),
            literal!(".makefile"),
            setMakeVars
        ]);
    }
    if Flags::isSet(Flags::DYN_LOAD.clone())? {
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("compileModel: running "));
            __mm_s.push_str(&*s_call);
            ArcStr::from(__mm_s)
        })?;
    }
    if System::regularFileExists(fileEXE.clone()) {
        let 0 = (System::removeFile(fileEXE.clone())) else {
            return Err("pattern mismatch");
        };
    }
    if System::regularFileExists(fileDLL.clone()) {
        let 0 = (System::removeFile(fileDLL.clone())) else {
            return Err("pattern mismatch");
        };
    }
    if System::regularFileExists(fileLOG.clone()) {
        let 0 = (System::removeFile(fileLOG.clone())) else {
            return Err("pattern mismatch");
        };
    }
    if Testsuite::isRunning()? {
        System::appendFile(Testsuite::getTempFilesFile()?, {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*fileEXE);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*fileDLL);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*fileLOG);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*fileprefix);
            __mm_s.push_str(&*literal!(".o\n"));
            __mm_s.push_str(&*fileprefix);
            __mm_s.push_str(&*literal!(".libs\n"));
            __mm_s.push_str(&*fileprefix);
            __mm_s.push_str(&*literal!("_records.o\n"));
            __mm_s.push_str(&*fileprefix);
            __mm_s.push_str(&*literal!("_res.mat\n"));
            ArcStr::from(__mm_s)
        })?;
    }
    if System::systemCall(s_call, if (isWindows) { literal!("") } else { fileLOG.clone() }) != 0 {
        if System::regularFileExists(fileLOG.clone()) {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![System::readFile(fileLOG)?])?;
        } else if isWindows {
            s_call = stringAppendList(list![
                omhome_1,
                pd.clone(),
                literal!("share"),
                pd.clone(),
                literal!("omc"),
                pd.clone(),
                literal!("scripts"),
                pd,
                literal!("Compile.bat")
            ]);
            if !(System::regularFileExists(s_call.clone())) {
                Error::addMessage(
                    Error::SIMULATOR_BUILD_ERROR.clone(),
                    list![stringAppendList(list![
                        literal!("command "),
                        s_call,
                        literal!(" not found. Check $OPENMODELICAHOME")
                    ])],
                )?;
            }
        }
        if Flags::isSet(Flags::DYN_LOAD.clone())? {
            Debug::trace(literal!("compileModel: failed!\n"))?;
        }
        return Err("fail");
    }
    if Flags::isSet(Flags::DYN_LOAD.clone())? {
        Debug::trace(literal!("compileModel: successful!\n"))?;
    }
    Ok(())
}

pub(crate) fn loadFile(
    mut inName: ArcStr,
    mut encoding: ArcStr,
    mut p: Absyn::Program,
    mut checkUses: bool,
    mut notifyLoad: bool,
    mut requireExactVersion: bool,
    mut allowWithin: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut dir: ArcStr;
    let mut name: ArcStr = inName.clone();
    let mut filename: ArcStr;
    let mut cname: ArcStr;
    let mut prio: ArcStr;
    let mut mp: ArcStr;
    let mut msg: ArcStr;
    let mut rest: metamodelica::List<ArcStr>;
    if System::directoryExists(inName.clone()) {
        name = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inName);
            __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
            __mm_s.push_str(&*literal!("package.mo"));
            ArcStr::from(__mm_s)
        };
    }
    if !(System::regularFileReadable(name.clone())) {
        if !(System::regularFileExists(name.clone())) {
            msg = literal!("file does not exist");
        } else {
            msg = literal!("read access denied");
        }
        Error::addMessage(Error::LOAD_FILE_FAILED.clone(), list![name.clone(), msg])?;
        return Err("fail");
    }
    (dir, filename) = Util::getAbsoluteDirectoryAndFile(name.clone())?;
    if metamodelica::stringEq(&filename, &(literal!("package.mo")))
        || metamodelica::stringEq(&filename, &(literal!("package.moc")))
    {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok(List::last(&(System::strtok(dir.clone(), literal!("/"))))?, literal!(" "))) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cname = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        prio = stringDelimitList(rest, literal!(" "));
        mp = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*System::realpath({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*literal!("/../"));
                ArcStr::from(__mm_s)
            })?);
            __mm_s.push_str(&*arcstr::literal!(Autoconf::groupDelimiter));
            __mm_s.push_str(&*Settings::getModelicaPath(Testsuite::isRunning()?)?);
            ArcStr::from(__mm_s)
        };
        let (__pa2, true) = (loadModel(
            &(metamodelica::cons(
                (
                    metamodelica::Ref::new(Absyn::Path::IDENT { name: cname }),
                    literal!("loadFile automatically converted to loadModel"),
                    list![prio],
                    true,
                ),
                metamodelica::nil(),
            )),
            mp,
            p,
            true,
            notifyLoad,
            checkUses,
            requireExactVersion,
            metamodelica::stringEq(&filename, &(literal!("package.moc"))),
            System::realpath(name)?,
        )?) else {
            return Err("pattern mismatch");
        };
        outProgram = metamodelica::Own::own(__pa2);
        return Ok(outProgram);
    }
    outProgram = Parser::parse(
        name.clone(),
        encoding,
        literal!(""),
        None,
        Config::acceptedGrammar()?,
        Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
        Flags::getConfigBool(Flags::STRICT.clone())?,
    )?;
    if !(allowWithin) {
        checkTopClassWithin(&outProgram, name)?;
    }
    ClassLoader::checkOnLoadMessage(outProgram.clone())?;
    if checkDuplicateTopLevelClasses(&outProgram)? {
        return Err("fail");
    }
    outProgram = checkUsesAndUpdateProgram(
        outProgram,
        p,
        checkUses,
        Settings::getModelicaPath(Testsuite::isRunning()?)?,
        notifyLoad,
        requireExactVersion,
        false,
    )?;
    Ok(outProgram)
}

fn checkDuplicateTopLevelClasses(mut program: &Absyn::Program) -> Result<bool> {
    let mut hasDuplicates: bool = false;
    let mut skip: bool;
    let mut infos: metamodelica::List<SourceInfo>;
    let mut classInfoMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SourceInfo>>;
    let mut optClassInfo: Option<SourceInfo>;
    if ((program.classes).len() as i32) < 2 {
        return Ok(hasDuplicates);
    }
    classInfoMap = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut cl in &*program.classes.clone() {
        let () = (match &*cl.clone() {
            Absyn::Class {
                info: SourceInfo { .. },
                ..
            } => {
                skip = stringEq(&cl.info.fileName, &(literal!("<interactive>")))
                    || stringEq(
                        &(System::basename(cl.info.fileName.clone())),
                        &(literal!("ModelicaBuiltin.mo")),
                    )
                    || stringEq(
                        &(System::basename(cl.info.fileName.clone())),
                        &(literal!("MetaModelicaBuiltin.mo")),
                    );
                if !(skip) {
                    optClassInfo = UnorderedMap::get(cl.name.clone(), classInfoMap.clone())?;
                    if (optClassInfo).is_some() {
                        infos = list![optClassInfo.ok_or("pattern mismatch")?, cl.info.clone()];
                        Error::addMultiSourceMessage(
                            &(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone()),
                            &(list![cl.name.clone()]),
                            &infos,
                        )?;
                        hasDuplicates = true;
                        return Ok(hasDuplicates);
                    } else {
                        UnorderedMap::add(cl.name.clone(), cl.info.clone(), classInfoMap.clone())?;
                    }
                }
                ()
            }
            _ => (),
        });
    }
    Ok(hasDuplicates)
}

fn checkTopClassWithin(mut program: &Absyn::Program, mut filename: ArcStr) -> Result<()> {
    if !(AbsynUtil::withinEqual(&program.within_, &(openmodelica_ast::Absyn::Within::TOP))) {
        Error::addSourceMessage(
            &(Error::LIBRARY_UNEXPECTED_WITHIN.clone()),
            list![
                AbsynUtil::withinString(&(openmodelica_ast::Absyn::Within::TOP))?,
                AbsynUtil::withinString(&program.within_)?
            ],
            &(SourceInfo {
                fileName: filename,
                isReadOnly: false,
                lineNumberStart: 1,
                columnNumberStart: 0,
                lineNumberEnd: 1,
                columnNumberEnd: 0,
                lastModification: metamodelica::OrderedFloat((0) as f64),
            }),
        )?;
        return Err("fail");
    }
    Ok(())
}

fn checkUsesAndUpdateProgram(
    mut newp: Absyn::Program,
    mut p: Absyn::Program,
    mut checkUses: bool,
    mut modelicaPath: ArcStr,
    mut notifyLoad: bool,
    mut requireExactVersion: bool,
    mut mergeAST: bool,
) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = p;
    let mut modelsToLoad: metamodelica::List<(
        metamodelica::Ref<Absyn::Path>,
        ArcStr,
        metamodelica::List<ArcStr>,
        bool,
    )>;
    modelsToLoad = if (checkUses) {
        Interactive::getUsesAnnotationOrDefault(newp.clone(), requireExactVersion)?
    } else {
        metamodelica::nil()
    };
    p = ProgramUtil::updateProgram(newp, p, mergeAST, false)?;
    (p, _) = loadModel(
        &modelsToLoad,
        modelicaPath,
        p,
        false,
        notifyLoad,
        checkUses,
        requireExactVersion,
        false,
        literal!(""),
    )?;
    Ok(p)
}

pub(crate) fn loadModel(
    mut imodelsToLoad: &metamodelica::List<(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool)>,
    mut modelicaPath: ArcStr,
    mut ip: Absyn::Program,
    mut forceLoad: bool,
    mut notifyLoad: bool,
    mut checkUses: bool,
    mut requireExactVersion: bool,
    mut encrypted: bool,
    mut pathToFile: ArcStr,
) -> Result<(Absyn::Program, bool)> {
    let mut pnew: Absyn::Program = ip;
    let mut success: bool = true;
    let mut b: bool;
    PackageManagement::installCachedPackages()?;
    for mut m in &**imodelsToLoad {
        (pnew, b) = loadModel1(
            &(m.clone()),
            modelicaPath.clone(),
            forceLoad,
            notifyLoad,
            checkUses,
            requireExactVersion,
            encrypted,
            pathToFile.clone(),
            pnew,
        )?;
        success = b && success;
    }
    Ok((pnew, success))
}

fn loadModel1(
    mut modelToLoad: &(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool),
    mut modelicaPath: ArcStr,
    mut forceLoad: bool,
    mut notifyLoad: bool,
    mut checkUses: bool,
    mut requireExactVersion: bool,
    mut encrypted: bool,
    mut pathToFile: ArcStr,
    mut program: Absyn::Program,
) -> Result<(Absyn::Program, bool)> {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut modelsToLoad: metamodelica::List<(
        metamodelica::Ref<Absyn::Path>,
        ArcStr,
        metamodelica::List<ArcStr>,
        bool,
    )>;
    let mut onlyCheckFirstModelicaPath: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut versionsLst: metamodelica::List<ArcStr>;
    let mut pathStr: ArcStr;
    let mut versions: ArcStr;
    let mut version: ArcStr;
    let mut thisModelicaPath: ArcStr;
    let mut dir: ArcStr;
    let mut requestedBy: ArcStr;
    let mut pnew: Absyn::Program;
    let mut msgTokens: metamodelica::List<ArcStr>;
    let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
    (path, requestedBy, versionsLst, onlyCheckFirstModelicaPath) = modelToLoad.clone();
    if onlyCheckFirstModelicaPath {
        let __pa0 = ::match_deref::match_deref! { match &(System::strtok(modelicaPath.clone(), arcstr::literal!(Autoconf::groupDelimiter))) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        thisModelicaPath = metamodelica::Own::own(__pa0);
    } else {
        thisModelicaPath = modelicaPath.clone();
    }
    if '__try1: {
        if unwrap_break_err!(checkModelLoaded(modelToLoad, &program, forceLoad, None), '__try1) {
            pnew = Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP };
            version = literal!("");
            return Ok((program, success));
        } else {
            if metamodelica::stringEq(&pathToFile, &(literal!(""))) {
                pnew = unwrap_break_err!(ClassLoader::loadClass(&path, versionsLst.clone(), thisModelicaPath.clone(), None, requireExactVersion, encrypted), '__try1);
            } else {
                dir = System::dirname(pathToFile.clone());
                cl = unwrap_break_err!(ClassLoader::loadClassFromMp(AbsynUtil::pathFirstIdent(&path), System::dirname(dir.clone()), System::basename(dir.clone()), true, None, encrypted), '__try1);
                if (cl).is_some() {
                    pnew = Absyn::Program { classes: list![unwrap_break_err!(cl.clone().ok_or("pattern mismatch"), '__try1)], within_: openmodelica_ast::Absyn::Within::TOP };
                } else {
                    pnew = Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP };
                }
            }
            checkPatchedModelicaServices(&(AbsynUtil::pathFirstIdent(&path)), &pnew);
            if notifyLoad && !(forceLoad) {
                version = unwrap_break_err!(getPackageVersion(path.clone(), pnew.clone()), '__try1);
                msgTokens = list![unwrap_break_err!(AbsynUtil::pathString(path.clone(), literal!("."), true, false), '__try1), version.clone(), requestedBy.clone()];
                unwrap_break_err!(Error::addMessage(Error::NOTIFY_LOAD_MODEL_DUE_TO_USES.clone(), msgTokens.clone()), '__try1);
                System::loadModelCallBack(AbsynUtil::pathFirstIdent(&path));
            }
        }
        program = unwrap_break_err!(ProgramUtil::updateProgram(pnew.clone(), program.clone(), false, false), '__try1);
        if checkUses {
            modelsToLoad = unwrap_break_err!(Interactive::getUsesAnnotationOrDefault(pnew.clone(), requireExactVersion), '__try1);
            (program, success) = unwrap_break_err!(loadModel(&modelsToLoad, modelicaPath.clone(), program.clone(), false, notifyLoad, checkUses, requireExactVersion, false, literal!("")), '__try1);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        pathStr = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
        versions = stringDelimitList(versionsLst.clone(), literal!(","));
        msgTokens = list![pathStr.clone(), versions.clone(), thisModelicaPath.clone()];
        if forceLoad {
            Error::addMessage(Error::LOAD_MODEL_FAILED.clone(), msgTokens.clone())?;
            success = false;
        } else {
            Error::addMessage(Error::NOTIFY_LOAD_MODEL_FAILED.clone(), msgTokens.clone())?;
        }
    }
    Ok((program, success))
}

fn checkModelLoaded(
    mut tpl: &(metamodelica::Ref<Absyn::Path>, ArcStr, metamodelica::List<ArcStr>, bool),
    mut p: &Absyn::Program,
    mut forceLoad: bool,
    mut failNonLoad: Option<ArcStr>,
) -> Result<bool> {
    let mut loaded: bool;
    loaded = 'mc: {
        let __mc_input = (tpl, forceLoad, failNonLoad);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((path, requestOrigin, Deref @ metamodelica::ListNode::Cons { head: str1, tail: _ }, _), false, _) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut ostr2: Option<ArcStr>;
                    let mut withoutConversion: metamodelica::List<ArcStr>;
                    let mut withConversion: metamodelica::List<ArcStr>;
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), p, false, false)?;
                    ostr2 = AbsynUtil::getNamedAnnotationInClass(&cdef, &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("version") })), &Interactive::getAnnotationStringValueOrFail);
                    (withoutConversion, withConversion) = Interactive::getConversionAnnotation(&cdef);
                    checkValidVersion(path.clone(), str1.clone(), ostr2.clone(), requestOrigin.clone(), &withConversion, &withoutConversion)?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((path, _, _, _), _, Some(str2)) => {
                    let mut str1: ArcStr;
                    str1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::INST_NON_LOADED.clone(), list![str1.clone(), str2.clone()])?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(loaded)
}

fn checkValidVersion(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut version: ArcStr,
    mut actualVersion: Option<ArcStr>,
    mut requestOrigin: ArcStr,
    mut withConversion: &metamodelica::List<ArcStr>,
    mut withoutConversion: &metamodelica::List<ArcStr>,
) -> Result<()> {
    let mut semverWanted: SemanticVersion::Version;
    let mut semverActual: SemanticVersion::Version;
    let mut actualVersionStr: ArcStr;
    let mut pathStr: ArcStr;
    semverWanted = SemanticVersion::parse(version.clone(), false)?;
    actualVersionStr = actualVersion.unwrap_or(literal!(""));
    pathStr = AbsynUtil::pathString(path, literal!("."), true, false)?;
    semverActual = SemanticVersion::parse(actualVersionStr.clone(), false)?;
    if 0 == SemanticVersion::compare(&semverWanted, &semverActual, false, false)? {
        return Ok(());
    }
    if !(SemanticVersion::isSemVer(&semverActual) && SemanticVersion::isSemVer(&semverWanted)) {
        Error::addMessage(
            Error::LOAD_MODEL_DIFFERENT_VERSIONS.clone(),
            list![pathStr, version, actualVersionStr],
        )?;
        return Ok(());
    }
    for mut ver in &**withoutConversion {
        if 0 == SemanticVersion::compare(
            &semverWanted,
            &(SemanticVersion::parse(ver.clone(), false)?),
            false,
            false,
        )? {
            Error::addMessage(
                Error::LOAD_MODEL_DIFFERENT_VERSIONS_WITHOUT_CONVERSION.clone(),
                list![requestOrigin, pathStr, version, actualVersionStr],
            )?;
            return Ok(());
        }
    }
    for mut ver in &**withConversion {
        if 0 == SemanticVersion::compare(
            &semverWanted,
            &(SemanticVersion::parse(ver.clone(), false)?),
            false,
            false,
        )? {
            Error::addMessage(
                Error::LOAD_MODEL_DIFFERENT_VERSIONS_WITH_CONVERSION.clone(),
                list![requestOrigin, pathStr, version, actualVersionStr],
            )?;
            return Ok(());
        }
    }
    if SemanticVersion::compare(&semverWanted, &semverActual, true, false)? > 0 {
        Error::addMessage(
            Error::LOAD_MODEL_DIFFERENT_VERSIONS_NEWER.clone(),
            list![pathStr, version, actualVersionStr],
        )?;
    } else {
        Error::addMessage(
            Error::LOAD_MODEL_DIFFERENT_VERSIONS_OLDER.clone(),
            list![pathStr, version, actualVersionStr],
        )?;
    }
    Ok(())
}

fn checkPatchedModelicaServices(mut name: &ArcStr, mut program: &Absyn::Program) -> () {
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut alg: metamodelica::Ref<Absyn::Algorithm>;
    let mut r#fn: metamodelica::Ref<Absyn::ComponentRef>;
    if metamodelica::stringEq(&name, &(literal!("ModelicaServices"))) {
        if '__try0: {
            cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("ModelicaServices"), path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("ExternalReferences"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("loadResource") }) }) }), program, false, false), '__try0);
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(List::find(&(AbsynUtil::getClassPartsInClass(&cls)), &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isAlgorithmSection(&__a0)) }), '__try0)) {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            alg = metamodelica::Own::own(__pa1);
            let __pa3 = ::match_deref::match_deref! { match &(alg.clone()) {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { value: Deref @ Absyn::Exp::CALL { function_: __pa3, .. }, .. } => __pa3.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa3);
            if !metamodelica::stringEq(&(unwrap_break_err!(AbsynUtil::crefString(&r#fn), '__try0)), &(literal!("OpenModelica.Scripting.uriToFilename"))) {
                unwrap_break_err!(Error::addMessage(Error::UNPATCHED_MODELICA_SERVICES.clone(), metamodelica::nil()), '__try0);
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    ()
}

pub(crate) fn cevalInteractiveFunctions(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut msg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "timing" }, expLst: Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut t1: metamodelica::Real;
                    let mut t2: metamodelica::Real;
                    let mut t: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    t1 = System::time();
                    (cache, _) = Ceval::ceval(cache.clone(), env.clone(), exp.clone(), true, msg.clone(), numIter + 1)?;
                    t2 = System::time();
                    t = t2 - t1;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::REAL { real: t })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: eLst }) => {
                    let mut valLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, valLst) = Ceval::cevalList(cache.clone(), env.clone(), eLst.clone(), true, msg.clone(), numIter)?;
                    valLst = List::map1(valLst.clone(), &fnptr!(evalCodeTypeName, metamodelica::Ref<Values::Value>, FCore::Graph), env.clone())?;
                    (cache, value) = cevalInteractiveFunctions2(cache.clone(), env.clone(), name.clone(), valLst.clone(), msg.clone())?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn cevalInteractiveFunctions2(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut functionName: ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut msg: Absyn::Msg,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache = cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (functionName.clone(), &*args);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "parseString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                            let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                            let mut within_: Absyn::Within;
                            let Absyn::PROGRAM { classes: __pa0, within_: __pa1 } = Parser::parsestring(str1.clone(), str2.clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
                            classes = metamodelica::Own::own(__pa0);
                            within_ = metamodelica::Own::own(__pa1);
                            paths = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut c in (classes.clone()).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(Absyn::Path::IDENT { name: AbsynUtil::className(&(c.clone())) });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            paths = List::map1r(paths.clone(), &move |__a0: Absyn::Within, __a1: metamodelica::Ref<Absyn::Path>| AbsynUtil::joinWithinPath(&__a0, __a1), within_.clone())?;
                            Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseString", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    Error::clearMessages();
                    Print::clearErrorBuf();
                    paths = Interactive::parseFile(str1.clone(), encoding.clone(), false)?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFileInteractiveQualified", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    Error::clearMessages();
                    Print::clearErrorBuf();
                    paths = Interactive::parseFile(str1.clone(), encoding.clone(), true)?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFileInteractive", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut newp: Absyn::Program;
                    newp = loadFile(str1.clone(), encoding.clone(), SymbolTable::getAbsyn(), b.clone(), b1.clone(), requireExactVersion.clone(), true)?;
                    vals = List::map(Interactive::getTopClassnames(&newp)?, &fnptr!(ValuesMake::makeCodeTypeName, metamodelica::Ref<Absyn::Path>))?;
                    SymbolTable::setAbsyn(newp.clone())?;
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getSourceFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    r#str = Interactive::getSourceFile(path.clone(), SymbolTable::getAbsyn());
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setSourceFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut p: Absyn::Program;
                    let mut access: Access;
                    let mut b: bool;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access >= Access::all.clone() {
                        (b, p) = Interactive::setSourceFile(path.clone(), r#str.clone(), SymbolTable::getAbsyn());
                        SymbolTable::setAbsyn(p.clone())?;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "basename", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::basename(r#str.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dirname", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::dirname(r#str.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "codeToString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: codeNode }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Dump::printCodeStr(codeNode.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeOf", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. } } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Interactive::getTypeOfVariable(name.clone(), &(SymbolTable::getVars()))?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: TypesDump::unparseType(ty.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_gcollect_and_unmap", Deref @ metamodelica::ListNode::Nil) => {
                    GCExt::gcollectAndUnmap();
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_expand_hp", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: GCExt::expandHeap(metamodelica::OrderedFloat((i.clone()) as f64)) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_set_max_heap_size", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    GCExt::setMaxHeapSize(metamodelica::OrderedFloat((i.clone()) as f64));
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_get_prof_stats", Deref @ metamodelica::ListNode::Nil) => {
                    let mut gcStats: GCExt::ProfStats;
                    gcStats = GCExt::getProfStats();
                    Ok(metamodelica::Ref::new(Values::Value::RECORD { record_: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("GC_PROFSTATS") }), orderd: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.heapsize_full.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.free_bytes_full.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.unmapped_bytes.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.bytes_allocd_since_gc.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.allocd_bytes_before_gc.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.bytes_allocd_since_gc.clone() + gcStats.allocd_bytes_before_gc.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.non_gc_bytes.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.gc_no.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.markers_m1.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.bytes_reclaimed_since_gc.clone() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: gcStats.reclaimed_bytes_before_gc.clone() })], comp: list![literal!("heapsize_full"), literal!("free_bytes_full"), literal!("unmapped_bytes"), literal!("bytes_allocd_since_gc"), literal!("allocd_bytes_before_gc"), literal!("total_allocd_bytes"), literal!("non_gc_bytes"), literal!("gc_no"), literal!("markers_m1"), literal!("bytes_reclaimed_since_gc"), literal!("reclaimed_bytes_before_gc")], index: -1 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clear", Deref @ metamodelica::ListNode::Nil) => {
                    SymbolTable::reset()?;
                    NFApi::clearCache();
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearProgram", Deref @ metamodelica::ListNode::Nil) => {
                    SymbolTable::clearProgram()?;
                    NFApi::clearCache();
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearVariables", Deref @ metamodelica::ListNode::Nil) => {
                    SymbolTable::setVars(metamodelica::nil());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "list", _) => {
                    Ok(listClass(&args))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "listFile", _) => {
                    Ok(listFile(&args))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sortStrings", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    strs = List::sort(strs.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                    Ok(ValuesMake::makeArray(List::map(strs.clone(), &fnptr!(ValuesMake::makeString, ArcStr))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "listVariables", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(ValuesMake::makeArray(getVariableNames(&(SymbolTable::getVars()), metamodelica::nil())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setTempDirectoryPath", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Settings::setTempDirectoryPath(cmd.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTempDirectoryPath", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getTempDirectoryPath() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setEnvironmentVar", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::setEnv(name.clone(), r#str.clone(), true) == 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEnvironmentVar", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Util::makeValueOrDefault(&System::readEnv, name.clone(), literal!("")) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setInstallationDirectoryPath", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Settings::setInstallationDirectoryPath(cmd.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstallationDirectoryPath", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getInstallationDirectoryPath()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelicaPath", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getModelicaPath(Testsuite::isRunning()?)? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setModelicaPath", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Settings::setModelicaPath(cmd.clone());
                    { let __v = None; openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setModelicaPath", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getHomeDirectoryPath", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getHomeDir(Testsuite::isRunning()?) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLanguageStandard", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Config::languageStandardString(Config::getLanguageStandard()?)? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reopenStandardStream", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::reopenStandardStream(i.clone() - 1, filename.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "iconv", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: from }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: to }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::iconv(r#str.clone(), from.clone(), to.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCompiler", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::getCCompiler() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCFlags", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setCFlags(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCFlags", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::getCFlags() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCompiler", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setCCompiler(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCXXCompiler", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::getCXXCompiler() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCXXCompiler", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setCXXCompiler(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCompilerFlags", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setCFlags(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLinker", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::getLinker() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setLinker", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setLinker(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLinkerFlags", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::getLDFlags() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setLinkerFlags", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::setLDFlags(r#str.clone());
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCommandLineOptions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut b: bool;
                    let mut outCache: FCore::Cache = outCache.clone();
                    b = Flags::isSet(Flags::SCODE_INST.clone())?;
                    strs = System::strtok(r#str.clone(), literal!(" "));
                    ::match_deref::match_deref! { match &(FlagsUtil::readArgs(strs.clone())?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    FlagsUtil::applyNumProcEnvironment()?;
                    outCache = FCore::emptyCache();
                    if b != Flags::isSet(Flags::SCODE_INST.clone())? {
                        Builtin::clearInitialGraph();
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCommandLineOptions", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCommandLineOptions", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(ValuesMake::makeStringArray(FlagsUtil::unparseFlags()?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCommandLineOptions", _) => {
                    Ok(openmodelica_frontend_types::Values::Value::interned_META_FAIL())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearCommandLineOptions", Deref @ metamodelica::ListNode::Nil) => {
                    FlagsUtil::resetDebugFlags()?;
                    FlagsUtil::resetConfigFlags()?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearCommandLineOptions", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "enableNewInstantiation", _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
                        Builtin::clearInitialGraph();
                        FlagsUtil::enableDebug(Flags::SCODE_INST.clone())?;
                        outCache = FCore::emptyCache();
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "enableNewInstantiation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "disableNewInstantiation", _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    if Flags::isSet(Flags::SCODE_INST.clone())? {
                        FlagsUtil::disableDebug(Flags::SCODE_INST.clone())?;
                        outCache = FCore::emptyCache();
                        Builtin::clearInitialGraph();
                    }
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "disableNewInstantiation", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearDebugFlags", _) => {
                    FlagsUtil::resetDebugFlags()?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearDebugFlags", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConfigFlagValidOptions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    let mut r#str = (*r#str).clone();
                    (strs1, r#str, strs2) = FlagsUtil::getValidOptionsAndDescription(r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeStringArray(strs1.clone())?, metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }), ValuesMake::makeStringArray(strs2.clone())?] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConfigFlagValidOptions", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(metamodelica::nil()), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), ValuesMake::makeArray(metamodelica::nil())] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::pwd() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let 0 = (System::cd(r#str.clone())) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::pwd() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut res: ArcStr;
                    let false = (System::directoryExists(r#str.clone())) else { return Err("pattern mismatch") };
                    res = stringAppendList(list![literal!("Error, directory "), r#str.clone(), literal!(" does not exist,")]);
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: res.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "mkdir", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let true = (System::directoryExists(r#str.clone())) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "mkdir", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: Util::createDirectoryTree(r#str.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "copy", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::copyFile(str1.clone(), str2.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "remove", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::removeDirectory(r#str.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVersion", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "OpenModelica" } } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getVersionNr() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVersion", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: getPackageVersion(path.clone(), SymbolTable::getAbsyn())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTempDirectoryPath", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Settings::getTempDirectoryPath() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "system", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: System::systemCall(r#str.clone(), filename.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "system_parallel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    Ok(ValuesMake::makeIntArray(System::systemCallParallel(strs.clone(), i.clone()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerClear", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::realtimeClear(i.clone())?;
                    Ok(openmodelica_frontend_types::Values::Value::interned_NORETCALL())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTick", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::realtimeTick(i.clone())?;
                    Ok(openmodelica_frontend_types::Values::Value::interned_NORETCALL())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTock", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let true = (System::realtimeNtick(i.clone())? > 0) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: System::realtimeTock(i.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTock", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(-1.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerAccumulated", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let true = (System::realtimeNtick(i.clone())? > 0) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: System::realtimeAccumulated(i.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerAccumulated", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(-1.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: System::readFile(r#str.clone())? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readFile", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    System::writeFile(r#str.clone(), str1.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    System::appendFile(r#str.clone(), str1.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::removeFile(r#str.clone()) == 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFiles", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: System::fileContentsEqual(str1.clone(), str2.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFilesAndMove", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    let true = (System::regularFileExists(str1.clone())) else { return Err("pattern mismatch") };
                    b = System::regularFileExists(str2.clone()) && System::fileContentsEqual(str1.clone(), str2.clone());
                    b = if (!(b)) {System::rename(str1.clone(), str2.clone())} else {System::removeFile(str1.clone()) == 0};
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFilesAndMove", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getErrorString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: Error::printMessagesStr(b.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "countMessages", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: Error::getNumMessages() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: Error::getNumErrorMessages() }), metamodelica::Ref::new(Values::Value::INTEGER { integer: ErrorExt::getNumWarningMessages() })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearMessages", Deref @ metamodelica::ListNode::Nil) => {
                    Error::clearMessages();
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMessagesStringInternal", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut messages: metamodelica::List<ErrorTypes::TotalMessage>;
                    messages = List::unique(&(Error::getMessages()));
                    Ok(ValuesMake::makeArray(List::map(messages.clone(), &move |__a0: ErrorTypes::TotalMessage| errorToValue(&__a0))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMessagesStringInternal", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeArray(List::map(Error::getMessages(), &move |__a0: ErrorTypes::TotalMessage| errorToValue(&__a0))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringTypeName", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(r#str.clone())? }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringVariableName", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: Parser::stringCref(r#str.clone())? }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeNameString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: AbsynUtil::pathString(path.clone(), literal!("."), true, false)? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeNameStrings", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(ValuesMake::makeArray(List::map(AbsynUtil::pathToStringList(metamodelica::AsArg::as_arg(&path)), &fnptr!(ValuesMake::makeString, ArcStr))?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateHeader", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    r#str = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<SCode::Element>>| Unparsing::programExternalHeader(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<metamodelica::Ref<SCode::Element>>) -> Result<Tpl::Text> + 'static>), SymbolTable::getSCode()?)?;
                    System::writeFile(filename.clone(), r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateHeader", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateJuliaHeader", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    r#str = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<SCode::Element>>| Unparsing::programExternalHeaderJulia(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<metamodelica::Ref<SCode::Element>>) -> Result<Tpl::Text> + 'static>), SymbolTable::getSCode()?)?;
                    System::writeFile(filename.clone(), r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateJuliaHeader", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateCode", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunction(outCache.clone(), env.clone(), path.clone(), false, None, true)) else { return Err("pattern mismatch") };
                    outCache = metamodelica::Own::own(__pa0);
                    (outCache, _, _) = cevalGenerateFunction(outCache.clone(), env.clone(), &(SymbolTable::getAbsyn()), path.clone())?;
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateCode", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "generateScriptingAPI", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                            let mut str1: ArcStr;
                            let mut str2: ArcStr;
                            let mut str3: ArcStr;
                            let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
                            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut cl: metamodelica::Ref<SCode::Element>;
                            let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                            let mut outCache: FCore::Cache = outCache.clone();
                            sp = SymbolTable::getSCode()?;
                            elts = (::match_deref::match_deref! { match &(FBuiltin::getElementWithPathCheckBuiltin(sp.clone(), metamodelica::AsArg::as_arg(&className))?) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __esc_elts, .. }, .. } => {
                            elts = (*__esc_elts).clone();
                            elts.clone()
                },
                __esc_cl => {
                            cl = (*__esc_cl).clone();
                            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" does not contain SCode.PARTS")); ArcStr::from(__mm_s) }], &(SCodeUtil::elementInfo(metamodelica::AsArg::as_arg(&cl))))?;
                            return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            tys = metamodelica::nil();
                            for mut elt in &*elts {
                                let () = 'mc: {
                let __mc_input = elt.clone();
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, .. } => {
                                    let mut outCache: FCore::Cache = outCache.clone();
                                    let mut ty: metamodelica::Ref<DAE::Type>;
                                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>> = tys.clone();
                                    (outCache, ty, _) = Lookup::lookupType(outCache.clone(), env.clone(), AbsynUtil::suffixPath(metamodelica::AsArg::as_arg(&className), var_field!((**elt).name, SCode::Element::CLASS)), None)?;
                                    if isSimpleAPIFunction(&ty) {
                                                tys = metamodelica::cons(ty.clone(), tys.clone());
                                    }
                                    Ok(((), outCache.clone(), tys.clone()))
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { outCache = __wb0; tys = __wb1; break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                _ => {
                                    Ok(())
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            }
                            str1 = Tpl::tplString((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<DAE::Type>>| GenerateAPIFunctionsTpl::getCevalScriptInterface(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<metamodelica::Ref<DAE::Type>>) -> Result<Tpl::Text> + 'static>), tys.clone())?;
                            str2 = Tpl::tplString3((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<DAE::Type>>, __a2: ArcStr, __a3: ArcStr| GenerateAPIFunctionsTpl::getQtInterface(__a0, &__a1, &__a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<metamodelica::Ref<DAE::Type>>, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>), tys.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("::")); ArcStr::from(__mm_s) }, name.clone())?;
                            str3 = Tpl::tplString2((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<DAE::Type>>, __a2: ArcStr| GenerateAPIFunctionsTpl::getQtInterfaceHeaders(__a0, &__a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::List<metamodelica::Ref<DAE::Type>>, ArcStr) -> Result<Tpl::Text> + 'static>), tys.clone(), name.clone())?;
                            Ok((metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), metamodelica::Ref::new(Values::Value::STRING { string: str1.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: str2.clone() }), metamodelica::Ref::new(Values::Value::STRING { string: str3.clone() })] }), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateScriptingAPI", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: false }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateEntryPoint", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str = (*r#str).clone();
                    r#str = Tpl::tplString2((std::sync::Arc::new(move |__a0: Tpl::Text, __a1: metamodelica::Ref<Absyn::Path>, __a2: ArcStr| CodegenCFunctions::generateEntryPoint(__a0, &__a1, &__a2)) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<Absyn::Path>, ArcStr) -> Result<Tpl::Text> + 'static>), path.clone(), r#str.clone())?;
                    System::writeFile(filename.clone(), r#str.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateEntryPoint", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkInterfaceOfPackages", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut interfaceType: metamodelica::List<ArcStr>;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut interfaceTypeAssoc: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
                    sp = SymbolTable::getSCode()?;
                    cl = SCodeUtil::getElementWithPath(sp.clone(), metamodelica::AsArg::as_arg(&path))?;
                    interfaceTypeAssoc = List::map1(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>, __a1: SourceInfo| getInterfaceTypeAssocElt(&__a0, &__a1), SCodeUtil::elementInfo(&cl))?;
                    interfaceType = getInterfaceType(&cl, interfaceTypeAssoc.clone())?;
                    List::map1_0(&sp, &verifyInterfaceType, interfaceType.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkInterfaceOfPackages", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCodeDependenciesMakefile", _) => {
                    Ok(generateSeparateCodeDependenciesMakefile(&args))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCodeDependencies", _) => {
                    Ok(generateSeparateCodeDependencies(&args))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCode", _) => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (v, outCache) = generateSeparateCode(&args, outCache.clone(), env.clone());
                    Ok((v.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportedNames", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cvars: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    (vals, cvars) = getImportedNames(&(ProgramUtil::getPathedClassInProgram(path.clone(), &(SymbolTable::getAbsyn()), false, false)?));
                    v = metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(vals.clone()), ValuesMake::makeArray(cvars.clone())] });
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportedNames", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(metamodelica::nil()), ValuesMake::makeArray(metamodelica::nil())] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ "getMMfileTotalDependencies", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                            let mut strs: metamodelica::List<ArcStr>;
                            strs = getMMfileTotalDependencies(str1.clone(), metamodelica::AsArg::as_arg(&str2))?;
                            vals = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut s in (strs.clone()).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(Values::Value::STRING { string: s.clone() });
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok(ValuesMake::makeArray(vals.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMMfileTotalDependencies", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut pathstr: ArcStr;
                    let mut p: Absyn::Program;
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut b1: bool;
                    let mut oldLanguageStd: Config::LanguageStandard;
                    let mut b = (*b).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    p = SymbolTable::getAbsyn();
                    execStatReset()?;
                    pathstr = Settings::getModelicaPath(Testsuite::isRunning()?)?;
                    strs = List::map(cvars.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?;
                    oldLanguageStd = Config::getLanguageStandard()?;
                    b1 = !(stringEq(&r#str, &(literal!(""))));
                    if b1 {
                        Config::setLanguageStandard(Config::versionStringToStd(r#str.clone()))?;
                    }
                    (p, b) = loadModel(&(list![(path.clone(), literal!("call to loadModel"), strs.clone(), false)]), pathstr.clone(), p.clone(), true, b.clone(), true, requireExactVersion.clone(), false, literal!(""))?;
                    if b1 {
                        Config::setLanguageStandard(oldLanguageStd)?;
                    }
                    Print::clearBuf();
                    SymbolTable::setAbsyn(p.clone())?;
                    execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadModel(")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }))?;
                    outCache = FCore::emptyCache();
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: _ }) => {
                    let mut pathstr: ArcStr;
                    pathstr = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![pathstr.clone()])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: allowWithin }, tail: _ } } } } } }) => {
                    let mut newp: Absyn::Program;
                    let mut name = (*name).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    execStatReset()?;
                    name = Testsuite::friendlyPath(name.clone());
                    newp = loadFile(name.clone(), encoding.clone(), SymbolTable::getAbsyn(), b.clone(), b1.clone(), requireExactVersion.clone(), allowWithin.clone())?;
                    execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadFile(")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }))?;
                    SymbolTable::setAbsyn(newp.clone())?;
                    outCache = FCore::emptyCache();
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFile", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFiles", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: allowWithin }, tail: _ } } } } } } }) => {
                    let mut r#str: ArcStr;
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut newp: Absyn::Program;
                    let mut newps: metamodelica::List<Absyn::Program>;
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    strs = List::mapMap(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0), &fnptr!(Testsuite::friendlyPath, ArcStr))?;
                    newps = Parser::parallelParseFilesToProgramList(strs.clone(), encoding.clone(), i.clone())?;
                    newp = SymbolTable::getAbsyn();
                    for mut p in &*newps {
                        let mut p = p.clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(strs.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        r#str = metamodelica::Own::own(__pa0);
                        strs = metamodelica::Own::own(__pa1);
                        if !(allowWithin.clone()) {
                            checkTopClassWithin(&p, r#str.clone())?;
                        }
                        newp = checkUsesAndUpdateProgram(p.clone(), newp.clone(), b.clone(), Settings::getModelicaPath(Testsuite::isRunning()?)?, b1.clone(), requireExactVersion.clone(), false)?;
                    }
                    SymbolTable::setAbsyn(newp.clone())?;
                    outCache = FCore::emptyCache();
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFiles", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseEncryptedPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: _ } }) => {
                    let mut r#str: ArcStr;
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut b: bool;
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut filename = (*filename).clone();
                    vals = metamodelica::nil();
                    r#str = System::pwd();
                    match '__try0: {
                        filename = unwrap_break_err!(System::realpath(filename.clone()), '__try0);
                        let 0 = (System::cd(System::dirname(filename.clone()))) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        (b, filename) = unwrap_break_err!(unZipEncryptedPackageAndCheckFile(workdir.clone(), filename.clone(), false), '__try0);
                        if b {
                            Error::clearMessages();
                            Print::clearErrorBuf();
                            filename = Testsuite::friendlyPath(filename.clone());
                            paths = unwrap_break_err!(Interactive::parseFile(filename.clone(), literal!("UTF-8"), false), '__try0);
                            vals = unwrap_break_err!(List::map(paths.clone(), &fnptr!(ValuesMake::makeCodeTypeName, metamodelica::Ref<Absyn::Path>)), '__try0);
                        }
                        Ok::<_, &'static str>((b.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            b = __try0_o0;
                        }
                        Err(_) => {
                            b = false;
                        }
                    }
                    let 0 = (System::cd(r#str.clone())) else { return Err("pattern mismatch") };
                    Ok(ValuesMake::makeArray(vals.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseEncryptedPackage", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadEncryptedPackage", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: bval }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: _ } } } } } }) => {
                    let mut r#str: ArcStr;
                    let mut p: Absyn::Program;
                    let mut newp: Absyn::Program;
                    let mut filename = (*filename).clone();
                    let mut b = (*b).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    r#str = System::pwd();
                    match '__try0: {
                        filename = unwrap_break_err!(System::realpath(filename.clone()), '__try0);
                        let 0 = (System::cd(System::dirname(filename.clone()))) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        (b, filename) = unwrap_break_err!(unZipEncryptedPackageAndCheckFile(workdir.clone(), filename.clone(), bval.clone()), '__try0);
                        if b.clone() {
                            unwrap_break_err!(execStatReset(), '__try0);
                            filename = Testsuite::friendlyPath(filename.clone());
                            p = SymbolTable::getAbsyn();
                            newp = unwrap_break_err!(loadFile(filename.clone(), literal!("UTF-8"), p.clone(), b.clone(), b1.clone(), requireExactVersion.clone(), true), '__try0);
                            unwrap_break_err!(execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadFile(")); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })), '__try0);
                            unwrap_break_err!(SymbolTable::setAbsyn(newp.clone()), '__try0);
                        }
                        outCache = FCore::emptyCache();
                        Ok::<_, &'static str>((b.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            b = __try0_o0;
                        }
                        Err(_) => {
                            b = false;
                        }
                    }
                    let 0 = (System::cd(r#str.clone())) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadEncryptedPackage", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "alarm", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: System::alarm(i.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassNames", _) => {
                    Ok(getClassNames(&args)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut filename: ArcStr;
                    let mut r1: metamodelica::Real;
                    let mut r2: metamodelica::Real;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, lastModification: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    filename = metamodelica::Own::own(__pa0);
                    r2 = metamodelica::Own::own(__pa1);
                    let (true, _, __pa2, _) = (System::stat(filename.clone())) else { return Err("pattern mismatch") };
                    r1 = metamodelica::Own::own(__pa2);
                    if !(realEq(r1, r2)) {
                        reloadClass(filename.clone(), encoding.clone())?;
                    }
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: true }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    if '__try0: {
                        unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?])?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadString", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: mergeAST }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: _ } } } } } } }) => {
                    let mut newp: Absyn::Program;
                    let mut parsed: Absyn::Program;
                    let mut r#str = (*r#str).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    r#str = if (!(metamodelica::stringEq(&encoding, &(literal!("UTF-8"))))) {System::iconv(r#str.clone(), encoding.clone(), literal!("UTF-8"))} else {r#str.clone()};
                    parsed = Parser::parsestring(r#str.clone(), name.clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
                    newp = checkUsesAndUpdateProgram(parsed.clone(), SymbolTable::getAbsyn(), b.clone(), Settings::getModelicaPath(Testsuite::isRunning()?)?, b1.clone(), requireExactVersion.clone(), mergeAST.clone())?;
                    SymbolTable::setAbsynLoaded(newp.clone(), parsed.clone())?;
                    outCache = FCore::emptyCache();
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadString", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "help", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: FlagsUtil::printUsage()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "help", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: FlagsUtil::printHelp(&(list![r#str.clone()]))? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTimeStamp", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    let mut r: metamodelica::Real;
                    let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(classpath.clone(), &(SymbolTable::getAbsyn()), false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { lastModification: __pa0, .. }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    r#str = System::ctime(r);
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: r }), metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTimeStamp", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }), metamodelica::Ref::new(Values::Value::STRING { string: literal!("") })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassRestriction", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut r#str: ArcStr;
                    r#str = Interactive::getClassRestriction(classpath.clone(), &(SymbolTable::getAbsyn()));
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "classAnnotationExists", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    b = ProgramUtil::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), metamodelica::AsArg::as_arg(&path), Some(false), &fnptr!(isSome, _))?;
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBooleanClassAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut b: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), metamodelica::AsArg::as_arg(&path), None, &Interactive::getAnnotationExp)?) {
                        Deref @ Absyn::Exp::BOOL { value: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    b = metamodelica::Own::own(__pa0);
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: b }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBooleanClassAnnotation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    Error::addMessage(Error::CLASS_ANNOTATION_DOES_NOT_EXIST.clone(), list![AbsynUtil::pathString(path.clone(), literal!("."), true, false)?, AbsynUtil::pathString(classpath.clone(), literal!("."), true, false)?])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "strtok", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: token }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = System::strtok(r#str.clone(), token.clone());
                    Ok(ValuesMake::makeStringArray(strs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringSplit", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: token }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                    let mut strs: metamodelica::List<ArcStr>;
                    strs = Util::stringSplitAtChar(r#str.clone(), token.clone())?;
                    Ok(ValuesMake::makeStringArray(strs.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringReplace", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                    let mut r#str: ArcStr;
                    r#str = System::stringReplace(str1.clone(), str2.clone(), str3.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: r#str.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkSettings", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(checkSettings()?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "echo", Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ Values::Value::BOOL { boolean: bval }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Settings::setEcho(if (bval.clone()) {1} else {0});
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "numProcessors", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: Config::noProc()? }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScript", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut res: ArcStr;
                    let mut r#str = (*r#str).clone();
                    r#str = Testsuite::friendlyPath(r#str.clone());
                    res = Interactive::evaluate(&(Parser::parseexp(r#str.clone())?), true)?;
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: res.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScript", _) => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("Failed") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exit", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    System::exit(i.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMemorySize", Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: System::getMemorySize() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAllSubtypeOf", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: parentClass } }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: includePartial }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: sort }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
                    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    paths = InteractiveUtil::getAllSubtypeOf(path.clone(), parentClass.clone(), SymbolTable::getAbsyn(), includePartial.clone(), sort.clone())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getReplaceableChoices", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: parentClass } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: includePartial }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: sort }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
                    Ok(InteractiveUtil::getReplaceableChoices(path.clone(), parentClass.clone(), SymbolTable::getAbsyn(), includePartial.clone(), sort.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(CevalScriptOMSimulator::ceval(functionName.clone(), args.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, v) = CevalScriptBackend::cevalInteractiveFunctions3(outCache.clone(), env.clone(), &functionName, args.clone(), msg.clone())?;
                    Ok((v.clone(), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

pub(crate) fn evalCodeTypeName(
    mut val: metamodelica::Ref<Values::Value>,
    mut env: FCore::Graph,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    res = 'mc: {
        let __mc_input = &*val;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: path @ Deref @ Absyn::Path::IDENT { name: _ } } } => {
                    let mut res: metamodelica::Ref<Values::Value> = res.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(FCore::emptyCache(), env.clone(), ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path)))?) {
                        (_, _, _, Deref @ DAE::Binding::VALBOUND { valBound: __pa0 @ Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { .. } }, .. }, _, _, _, _, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    res = metamodelica::Own::own(__pa0);
                    Ok((res.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(val.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn getVariableNames<'__b>(
    mut vars: &'__b metamodelica::List<InteractiveTypes::Variable>,
    mut acc: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match vars {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(acc.reverse())
            },
            Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: Deref @ "$echo", .. }, tail: vs } => {
                { (vars, acc) = (vs, acc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: InteractiveTypes::Variable { varIdent: p, .. }, tail: vs } => {
                { (vars, acc) = (vs, metamodelica::cons(metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: p.clone(), subscripts: metamodelica::nil() }) }) }), acc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getPackageVersion(mut path: metamodelica::Ref<Absyn::Path>, mut p: Absyn::Program) -> Result<ArcStr> {
    let mut version: ArcStr = literal!("");
    let mut evalParamAnn: bool;
    evalParamAnn = Config::getEvaluateParametersInAnnotations()?;
    Config::setEvaluateParametersInAnnotations(true)?;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(ProgramUtil::getNamedAnnotationExp(path.clone(), p.clone(), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("version") })), Some(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") })), &Interactive::getAnnotationExp), '__try0)) {
            Deref @ Absyn::Exp::STRING { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        version = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((version.clone(),))
    } {
        Ok((__try0_o0,)) => {
            version = __try0_o0;
        }
        Err(_) => {
            version = literal!("");
        }
    }
    Config::setEvaluateParametersInAnnotations(evalParamAnn)?;
    Ok(version)
}

fn errorToValue(mut err: &ErrorTypes::TotalMessage) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = (match err.clone() {
        ErrorTypes::TotalMessage {
            msg:
                ErrorTypes::Message {
                    id: mut id,
                    ty: mut ty,
                    severity: mut severity,
                    message: mut message,
                },
            info: mut info,
        } => {
            let mut msgpath: metamodelica::Ref<Absyn::Path>;
            let mut tyVal: metamodelica::Ref<Values::Value>;
            let mut severityVal: metamodelica::Ref<Values::Value>;
            let mut infoVal: metamodelica::Ref<Values::Value>;
            let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut msg_str: ArcStr;
            msg_str = message.clone();
            msgpath = metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("OpenModelica"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: literal!("Scripting"),
                        path: metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: literal!("ErrorMessage"),
                        }),
                    }),
                }),
            });
            tyVal = errorTypeToValue(ty.clone())?;
            severityVal = errorLevelToValue(severity.clone())?;
            infoVal = infoToValue(metamodelica::AsArg::as_arg(&info))?;
            values = list![
                infoVal,
                metamodelica::Ref::new(Values::Value::STRING { string: msg_str }),
                tyVal,
                severityVal,
                metamodelica::Ref::new(Values::Value::INTEGER { integer: id.clone() })
            ];
            metamodelica::Ref::new(Values::Value::RECORD {
                record_: msgpath,
                orderd: values,
                comp: list![
                    literal!("info"),
                    literal!("message"),
                    literal!("kind"),
                    literal!("level"),
                    literal!("id")
                ],
                index: -1,
            })
        }
    });
    Ok(val)
}

fn infoToValue(mut info: &SourceInfo) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = (match info.clone() {
        SourceInfo {
            fileName: mut filename,
            isReadOnly: mut readonly,
            lineNumberStart: mut ls,
            columnNumberStart: mut cs,
            lineNumberEnd: mut le,
            columnNumberEnd: mut ce,
            lastModification: _,
        } => {
            let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut infopath: metamodelica::Ref<Absyn::Path>;
            infopath = metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("OpenModelica"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: literal!("Scripting"),
                        path: metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: literal!("SourceInfo"),
                        }),
                    }),
                }),
            });
            values = list![
                metamodelica::Ref::new(Values::Value::STRING {
                    string: filename.clone()
                }),
                metamodelica::Ref::new(Values::Value::BOOL {
                    boolean: readonly.clone()
                }),
                metamodelica::Ref::new(Values::Value::INTEGER { integer: ls.clone() }),
                metamodelica::Ref::new(Values::Value::INTEGER { integer: cs.clone() }),
                metamodelica::Ref::new(Values::Value::INTEGER { integer: le.clone() }),
                metamodelica::Ref::new(Values::Value::INTEGER { integer: ce.clone() })
            ];
            metamodelica::Ref::new(Values::Value::RECORD {
                record_: infopath,
                orderd: values,
                comp: list![
                    literal!("filename"),
                    literal!("readonly"),
                    literal!("lineStart"),
                    literal!("columnStart"),
                    literal!("lineEnd"),
                    literal!("columnEnd")
                ],
                index: -1,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(val)
}

fn makeErrorEnumLiteral(
    mut enumName: ArcStr,
    mut enumField: ArcStr,
    mut index: i32,
) -> metamodelica::Ref<Values::Value> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = metamodelica::Ref::new(Values::Value::ENUM_LITERAL {
        name: metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("OpenModelica"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("Scripting"),
                    path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: enumName,
                        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: enumField }),
                    }),
                }),
            }),
        }),
        index: index,
    });
    val
}

fn errorTypeToValue(mut ty: ErrorTypes::MessageType) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = (match ty {
        ErrorTypes::MessageType::SYNTAX { .. } => makeErrorEnumLiteral(literal!("ErrorKind"), literal!("syntax"), 1),
        ErrorTypes::MessageType::GRAMMAR { .. } => makeErrorEnumLiteral(literal!("ErrorKind"), literal!("grammar"), 2),
        ErrorTypes::MessageType::TRANSLATION { .. } => {
            makeErrorEnumLiteral(literal!("ErrorKind"), literal!("translation"), 3)
        }
        ErrorTypes::MessageType::SYMBOLIC { .. } => {
            makeErrorEnumLiteral(literal!("ErrorKind"), literal!("symbolic"), 4)
        }
        ErrorTypes::MessageType::SIMULATION { .. } => {
            makeErrorEnumLiteral(literal!("ErrorKind"), literal!("runtime"), 5)
        }
        ErrorTypes::MessageType::SCRIPTING { .. } => {
            makeErrorEnumLiteral(literal!("ErrorKind"), literal!("scripting"), 6)
        }
        _ => {
            metamodelica::print(literal!("errorTypeToValue failed\n"));
            return Err("fail");
        }
    });
    Ok(val)
}

fn errorLevelToValue(mut severity: ErrorTypes::Severity) -> Result<metamodelica::Ref<Values::Value>> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = (match severity {
        ErrorTypes::Severity::INTERNAL { .. } => makeErrorEnumLiteral(literal!("ErrorLevel"), literal!("internal"), 1),
        ErrorTypes::Severity::ERROR { .. } => makeErrorEnumLiteral(literal!("ErrorLevel"), literal!("error"), 2),
        ErrorTypes::Severity::WARNING { .. } => makeErrorEnumLiteral(literal!("ErrorLevel"), literal!("warning"), 3),
        ErrorTypes::Severity::NOTIFICATION { .. } => {
            makeErrorEnumLiteral(literal!("ErrorLevel"), literal!("notification"), 4)
        }
        _ => {
            metamodelica::print(literal!("errorLevelToValue failed\n"));
            return Err("fail");
        }
    });
    Ok(val)
}

fn generateFunctionName(mut functionPath: &metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    let mut functionName: ArcStr;
    functionName = AbsynUtil::pathStringUnquoteReplaceDot(functionPath, literal!("_"))?;
    Ok(functionName)
}

fn generateFunctionFileName(mut functionPath: &metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    let mut functionName: ArcStr;
    let mut n1: ArcStr;
    let mut n2: ArcStr;
    functionName = AbsynUtil::pathStringUnquoteReplaceDot(functionPath, literal!("_"))?;
    if ((functionName).len() as i32) > Global::maxFunctionFileLength.clone() {
        n1 = AbsynUtil::pathFirstIdent(functionPath);
        n2 = AbsynUtil::pathLastIdent(functionPath);
        functionName = System::unquoteIdentifier({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*n1);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*n2);
            ArcStr::from(__mm_s)
        });
        functionName = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*functionName);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(tick()));
            ArcStr::from(__mm_s)
        };
    }
    Ok(functionName)
}

pub(crate) fn getFunctionDependencies(
    mut cache: &FCore::Cache,
    mut functionName: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    DAE::Function,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut mainFunction: DAE::Function;
    let mut dependencies: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    funcs = FCore::getFunctionTree(cache);
    mainFunction = DAEUtil::getNamedFunction(functionName.clone(), &funcs)?;
    dependencies = SimCodeFunctionUtil::getCalledFunctionsInFunction(functionName, &funcs)?;
    Ok((mainFunction, dependencies, funcs))
}

pub(crate) fn collectDependencies(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut functionName: metamodelica::Ref<Absyn::Path>,
) -> Result<(
    FCore::Cache,
    DAE::Function,
    metamodelica::List<DAE::Function>,
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
)> {
    let mut outCache: FCore::Cache;
    let mut mainFunction: DAE::Function;
    let mut dependencies: metamodelica::List<DAE::Function>;
    let mut metarecordTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut uniontypePaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (mainFunction, paths, funcs) = getFunctionDependencies(&inCache, functionName)?;
    dependencies = List::map1(
        paths,
        &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<AvlTreePathFunction::Tree>| {
            DAEUtil::getNamedFunction(__a0, &__a1)
        },
        funcs,
    )?;
    dependencies = List::setDifference(dependencies, &(list![mainFunction.clone()]))?;
    uniontypePaths = DAEUtil::getUniontypePaths(dependencies.clone(), &(metamodelica::nil()))?;
    (outCache, metarecordTypes) = Lookup::lookupMetarecordsRecursive(inCache, env, &uniontypePaths)?;
    Ok((outCache, mainFunction, dependencies, metarecordTypes))
}

pub(crate) fn cevalGenerateFunction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut program: &Absyn::Program,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, ArcStr, ArcStr)> {
    let mut outCache: FCore::Cache;
    let mut functionName: ArcStr;
    let mut functionFileName: ArcStr;
    (outCache, functionName, functionFileName) = 'mc: {
        let __mc_input = (inCache, inEnv, inPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && !(Flags::isSet(Flags::GENERATE_CODE_CHEAT.clone())?))) { return Err("guard") }
                    let mut pathstr: ArcStr;
                    let mut fileName: ArcStr;
                    let mut mainFunction: DAE::Function;
                    let mut dependencies: metamodelica::List<DAE::Function>;
                    let mut metarecordTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    (cache, mainFunction, dependencies, metarecordTypes) = collectDependencies(cache.clone(), env.clone(), path.clone())?;
                    pathstr = generateFunctionName(metamodelica::AsArg::as_arg(&path))?;
                    fileName = generateFunctionFileName(metamodelica::AsArg::as_arg(&path))?;
                    translateFunctions(program, fileName.clone(), Some(mainFunction.clone()), dependencies.clone(), &metarecordTypes, metamodelica::nil())?;
                    if !metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                        compileModel(fileName.clone(), metamodelica::nil(), literal!(""), metamodelica::nil())?;
                    }
                    Ok((cache.clone(), pathstr.clone(), fileName.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && Flags::isSet(Flags::GENERATE_CODE_CHEAT.clone())?)) { return Err("guard") }
                    let mut pathstr: ArcStr;
                    let mut fileName: ArcStr;
                    let mut dependencies: metamodelica::List<DAE::Function>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    funcs = FCore::getFunctionTree(metamodelica::AsArg::as_arg(&cache));
                    pathstr = generateFunctionName(metamodelica::AsArg::as_arg(&path))?;
                    fileName = generateFunctionFileName(metamodelica::AsArg::as_arg(&path))?;
                    dependencies = DAEUtil::getFunctionList(&funcs, false)?;
                    translateFunctions(program, fileName.clone(), None, dependencies.clone(), &(metamodelica::nil()), metamodelica::nil())?;
                    Ok((cache.clone(), pathstr.clone(), fileName.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && Flags::isSet(Flags::FAILTRACE.clone())?)) { return Err("guard") }
                    let mut pathstr: ArcStr;
                    let mut fileName: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, false) = (Static::isExternalObjectFunction(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&path))) else { return Err("pattern mismatch") };
                    cache = metamodelica::Own::own(__pa0);
                    pathstr = generateFunctionName(metamodelica::AsArg::as_arg(&path))?;
                    fileName = generateFunctionFileName(metamodelica::AsArg::as_arg(&path))?;
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.cevalGenerateFunction failed:\nfunction: ")); __mm_s.push_str(&*pathstr); __mm_s.push_str(&*literal!("\nfile: ")); __mm_s.push_str(&*fileName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, functionName, functionFileName))
}

fn matchQualifiedCalls(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAcc: metamodelica::List<ArcStr>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::List<ArcStr>) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outAcc: metamodelica::List<ArcStr>;
    outAcc = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, .. }, .. } => {
            List::consOnTrue(!(listMember(name.clone(), inAcc.clone())), name.clone(), inAcc)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. } => {
            List::consOnTrue(!(listMember(name.clone(), inAcc.clone())), name.clone(), inAcc)
        },
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, .. }, ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: false, .. } } => {
            List::consOnTrue(!(listMember(name.clone(), inAcc.clone())), name.clone(), inAcc)
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, .. } => {
            List::consOnTrue(!(listMember(name.clone(), inAcc.clone())), name.clone(), inAcc)
        },
        _ => {
            inAcc
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outAcc)
}

fn instantiateDaeFunctions(
    mut icache: FCore::Cache,
    mut ienv: FCore::Graph,
    mut ipaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<FCore::Cache> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((icache, ienv, ipaths)) {
            (cache, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(cache.clone())
            },
            (cache, env, Deref @ metamodelica::ListNode::Cons { head: path, tail: paths }) => {
                let mut cache = (*cache).clone();
                let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunctionForceInst(cache.clone(), env.clone(), path.clone(), false, None, true)) else { return Err("pattern mismatch") };
                cache = metamodelica::Own::own(__pa0);
                { (icache, ienv, ipaths) = (cache.clone(), env.clone(), paths.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn generateFunctions<'__b>(
    mut icache: FCore::Cache,
    mut ienv: FCore::Graph,
    mut p: &'__b Absyn::Program,
    mut fullScodeProgram: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut isp: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut cleanCache: bool,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache;
    let mut env: FCore::Graph;
    (cache, env) = (::match_deref::match_deref! { match &((icache, ienv, isp)) {
        (__esc_cache, __esc_env, Deref @ metamodelica::ListNode::Nil) => {
            cache = (*__esc_cache).clone();
            env = (*__esc_env).clone();
            (cache.clone(), env.clone())
        },
        (__esc_cache, __esc_env, Deref @ metamodelica::ListNode::Cons { head: cl @ Deref @ SCode::Element::CLASS { name, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, restriction: restr, info, .. }, tail: sp }) => {
            cache = (*__esc_cache).clone();
            env = (*__esc_env).clone();
            let () = (match restr.clone() {
        SCode::Restriction::R_PACKAGE { .. } => (),
        SCode::Restriction::R_UNIONTYPE { .. } => (),
        _ => {
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Only package and uniontype is supported as top-level classes in OpenModelica.")], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
    });
            (cache, env) = generateFunctions2(cache.clone(), env.clone(), p, fullScodeProgram.clone(), metamodelica::AsArg::as_arg(&cl), name.clone(), info.clone(), cleanCache)?;
            (cache, env) = generateFunctions(cache.clone(), env.clone(), p, fullScodeProgram, sp.clone(), cleanCache)?;
            (cache.clone(), env.clone())
        },
        (_, _, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED { .. }, name, info: info @ SourceInfo { fileName: file, .. }, .. }, tail: _ }) => {
            let mut n: i32;
            (n, _) = System::regex(file.clone(), literal!("ModelicaBuiltin.mo$"), 1, false, false);
            Error::assertion(n > 0, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Not an encapsulated class (required for separate compilation): ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((cache, env))
}

fn generateFunctions2(
    mut icache: FCore::Cache,
    mut ienv: FCore::Graph,
    mut p: &Absyn::Program,
    mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut name: ArcStr,
    mut info: SourceInfo,
    mut cleanCache: bool,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache;
    let mut env: FCore::Graph;
    (cache, env) = 'mc: {
        let __mc_input = (icache.clone(), ienv, info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, SourceInfo { fileName: mut file, .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            ::match_deref::match_deref! { match &(System::regex(file.clone(), literal!("ModelicaBuiltin.mo$"), 1, false, false)) {
                (1, _) => (),
                _ => return Err("pattern mismatch"),
            } };
            Ok((cache.clone(), env.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut dependencies: metamodelica::List<ArcStr>;
            let mut strs: metamodelica::List<ArcStr>;
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut pathsMetarecord: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut d: metamodelica::List<DAE::Function>;
            let mut nameHeader: ArcStr;
            let mut r#str: ArcStr;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut metarecords: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut t: metamodelica::Ref<DAE::Type>;
            cache = if (cleanCache) {
                FCore::emptyCache()
            } else {
                cache.clone()
            };
            if SCodeUtil::isPartial(cl) {
                paths = metamodelica::nil();
                pathsMetarecord = metamodelica::nil();
            } else {
                path = metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
                });
                elements = getNonPartialElementsForInstantiatedClass(sp.clone(), cl, path.clone())?;
                (paths, pathsMetarecord) = List::fold22(
                    &elements,
                    &move |__a0: metamodelica::Ref<SCode::Element>,
                           __a1: metamodelica::Ref<Absyn::Path>,
                           __a2: metamodelica::List<metamodelica::Ref<SCode::Element>>,
                           __a3: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
                           __a4: metamodelica::List<metamodelica::Ref<Absyn::Path>>| {
                        findFunctionsToCompile(&__a0, __a1, __a2, __a3, __a4)
                    },
                    path.clone(),
                    sp.clone(),
                    metamodelica::nil(),
                    metamodelica::nil(),
                )?;
            }
            metarecords = metamodelica::nil();
            for mut mr in &*pathsMetarecord {
                (cache, t, _) = Lookup::lookupType(cache.clone(), env.clone(), mr.clone(), Some(info.clone()))?;
                metarecords = metamodelica::cons(t.clone(), metarecords.clone());
            }
            cache = instantiateDaeFunctions(cache.clone(), env.clone(), paths.clone())?;
            InstHashTable::release()?;
            funcs = FCore::getFunctionTree(&cache);
            d = List::map2(
                paths.clone(),
                &move |__a0: metamodelica::Ref<Absyn::Path>,
                       __a1: metamodelica::Ref<AvlTreePathFunction::Tree>,
                       __a2: SourceInfo| DAEUtil::getNamedFunctionWithError(__a0, &__a1, &__a2),
                funcs.clone(),
                info.clone(),
            )?;
            let (_, (_, __pa0)) = DAEUtil::traverseDAEFunctions(
                d.clone(),
                (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
                (
                    (std::sync::Arc::new(fnptr!(
                        matchQualifiedCalls,
                        metamodelica::Ref<DAE::Exp>,
                        metamodelica::List<ArcStr>
                    ))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    metamodelica::List<ArcStr>,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<ArcStr>)>
                                + 'static,
                        >),
                    metamodelica::nil(),
                ),
            )?;
            dependencies = metamodelica::Own::own(__pa0);
            dependencies = List::sort(
                dependencies.clone(),
                (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1))
                }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
            )?;
            dependencies = List::map1(
                dependencies.clone(),
                &fnptr!(stringAppend, ArcStr, ArcStr),
                literal!(".h"),
            )?;
            nameHeader = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(".h"));
                ArcStr::from(__mm_s)
            };
            strs = List::map1r(
                metamodelica::cons(nameHeader.clone(), dependencies.clone()),
                &fnptr!(stringAppend, ArcStr, ArcStr),
                literal!("$(GEN_DIR)"),
            )?;
            System::writeFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(".deps"));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$(GEN_DIR)"));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(".o: $(GEN_DIR)"));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(".c"));
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*stringDelimitList(strs.clone(), literal!(" ")));
                    ArcStr::from(__mm_s)
                },
            )?;
            dependencies = List::map1(
                dependencies.clone(),
                &fnptr!(stringAppend, ArcStr, ArcStr),
                literal!("\""),
            )?;
            dependencies = List::map1r(
                dependencies.clone(),
                &fnptr!(stringAppend, ArcStr, ArcStr),
                literal!("#include \""),
            )?;
            translateFunctions(
                p,
                name.clone(),
                None,
                d.clone(),
                &(metamodelica::nil()),
                dependencies.clone(),
            )?;
            r#str = Tpl::tplString(
                (std::sync::Arc::new(
                    move |__a0: Tpl::Text, __a1: metamodelica::List<metamodelica::Ref<DAE::Type>>| {
                        Unparsing::programExternalHeaderFromTypes(__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Tpl::Text,
                                metamodelica::List<metamodelica::Ref<DAE::Type>>,
                            ) -> Result<Tpl::Text>
                            + 'static,
                    >),
                metarecords.clone(),
            )?;
            System::writeFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!("_records.c"));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("#include <meta/meta_modelica.h>\n"));
                    __mm_s.push_str(&*r#str);
                    ArcStr::from(__mm_s)
                },
            )?;
            cache = if (cleanCache) { icache.clone() } else { cache.clone() };
            Ok((cache.clone(), env.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addSourceMessage(
                &(Error::SEPARATE_COMPILATION_PACKAGE_FAILED.clone()),
                list![name.clone()],
                &info,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((cache, env))
}

fn findFunctionsToCompile(
    mut elt: &metamodelica::Ref<SCode::Element>,
    mut pathPrefix: metamodelica::Ref<Absyn::Path>,
    mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut acc: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut accMetarecord: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    metamodelica::List<metamodelica::Ref<Absyn::Path>>,
)> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut pathsMetarecord: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut name: ArcStr;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let __pa0 = ::match_deref::match_deref! { match &((*elt)) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    path = AbsynUtil::joinPaths(pathPrefix, metamodelica::Ref::new(Absyn::Path::IDENT { name: name }))?;
    paths = if (SCodeUtil::isFunction(elt)) {
        metamodelica::cons(path.clone(), acc)
    } else {
        acc
    };
    pathsMetarecord = (match &**elt {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_METARECORD { .. },
            ..
        } => metamodelica::cons(path.clone(), accMetarecord),
        _ => accMetarecord,
    });
    elements = getNonPartialElementsForInstantiatedClass(sp.clone(), elt, path.clone())?;
    (paths, pathsMetarecord) = List::fold22(
        &elements,
        &move |__a0: metamodelica::Ref<SCode::Element>,
               __a1: metamodelica::Ref<Absyn::Path>,
               __a2: metamodelica::List<metamodelica::Ref<SCode::Element>>,
               __a3: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
               __a4: metamodelica::List<metamodelica::Ref<Absyn::Path>>| {
            findFunctionsToCompile(&__a0, __a1, __a2, __a3, __a4)
        },
        path,
        sp,
        paths,
        pathsMetarecord,
    )?;
    Ok((paths, pathsMetarecord))
}

fn getNonPartialElementsForInstantiatedClass(
    mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut p: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut env: FCore::Graph;
    let mut skip: bool;
    let mut eltsTmp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    skip = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __esc_eltsTmp, .. }, .. } => {
            eltsTmp = (*__esc_eltsTmp).clone();
            !(List::any(metamodelica::AsArg::as_arg(&eltsTmp), &move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isElementExtendsOrClassExtends(&__a0)) })?)
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(skip) {
        if '__try0: {
            ErrorExt::setCheckpoint(literal!("getNonPartialElementsForInstantiatedClass"));
            (_, env, _, _) = unwrap_break_err!(Inst::instantiateClass(FCore::emptyCache(), InnerOuter::emptyInstHierarchy().clone(), sp.clone(), AbsynUtil::makeNotFullyQualified(p.clone()), false, true, false), '__try0);
            elts = unwrap_break_err!(FCore::RefTree::fold(&(FNode::children(&(FNode::fromRef(unwrap_break_err!(FGraph::lastScopeRef(&env), '__try0))))), &move |__a0: ArcStr, __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>, __a2: metamodelica::List<metamodelica::Ref<SCode::Element>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(addNonPartialClassRef(&__a0, __a1, __a2)) }, metamodelica::nil()), '__try0);
            ErrorExt::rollBack(literal!("getNonPartialElementsForInstantiatedClass"));
            return Ok(elts);
            Ok::<(), &'static str>(())
        }.is_err() {
        }
        ErrorExt::rollBack(literal!("getNonPartialElementsForInstantiatedClass"));
    }
    elts = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __esc_elts, .. }, .. } => {
            elts = (*__esc_elts).clone();
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (elts.clone()).into_iter().cloned() {
            if !(!(SCodeUtil::isPartial(&(e.clone()))) && SCodeUtil::isClass(&(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elts)
}

fn addNonPartialClassRef(
    mut name: &ArcStr,
    mut r#ref: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
    mut accum: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut classes: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut e: metamodelica::Ref<SCode::Element>;
    classes = (::match_deref::match_deref! { match &(FNode::fromRef(r#ref)) {
        Deref @ FCore::Node { data: Deref @ FCore::Data::CL { e: __esc_e @ Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, .. }, .. }, .. } => {
            e = (*__esc_e).clone();
            metamodelica::cons(e.clone(), accum)
        },
        _ => accum,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classes
}

pub(crate) fn cevalCallFunction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut r#impl: bool,
    mut inMsg: Absyn::Msg,
    mut numIter: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            &*inExp,
            inValuesValueLst.clone(),
            inMsg.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, .. }, vallst, msg) => {
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut cache = (*cache).clone();
                    (cache, newval) = Ceval::cevalKnownExternalFuncs(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&funcpath), metamodelica::AsArg::as_arg(&vallst), metamodelica::AsArg::as_arg(&msg))?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, .. }, _, msg) => {
                    let true = (FGraph::isNotEmpty(metamodelica::AsArg::as_arg(&env))) else { return Err("pattern mismatch") };
                    cevalIsExternalObjectConstructor(cache.clone(), metamodelica::AsArg::as_arg(&funcpath), env.clone(), msg.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: complexName }, varLst, .. }, .. }, .. }, pubVallst, msg) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut proVallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut pubVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut proVarLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut pubVarNames: metamodelica::List<ArcStr>;
                    let mut proVarNames: metamodelica::List<ArcStr>;
                    let mut varNames: metamodelica::List<ArcStr>;
                    let mut cache = (*cache).clone();
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: record constructor: func: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" type path: ")); __mm_s.push_str(&*AbsynUtil::pathString(complexName.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    }
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&funcpath), metamodelica::AsArg::as_arg(&complexName))) else { return Err("pattern mismatch") };
                    (pubVarLst, proVarLst) = List::splitOnTrue(metamodelica::AsArg::as_arg(&varLst), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::isPublicVar(&__a0)) })?;
                    expl = List::map1(proVarLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<Absyn::Path>| Types::getBindingExp(&__a0, __a1), funcpath.clone())?;
                    (cache, proVallst) = Ceval::cevalList(cache.clone(), env.clone(), expl.clone(), r#impl, msg.clone(), numIter)?;
                    pubVarNames = List::map(pubVarLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::varName(&__a0)) })?;
                    proVarNames = List::map(proVarLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::varName(&__a0)) })?;
                    varNames = listAppend(pubVarNames.clone(), proVarNames.clone());
                    vallst = listAppend(pubVallst.clone(), proVallst.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: varNames.clone(), index: -1 })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, attr: Deref @ DAE::CallAttributes { ty, builtin: false, .. }, .. }, _, msg) => {
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut bIsCompleteFunction: bool;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), metamodelica::AsArg::as_arg(&funcpath), env.clone(), msg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: try to evaluate or generate function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    }
                    bIsCompleteFunction = isCompleteFunction(cache.clone(), env.clone(), funcpath.clone());
                    let false = (Types::hasMetaArray(ty.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: is complete function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*if (bIsCompleteFunction) {literal!("[true]")} else {literal!("[false]")}); ArcStr::from(__mm_s) })?;
                    }
                    (cache, newval) = cevalCallFunctionEvaluateOrGenerate(inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), r#impl, inMsg.clone(), bIsCompleteFunction)?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, _, msg) => {
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), metamodelica::AsArg::as_arg(&funcpath), env.clone(), msg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    let false = (isCompleteFunction(cache.clone(), env.clone(), funcpath.clone())) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: constant evaluation failed (not complete function): ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalCallFunctionEvaluateOrGenerate(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut r#impl: bool,
    mut inMsg: Absyn::Msg,
    mut bIsCompleteFunction: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    let mut numCheckpoints: i32;
    if (openmodelica_util::Globals::stackoverFlowIndex.with(|__root| __root.borrow().clone())).is_none() {
        {
            let __v = Some(1);
            openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
        };
        numCheckpoints = ErrorExt::getNumCheckpoints();
        let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
            StackOverflow::clearStacktraceMessages();
            (outCache, outValue) = cevalCallFunctionEvaluateOrGenerate2(
                inCache.clone(),
                inEnv.clone(),
                &inExp,
                inValuesValueLst.clone(),
                r#impl,
                inMsg.clone(),
                bIsCompleteFunction,
            )?;
            Ok(false)
        });
        match __cp0 {
            Ok(__returned) => {
                if __returned? {
                    return Ok((outCache.clone(), outValue.clone()));
                }
            }
            Err(_) => {
                let _rearm = metamodelica::heap_limit::RearmOnDrop;
                {
                    let __v = None;
                    openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
                };
                ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Stack overflow when evaluating function call: "));
                        __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?);
                        __mm_s.push_str(&*literal!("...\n"));
                        __mm_s.push_str(&*stringDelimitList(
                            StackOverflow::readableStacktraceMessages()?,
                            literal!("\n"),
                        ));
                        ArcStr::from(__mm_s)
                    },
                    (match inMsg.clone() {
                        Absyn::Msg::MSG { info: mut info } => info.clone(),
                        _ => {
                            metamodelica::sourceInfo!("Script/CevalScript.mo")
                        }
                    }),
                )?;
                StackOverflow::clearStacktraceMessages();
                outCache = inCache.clone();
                outValue = openmodelica_frontend_types::Values::Value::interned_META_FAIL();
            }
        }
        {
            let __v = None;
            openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
        };
    } else {
        (outCache, outValue) = cevalCallFunctionEvaluateOrGenerate2(
            inCache,
            inEnv,
            &inExp,
            inValuesValueLst,
            r#impl,
            inMsg,
            bIsCompleteFunction,
        )?;
    }
    Ok((outCache, outValue))
}

fn cevalCallFunctionEvaluateOrGenerate2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inValuesValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut r#impl: bool,
    mut inMsg: Absyn::Msg,
    mut bIsCompleteFunction: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inExp, inValuesValueLst, inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, vallst, msg) => {
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut sc: metamodelica::Ref<SCode::Element>;
                    let mut func: DAE::Function;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (Flags::isSet(Flags::EVAL_FUNC.clone())?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), metamodelica::AsArg::as_arg(&funcpath), env.clone(), msg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    match '__try1: {
                        func = unwrap_break_err!(FCore::getCachedInstFunc(metamodelica::AsArg::as_arg(&cache), funcpath.clone()), '__try1);
                        Ok::<_, &'static str>((func.clone(),))
                    } {
                        Ok((__try1_o0,)) => {
                            func = __try1_o0;
                        }
                        Err(_) => {
                            let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&funcpath), None)?) {
                                        (__pa2, __pa3 @ Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, .. }, __pa4) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                                        _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa2);
                            sc = metamodelica::Own::own(__pa3);
                            env = metamodelica::Own::own(__pa4);
                            isCevaluableFunction(&sc)?;
                            (cache, env, _) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, sc.clone(), metamodelica::nil())?;
                            func = FCore::getCachedInstFunc(metamodelica::AsArg::as_arg(&cache), funcpath.clone())?;
                        }
                    }
                    (cache, newval) = CevalFunction::evaluate(cache.clone(), env.clone(), &func, metamodelica::AsArg::as_arg(&vallst))?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, vallst, msg) => {
                    if !((bIsCompleteFunction && Flags::isSet(Flags::GEN.clone())?)) { return Err("guard") }
                    let mut newval: metamodelica::Ref<Values::Value>;
                    let mut print_debug: bool;
                    let mut p: Absyn::Program;
                    let mut libHandle: i32;
                    let mut funcHandle: i32;
                    let mut funcstr: ArcStr;
                    let mut fileName: ArcStr;
                    let mut info: SourceInfo;
                    let mut w: Absyn::Within;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), metamodelica::AsArg::as_arg(&funcpath), env.clone(), msg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: [SOME SYMTAB] not in in CF list: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    p = SymbolTable::getAbsyn();
                    (cache, funcstr, fileName) = cevalGenerateFunction(cache.clone(), env.clone(), &p, funcpath.clone())?;
                    print_debug = Flags::isSet(Flags::DYN_LOAD.clone())?;
                    execStatReset()?;
                    if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                        newval = CodegenWasmJitFunctions::loadAndExecute(fileName.clone(), funcstr.clone(), vallst.clone());
                    } else {
                        libHandle = System::loadLibrary({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }, true, print_debug)?;
                        funcHandle = System::lookupFunction(libHandle, stringAppend(literal!("in_"), funcstr.clone()))?;
                        newval = DynLoad::executeFunction(funcHandle, vallst.clone(), print_debug)?;
                        if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
                            System::freeFunction(funcHandle, print_debug)?;
                        }
                        System::freeLibrary(libHandle, print_debug)?;
                    }
                    execStat(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("executeFunction(")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }))?;
                    let __pa1 = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(funcpath.clone(), &p, false, false)?) {
                        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, info: __pa1, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    info = metamodelica::Own::own(__pa1);
                    w = ProgramUtil::buildWithin(funcpath.clone())?;
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: Updating build time for function path: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" within: ")); __mm_s.push_str(&*Dump::unparseWithin(w.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: [SOME SYMTAB] not in in CF list [finished]: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    Ok((cache.clone(), newval.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::CALL { path: funcpath, .. }, _, _) => {
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: FAILED to constant evaluate function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    let false = (Flags::isSet(Flags::GEN.clone())?) else { return Err("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- codegeneration is turned off. switch \"nogen\" flag off\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outValue))
}

fn cevalIsExternalObjectConstructor(
    mut cache: FCore::Cache,
    mut funcpath: &metamodelica::Ref<Absyn::Path>,
    mut env: FCore::Graph,
    mut msg: Absyn::Msg,
) -> Result<()> {
    let mut funcpath2: metamodelica::Ref<Absyn::Path>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut info: Option<SourceInfo>;
    let () = (match (env.clone(), msg.clone()) {
        (FCore::Graph::EG { name: _ }, Absyn::Msg::NO_MSG { .. }) => return Err("fail"),
        (_, Absyn::Msg::NO_MSG { .. }) => {
            let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(funcpath)?) {
                (__pa0, Deref @ Absyn::Path::IDENT { name: Deref @ "constructor" }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            funcpath2 = metamodelica::Own::own(__pa0);
            info = if (msg == openmodelica_ast::Absyn::Msg::NO_MSG) {
                None
            } else {
                Some(Absyn::dummyInfo.clone())
            };
            (_, tp, _) = Lookup::lookupType(cache, env, funcpath2, info)?;
            Types::externalObjectConstructorType(&tp)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkLibraryUsage(mut inLibrary: ArcStr, mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut isUsed: bool;
    isUsed = (match &**inExp {
        Absyn::Exp::STRING { value: s } => stringEq(&s, &inLibrary),
        Absyn::Exp::ARRAY { arrayExp: exps } => List::isMemberOnTrue(
            inLibrary,
            exps,
            &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::Exp>| checkLibraryUsage(__a0, &__a1),
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(isUsed)
}

fn isCevaluableFunction(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(fid), annotation_: Some(Deref @ SCode::Annotation { modification: r#mod }), .. }), .. }, .. } => {
            let mut lib: metamodelica::Ref<Absyn::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(metamodelica::AsArg::as_arg(&r#mod), &(literal!("Library")))?) {
                Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            lib = metamodelica::Own::own(__pa0);
            let true = (checkLibraryUsage(literal!("Lapack"), &lib)? || checkLibraryUsage(literal!("lapack"), &lib)?) else { return Err("pattern mismatch") };
            isCevaluableFunction2(metamodelica::AsArg::as_arg(&fid))?;
            ()
        },
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => {
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn isCevaluableFunction2(mut inFuncName: &ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inFuncName.clone()) {
        Deref @ "dgbsv" => (),
        Deref @ "dgeev" => (),
        Deref @ "dgegv" => (),
        Deref @ "dgels" => (),
        Deref @ "dgelsx" => (),
        Deref @ "dgelsy" => (),
        Deref @ "dgeqpf" => (),
        Deref @ "dgesv" => (),
        Deref @ "dgesvd" => (),
        Deref @ "dgetrf" => (),
        Deref @ "dgetri" => (),
        Deref @ "dgetrs" => (),
        Deref @ "dgglse" => (),
        Deref @ "dgtsv" => (),
        Deref @ "dorgqr" => (),
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn isSimpleAPIFunction(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_FUNCTION {
            functionAttributes:
                DAE::FunctionAttributes {
                    isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. },
                    ..
                },
            funcArg: __ty_funcArg,
            funcResultType: __ty_funcResultType,
            ..
        } => {
            isSimpleAPIFunctionArg(__ty_funcResultType.clone())
                && ({
                    let mut __acc: Option<bool> = None;
                    for mut fa in (__ty_funcArg.clone()).into_iter().cloned() {
                        let __x = (match &*fa.clone() {
                            DAE::FuncArg { .. } => isSimpleAPIFunctionArg(fa.ty.clone()),
                        });
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x < __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or(true)
                })
        }
        _ => false,
    });
    b
}

fn isSimpleAPIFunctionArg(mut ty: metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &*ty {
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_REAL { .. } => return true,
            DAE::Type::T_BOOL { .. } => return true,
            DAE::Type::T_STRING { .. } => return true,
            DAE::Type::T_NORETCALL { .. } => return true,
            DAE::Type::T_ARRAY { ty: __ty_ty, .. } => {
                ty = __ty_ty.clone();
                continue '__tco;
            }
            DAE::Type::T_CODE {
                ty: DAE::CodeType::C_TYPENAME { .. },
            } => return true,
            DAE::Type::T_TUPLE { types: __ty_types, .. } => {
                return ({
                    let mut __acc: Option<bool> = None;
                    for mut t in (__ty_types.clone()).into_iter().cloned() {
                        let __x = isSimpleAPIFunctionArg(t.clone());
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x < __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or(true)
                });
            }
            _ => return false,
        }
    }
}

fn verifyInterfaceType(
    mut elt: metamodelica::Ref<SCode::Element>,
    mut expected: metamodelica::List<ArcStr>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&*elt, &*expected);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { moved: true, .. }, .. }, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. }, Deref @ metamodelica::ListNode::Cons { head: name, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotation(metamodelica::AsArg::as_arg(&ann), &(literal!("__OpenModelica_Interface")))) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    info = metamodelica::Own::own(__pa1);
                    Error::assertionOrAddSourceMessage(listMember(r#str.clone(), expected.clone()), &(Error::MISMATCHING_INTERFACE_TYPE.clone()), list![r#str.clone(), name.clone()], &info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::unparseElementStr(elt.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Error::addSourceMessage(&(Error::MISSING_INTERFACE_TYPE.clone()), metamodelica::nil(), &(SCodeUtil::elementInfo(&elt)))?;
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

fn getInterfaceType(
    mut elt: &metamodelica::Ref<SCode::Element>,
    mut assoc: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut it: metamodelica::List<ArcStr> = metamodelica::nil();
    it = 'mc: {
        let __mc_input = &**elt;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
                    let mut r#str: ArcStr;
                    let mut it: metamodelica::List<ArcStr> = it.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(metamodelica::AsArg::as_arg(&ann), &(literal!("__OpenModelica_Interface")))) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    it = Util::assoc(r#str.clone(), assoc.clone())?;
                    Ok((it.clone(), it.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            it = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::MISSING_INTERFACE_TYPE.clone()), metamodelica::nil(), &(SCodeUtil::elementInfo(elt)))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(it)
}

fn getInterfaceTypeAssocElt(
    mut val: &metamodelica::Ref<Values::Value>,
    mut info: &SourceInfo,
) -> Result<(ArcStr, metamodelica::List<ArcStr>)> {
    let mut assoc: (ArcStr, metamodelica::List<ArcStr>);
    assoc = (::match_deref::match_deref! { match val {
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: _ }, .. } => {
            Error::addSourceMessage(&(Error::MISSING_INTERFACE_TYPE.clone()), metamodelica::nil(), info)?;
            return Err("fail")
        },
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: vals }, .. } => {
            let mut strs: metamodelica::List<ArcStr>;
            strs = List::select(List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::extractValueString(&__a0))?, (std::sync::Arc::new(move |__a0: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::isNotEmptyString(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>))?;
            (r#str.clone(), metamodelica::cons(r#str.clone(), strs))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(assoc)
}

fn buildDependencyGraph(
    mut name: ArcStr,
    mut sp: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut edges: metamodelica::List<ArcStr>;
    edges = (::match_deref::match_deref! { match sp {
        _ => {
            let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue(name, sp, &move |__a0: ArcStr, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isClassNamed(&__a0, &__a1)) })?) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elts = metamodelica::Own::own(__pa0);
            elts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (elts).into_iter().cloned() {
            if !(SCodeUtil::isImport(&(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            List::map(elts, &move |__a0: metamodelica::Ref<SCode::Element>| importDependency(&__a0))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(edges)
}

fn buildDependencyGraphPublicImports(
    mut name: ArcStr,
    mut sp: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut edges: metamodelica::List<ArcStr>;
    edges = (::match_deref::match_deref! { match sp {
        _ => {
            let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue(name, sp, &move |__a0: ArcStr, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isClassNamed(&__a0, &__a1)) })?) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elts = metamodelica::Own::own(__pa0);
            elts = List::select(elts, (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::elementIsPublicImport(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?;
            List::map(elts, &move |__a0: metamodelica::Ref<SCode::Element>| importDependency(&__a0))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(edges)
}

fn buildTransitiveDependencyGraph(
    mut name: ArcStr,
    mut oldgraph: &metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut edges: metamodelica::List<ArcStr>;
    edges = 'mc: {
        let __mc_input = &**oldgraph;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(List::setDifference(Graph::allReachableNodes(&((list![name.clone()], metamodelica::nil())), oldgraph, &fnptr!(stringEq, ArcStr, ArcStr))?, &(list![name.clone()]))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.buildTransitiveDependencyGraph failed: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(edges)
}

fn importDependency(mut simp: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (match &**simp {
        SCode::Element::IMPORT {
            imp: Absyn::Import::NAMED_IMPORT { path, .. },
            ..
        } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)),
        SCode::Element::IMPORT {
            imp: Absyn::Import::NAMED_IMPORT { path, .. },
            ..
        } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)),
        SCode::Element::IMPORT {
            imp: Absyn::Import::QUAL_IMPORT { path },
            ..
        } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)),
        SCode::Element::IMPORT {
            imp: Absyn::Import::UNQUAL_IMPORT { path },
            ..
        } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)),
        SCode::Element::IMPORT {
            imp: Absyn::Import::GROUP_IMPORT { prefix: path, .. },
            ..
        } => AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)),
        SCode::Element::IMPORT { imp, info, .. } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("CevalScript.importDependency could not handle:"));
                __mm_s.push_str(&*Dump::unparseImportStr(imp.clone())?);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str], info)?;
            return Err("fail");
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(name)
}

fn compareNumberOfDependencies(
    mut node1: &(ArcStr, metamodelica::List<ArcStr>),
    mut node2: &(ArcStr, metamodelica::List<ArcStr>),
) -> bool {
    let mut cmp: bool;
    let mut deps1: metamodelica::List<ArcStr>;
    let mut deps2: metamodelica::List<ArcStr>;
    (_, deps1) = node1.clone();
    (_, deps2) = node2.clone();
    cmp = ((deps1).len() as i32) >= ((deps2).len() as i32);
    cmp
}

fn compareDependencyNode(
    mut node1: &(ArcStr, metamodelica::List<ArcStr>),
    mut node2: &(ArcStr, metamodelica::List<ArcStr>),
) -> bool {
    let mut cmp: bool;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    (s1, _) = node1.clone();
    (s2, _) = node2.clone();
    cmp = Util::strcmpBool(&s1, &s2);
    cmp
}

fn dependencyString(mut deps: &(ArcStr, metamodelica::List<ArcStr>)) -> ArcStr {
    let mut r#str: ArcStr;
    let mut strs: metamodelica::List<ArcStr>;
    (r#str, strs) = deps.clone();
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((strs).len() as i32)));
        __mm_s.push_str(&*literal!("): "));
        __mm_s.push_str(&*stringDelimitList(strs, literal!(",")));
        ArcStr::from(__mm_s)
    };
    r#str
}

fn transitiveDependencyString(mut deps: &(ArcStr, metamodelica::List<ArcStr>)) -> ArcStr {
    let mut r#str: ArcStr;
    let mut strs: metamodelica::List<ArcStr>;
    (r#str, strs) = deps.clone();
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(((strs).len() as i32)));
        __mm_s.push_str(&*literal!(": ("));
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(") "));
        __mm_s.push_str(&*stringDelimitList(strs, literal!(",")));
        ArcStr::from(__mm_s)
    };
    r#str
}

fn containsPublicInterface(mut elt: &metamodelica::Ref<SCode::Element>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match elt {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            List::any(metamodelica::AsArg::as_arg(&elts), &move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(containsPublicInterface2(&__a0)) })?
        },
        _ => {
            let mut name: ArcStr;
            name = SCodeUtil::elementName(elt)?;
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.containsPublicInterface failed: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![name])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn containsPublicInterface2(mut elt: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match elt {
        Deref @ SCode::Element::IMPORT { .. } => false,
        Deref @ SCode::Element::EXTENDS { .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => false,
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn containsImport(mut elt: &metamodelica::Ref<SCode::Element>, mut visibility: SCode::Visibility) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match elt {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            List::exist1(metamodelica::AsArg::as_arg(&elts), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: SCode::Visibility| -> metamodelica::Result<_> { ::std::result::Result::Ok(containsImport2(&__a0, __a1)) }, visibility)?
        },
        _ => {
            let mut name: ArcStr;
            name = SCodeUtil::elementName(elt)?;
            name = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.containsPublicInterface failed: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![name])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn containsImport2(mut elt: &metamodelica::Ref<SCode::Element>, mut visibility: SCode::Visibility) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((&**elt, visibility)) {
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PUBLIC { .. }, .. }, SCode::Visibility::PUBLIC { .. }) => true,
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PROTECTED { .. }, .. }, SCode::Visibility::PROTECTED { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn printInterfaceString(mut elt: &metamodelica::Ref<SCode::Element>) -> Result<()> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*elt)) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*boolString(containsPublicInterface(elt)?));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn writeModuleDepends(
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut prefix: ArcStr,
    mut suffix: &ArcStr,
    mut deps: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = &**cl;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, info: SourceInfo { .. }, .. } => {
                    let mut allDepends: metamodelica::List<ArcStr>;
                    let mut protectedDepends: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr = r#str.clone();
                    protectedDepends = List::map(List::select(elts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::elementIsProtectedImport(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?, &move |__a0: metamodelica::Ref<SCode::Element>| importDependency(&__a0))?;
                    protectedDepends = List::select(protectedDepends.clone(), (std::sync::Arc::new(move |__a0: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(isNotBuiltinImport(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>))?;
                    let __pa0 = ::match_deref::match_deref! { match &(Graph::allReachableNodes(&((metamodelica::cons(name.clone(), protectedDepends.clone()), metamodelica::nil())), &deps, &fnptr!(stringEq, ArcStr, ArcStr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    allDepends = metamodelica::Own::own(__pa0);
                    allDepends = List::map1r(allDepends.clone(), &fnptr!(stringAppend, ArcStr, ArcStr), prefix.clone())?;
                    allDepends = List::map1(allDepends.clone(), &fnptr!(stringAppend, ArcStr, ArcStr), literal!(".interface.mo"))?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix); __mm_s.push_str(&*name); __mm_s.push_str(&*suffix); __mm_s.push_str(&*literal!(": $(RELPATH_")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*stringDelimitList(allDepends.clone(), literal!(" "))); ArcStr::from(__mm_s) };
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, info, .. } => {
                            let mut tmp1: ArcStr;
                            let mut allDepends: metamodelica::List<ArcStr>;
                            let mut protectedDepends: metamodelica::List<ArcStr>;
                            let mut tmp2: metamodelica::List<ArcStr>;
                            protectedDepends = List::map(List::select(elts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::elementIsProtectedImport(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?, &move |__a0: metamodelica::Ref<SCode::Element>| importDependency(&__a0))?;
                            protectedDepends = List::select(protectedDepends.clone(), (std::sync::Arc::new(move |__a0: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(isNotBuiltinImport(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>))?;
                            allDepends = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut e in (deps.clone()).into_iter().cloned() {
                            let __x = Util::tuple21(e.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            for mut d in &*protectedDepends {
                                if !(listMember(d.clone(), allDepends.clone())) {
                                    Error::addSourceMessage(&(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED_UNKNOWN_PACKAGE.clone()), list![name.clone(), name.clone(), d.clone()], metamodelica::AsArg::as_arg(&info))?;
                                    return Err("fail");
                                }
                            }
                            for mut dep in &*deps {
                                (tmp1, tmp2) = dep.clone();
                                for mut d in &*tmp2 {
                                    if !(listMember(d.clone(), allDepends.clone())) {
                                                Error::addSourceMessage(&(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED_UNKNOWN_PACKAGE.clone()), list![name.clone(), tmp1.clone(), d.clone()], metamodelica::AsArg::as_arg(&info))?;
                                                return Err("fail");
                                    }
                                }
                            }
                            Ok(return Err("fail"))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, info, .. } => {
                    Error::addSourceMessage(&(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#str)
}

fn isNotBuiltinImport(mut module: &ArcStr) -> bool {
    let mut b: bool = !metamodelica::stringEq(&module, &(literal!("MetaModelica")));
    b
}

fn getTypeNameIdent(mut val: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*val)) {
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: __pa0 } } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    Ok(r#str)
}

fn getChangedClass(mut elt: &metamodelica::Ref<SCode::Element>, mut suffix: &ArcStr) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (match &**elt {
        SCode::Element::CLASS {
            name: __esc_name,
            info: SourceInfo { .. },
            ..
        } if (!(System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*__esc_name);
            __mm_s.push_str(&*suffix);
            ArcStr::from(__mm_s)
        }))) =>
        {
            name = (*__esc_name).clone();
            name.clone()
        }
        SCode::Element::CLASS {
            name: __esc_name,
            info: SourceInfo { fileName, .. },
            ..
        } => {
            name = (*__esc_name).clone();
            let true = (System::fileIsNewerThan(fileName.clone(), {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*suffix);
                ArcStr::from(__mm_s)
            })?) else {
                return Err("pattern mismatch");
            };
            name.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(name)
}

fn isChanged(
    mut node: &(ArcStr, metamodelica::List<ArcStr>),
    mut hs: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<ArcStr>>),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<bool> {
    let mut b: bool;
    let mut r#str: ArcStr;
    let mut strs: metamodelica::List<ArcStr>;
    (r#str, strs) = node.clone();
    b = List::exist1(
        &(metamodelica::cons(r#str, strs)),
        &move |__a0: _, __a1: _| BaseHashSet::has(__a0, &__a1),
        hs,
    )?;
    Ok(b)
}

fn reloadClass(mut filename: ArcStr, mut encoding: ArcStr) -> Result<()> {
    let mut newp: Absyn::Program;
    newp = Parser::parse(
        filename,
        encoding,
        literal!(""),
        None,
        Config::acceptedGrammar()?,
        Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
        Flags::getConfigBool(Flags::STRICT.clone())?,
    )?;
    newp = ProgramUtil::updateProgram(newp, SymbolTable::getAbsyn(), false, false)?;
    SymbolTable::setAbsyn(newp)?;
    Ok(())
}

pub(crate) fn translateFunctions(
    mut program: &Absyn::Program,
    mut name: ArcStr,
    mut optMainFunction: Option<DAE::Function>,
    mut idaeElements: metamodelica::List<DAE::Function>,
    mut metarecordTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inIncludes: metamodelica::List<ArcStr>,
) -> Result<()> {
    {
        let __v = None;
        openmodelica_codegen_util::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v)
    };
    let () = (match optMainFunction {
        Some(mut daeMainFunction) => {
            let mut daeElements = idaeElements;
            let mut includes = inIncludes;
            let mut mainFunction: metamodelica::Ref<SimCodeFunction::Function::Function>;
            let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
            let mut libs: metamodelica::List<ArcStr>;
            let mut libPaths: metamodelica::List<ArcStr>;
            let mut includeDirs: metamodelica::List<ArcStr>;
            let mut makefileParams: SimCodeFunction::MakefileParams;
            let mut fnCode: SimCodeFunction::FunctionCode;
            let mut extraRecordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
            let mut literals: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut midCode: Tpl::Text;
            let mut midfuncs: metamodelica::List<MidCode::Function>;
            (daeElements, literals) =
                SimCodeFunctionUtil::findLiterals(metamodelica::cons(daeMainFunction, daeElements))?;
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(SimCodeFunctionUtil::elaborateFunctions(program, daeElements, metarecordTypes, &literals, includes)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 }, __pa2, __pa3, __pa4, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => return Err("pattern mismatch"),
            } };
            mainFunction = metamodelica::Own::own(__pa0);
            fns = metamodelica::Own::own(__pa1);
            extraRecordDecls = metamodelica::Own::own(__pa2);
            includes = metamodelica::Own::own(__pa3);
            includeDirs = metamodelica::Own::own(__pa4);
            libs = metamodelica::Own::own(__pa5);
            libPaths = metamodelica::Own::own(__pa6);
            SimCodeFunctionUtil::checkValidMainFunction(name.clone(), &mainFunction)?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs, libs, libPaths, true, false)?;
            fnCode = SimCodeFunction::FunctionCode {
                name: name.clone(),
                mainFunction: Some(mainFunction.clone()),
                functions: fns.clone(),
                literals: literals,
                externalFunctionIncludes: includes,
                makefileParams: makefileParams,
                extraRecordDecls: extraRecordDecls.clone(),
            };
            SimCodeFunctionUtil::setTrivialRecords(&extraRecordDecls)?;
            if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("wasm-jit"))) {
                CodegenWasmJitFunctions::translateFunctions(fnCode);
            } else if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("MidC"))) {
                Tpl::tplString(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: SimCodeFunction::FunctionCode| {
                        CodegenCFunctions::translateFunctionHeaderFiles(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static,
                        >),
                    fnCode,
                )?;
                midfuncs = DAEToMid::DAEFunctionsToMid(metamodelica::cons(mainFunction, fns))?;
                midCode = Tpl::tplCallWithFailError(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: MidCode::Program| {
                        CodegenMidToC::genProgram(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, MidCode::Program) -> Result<Tpl::Text> + 'static,
                        >),
                    MidCode::Program {
                        name: name.clone(),
                        functions: midfuncs,
                    },
                    Tpl::emptyTxt.clone(),
                )?;
                Tpl::textFileConvertLines(midCode, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(".c"));
                    ArcStr::from(__mm_s)
                })?;
            } else {
                Tpl::tplString(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: SimCodeFunction::FunctionCode| {
                        CodegenCFunctions::translateFunctions(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static,
                        >),
                    fnCode,
                )?;
            }
            ()
        }
        None => {
            let mut daeElements = idaeElements;
            let mut includes = inIncludes;
            let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
            let mut libs: metamodelica::List<ArcStr>;
            let mut libPaths: metamodelica::List<ArcStr>;
            let mut includeDirs: metamodelica::List<ArcStr>;
            let mut makefileParams: SimCodeFunction::MakefileParams;
            let mut fnCode: SimCodeFunction::FunctionCode;
            let mut extraRecordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
            let mut literals: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut midCode: Tpl::Text;
            let mut midfuncs: metamodelica::List<MidCode::Function>;
            (daeElements, literals) = SimCodeFunctionUtil::findLiterals(daeElements)?;
            (fns, extraRecordDecls, includes, includeDirs, libs, libPaths) =
                SimCodeFunctionUtil::elaborateFunctions(program, daeElements, metarecordTypes, &literals, includes)?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs, libs, libPaths, true, false)?;
            fns = removeThreadDataFunction(&fns, metamodelica::nil());
            extraRecordDecls = removeThreadDataRecord(&extraRecordDecls, metamodelica::nil());
            fnCode = SimCodeFunction::FunctionCode {
                name: name.clone(),
                mainFunction: None,
                functions: fns.clone(),
                literals: literals,
                externalFunctionIncludes: includes,
                makefileParams: makefileParams,
                extraRecordDecls: extraRecordDecls.clone(),
            };
            SimCodeFunctionUtil::setTrivialRecords(&extraRecordDecls)?;
            if metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("MidC"))) {
                Tpl::tplString(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: SimCodeFunction::FunctionCode| {
                        CodegenCFunctions::translateFunctionHeaderFiles(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static,
                        >),
                    fnCode,
                )?;
                midfuncs = DAEToMid::DAEFunctionsToMid(fns)?;
                midCode = Tpl::tplCallWithFailError(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: MidCode::Program| {
                        CodegenMidToC::genProgram(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, MidCode::Program) -> Result<Tpl::Text> + 'static,
                        >),
                    MidCode::Program {
                        name: name.clone(),
                        functions: midfuncs,
                    },
                    Tpl::emptyTxt.clone(),
                )?;
                Tpl::textFileConvertLines(midCode, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(".c"));
                    ArcStr::from(__mm_s)
                })?;
            } else {
                Tpl::tplString(
                    (std::sync::Arc::new(move |__a0: Tpl::Text, __a1: SimCodeFunction::FunctionCode| {
                        CodegenCFunctions::translateFunctions(__a0, &__a1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static,
                        >),
                    fnCode,
                )?;
            }
            ()
        }
    });
    Ok(())
}

fn removeThreadDataRecord<'__b>(
    mut inRecs: &'__b metamodelica::List<SimCodeFunction::RecordDeclaration>,
    mut inAcc: metamodelica::List<SimCodeFunction::RecordDeclaration>,
) -> metamodelica::List<SimCodeFunction::RecordDeclaration> {
    '__tco: loop {
        ::match_deref::match_deref! { match inRecs {
            Deref @ metamodelica::ListNode::Nil => {
                return inAcc.reverse()
            },
            Deref @ metamodelica::ListNode::Cons { head: SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: Deref @ "OpenModelica_threadData_ThreadData", .. }, tail: rest } => {
                let mut acc: metamodelica::List<SimCodeFunction::RecordDeclaration>;
                { (inRecs, inAcc) = (rest, inAcc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } }, .. }, tail: rest } => {
                let mut acc: metamodelica::List<SimCodeFunction::RecordDeclaration>;
                { (inRecs, inAcc) = (rest, inAcc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: r, tail: rest } => {
                let mut acc: metamodelica::List<SimCodeFunction::RecordDeclaration>;
                { (inRecs, inAcc) = (rest, metamodelica::cons(r.clone(), inAcc)); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn removeThreadDataFunction<'__b>(
    mut inFuncs: &'__b metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    mut inAcc: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
) -> metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inFuncs {
            Deref @ metamodelica::ListNode::Nil => {
                return inAcc.reverse()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } } }, .. }, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                { (inFuncs, inAcc) = (rest, inAcc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: f, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                { (inFuncs, inAcc) = (rest, metamodelica::cons(f.clone(), inAcc)); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn unZipEncryptedPackageAndCheckFile(
    mut inWorkdir: ArcStr,
    mut filename: ArcStr,
    mut skipUnzip: bool,
) -> Result<(bool, ArcStr)> {
    let mut success: bool;
    let mut outFilename: ArcStr;
    let mut workdir: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    let mut filename_1: ArcStr;
    let mut filename1: ArcStr;
    let mut filename2: ArcStr;
    let mut filename3: ArcStr;
    let mut filename4: ArcStr;
    let mut r#str: ArcStr;
    let mut str1: ArcStr;
    let mut str2: ArcStr;
    let mut str3: ArcStr;
    let mut str4: ArcStr;
    let mut cmd: ArcStr;
    let mut cmdPrefix: ArcStr;
    let mut isWindows: bool = metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")));
    success = false;
    outFilename = literal!("");
    if System::regularFileExists(filename.clone()) {
        if StringUtil::endsWith(filename.clone(), literal!(".mol")) {
            workdir = if (System::directoryExists(inWorkdir.clone())) {
                inWorkdir
            } else {
                System::pwd()
            };
            cmdPrefix = if (isWindows) {
                literal!("ripunzip.exe -q unzip-file -d ")
            } else {
                literal!("unzip -q -o -d ")
            };
            cmd = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*cmdPrefix);
                __mm_s.push_str(&*literal!("\""));
                __mm_s.push_str(&*workdir);
                __mm_s.push_str(&*literal!("\" \""));
                __mm_s.push_str(&*filename);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            };
            if skipUnzip || 0 == System::systemCall(cmd, literal!("")) {
                s1 = System::basename(filename);
                s2 = Util::removeLast4Char(s1)?;
                s3 = (Util::stringSplitAtChar(s2.clone(), literal!(" "))?).get(1)?;
                filename1 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!("/package.moc"));
                    ArcStr::from(__mm_s)
                };
                filename2 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!(".moc"));
                    ArcStr::from(__mm_s)
                };
                filename3 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!("/package.moc"));
                    ArcStr::from(__mm_s)
                };
                filename4 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!(".moc"));
                    ArcStr::from(__mm_s)
                };
                if System::regularFileExists(filename1.clone()) {
                    filename_1 = filename1.clone();
                } else if System::regularFileExists(filename2.clone()) {
                    filename_1 = filename2.clone();
                } else if System::regularFileExists(filename3.clone()) {
                    filename_1 = filename3.clone();
                } else {
                    filename_1 = filename4.clone();
                }
                str1 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!("/package.mo"));
                    ArcStr::from(__mm_s)
                };
                str2 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                };
                str3 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!("/package.mo"));
                    ArcStr::from(__mm_s)
                };
                str4 = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*workdir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                };
                if System::regularFileExists(str1.clone()) {
                    r#str = str1.clone();
                } else if System::regularFileExists(str2.clone()) {
                    r#str = str2.clone();
                } else if System::regularFileExists(str3.clone()) {
                    r#str = str3.clone();
                } else {
                    r#str = str4.clone();
                }
                filename_1 = if (System::regularFileExists(filename_1.clone())) {
                    filename_1
                } else {
                    r#str
                };
                if System::regularFileExists(filename_1.clone()) {
                    success = true;
                    outFilename = filename_1;
                } else {
                    Error::addMessage(
                        Error::PACKAGE_FILE_NOT_FOUND_ERROR.clone(),
                        list![filename1, filename2, filename3, filename4, str1, str2, str3, str4],
                    )?;
                }
            } else {
                Error::addMessage(Error::UNABLE_TO_UNZIP_FILE.clone(), list![filename])?;
            }
        } else {
            Error::addMessage(Error::EXPECTED_ENCRYPTED_PACKAGE.clone(), list![filename])?;
        }
    } else {
        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![filename])?;
    }
    Ok((success, outFilename))
}

fn listClass(mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut r#str: ArcStr;
    let mut name: ArcStr;
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut absynClass: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut cl: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut className: metamodelica::Ref<Absyn::Path>;
    let mut interface_only: bool;
    let mut short_only: bool;
    let dumpOpt: SCodeDump::SCodeDumpOptions = SCodeDump::SCodeDumpOptions {
        stripAlgorithmSections: true,
        stripProtectedImports: false,
        stripProtectedClasses: true,
        stripProtectedComponents: true,
        stripMetaRecords: true,
        stripGraphicalAnnotations: true,
        stripStringComments: true,
        stripExternalDecl: true,
        stripOutputBindings: true,
    };
    r#str = 'mc: {
        let __mc_input = &**args;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p: Absyn::Program = p.clone();
                    p = SymbolTable::getAbsyn();
                    let true = (Interactive::astContainsEncryptedClass(p.clone())?) else { return Err("pattern mismatch") };
                    Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                    Ok((literal!(""), p.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            p = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AllLoadedClasses" } } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } => {
                            Ok((::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))) {
                Deref @ "Absyn" => Dump::unparseStr(SymbolTable::getAbsyn(), false, Dump::defaultDumpOptions.clone())?,
                Deref @ "SCode" => SCodeDump::programStr(SymbolTable::getSCode()?, SCodeDump::defaultOptions.clone())?,
                Deref @ "MetaModelicaInterface" => SCodeDump::programStr(SymbolTable::getSCode()?, dumpOpt)?,
                Deref @ "Internal" => System::anyStringCode(SymbolTable::getAbsyn()),
                _ => literal!(""),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: interface_only }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: short_only }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } } => {
                            let mut absynClass: metamodelica::Ref<Absyn::Class> = absynClass.clone();
                            let mut cl: metamodelica::Ref<SCode::Element> = cl.clone();
                            let mut p: Absyn::Program = p.clone();
                            let mut scodeP: metamodelica::List<metamodelica::Ref<SCode::Element>> = scodeP.clone();
                            let false = (metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("AllLoadedClasses") }) == className.clone()) else { return Err("pattern mismatch") };
                            p = SymbolTable::getAbsyn();
                            scodeP = SymbolTable::getSCode()?;
                            absynClass = ProgramUtil::getPathedClassInProgram(className.clone(), &p, false, false)?;
                            absynClass = if (interface_only.clone()) {AbsynUtil::getFunctionInterface(absynClass.clone())?} else {absynClass.clone()};
                            absynClass = if (short_only.clone()) {AbsynUtil::getShortClass(absynClass.clone())?} else {absynClass.clone()};
                            p = Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP };
                            cl = FBuiltin::getElementWithPathCheckBuiltin(scodeP.clone(), metamodelica::AsArg::as_arg(&className))?;
                            Ok(((::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))) {
                Deref @ "Absyn" => Dump::unparseStr(p.clone(), false, Dump::defaultDumpOptions.clone())?,
                Deref @ "SCode" => SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?,
                Deref @ "MetaModelicaInterface" => SCodeDump::unparseElementStr(cl.clone(), dumpOpt)?,
                Deref @ "Internal" => System::anyStringCode(p.clone()),
                _ => literal!(""),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }), absynClass.clone(), cl.clone(), p.clone(), scodeP.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            absynClass = __wb0;
            cl = __wb1;
            p = __wb2;
            scodeP = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res = metamodelica::Ref::new(Values::Value::STRING { string: r#str });
    res
}

fn listFile(mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut className: metamodelica::Ref<Absyn::Path>;
    let mut nested: bool;
    let mut access: Access = Access::hide;
    let mut absynClass: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    r#str = 'mc: {
        let __mc_input = &**args;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: nested }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                            let mut absynClass: metamodelica::Ref<Absyn::Class> = absynClass.clone();
                            let mut access: Access = access.clone();
                            let mut path: metamodelica::Ref<Absyn::Path> = path.clone();
                            let mut restriction: Absyn::Restriction = restriction.clone();
                            let mut r#str: ArcStr = r#str.clone();
                            path = (match &*className.clone() {
                Absyn::Path::FULLYQUALIFIED { path: __className_path } => __className_path.clone(),
                _ => className.clone(),
            });
                            access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                            let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(className.clone(), &(SymbolTable::getAbsyn()), false, false)?) {
                                __pa2 @ Deref @ Absyn::Class { restriction: __pa0, info: SourceInfo { fileName: __pa1, .. }, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            restriction = metamodelica::Own::own(__pa0);
                            r#str = metamodelica::Own::own(__pa1);
                            absynClass = metamodelica::Own::own(__pa2);
                            absynClass = if (nested.clone()) {absynClass.clone()} else {AbsynUtil::filterNestedClasses(absynClass.clone())?};
                            if access >= Access::packageText.clone() || access >= Access::nonPackageText.clone() && !(AbsynUtil::isPackageRestriction(&restriction)) {
                                r#str = Dump::unparseStr(Absyn::Program { classes: list![absynClass.clone()], within_: (match &*path {
                Absyn::Path::IDENT { .. } => openmodelica_ast::Absyn::Within::TOP,
                _ => Absyn::Within::WITHIN { path: AbsynUtil::stripLast(&path)? },
            }) }, false, Dump::DumpOptions { fileName: r#str.clone() })?;
                            } else {
                                Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                                r#str = literal!("");
                            }
                            Ok((r#str.clone(), absynClass.clone(), access.clone(), path.clone(), restriction.clone(), r#str.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            absynClass = __wb0;
            access = __wb1;
            path = __wb2;
            restriction = __wb3;
            r#str = __wb4;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res = metamodelica::Ref::new(Values::Value::STRING { string: r#str });
    res
}

fn getClassNames(
    mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut recursive: bool;
    let mut qualified: bool;
    let mut sort: bool;
    let mut builtin: bool;
    let mut protects: bool;
    let mut constants: bool;
    let mut p: Absyn::Program;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa5 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa6 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    recursive = metamodelica::Own::own(__pa1);
    qualified = metamodelica::Own::own(__pa2);
    sort = metamodelica::Own::own(__pa3);
    builtin = metamodelica::Own::own(__pa4);
    protects = metamodelica::Own::own(__pa5);
    constants = metamodelica::Own::own(__pa6);
    p = SymbolTable::getAbsyn();
    if builtin {
        p = ProgramUtil::updateProgram(p, (FBuiltin::getInitialFunctions()?).0, false, false)?;
    }
    if AbsynUtil::pathEqual(
        &path,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("AllLoadedClasses"),
        })),
    ) {
        if recursive {
            (_, paths) = ProgramUtil::getClassNamesRecursive(None, p, protects, constants, metamodelica::nil())?;
            paths = metamodelica::Dangerous::listReverseInPlace(paths);
        } else {
            paths = Interactive::getTopClassnames(&p)?;
        }
    } else {
        if recursive {
            (_, paths) = ProgramUtil::getClassNamesRecursive(Some(path), p, protects, constants, metamodelica::nil())?;
            paths = metamodelica::Dangerous::listReverseInPlace(paths);
        } else {
            paths = Interactive::getClassnamesInPath(path.clone(), p, protects, constants);
            if qualified {
                paths = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                    for mut p in (paths).into_iter().cloned() {
                        let __x = AbsynUtil::joinPaths(path.clone(), p.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
            }
        }
    }
    if sort {
        paths = List::sort(
            paths,
            (std::sync::Arc::new(AbsynUtil::pathGe)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool>
                        + 'static,
                >),
        )?;
    }
    res = ValuesMake::makeCodeTypeNameArray(paths);
    Ok(res)
}

fn checkSettings() -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut vars: metamodelica::List<ArcStr>;
    let mut omhome: ArcStr;
    let mut omlib: ArcStr;
    let mut omcpath: ArcStr;
    let mut systemPath: ArcStr;
    let mut omdev: ArcStr;
    let mut os: ArcStr;
    let mut touch_file: ArcStr;
    let mut usercflags: ArcStr;
    let mut workdir: ArcStr;
    let mut uname: ArcStr;
    let mut senddata: ArcStr;
    let mut gcc: ArcStr;
    let mut gccVersion: ArcStr;
    let mut confcmd: ArcStr;
    let mut omcfound: bool;
    let mut touch_res: bool;
    let mut rm_res: bool;
    let mut gcc_res: bool;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    vars = list![
        literal!("OPENMODELICAHOME"),
        literal!("OPENMODELICALIBRARY"),
        literal!("OMC_PATH"),
        literal!("SYSTEM_PATH"),
        literal!("OMDEV_PATH"),
        literal!("OMC_FOUND"),
        literal!("MODELICAUSERCFLAGS"),
        literal!("WORKING_DIRECTORY"),
        literal!("CREATE_FILE_WORKS"),
        literal!("REMOVE_FILE_WORKS"),
        literal!("OS"),
        literal!("SYSTEM_INFO"),
        literal!("RTLIBS"),
        literal!("C_COMPILER"),
        literal!("C_COMPILER_VERSION"),
        literal!("C_COMPILER_RESPONDING"),
        literal!("CONFIGURE_CMDLINE")
    ];
    omhome = Settings::getInstallationDirectoryPath()?;
    omlib = Settings::getModelicaPath(Testsuite::isRunning()?)?;
    omcpath = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*omhome);
        __mm_s.push_str(&*literal!("/bin/omc"));
        __mm_s.push_str(&*arcstr::literal!(Autoconf::exeExt));
        ArcStr::from(__mm_s)
    };
    systemPath = Util::makeValueOrDefault(&System::readEnv, literal!("PATH"), literal!(""));
    omdev = Util::makeValueOrDefault(&System::readEnv, literal!("OMDEV"), literal!(""));
    omcfound = System::regularFileExists(omcpath.clone());
    os = arcstr::literal!(Autoconf::os);
    touch_file = literal!("omc.checksettings.create_file_test");
    usercflags = Util::makeValueOrDefault(&System::readEnv, literal!("MODELICAUSERCFLAGS"), literal!(""));
    workdir = System::pwd();
    touch_res = 0
        == System::systemCall(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("touch "));
                __mm_s.push_str(&*touch_file);
                ArcStr::from(__mm_s)
            },
            literal!(""),
        );
    System::systemCall(literal!("uname -a"), touch_file.clone());
    uname = System::readFile(touch_file.clone())?;
    rm_res = 0
        == System::systemCall(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("rm "));
                __mm_s.push_str(&*touch_file);
                ArcStr::from(__mm_s)
            },
            literal!(""),
        );
    senddata = arcstr::literal!(Autoconf::ldflags_runtime);
    gcc = System::getCCompiler();
    System::systemCall(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("rm -f "));
            __mm_s.push_str(&*touch_file);
            ArcStr::from(__mm_s)
        },
        literal!(""),
    );
    gcc_res = 0
        == System::systemCall(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*gcc);
                __mm_s.push_str(&*literal!(" --version"));
                ArcStr::from(__mm_s)
            },
            touch_file.clone(),
        );
    gccVersion = System::readFile(touch_file.clone())?;
    System::systemCall(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("rm -f "));
            __mm_s.push_str(&*touch_file);
            ArcStr::from(__mm_s)
        },
        literal!(""),
    );
    confcmd = arcstr::literal!(Autoconf::configureCommandLine);
    vals = list![
        metamodelica::Ref::new(Values::Value::STRING { string: omhome }),
        metamodelica::Ref::new(Values::Value::STRING { string: omlib }),
        metamodelica::Ref::new(Values::Value::STRING { string: omcpath }),
        metamodelica::Ref::new(Values::Value::STRING { string: systemPath }),
        metamodelica::Ref::new(Values::Value::STRING { string: omdev }),
        metamodelica::Ref::new(Values::Value::BOOL { boolean: omcfound }),
        metamodelica::Ref::new(Values::Value::STRING { string: usercflags }),
        metamodelica::Ref::new(Values::Value::STRING { string: workdir }),
        metamodelica::Ref::new(Values::Value::BOOL { boolean: touch_res }),
        metamodelica::Ref::new(Values::Value::BOOL { boolean: rm_res }),
        metamodelica::Ref::new(Values::Value::STRING { string: os }),
        metamodelica::Ref::new(Values::Value::STRING { string: uname }),
        metamodelica::Ref::new(Values::Value::STRING { string: senddata }),
        metamodelica::Ref::new(Values::Value::STRING { string: gcc }),
        metamodelica::Ref::new(Values::Value::STRING { string: gccVersion }),
        metamodelica::Ref::new(Values::Value::BOOL { boolean: gcc_res }),
        metamodelica::Ref::new(Values::Value::STRING { string: confcmd })
    ];
    res = metamodelica::Ref::new(Values::Value::RECORD {
        record_: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("OpenModelica.Scripting.CheckSettingsResult"),
        }),
        orderd: vals,
        comp: vars,
        index: -1,
    });
    Ok(res)
}

fn generateSeparateCodeDependenciesMakefile(
    mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut names: metamodelica::List<ArcStr>;
    let mut strs: metamodelica::List<ArcStr>;
    let mut deps: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut filename: ArcStr;
    let mut prefix: ArcStr;
    let mut suffix: ArcStr;
    match '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*args)) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa3 }, tail: Deref @ metamodelica::ListNode::Nil } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        filename = metamodelica::Own::own(__pa1);
        prefix = metamodelica::Own::own(__pa2);
        suffix = metamodelica::Own::own(__pa3);
        sp = unwrap_break_err!(SymbolTable::getSCode(), '__try0);
        names = List::filterMap(&sp, &move |__a0: metamodelica::Ref<SCode::Element>| {
            SCodeUtil::getElementName(&__a0)
        });
        deps = unwrap_break_err!(Graph::buildGraph(names.clone(), &move |__a0: ArcStr, __a1: metamodelica::List<metamodelica::Ref<SCode::Element>>| buildDependencyGraphPublicImports(__a0, &__a1), sp.clone()), '__try0);
        strs = unwrap_break_err!(List::map3(sp.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: ArcStr, __a2: ArcStr, __a3: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>| writeModuleDepends(&__a0, __a1, &__a2, __a3), prefix.clone(), suffix.clone(), deps.clone()), '__try0);
        unwrap_break_err!(System::writeFile(filename.clone(), stringDelimitList(strs.clone(), literal!("\n"))), '__try0);
        res = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn generateSeparateCodeDependencies(
    mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut suffix: ArcStr;
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut names: metamodelica::List<ArcStr>;
    let mut namesPublic: metamodelica::List<ArcStr>;
    let mut namesChanged: metamodelica::List<ArcStr>;
    let mut fileNames: metamodelica::List<ArcStr>;
    let mut deps: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut depstransitive: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut depstransposed: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut depstransposedtransitive: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut depsmerged: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut depschanged: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>;
    let mut hashSetString: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<ArcStr>>),
        i32,
        i32,
        (
            HashSetString::FuncHashCref,
            HashSetString::FuncCrefEqual,
            HashSetString::FuncCrefStr,
        ),
    );
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*args)) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        suffix = metamodelica::Own::own(__pa1);
        sp = unwrap_break_err!(SymbolTable::getSCode(), '__try0);
        names = List::filterMap(&sp, &move |__a0: metamodelica::Ref<SCode::Element>| {
            SCodeUtil::getElementName(&__a0)
        });
        deps = unwrap_break_err!(Graph::buildGraph(names.clone(), &move |__a0: ArcStr, __a1: metamodelica::List<metamodelica::Ref<SCode::Element>>| buildDependencyGraph(__a0, &__a1), sp.clone()), '__try0);
        namesPublic = unwrap_break_err!(List::map(unwrap_break_err!(List::select(sp.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| containsPublicInterface(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>)), '__try0), &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::getElementName(&__a0)), '__try0);
        namesChanged = List::filterMap1(
            &sp,
            &move |__a0: metamodelica::Ref<SCode::Element>, __a1: ArcStr| getChangedClass(&__a0, &__a1),
            suffix.clone(),
        );
        hashSetString = HashSetString::emptyHashSet();
        hashSetString = unwrap_break_err!(List::fold(&namesChanged, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), hashSetString.clone()), '__try0);
        depstransposed = unwrap_break_err!(Graph::transposeGraph(&(unwrap_break_err!(Graph::emptyGraph(names.clone()), '__try0)), &deps, (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0);
        depstransposedtransitive = unwrap_break_err!(Graph::buildGraph(namesPublic.clone(), &move |__a0: ArcStr, __a1: metamodelica::List<(ArcStr, metamodelica::List<ArcStr>)>| buildTransitiveDependencyGraph(__a0, &__a1), depstransposed.clone()), '__try0);
        depstransitive = unwrap_break_err!(Graph::transposeGraph(&(unwrap_break_err!(Graph::emptyGraph(names.clone()), '__try0)), &depstransposedtransitive, (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0);
        depstransitive = unwrap_break_err!(List::sort(depstransitive.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::List<ArcStr>), __a1: (ArcStr, metamodelica::List<ArcStr>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(compareNumberOfDependencies(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::List<ArcStr>), (ArcStr, metamodelica::List<ArcStr>)) -> Result<bool> + 'static>)), '__try0);
        depsmerged = unwrap_break_err!(Graph::merge(deps.clone(), depstransitive.clone(), &fnptr!(stringEq, ArcStr, ArcStr), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::List<ArcStr>), __a1: (ArcStr, metamodelica::List<ArcStr>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(compareDependencyNode(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::List<ArcStr>), (ArcStr, metamodelica::List<ArcStr>)) -> Result<bool> + 'static>)), '__try0);
        depschanged = unwrap_break_err!(List::select1(depsmerged.clone(), (std::sync::Arc::new(move |__a0: (ArcStr, metamodelica::List<ArcStr>), __a1: (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))| isChanged(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, metamodelica::List<ArcStr>), (metamodelica::Array<metamodelica::List<(ArcStr, i32)>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))) -> Result<bool> + 'static>), hashSetString.clone()), '__try0);
        names = unwrap_break_err!(List::map(depschanged.clone(), &fnptr!(Util::tuple21, _)), '__try0);
        fileNames = unwrap_break_err!(List::map1(names.clone(), &fnptr!(stringAppend, ArcStr, ArcStr), suffix.clone()), '__try0);
        for mut f in &*fileNames {
            System::removeFile(f.clone());
        }
        res = ValuesMake::makeArray(
            unwrap_break_err!(List::map(names.clone(), &fnptr!(ValuesMake::makeString, ArcStr)), '__try0),
        );
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = openmodelica_frontend_types::Values::Value::interned_META_FAIL();
        }
    }
    res
}

fn generateSeparateCode(
    mut args: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
) -> (metamodelica::Ref<Values::Value>, FCore::Cache) {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut outCache: FCore::Cache = cache.clone();
    let mut v: metamodelica::Ref<Values::Value>;
    let mut b: bool;
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cl: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    res = 'mc: {
        let __mc_input = &**args;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut cl: metamodelica::Ref<SCode::Element> = cl.clone();
                    let mut name: ArcStr = name.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut p: Absyn::Program = p.clone();
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
                    p = SymbolTable::getAbsyn();
                    sp = SymbolTable::getSCode()?;
                    name = getTypeNameIdent(metamodelica::AsArg::as_arg(&v))?;
                    { let __v = Some(true); openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    cl = List::getMemberOnTrue(name.clone(), &sp, &move |__a0: ArcStr, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isClassNamed(&__a0, &__a1)) })?;
                    (outCache, _) = generateFunctions(cache.clone(), env.clone(), &p, &sp, list![cl.clone()], b.clone())?;
                    { let __v = None; openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    Ok((metamodelica::Ref::new(Values::Value::BOOL { boolean: true }), cl.clone(), name.clone(), outCache.clone(), p.clone(), sp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cl = __wb0;
            name = __wb1;
            outCache = __wb2;
            p = __wb3;
            sp = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: v, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut name: ArcStr = name.clone();
                    let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>> = sp.clone();
                    sp = SymbolTable::getSCode()?;
                    name = getTypeNameIdent(metamodelica::AsArg::as_arg(&v))?;
                    let false = (List::isMemberOnTrue(name.clone(), &sp, &move |__a0: ArcStr, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::isClassNamed(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![name.clone(), literal!("<TOP>")])?;
                    Ok((return Err("fail"), name.clone(), sp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            name = __wb0;
            sp = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    { let __v = None; openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (res, outCache)
}

pub(crate) fn getImportedNames(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> (
    metamodelica::List<metamodelica::Ref<Values::Value>>,
    metamodelica::List<metamodelica::Ref<Values::Value>>,
) {
    let mut outPublicImports: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut outProtectedImports: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut ident: ArcStr;
    let mut pub_imports_list: metamodelica::List<Absyn::Import>;
    let mut pro_imports_list: metamodelica::List<Absyn::Import>;
    (pub_imports_list, pro_imports_list) = getImportList(inClass, metamodelica::nil(), metamodelica::nil());
    outPublicImports = metamodelica::nil();
    for mut imp in &*pub_imports_list {
        ident = AbsynUtil::pathFirstIdent(&(AbsynUtil::importPath(metamodelica::AsArg::as_arg(&imp))));
        if !metamodelica::stringEq(&ident, &(literal!("MetaModelica"))) {
            outPublicImports = metamodelica::cons(
                metamodelica::Ref::new(Values::Value::STRING { string: ident }),
                outPublicImports,
            );
        }
    }
    outProtectedImports = metamodelica::nil();
    for mut imp in &*pro_imports_list {
        ident = AbsynUtil::pathFirstIdent(&(AbsynUtil::importPath(metamodelica::AsArg::as_arg(&imp))));
        if !metamodelica::stringEq(&ident, &(literal!("MetaModelica"))) {
            outProtectedImports = metamodelica::cons(
                metamodelica::Ref::new(Values::Value::STRING { string: ident }),
                outProtectedImports,
            );
        }
    }
    (outPublicImports, outProtectedImports)
}

pub(crate) fn getImportList(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut pub_imports_list: metamodelica::List<Absyn::Import>,
    mut pro_imports_list: metamodelica::List<Absyn::Import>,
) -> (metamodelica::List<Absyn::Import>, metamodelica::List<Absyn::Import>) {
    let mut pub_imports_list: metamodelica::List<Absyn::Import> = pub_imports_list;
    let mut pro_imports_list: metamodelica::List<Absyn::Import> = pro_imports_list;
    for mut part in &*AbsynUtil::getClassPartsInClass(inClass) {
        (pub_imports_list, pro_imports_list) =
            getImportsInClassPart(metamodelica::AsArg::as_arg(&part), pub_imports_list, pro_imports_list);
    }
    pub_imports_list = metamodelica::Dangerous::listReverseInPlace(pub_imports_list);
    pro_imports_list = metamodelica::Dangerous::listReverseInPlace(pro_imports_list);
    (pub_imports_list, pro_imports_list)
}

fn getImportsInClassPart(
    mut part: &metamodelica::Ref<Absyn::ClassPart>,
    mut pub_imports_list: metamodelica::List<Absyn::Import>,
    mut pro_imports_list: metamodelica::List<Absyn::Import>,
) -> (metamodelica::List<Absyn::Import>, metamodelica::List<Absyn::Import>) {
    let mut pub_imports_list: metamodelica::List<Absyn::Import> = pub_imports_list;
    let mut pro_imports_list: metamodelica::List<Absyn::Import> = pro_imports_list;
    let () = (match &**part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            for mut elem in &*__part_contents.clone() {
                pub_imports_list = getImportsInElementItem(metamodelica::AsArg::as_arg(&elem), pub_imports_list);
            }
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            for mut elem in &*__part_contents.clone() {
                pro_imports_list = getImportsInElementItem(metamodelica::AsArg::as_arg(&elem), pro_imports_list);
            }
            ()
        }
        _ => (),
    });
    (pub_imports_list, pro_imports_list)
}

fn getImportsInElementItem(
    mut item: &metamodelica::Ref<Absyn::ElementItem>,
    mut imports_list: metamodelica::List<Absyn::Import>,
) -> metamodelica::List<Absyn::Import> {
    let mut imports_list: metamodelica::List<Absyn::Import> = imports_list;
    let () = (::match_deref::match_deref! { match item {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::IMPORT { import_, .. }, .. } } => {
            imports_list = metamodelica::cons(import_.clone(), imports_list);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    imports_list
}

fn getMMfileTotalDependencies(
    mut in_package_name: ArcStr,
    mut public_imports_dir: &ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut total_pub_imports: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut package_class: metamodelica::Ref<Absyn::Class>;
    let mut pub_imports_list: metamodelica::List<Absyn::Import>;
    let mut pro_imports_list: metamodelica::List<Absyn::Import>;
    let mut imp_ident: ArcStr;
    package_class = ProgramUtil::getPathedClassInProgram(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: in_package_name }),
        &(SymbolTable::getAbsyn()),
        false,
        false,
    )?;
    (pub_imports_list, pro_imports_list) = getImportList(&package_class, metamodelica::nil(), metamodelica::nil());
    for mut imp in &*pub_imports_list {
        imp_ident = AbsynUtil::pathFirstIdent(&(AbsynUtil::importPath(metamodelica::AsArg::as_arg(&imp))));
        if !metamodelica::stringEq(&imp_ident, &(literal!("MetaModelica"))) {
            total_pub_imports = getMMfilePublicDependencies(imp_ident, public_imports_dir, total_pub_imports)?;
        }
    }
    for mut imp in &*pro_imports_list {
        imp_ident = AbsynUtil::pathFirstIdent(&(AbsynUtil::importPath(metamodelica::AsArg::as_arg(&imp))));
        if !metamodelica::stringEq(&imp_ident, &(literal!("MetaModelica"))) {
            total_pub_imports = getMMfilePublicDependencies(imp_ident, public_imports_dir, total_pub_imports)?;
        }
    }
    Ok(total_pub_imports)
}

fn getMMfilePublicDependencies(
    mut in_package_name: ArcStr,
    mut public_imports_dir: &ArcStr,
    mut packages: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut packages: metamodelica::List<ArcStr> = packages;
    let mut dep_public_imports_file: ArcStr;
    let mut pub_imports_total: ArcStr;
    if listMember(in_package_name.clone(), packages.clone()) {
        return Ok(packages);
    }
    packages = metamodelica::cons(in_package_name.clone(), packages);
    dep_public_imports_file = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*public_imports_dir);
        __mm_s.push_str(&*literal!("/"));
        __mm_s.push_str(&*in_package_name);
        __mm_s.push_str(&*literal!(".public.imports"));
        ArcStr::from(__mm_s)
    };
    if !(System::regularFileExists(dep_public_imports_file.clone())) {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("getMMfileTotalDependencies: missing dependency file "));
                __mm_s.push_str(&*dep_public_imports_file);
                __mm_s.push_str(&*literal!(" — the module "));
                __mm_s.push_str(&*in_package_name);
                __mm_s.push_str(&*literal!(" is imported (transitively) but is not part of the build. "));
                __mm_s.push_str(&*literal!("Add its source file to the build configuration."));
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("Script/CevalScript.mo"),
        )?;
        return Err("fail");
    }
    pub_imports_total = System::readFile(dep_public_imports_file)?;
    for mut pub_imp in &*System::strtok(pub_imports_total, literal!(";")) {
        packages = getMMfilePublicDependencies(pub_imp.clone(), public_imports_dir, packages)?;
    }
    Ok(packages)
}
