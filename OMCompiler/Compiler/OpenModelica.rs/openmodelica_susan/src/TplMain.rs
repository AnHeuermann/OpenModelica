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

use crate::TplAbsyn;
use crate::TplCodegen;
use crate::TplParser;
use openmodelica_tpl::Tpl;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::System;

pub(crate) static emptyTxt: std::sync::LazyLock<Tpl::Text> = std::sync::LazyLock::new(|| Tpl::emptyTxt.clone());

pub(crate) static dsi: std::sync::LazyLock<SourceInfo> = std::sync::LazyLock::new(|| TplAbsyn::dummySourceInfo.clone());

pub fn main(mut inFile: ArcStr, mut inOutputDir: &ArcStr, mut inInterfaceDir: ArcStr) -> Result<()> {
    if !metamodelica::stringEq(&inInterfaceDir, &(literal!(""))) {
        FlagsUtil::setConfigString(Flags::TPL_INTERFACE_DIR.clone(), inInterfaceDir)?;
    }
    let () = (::match_deref::match_deref! { match &(inFile) {
        Deref @ "SusanTest.tpl" => {
            tplMainTest(literal!("a"))?;
            ()
        },
        file => {
            let mut strErrBuf: ArcStr;
            Print::clearBuf();
            translateFile(file.clone(), inOutputDir)?;
            strErrBuf = Print::getErrorString()?;
            strErrBuf = if (metamodelica::stringEq(&strErrBuf, &(literal!("")))) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Error Buffer ###\n")); __mm_s.push_str(&*strErrBuf); __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n")); ArcStr::from(__mm_s) }};
            metamodelica::print(strErrBuf);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn transformFile(
    mut inFile: ArcStr,
    mut inInterfaceDir: ArcStr,
) -> Result<(TplAbsyn::TemplPackage, TplAbsyn::MMPackage)> {
    let mut outTplPackage: TplAbsyn::TemplPackage;
    let mut outMMPackage: TplAbsyn::MMPackage;
    let mut nErrors: i32;
    if !metamodelica::stringEq(&inInterfaceDir, &(literal!(""))) {
        FlagsUtil::setConfigString(Flags::TPL_INTERFACE_DIR.clone(), inInterfaceDir)?;
    }
    nErrors = Error::getNumErrorMessages();
    outTplPackage = TplParser::templPackageFromFile(inFile)?;
    outMMPackage = TplAbsyn::transformAST(&outTplPackage)?;
    outTplPackage = TplAbsyn::fullyQualifyTemplatePackage(&outTplPackage)?;
    let true = (nErrors == Error::getNumErrorMessages()) else {
        return Err("pattern mismatch");
    };
    Ok((outTplPackage, outMMPackage))
}

pub(crate) fn translateFile(mut inFile: ArcStr, mut inOutputDir: &ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inFile;
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut destFile: ArcStr;
            let mut res: ArcStr;
            let mut txt: Tpl::Text;
            let mut tplPackage: TplAbsyn::TemplPackage;
            let mut mmPckg: TplAbsyn::MMPackage;
            let mut nErrors: i32;
            let mut wasError: bool;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nProcessing file '"));
                __mm_s.push_str(&*file);
                __mm_s.push_str(&*literal!("'\n"));
                ArcStr::from(__mm_s)
            });
            nErrors = Error::getNumErrorMessages();
            destFile = System::stringReplace(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("*"));
                    ArcStr::from(__mm_s)
                },
                literal!(".tpl*"),
                literal!(".mo"),
            )?;
            let false = (stringEq(&file, &destFile)) else {
                return Err("pattern mismatch");
            };
            if !metamodelica::stringEq(&inOutputDir, &(literal!(""))) {
                destFile = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*inOutputDir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*System::basename(destFile.clone()));
                    ArcStr::from(__mm_s)
                };
            }
            tplPackage = TplParser::templPackageFromFile(file.clone())?;
            mmPckg = TplAbsyn::transformAST(&tplPackage)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::mmPackage(txt.clone(), &mmPckg)?;
            res = Tpl::textString(txt.clone())?;
            wasError = nErrors < Error::getNumErrorMessages();
            destFile = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*destFile);
                __mm_s.push_str(&*if (wasError) { literal!(".err.mo") } else { literal!("") });
                ArcStr::from(__mm_s)
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nWriting result to file '"));
                __mm_s.push_str(&*destFile);
                __mm_s.push_str(&*literal!("'\n"));
                ArcStr::from(__mm_s)
            });
            System::writeFile(destFile.clone(), res.clone())?;
            let false = (wasError) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n### translation of file '"));
                __mm_s.push_str(&*file);
                __mm_s.push_str(&*literal!("' failed!  ###\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("### Error Buffer ###\n"));
            metamodelica::print(Print::getErrorString()?);
            metamodelica::print(literal!("\n### End of Error Buffer ###\n"));
            Print::clearErrorBuf();
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

// ********** Tests ****************
pub(crate) fn testStringEquality(
    mut inStringReturned: ArcStr,
    mut inStringShouldBe: ArcStr,
    mut inPrintResult: bool,
    mut inPrintErrorBuffer: bool,
    mut inTestLabel: ArcStr,
    mut inNotPassedCnt: i32,
) -> Result<i32> {
    let mut outNotPassedCnt: i32;
    outNotPassedCnt = 'mc: {
        let __mc_input = (
            inStringReturned,
            inStringShouldBe,
            inPrintResult,
            inPrintErrorBuffer,
            inTestLabel,
            inNotPassedCnt,
        );
        if let Ok(__v) = (|| -> Result<_> {
            let (mut strRet, mut strShouldBe, mut printResult, mut printErrBuf, mut strLabel, mut notPassedCnt) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut strRes: ArcStr;
            let mut strErrBuf: ArcStr;
            let true = (stringEq(&strRet, &strShouldBe)) else {
                return Err("pattern mismatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n**************************************************\n"));
                __mm_s.push_str(&*strLabel);
                ArcStr::from(__mm_s)
            });
            strRes = if (printResult.clone()) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  returned <<\n"));
                    __mm_s.push_str(&*strRet);
                    __mm_s.push_str(&*literal!(">>\n"));
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("\n result not shown \n")
            };
            metamodelica::print(strRes.clone());
            strErrBuf = Print::getErrorString()?;
            strErrBuf = if (metamodelica::stringEq(&strErrBuf, &(literal!("")))) {
                literal!("")
            } else {
                if (printErrBuf.clone()) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("### Error Buffer ###\n"));
                        __mm_s.push_str(&*strErrBuf);
                        __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n"));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    literal!("### Error Buffer is NOT empty - not shown ###\n")
                }
            };
            metamodelica::print(strErrBuf.clone());
            metamodelica::print(literal!("*** OK ***\n"));
            Print::clearErrorBuf();
            Ok(notPassedCnt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut strRet, mut strShouldBe, mut printResult, mut printErrBuf, mut strLabel, mut notPassedCnt) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut strRes: ArcStr;
            let mut strErrBuf: ArcStr;
            let false = (stringEq(&strRet, &strShouldBe)) else {
                return Err("pattern mismatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n##################################################\n"));
                __mm_s.push_str(&*strLabel);
                ArcStr::from(__mm_s)
            });
            strRes = if (printResult.clone()) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  returned <<\n"));
                    __mm_s.push_str(&*strRet);
                    __mm_s.push_str(&*literal!(">>\nshould be <<\n"));
                    __mm_s.push_str(&*strShouldBe);
                    __mm_s.push_str(&*literal!(">>\n"));
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("\n result not shown \n")
            };
            metamodelica::print(strRes.clone());
            strErrBuf = Print::getErrorString()?;
            strErrBuf = if (metamodelica::stringEq(&strErrBuf, &(literal!("")))) {
                literal!("")
            } else {
                if (printErrBuf.clone()) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("### Error Buffer ###\n"));
                        __mm_s.push_str(&*strErrBuf);
                        __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n"));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    literal!("### Error Buffer is NOT empty - not shown ###\n")
                }
            };
            metamodelica::print(strErrBuf.clone());
            metamodelica::print(literal!("### NOT Passed ###\n"));
            Print::clearErrorBuf();
            Ok(notPassedCnt.clone() + 1)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("-!!!Tpl.tplMainTest failed.\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outNotPassedCnt)
}

