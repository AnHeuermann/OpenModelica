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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::DAEDumpTypes::*;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::Graphviz;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Print;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub(crate) fn dump(
    mut dae: &DAE::DAElist,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<()> {
    let () = (match dae.clone() {
        DAE::DAElist {
            elementLst: ref daelist,
        } => {
            List::map_0(
                &(sortFunctions(DAEUtil::getFunctionList(functionTree, false)?)?),
                &move |__a0: DAE::Function| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dumpFunction(&__a0))
                },
            )?;
            List::map_0(
                metamodelica::AsArg::as_arg(&daelist),
                &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dumpExtObjectClass(&__a0))
                },
            )?;
            List::map_0(
                metamodelica::AsArg::as_arg(&daelist),
                &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dumpCompElement(&__a0))
                },
            )?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn dumpFunctionNamesStr(mut funcs: &metamodelica::Ref<AvlTreePathFunction::Tree>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        List::map(
            sortFunctions(DAEUtil::getFunctionList(funcs, false)?)?,
            &move |__a0: DAE::Function| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(functionNameStr(&__a0))
            },
        )?,
        literal!(","),
    );
    Ok(r#str)
}

pub(crate) fn functionNameStr(mut inElement: &DAE::Function) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let DAE::Function::FUNCTION { path: ref fpath, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut res: ArcStr = res.clone();
            res = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
            Ok((res.clone(), res.clone()))
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let DAE::Function::RECORD_CONSTRUCTOR { path: ref fpath, .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut res: ArcStr = res.clone();
            res = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
            Ok((res.clone(), res.clone()))
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn sortFunctions(mut funcs: metamodelica::List<DAE::Function>) -> Result<metamodelica::List<DAE::Function>> {
    let mut sortedFuncs: metamodelica::List<DAE::Function>;
    sortedFuncs = List::sort(
        funcs,
        (std::sync::Arc::new(
            move |__a0: DAE::Function, __a1: DAE::Function| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(funcGreaterThan(&__a0, &__a1))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Function, DAE::Function) -> Result<bool> + 'static>),
    )?;
    Ok(sortedFuncs)
}

fn funcGreaterThan(mut func1: &DAE::Function, mut func2: &DAE::Function) -> bool {
    let mut res: bool;
    res = (match func2.clone() {
        _ => {
            res = stringCompare(&(functionNameStr(func1)), &(functionNameStr(func2))) > 0;
            res
        }
        _ => true,
    });
    res
}

pub(crate) fn dumpOperatorString(mut op: &DAE::Operator) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match op.clone() {
        DAE::Operator::ADD { .. } => {
            literal!(" ADD ")
        }
        DAE::Operator::SUB { .. } => {
            literal!(" SUB ")
        }
        DAE::Operator::MUL { .. } => {
            literal!(" MUL ")
        }
        DAE::Operator::DIV { .. } => {
            literal!(" DIV ")
        }
        DAE::Operator::POW { .. } => {
            literal!(" POW ")
        }
        DAE::Operator::UMINUS { .. } => {
            literal!(" UMINUS ")
        }
        DAE::Operator::UMINUS_ARR { .. } => {
            literal!(" UMINUS_ARR ")
        }
        DAE::Operator::ADD_ARR { .. } => {
            literal!(" ADD_ARR ")
        }
        DAE::Operator::SUB_ARR { .. } => {
            literal!(" SUB_ARR ")
        }
        DAE::Operator::MUL_ARR { .. } => {
            literal!(" MUL_ARR ")
        }
        DAE::Operator::DIV_ARR { .. } => {
            literal!(" DIV_ARR ")
        }
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            literal!(" MUL_ARRAY_SCALAR ")
        }
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => {
            literal!(" ADD_ARRAY_SCALAR ")
        }
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => {
            literal!(" SUB_SCALAR_ARRAY ")
        }
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            literal!(" MUL_SCALAR_PRODUCT ")
        }
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            literal!(" MUL_MATRIX_PRODUCT ")
        }
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            literal!(" DIV_ARRAY_SCALAR ")
        }
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => {
            literal!(" DIV_SCALAR_ARRAY ")
        }
        DAE::Operator::POW_ARRAY_SCALAR { .. } => {
            literal!(" POW_ARRAY_SCALAR ")
        }
        DAE::Operator::POW_SCALAR_ARRAY { .. } => {
            literal!(" POW_SCALAR_ARRAY ")
        }
        DAE::Operator::POW_ARR { .. } => {
            literal!(" POW_ARR ")
        }
        DAE::Operator::POW_ARR2 { .. } => {
            literal!(" POW_ARR2 ")
        }
        DAE::Operator::OR { ty: _ } => {
            literal!(" OR ")
        }
        DAE::Operator::AND { ty: _ } => {
            literal!(" AND ")
        }
        DAE::Operator::NOT { ty: _ } => {
            literal!(" NOT ")
        }
        DAE::Operator::LESSEQ { .. } => {
            literal!(" LESSEQ ")
        }
        DAE::Operator::GREATER { .. } => {
            literal!(" GREATER ")
        }
        DAE::Operator::GREATEREQ { .. } => {
            literal!(" GREATEREQ ")
        }
        DAE::Operator::LESS { .. } => {
            literal!(" LESS ")
        }
        DAE::Operator::EQUAL { .. } => {
            literal!(" EQUAL ")
        }
        DAE::Operator::NEQUAL { .. } => {
            literal!(" NEQUAL ")
        }
        DAE::Operator::USERDEFINED { fqName: ref p } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" Userdefined:"));
            __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
            __mm_s.push_str(&*literal!(" "));
            ArcStr::from(__mm_s)
        }
        _ => {
            literal!(" --UNDEFINED-- ")
        }
    });
    Ok(r#str)
}

pub(crate) fn dumpOperatorSymbol(mut op: &DAE::Operator) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match op.clone() {
        DAE::Operator::ADD { ty: _ } => {
            literal!(" + ")
        }
        DAE::Operator::SUB { ty: _ } => {
            literal!(" - ")
        }
        DAE::Operator::MUL { ty: _ } => {
            literal!(" .* ")
        }
        DAE::Operator::DIV { ty: _ } => {
            literal!(" / ")
        }
        DAE::Operator::POW { ty: _ } => {
            literal!(" ^ ")
        }
        DAE::Operator::UMINUS { ty: _ } => {
            literal!(" - ")
        }
        DAE::Operator::UMINUS_ARR { ty: _ } => {
            literal!(" - ")
        }
        DAE::Operator::ADD_ARR { ty: _ } => {
            literal!(" + ")
        }
        DAE::Operator::SUB_ARR { ty: _ } => {
            literal!(" - ")
        }
        DAE::Operator::MUL_ARR { ty: _ } => {
            literal!(" .* ")
        }
        DAE::Operator::DIV_ARR { ty: _ } => {
            literal!(" ./ ")
        }
        DAE::Operator::MUL_ARRAY_SCALAR { ty: _ } => {
            literal!(" * ")
        }
        DAE::Operator::ADD_ARRAY_SCALAR { ty: _ } => {
            literal!(" .+ ")
        }
        DAE::Operator::SUB_SCALAR_ARRAY { ty: _ } => {
            literal!(" .- ")
        }
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ } => {
            literal!(" * ")
        }
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ } => {
            literal!(" * ")
        }
        DAE::Operator::DIV_ARRAY_SCALAR { ty: _ } => {
            literal!(" / ")
        }
        DAE::Operator::DIV_SCALAR_ARRAY { ty: _ } => {
            literal!(" ./ ")
        }
        DAE::Operator::POW_ARRAY_SCALAR { ty: _ } => {
            literal!(" .^ ")
        }
        DAE::Operator::POW_SCALAR_ARRAY { ty: _ } => {
            literal!(" .^ ")
        }
        DAE::Operator::POW_ARR { ty: _ } => {
            literal!(" ^ ")
        }
        DAE::Operator::POW_ARR2 { ty: _ } => {
            literal!(" .^ ")
        }
        DAE::Operator::OR { ty: _ } => {
            literal!(" or ")
        }
        DAE::Operator::AND { ty: _ } => {
            literal!(" and ")
        }
        DAE::Operator::NOT { ty: _ } => {
            literal!(" not ")
        }
        DAE::Operator::LESSEQ { ty: _ } => {
            literal!(" <= ")
        }
        DAE::Operator::GREATER { ty: _ } => {
            literal!(" > ")
        }
        DAE::Operator::GREATEREQ { ty: _ } => {
            literal!(" >= ")
        }
        DAE::Operator::LESS { ty: _ } => {
            literal!(" < ")
        }
        DAE::Operator::EQUAL { ty: _ } => {
            literal!(" == ")
        }
        DAE::Operator::NEQUAL { ty: _ } => {
            literal!(" <> ")
        }
        DAE::Operator::USERDEFINED { fqName: ref p } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" Userdefined:"));
            __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
            __mm_s.push_str(&*literal!(" "));
            ArcStr::from(__mm_s)
        }
        _ => {
            literal!(" --UNDEFINED-- ")
        }
    });
    Ok(r#str)
}

fn dumpStartValue(mut inStartValue: Option<metamodelica::Ref<DAE::Exp>>) -> () {
    let () = 'mc: {
        let __mc_input = inStartValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(e) => {
                    Print::printBuf(literal!("(start="))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(")"))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn dumpStartValueStr(mut inStartValue: Option<metamodelica::Ref<DAE::Exp>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inStartValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(e) => {
                    let mut s: ArcStr;
                    let mut res: ArcStr;
                    s = ExpressionBasics::printExpStr(e.clone())?;
                    res = stringAppendList(list![literal!("(start="), s.clone(), literal!(")")]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    outString
}

pub(crate) fn dumpExtDeclStr(mut inExternalDecl: &DAE::ExternalDecl) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inExternalDecl.clone() {
        DAE::ExternalDecl {
            name: mut id,
            args: ref extargs,
            returnArg: mut retty,
            language: mut lang,
            ..
        } => {
            let mut extargsstr: ArcStr;
            let mut rettystr: ArcStr;
            let mut r#str: ArcStr;
            extargsstr = List::toStringCustom(
                extargs.clone(),
                &move |__a0: DAE::ExtArg| dumpExtArgStr(&__a0),
                literal!(""),
                literal!(""),
                literal!(", "),
                literal!(""),
                true,
                0,
            )?;
            rettystr = dumpExtArgStr(metamodelica::AsArg::as_arg(&retty))?;
            rettystr = if (stringEq(&rettystr, &(literal!("")))) {
                rettystr
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*rettystr);
                    __mm_s.push_str(&*literal!(" = "));
                    ArcStr::from(__mm_s)
                }
            };
            r#str = stringAppendList(list![
                literal!("external \""),
                lang.clone(),
                literal!("\" "),
                rettystr,
                id.clone(),
                literal!("("),
                extargsstr,
                literal!(");")
            ]);
            r#str
        }
    });
    Ok(outString)
}

pub fn dumpExtArgStr(mut inExtArg: &DAE::ExtArg) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inExtArg.clone() {
        DAE::ExtArg::NOEXTARG { .. } => {
            literal!("")
        }
        DAE::ExtArg::EXTARG {
            componentRef: ref cr, ..
        } => {
            let mut crstr: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            crstr
        }
        DAE::ExtArg::EXTARGEXP { exp: mut exp, .. } => {
            let mut crstr: ArcStr;
            crstr = ExpressionBasics::printExpStr(exp.clone())?;
            crstr
        }
        DAE::ExtArg::EXTARGSIZE {
            componentRef: ref cr,
            exp: ref dim,
            ..
        } => {
            let mut crstr: ArcStr;
            let mut r#str: ArcStr;
            let mut dimstr: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
            dimstr = ExpressionBasics::printExpStr(dim.clone())?;
            r#str = stringAppendList(list![literal!("size("), crstr, literal!(", "), dimstr, literal!(")")]);
            r#str
        }
    });
    Ok(outString)
}

fn dumpCompElement(mut inElement: &metamodelica::Ref<DAE::Element>) -> () {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { ident: n, dAElist: l, comment: c, .. } => {
                    Print::printBuf(literal!("class "))?;
                    Print::printBuf(n.clone())?;
                    dumpCommentOption(c.clone())?;
                    Print::printBuf(literal!("\n"))?;
                    dumpElements(l.clone())?;
                    Print::printBuf(literal!("end "))?;
                    Print::printBuf(n.clone())?;
                    Print::printBuf(literal!(";\n"))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn dumpElements(mut l: metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<()> {
    dumpVars(l.clone(), false)?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpExtObjectClass(&__a0))
        },
    )?;
    Print::printBuf(literal!("initial equation\n"))?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpInitialEquation(&__a0))
        },
    )?;
    Print::printBuf(literal!("equation\n"))?;
    List::map_0(&l, &move |__a0: metamodelica::Ref<DAE::Element>| dumpEquation(&__a0))?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpInitialAlgorithm(&__a0))
        },
    )?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpAlgorithm(&__a0))
        },
    )?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpCompElement(&__a0))
        },
    )?;
    Ok(())
}

pub(crate) fn dumpFunctionElements(mut l: metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<()> {
    dumpVars(l.clone(), true)?;
    List::map_0(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpAlgorithm(&__a0))
        },
    )?;
    Ok(())
}

