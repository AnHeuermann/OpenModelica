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
use crate::DAEDump;
use crate::Expression;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::Graphviz;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
/*
 * - Printing expressions
 *   This module provides some functions to print data to the standard
 *   output.  This is used for error messages, and for debugging the
 *   semantic description.
 */
pub fn subscriptString(mut subscript: &metamodelica::Ref<DAE::Subscript>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match subscript {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            let mut res: ArcStr;
            res = intString(i.clone());
            res
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { name: enum_lit, .. } } => {
            let mut res: ArcStr;
            res = AbsynUtil::pathString(enum_lit.clone(), literal!("."), true, false)?;
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub fn binopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = if (Config::typeinfo()?) {
        binopSymbol2(inOperator)?
    } else {
        binopSymbol1(inOperator)
    };
    Ok(outString)
}

pub(crate) fn binopSymbol1(mut inOperator: &DAE::Operator) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::ADD { .. } => literal!(" + "),
        DAE::Operator::SUB { .. } => literal!(" - "),
        DAE::Operator::MUL { .. } => literal!(" * "),
        DAE::Operator::DIV { .. } => literal!(" / "),
        DAE::Operator::POW { .. } => literal!(" ^ "),
        DAE::Operator::ADD_ARR { .. } => literal!(" + "),
        DAE::Operator::SUB_ARR { .. } => literal!(" - "),
        DAE::Operator::MUL_ARR { .. } => literal!(" * "),
        DAE::Operator::DIV_ARR { .. } => literal!(" / "),
        DAE::Operator::POW_ARR { .. } => literal!(" ^ "),
        DAE::Operator::POW_ARR2 { .. } => literal!(" ^ "),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => literal!(" * "),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => literal!(" + "),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => literal!(" - "),
        DAE::Operator::POW_SCALAR_ARRAY { .. } => literal!(" ^ "),
        DAE::Operator::POW_ARRAY_SCALAR { .. } => literal!(" ^ "),
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => literal!(" * "),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => literal!(" * "),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => literal!(" / "),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => literal!(" / "),
        _ => literal!(" <UNKNOWN_SYMBOL> "),
    });
    outString
}