pub(crate) fn testTranslateTplFile(
    mut inFile: ArcStr,
    mut inPrintResult: bool,
    mut inPrintErrorBuffer: bool,
    mut inNotPassedCnt: i32,
) -> Result<i32> {
    let mut outNotPassedCnt: i32;
    outNotPassedCnt = 'mc: {
        let __mc_input = (inFile, inPrintResult, inPrintErrorBuffer, inNotPassedCnt);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut file, mut printRes, mut printErrBuf, mut notPassedCnt) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut res: ArcStr;
            let mut resToBe: ArcStr;
            System::writeFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                },
                literal!("Test failed."),
            )?;
            translateFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".tpl"));
                    ArcStr::from(__mm_s)
                },
                &(literal!("")),
            )?;
            res = System::stringReplace(
                System::readFile({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                })?,
                intStringChar(13),
                literal!(""),
            )?;
            resToBe = System::stringReplace(
                System::readFile({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("__testShouldBe.mo"));
                    ArcStr::from(__mm_s)
                })?,
                intStringChar(13),
                literal!(""),
            )?;
            notPassedCnt = testStringEquality(
                res.clone(),
                resToBe.clone(),
                printRes.clone(),
                printErrBuf.clone(),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("translateFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".tpl"));
                    ArcStr::from(__mm_s)
                },
                notPassedCnt.clone(),
            )?;
            Ok(notPassedCnt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut file, mut printRes, mut printErrBuf, mut notPassedCnt) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut res: ArcStr;
            let mut resToBe: ArcStr;
            System::writeFile(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                },
                literal!("Test failed."),
            )?;
            res = System::stringReplace(
                System::readFile({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".mo"));
                    ArcStr::from(__mm_s)
                })?,
                intStringChar(13),
                literal!(""),
            )?;
            resToBe = System::stringReplace(
                System::readFile({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!("__testShouldBe.mo"));
                    ArcStr::from(__mm_s)
                })?,
                intStringChar(13),
                literal!(""),
            )?;
            notPassedCnt = testStringEquality(
                res.clone(),
                resToBe.clone(),
                printRes.clone(),
                printErrBuf.clone(),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("translateFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(".tpl"));
                    ArcStr::from(__mm_s)
                },
                notPassedCnt.clone(),
            )?;
            Ok(notPassedCnt.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outNotPassedCnt)
}