fn dumpVars(mut lst: metamodelica::List<metamodelica::Ref<DAE::Element>>, mut printTypeDimension: bool) -> Result<()> {
    let mut r#str: ArcStr;
    let mut myStream: IOStream::IOStream;
    myStream = IOStream::create(literal!(""), openmodelica_util::IOStream::IOStreamType::LIST)?;
    myStream = dumpVarsStream(lst, printTypeDimension, myStream)?;
    r#str = IOStream::string(&myStream)?;
    Print::printBuf(r#str)?;
    Ok(())
}

fn dumpKind(mut inVarKind: DAE::VarKind) -> Result<()> {
    let () = (match inVarKind {
        DAE::VarKind::CONST { .. } => {
            Print::printBuf(literal!(" constant  "))?;
            ()
        }
        DAE::VarKind::PARAM { .. } => {
            Print::printBuf(literal!(" parameter "))?;
            ()
        }
        DAE::VarKind::DISCRETE { .. } => {
            Print::printBuf(literal!(" discrete  "))?;
            ()
        }
        DAE::VarKind::VARIABLE { .. } => {
            Print::printBuf(literal!("           "))?;
            ()
        }
    });
    Ok(())
}

pub fn dumpKindStr(mut inVarKind: DAE::VarKind) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarKind {
        DAE::VarKind::CONST { .. } => literal!("constant "),
        DAE::VarKind::PARAM { .. } => literal!("parameter "),
        DAE::VarKind::DISCRETE { .. } => literal!("discrete "),
        DAE::VarKind::VARIABLE { .. } => literal!(""),
    });
    outString
}

fn dumpDirection(mut inVarDirection: DAE::VarDirection) -> Result<()> {
    let () = (match inVarDirection {
        DAE::VarDirection::INPUT { .. } => {
            Print::printBuf(literal!(" input  "))?;
            ()
        }
        DAE::VarDirection::OUTPUT { .. } => {
            Print::printBuf(literal!(" output "))?;
            ()
        }
        DAE::VarDirection::BIDIR { .. } => {
            Print::printBuf(literal!("        "))?;
            ()
        }
    });
    Ok(())
}

fn dumpParallelism(mut inVarParallelism: DAE::VarParallelism) -> Result<()> {
    let () = (match inVarParallelism {
        DAE::VarParallelism::NON_PARALLEL { .. } => {
            Print::printBuf(literal!("        "))?;
            ()
        }
        DAE::VarParallelism::PARGLOBAL { .. } => {
            Print::printBuf(literal!(" parglobal "))?;
            ()
        }
        DAE::VarParallelism::PARLOCAL { .. } => {
            Print::printBuf(literal!(" parlocal "))?;
            ()
        }
    });
    Ok(())
}

pub fn dumpDirectionStr(mut inVarDirection: DAE::VarDirection) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarDirection {
        DAE::VarDirection::INPUT { .. } => literal!("input "),
        DAE::VarDirection::OUTPUT { .. } => literal!("output "),
        DAE::VarDirection::BIDIR { .. } => literal!(""),
    });
    outString
}

fn dumpStateSelectStr(mut inStateSelect: DAE::StateSelect) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inStateSelect {
        DAE::StateSelect::NEVER { .. } => literal!("StateSelect.never"),
        DAE::StateSelect::AVOID { .. } => literal!("StateSelect.avoid"),
        DAE::StateSelect::PREFER { .. } => literal!("StateSelect.prefer"),
        DAE::StateSelect::ALWAYS { .. } => literal!("StateSelect.always"),
        DAE::StateSelect::DEFAULT { .. } => literal!("StateSelect.default"),
    });
    outString
}

fn dumpUncertaintyStr(mut uncertainty: DAE::Uncertainty) -> ArcStr {
    let mut out: ArcStr;
    out = (match uncertainty {
        DAE::Uncertainty::GIVEN { .. } => literal!("Uncertainty.given"),
        DAE::Uncertainty::SOUGHT { .. } => literal!("Uncertainty.sought"),
        DAE::Uncertainty::REFINE { .. } => literal!("Uncertainty.refine"),
        DAE::Uncertainty::PROPAGATE { .. } => literal!("Uncertainty.propagate"),
    });
    out
}