pub fn debugBinopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::ADD { .. } => literal!(" + "),
        DAE::Operator::SUB { .. } => literal!(" - "),
        DAE::Operator::MUL { .. } => literal!(" * "),
        DAE::Operator::DIV { .. } => literal!(" / "),
        DAE::Operator::POW { .. } => literal!(" ^ "),
        DAE::Operator::EQUAL { .. } => literal!(" = "),
        DAE::Operator::ADD_ARR { .. } => literal!(" +ARR "),
        DAE::Operator::SUB_ARR { .. } => literal!(" -ARR "),
        DAE::Operator::MUL_ARR { .. } => literal!(" *ARR "),
        DAE::Operator::DIV_ARR { .. } => literal!(" /ARR "),
        DAE::Operator::POW_ARR { .. } => literal!(" ^ARR "),
        DAE::Operator::POW_ARR2 { .. } => literal!(" ^ARR2 "),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => literal!(" ARR*S "),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => literal!(" ARR+S "),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => literal!(" - "),
        DAE::Operator::POW_SCALAR_ARRAY { .. } => literal!(" S^ARR "),
        DAE::Operator::POW_ARRAY_SCALAR { .. } => literal!(" ARR^S "),
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => literal!(" Dot "),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => literal!(" MatrixProd "),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => literal!(" S/ARR "),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => literal!(" ARR/S "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn binopSymbol2(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::ADD { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" +<"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::SUB { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" -<"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::MUL { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" *<"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::DIV { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" /<"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::POW { .. } => {
            literal!(" ^ ")
        }
        DAE::Operator::ADD_ARR { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" +<ADD_ARR><"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::SUB_ARR { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" -<SUB_ARR><"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::MUL_ARR { .. } => {
            literal!(" *<MUL_ARRAY> ")
        }
        DAE::Operator::DIV_ARR { ty: ref t } => {
            let mut ts: ArcStr;
            let mut s: ArcStr;
            ts = TypesDump::unparseType(t.clone())?;
            s = stringAppendList(list![literal!(" /<DIV_ARR><"), ts, literal!("> ")]);
            s
        }
        DAE::Operator::POW_ARR { .. } => {
            literal!(" ^<POW_ARR> ")
        }
        DAE::Operator::POW_ARR2 { .. } => {
            literal!(" ^<POW_ARR2> ")
        }
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            literal!(" *<MUL_ARRAY_SCALAR> ")
        }
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => {
            literal!(" +<ADD_ARRAY_SCALAR> ")
        }
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => {
            literal!(" -<SUB_SCALAR_ARRAY> ")
        }
        DAE::Operator::POW_SCALAR_ARRAY { .. } => {
            literal!(" ^<POW_SCALAR_ARRAY> ")
        }
        DAE::Operator::POW_ARRAY_SCALAR { .. } => {
            literal!(" ^<POW_ARRAY_SCALAR> ")
        }
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            literal!(" *<MUL_SCALAR_PRODUCT> ")
        }
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            literal!(" *<MUL_MATRIX_PRODUCT> ")
        }
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => {
            literal!(" /<DIV_SCALAR_ARRAY> ")
        }
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            literal!(" /<DIV_ARRAY_SCALAR> ")
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub fn unaryopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::UMINUS { .. } => {
            if (Config::typeinfo()?) {
                literal!("-<UMINUS>")
            } else {
                literal!("-")
            }
        }
        DAE::Operator::UMINUS_ARR { .. } => {
            if (Config::typeinfo()?) {
                literal!("-<UMINUS_ARR>")
            } else {
                literal!("-")
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn lbinopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::AND { ty: _ } => literal!(" and "),
        DAE::Operator::OR { ty: _ } => literal!(" or "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn lunaryopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::NOT { ty: _ } => literal!("not "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub fn relopSymbol(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::LESS { .. } => literal!(" < "),
        DAE::Operator::LESSEQ { .. } => literal!(" <= "),
        DAE::Operator::GREATER { .. } => literal!(" > "),
        DAE::Operator::GREATEREQ { .. } => literal!(" >= "),
        DAE::Operator::EQUAL { .. } => literal!(" == "),
        DAE::Operator::NEQUAL { .. } => literal!(" <> "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn printList<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: &metamodelica::List<Type_a>,
    mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>,
    mut inString: ArcStr,
) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;

    let () = 'mc: {
        let __mc_input = (&**inTypeALst, inFuncTypeTypeATo.clone(), inString);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf(sep.clone())?;
                    printList(metamodelica::AsArg::as_arg(&t), r.clone(), sep.clone())?;
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

fn printRow(mut es_1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<()> {
    printList(
        es_1,
        (std::sync::Arc::new(printExp)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<()> + 'static>),
        literal!(","),
    )?;
    Ok(())
}

pub(crate) fn debugPrintSubscriptStr(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inSubscript {
        DAE::Subscript::WHOLEDIM { .. } => {
            literal!(":")
        }
        DAE::Subscript::INDEX { exp: e1 } => {
            let mut s: ArcStr;
            s = dumpExpStr(e1.clone(), 0)?;
            s = System::stringReplace(s, literal!("\n"), literal!(""))?;
            s
        }
        DAE::Subscript::SLICE { exp: e1 } => {
            let mut s: ArcStr;
            s = dumpExpStr(e1.clone(), 0)?;
            s = System::stringReplace(s, literal!("\n"), literal!(""))?;
            s
        }
        DAE::Subscript::WHOLE_NONEXP { exp: e1 } => {
            let mut s: ArcStr;
            s = dumpExpStr(e1.clone(), 0)?;
            s = System::stringReplace(s, literal!("\n"), literal!(""))?;
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("1:"));
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(outString)
}

pub fn printSubscriptLstStr(
    mut inSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = stringDelimitList(
        List::map(inSubscriptLst, &move |__a0: metamodelica::Ref<DAE::Subscript>| {
            ExpressionBasics::printSubscriptStr(&__a0)
        })?,
        literal!(" , "),
    );
    Ok(outString)
}

pub fn printExpListStr(mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringDelimitList(List::map(expl, &ExpressionBasics::printExpStr)?, literal!(", "));
    Ok(res)
}

// stefan
pub(crate) fn printExpListStrNoSpace(mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringAppendList(List::map(expl, &ExpressionBasics::printExpStr)?);
    Ok(res)
}

pub fn printOptExpStr(mut oexp: Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(oexp) {
        Some(e) => {
            ExpressionBasics::printExpStr(e.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}

pub(crate) fn printCrefsFromExpStr(mut e: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = Tpl::tplString2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<DAE::Exp>, __a2: ArcStr| {
                ExpressionDumpTpl::dumpExpCrefs(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<DAE::Exp>, ArcStr) -> Result<Tpl::Text> + 'static,
            >),
        e,
        literal!(""),
    )?;
    Ok(s)
}

pub fn printExp2Str<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut stringDelimiter: &ArcStr,
    mut opcreffunc: Option<(
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>,
        Type_a,
    )>,
    mut opcallfunc: Option<
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::Exp>,
                    ArcStr,
                    Option<(
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr>
                                + 'static,
                        >,
                        Type_a,
                    )>,
                ) -> Result<ArcStr>
                + 'static,
        >,
    >,
) -> ArcStr {
    pub type printComponentRefStrFunc<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>;

    pub type printCallFunc<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                ArcStr,
                Option<(
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>,
                    Type_a,
                )>,
            ) -> Result<ArcStr>
            + 'static,
    >;

    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inExp.clone(), &opcreffunc, &opcallfunc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::EMPTY { scope, name, tyStr, .. }, _, _) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<EMPTY(scope: ")); __mm_s.push_str(&*scope); __mm_s.push_str(&*literal!(", name: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?); __mm_s.push_str(&*literal!(", ty: ")); __mm_s.push_str(&*tyStr); __mm_s.push_str(&*literal!(")>")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i }, _, _) => {
                    let mut s: ArcStr;
                    s = intString(i.clone());
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, _, _) => {
                    let mut s: ArcStr;
                    s = realString(r.clone());
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s }, _, _) => {
                    let mut s = (*s).clone();
                    s = System::escapedString(s.clone(), false);
                    s = stringAppendList(list![stringDelimiter.clone(), s.clone(), stringDelimiter.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: b }, _, _) => {
                    Ok(boolString(b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, Some((pcreffunc, creffuncparam)), _) => {
                    let mut s: ArcStr;
                    s = pcreffunc(c.clone(), creffuncparam.clone())?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, _, _) => {
                    let mut s: ArcStr;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    if listMember(literal!("dataReconciliation"), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) || listMember(literal!("dataReconciliationStateEstimation"), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) || listMember(literal!("dataReconciliationBoundaryConditions"), Flags::getConfigStringList(Flags::PRE_OPT_MODULES_ADD.clone())?) {
                        s = System::stringReplace(s.clone(), literal!("."), literal!("_"))?;
                    }
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { name: lit, .. }, _, _) => {
                    let mut s: ArcStr;
                    s = AbsynUtil::pathString(lit.clone(), literal!("."), true, false)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, _, _) => {
                    let mut s: ArcStr;
                    let mut sym: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut p1: i32;
                    let mut p2: i32;
                    let mut p: i32;
                    sym = binopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s2 = printExp2Str(e2.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                    p2 = expPriority(metamodelica::AsArg::as_arg(&e2));
                    s1_1 = parenthesize(s1.clone(), p1, p, false);
                    s2_1 = parenthesize(s2.clone(), p2, p, true);
                    s = stringAppendList(list![s1_1.clone(), sym.clone(), s2_1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, _, _) => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sym: ArcStr;
                    let mut p1: i32;
                    let mut p: i32;
                    sym = unaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                    s_1 = parenthesize(s.clone(), p1, p, true);
                    s_2 = stringAppend(sym.clone(), s_1.clone());
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, _, _) => {
                    let mut s: ArcStr;
                    let mut sym: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut p1: i32;
                    let mut p2: i32;
                    let mut p: i32;
                    sym = lbinopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s2 = printExp2Str(e2.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                    p2 = expPriority(metamodelica::AsArg::as_arg(&e2));
                    s1_1 = parenthesize(s1.clone(), p1, p, false);
                    s2_1 = parenthesize(s2.clone(), p2, p, true);
                    s = stringAppendList(list![s1_1.clone(), sym.clone(), s2_1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, _, _) => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sym: ArcStr;
                    let mut p1: i32;
                    let mut p: i32;
                    sym = lunaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                    s_1 = parenthesize(s.clone(), p1, p, false);
                    s_2 = stringAppend(sym.clone(), s_1.clone());
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut sym: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut p1: i32;
                    let mut p2: i32;
                    let mut p: i32;
                    sym = relopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s2 = printExp2Str(e2.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                    p2 = expPriority(metamodelica::AsArg::as_arg(&e2));
                    s1_1 = parenthesize(s1.clone(), p1, p, false);
                    s2_1 = parenthesize(s2.clone(), p2, p, true);
                    s = stringAppendList(list![s1_1.clone(), sym.clone(), s2_1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb }, _, _) => {
                    let mut fs: ArcStr;
                    let mut r#str: ArcStr;
                    let mut cs: ArcStr;
                    let mut ts: ArcStr;
                    let mut cs_1: ArcStr;
                    let mut ts_1: ArcStr;
                    let mut fs_1: ArcStr;
                    let mut pc: i32;
                    let mut pt: i32;
                    let mut pf: i32;
                    let mut p: i32;
                    cs = printExp2Str(cond.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    ts = printExp2Str(tb.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    fs = printExp2Str(fb.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    pc = expPriority(metamodelica::AsArg::as_arg(&cond));
                    pt = expPriority(metamodelica::AsArg::as_arg(&tb));
                    pf = expPriority(metamodelica::AsArg::as_arg(&fb));
                    cs_1 = parenthesize(cs.clone(), pc, p, false);
                    ts_1 = parenthesize(ts.clone(), pt, p, false);
                    fs_1 = parenthesize(fs.clone(), pf, p, false);
                    r#str = stringAppendList(list![literal!("if "), cs_1.clone(), literal!(" then "), ts_1.clone(), literal!(" else "), fs_1.clone()]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { .. }, _, Some(pcallfunc)) => {
                    let mut s_2: ArcStr;
                    s_2 = pcallfunc(e.clone(), stringDelimiter.clone(), opcreffunc.clone())?;
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fcn.clone()), literal!("."), true, false)?;
                    argstr = stringDelimitList(List::map3(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![fs.clone(), literal!("("), argstr.clone(), literal!(")")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path: fcn, expList: args, .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fcn.clone()), literal!("."), true, false)?;
                    argstr = stringDelimitList(List::map3(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![literal!("function "), fs.clone(), literal!("("), argstr.clone(), literal!(")")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: es, .. }, _, _) => {
                    let mut s: ArcStr;
                    s = stringDelimitList(List::map3(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![literal!("{"), s.clone(), literal!("}")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: es }, _, _) => {
                    let mut s: ArcStr;
                    s = stringDelimitList(List::map3(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![literal!("("), s.clone(), literal!(")")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: lstes, .. }, _, _) => {
                    let mut s: ArcStr;
                    s = stringDelimitList(List::map1(lstes.clone(), &printRowStr, stringDelimiter.clone())?, literal!("},{"));
                    s = stringAppendList(list![literal!("{{"), s.clone(), literal!("}}")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { ty: _, start, step: None, stop }, _, _) => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s3: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s3_1: ArcStr;
                    let mut p: i32;
                    let mut pstop: i32;
                    let mut pstart: i32;
                    s1 = printExp2Str(start.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s3 = printExp2Str(stop.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    pstart = expPriority(metamodelica::AsArg::as_arg(&start));
                    pstop = expPriority(metamodelica::AsArg::as_arg(&stop));
                    s1_1 = parenthesize(s1.clone(), pstart, p, false);
                    s3_1 = parenthesize(s3.clone(), pstop, p, false);
                    s = stringAppendList(list![s1_1.clone(), literal!(":"), s3_1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { ty: _, start, step: Some(step), stop }, _, _) => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut s3_1: ArcStr;
                    let mut p: i32;
                    let mut pstop: i32;
                    let mut pstart: i32;
                    let mut pstep: i32;
                    s1 = printExp2Str(start.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s2 = printExp2Str(step.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s3 = printExp2Str(stop.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    p = expPriority(metamodelica::AsArg::as_arg(&e));
                    pstart = expPriority(metamodelica::AsArg::as_arg(&start));
                    pstop = expPriority(metamodelica::AsArg::as_arg(&stop));
                    pstep = expPriority(metamodelica::AsArg::as_arg(&step));
                    s1_1 = parenthesize(s1.clone(), pstart, p, false);
                    s3_1 = parenthesize(s3.clone(), pstop, p, false);
                    s2_1 = parenthesize(s2.clone(), pstep, p, false);
                    s = stringAppendList(list![s1_1.clone(), literal!(":"), s2_1.clone(), literal!(":"), s3_1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: tp, exp: e }, _, _) => {
                    let mut s: ArcStr;
                    let mut res: ArcStr;
                    let mut r#str: ArcStr;
                    r#str = TypesDump::unparseType(tp.clone())?;
                    s = printExp2Str(e.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    res = stringAppendList(list![literal!("DAE.CAST("), r#str.clone(), literal!(", "), s.clone(), literal!(")")]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (e @ Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }, _, _) => {
                            let mut s1: ArcStr;
                            let mut s4: ArcStr;
                            let mut s_4: ArcStr;
                            let mut s1_1: ArcStr;
                            let mut pe1: i32;
                            let mut p: i32;
                            let mut aexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            aexpl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            p = expPriority(metamodelica::AsArg::as_arg(&e));
                            pe1 = expPriority(metamodelica::AsArg::as_arg(&e1));
                            s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                            s1_1 = parenthesize(s1.clone(), pe1, p, false);
                            s4 = stringDelimitList(List::map3(aexpl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                            s_4 = { let mut __mm_s = String::new(); __mm_s.push_str(&*s1_1); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*s4); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                            Ok(s_4.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { exp: cr, sz: Some(dim) }, _, _) => {
                    let mut r#str: ArcStr;
                    let mut crstr: ArcStr;
                    let mut dimstr: ArcStr;
                    crstr = printExp2Str(cr.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    dimstr = printExp2Str(dim.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    r#str = stringAppendList(list![literal!("size("), crstr.clone(), literal!(","), dimstr.clone(), literal!(")")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { exp: cr, sz: None }, _, _) => {
                    let mut r#str: ArcStr;
                    let mut crstr: ArcStr;
                    crstr = printExp2Str(cr.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    r#str = stringAppendList(list![literal!("size("), crstr.clone(), literal!(")")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. }, expr: exp, iterators: riters }, _, _) => {
                    let mut fs: ArcStr;
                    let mut r#str: ArcStr;
                    let mut expstr: ArcStr;
                    let mut iterstr: ArcStr;
                    fs = AbsynUtil::pathStringNoQual(fcn.clone(), literal!("."), false, false)?;
                    expstr = printExp2Str(exp.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    iterstr = stringDelimitList(List::map(riters.clone(), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>| reductionIteratorStr(&__a0))?, literal!(","));
                    r#str = stringAppendList(list![literal!("<reduction>"), fs.clone(), literal!("("), expstr.clone(), literal!(" for "), iterstr.clone(), literal!(")")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_TUPLE { listExp: es }, _, _) => {
                    let mut s: ArcStr;
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tuple")); __mm_s.push_str(&*printExp2Str(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: es.clone() }), stringDelimiter, opcreffunc.clone(), opcallfunc.clone())); ArcStr::from(__mm_s) };
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: es }, _, _) => {
                    let mut s: ArcStr;
                    s = stringDelimitList(List::map3(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![literal!("List("), s.clone(), literal!(")")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }, _, _) => {
                    let mut s_2: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s2 = printExp2Str(e2.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s_2 = stringAppendList(list![literal!("listCons("), s1.clone(), literal!(","), s2.clone(), literal!(")")]);
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: None }, _, _) => {
                    Ok(literal!("NONE()"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, _, _) => {
                    let mut s_1: ArcStr;
                    let mut s1: ArcStr;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s_1 = stringAppendList(list![literal!("SOME("), s1.clone(), literal!(")")]);
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e1 }, _, _) => {
                    let mut s_1: ArcStr;
                    let mut s1: ArcStr;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s_1 = stringAppendList(list![literal!("#("), s1.clone(), literal!(")")]);
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: e1, ty: _ }, _, _) => {
                    let mut s_1: ArcStr;
                    let mut s1: ArcStr;
                    s1 = printExp2Str(e1.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s_1 = stringAppendList(list![literal!("unbox("), s1.clone(), literal!(")")]);
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::METARECORDCALL { path: fcn, args, .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    argstr = stringDelimitList(List::map3(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> { ::std::result::Result::Ok(printExp2Str(__a0, &__a1, __a2, __a3)) }, stringDelimiter.clone(), opcreffunc.clone(), opcallfunc.clone())?, literal!(","));
                    s = stringAppendList(list![fs.clone(), literal!("("), argstr.clone(), literal!(")")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchTy, inputs: es, cases, .. }, _, _) => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    s1 = printMatchType(metamodelica::AsArg::as_arg(&matchTy))?;
                    s2 = printExp2Str(metamodelica::Ref::new(DAE::Exp::TUPLE { PR: es.clone() }), stringDelimiter, opcreffunc.clone(), opcallfunc.clone());
                    s3 = stringAppendList(List::map(cases.clone(), &move |__a0: metamodelica::Ref<DAE::MatchCase>| printCase2Str(&__a0))?);
                    s = stringAppendList(list![s1.clone(), s2.clone(), literal!("\n"), s3.clone(), literal!("  end "), s1.clone()]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SHARED_LITERAL { exp: e, .. }, _, _) => {
                    Ok(printExp2Str(e.clone(), stringDelimiter, opcreffunc.clone(), opcallfunc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PATTERN { pattern: pat }, _, _) => {
                    Ok(patternStr(metamodelica::AsArg::as_arg(&pat))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CODE { code, .. }, _, _) => {
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$Code(")); __mm_s.push_str(&*Dump::printCodeStr(code.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(printExpTypeStr(&inExp))
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

fn printExpTypeStr(mut inExp: &metamodelica::Ref<DAE::Exp>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inExp {
        DAE::Exp::ICONST { integer: _ } => literal!("ICONST"),
        DAE::Exp::RCONST { real: _ } => literal!("RCONST"),
        DAE::Exp::SCONST { string: _ } => literal!("SCONST"),
        DAE::Exp::BCONST { bool: _ } => literal!("BCONST"),
        DAE::Exp::ENUM_LITERAL { .. } => literal!("ENUM_LITERAL"),
        DAE::Exp::CREF { .. } => literal!("CREF"),
        DAE::Exp::BINARY { .. } => literal!("BINARY"),
        DAE::Exp::UNARY { .. } => literal!("UNARY"),
        DAE::Exp::LBINARY { .. } => literal!("LBINARY"),
        DAE::Exp::LUNARY { .. } => literal!("LUNARY"),
        DAE::Exp::RELATION { .. } => literal!("RELATION"),
        DAE::Exp::IFEXP { .. } => literal!("IFEXP"),
        DAE::Exp::CALL { .. } => literal!("CALL"),
        DAE::Exp::PARTEVALFUNCTION { .. } => literal!("PARTEVALFUNCTION"),
        DAE::Exp::ARRAY { .. } => literal!("ARRAY"),
        DAE::Exp::MATRIX { .. } => literal!("MATRIX"),
        DAE::Exp::RANGE { .. } => literal!("RANGE"),
        DAE::Exp::TUPLE { .. } => literal!("TUPLE"),
        DAE::Exp::CAST { .. } => literal!("CAST"),
        DAE::Exp::ASUB { .. } => literal!("ASUB"),
        DAE::Exp::TSUB { .. } => literal!("TSUB"),
        DAE::Exp::SIZE { .. } => literal!("SIZE"),
        DAE::Exp::CODE { .. } => literal!("CODE"),
        DAE::Exp::EMPTY { .. } => literal!("EMPTY"),
        DAE::Exp::REDUCTION { .. } => literal!("REDUCTION"),
        DAE::Exp::LIST { .. } => literal!("LIST"),
        DAE::Exp::CONS { .. } => literal!("CAR"),
        DAE::Exp::META_TUPLE { .. } => literal!("META_TUPLE"),
        DAE::Exp::META_OPTION { .. } => literal!("META_OPTION"),
        DAE::Exp::METARECORDCALL { .. } => literal!("METARECORDCALL"),
        DAE::Exp::MATCHEXPRESSION { .. } => literal!("MATCHEXPRESSION"),
        DAE::Exp::BOX { .. } => literal!("BOX"),
        DAE::Exp::UNBOX { .. } => literal!("UNBOX"),
        DAE::Exp::SHARED_LITERAL { .. } => literal!("SHARED_LITERAL"),
        DAE::Exp::PATTERN { .. } => literal!("PATTERN"),
        _ => literal!("#UNKNOWN EXPRESSION#"),
    });
    outString
}

fn reductionIteratorStr(mut riter: &metamodelica::Ref<DAE::ReductionIterator>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match riter {
        Deref @ DAE::ReductionIterator { id, exp, guardExp: None, .. } => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); ArcStr::from(__mm_s) };
            r#str
        },
        Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(gexp), .. } => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" guard ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(gexp.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); ArcStr::from(__mm_s) };
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

fn printMatchType(mut ty: &DAE::MatchType) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &(ty) {
        DAE::MatchType::MATCHCONTINUE { .. } => literal!("matchcontinue"),
        DAE::MatchType::MATCH { switch: None } => literal!("match"),
        DAE::MatchType::MATCH { switch: Some(_) } => literal!("match /* switch */"),
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

fn printCase2Str(mut matchCase: &metamodelica::Ref<DAE::MatchCase>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match matchCase {
        Deref @ DAE::MatchCase { patterns, body: Deref @ metamodelica::ListNode::Nil, result: Some(result), .. } => {
            let mut resultStr: ArcStr;
            let mut patternsStr: ArcStr;
            patternsStr = patternStr(&(metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))?;
            resultStr = ExpressionBasics::printExpStr(result.clone())?;
            stringAppendList(list![literal!("    case "), patternsStr, literal!(" then "), resultStr, literal!(";\n")])
        },
        Deref @ DAE::MatchCase { patterns, body: Deref @ metamodelica::ListNode::Nil, result: None, .. } => {
            let mut patternsStr: ArcStr;
            patternsStr = patternStr(&(metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))?;
            stringAppendList(list![literal!("    case "), patternsStr, literal!(" then fail();\n")])
        },
        Deref @ DAE::MatchCase { patterns, body, result: Some(result), .. } => {
            let mut resultStr: ArcStr;
            let mut patternsStr: ArcStr;
            let mut bodyStr: ArcStr;
            patternsStr = patternStr(&(metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))?;
            resultStr = ExpressionBasics::printExpStr(result.clone())?;
            bodyStr = stringAppendList(List::map1(body.clone(), &fnptr!(DAEDump::ppStmtStr, metamodelica::Ref<DAE::Statement>, i32), 8)?);
            stringAppendList(list![literal!("    case "), patternsStr, literal!("\n      algorithm\n"), bodyStr, literal!("      then "), resultStr, literal!(";\n")])
        },
        Deref @ DAE::MatchCase { patterns, body, result: None, .. } => {
            let mut patternsStr: ArcStr;
            let mut bodyStr: ArcStr;
            patternsStr = patternStr(&(metamodelica::Ref::new(DAE::Pattern::PAT_META_TUPLE { patterns: patterns.clone() })))?;
            bodyStr = stringAppendList(List::map1(body.clone(), &fnptr!(DAEDump::ppStmtStr, metamodelica::Ref<DAE::Statement>, i32), 8)?);
            stringAppendList(list![literal!("    case "), patternsStr, literal!("\n      algorithm\n"), bodyStr, literal!("      then fail();\n")])
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub fn expPriority(mut inExp: &metamodelica::Ref<DAE::Exp>) -> i32 {
    let mut outInteger: i32;
    outInteger = (match &**inExp {
        DAE::Exp::ICONST { integer: _ } => 0,
        DAE::Exp::RCONST { real: _ } => 0,
        DAE::Exp::SCONST { string: _ } => 0,
        DAE::Exp::BCONST { bool: _ } => 0,
        DAE::Exp::ENUM_LITERAL { .. } => 0,
        DAE::Exp::CREF { componentRef: _, ty: _ } => 0,
        DAE::Exp::ASUB { exp: _, sub: _ } => 0,
        DAE::Exp::CAST { ty: _, exp: _ } => 0,
        DAE::Exp::CALL { .. } => 0,
        DAE::Exp::PARTEVALFUNCTION { .. } => 0,
        DAE::Exp::ARRAY { .. } => 0,
        DAE::Exp::MATRIX { .. } => 0,
        DAE::Exp::BINARY {
            operator: DAE::Operator::POW { ty: _ },
            ..
        } => 3,
        DAE::Exp::BINARY {
            operator: DAE::Operator::POW_ARR { ty: _ },
            ..
        } => 3,
        DAE::Exp::BINARY {
            operator: DAE::Operator::POW_ARR2 { ty: _ },
            ..
        } => 3,
        DAE::Exp::BINARY {
            operator: DAE::Operator::POW_SCALAR_ARRAY { ty: _ },
            ..
        } => 3,
        DAE::Exp::BINARY {
            operator: DAE::Operator::POW_ARRAY_SCALAR { ty: _ },
            ..
        } => 3,
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV { ty: _ },
            ..
        } => 5,
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_ARR { ty: _ },
            ..
        } => 5,
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ },
            ..
        } => 5,
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ },
            ..
        } => 5,
        DAE::Exp::BINARY {
            operator: DAE::Operator::MUL { ty: _ },
            ..
        } => 7,
        DAE::Exp::BINARY {
            operator: DAE::Operator::MUL_ARR { ty: _ },
            ..
        } => 7,
        DAE::Exp::BINARY {
            operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ },
            ..
        } => 7,
        DAE::Exp::BINARY {
            operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ },
            ..
        } => 7,
        DAE::Exp::BINARY {
            operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ },
            ..
        } => 7,
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS { ty: _ },
            ..
        } => 8,
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS_ARR { ty: _ },
            ..
        } => 8,
        DAE::Exp::BINARY {
            operator: DAE::Operator::ADD { ty: _ },
            ..
        } => 9,
        DAE::Exp::BINARY {
            operator: DAE::Operator::ADD_ARR { ty: _ },
            ..
        } => 9,
        DAE::Exp::BINARY {
            operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: _ },
            ..
        } => 9,
        DAE::Exp::BINARY {
            operator: DAE::Operator::SUB { ty: _ },
            ..
        } => 9,
        DAE::Exp::BINARY {
            operator: DAE::Operator::SUB_ARR { ty: _ },
            ..
        } => 9,
        DAE::Exp::BINARY {
            operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: _ },
            ..
        } => 9,
        DAE::Exp::RELATION {
            operator: DAE::Operator::LESS { ty: _ },
            ..
        } => 11,
        DAE::Exp::RELATION {
            operator: DAE::Operator::LESSEQ { ty: _ },
            ..
        } => 11,
        DAE::Exp::RELATION {
            operator: DAE::Operator::GREATER { ty: _ },
            ..
        } => 11,
        DAE::Exp::RELATION {
            operator: DAE::Operator::GREATEREQ { ty: _ },
            ..
        } => 11,
        DAE::Exp::RELATION {
            operator: DAE::Operator::EQUAL { ty: _ },
            ..
        } => 11,
        DAE::Exp::RELATION {
            operator: DAE::Operator::NEQUAL { ty: _ },
            ..
        } => 11,
        DAE::Exp::LUNARY {
            operator: DAE::Operator::NOT { ty: _ },
            ..
        } => 13,
        DAE::Exp::LBINARY {
            operator: DAE::Operator::AND { ty: _ },
            ..
        } => 15,
        DAE::Exp::LBINARY {
            operator: DAE::Operator::OR { ty: _ },
            ..
        } => 17,
        DAE::Exp::RANGE { .. } => 19,
        DAE::Exp::IFEXP { .. } => 21,
        DAE::Exp::TUPLE { PR: _ } => 23,
        _ => 25,
    });
    outInteger
}

pub(crate) fn printRowStr(
    mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut stringDelimiter: ArcStr,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(
        List::map3(
            es_1,
            &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: ArcStr, __a2: _, __a3: _| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(printExp2Str::<()>(__a0, &__a1, __a2, __a3))
            },
            stringDelimiter,
            None,
            None,
        )?,
        literal!(","),
    );
    Ok(s)
}

pub(crate) fn dumpExpGraphviz(mut inExp: &metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<Graphviz::Node> {
    let mut outNode: metamodelica::Ref<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
                    let mut s: ArcStr;
                    s = intString(i.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("ICONST"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: r } => {
                    let mut s: ArcStr;
                    s = realString(r.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("RCONST"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    let mut s = (*s).clone();
                    s = System::escapedString(s.clone(), true);
                    s = stringAppendList(list![literal!("\""), s.clone(), literal!("\"")]);
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("SCONST"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: b } => {
                    let mut s: ArcStr;
                    s = boolString(b.clone());
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("BCONST"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: c, .. } => {
                    let mut s: ArcStr;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("CREF"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr;
                    let mut lt: metamodelica::Ref<Graphviz::Node>;
                    let mut rt: metamodelica::Ref<Graphviz::Node>;
                    sym = binopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    lt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e1));
                    rt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e2));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("BINARY"), labelLst: list![sym.clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e } => {
                    let mut sym: ArcStr;
                    let mut ct: metamodelica::Ref<Graphviz::Node>;
                    sym = unaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    ct = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("UNARY"), labelLst: list![sym.clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr;
                    let mut lt: metamodelica::Ref<Graphviz::Node>;
                    let mut rt: metamodelica::Ref<Graphviz::Node>;
                    sym = lbinopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    lt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e1));
                    rt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e2));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("LBINARY"), labelLst: list![sym.clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e } => {
                    let mut sym: ArcStr;
                    let mut ct: metamodelica::Ref<Graphviz::Node>;
                    sym = lunaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    ct = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("LUNARY"), labelLst: list![sym.clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. } => {
                    let mut sym: ArcStr;
                    let mut lt: metamodelica::Ref<Graphviz::Node>;
                    let mut rt: metamodelica::Ref<Graphviz::Node>;
                    sym = relopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    lt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e1));
                    rt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e2));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("RELATION"), labelLst: list![sym.clone()], attributes: metamodelica::nil(), children: list![lt.clone(), rt.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: t, expElse: f } => {
                    let mut ct: metamodelica::Ref<Graphviz::Node>;
                    let mut tt: metamodelica::Ref<Graphviz::Node>;
                    let mut ft: metamodelica::Ref<Graphviz::Node>;
                    ct = dumpExpGraphviz(metamodelica::AsArg::as_arg(&cond));
                    tt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&t));
                    ft = dumpExpGraphviz(metamodelica::AsArg::as_arg(&f));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("IFEXP"), attributes: metamodelica::nil(), children: list![ct.clone(), tt.clone(), ft.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
                    let mut fs: ArcStr;
                    let mut argnodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    argnodes = List::map(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpExpGraphviz(&__a0)) })?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("CALL"), labelLst: list![fs.clone()], attributes: metamodelica::nil(), children: argnodes.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { expList: args, .. } => {
                    let mut argnodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    argnodes = List::map(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpExpGraphviz(&__a0)) })?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("PARTEVALFUNCTION"), attributes: metamodelica::nil(), children: argnodes.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: es, .. } => {
                    let mut nodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nodes = List::map(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpExpGraphviz(&__a0)) })?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("ARRAY"), attributes: metamodelica::nil(), children: nodes.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: es } => {
                    let mut nodes: metamodelica::List<metamodelica::Ref<Graphviz::Node>>;
                    nodes = List::map(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(dumpExpGraphviz(&__a0)) })?;
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("TUPLE"), attributes: metamodelica::nil(), children: nodes.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: lstes, .. } => {
                    let mut s: ArcStr;
                    s = stringDelimitList(List::map1(lstes.clone(), &printRowStr, literal!("\""))?, literal!("},{"));
                    s = stringAppendList(list![literal!("{{"), s.clone(), literal!("}}")]);
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("MATRIX"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { start, step: None, stop, .. } => {
                    let mut t1: metamodelica::Ref<Graphviz::Node>;
                    let mut t2: metamodelica::Ref<Graphviz::Node>;
                    let mut t3: metamodelica::Ref<Graphviz::Node>;
                    t1 = dumpExpGraphviz(metamodelica::AsArg::as_arg(&start));
                    t2 = metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!(":"), attributes: metamodelica::nil(), children: metamodelica::nil() });
                    t3 = dumpExpGraphviz(metamodelica::AsArg::as_arg(&stop));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("RANGE"), attributes: metamodelica::nil(), children: list![t1.clone(), t2.clone(), t3.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { start, step: Some(step), stop, .. } => {
                    let mut t1: metamodelica::Ref<Graphviz::Node>;
                    let mut t2: metamodelica::Ref<Graphviz::Node>;
                    let mut t3: metamodelica::Ref<Graphviz::Node>;
                    t1 = dumpExpGraphviz(metamodelica::AsArg::as_arg(&start));
                    t2 = dumpExpGraphviz(metamodelica::AsArg::as_arg(&step));
                    t3 = dumpExpGraphviz(metamodelica::AsArg::as_arg(&stop));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("RANGE"), attributes: metamodelica::nil(), children: list![t1.clone(), t2.clone(), t3.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty, exp: e } => {
                    let mut tystr: ArcStr;
                    let mut ct: metamodelica::Ref<Graphviz::Node>;
                    tystr = TypesDump::unparseType(ty.clone())?;
                    ct = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("CAST"), labelLst: list![tystr.clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut s: ArcStr;
                    let mut istr: ArcStr;
                    let mut ct: metamodelica::Ref<Graphviz::Node>;
                    ct = dumpExpGraphviz(metamodelica::AsArg::as_arg(&e));
                    istr = intString(i.clone());
                    s = stringAppendList(list![literal!("["), istr.clone(), literal!("]")]);
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("ASUB"), labelLst: list![s.clone()], attributes: metamodelica::nil(), children: list![ct.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: cr, sz: Some(dim) } => {
                    let mut crt: metamodelica::Ref<Graphviz::Node>;
                    let mut dimt: metamodelica::Ref<Graphviz::Node>;
                    crt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&cr));
                    dimt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&dim));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("SIZE"), attributes: metamodelica::nil(), children: list![crt.clone(), dimt.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: cr, sz: None } => {
                    let mut crt: metamodelica::Ref<Graphviz::Node>;
                    crt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&cr));
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("SIZE"), attributes: metamodelica::nil(), children: list![crt.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. }, expr: exp, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: iterexp, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut fs: ArcStr;
                    let mut expt: metamodelica::Ref<Graphviz::Node>;
                    let mut itert: metamodelica::Ref<Graphviz::Node>;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    expt = dumpExpGraphviz(metamodelica::AsArg::as_arg(&exp));
                    itert = dumpExpGraphviz(metamodelica::AsArg::as_arg(&iterexp));
                    Ok(metamodelica::Ref::new(Graphviz::Node::LNODE { type_: literal!("REDUCTION"), labelLst: list![fs.clone()], attributes: metamodelica::nil(), children: list![expt.clone(), itert.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(Graphviz::Node::NODE { type_: literal!("#UNKNOWN EXPRESSION# ----eeestr "), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

pub fn dumpExpStr(mut inExp: metamodelica::Ref<DAE::Exp>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inExp.clone(), inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: x }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = intString(x.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("ICONST "), s.clone(), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = realString(r.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RCONST "), s.clone(), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s = (*s).clone();
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = System::escapedString(s.clone(), true);
                    res_str = stringAppendList(list![gen_str.clone(), literal!("SCONST "), literal!("\""), s.clone(), literal!("\"\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("BCONST "), literal!("false"), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("BCONST "), literal!("true"), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CLKCONST { clk }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = clockKindString(metamodelica::AsArg::as_arg(&clk))?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("CLKCONST "), s.clone(), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { name: fcn, index: i }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    let mut istr: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    istr = intString(i.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("ENUM_LITERAL "), s.clone(), literal!(" ["), istr.clone(), literal!("]"), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, ty }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    let mut tpStr: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    tpStr = TypesDump::unparseType(ty.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("CREF "), s.clone(), literal!(" CREFTYPE:"), tpStr.clone(), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut sym: ArcStr;
                    let mut lt: ArcStr;
                    let mut rt: ArcStr;
                    let mut r#str: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = debugBinopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    tp = Expression::r#typeof(exp.clone())?;
                    r#str = TypesDump::unparseType(tp.clone())?;
                    lt = dumpExpStr(e1.clone(), new_level1)?;
                    rt = dumpExpStr(e2.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("BINARY "), sym.clone(), literal!(" "), r#str.clone(), literal!("\n"), lt.clone(), rt.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: op, exp: e }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut sym: ArcStr;
                    let mut ct: ArcStr;
                    let mut r#str: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    sym = unaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("expType:")); __mm_s.push_str(&*TypesDump::unparseType(Expression::r#typeof(e.clone())?)?); __mm_s.push_str(&*literal!(" optype:")); __mm_s.push_str(&*TypesDump::unparseType(Expression::typeofOp(metamodelica::AsArg::as_arg(&op)))?); ArcStr::from(__mm_s) };
                    res_str = stringAppendList(list![gen_str.clone(), literal!("UNARY "), sym.clone(), literal!(" "), r#str.clone(), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut sym: ArcStr;
                    let mut lt: ArcStr;
                    let mut rt: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = lbinopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    lt = dumpExpStr(e1.clone(), new_level1)?;
                    rt = dumpExpStr(e2.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("LBINARY "), sym.clone(), literal!("\n"), lt.clone(), rt.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { operator: op, exp: e }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut sym: ArcStr;
                    let mut ct: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    sym = lunaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("LUNARY "), sym.clone(), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut sym: ArcStr;
                    let mut lt: ArcStr;
                    let mut rt: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    sym = relopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    lt = dumpExpStr(e1.clone(), new_level1)?;
                    rt = dumpExpStr(e2.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RELATION "), sym.clone(), literal!("\n"), lt.clone(), rt.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: t, expElse: f }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut ct: ArcStr;
                    let mut tt: ArcStr;
                    let mut ft: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    let mut new_level3: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    new_level3 = level.clone() + 1;
                    ct = dumpExpStr(cond.clone(), new_level1)?;
                    tt = dumpExpStr(t.clone(), new_level2)?;
                    ft = dumpExpStr(f.clone(), new_level3)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("IFEXP "), literal!("\n"), ct.clone(), tt.clone(), ft.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut fs: ArcStr;
                    let mut argnodes_1: ArcStr;
                    let mut new_level1: i32;
                    let mut argnodes: metamodelica::List<ArcStr>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), &dumpExpStr, new_level1)?;
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("CALL "), fs.clone(), literal!("\n"), argnodes_1.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path: fcn, expList: args, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut fs: ArcStr;
                    let mut argnodes_1: ArcStr;
                    let mut new_level1: i32;
                    let mut argnodes: metamodelica::List<ArcStr>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), &dumpExpStr, new_level1)?;
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("CALL "), fs.clone(), literal!("\n"), argnodes_1.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: es, scalar: b, ty: tp }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    let mut nodes_1: ArcStr;
                    let mut tpStr: ArcStr;
                    let mut new_level1: i32;
                    let mut nodes: metamodelica::List<ArcStr>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    nodes = List::map1(es.clone(), &dumpExpStr, new_level1)?;
                    nodes_1 = stringAppendList(nodes.clone());
                    s = boolString(b.clone());
                    tpStr = TypesDump::unparseType(tp.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("ARRAY scalar:"), s.clone(), literal!(" tp: "), tpStr.clone(), literal!("\n"), nodes_1.clone()]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: es }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut nodes_1: ArcStr;
                    let mut new_level1: i32;
                    let mut nodes: metamodelica::List<ArcStr>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    nodes = List::map1(es.clone(), &dumpExpStr, new_level1)?;
                    nodes_1 = stringAppendList(nodes.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("TUPLE "), nodes_1.clone(), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: lstes, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    s = stringDelimitList(List::map1(lstes.clone(), &printRowStr, literal!("\""))?, literal!("},{"));
                    res_str = stringAppendList(list![gen_str.clone(), literal!("MATRIX "), literal!("\n"), literal!("{{"), s.clone(), literal!("}}"), literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { start, step: None, stop, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut t1: ArcStr;
                    let mut t2: ArcStr;
                    let mut t3: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    t1 = dumpExpStr(start.clone(), new_level1)?;
                    t2 = literal!(":");
                    t3 = dumpExpStr(stop.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RANGE "), literal!("\n"), t1.clone(), t2.clone(), t3.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RANGE { start, step: Some(step), stop, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut t1: ArcStr;
                    let mut t2: ArcStr;
                    let mut t3: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    let mut new_level3: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    new_level3 = level.clone() + 1;
                    t1 = dumpExpStr(start.clone(), new_level1)?;
                    t2 = dumpExpStr(step.clone(), new_level2)?;
                    t3 = dumpExpStr(stop.clone(), new_level3)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RANGE "), literal!("\n"), t1.clone(), t2.clone(), t3.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut ct: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("CAST "), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, sub: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: Deref @ metamodelica::ListNode::Nil } }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    let mut ct: ArcStr;
                    let mut istr: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    istr = intString(i.clone());
                    s = stringAppendList(list![literal!("["), istr.clone(), literal!("]")]);
                    res_str = stringAppendList(list![gen_str.clone(), literal!("ASUB "), s.clone(), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut ct: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("ASUB "), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { exp: cr, sz: Some(dim) }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut crt: ArcStr;
                    let mut dimt: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    crt = dumpExpStr(cr.clone(), new_level1)?;
                    dimt = dumpExpStr(dim.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("SIZE "), literal!("\n"), crt.clone(), dimt.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SIZE { exp: cr, sz: None }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut crt: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    crt = dumpExpStr(cr.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("SIZE "), literal!("\n"), crt.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { .. }, expr: exp, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: iterexp, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut expt: ArcStr;
                    let mut itert: ArcStr;
                    let mut new_level1: i32;
                    let mut new_level2: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    new_level2 = level.clone() + 1;
                    expt = dumpExpStr(exp.clone(), new_level1)?;
                    itert = dumpExpStr(iterexp.clone(), new_level2)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("REDUCTION "), literal!("\n"), expt.clone(), itert.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { path: fcn, exps: args, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut fs: ArcStr;
                    let mut argnodes_1: ArcStr;
                    let mut new_level1: i32;
                    let mut argnodes: metamodelica::List<ArcStr>;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    new_level1 = level.clone() + 1;
                    argnodes = List::map1(args.clone(), &dumpExpStr, new_level1)?;
                    argnodes_1 = stringAppendList(argnodes.clone());
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RECORD "), fs.clone(), literal!("\n"), argnodes_1.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RSUB { exp: e, ix: i, fieldName: fs, ty: tp }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut s: ArcStr;
                    let mut ct: ArcStr;
                    let mut istr: ArcStr;
                    let mut tpStr: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    istr = intString(i.clone());
                    s = stringAppendList(list![literal!("["), istr.clone(), literal!("]")]);
                    tpStr = TypesDump::unparseType(tp.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("RSUB "), s.clone(), literal!(" fieldName: "), fs.clone(), literal!(" tp: "), tpStr.clone(), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BOX { exp: e }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut ct: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("BOX "), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: e, .. }, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    let mut ct: ArcStr;
                    let mut new_level1: i32;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    new_level1 = level.clone() + 1;
                    ct = dumpExpStr(e.clone(), new_level1)?;
                    res_str = stringAppendList(list![gen_str.clone(), literal!("UNBOX "), literal!("\n"), ct.clone(), literal!("")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, level) => {
                    let mut gen_str: ArcStr;
                    let mut res_str: ArcStr;
                    gen_str = genStringNTime(literal!("   |"), level.clone())?;
                    res_str = stringAppendList(list![gen_str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" UNKNOWN EXPRESSION (")); __mm_s.push_str(&*printExpTypeStr(&inExp)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }, literal!("\n")]);
                    Ok(res_str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

fn genStringNTime(mut inString: ArcStr, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match (inString, inInteger) {
        (_, 0) => {
            literal!("")
        }
        (mut r#str, mut level) => {
            let mut new_str: ArcStr;
            let mut res_str: ArcStr;
            let mut new_level: i32;
            new_level = level + -1;
            new_str = genStringNTime(r#str.clone(), new_level)?;
            res_str = stringAppend(r#str, new_str);
            res_str
        }
    });
    Ok(outString)
}

pub fn dumpExp(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = dumpExpStr(exp, 0)?;
    metamodelica::print(r#str);
    metamodelica::print(literal!("--------------------\n"));
    Ok(())
}

fn printExpIfDiff(mut e1: metamodelica::Ref<DAE::Exp>, mut e2: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = if (ExpressionBasics::expEqual(&e1, e2.clone())?) {
        literal!("")
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e1)?);
            __mm_s.push_str(&*literal!(" =!= "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e2)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
    };
    Ok(s)
}

pub(crate) fn printArraySizes<'__b>(mut inLst: &'__b metamodelica::List<Option<i32>>) -> ArcStr {
    '__tco: loop {
        ::match_deref::match_deref! { match inLst {
            Deref @ metamodelica::ListNode::Nil => {
                return literal!("")
            },
            Deref @ metamodelica::ListNode::Cons { head: Some(x), tail: lst } => {
                let mut s: ArcStr;
                let mut s2: ArcStr;
                s = printArraySizes(lst);
                s2 = intString(x.clone());
                return stringAppendList(list![s2, s])
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: lst } => {
                let mut s: ArcStr;
                { inLst = lst; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn typeOfString(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Expression::r#typeof(inExp)?;
    r#str = TypesDump::unparseType(ty)?;
    Ok(r#str)
}

pub(crate) fn debugPrintComponentRefExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
                    Ok(ComponentReference::debugPrintComponentRefTypeStr(metamodelica::AsArg::as_arg(&cr))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl } => {
                    let mut s1: ArcStr;
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringAppendList(List::map(expl.clone(), &debugPrintComponentRefExp)?)); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
                    Ok(s1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(ExpressionBasics::printExpStr(inExp.clone())?)
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

pub fn dimensionIntString(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**dim {
        DAE::Dimension::DIM_UNKNOWN { .. } => {
            literal!(":")
        }
        DAE::Dimension::DIM_ENUM { size, .. } => intString(size.clone()),
        DAE::Dimension::DIM_BOOLEAN { .. } => {
            literal!("1")
        }
        DAE::Dimension::DIM_INTEGER { integer: x } => intString(x.clone()),
        DAE::Dimension::DIM_EXP { exp: e } => {
            let mut s: ArcStr;
            s = ExpressionBasics::printExpStr(e.clone())?;
            s
        }
    });
    Ok(r#str)
}

pub(crate) fn dumpExpWithTitle(mut title: ArcStr, mut exp: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = dumpExpStr(exp, 0)?;
    metamodelica::print(title);
    metamodelica::print(r#str);
    metamodelica::print(literal!("\n"));
    Ok(())
}

pub(crate) fn printSubscript(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<()> {
    let () = (match &**inSubscript {
        DAE::Subscript::WHOLEDIM { .. } => {
            Print::printBuf(literal!(":"))?;
            ()
        }
        DAE::Subscript::INDEX { exp: e1 } => {
            printExp(e1.clone())?;
            ()
        }
        DAE::Subscript::SLICE { exp: e1 } => {
            printExp(e1.clone())?;
            ()
        }
        DAE::Subscript::WHOLE_NONEXP { exp: e1 } => {
            Print::printBuf(literal!("1:"))?;
            printExp(e1.clone())?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn printExp(mut e: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    Tpl::tplPrint2(
        (std::sync::Arc::new(
            move |__a0: Tpl::Text, __a1: metamodelica::Ref<DAE::Exp>, __a2: ArcStr| {
                ExpressionDumpTpl::dumpExp(__a0, &__a1, &__a2)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(Tpl::Text, metamodelica::Ref<DAE::Exp>, ArcStr) -> Result<Tpl::Text> + 'static,
            >),
        e,
        literal!("\""),
    )?;
    Ok(())
}

pub fn parenthesize(
    mut inString1: ArcStr,
    mut inInteger2: i32,
    mut inInteger3: i32,
    mut rightOpParenthesis: bool,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match (inString1, inInteger2, inInteger3, rightOpParenthesis) {
        (mut r#str, mut pparent, mut pexpr, _) if (pparent > pexpr) => {
            let mut str_1: ArcStr;
            str_1 = stringAppendList(list![literal!("("), r#str, literal!(")")]);
            str_1
        }
        (mut r#str, mut pparent, mut pexpr, true) if (pparent == pexpr) => {
            let mut str_1: ArcStr;
            str_1 = stringAppendList(list![literal!("("), r#str, literal!(")")]);
            str_1
        }
        (mut r#str, _, _, _) => r#str,
    });
    outString
}

pub fn clockKindString(mut inClockKind: &metamodelica::Ref<DAE::ClockKind>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inClockKind {
        DAE::ClockKind::INFERRED_CLOCK { .. } => {
            literal!("Clock()")
        }
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter,
            resolution,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Clock("));
            __mm_s.push_str(&*dumpExpStr(intervalCounter.clone(), 0)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*dumpExpStr(resolution.clone(), 0)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::REAL_CLOCK { interval } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Clock("));
            __mm_s.push_str(&*dumpExpStr(interval.clone(), 0)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition,
            startInterval,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Clock("));
            __mm_s.push_str(&*dumpExpStr(condition.clone(), 0)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*dumpExpStr(startInterval.clone(), 0)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
        DAE::ClockKind::SOLVER_CLOCK { c, solverMethod } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Clock("));
            __mm_s.push_str(&*dumpExpStr(c.clone(), 0)?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*dumpExpStr(solverMethod.clone(), 0)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(outString)
}

pub(crate) fn constraintDTtoString(mut con: &metamodelica::Ref<DAE::Constraint>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut c: metamodelica::Ref<DAE::Exp>;
    let mut localCon: bool;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*con)) {
        Deref @ DAE::Constraint::CONSTRAINT_DT { constraint: __pa0, localCon: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    c = metamodelica::Own::own(__pa0);
    localCon = metamodelica::Own::own(__pa1);
    r#str = ExpressionBasics::printExpStr(c)?;
    r#str = if (localCon) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(" (local)"));
            ArcStr::from(__mm_s)
        }
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(" (global)"));
            ArcStr::from(__mm_s)
        }
    };
    Ok(r#str)
}

pub fn constraintDTlistToString(
    mut cons: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut delim: &ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut con: metamodelica::Ref<DAE::Constraint> =
        <metamodelica::Ref<DAE::Constraint> as ::std::default::Default>::default();
    for mut con in &**cons {
        let mut con = con.clone();
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*delim);
            __mm_s.push_str(&*constraintDTtoString(&con)?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub fn patternStr(mut pattern: &metamodelica::Ref<DAE::Pattern>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match pattern {
        Deref @ DAE::Pattern::PAT_WILD { .. } => {
            literal!("_")
        },
        Deref @ DAE::Pattern::PAT_AS { id, pat: Deref @ DAE::Pattern::PAT_WILD { .. }, .. } => {
            id.clone()
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat: Deref @ DAE::Pattern::PAT_WILD { .. } } => {
            id.clone()
        },
        Deref @ DAE::Pattern::PAT_SOME { pat } => {
            r#str = patternStr(pat)?;
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SOME(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: pats } => {
            r#str = stringDelimitList(List::map(pats.clone(), &move |__a0: metamodelica::Ref<DAE::Pattern>| patternStr(&__a0))?, literal!(","));
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: pats } => {
            r#str = stringDelimitList(List::map(pats.clone(), &move |__a0: metamodelica::Ref<DAE::Pattern>| patternStr(&__a0))?, literal!(","));
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Pattern::PAT_CALL { name, patterns: pats, .. } => {
            let mut id: ArcStr;
            id = AbsynUtil::pathString(name.clone(), literal!("."), true, false)?;
            r#str = stringDelimitList(List::map(pats.clone(), &move |__a0: metamodelica::Ref<DAE::Pattern>| patternStr(&__a0))?, literal!(","));
            stringAppendList(list![id, literal!("("), r#str, literal!(")")])
        },
        Deref @ DAE::Pattern::PAT_CALL_NAMED { name, patterns: namedpats } => {
            let mut fields: metamodelica::List<ArcStr>;
            let mut patsStr: metamodelica::List<ArcStr>;
            let mut id: ArcStr;
            id = AbsynUtil::pathString(name.clone(), literal!("."), true, false)?;
            fields = List::map(namedpats.clone(), &fnptr!(Util::tuple32, _))?;
            patsStr = List::map1r(List::mapMap(namedpats.clone(), &fnptr!(Util::tuple31, _), &move |__a0: metamodelica::Ref<DAE::Pattern>| patternStr(&__a0))?, &fnptr!(stringAppend, ArcStr, ArcStr), literal!("="))?;
            r#str = stringDelimitList(List::threadMap(fields, patsStr, &fnptr!(stringAppend, ArcStr, ArcStr))?, literal!(","));
            stringAppendList(list![id, literal!("("), r#str, literal!(")")])
        },
        Deref @ DAE::Pattern::PAT_CONS { head, tail } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*patternStr(head)?); __mm_s.push_str(&*literal!("::")); __mm_s.push_str(&*patternStr(tail)?); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Pattern::PAT_CONSTANT { exp, .. } => {
            ExpressionBasics::printExpStr(exp.clone())?
        },
        Deref @ DAE::Pattern::PAT_AS { id, pat, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" as ")); __mm_s.push_str(&*patternStr(pat)?); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id, pat } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!(" as ")); __mm_s.push_str(&*patternStr(pat)?); ArcStr::from(__mm_s) }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("ExpressionDump.patternStr not implemented correctly")])?;
            literal!("*PATTERN*")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#str)
}