pub(crate) fn tplMainTest(mut inFile: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inFile) {
        Deref @ "a" => {
            let mut r#str: ArcStr;
            let mut strOut: ArcStr;
            let mut ident: ArcStr;
            let mut cval: ArcStr;
            let mut chars: metamodelica::List<ArcStr>;
            let mut txt: Tpl::Text;
            let mut tequal: bool;
            let mut tplPackage: TplAbsyn::TemplPackage;
            let mut mmPckg: TplAbsyn::MMPackage;
            let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
            let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
            let mut astDefs: metamodelica::List<TplAbsyn::ASTDef>;
            let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
            let mut tok: metamodelica::Ref<Tpl::StringToken>;
            let mut tstart: metamodelica::Real;
            let mut lnum: i32;
            let mut colnum: i32;
            let mut llen: i32;
            let mut notPassedCnt: i32;
            notPassedCnt = 0;
            Print::clearErrorBuf();
            metamodelica::print(literal!("\n A Test:\n"));
            tstart = clock();
            txt = Tpl::writeStr(emptyTxt.clone(), literal!("Ahoj Susan"))?;
            txt = Tpl::pushBlock(txt, metamodelica::Ref::new(Tpl::BlockType::BT_ANCHOR { offset: 0 }))?;
            txt = Tpl::writeStr(txt, literal!("Ahoj Susan"))?;
            txt = Tpl::newLine(txt)?;
            txt = Tpl::writeStr(txt, literal!("Ahoj Susan"))?;
            txt = Tpl::popBlock(txt)?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("Ahoj SusanAhoj Susan\n          Ahoj Susan"), true, true, literal!("Anchor"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("Susan"), true, true, literal!("PathIdent IDENT"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Hej"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }) }))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("Hej.Susan"), true, true, literal!("PathIdent PATH_IDENT"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::typedIdents(txt, &(list![(literal!("Hej"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), (literal!("Susan"), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Pa"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Li") }) }) }) }))]))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("Tpl.Text Hej;\nlist<Pa.Li> Susan;"), true, true, literal!("typedIdents"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::typedIdentsEx(txt, &(list![(literal!("Hej"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), (literal!("Susan"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Pa"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Li") }) }) }))]), literal!("input"), literal!("in"))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("input Tpl.Text inHej;\ninput Pa.Li inSusan;"), true, true, literal!("typedIdentsEx"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::mmPackage(txt, &(TplAbsyn::MMPackage { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }), mmDeclarations: list![TplAbsyn::MMDeclaration::MM_IMPORT { isPublic: true, packageName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Pa"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Li"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ko") }) }) }) }, TplAbsyn::MMDeclaration::MM_STR_TOKEN_DECL { isPublic: true, name: literal!("strTokConst"), value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: list![literal!("Susan"), literal!("is"), literal!("beautiful\n")], lastHasNewLine: true }) }, TplAbsyn::MMDeclaration::MM_LITERAL_DECL { isPublic: false, name: literal!("c_literalValueConst"), value: literal!("123"), litType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE() }, TplAbsyn::MMDeclaration::MM_FUN { isPublic: true, name: literal!("MuchFun"), inArgs: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), (literal!("laughLevel"), crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()), (literal!("jokes"), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }))], outArgs: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE())], locals: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), (literal!("laughLevel"), crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()), (literal!("jokes"), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }))], statements: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("out_txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeStr") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_STRING { value: literal!("Susan") })] }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("out_txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeTok") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("out_txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_LINE { line: literal!("Susan is cosmic!\n") }) })] }) })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN }, TplAbsyn::MMDeclaration::MM_FUN { isPublic: true, name: literal!("MoreFun"), inArgs: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), (literal!("v_laughLevel"), metamodelica::Ref::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() })), (literal!("v_jokes"), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }))], outArgs: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE())], locals: list![(literal!("txt"), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE())], statements: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_MATCH { matchCases: list![(list![metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("txt") }), metamodelica::Ref::new(TplAbsyn::MatchingExp::SOME_MATCH { value: metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_AS_MATCH { bindIdent: literal!("v_hej"), matchingExp: metamodelica::Ref::new(TplAbsyn::MatchingExp::STRING_MATCH { value: literal!("Hej") }) }) }), metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("v_jokes") })], list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeStr") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("v_hej") }) })] }) })]), (list![metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("txt") }), metamodelica::Ref::new(TplAbsyn::MatchingExp::SOME_MATCH { value: metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("v_hej") }) }), crate::TplAbsyn::MatchingExp::interned_REST_MATCH()], list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeStr") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_STRING { value: literal!("Not hej:") })] }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeStr") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("v_hej") }) })] }) })]), (list![metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("txt") }), crate::TplAbsyn::MatchingExp::interned_NONE_MATCH(), crate::TplAbsyn::MatchingExp::interned_REST_MATCH()], list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![literal!("txt")], rhs: metamodelica::Ref::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("writeStr") }) }), args: list![metamodelica::Ref::new(TplAbsyn::MMExp::MM_IDENT { ident: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("txt") }) }), metamodelica::Ref::new(TplAbsyn::MMExp::MM_STRING { value: literal!("NONE at all") })] }) })])] })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN }], annotationFooter: literal!("") }))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("package Susan\n\npublic import Tpl;\n\npublic import Pa.Li.Ko;\n\npublic constant Tpl.StringToken strTokConst = Tpl.ST_STRING_LIST({\n                                                  \"Susan\",\n                                                  \"is\",\n                                                  \"beautiful\\n\"\n                                              }, true);\n\nprotected constant Integer c_literalValueConst = 123;\n\npublic function MuchFun\n  input Tpl.Text txt;\n  input Integer laughLevel;\n  input list<String> jokes;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt := Tpl.writeStr(txt, \"Susan\");\n  out_txt := Tpl.writeTok(out_txt, Tpl.ST_LINE(\"Susan is cosmic!\\n\"));\nend MuchFun;\n\npublic function MoreFun\n  input Tpl.Text in_txt;\n  input Option<String> in_v_laughLevel;\n  input list<String> in_v_jokes;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_v_laughLevel, in_v_jokes)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           SOME((v_hej as \"Hej\")),\n           v_jokes )\n      local\n        String v_hej;\n        list<String> v_jokes;\n      algorithm\n        txt = Tpl.writeStr(txt, v_hej);\n      then txt;\n\n    case ( txt,\n           SOME(v_hej),\n           _ )\n      local\n        String v_hej;\n      algorithm\n        txt = Tpl.writeStr(txt, \"Not hej:\");\n        txt = Tpl.writeStr(txt, v_hej);\n      then txt;\n\n    case ( txt,\n           NONE(),\n           _ )\n      algorithm\n        txt = Tpl.writeStr(txt, \"NONE at all\");\n      then txt;\n  end matchcontinue;\nend MoreFun;\n\nend Susan;"), false, false, literal!("mmPackage"), notPassedCnt)?;
            tplPackage = TplAbsyn::TemplPackage { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }), astDefs: list![TplAbsyn::ASTDef { importPackage: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("TplAbsyn") }), isDefault: true, isInterface: true, types: list![(literal!("Ident"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }), (literal!("TypedIdents"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: metamodelica::Ref::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) }), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PathIdent") }) })] }) }) }), (literal!("PathIdent"), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: list![(literal!("IDENT"), list![(literal!("ident"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) }))]), (literal!("PATH_IDENT"), list![(literal!("ident"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) })), (literal!("path"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PathIdent") }) }))])] })] }], templateDefs: list![(literal!("pathIdent"), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: list![(literal!("it"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PathIdent") }) }))], lesc: literal!(""), resc: literal!(""), exp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MATCH { matchExp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("it") }) }), dsi.clone()), cases: list![(metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("IDENT") }), fieldMatchings: metamodelica::nil() }), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("ident") }) }), dsi.clone())), (metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PATH_IDENT") }), fieldMatchings: metamodelica::nil() }), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEMPLATE { items: list![(metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("ident") }) }), dsi.clone()), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(".") }) }), dsi.clone()), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::FUN_CALL { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("pathIdent") }), args: list![(metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("path") }) }), dsi.clone())] }), dsi.clone())], lquote: literal!("\""), rquote: literal!("\"") }), dsi.clone()))] }), dsi.clone()) }), (literal!("typedIdents"), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: list![(literal!("decls"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("TypedIdents") }) }))], lesc: literal!(""), resc: literal!(""), exp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::ESCAPED { exp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MAP { argExp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("decls") }) }), dsi.clone()), ofBinding: metamodelica::Ref::new(TplAbsyn::MatchingExp::TUPLE_MATCH { tupleArgs: list![metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("id") }), metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: literal!("pid") })] }), mapExp: (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEMPLATE { items: list![(metamodelica::Ref::new(TplAbsyn::ExpressionBase::FUN_CALL { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("pathIdent") }), args: list![(metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("pid") }) }), dsi.clone())] }), dsi.clone()), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(" ") }) }), dsi.clone()), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("id") }) }), dsi.clone()), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(";") }) }), dsi.clone())], lquote: literal!("\""), rquote: literal!("\"") }), dsi.clone()), hasIndexIdentOpt: None }), dsi.clone()), options: list![(literal!("separator"), Some((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }), dsi.clone())))] }), dsi.clone()) })], annotationFooter: literal!("") };
            mmPckg = TplAbsyn::transformAST(&tplPackage)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::mmPackage(txt, &mmPckg)?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("package Susan\n\npublic import Tpl;\n\npublic import TplAbsyn;\n\npublic function pathIdent\n  input Tpl.Text in_txt;\n  input TplAbsyn.PathIdent in_i_it;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_i_it)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           TplAbsyn.IDENT(ident = i_ident) )\n      local\n        TplAbsyn.Ident i_ident;\n      algorithm\n        txt = Tpl.writeStr(txt, i_ident);\n      then txt;\n\n    case ( txt,\n           TplAbsyn.PATH_IDENT(ident = i_ident, path = i_path) )\n      local\n        TplAbsyn.PathIdent i_path;\n        TplAbsyn.Ident i_ident;\n      algorithm\n        txt = Tpl.writeStr(txt, i_ident);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\".\"));\n        txt = pathIdent(txt, i_path);\n      then txt;\n\n    else in_txt;\n  end matchcontinue;\nend pathIdent;\n\nprotected function lm_2\n  input Tpl.Text in_txt;\n  input TplAbsyn.TypedIdents in_items;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_items)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           {} )\n      then txt;\n\n    case ( txt,\n           (i_id, i_pid) :: rest )\n      local\n        TplAbsyn.TypedIdents rest;\n        TplAbsyn.PathIdent i_pid;\n        TplAbsyn.Ident i_id;\n      algorithm\n        txt = pathIdent(txt, i_pid);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\" \"));\n        txt = Tpl.writeStr(txt, i_id);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\";\"));\n        txt = Tpl.nextIter(txt);\n        txt = lm_2(txt, rest);\n      then txt;\n\n    case ( txt,\n           _ :: rest )\n      local\n        TplAbsyn.TypedIdents rest;\n      algorithm\n        txt = lm_2(txt, rest);\n      then txt;\n  end matchcontinue;\nend lm_2;\n\npublic function typedIdents\n  input Tpl.Text txt;\n  input TplAbsyn.TypedIdents i_decls;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt := Tpl.pushIter(txt, Tpl.ITER_OPTIONS(0, NONE(), SOME(Tpl.ST_NEW_LINE()), 0, 0, Tpl.ST_NEW_LINE(), 0, Tpl.ST_NEW_LINE()));\n  out_txt := lm_2(out_txt, i_decls);\n  out_txt := Tpl.popIter(out_txt);\nend typedIdents;\n\nend Susan;"), false, false, literal!("transformAST - pathIdent() + typedIdents()"), notPassedCnt)?;
            r#str = literal!("// Hej Susan\n/*this is another dance with Susan */\n/* event I will /*nest*/ into */ //and still comment\n      Susan lives!");
            chars = stringListStringChar(r#str.clone());
            (chars, _) = TplParser::interleave(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?);
            strOut = stringCharListString(chars);
            notPassedCnt = testStringEquality(strOut, literal!("Susan lives!"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.interleave \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("(Susan)");
            chars = stringListStringChar(r#str.clone());
            TplParser::afterKeyword(&chars)?;
            strOut = stringCharListString(chars);
            notPassedCnt = testStringEquality(strOut, literal!("(Susan)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.afterKeyword \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("Susan2:)");
            chars = stringListStringChar(r#str.clone());
            (chars, ident) = TplParser::identifier(&chars)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("*Susan2*:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.identifier \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("Susan:)");
            chars = stringListStringChar(r#str.clone());
            (chars, _, pid) = TplParser::pathIdent(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, pid)?;
            ident = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("*Susan*:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.pathIdent \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("Susan./*comment*/ Susan2 . tpl3_h4:)");
            chars = stringListStringChar(r#str.clone());
            (chars, _, pid) = TplParser::pathIdent(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?)?;
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, pid)?;
            ident = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("*Susan.Susan2.tpl3_h4*:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.pathIdent \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("Tpl.Susan:)");
            chars = stringListStringChar(r#str.clone());
            (chars, _, ts) = TplParser::typeSig(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?)?;
            tequal = ts.clone() == metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Tpl"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }) }) });
            txt = emptyTxt.clone();
            txt = TplCodegen::typeSig(txt, &ts)?;
            ident = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*Tpl.Susan*:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.typeSig \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("list< tuple<Hej.Susan,list <String>,Option< /*uáá*/Integer>> >:)");
            chars = stringListStringChar(r#str.clone());
            (chars, _, ts) = TplParser::typeSig(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?)?;
            tequal = ts.clone() == metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: metamodelica::Ref::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: literal!("Hej"), path: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") }) }) }), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }), metamodelica::Ref::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE() })] }) });
            txt = emptyTxt.clone();
            txt = TplCodegen::typeSig(txt, &ts)?;
            ident = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*list<tuple<Hej.Susan, list<String>, Option<Integer>>>*:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.typeSig \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\ninterface package Susan\n  package TplAbsyn\n    type Ident = String;\n    type TypedIdents = list<tuple<Ident, PathIdent>>;\n\n    uniontype PathIdent\n      record IDENT\n        Ident ident;\n      end IDENT;\n\n      record PATH_IDENT\n        Ident ident;\n        PathIdent path;\n      end PATH_IDENT;\n    end PathIdent;\n  end TplAbsyn;\nend Susan;:)");
            chars = stringListStringChar(r#str);
            (chars, _, pid, astDefs) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, metamodelica::nil())?;
            tequal = astDefs == list![TplAbsyn::ASTDef { importPackage: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("TplAbsyn") }), isDefault: true, isInterface: true, types: list![(literal!("Ident"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }), (literal!("TypedIdents"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: metamodelica::Ref::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) }), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PathIdent") }) })] }) }) }), (literal!("PathIdent"), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: list![(literal!("IDENT"), list![(literal!("ident"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) }))]), (literal!("PATH_IDENT"), list![(literal!("ident"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Ident") }) })), (literal!("path"), metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("PathIdent") }) }))])] })] }];
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, pid)?;
            ident = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*Susan*:)"), true, true, literal!("TplParser.templPackage - absyn - type Ident, TypedIdents, PathIdent \n"), notPassedCnt)?;
            r#str = literal!("\ninterface package Susan\npackage builtin\n  function stringListStringChar\n    input String inString;\n    output list<String> outStringList;\n  end stringListStringChar;\nend builtin;\nend Susan;:)");
            chars = stringListStringChar(r#str);
            (chars, _, pid, astDefs) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, metamodelica::nil())?;
            tequal = astDefs == list![TplAbsyn::ASTDef { importPackage: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("builtin") }), isDefault: true, isInterface: true, types: list![(literal!("stringListStringChar"), TplAbsyn::TypeInfo::TI_FUN_TYPE { inArgs: list![(literal!("inString"), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE())], outArgs: list![(literal!("outStringList"), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: crate::TplAbsyn::TypeSignature::interned_STRING_TYPE() }))], tyVars: metamodelica::nil() })] }] && pid.clone() == metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: literal!("Susan") });
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, pid)?;
            Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*:)"), true, true, literal!("TplParser.templPackage - function stringListStringChar\n"), notPassedCnt)?;
            r#str = literal!("\ninterface package Susan\npackage builtin\n  function stringListStringChar\n    input String inString;\n    output list<String> outStringList;\n  end stringListStringChar;\nend builtin;\n\n\nprotected package Tpl\n  uniontype StringToken\n    record ST_NEW_LINE \"Always outputs the new-line char.\"  end ST_NEW_LINE;\n\n    record ST_STRING \"A string without new-lines in it.\"\n      String value;\n    end ST_STRING;\n\n    record ST_LINE \"A (non-empty) string with new-line at the end.\"\n      String line;\n    end ST_LINE;\n\n    record ST_STRING_LIST \"Every string in the list can have a new-line at its end (but does not have to).\"\n      list<String> strList;\n      Boolean lastHasNewLine \"True when the last string in the list has new-line at the end.\";\n    end ST_STRING_LIST;\n  end StringToken;\nend Tpl;\n\n\npackage TplAbsyn\n  type Ident = String;\n  type TypedIdents = list<tuple<Ident, TypeSignature>>;\n  type StringToken = Tpl.StringToken;\n\n  uniontype PathIdent\n    record IDENT\n      Ident ident;\n    end IDENT;\n\n    record PATH_IDENT\n      Ident ident;\n      PathIdent path;\n    end PATH_IDENT;\n  end PathIdent;\n\n  uniontype TypeSignature\n    record LIST_TYPE\n      TypeSignature ofType;\n    end LIST_TYPE;\n\n    record ARRAY_TYPE  // one-dimensional arrays --> with only (safe) list behaviour\n      TypeSignature ofType;\n    end ARRAY_TYPE;\n\n    record OPTION_TYPE\n      TypeSignature ofType;\n    end OPTION_TYPE;\n\n    record TUPLE_TYPE\n      list<TypeSignature> ofTypes;\n    end TUPLE_TYPE;\n\n    record NAMED_TYPE \"key/path to a TypeInfo list from an AST definition\"\n      PathIdent name;\n    end NAMED_TYPE;\n\n    record STRING_TYPE  end STRING_TYPE;\n    record TEXT_TYPE    end TEXT_TYPE;\n    record STRING_TOKEN_TYPE \"Used only for internal string constants.\" end STRING_TOKEN_TYPE;\n\n    record INTEGER_TYPE end INTEGER_TYPE;\n    record REAL_TYPE    end REAL_TYPE;\n    record BOOLEAN_TYPE end BOOLEAN_TYPE;\n\n    record UNRESOLVED_TYPE \"Errorneous resolving type. Only used during elaboration phase.\"\n      String reason;\n    end UNRESOLVED_TYPE;\n  end TypeSignature;\n\n\n  uniontype MatchingExp\n    record BIND_AS_MATCH\n      Ident bindIdent;\n      MatchingExp matchingExp;\n    end BIND_AS_MATCH;\n\n    record BIND_MATCH\n      Ident bindIdent;\n    end BIND_MATCH;\n\n    record RECORD_MATCH\n      PathIdent tagName;\n      list<tuple<Ident, MatchingExp>> fieldMatchings;\n    end RECORD_MATCH;\n\n    record SOME_MATCH\n      MatchingExp value;\n    end SOME_MATCH;\n\n    record NONE_MATCH end NONE_MATCH;\n\n    record TUPLE_MATCH\n      list<MatchingExp> tupleArgs;\n    end TUPLE_MATCH;\n\n    record LIST_MATCH //non-empty list\n      list<MatchingExp> listElts;\n    end LIST_MATCH;\n\n    record LIST_CONS_MATCH\n      MatchingExp head;\n      MatchingExp rest;\n    end LIST_CONS_MATCH;\n\n    record STRING_MATCH\n      String value;\n    end STRING_MATCH;\n\n    record LITERAL_MATCH\n      String value;\n      TypeSignature litType; // only INTEGER_TYPE, REAL_TYPE or BOOLEAN_TYPE\n    end LITERAL_MATCH;\n\n    record REST_MATCH end REST_MATCH;\n  end MatchingExp;\n\n\n  // **** the (core) output AST\n\n  uniontype MMPackage\n    record MM_PACKAGE\n      PathIdent name;\n      list<MMDeclaration> mmDeclarations;\n    end MM_PACKAGE;\n  end MMPackage;\n\n  type MMMatchCase = tuple<list<MatchingExp>, TypedIdents, list<MMExp>>;\n\n  uniontype MMDeclaration\n    record MM_IMPORT\n      Boolean isPublic;\n      PathIdent packageName;\n    end MM_IMPORT;\n\n    record MM_STR_TOKEN_DECL\n      Boolean isPublic;\n      Ident name;\n      StringToken value;\n    end MM_STR_TOKEN_DECL;\n\n    record MM_LITERAL_DECL\n      Boolean isPublic;\n      Ident name;\n      String value;\n      TypeSignature litType;\n    end MM_LITERAL_DECL;\n\n\n    record MM_FUN\n      Boolean isPublic;\n      Ident name;\n      TypedIdents inArgs; //inTxt inclusive\n      TypedIdents outArgs; // outTxt + extra Texts\n      TypedIdents locals;\n      list<MMExp> statements;\n    end MM_FUN;\n  end MMDeclaration;\n\n  uniontype MMExp\n    record MM_ASSIGN\n      list<Ident> lhsArgs;\n      MMExp rhs;\n    end MM_ASSIGN;\n\n    record MM_FN_CALL\n      PathIdent fnName;\n      list<MMExp> args;\n    end MM_FN_CALL;\n\n    record MM_IDENT\n      PathIdent ident;\n    end MM_IDENT;\n\n    record MM_STR_TOKEN \"constructor of type StringToken\"\n      StringToken value;\n    end MM_STR_TOKEN;\n\n    record MM_STRING \"to pass a string constant as parameter of type String\"\n      String value;\n    end MM_STRING;\n\n    record MM_LITERAL \"to pass a literal constant as parameter of type Integer, Real or Boolean\"\n      String value;\n    end MM_LITERAL;\n\n    record MM_MATCH\n      list<MMMatchCase> matchCases;\n    end MM_MATCH;\n  end MMExp;\nend TplAbsyn;\nend Susan;:)");
            chars = stringListStringChar(r#str);
            (chars, _, pid, _) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, metamodelica::nil())?;
            txt = emptyTxt.clone();
            txt = TplCodegen::pathIdent(txt, pid)?;
            Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("parsed*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("parsed*:)"), true, true, literal!("TplParser.templPackage - all types for Susan's backend\n"), notPassedCnt)?;
            r#str = literal!("\"Susan\"~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa0, _, (__pa1, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa0);
            expB = metamodelica::Own::own(__pa1);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("Susan") }) });
            let __pa2 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa2 } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa2);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*Susan*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\"\\n\"~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa3, _, (__pa4, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa3);
            expB = metamodelica::Own::own(__pa4);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() });
            let __pa5 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa5 } => __pa5.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa5);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*\n*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\",\\n\"~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa6, _, (__pa7, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa6);
            expB = metamodelica::Own::own(__pa7);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_LINE { line: literal!(",\n") }) });
            let __pa8 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa8 } => __pa8.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa8);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*,\n*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\"Susan\nis\\nfantastic!\"~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa9, _, (__pa10, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa9);
            expB = metamodelica::Own::own(__pa10);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: list![literal!("Susan\n"), literal!("is\n"), literal!("fantastic!")], lastHasNewLine: false }) });
            let __pa11 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa11 } => __pa11.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa11);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*Susan\nis\nfantastic!*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\"\nSusan\nis\\n new lined!\n\"~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa12, _, (__pa13, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa12);
            expB = metamodelica::Own::own(__pa13);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: list![literal!("\n"), literal!("Susan\n"), literal!("is\n"), literal!(" new lined!\n")], lastHasNewLine: true }) });
            let __pa14 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa14 } => __pa14.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa14);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*\nSusan\nis\n new lined!\n*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("1234567~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa15, _, (__pa16, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa15);
            expB = metamodelica::Own::own(__pa16);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!("1234567"), litType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE() });
            let __pa17 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa17, litType: _ } => __pa17.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa17);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*1234567*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("- 1234567~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa18, _, (__pa19, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa18);
            expB = metamodelica::Own::own(__pa19);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!("-1234567"), litType: crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE() });
            let __pa20 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa20, litType: _ } => __pa20.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa20);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*-1234567*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("- 1234567.0123e-12~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa21, _, (__pa22, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa21);
            expB = metamodelica::Own::own(__pa22);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!("-1234567.0123e-12"), litType: crate::TplAbsyn::TypeSignature::interned_REAL_TYPE() });
            let __pa23 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa23, litType: _ } => __pa23.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa23);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*-1234567.0123e-12*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!(".0123E12~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa24, _, (__pa25, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa24);
            expB = metamodelica::Own::own(__pa25);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!(".0123E12"), litType: crate::TplAbsyn::TypeSignature::interned_REAL_TYPE() });
            let __pa26 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa26, litType: _ } => __pa26.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa26);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*.0123E12*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("true~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa27, _, (__pa28, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa27);
            expB = metamodelica::Own::own(__pa28);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!("true"), litType: crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE() });
            let __pa29 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa29, litType: _ } => __pa29.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa29);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*true*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("false~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa30, _, (__pa31, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa30);
            expB = metamodelica::Own::own(__pa31);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: literal!("false"), litType: crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE() });
            let __pa32 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa32, litType: _ } => __pa32.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cval = metamodelica::Own::own(__pa32);
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*false*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\\n~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa33, _, (__pa34, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa33);
            expB = metamodelica::Own::own(__pa34);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() });
            let __pa35 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa35 } => __pa35.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa35);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*\n*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("\\\"\\n\\n\\ ~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa36, _, (__pa37, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa36);
            expB = metamodelica::Own::own(__pa37);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: list![literal!("\"\n"), literal!("\n"), literal!(" ")], lastHasNewLine: false }) });
            let __pa38 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa38 } => __pa38.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa38);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*\"\n\n *~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("'Susan'~:)");
            chars = stringListStringChar(r#str.clone());
            let (__pa39, _, (__pa40, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars, literal!("in memory test"))?, literal!("<"), literal!(">"), false)?;
            chars = metamodelica::Own::own(__pa39);
            expB = metamodelica::Own::own(__pa40);
            tequal = expB.clone() == metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("Susan") }) });
            let __pa41 = ::match_deref::match_deref! { match &(expB) {
                Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa41 } => __pa41.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tok = metamodelica::Own::own(__pa41);
            txt = emptyTxt.clone();
            txt = Tpl::writeTok(txt, tok)?;
            strOut = Tpl::textString(txt)?;
            strOut = { let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal)); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars)); ArcStr::from(__mm_s) };
            notPassedCnt = testStringEquality(strOut, literal!("true*Susan*~:)"), true, true, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }, notPassedCnt)?;
            r#str = literal!("Susan:)");
            chars = stringListStringChar(r#str);
            llen = TplParser::charsTillEndOfLine(&chars, 1)?;
            let __pa42 = ::match_deref::match_deref! { match &(chars) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa42 } } => __pa42.clone(),
                _ => return Err("pattern mismatch"),
            } };
            chars = metamodelica::Own::own(__pa42);
            (lnum, colnum) = TplParser::getPosition(chars.clone(), &(TplParser::LineInfo { parseInfo: TplParser::ParseInfo { fileName: literal!("test - no file"), errors: metamodelica::nil(), wasFatalError: false }, lineNumber: 11, lineLength: llen, startOfLineChars: chars }))?;
            notPassedCnt = testStringEquality({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(lnum)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(colnum)); __mm_s.push_str(&*literal!(" of ")); __mm_s.push_str(&*intString(llen)); ArcStr::from(__mm_s) }, literal!("11,3 of 8"), true, true, literal!("TplParser.charsTillEndOfLine and getPosition \n"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = statement(txt, &(metamodelica::Ref::new(Statement::WHILE { condition: metamodelica::Ref::new(Exp::BINARY { lhs: metamodelica::Ref::new(Exp::VARIABLE { name: literal!("x") }), op: crate::TplMain::Operator::LESS, rhs: metamodelica::Ref::new(Exp::ICONST { value: 20 }) }), statements: list![metamodelica::Ref::new(Statement::ASSIGN { lhs: metamodelica::Ref::new(Exp::VARIABLE { name: literal!("x") }), rhs: metamodelica::Ref::new(Exp::BINARY { lhs: metamodelica::Ref::new(Exp::VARIABLE { name: literal!("x") }), op: crate::TplMain::Operator::PLUS, rhs: metamodelica::Ref::new(Exp::BINARY { lhs: metamodelica::Ref::new(Exp::VARIABLE { name: literal!("y") }), op: crate::TplMain::Operator::TIMES, rhs: metamodelica::Ref::new(Exp::ICONST { value: 2 }) }) }) })] })));
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("while((x < 20)) {\n  x = (x + (y * 2));\n}"), true, true, literal!("Paper Example statement()"), notPassedCnt)?;
            txt = emptyTxt.clone();
            txt = intMatrix(txt, &(list![list![1, 2, 3, 4, 5], list![6, 7, 8, 9, 10], list![11, 12, 13, 14, 15]]))?;
            r#str = Tpl::textString(txt)?;
            notPassedCnt = testStringEquality(r#str, literal!("[ 1, 2, 3, 4, 5;\n  6, 7, 8, 9, 10;\n  11, 12, 13, 14, 15 ]"), true, true, literal!("intMatrix() from test.tpl"), notPassedCnt)?;
            notPassedCnt = testTranslateTplFile(literal!("TplCodegen"), false, false, notPassedCnt)?;
            notPassedCnt = testTranslateTplFile(literal!("paper"), false, false, notPassedCnt)?;
            notPassedCnt = testTranslateTplFile(literal!("test"), false, true, notPassedCnt)?;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All tests took ")); __mm_s.push_str(&*realString(clock() - tstart)); __mm_s.push_str(&*literal!(" seconds.\n")); ArcStr::from(__mm_s) });
            r#str = if (notPassedCnt == 0) {literal!("\n ***** All a) tests OK *****\n\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n #### ")); __mm_s.push_str(&*intString(notPassedCnt)); __mm_s.push_str(&*literal!(" test")); __mm_s.push_str(&*if (notPassedCnt > 1) {literal!("s")} else {literal!("")}); __mm_s.push_str(&*literal!(" DID NOT passed ####\n\n")); ArcStr::from(__mm_s) }};
            metamodelica::print(r#str);
            ()
        },
        r#str => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n######## tplMainTest '")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("' (fatally) failed!  ########\n")); ArcStr::from(__mm_s) });
            metamodelica::print(literal!("### Error Buffer ###\n"));
            metamodelica::print(Print::getErrorString()?);
            metamodelica::print(literal!("\n### End of Error Buffer ###\n"));
            Print::clearErrorBuf();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

/* the paper example */
/// Algorithmic stmts
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Statement {
    /// An assignment stmt
    ASSIGN {
        lhs: metamodelica::Ref<Exp>,
        rhs: metamodelica::Ref<Exp>,
    },
    /// A while statement
    WHILE {
        condition: metamodelica::Ref<Exp>,
        statements: metamodelica::List<metamodelica::Ref<Statement>>,
    },
}
impl metamodelica::gc::MMTrace for Statement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Statement::ASSIGN { lhs, rhs } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                Ok(())
            }
            Statement::WHILE { condition, statements } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statements, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Statement::{ASSIGN, WHILE};