fn dumpDistributionStr(mut distribution: &metamodelica::Ref<DAE::Distribution>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (match &**distribution {
        DAE::Distribution {
            name,
            params,
            paramNames,
        } => {
            let mut name_str: ArcStr;
            let mut params_str: ArcStr;
            let mut paramNames_str: ArcStr;
            name_str = ExpressionBasics::printExpStr(name.clone())?;
            params_str = ExpressionBasics::printExpStr(params.clone())?;
            paramNames_str = ExpressionBasics::printExpStr(paramNames.clone())?;
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Distribution(name = "));
                __mm_s.push_str(&*name_str);
                __mm_s.push_str(&*literal!(", params = "));
                __mm_s.push_str(&*params_str);
                __mm_s.push_str(&*literal!(", paramNames= "));
                __mm_s.push_str(&*paramNames_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(out)
}

pub(crate) fn dumpVariableAttributes(mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>) -> Result<()> {
    let mut res: ArcStr;
    res = dumpVariableAttributesStr(attr);
    Print::printBuf(res)?;
    Ok(())
}

pub(crate) fn dumpVariableAttributesStr(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inVariableAttributesOption;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quant, unit, displayUnit, min, max, start: initialExp, fixed, nominal, stateSelectOption: stateSel, uncertainOption: uncertainty, distributionOption: dist, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr;
                    let mut unit_str: ArcStr;
                    let mut displayUnit_str: ArcStr;
                    let mut stateSel_str: ArcStr;
                    let mut min_str: ArcStr;
                    let mut max_str: ArcStr;
                    let mut nominal_str: ArcStr;
                    let mut initial_str: ArcStr;
                    let mut fixed_str: ArcStr;
                    let mut uncertainty_str: ArcStr;
                    let mut dist_str: ArcStr;
                    let mut res_1: ArcStr;
                    let mut res: ArcStr;
                    let mut startOriginStr: ArcStr;
                    quantity = getOptionWithConcatStr(quant.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("quantity = "))?;
                    unit_str = getOptionWithConcatStr(unit.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("unit = "))?;
                    displayUnit_str = getOptionWithConcatStr(displayUnit.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("displayUnit = "))?;
                    stateSel_str = getOptionWithConcatStr(stateSel.clone(), (std::sync::Arc::new(fnptr!(dumpStateSelectStr, DAE::StateSelect)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::StateSelect) -> Result<ArcStr> + 'static>), literal!("stateSelect = "))?;
                    min_str = getOptionWithConcatStr(min.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("min = "))?;
                    max_str = getOptionWithConcatStr(max.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("max = "))?;
                    nominal_str = getOptionWithConcatStr(nominal.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("nominal = "))?;
                    initial_str = getOptionWithConcatStr(initialExp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("start = "))?;
                    fixed_str = getOptionWithConcatStr(fixed.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("fixed = "))?;
                    uncertainty_str = getOptionWithConcatStr(uncertainty.clone(), (std::sync::Arc::new(fnptr!(dumpUncertaintyStr, DAE::Uncertainty)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Uncertainty) -> Result<ArcStr> + 'static>), literal!("uncertainty = "))?;
                    dist_str = getOptionWithConcatStr(dist.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Distribution>| dumpDistributionStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Distribution>) -> Result<ArcStr> + 'static>), literal!("distribution = "))?;
                    startOriginStr = getStartOrigin(startOrigin.clone())?;
                    res_1 = Util::stringDelimitListNonEmptyElts(list![quantity.clone(), unit_str.clone(), displayUnit_str.clone(), min_str.clone(), max_str.clone(), initial_str.clone(), fixed_str.clone(), nominal_str.clone(), stateSel_str.clone(), uncertainty_str.clone(), dist_str.clone(), startOriginStr.clone()], literal!(", "))?;
                    res = if (stringEmpty(&res_1)) {literal!("")} else {stringAppendList(list![literal!("("), res_1.clone(), literal!(")")])};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: quant, min, max, start: initialExp, fixed, uncertainOption: uncertainty, distributionOption: dist, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr;
                    let mut min_str: ArcStr;
                    let mut max_str: ArcStr;
                    let mut initial_str: ArcStr;
                    let mut fixed_str: ArcStr;
                    let mut uncertainty_str: ArcStr;
                    let mut dist_str: ArcStr;
                    let mut res_1: ArcStr;
                    let mut res: ArcStr;
                    let mut startOriginStr: ArcStr;
                    quantity = getOptionWithConcatStr(quant.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("quantity = "))?;
                    min_str = getOptionWithConcatStr(min.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("min = "))?;
                    max_str = getOptionWithConcatStr(max.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("max = "))?;
                    initial_str = getOptionWithConcatStr(initialExp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("start = "))?;
                    fixed_str = getOptionWithConcatStr(fixed.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("fixed = "))?;
                    uncertainty_str = getOptionWithConcatStr(uncertainty.clone(), (std::sync::Arc::new(fnptr!(dumpUncertaintyStr, DAE::Uncertainty)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::Uncertainty) -> Result<ArcStr> + 'static>), literal!("uncertainty = "))?;
                    dist_str = getOptionWithConcatStr(dist.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Distribution>| dumpDistributionStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Distribution>) -> Result<ArcStr> + 'static>), literal!("distribution = "))?;
                    startOriginStr = getStartOrigin(startOrigin.clone())?;
                    res_1 = Util::stringDelimitListNonEmptyElts(list![quantity.clone(), min_str.clone(), max_str.clone(), initial_str.clone(), fixed_str.clone(), uncertainty_str.clone(), dist_str.clone(), startOriginStr.clone()], literal!(", "))?;
                    res = if (stringEmpty(&res_1)) {literal!("")} else {stringAppendList(list![literal!("("), res_1.clone(), literal!(")")])};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quant, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr;
                    let mut initial_str: ArcStr;
                    let mut fixed_str: ArcStr;
                    let mut res_1: ArcStr;
                    let mut res: ArcStr;
                    let mut startOriginStr: ArcStr;
                    quantity = getOptionWithConcatStr(quant.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("quantity = "))?;
                    initial_str = getOptionWithConcatStr(initialExp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("start = "))?;
                    fixed_str = getOptionWithConcatStr(fixed.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("fixed = "))?;
                    startOriginStr = getStartOrigin(startOrigin.clone())?;
                    res_1 = Util::stringDelimitListNonEmptyElts(list![quantity.clone(), initial_str.clone(), fixed_str.clone(), startOriginStr.clone()], literal!(", "))?;
                    res = if (stringEmpty(&res_1)) {literal!("")} else {stringAppendList(list![literal!("("), res_1.clone(), literal!(")")])};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quant, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr;
                    let mut initial_str: ArcStr;
                    let mut fixed_str: ArcStr;
                    let mut res_1: ArcStr;
                    let mut res: ArcStr;
                    let mut startOriginStr: ArcStr;
                    quantity = getOptionWithConcatStr(quant.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("quantity = "))?;
                    initial_str = getOptionWithConcatStr(initialExp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("start = "))?;
                    fixed_str = getOptionWithConcatStr(fixed.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("fixed = "))?;
                    startOriginStr = getStartOrigin(startOrigin.clone())?;
                    res_1 = Util::stringDelimitListNonEmptyElts(list![quantity.clone(), initial_str.clone(), fixed_str.clone(), startOriginStr.clone()], literal!(", "))?;
                    res = if (stringEmpty(&res_1)) {literal!("")} else {stringAppendList(list![literal!("("), res_1.clone(), literal!(")")])};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quant, min, max, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr;
                    let mut min_str: ArcStr;
                    let mut max_str: ArcStr;
                    let mut initial_str: ArcStr;
                    let mut fixed_str: ArcStr;
                    let mut res_1: ArcStr;
                    let mut res: ArcStr;
                    let mut startOriginStr: ArcStr;
                    quantity = getOptionWithConcatStr(quant.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("quantity = "))?;
                    min_str = getOptionWithConcatStr(min.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("min = "))?;
                    max_str = getOptionWithConcatStr(max.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("max = "))?;
                    initial_str = getOptionWithConcatStr(initialExp.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("start = "))?;
                    fixed_str = getOptionWithConcatStr(fixed.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>), literal!("fixed = "))?;
                    startOriginStr = getStartOrigin(startOrigin.clone())?;
                    res_1 = Util::stringDelimitListNonEmptyElts(list![quantity.clone(), min_str.clone(), max_str.clone(), initial_str.clone(), fixed_str.clone(), startOriginStr.clone()], literal!(", "))?;
                    res = if (stringEmpty(&res_1)) {literal!("")} else {stringAppendList(list![literal!("("), res_1.clone(), literal!(")")])};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("(unknown VariableAttributes)"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn getStartOrigin(mut inStartOrigin: Option<DAE::StartOrigin>) -> Result<ArcStr> {
    let mut outStartOrigin: ArcStr;
    outStartOrigin = (match inStartOrigin {
        Some(mut so) if (Flags::isSet(Flags::SHOW_START_ORIGIN.clone())?) => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("startOrigin = "));
            __mm_s.push_str(&*startOriginStr(so));
            ArcStr::from(__mm_s)
        }
        _ => {
            literal!("")
        }
    });
    Ok(outStartOrigin)
}

pub(crate) fn startOriginStr(mut so: DAE::StartOrigin) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match so {
        DAE::StartOrigin::UNDEFINED_ORIGIN { .. } => literal!("undefined"),
        DAE::StartOrigin::TYPE_ORIGIN { .. } => literal!("type"),
        DAE::StartOrigin::BINDING_ORIGIN { .. } => literal!("binding"),
        DAE::StartOrigin::CONFIDENCE { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("confidence("));
            __mm_s.push_str(&*intString(var_field!(so.actual, DAE::StartOrigin::CONFIDENCE).clone()));
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*intString(var_field!(so.raw, DAE::StartOrigin::CONFIDENCE).clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::StartOrigin::TYPE_CONFIDENCE { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("typeConfidence("));
            __mm_s.push_str(&*intString(
                var_field!(so.level, DAE::StartOrigin::TYPE_CONFIDENCE).clone(),
            ));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    r#str
}

fn dumpVarVisibilityStr(mut prot: DAE::VarVisibility) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match prot {
        DAE::VarVisibility::PUBLIC { .. } => literal!(""),
        DAE::VarVisibility::PROTECTED { .. } => literal!("protected "),
    });
    r#str
}

pub fn dumpVarParallelismStr(mut inVarParallelism: DAE::VarParallelism) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarParallelism {
        DAE::VarParallelism::NON_PARALLEL { .. } => literal!(""),
        DAE::VarParallelism::PARGLOBAL { .. } => literal!("parglobal "),
        DAE::VarParallelism::PARLOCAL { .. } => literal!("parlocal "),
    });
    outString
}

fn dumpCommentOption(mut comment: Option<metamodelica::Ref<SCode::Comment>>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = dumpCommentAnnotationStr(comment);
    Print::printBuf(r#str)?;
    Ok(())
}

fn dumpEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: src } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: src } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("  "))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr1))?;
                    Print::printBuf(literal!(" = "))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr2))?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, source: src, .. } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: src } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: c, exp: e, source: src } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("  "))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&c))?;
                    Print::printBuf(literal!(" ::= "))?;
                    ExpressionDump::printExp(e.clone())?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: e1, message: e2, source: src, .. } => {
                    let mut sourceStr: ArcStr;
                    Print::printBuf(literal!("assert("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(") "))?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { exp: e1, source: src } => {
                    let mut sourceStr: ArcStr;
                    ExpressionDump::printExp(e1.clone())?;
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    Print::printBuf(sourceStr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
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
                    Print::printBuf(literal!("/* FIXME: UNHANDLED_EQUATION in DAEDump.dumpEquation */;\n"))?;
                    Ok(())
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

fn dumpInitialEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> () {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, .. } => {
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { componentRef: c, exp: e, .. } => {
                    Print::printBuf(literal!("  "))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&c))?;
                    Print::printBuf(literal!(" ::= "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { exp: e1, array: e2, .. } => {
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
                    Print::printBuf(literal!("  "))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(" = "))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: e, tail: conds }, equations2: Deref @ metamodelica::ListNode::Cons { head: xs1, tail: trueBranches }, equations3: xs2, .. } => {
                    let mut s: ArcStr;
                    let mut r#str: IOStream::IOStream;
                    Print::printBuf(literal!("  if "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(" then\n"))?;
                    List::map_0(metamodelica::AsArg::as_arg(&xs1), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpInitialEquation(&__a0)) })?;
                    r#str = dumpIfEquationsStream(conds.clone(), trueBranches.clone(), IOStream::emptyStreamOfTypeList.clone())?;
                    s = IOStream::string(&r#str)?;
                    Print::printBuf(s.clone())?;
                    Print::printBuf(literal!("  else\n"))?;
                    List::map_0(metamodelica::AsArg::as_arg(&xs2), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpInitialEquation(&__a0)) })?;
                    Print::printBuf(literal!("end if;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, source: src, .. } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    s = stringAppendList(list![literal!("  assert("), s1.clone(), literal!(","), s2.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source: src } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s = stringAppendList(list![literal!("  terminate("), s1.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Print::printBuf(s.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_NORETCALL { exp: e1, .. } => {
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(";\n"))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub fn dumpEquationStr(mut inElement: &metamodelica::Ref<DAE::Element>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr1))?;
                    s2 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr2))?;
                    r#str = stringAppendList(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: c, exp: e, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    s2 = stringAppend(literal!("  "), s1.clone());
                    s3 = stringAppend(literal!(" ::= "), s2.clone());
                    s4 = ExpressionBasics::printExpStr(e.clone())?;
                    s5 = stringAppend(s3.clone(), s4.clone());
                    r#str = stringAppend(s5.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) });
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: e1, message: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("  assert("), s1.clone(), literal!(","), s2.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { message: e1, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("  terminate("), s1.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { exp: e1, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("  "), s1.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("#UNKNOWN_EQUATION#"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn dumpAlgorithm(mut inElement: &metamodelica::Ref<DAE::Element>) -> () {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    Print::printBuf(literal!("algorithm\n"))?;
                    Dump::printList(metamodelica::AsArg::as_arg(&stmts), (std::sync::Arc::new(ppStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Statement>) -> Result<()> + 'static>), literal!(""))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn dumpInitialAlgorithm(mut inElement: &metamodelica::Ref<DAE::Element>) -> () {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    Print::printBuf(literal!("initial algorithm\n"))?;
                    Dump::printList(metamodelica::AsArg::as_arg(&stmts), (std::sync::Arc::new(ppStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Statement>) -> Result<()> + 'static>), literal!(""))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn dumpExtObjectClass(mut inElement: &metamodelica::Ref<DAE::Element>) -> () {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { path: fpath, .. } => {
                    let mut fstr: ArcStr;
                    Print::printBuf(literal!("class "))?;
                    fstr = AbsynUtil::pathString(fpath.clone(), literal!("."), true, false)?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(literal!("\n extends ExternalObject;\n"))?;
                    Print::printBuf(literal!("end "))?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(literal!(";\n"))?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn derivativeCondStr(mut dc: &DAE::derivativeCond) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match dc.clone() {
        DAE::derivativeCond::NO_DERIVATIVE { binding: ref e } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("noDerivative("));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        DAE::derivativeCond::ZERO_DERIVATIVE { .. } => {
            literal!("zeroDerivative")
        }
    });
    Ok(r#str)
}

fn dumpFunction(mut inElement: &DAE::Function) -> () {
    let () = 'mc: {
        let __mc_input = inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: fpath, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, type_: t, isImpure, comment: c, .. } => {
                    let mut fstr: ArcStr;
                    let mut parallelism_str: ArcStr;
                    let mut impureStr: ArcStr;
                    let mut typeStr: ArcStr;
                    typeStr = TypesDump::printTypeStr(t.clone());
                    Print::printBuf(typeStr.clone())?;
                    parallelism_str = dumpParallelismStr(metamodelica::AsArg::as_arg(&t));
                    Print::printBuf(parallelism_str.clone())?;
                    impureStr = if (isImpure.clone()) {literal!("impure ")} else {literal!("")};
                    Print::printBuf(impureStr.clone())?;
                    Print::printBuf(literal!("function "))?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(dumpCommentStr(c.clone()))?;
                    Print::printBuf(literal!("\n"))?;
                    dumpFunctionElements(daeElts.clone())?;
                    Print::printBuf(dumpClassAnnotationStr(c.clone()))?;
                    Print::printBuf(literal!("end "))?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(literal!(";\n\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: fpath, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { body: daeElts, externalDecl: ext_decl }, tail: _ }, isImpure, comment: c, .. } => {
                    let mut fstr: ArcStr;
                    let mut ext_decl_str: ArcStr;
                    let mut impureStr: ArcStr;
                    impureStr = if (isImpure.clone()) {literal!("impure ")} else {literal!("")};
                    Print::printBuf(impureStr.clone())?;
                    Print::printBuf(literal!("function "))?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(dumpCommentStr(c.clone()))?;
                    Print::printBuf(literal!("\n"))?;
                    dumpFunctionElements(daeElts.clone())?;
                    ext_decl_str = dumpExtDeclStr(metamodelica::AsArg::as_arg(&ext_decl))?;
                    Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*ext_decl_str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Print::printBuf(dumpClassAnnotationStr(c.clone()))?;
                    Print::printBuf(literal!("end "))?;
                    Print::printBuf(fstr.clone())?;
                    Print::printBuf(literal!(";\n\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::RECORD_CONSTRUCTOR { path: fpath, type_: t, .. } => {
                    let mut fstr: ArcStr;
                    let false = (Flags::isSet(Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::PRINT_RECORD_TYPES.clone())? {
                        Print::printBuf(TypesDump::unparseType(t.clone())?)?;
                        Print::printBuf(literal!("\n"))?;
                    } else {
                        Print::printBuf(literal!("function "))?;
                        fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                        Print::printBuf(fstr.clone())?;
                        Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"Automatically generated record constructor for ")); __mm_s.push_str(&*fstr); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) })?;
                        Print::printBuf(printRecordConstructorInputsStr(metamodelica::AsArg::as_arg(&t))?)?;
                        Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  output ")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&fpath))); __mm_s.push_str(&*literal!(" res;\n")); ArcStr::from(__mm_s) })?;
                        Print::printBuf(literal!("end "))?;
                        Print::printBuf(fstr.clone())?;
                        Print::printBuf(literal!(";\n\n"))?;
                    }
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn dumpParallelismStr(mut inType: &metamodelica::Ref<DAE::Type>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inType {
        DAE::Type::T_FUNCTION {
            funcArg: _,
            funcResultType: _,
            functionAttributes:
                DAE::FunctionAttributes {
                    functionParallelism: DAE::FunctionParallelism::FP_NON_PARALLEL { .. },
                    ..
                },
            path: _,
        } => literal!(""),
        DAE::Type::T_FUNCTION {
            funcArg: _,
            funcResultType: _,
            functionAttributes:
                DAE::FunctionAttributes {
                    functionParallelism: DAE::FunctionParallelism::FP_PARALLEL_FUNCTION { .. },
                    ..
                },
            path: _,
        } => literal!("parallel "),
        DAE::Type::T_FUNCTION {
            funcArg: _,
            funcResultType: _,
            functionAttributes:
                DAE::FunctionAttributes {
                    functionParallelism: DAE::FunctionParallelism::FP_KERNEL_FUNCTION { .. },
                    ..
                },
            path: _,
        } => literal!("kernel "),
        _ => literal!("#dumpParallelismStr failed#"),
    });
    outString
}

pub fn dumpInlineTypeStr(mut inlineType: DAE::InlineType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inlineType {
        DAE::InlineType::NO_INLINE { .. } => literal!("\"Inline never\""),
        DAE::InlineType::AFTER_INDEX_RED_INLINE { .. } => literal!(" \"Inline after index reduction\""),
        DAE::InlineType::NORM_INLINE { .. } => literal!(" \"Inline before index reduction\""),
        DAE::InlineType::DEFAULT_INLINE { .. } => literal!("\"Inline if necessary\""),
        DAE::InlineType::EARLY_INLINE { .. } => literal!("\"Inline earier than normal inline\""),
        DAE::InlineType::BUILTIN_EARLY_INLINE { .. } => literal!("\"Inline even if inlining is disabled\""),
        _ => literal!("\"unknown\""),
    });
    r#str
}

pub fn dumpInlineTypeBackendStr(mut inlineType: DAE::InlineType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match inlineType {
        DAE::InlineType::NO_INLINE { .. } => literal!("NONE"),
        DAE::InlineType::AFTER_INDEX_RED_INLINE { .. } => literal!("AFTER_INDEX_RED"),
        DAE::InlineType::NORM_INLINE { .. } => literal!("NORMAL"),
        DAE::InlineType::DEFAULT_INLINE { .. } => literal!("DEFAULT"),
        DAE::InlineType::EARLY_INLINE { .. } => literal!("EARLY"),
        DAE::InlineType::BUILTIN_EARLY_INLINE { .. } => literal!("BUILTIN_EARLY"),
        _ => literal!("UNKNOWN"),
    });
    r#str
}

fn printRecordConstructorInputsStr<'__b>(mut itp: &'__b metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    '__tco: loop {
        match &**itp {
            DAE::Type::T_COMPLEX { varLst: vars, .. } => {
                let mut var_strl: metamodelica::List<ArcStr>;
                var_strl = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| {
                    printRecordConstructorInputStr(&__a0)
                })?;
                return Ok(stringAppendList(var_strl));
            }
            DAE::Type::T_FUNCTION { funcResultType: tp, .. } => {
                itp = tp;
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn printRecordConstructorInputStr(mut inVar: &metamodelica::Ref<DAE::Var>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut name: ArcStr;
    let mut attr_str: ArcStr;
    let mut binding_str: ArcStr;
    let mut ty_str: ArcStr;
    let mut ty_vars_str: ArcStr;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    let __arc4 = &(*inVar);
    let DAE::TYPES_VAR {
        name: __pa0,
        attributes: __pa1,
        ty: __pa2,
        binding: __pa3,
        ..
    } = &**__arc4;
    name = metamodelica::Own::own(__pa0);
    attr = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    binding = metamodelica::Own::own(__pa3);
    attr_str = printRecordConstructorInputAttrStr(&attr);
    binding_str = printRecordConstructorBinding(&binding)?;
    (ty_str, ty_vars_str) = printTypeStr(&ty)?;
    outString = stringAppendList(list![
        literal!("  "),
        attr_str,
        ty_str,
        literal!(" "),
        name,
        ty_vars_str,
        binding_str,
        literal!(";\n")
    ]);
    Ok(outString)
}

fn printRecordConstructorInputAttrStr(mut inAttributes: &metamodelica::Ref<DAE::Attributes>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inAttributes {
        DAE::Attributes {
            visibility: SCode::Visibility::PROTECTED { .. },
            ..
        } => literal!("protected "),
        DAE::Attributes {
            variability: SCode::Variability::CONST { .. },
            ..
        } => literal!("constant "),
        _ => literal!("input "),
    });
    outString
}

fn printRecordConstructorBinding(mut binding: &metamodelica::Ref<DAE::Binding>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**binding {
        DAE::Binding::UNBOUND { .. } => {
            literal!("")
        }
        DAE::Binding::EQBOUND {
            exp: e,
            source: DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE { .. },
            ..
        } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        DAE::Binding::EQBOUND {
            exp: e,
            source: DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS { .. },
            ..
        } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        DAE::Binding::VALBOUND {
            valBound: v,
            source: DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE { .. },
        } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ValuesDump::valString(v)?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

fn ppStatement(mut alg: metamodelica::Ref<DAE::Statement>) -> Result<()> {
    ppStmt(alg, 2)?;
    Ok(())
}

pub fn ppStatementStr(mut alg: metamodelica::Ref<DAE::Statement>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ppStmtStr(alg, 2);
    r#str
}

fn ppStmt(mut inStatement: metamodelica::Ref<DAE::Statement>, mut inInteger: i32) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inStatement, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp1: e2, exp: e, source, .. }, i) => {
                    indent(i.clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(" := "))?;
                    ExpressionDump::printExp(e.clone())?;
                    if Config::typeinfo()? {
                        Print::printBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /* ")); __mm_s.push_str(&*Error::infoStr(&(ElementSource::getElementSourceFileInfo(source.clone())))?); __mm_s.push_str(&*literal!(" */")); ArcStr::from(__mm_s) })?;
                    }
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: e2, exp: e, .. }, i) => {
                    indent(i.clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(" := "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(";\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expl, exp: e, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut es: metamodelica::List<ArcStr>;
                    s1 = indentStr(i.clone());
                    s2 = ExpressionBasics::printExpStr(e.clone())?;
                    es = List::map(expl.clone(), &ExpressionBasics::printExpStr)?;
                    s3 = stringDelimitList(es.clone(), literal!(", "));
                    r#str = stringAppendList(list![s1.clone(), literal!("("), s3.clone(), literal!(") := "), s2.clone(), literal!(";\n")]);
                    Print::printBuf(r#str.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: then_, else_, .. }, i) => {
                    let mut i_1: i32;
                    indent(i.clone())?;
                    Print::printBuf(literal!("if "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(" then\n"))?;
                    i_1 = i.clone() + 2;
                    ppStmtList(metamodelica::AsArg::as_arg(&then_), i_1)?;
                    ppElse(metamodelica::AsArg::as_arg(&else_), i.clone())?;
                    indent(i.clone())?;
                    Print::printBuf(literal!("end if;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { iter: id, range: e, statementLst: stmts, .. }, i) => {
                    let mut i_1: i32;
                    indent(i.clone())?;
                    Print::printBuf(literal!("for "))?;
                    Print::printBuf(id.clone())?;
                    Print::printBuf(literal!(" in "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(" loop\n"))?;
                    i_1 = i.clone() + 2;
                    ppStmtList(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    indent(i.clone())?;
                    Print::printBuf(literal!("end for;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { iter: id, range: e, statementLst: stmts, .. }, i) => {
                    let mut i_1: i32;
                    indent(i.clone())?;
                    Print::printBuf(literal!("parfor "))?;
                    Print::printBuf(id.clone())?;
                    Print::printBuf(literal!(" in "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(" loop\n"))?;
                    i_1 = i.clone() + 2;
                    ppStmtList(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    indent(i.clone())?;
                    Print::printBuf(literal!("end parfor;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, .. }, i) => {
                    let mut i_1: i32;
                    indent(i.clone())?;
                    Print::printBuf(literal!("while "))?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf(literal!(" loop\n"))?;
                    i_1 = i.clone() + 2;
                    ppStmtList(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    indent(i.clone())?;
                    Print::printBuf(literal!("end while;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Statement::STMT_NORETCALL { exp: e1, .. }, i) => {
                            indent(i.clone())?;
                            let () = (::match_deref::match_deref! { match &(e1.clone()) {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => {
                            Print::printBuf(literal!("return "))?;
                            ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            ExpressionDump::printExp(e1.clone())?;
                            Print::printBuf(literal!(";\n"))?;
                            Ok(())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(ppWhenStmtStr(metamodelica::AsArg::as_arg(&stmt), 1)?)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { cond, msg, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("assert("))?;
                    ExpressionDump::printExp(cond.clone())?;
                    Print::printBuf(literal!(", "))?;
                    ExpressionDump::printExp(msg.clone())?;
                    Print::printBuf(literal!(");\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_RETURN { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("return;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_BREAK { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("break;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { var: e1, value: e2, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("reinit("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(");\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("begin failure\n"))?;
                    ppStmtList(metamodelica::AsArg::as_arg(&stmts), i.clone() + 2)?;
                    Print::printBuf(literal!("end try;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ARRAY_INIT { name, ty, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("/* "))?;
                    Print::printBuf(name.clone())?;
                    Print::printBuf(literal!(" := array_alloc("))?;
                    Print::printBuf(TypesDump::unparseType(ty.clone())?)?;
                    Print::printBuf(literal!(") */;\n"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, i) => {
                    indent(i.clone())?;
                    Print::printBuf(literal!("**ALGORITHM**;\n"))?;
                    Ok(())
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

fn ppWhenStmtStr(mut inStatement: &metamodelica::Ref<DAE::Statement>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inStatement {
        Deref @ DAE::Statement::STMT_WHEN { exp: e, statementLst: stmts, elseWhen: None, .. } => {
            let mut i = inInteger;
            let mut s3: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            let mut r#str: ArcStr;
            let mut s7: ArcStr;
            let mut s8: ArcStr;
            let mut s9: ArcStr;
            let mut i_1: i32;
            s3 = stringAppend(literal!("when "), ExpressionBasics::printExpStr(e.clone())?);
            s5 = stringAppend(s3, literal!(" then\n"));
            i_1 = i + 2;
            s6 = ppStmtListStr(stmts, i_1)?;
            s7 = stringAppend(s5, s6);
            s8 = indentStr(i);
            s9 = stringAppend(s7, s8);
            r#str = stringAppend(s9, literal!("end when;\n"));
            r#str
        },
        Deref @ DAE::Statement::STMT_WHEN { exp: e, statementLst: stmts, elseWhen: Some(stmt), .. } => {
            let mut i = inInteger;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            let mut r#str: ArcStr;
            let mut s7: ArcStr;
            let mut s8: ArcStr;
            let mut s9: ArcStr;
            let mut s10: ArcStr;
            let mut i_1: i32;
            s3 = ExpressionBasics::printExpStr(e.clone())?;
            s4 = stringAppend(literal!("when "), s3);
            s5 = stringAppend(s4, literal!(" then\n"));
            i_1 = i + 2;
            s6 = ppStmtListStr(stmts, i_1)?;
            s7 = stringAppend(s5, s6);
            s8 = ppWhenStmtStr(metamodelica::AsArg::as_arg(&stmt), i)?;
            s9 = stringAppend(indentStr(i), literal!("else"));
            s10 = stringAppend(s7, s9);
            r#str = stringAppend(s10, s8);
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub fn ppStmtStr(mut inStatement: metamodelica::Ref<DAE::Statement>, mut inInteger: i32) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inStatement, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp1: e2, exp: e, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    r#str = stringAppendList(list![s1.clone(), s2.clone(), literal!(" := "), s3.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: e2, exp: e, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    r#str = stringAppendList(list![s1.clone(), s2.clone(), literal!(" := "), s3.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expl, exp: e, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut es: metamodelica::List<ArcStr>;
                    s1 = indentStr(i.clone());
                    s2 = ExpressionBasics::printExpStr(e.clone())?;
                    es = List::map(expl.clone(), &ExpressionBasics::printExpStr)?;
                    s3 = stringDelimitList(es.clone(), literal!(", "));
                    r#str = stringAppendList(list![s1.clone(), literal!("("), s3.clone(), literal!(") := "), s2.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: then_, else_, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut s6: ArcStr;
                    let mut r#str: ArcStr;
                    let mut s7: ArcStr;
                    let mut s8: ArcStr;
                    let mut s9: ArcStr;
                    let mut s10: ArcStr;
                    let mut s11: ArcStr;
                    let mut i_1: i32;
                    s1 = indentStr(i.clone());
                    s2 = stringAppend(s1.clone(), literal!("if "));
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    s4 = stringAppend(s2.clone(), s3.clone());
                    s5 = stringAppend(s4.clone(), literal!(" then\n"));
                    i_1 = i.clone() + 2;
                    s6 = ppStmtListStr(metamodelica::AsArg::as_arg(&then_), i_1)?;
                    s7 = stringAppend(s5.clone(), s6.clone());
                    s8 = ppElseStr(metamodelica::AsArg::as_arg(&else_), i.clone())?;
                    s9 = stringAppend(s7.clone(), s8.clone());
                    s10 = indentStr(i.clone());
                    s11 = stringAppend(s9.clone(), s10.clone());
                    r#str = stringAppend(s11.clone(), literal!("end if;\n"));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { iter: id, range: e, statementLst: stmts, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut r#str: ArcStr;
                    let mut i_1: i32;
                    s1 = indentStr(i.clone());
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    i_1 = i.clone() + 2;
                    s4 = ppStmtListStr(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    s5 = indentStr(i.clone());
                    r#str = stringAppendList(list![s1.clone(), literal!("for "), id.clone(), literal!(" in "), s3.clone(), literal!(" loop\n"), s4.clone(), s5.clone(), literal!("end for;\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { iter: id, range: e, statementLst: stmts, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut r#str: ArcStr;
                    let mut i_1: i32;
                    s1 = indentStr(i.clone());
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    i_1 = i.clone() + 2;
                    s4 = ppStmtListStr(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    s5 = indentStr(i.clone());
                    r#str = stringAppendList(list![s1.clone(), literal!("parfor "), id.clone(), literal!(" in "), s3.clone(), literal!(" loop\n"), s4.clone(), s5.clone(), literal!("end for;\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut s6: ArcStr;
                    let mut r#str: ArcStr;
                    let mut s7: ArcStr;
                    let mut s8: ArcStr;
                    let mut s9: ArcStr;
                    let mut i_1: i32;
                    s1 = indentStr(i.clone());
                    s2 = stringAppend(s1.clone(), literal!("while "));
                    s3 = ExpressionBasics::printExpStr(e.clone())?;
                    s4 = stringAppend(s2.clone(), s3.clone());
                    s5 = stringAppend(s4.clone(), literal!(" loop\n"));
                    i_1 = i.clone() + 2;
                    s6 = ppStmtListStr(metamodelica::AsArg::as_arg(&stmts), i_1)?;
                    s7 = stringAppend(s5.clone(), s6.clone());
                    s8 = indentStr(i.clone());
                    s9 = stringAppend(s7.clone(), s8.clone());
                    r#str = stringAppend(s9.clone(), literal!("end while;\n"));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    s2 = ppWhenStmtStr(metamodelica::AsArg::as_arg(&stmt), i.clone())?;
                    r#str = stringAppend(s1.clone(), s2.clone());
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { cond, msg, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cond_str: ArcStr;
                    let mut msg_str: ArcStr;
                    s1 = indentStr(i.clone());
                    cond_str = ExpressionBasics::printExpStr(cond.clone())?;
                    msg_str = ExpressionBasics::printExpStr(msg.clone())?;
                    r#str = stringAppendList(list![s1.clone(), literal!("assert("), cond_str.clone(), literal!(", "), msg_str.clone(), literal!(");\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { msg, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut msg_str: ArcStr;
                    s1 = indentStr(i.clone());
                    msg_str = ExpressionBasics::printExpStr(msg.clone())?;
                    r#str = stringAppendList(list![s1.clone(), literal!("terminate("), msg_str.clone(), literal!(");\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Statement::STMT_NORETCALL { exp: e, .. }, i) => {
                            let mut s1: ArcStr;
                            let mut s2: ArcStr;
                            let mut s3: ArcStr;
                            let mut r#str: ArcStr;
                            s1 = indentStr(i.clone());
                            s2 = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => literal!("return "),
                _ => literal!(""),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            s3 = ExpressionBasics::printExpStr(e.clone())?;
                            r#str = stringAppendList(list![s1.clone(), s2.clone(), s3.clone(), literal!(";\n")]);
                            Ok(r#str.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_RETURN { .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    r#str = stringAppend(s1.clone(), literal!("return;\n"));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_BREAK { .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    r#str = stringAppend(s1.clone(), literal!("break;\n"));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { var: e1, value: e2, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut e1_str: ArcStr;
                    let mut e2_str: ArcStr;
                    s1 = indentStr(i.clone());
                    e1_str = ExpressionBasics::printExpStr(e1.clone())?;
                    e2_str = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![s1.clone(), literal!("reinit("), e1_str.clone(), literal!(", "), e2_str.clone(), literal!(");\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    s2 = ppStmtListStr(metamodelica::AsArg::as_arg(&stmts), i.clone() + 2)?;
                    r#str = stringAppendList(list![s1.clone(), literal!("failure(\n"), s2.clone(), s1.clone(), literal!(");\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ARRAY_INIT { name: s2, .. }, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    r#str = stringAppendList(list![s1.clone(), literal!("arrayInit(\n"), s2.clone(), s1.clone(), literal!(");\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, i) => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = indentStr(i.clone());
                    r#str = stringAppend(s1.clone(), literal!("**ALGORITHM COULD NOT BE GENERATED(DAE.mo)**;\n"));
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn ppStmtList(
    mut inAlgorithmStatementLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inInteger: i32,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inAlgorithmStatementLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: stmt, tail: stmts } => {
            let mut i = inInteger;
            ppStmt(stmt.clone(), i)?;
            ppStmtList(stmts, i)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn ppStmtListStr(
    mut inAlgorithmStatementLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inAlgorithmStatementLst {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: stmt, tail: stmts } => {
            let mut i = inInteger;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut r#str: ArcStr;
            s1 = ppStmtStr(stmt.clone(), i);
            s2 = ppStmtListStr(stmts, i)?;
            r#str = stringAppend(s1, s2);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn ppElse(mut inElse: &metamodelica::Ref<DAE::Else>, mut inInteger: i32) -> Result<()> {
    let () = (match &**inElse {
        DAE::Else::NOELSE { .. } => (),
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: then_,
            else_,
        } => {
            let mut i = inInteger;
            let mut i_1: i32;
            indent(i)?;
            Print::printBuf(literal!("elseif "))?;
            ExpressionDump::printExp(e.clone())?;
            Print::printBuf(literal!(" then\n"))?;
            i_1 = i + 2;
            ppStmtList(then_, i_1)?;
            ppElse(else_, i)?;
            ()
        }
        DAE::Else::ELSE { statementLst: stmts } => {
            let mut i = inInteger;
            let mut i_1: i32;
            indent(i)?;
            Print::printBuf(literal!("else\n"))?;
            i_1 = i + 2;
            ppStmtList(stmts, i_1)?;
            ()
        }
    });
    Ok(())
}

fn ppElseStr(mut inElse: &metamodelica::Ref<DAE::Else>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inElse {
        DAE::Else::NOELSE { .. } => {
            literal!("")
        }
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: then_,
            else_,
        } => {
            let mut i = inInteger;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            let mut s7: ArcStr;
            let mut s8: ArcStr;
            let mut r#str: ArcStr;
            let mut i_1: i32;
            s1 = indentStr(i);
            s2 = stringAppend(s1, literal!("elseif "));
            s3 = ExpressionBasics::printExpStr(e.clone())?;
            s4 = stringAppend(s2, s3);
            s5 = stringAppend(s4, literal!(" then\n"));
            i_1 = i + 2;
            s6 = ppStmtListStr(then_, i_1)?;
            s7 = stringAppend(s5, s6);
            s8 = ppElseStr(else_, i)?;
            r#str = stringAppend(s7, s8);
            r#str
        }
        DAE::Else::ELSE { statementLst: stmts } => {
            let mut i = inInteger;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut r#str: ArcStr;
            let mut i_1: i32;
            s1 = indentStr(i);
            s2 = stringAppend(s1, literal!("else\n"));
            i_1 = i + 2;
            s3 = ppStmtListStr(stmts, i_1)?;
            r#str = stringAppend(s2, s3);
            r#str
        }
    });
    Ok(outString)
}

fn indent(mut inInteger: i32) -> Result<()> {
    let () = (match inInteger {
        0 => (),
        mut i => {
            let mut i_1: i32;
            Print::printBuf(literal!(" "))?;
            i_1 = i - 1;
            indent(i_1)?;
            ()
        }
    });
    Ok(())
}

fn indentStr(mut inInteger: i32) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inInteger {
        0 => {
            literal!("")
        }
        mut i => {
            let mut i_1: i32;
            let mut s1: ArcStr;
            let mut r#str: ArcStr;
            i_1 = i - 1;
            s1 = indentStr(i_1);
            r#str = stringAppend(literal!(" "), s1);
            r#str
        }
    });
    outString
}

pub(crate) fn dumpDebug(mut inDAElist: &DAE::DAElist) -> Result<()> {
    let () = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref elist } => {
            Print::printBuf(literal!("DAE("))?;
            dumpDebugElist(metamodelica::AsArg::as_arg(&elist))?;
            Print::printBuf(literal!(")"))?;
            ()
        }
    });
    Ok(())
}

fn dumpDebugElist(mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inElementLst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: first, tail: rest } => {
            dumpDebugElement(metamodelica::AsArg::as_arg(&first))?;
            Print::printBuf(literal!("\n"))?;
            dumpDebugElist(rest)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpDebugDAE(mut dae: &DAE::DAElist) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match dae.clone() {
        DAE::DAElist { elementLst: ref elems } => {
            Print::clearBuf();
            dumpDebugElist(metamodelica::AsArg::as_arg(&elems))?;
            r#str = Print::getString()?;
            r#str
        }
    });
    Ok(r#str)
}

pub(crate) fn dumpDebugElement(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, binding: None, variableAttributesOption: dae_var_attr, comment, .. } => {
                    let mut comment_str: ArcStr;
                    let mut tmp_str: ArcStr;
                    Print::printBuf(literal!("VAR("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr))?;
                    Print::printBuf(literal!(", "))?;
                    dumpKind(vk.clone())?;
                    comment_str = dumpCommentAnnotationStr(comment.clone());
                    Print::printBuf(literal!("  comment:"))?;
                    Print::printBuf(comment_str.clone())?;
                    tmp_str = dumpVariableAttributesStr(dae_var_attr.clone());
                    Print::printBuf(tmp_str.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, binding: Some(e), variableAttributesOption: dae_var_attr, comment, .. } => {
                    let mut comment_str: ArcStr;
                    let mut tmp_str: ArcStr;
                    Print::printBuf(literal!("VAR("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr))?;
                    Print::printBuf(literal!(", "))?;
                    dumpKind(vk.clone())?;
                    Print::printBuf(literal!(", binding: "))?;
                    ExpressionDump::printExp(e.clone())?;
                    comment_str = dumpCommentAnnotationStr(comment.clone());
                    Print::printBuf(literal!("  comment:"))?;
                    Print::printBuf(comment_str.clone())?;
                    tmp_str = dumpVariableAttributesStr(dae_var_attr.clone());
                    Print::printBuf(tmp_str.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: cr, exp, .. } => {
                    Print::printBuf(literal!("DEFINE("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr))?;
                    Print::printBuf(literal!(", "))?;
                    ExpressionDump::printExp(exp.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { componentRef: cr, exp, .. } => {
                    Print::printBuf(literal!("INITIALDEFINE("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr))?;
                    Print::printBuf(literal!(", "))?;
                    ExpressionDump::printExp(exp.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, .. } => {
                    Print::printBuf(literal!("EQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1, cr2, .. } => {
                    Print::printBuf(literal!("EQUATION("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr1))?;
                    Print::printBuf(literal!(","))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr2))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, .. } => {
                    Print::printBuf(literal!("INITIALEQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { .. } => {
                    Print::printBuf(literal!("ALGORITHM()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { .. } => {
                    Print::printBuf(literal!("INITIALALGORITHM()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { ident: n, dAElist: l, .. } => {
                    Print::printBuf(literal!("COMP("))?;
                    Print::printBuf(n.clone())?;
                    Print::printBuf(literal!(","))?;
                    dumpDebugElist(metamodelica::AsArg::as_arg(&l))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, .. } => {
                    Print::printBuf(literal!("ARRAY_EQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { exp: e1, array: e2, .. } => {
                    Print::printBuf(literal!("INITIAL_ARRAY_EQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
                    Print::printBuf(literal!("COMPLEX_EQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
                    Print::printBuf(literal!("INITIAL_COMPLEX_EQUATION("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { .. } => {
                    Print::printBuf(literal!("IF_EQUATION()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
                    Print::printBuf(literal!("INITIAL_IF_EQUATION()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { .. } => {
                    Print::printBuf(literal!("WHEN_EQUATION()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
                    Print::printBuf(literal!("EXTOBJECTCLASS()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: e1, message: e2, .. } => {
                    Print::printBuf(literal!("ASSERT("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, .. } => {
                    Print::printBuf(literal!("INITIAL_ASSERT("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(","))?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { message: e1, .. } => {
                    Print::printBuf(literal!("TERMINATE("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, .. } => {
                    Print::printBuf(literal!("INITIAL_TERMINATE("))?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::REINIT { .. } => {
                    Print::printBuf(literal!("REINIT()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { .. } => {
                    Print::printBuf(literal!("NORETCALL()"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::SM_COMP { componentRef: cr, dAElist: l } => {
                    Print::printBuf(literal!("SM_COMP("))?;
                    ComponentReference::printComponentRef(metamodelica::AsArg::as_arg(&cr))?;
                    Print::printBuf(literal!(","))?;
                    dumpDebugElist(metamodelica::AsArg::as_arg(&l))?;
                    Print::printBuf(literal!(")"))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::FLAT_SM { ident: n, dAElist: l } => {
                    Print::printBuf(literal!("FLAT_SM("))?;
                    Print::printBuf(n.clone())?;
                    Print::printBuf(literal!(","))?;
                    dumpDebugElist(metamodelica::AsArg::as_arg(&l))?;
                    Print::printBuf(literal!(")"))?;
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
                    Print::printBuf(literal!("UNKNOWN "))?;
                    Ok(())
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

pub(crate) fn dumpFlow(mut var: &metamodelica::Ref<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut flowString: ArcStr;
    flowString = (match &**var {
        DAE::ConnectorType::FLOW { .. } => literal!("flow"),
        DAE::ConnectorType::POTENTIAL { .. } => literal!("effort"),
        DAE::ConnectorType::NON_CONNECTOR { .. } => literal!("non_connector"),
        _ => return Err("match: no arm matched"),
    });
    Ok(flowString)
}

pub fn dumpConnectorType(mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inConnectorType {
        DAE::ConnectorType::FLOW { .. } => literal!("flow"),
        DAE::ConnectorType::STREAM { .. } => literal!("stream"),
        _ => literal!(""),
    });
    outString
}

pub(crate) fn dumpGraphviz(mut dae: &DAE::DAElist) -> Result<()> {
    let mut r: metamodelica::Ref<Graphviz::Node>;
    r = buildGraphviz(dae)?;
    Graphviz::dump(&r)?;
    Ok(())
}

fn buildGraphviz(mut inDAElist: &DAE::DAElist) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref els } => {
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut nonvars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut nonvarnodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            let mut varnodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            let mut nodelist: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            vars = DAEUtil::getMatchingElements(
                els.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::isVar(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            nonvars = DAEUtil::getMatchingElements(
                els.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::isNotVar(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            nonvarnodes = buildGrList(&nonvars)?;
            varnodes = buildGrVars(vars)?;
            nodelist = listAppend(nonvarnodes, varnodes);
            metamodelica::Ref::new(Graphviz::Node::NODE {
                type_: literal!("DAE"),
                attributes: metamodelica::nil(),
                children: nodelist,
            })
        }
    });
    Ok(outNode)
}

fn buildGrList(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outGraphvizNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outGraphvizNodeLst = (::match_deref::match_deref! { match inElementLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: el, tail: rest } => {
            let mut node: metamodelica::Ref<Graphviz::Node>;
            let mut nodelist: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            node = buildGrElement(metamodelica::AsArg::as_arg(&el))?;
            nodelist = buildGrList(rest)?;
            metamodelica::cons(node, nodelist)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraphvizNodeLst)
}

fn buildGrVars(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<Graphviz::Node>>> {
    let mut outGraphvizNodeLst: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
    outGraphvizNodeLst = (::match_deref::match_deref! { match &(inElementLst) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        vars => {
            let mut strlist: metamodelica::List<ArcStr>;
            (strlist, _) = buildGrStrlist(vars.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| buildGrVarStr(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<ArcStr> + 'static>), 10)?;
            list![metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("VARS"), labelLst: strlist, attributes: list![Graphviz::r#box.clone()], children: metamodelica::nil() })]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraphvizNodeLst)
}

pub(crate) fn buildGrStrlist<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: metamodelica::List<Type_a>,
    mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>,
    mut inInteger: i32,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<Type_a>)> {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outStringLst: metamodelica::List<ArcStr>;
    let mut outTypeALst: metamodelica::List<Type_a>;
    (outStringLst, outTypeALst) = (::match_deref::match_deref! { match &((inTypeALst, inInteger)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (ignored, count) if (count.clone() <= 0) => {
            (list![literal!("...")], ignored.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: var, tail: rest }, count) if (count.clone() > 0) => {
            let mut printer = inFuncTypeTypeAToString.clone();
            let mut ignored: metamodelica::List<Type_a>;
            let mut count_1: i32;
            let mut strlist: metamodelica::List<ArcStr>;
            let mut r#str: ArcStr;
            count_1 = count.clone() - 1;
            (strlist, ignored) = buildGrStrlist(rest.clone(), printer.clone(), count_1)?;
            r#str = printer(var.clone())?;
            (metamodelica::cons(r#str, strlist), ignored)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outStringLst, outTypeALst))
}

fn buildGrVarStr(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::VAR { componentRef: cr, binding: None, .. } => {
            let mut r#str: ArcStr;
            r#str = ComponentReferenceBasics::printComponentRefStr(cr)?;
            r#str
        },
        Deref @ DAE::Element::VAR { componentRef: cr, binding: Some(exp), .. } => {
            let mut r#str: ArcStr;
            let mut expstr: ArcStr;
            let mut str_1: ArcStr;
            let mut str_2: ArcStr;
            r#str = ComponentReferenceBasics::printComponentRefStr(cr)?;
            expstr = printExpStrSpecial(exp.clone())?;
            str_1 = stringAppend(r#str, literal!(" = "));
            str_2 = stringAppend(str_1, expstr);
            str_2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn printExpStrSpecial(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::SCONST { string: s } => {
            let mut s_1: ArcStr;
            let mut s_2: ArcStr;
            s_1 = stringAppend(literal!("\\\""), s.clone());
            s_2 = stringAppend(s_1, literal!("\\\""));
            s_2
        },
        exp => {
            let mut r#str: ArcStr;
            r#str = ExpressionBasics::printExpStr(exp.clone())?;
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn buildGrElement(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<Graphviz::Node>> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, binding: None, .. } => {
            let mut crstr: ArcStr;
            let mut vkstr: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(cr)?;
            vkstr = dumpKindStr(vk.clone());
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("VAR"), labelLst: list![crstr, vkstr], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::VAR { componentRef: cr, kind: vk, binding: Some(exp), .. } => {
            let mut crstr: ArcStr;
            let mut vkstr: ArcStr;
            let mut expstr: ArcStr;
            let mut expstr_1: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(cr)?;
            vkstr = dumpKindStr(vk.clone());
            expstr = printExpStrSpecial(exp.clone())?;
            expstr_1 = stringAppend(literal!("= "), expstr);
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("VAR"), labelLst: list![crstr, vkstr, expstr_1], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr, exp, .. } => {
            let mut crstr: ArcStr;
            let mut expstr: ArcStr;
            let mut expstr_1: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(cr)?;
            expstr = printExpStrSpecial(exp.clone())?;
            expstr_1 = stringAppend(literal!("= "), expstr);
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("DEFINE"), labelLst: list![crstr, expstr_1], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut e1str: ArcStr;
            let mut e2str: ArcStr;
            e1str = printExpStrSpecial(e1.clone())?;
            e2str = printExpStrSpecial(e2.clone())?;
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQUATION"), labelLst: list![e1str, literal!("="), e2str], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, .. } => {
            let mut e1str: ArcStr;
            let mut e2str: ArcStr;
            e1str = printExpStrSpecial(Expression::crefExp(cr1.clone())?)?;
            e2str = printExpStrSpecial(Expression::crefExp(cr2.clone())?)?;
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("EQUEQUATION"), labelLst: list![e1str, literal!("="), e2str], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("ALGORITHM"), attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: cr, exp, .. } => {
            let mut crstr: ArcStr;
            let mut expstr: ArcStr;
            let mut expstr_1: ArcStr;
            crstr = ComponentReferenceBasics::printComponentRefStr(cr)?;
            expstr = printExpStrSpecial(exp.clone())?;
            expstr_1 = stringAppend(literal!("= "), expstr);
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("INITIALDEFINE"), labelLst: list![crstr, expstr_1], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, .. } => {
            let mut e1str: ArcStr;
            let mut e2str: ArcStr;
            e1str = printExpStrSpecial(e1.clone())?;
            e2str = printExpStrSpecial(e2.clone())?;
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("INITIALEQUATION"), labelLst: list![e1str, literal!("="), e2str], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("INITIALALGORITHM"), attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::COMP { ident: n, dAElist: elts, .. } => {
            let mut nodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
            nodes = buildGrList(elts)?;
            metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("COMP"), labelLst: list![n.clone()], attributes: metamodelica::nil(), children: nodes })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNode)
}

fn unparseType(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &**tp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, .. } => {
                    let mut name: ArcStr;
                    name = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                    Ok(name.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    let mut name: ArcStr;
                    let mut dim_str: ArcStr;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&ty))) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    dims = TypesDump::getDimensions(tp);
                    name = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                    dim_str = List::toStringCustom(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| ExpressionBasics::dimensionString(&__a0), literal!(""), literal!("["), literal!(", "), literal!("]"), false, 0)?;
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*dim_str); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty @ Deref @ DAE::Type::T_SUBTYPE_BASIC { .. }, .. } => {
                    Ok(unparseType(metamodelica::AsArg::as_arg(&ty))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp, .. } => {
                    Ok(TypesDump::unparseType(bc_tp.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(TypesDump::unparseType(tp.clone())?)
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

pub(crate) fn unparseDimensions(
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut printTypeDimension: bool,
) -> Result<ArcStr> {
    let mut dimsStr: ArcStr;
    dimsStr = (::match_deref::match_deref! { match &((dims.clone(), printTypeDimension)) {
        (_, false) => {
            literal!("")
        },
        (Deref @ metamodelica::ListNode::Nil, true) => {
            literal!("")
        },
        (_, true) => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(dims, &move |__a0: metamodelica::Ref<DAE::Dimension>| ExpressionBasics::dimensionString(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(dimsStr)
}

pub fn dumpStr(
    mut inDAElist: DAE::DAElist,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut daelist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut funList: DAEDumpTypes::functionList;
    let mut fixedDae: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>;
    let DAE::DAE { elementLst: __pa0 } = inDAElist;
    daelist = metamodelica::Own::own(__pa0);
    funList = dumpFunctionList(functionTree)?;
    fixedDae = List::map(daelist, &move |__a0: metamodelica::Ref<DAE::Element>| {
        DAEUtil::splitComponent(&__a0)
    })?;
    outString = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text,
                  __a1: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>,
                  __a2: DAEDumpTypes::functionList| DAEDumpTpl::dumpDAE(__a0, &__a1, &__a2),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Tpl::Text,
                        metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>,
                        DAEDumpTypes::functionList,
                    ) -> Result<Tpl::Text>
                    + 'static,
            >),
        fixedDae,
        funList,
    )?;
    Ok(outString)
}

pub fn dumpElementsStr(mut els: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match els {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr;
            myStream = IOStream::create(literal!("dae"), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpElementsStream(els, myStream)?;
            r#str = IOStream::string(&myStream)?;
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn dumpAlgorithmsStr(mut algs: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match algs {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr;
            myStream = IOStream::create(literal!("algs"), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpAlgorithmsStream(algs, myStream)?;
            r#str = IOStream::string(&myStream)?;
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn dumpConstraintsStr(mut constrs: &metamodelica::List<metamodelica::Ref<DAE::Element>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match constrs {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr;
            myStream = IOStream::create(literal!("constrs"), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpConstraintStream(constrs, myStream)?;
            r#str = IOStream::string(&myStream)?;
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
pub(crate) fn dumpStream(
    mut dae: &DAE::DAElist,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (match (dae.clone(), inStream) {
        (
            DAE::DAElist {
                elementLst: ref daelist,
            },
            mut r#str,
        ) => {
            let mut funcs: metamodelica::List<DAE::Function>;
            funcs = DAEUtil::getFunctionList(functionTree, false)?;
            funcs = sortFunctions(funcs)?;
            r#str = List::fold(
                &funcs,
                &move |__a0: DAE::Function, __a1: IOStream::IOStream| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dumpFunctionStream(&__a0, __a1))
                },
                r#str,
            )?;
            r#str = IOStream::appendList(
                r#str,
                &(List::map(
                    daelist.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(dumpExtObjClassStr(&__a0))
                    },
                )?),
            )?;
            r#str = List::fold(
                metamodelica::AsArg::as_arg(&daelist),
                &move |__a0: metamodelica::Ref<DAE::Element>, __a1: IOStream::IOStream| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dumpCompElementStream(&__a0, __a1))
                },
                r#str,
            )?;
            r#str
        }
    });
    Ok(outStream)
}

pub(crate) fn dumpFunctionList(
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<DAEDumpTypes::functionList> {
    let mut funList: DAEDumpTypes::functionList;
    funList = (match &**functionTree {
        _ => {
            let mut funcs: metamodelica::List<DAE::Function>;
            funcs = DAEUtil::getFunctionList(functionTree, false)?;
            funcs = List::filter2OnTrue(
                funcs,
                (std::sync::Arc::new(
                    move |__a0: DAE::Function, __a1: bool, __a2: bool| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isVisibleFunction(&__a0, __a1, __a2))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(DAE::Function, bool, bool) -> Result<bool> + 'static>),
                Flags::isSet(Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone())?,
                Flags::isSet(Flags::INLINE_FUNCTIONS.clone())?,
            )?;
            funcs = sortFunctions(funcs)?;
            funList = DAEDumpTypes::functionList { funcs: funcs };
            funList
        }
    });
    Ok(funList)
}

fn isVisibleFunction(mut inFunc: &DAE::Function, mut inHideRecordCons: bool, mut inInliningEnabled: bool) -> bool {
    let mut outIsVisible: bool;
    outIsVisible = (::match_deref::match_deref! { match &((inFunc, inHideRecordCons, inInliningEnabled)) {
        (DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", .. } }, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { inlineType: DAE::InlineType::BUILTIN_EARLY_INLINE { .. }, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { inlineType: DAE::InlineType::EARLY_INLINE { .. }, .. }, _, true) => {
            false
        },
        (DAE::Function::FUNCTION { comment: cmt, .. }, _, _) => {
            !(SCodeUtil::optCommentHasBooleanNamedAnnotation(cmt.clone(), &(literal!("__OpenModelica_builtin"))))
        },
        (DAE::Function::RECORD_CONSTRUCTOR { .. }, true, _) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsVisible
}

fn dumpCompElementStream(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inStream: IOStream::IOStream,
) -> IOStream::IOStream {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElement, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMP { ident: n, dAElist: l, comment: c, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("class "))?;
                    r#str = IOStream::append(r#str.clone(), n.clone())?;
                    r#str = IOStream::append(r#str.clone(), dumpCommentStr(c.clone()))?;
                    r#str = IOStream::append(r#str.clone(), literal!("\n"))?;
                    r#str = dumpElementsStream(metamodelica::AsArg::as_arg(&l), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), dumpClassAnnotationStr(c.clone()))?;
                    r#str = IOStream::append(r#str.clone(), literal!("end "))?;
                    r#str = IOStream::append(r#str.clone(), n.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!(";\n"))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStream
}

pub(crate) fn dumpElementsStream(
    mut l: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (match inStream {
        mut r#str => {
            let mut v: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut ie: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut ia: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut e: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut a: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut co: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut sm: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>;
            let mut comments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
            let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
            (v, ie, ia, e, a, _, co, _, sm, comments) = DAEUtil::splitElements(l)?;
            r#str = dumpCompWithSplitElementsStream(sm, r#str)?;
            r#str = dumpVarsStream(v, false, r#str)?;
            r#str = IOStream::append(
                r#str,
                if ((ie).is_empty()) {
                    literal!("")
                } else {
                    literal!("initial equation\n")
                },
            )?;
            r#str = dumpInitialEquationsStream(&ie, r#str)?;
            r#str = dumpInitialAlgorithmsStream(&ia, r#str)?;
            r#str = IOStream::append(
                r#str,
                if ((e).is_empty()) {
                    literal!("")
                } else {
                    literal!("equation\n")
                },
            )?;
            r#str = dumpEquationsStream(e, r#str)?;
            r#str = dumpAlgorithmsStream(&a, r#str)?;
            r#str = IOStream::append(
                r#str,
                if ((co).is_empty()) {
                    literal!("")
                } else {
                    literal!("constraint\n")
                },
            )?;
            r#str = dumpConstraintStream(&co, r#str)?;
            r#str = IOStream::append(
                r#str,
                stringAppendList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut cmt in (comments).into_iter().cloned() {
                            let __x = (::match_deref::match_deref! { match &(cmt.clone()) {
                                Deref @ SCode::Comment { annotation_: __esc_ann @ Some(_), .. } => {
                                    ann = (*__esc_ann).clone();
                                    SCodeDump::printCommentStr(&(metamodelica::Ref::new(SCode::Comment { annotation_: ann.clone(), comment: None })), SCodeDump::defaultOptions.clone())?
                                },
                                _ => literal!(""),
                                _ => unreachable!("match_deref! exhaustiveness placeholder"),
                            } });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                ),
            )?;
            r#str
        }
    });
    Ok(outStream)
}

pub(crate) fn dumpCompWithSplitElementsStream(
    mut inCompLst: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCompLst, inStream)) {
            (Deref @ metamodelica::ListNode::Nil, r#str) => {
                return Ok(r#str.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAEDumpTypes::compWithSplitElements { name, spltElems, comment }, tail: xs }, r#str) => {
                let mut cstr: ArcStr;
                let mut v: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut ie: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut ia: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut e: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut a: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut co: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut sm: metamodelica::List<metamodelica::Ref<DAEDumpTypes::compWithSplitElements>>;
                let mut r#str = (*r#str).clone();
                match '__try0: {
                    let __pa1 = ::match_deref::match_deref! { match &(comment.clone()) {
                        Some(Deref @ SCode::Comment { comment: Some(__pa1), .. }) => __pa1.clone(),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    cstr = metamodelica::Own::own(__pa1);
                    cstr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*cstr); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
                    Ok::<_, &'static str>((cstr.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        cstr = __try0_o0;
                    }
                    Err(_) => {
                        cstr = literal!("");
                    }
                }
                let __arc9 = spltElems.clone();
                let DAEDumpTypes::splitElements { v: __pa2, ie: __pa3, ia: __pa4, e: __pa5, a: __pa6, co: __pa7, o: _, ca: _, sm: __pa8 } = &*__arc9;
                v = metamodelica::Own::own(__pa2);
                ie = metamodelica::Own::own(__pa3);
                ia = metamodelica::Own::own(__pa4);
                e = metamodelica::Own::own(__pa5);
                a = metamodelica::Own::own(__pa6);
                co = metamodelica::Own::own(__pa7);
                sm = metamodelica::Own::own(__pa8);
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*cstr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                r#str = dumpCompWithSplitElementsStream(sm, r#str.clone())?;
                r#str = dumpVarsStream(v, false, r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), if ((ie).is_empty()) {literal!("")} else {literal!("initial equation\n")})?;
                r#str = dumpInitialEquationsStream(&ie, r#str.clone())?;
                r#str = dumpInitialAlgorithmsStream(&ia, r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), if ((e).is_empty()) {literal!("")} else {literal!("equation\n")})?;
                r#str = dumpEquationsStream(e, r#str.clone())?;
                r#str = dumpAlgorithmsStream(&a, r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), if ((co).is_empty()) {literal!("")} else {literal!("constraint\n")})?;
                r#str = dumpConstraintStream(&co, r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*name); __mm_s.push_str(&*cstr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) })?;
                { (inCompLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn dumpAlgorithmsStream(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElementLst, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("algorithm\n"))?;
                    r#str = IOStream::appendList(r#str.clone(), &(List::map(stmts.clone(), &fnptr!(ppStatementStr, metamodelica::Ref<DAE::Statement>))?))?;
                    r#str = dumpAlgorithmsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpAlgorithmsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStream)
}

fn dumpInitialAlgorithmsStream(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElementLst, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("initial algorithm\n"))?;
                    r#str = IOStream::appendList(r#str.clone(), &(List::map(stmts.clone(), &fnptr!(ppStatementStr, metamodelica::Ref<DAE::Statement>))?))?;
                    r#str = dumpInitialAlgorithmsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpInitialAlgorithmsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStream)
}

fn dumpEquationsStream(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElementLst, inStream)) {
            (Deref @ metamodelica::ListNode::Nil, r#str) => {
                return Ok(r#str.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                s2 = ExpressionBasics::printExpStr(e2.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1, literal!(" = "), s2, sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: src }, tail: xs }, r#str) => {
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr1))?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr2))?); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) })?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut s3: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                s2 = ExpressionBasics::printExpStr(e2.clone())?;
                s3 = if (Config::typeinfo()?) {TypesDump::printDimensionsStr(dims.clone())?} else {literal!("")};
                s3 = if (Config::typeinfo()?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /* array equation [")); __mm_s.push_str(&*s3); __mm_s.push_str(&*literal!("] */")); ArcStr::from(__mm_s) }} else {literal!("")};
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1, literal!(" = "), s2, s3, sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                s2 = ExpressionBasics::printExpStr(e2.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1, literal!(" = "), s2, sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: c, exp: e, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                s2 = ExpressionBasics::printExpStr(e.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1, literal!(" = "), s2, sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { condition: e1, message: e2, level: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. }, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s2: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                s2 = ExpressionBasics::printExpStr(e2.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  assert("), s1, literal!(","), s2, literal!(")"), sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { message: e1, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  terminate("), s1, literal!(")"), sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::FOR_EQUATION { iter: s, range: e1, equations: xs1, source: src, .. }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut r#str = (*r#str).clone();
                getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e1.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  for "), s.clone(), literal!(" in "), s1, literal!(" loop\n")]))?;
                r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  end for;\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Nil, equations2: Deref @ metamodelica::ListNode::Nil, equations3: Deref @ metamodelica::ListNode::Nil, .. }, tail: _ }, r#str) => {
                return Ok(r#str.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: e, tail: conds }, equations2: Deref @ metamodelica::ListNode::Cons { head: xs1, tail: tb }, equations3: Deref @ metamodelica::ListNode::Nil, source: src }, tail: xs }, r#str) => {
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                r#str = IOStream::append(r#str.clone(), literal!("  if "))?;
                r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(e.clone())?)?;
                r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                r#str = dumpIfEquationsStream(conds.clone(), tb.clone(), r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), literal!("  end if"))?;
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) })?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: e, tail: conds }, equations2: Deref @ metamodelica::ListNode::Cons { head: xs1, tail: tb }, equations3: xs2, source: src }, tail: xs }, r#str) => {
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                r#str = IOStream::append(r#str.clone(), literal!("  if "))?;
                r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(e.clone())?)?;
                r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                r#str = dumpIfEquationsStream(conds.clone(), tb.clone(), r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), literal!("  else\n"))?;
                r#str = dumpEquationsStream(xs2.clone(), r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  end if")); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) })?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { condition: e, equations: xs1, elsewhen_: Some(el), source: src }, tail: xs }, r#str) => {
                let mut r#str = (*r#str).clone();
                getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                r#str = IOStream::append(r#str.clone(), literal!("when "))?;
                r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(e.clone())?)?;
                r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), literal!(" else"))?;
                { (inElementLst, inStream) = (metamodelica::cons(el.clone(), xs.clone()), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { condition: e, equations: xs1, elsewhen_: None, source: src }, tail: xs }, r#str) => {
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                r#str = IOStream::append(r#str.clone(), literal!("  when "))?;
                r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(e.clone())?)?;
                r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  end when")); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) })?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::REINIT { componentRef: cr, exp: e, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut s: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                s1 = ExpressionBasics::printExpStr(e.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  reinit("), s, literal!(","), s1, literal!(")"), sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::NORETCALL { exp: e, source: src }, tail: xs }, r#str) => {
                let mut s1: ArcStr;
                let mut sourceStr: ArcStr;
                let mut r#str = (*r#str).clone();
                sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                s1 = ExpressionBasics::printExpStr(e.clone())?;
                r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1, sourceStr, literal!(";\n")]))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, r#str) => {
                let mut r#str = (*r#str).clone();
                r#str = IOStream::append(r#str.clone(), literal!("  /* unhandled equation in DAEDump.dumpEquationsStream FIXME! */\n"))?;
                { (inElementLst, inStream) = (xs.clone(), r#str.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpIfEquationsStream(
    mut iconds: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut itbs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((iconds, itbs, inStream)) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, r#str) => {
                return Ok(r#str.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: conds }, Deref @ metamodelica::ListNode::Cons { head: tb, tail: tbs }, r#str) => {
                let mut r#str = (*r#str).clone();
                r#str = IOStream::append(r#str.clone(), literal!("  elseif "))?;
                r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(c.clone())?)?;
                r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                r#str = dumpEquationsStream(tb.clone(), r#str.clone())?;
                { (iconds, itbs, inStream) = (conds.clone(), tbs.clone(), r#str.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpInitialEquationsStream(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElementLst, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str = (*r#str).clone();
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), literal!(";\n")]))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { exp: e1, array: e2, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str = (*r#str).clone();
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), literal!(";\n")]))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str = (*r#str).clone();
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), literal!(";\n")]))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIALDEFINE { componentRef: c, exp: e, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str = (*r#str).clone();
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    s2 = ExpressionBasics::printExpStr(e.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), literal!(";\n")]))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_FOR_EQUATION { iter: s2, range: e1, equations: xs1, source: src, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut r#str = (*r#str).clone();
                    getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  for "), s2.clone(), literal!(" in "), s1.clone(), literal!(" loop\n")]))?;
                    r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  end for;\n")]))?;
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: e, tail: conds }, equations2: Deref @ metamodelica::ListNode::Cons { head: xs1, tail: trueBranches }, equations3: xs2, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("  if "))?;
                    r#str = IOStream::append(r#str.clone(), ExpressionBasics::printExpStr(e.clone())?)?;
                    r#str = IOStream::append(r#str.clone(), literal!(" then\n"))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs1), r#str.clone())?;
                    r#str = dumpIfEquationsStream(conds.clone(), trueBranches.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!("  else\n"))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs2), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!("  end if;\n"))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_NORETCALL { exp: e, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut r#str = (*r#str).clone();
                    s1 = ExpressionBasics::printExpStr(e.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), s1.clone(), literal!(";\n")]))?;
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, level: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. }, source: src }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut r#str = (*r#str).clone();
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  assert("), s1.clone(), literal!(","), s2.clone(), literal!(")"), sourceStr.clone(), literal!(";\n")]))?;
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source: src }, tail: xs }, r#str) => {
                    let mut s1: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut r#str = (*r#str).clone();
                    sourceStr = getSourceInformationStr(metamodelica::AsArg::as_arg(&src))?;
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  terminate("), s1.clone(), literal!(")"), sourceStr.clone(), literal!(";\n")]))?;
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpInitialEquationsStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStream)
}

pub fn dumpConstraintStream(
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElementLst, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::CONSTRAINT { constraints: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("  "))?;
                    r#str = IOStream::append(r#str.clone(), stringDelimitList(List::map(exps.clone(), &ExpressionBasics::printExpStr)?, literal!(";\n  ")))?;
                    r#str = IOStream::append(r#str.clone(), literal!(";\n"))?;
                    r#str = dumpConstraintStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpConstraintStream(metamodelica::AsArg::as_arg(&xs), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStream)
}

pub(crate) fn dumpDAEElementsStr(mut d: &DAE::DAElist) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match d.clone() {
        DAE::DAElist { elementLst: ref l } => {
            let mut myStream: IOStream::IOStream;
            myStream = IOStream::create(literal!(""), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpElementsStream(metamodelica::AsArg::as_arg(&l), myStream)?;
            r#str = IOStream::string(&myStream)?;
            r#str
        }
    });
    Ok(r#str)
}

pub(crate) fn dumpVarsStream(
    mut inElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut printTypeDimension: bool,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElementLst, inStream.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(inStream)
            },
            (Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, r#str) => {
                let mut r#str = (*r#str).clone();
                r#str = dumpVarStream(metamodelica::AsArg::as_arg(&first), printTypeDimension, r#str.clone());
                { (inElementLst, printTypeDimension, inStream) = (rest.clone(), printTypeDimension, r#str.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn daeTypeStr(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut outTypeStr: ArcStr;
    let mut typeAttrStr: ArcStr;
    (outTypeStr, typeAttrStr) = printTypeStr(inType)?;
    if !metamodelica::stringEq(&typeAttrStr, &(literal!(""))) {
        outTypeStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outTypeStr);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*typeAttrStr);
            ArcStr::from(__mm_s)
        };
    }
    Ok(outTypeStr)
}

pub fn printTypeStr(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<(ArcStr, ArcStr)> {
    let mut outTypeStr: ArcStr;
    let mut outTypeAttrStr: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ty_vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    (ty, ty_vars) = TypesDump::stripTypeVars(inType);
    outTypeStr = unparseType(&ty)?;
    outTypeAttrStr = List::toStringCustom(
        ty_vars,
        &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(TypesDump::unparseVarAttr(&__a0))
        },
        literal!(""),
        literal!("("),
        literal!(", "),
        literal!(")"),
        false,
        0,
    )?;
    Ok((outTypeStr, outTypeAttrStr))
}

pub(crate) fn dumpCallAttr(mut ca: &metamodelica::Ref<DAE::CallAttributes>) -> Result<()> {
    let mut tpl: bool;
    let mut bi: bool;
    let mut impure_: bool;
    let mut isFunc: bool;
    let mut iType: DAE::InlineType;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut tailCall: DAE::TailCall;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let __arc7 = &(*ca);
    let DAE::CALL_ATTR {
        ty: __pa0,
        tuple_: __pa1,
        builtin: __pa2,
        isImpure: __pa3,
        isFunctionPointerCall: __pa4,
        inlineType: __pa5,
        tailCall: __pa6,
        ..
    } = &**__arc7;
    ty = metamodelica::Own::own(__pa0);
    tpl = metamodelica::Own::own(__pa1);
    bi = metamodelica::Own::own(__pa2);
    impure_ = metamodelica::Own::own(__pa3);
    isFunc = metamodelica::Own::own(__pa4);
    iType = metamodelica::Own::own(__pa5);
    tailCall = metamodelica::Own::own(__pa6);
    metamodelica::print(literal!("Call attributes: \n----------------------\n"));
    (s1, s2) = printTypeStr(&ty)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("DAE-type: "));
        __mm_s.push_str(&*s1);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("DAE-type attributes :"));
        __mm_s.push_str(&*s2);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("tuple_: "));
        __mm_s.push_str(&*boolString(tpl));
        __mm_s.push_str(&*literal!(" builtin: "));
        __mm_s.push_str(&*boolString(bi));
        __mm_s.push_str(&*literal!(" impure: "));
        __mm_s.push_str(&*boolString(impure_));
        __mm_s.push_str(&*literal!(" isFunctionPointerCall: "));
        __mm_s.push_str(&*boolString(isFunc));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub fn dumpVarBindingStr(mut inBinding: Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inBinding) {
        Some(exp) => {
            let mut bind_str: ArcStr;
            bind_str = ExpressionBasics::printExpStr(exp.clone())?;
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*bind_str); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn dumpVarStream(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut printTypeDimension: bool,
    mut inStream: IOStream::IOStream,
) -> IOStream::IOStream {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElement, inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { componentRef: id, kind, direction: dir, parallelism: prl, protection: vis, ty, dims, binding, variableAttributesOption: attr, comment: cmt, .. }, r#str) => {
                    let mut final_str: ArcStr;
                    let mut kind_str: ArcStr;
                    let mut dir_str: ArcStr;
                    let mut ty_str: ArcStr;
                    let mut ty_vars_str: ArcStr;
                    let mut dim_str: ArcStr;
                    let mut name_str: ArcStr;
                    let mut vis_str: ArcStr;
                    let mut par_str: ArcStr;
                    let mut cmt_str: ArcStr;
                    let mut attr_str: ArcStr;
                    let mut binding_str: ArcStr;
                    let mut r#str = (*r#str).clone();
                    final_str = if (DAEUtil::getFinalAttr(attr.clone())) {literal!("final ")} else {literal!("")};
                    kind_str = dumpKindStr(kind.clone());
                    dir_str = dumpDirectionStr(dir.clone());
                    (ty_str, ty_vars_str) = printTypeStr(metamodelica::AsArg::as_arg(&ty))?;
                    dim_str = unparseDimensions(dims.clone(), printTypeDimension)?;
                    name_str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&id))?;
                    vis_str = dumpVarVisibilityStr(vis.clone());
                    par_str = dumpVarParallelismStr(prl.clone());
                    cmt_str = dumpCommentAnnotationStr(cmt.clone());
                    attr_str = dumpVariableAttributesStr(attr.clone());
                    binding_str = dumpVarBindingStr(binding.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("  "), vis_str.clone(), final_str.clone(), par_str.clone(), kind_str.clone(), dir_str.clone(), ty_str.clone(), dim_str.clone(), literal!(" "), name_str.clone(), ty_vars_str.clone(), attr_str.clone(), binding_str.clone(), cmt_str.clone(), literal!(";\n")]))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStream.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStream
}

pub fn dumpAlgorithmStream(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inStream: IOStream::IOStream,
) -> IOStream::IOStream {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElement, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("algorithm\n"))?;
                    r#str = List::fold(metamodelica::AsArg::as_arg(&stmts), &ppStatementStream, r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStream
}

pub(crate) fn dumpInitialAlgorithmStream(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inStream: IOStream::IOStream,
) -> IOStream::IOStream {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (&**inElement, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), literal!("initial algorithm\n"))?;
                    r#str = List::fold(metamodelica::AsArg::as_arg(&stmts), &ppStatementStream, r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStream
}

pub(crate) fn ppStatementStream(
    mut alg: metamodelica::Ref<DAE::Statement>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    let mut hnd: i32;
    hnd = Print::saveAndClearBuf()?;
    ppStatement(alg)?;
    outStream = IOStream::append(inStream, Print::getString()?)?;
    Print::restoreBuf(hnd)?;
    Ok(outStream)
}

pub fn dumpFunctionTree(
    mut inFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inHeading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*inHeading);
        __mm_s.push_str(&*literal!("\n========================================\n"));
        ArcStr::from(__mm_s)
    });
    for mut fnc in &*sortFunctions(DAEUtil::getFunctionList(inFunctionTree, false)?)? {
        metamodelica::print(dumpFunctionStr(metamodelica::AsArg::as_arg(&fnc)));
    }
    Ok(())
}

pub fn dumpFunctionStr(mut inElement: &DAE::Function) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut s: ArcStr;
            let mut hnd: i32;
            hnd = Print::saveAndClearBuf()?;
            dumpFunction(inElement);
            s = Print::getString()?;
            Print::restoreBuf(hnd)?;
            Ok(s.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(literal!(""))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn dumpExtObjClassStr(mut inElement: &metamodelica::Ref<DAE::Element>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
                    let mut s: ArcStr;
                    let mut hnd: i32;
                    hnd = Print::saveAndClearBuf()?;
                    dumpExtObjectClass(inElement);
                    s = Print::getString()?;
                    Print::restoreBuf(hnd)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    outString
}

fn dumpFunctionStream(mut inElement: &DAE::Function, mut inStream: IOStream::IOStream) -> IOStream::IOStream {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement, inStream);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { path: fpath, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, type_: t, isImpure, comment: c, .. }, r#str) => {
                    let mut fstr: ArcStr;
                    let mut impureStr: ArcStr;
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), dumpParallelismStr(metamodelica::AsArg::as_arg(&t)))?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                    impureStr = if (isImpure.clone()) {literal!("impure ")} else {literal!("")};
                    r#str = IOStream::append(r#str.clone(), impureStr.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!("function "))?;
                    r#str = IOStream::append(r#str.clone(), fstr.clone())?;
                    r#str = IOStream::append(r#str.clone(), dumpCommentStr(c.clone()))?;
                    r#str = IOStream::append(r#str.clone(), literal!("\n"))?;
                    r#str = dumpFunctionElementsStream(daeElts.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), dumpClassAnnotationStr(c.clone()))?;
                    r#str = IOStream::append(r#str.clone(), literal!("end "))?;
                    r#str = IOStream::append(r#str.clone(), fstr.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!(";\n\n"))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. }, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { path: fpath, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { body: daeElts, externalDecl: ext_decl }, tail: _ }, isImpure, comment: c, .. }, r#str) => {
                    let mut fstr: ArcStr;
                    let mut ext_decl_str: ArcStr;
                    let mut impureStr: ArcStr;
                    let mut ann_str: ArcStr;
                    let mut r#str = (*r#str).clone();
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                    impureStr = if (isImpure.clone()) {literal!("impure ")} else {literal!("")};
                    r#str = IOStream::append(r#str.clone(), impureStr.clone())?;
                    r#str = IOStream::append(r#str.clone(), literal!("function "))?;
                    r#str = IOStream::append(r#str.clone(), fstr.clone())?;
                    r#str = IOStream::append(r#str.clone(), dumpCommentStr(c.clone()))?;
                    r#str = IOStream::append(r#str.clone(), literal!("\n"))?;
                    r#str = dumpFunctionElementsStream(daeElts.clone(), r#str.clone())?;
                    ext_decl_str = dumpExtDeclStr(metamodelica::AsArg::as_arg(&ext_decl))?;
                    ann_str = dumpClassAnnotationStr(c.clone());
                    r#str = IOStream::appendList(r#str.clone(), &(list![literal!("\n  "), ext_decl_str.clone(), literal!("\n"), ann_str.clone(), literal!("end "), fstr.clone(), literal!(";\n\n")]))?;
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::RECORD_CONSTRUCTOR { path: fpath, type_: tp, .. }, r#str) => {
                    let mut fstr: ArcStr;
                    let mut r#str = (*r#str).clone();
                    let false = (Flags::isSet(Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone())?) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::PRINT_RECORD_TYPES.clone())? {
                        r#str = IOStream::append(r#str.clone(), TypesDump::unparseType(tp.clone())?)?;
                        r#str = IOStream::append(r#str.clone(), literal!("\n"))?;
                    } else {
                        fstr = AbsynUtil::pathStringNoQual(fpath.clone(), literal!("."), false, false)?;
                        r#str = IOStream::append(r#str.clone(), literal!("function "))?;
                        r#str = IOStream::append(r#str.clone(), fstr.clone())?;
                        r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"Automatically generated record constructor for ")); __mm_s.push_str(&*fstr); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) })?;
                        r#str = IOStream::append(r#str.clone(), printRecordConstructorInputsStr(metamodelica::AsArg::as_arg(&tp))?)?;
                        r#str = IOStream::append(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  output ")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&fpath))); __mm_s.push_str(&*literal!(" res;\n")); ArcStr::from(__mm_s) })?;
                        r#str = IOStream::append(r#str.clone(), literal!("end "))?;
                        r#str = IOStream::append(r#str.clone(), fstr.clone())?;
                        r#str = IOStream::append(r#str.clone(), literal!(";\n\n"))?;
                    }
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStream
}

pub(crate) fn dumpFunctionElementsStream(
    mut l: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inStream: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = dumpVarsStream(l.clone(), true, inStream)?;
    outStream = List::fold(
        &l,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: IOStream::IOStream| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dumpAlgorithmStream(&__a0, __a1))
        },
        outStream,
    )?;
    Ok(outStream)
}

pub(crate) fn unparseVarKind(mut inVarKind: DAE::VarKind) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarKind {
        DAE::VarKind::VARIABLE { .. } => literal!(""),
        DAE::VarKind::PARAM { .. } => literal!("parameter"),
        DAE::VarKind::CONST { .. } => literal!("const"),
        DAE::VarKind::DISCRETE { .. } => literal!("discrete"),
    });
    outString
}

pub(crate) fn unparseVarDirection(mut inVarDirection: DAE::VarDirection) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inVarDirection {
        DAE::VarDirection::BIDIR { .. } => literal!(""),
        DAE::VarDirection::INPUT { .. } => literal!("input"),
        DAE::VarDirection::OUTPUT { .. } => literal!("output"),
    });
    outString
}

pub(crate) fn unparseVarInnerOuter(mut io: DAE::VarInnerOuter) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match io {
        DAE::VarInnerOuter::INNER { .. } => literal!("inner"),
        DAE::VarInnerOuter::OUTER { .. } => literal!("outer"),
        DAE::VarInnerOuter::INNER_OUTER { .. } => literal!("inner outer"),
        _ => literal!(""),
    });
    r#str
}

pub(crate) fn getSourceInformationStr(mut inSource: &metamodelica::Ref<DAE::ElementSource>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = 'mc: {
        let __mc_input = &**inSource;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::SHOW_EQUATION_SOURCE.clone())?) else { return Err("pattern mismatch") };
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { info: _, partOfLst: po, instance: _, connectEquationOptLst: ceol, typeLst: _, operations: _, comment: cmt } => {
                    let mut r#str: ArcStr;
                    r#str = cmtListToString(metamodelica::AsArg::as_arg(&cmt));
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" /* models: {")); __mm_s.push_str(&*stringDelimitList(List::map(po.clone(), &move |__a0: Absyn::Within| withinString(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!(" connects: {")); __mm_s.push_str(&*stringDelimitList(connectsStr(metamodelica::AsArg::as_arg(&ceol))?, literal!(", "))); __mm_s.push_str(&*literal!("} */")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStr)
}

fn connectsStr(
    mut inLst: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStr: metamodelica::List<ArcStr>;
    outStr = 'mc: {
        let __mc_input = &**inLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (c1, c2), tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Ok(list![r#str.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (c1, c2), tail: rest } => {
                    let mut slst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); ArcStr::from(__mm_s) };
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    slst = connectsStr(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(metamodelica::cons(r#str.clone(), slst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStr)
}

fn withinString(mut w: &Absyn::Within) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match w.clone() {
        Absyn::Within::TOP { .. } => {
            literal!("TOP")
        }
        Absyn::Within::WITHIN { path: ref p1 } => AbsynUtil::pathString(p1.clone(), literal!("."), true, false)?,
    });
    Ok(r#str)
}

pub(crate) fn cmtListToString(mut inCmtLst: &metamodelica::List<metamodelica::Ref<SCode::Comment>>) -> ArcStr {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match inCmtLst {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut r#str: ArcStr;
            r#str = dumpCommentAnnotationStr(Some(c.clone()));
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } => {
            let mut r#str: ArcStr;
            r#str = dumpCommentAnnotationStr(Some(c.clone()));
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*cmtListToString(rest)); ArcStr::from(__mm_s) };
            r#str
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStr
}

pub(crate) fn clockKindString(mut cK: &metamodelica::Ref<DAE::ClockKind>) -> Result<ArcStr> {
    let mut sOut: ArcStr;
    sOut = (match &**cK {
        DAE::ClockKind::INFERRED_CLOCK { .. } => {
            literal!("Inferred Clock")
        }
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: e1,
            resolution: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Rational Clock("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?);
            __mm_s.push_str(&*literal!("; "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::REAL_CLOCK { interval: e1 } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Real Clock("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition: e1,
            startInterval: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Event Clock("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?);
            __mm_s.push_str(&*literal!("; "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::SOLVER_CLOCK {
            c: e1,
            solverMethod: e2,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Solver Clock("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?);
            __mm_s.push_str(&*literal!("; "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(sOut)
}

pub(crate) fn dumpDebugElementStr(mut inElement: &metamodelica::Ref<DAE::Element>) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: c, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    r#str = stringAppendList(list![literal!("VAR:  "), s1.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: c, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    r#str = stringAppend(s1.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) });
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { componentRef: c, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    r#str = stringAppend(s1.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) });
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("  "), s1.clone(), literal!(" = "), s2.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr1))?;
                    s2 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr2))?;
                    r#str = stringAppendList(list![literal!("EQUEQUATION  "), s1.clone(), literal!(" = "), s2.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ARRAY_EQUATION  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { exp: e1, array: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INITIAL_ARRAY_EQUATION  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("COMPLEX_EQUATION  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INITIAL_COMPLEX_EQUATION  ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*sourceStr); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { condition: e1, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("WHEN_EQUATION:  "), s1.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { source: src, .. } => {
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    r#str = stringAppendList(list![literal!("IF_EQUATION:  "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { source: src, .. } => {
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    r#str = stringAppendList(list![literal!("INITIAL_IF_EQUATION:  "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source: src } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("INITIALEQUATION  "), s1.clone(), literal!(" = "), s2.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { source: src, .. } => {
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    r#str = stringAppendList(list![literal!("ALGO  "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { source: src, .. } => {
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    r#str = stringAppendList(list![literal!("INITIALALGORITHM  "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { source: src, dAElist: elst, .. } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = stringDelimitList(List::map(elst.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpDebugElementStr(&__a0)) })?, literal!("\n"));
                    r#str = stringAppendList(list![literal!("COMP  "), s1.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { path, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    r#str = stringAppendList(list![literal!("EXTOBJ  "), s1.clone(), literal!("  "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { condition: e1, message: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("  assert("), s1.clone(), literal!(","), s2.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, source: src, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    s2 = ExpressionBasics::printExpStr(e2.clone())?;
                    r#str = stringAppendList(list![literal!("  /* initial */ assert("), s1.clone(), literal!(","), s2.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { message: e1, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("  terminate("), s1.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("  /* initial */ terminate("), s1.clone(), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::REINIT { source: src, .. } => {
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    r#str = stringAppendList(list![literal!("  reinit("), literal!(") "), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { exp: e1, source: src } => {
                    let mut s1: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sourceStr: ArcStr;
                    let mut cmt: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                    cmt = ElementSource::getComments(metamodelica::AsArg::as_arg(&src));
                    sourceStr = cmtListToString(&cmt);
                    s1 = ExpressionBasics::printExpStr(e1.clone())?;
                    r#str = stringAppendList(list![literal!("  "), s1.clone(), sourceStr.clone(), literal!(";\n")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("#UNKNOWN_EQUATION#"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

fn getOptionWithConcatStr<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeAOption: Option<Type_a>,
    mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>,
    mut inString: ArcStr,
) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = (match (inTypeAOption, inFuncTypeTypeAToString.clone(), inString) {
        (Some(mut a), mut r, mut default_str) => {
            let mut r#str: ArcStr;
            let mut str_1: ArcStr;
            r#str = r(a)?;
            str_1 = stringAppend(default_str, r#str);
            str_1
        }
        (None, _, _) => {
            literal!("")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}