/// Expression nodes
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Exp {
    /// Integer constant value
    ICONST { value: i32 },
    /// Variable reference
    VARIABLE { name: ArcStr },
    /// Binary ops
    BINARY {
        lhs: metamodelica::Ref<Exp>,
        op: Operator,
        rhs: metamodelica::Ref<Exp>,
    },
}
impl metamodelica::gc::MMTrace for Exp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Exp::ICONST { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            Exp::VARIABLE { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            Exp::BINARY { lhs, op, rhs } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(op, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::Exp::{BINARY, ICONST, VARIABLE};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Operator {
    PLUS,
    TIMES,
    LESS,
}
impl metamodelica::gc::MMTrace for Operator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Operator::PLUS => Ok(()),
            Operator::TIMES => Ok(()),
            Operator::LESS => Ok(()),
        }
    }
}
pub(crate) use self::Operator::{LESS, PLUS, TIMES};

fn lm_1(mut in_txt: Tpl::Text, mut in_items: &metamodelica::List<metamodelica::Ref<Statement>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt, &**in_items);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: i_it, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = statement(txt.clone(), metamodelica::AsArg::as_arg(&i_it));
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_1(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_1(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out_txt)
}

pub(crate) fn statement(mut in_txt: Tpl::Text, mut in_i_it: &metamodelica::Ref<Statement>) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt, &**in_i_it);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Statement::ASSIGN { lhs: i_lhs, rhs: i_rhs }) => {
                    let mut txt = (*txt).clone();
                    txt = exp(txt.clone(), metamodelica::AsArg::as_arg(&i_lhs));
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(" = ") }))?;
                    txt = exp(txt.clone(), metamodelica::AsArg::as_arg(&i_rhs));
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(";") }))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Statement::WHILE { condition: i_condition, statements: i_statements }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("while(") }))?;
                    txt = exp(txt.clone(), metamodelica::AsArg::as_arg(&i_condition));
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_LINE { line: literal!(") {\n") }))?;
                    txt = Tpl::pushBlock(txt.clone(), metamodelica::Ref::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
                    txt = Tpl::pushIter(txt.clone(), metamodelica::Ref::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
                    txt = lm_1(txt.clone(), metamodelica::AsArg::as_arg(&i_statements))?;
                    txt = Tpl::popIter(txt.clone())?;
                    txt = Tpl::softNewLine(txt.clone())?;
                    txt = Tpl::popBlock(txt.clone())?;
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("}") }))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, _) => {
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_txt
}

pub(crate) fn exp(mut in_txt: Tpl::Text, mut in_i_it: &metamodelica::Ref<Exp>) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), &**in_i_it);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::ICONST { value: i_value }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), intString(i_value.clone()))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::VARIABLE { name: i_name }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), i_name.clone())?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::BINARY { lhs: i_lhs, op: i_op, rhs: i_rhs }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("(") }))?;
                    txt = exp(txt.clone(), metamodelica::AsArg::as_arg(&i_lhs));
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(" ") }))?;
                    txt = oper(txt.clone(), i_op.clone());
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(" ") }))?;
                    txt = exp(txt.clone(), metamodelica::AsArg::as_arg(&i_rhs));
                    txt = Tpl::writeTok(txt.clone(), metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(")") }))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(in_txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_txt
}

pub(crate) fn oper(mut in_txt: Tpl::Text, mut in_i_it: Operator) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_i_it);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::PLUS { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            txt = Tpl::writeTok(
                txt.clone(),
                metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("+") }),
            )?;
            Ok(txt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::TIMES { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            txt = Tpl::writeTok(
                txt.clone(),
                metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("*") }),
            )?;
            Ok(txt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::LESS { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            txt = Tpl::writeTok(
                txt.clone(),
                metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("<") }),
            )?;
            Ok(txt.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(in_txt.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_txt
}

/* **************************/
/* intMatrix from test.tpl */
/* **************************/
fn lm_54(mut in_txt: Tpl::Text, mut in_items: &metamodelica::List<i32>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt, &**in_items);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: i_it, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), intString(i_it.clone()))?;
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_54(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_54(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out_txt)
}

fn lm_55(mut in_txt: Tpl::Text, mut in_items: &metamodelica::List<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt, &**in_items);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: i_intLst, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::pushIter(txt.clone(), metamodelica::Ref::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(", ") })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
                    txt = lm_54(txt.clone(), metamodelica::AsArg::as_arg(&i_intLst))?;
                    txt = Tpl::popIter(txt.clone())?;
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_55(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_55(txt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok(txt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out_txt)
}

pub(crate) fn intMatrix(
    mut txt: Tpl::Text,
    mut i_lstOfLst: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(
        txt,
        metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("[ ") }),
    )?;
    out_txt = Tpl::pushBlock(out_txt, metamodelica::Ref::new(Tpl::BlockType::BT_ANCHOR { offset: 0 }))?;
    out_txt = Tpl::pushIter(
        out_txt,
        metamodelica::Ref::new(Tpl::IterOptions {
            startIndex0: 0,
            empty: None,
            separator: Some(metamodelica::Ref::new(Tpl::StringToken::ST_LINE {
                line: literal!(";\n"),
            })),
            alignNum: 0,
            alignOfset: 0,
            alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(),
            wrapWidth: 0,
            wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(),
        }),
    )?;
    out_txt = lm_55(out_txt, i_lstOfLst)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(
        out_txt,
        metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!(" ]") }),
    )?;
    Ok(out_txt)
}

/* **************************/
/* end of intMatrix from test.tpl */
/* **************************/
// !!! weird type behavior of MM
/*
public
function MuchFun2
  input Tpl.Text txt;
  input Integer inlaughLevel;
  input list<String> injokes;

  output Integer txt;

  Tpl.Text txt1;
  Integer laughLevel;
  list<String> jokes;
algorithm
(txt) := Tpl.writeStr(txt, "Susan");
txt := 1;
//(txt1) := Tpl.writeStr(txt, "Susan");

//(txt) := Tpl.writeTok(txt, Tpl.ST_LINE("Susan is cosmic!\n"));
end MuchFun2;
*/
