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
use openmodelica_backend::BackendDump;
use openmodelica_backend::BackendVariable;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn dumpMmaDAEStr(
    mut inTuple: &(
        BackendDAE::Variables,
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (::match_deref::match_deref! { match &(inTuple) {
        (vars, knvars, eqns, ieqns) => {
            let mut allVarStr: ArcStr;
            let mut s1_1: ArcStr;
            let mut s1_2: ArcStr;
            let mut s1_3: ArcStr;
            let mut s1_4: ArcStr;
            let mut s1_5: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut params: metamodelica::List<ArcStr>;
            let mut inputs: metamodelica::List<ArcStr>;
            let mut states: metamodelica::List<ArcStr>;
            let mut algs: metamodelica::List<ArcStr>;
            let mut outputs: metamodelica::List<ArcStr>;
            (states, algs, outputs, _) = printMmaVarsStr(vars.clone())?;
            (params, inputs) = printMmaParamsStr(metamodelica::AsArg::as_arg(&knvars))?;
            s1_1 = Util::stringDelimitListNonEmptyElts(states, literal!(","))?;
            s1_2 = Util::stringDelimitListNonEmptyElts(algs, literal!(","))?;
            s1_3 = Util::stringDelimitListNonEmptyElts(outputs, literal!(","))?;
            s1_4 = Util::stringDelimitListNonEmptyElts(inputs, literal!(","))?;
            s1_5 = Util::stringDelimitListNonEmptyElts(params, literal!(","))?;
            allVarStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s1_1); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_2); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_3); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_4); __mm_s.push_str(&*literal!("},{")); __mm_s.push_str(&*s1_5); __mm_s.push_str(&*literal!("}}")); ArcStr::from(__mm_s) };
            s3 = printMmaEqnsStr(eqns.clone(), (vars.clone(), knvars.clone()))?;
            s4 = printMmaEqnsStr(ieqns.clone(), (vars.clone(), knvars.clone()))?;
            res = stringAppendList(list![literal!("{"), allVarStr, literal!(","), s3, literal!(","), s4, literal!("}")]);
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn printMmaEqnsStr(
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inTuple: (BackendDAE::Variables, BackendDAE::Variables),
) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (::match_deref::match_deref! { match &(inEqns) {
        eqns => {
            let mut s1: ArcStr;
            s1 = Util::stringDelimitListNonEmptyElts(List::map1(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendDAE::Variables, BackendDAE::Variables)| printMmaEqnStr(&__a0, &__a1), inTuple)?, literal!(","))?;
            res = stringAppendList(list![literal!("{"), s1, literal!("}")]);
            res
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn printMmaEqnStr(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: &(BackendDAE::Variables, BackendDAE::Variables),
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match &((&**eqn, inTuple)) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, (vars, knvars)) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpMmaStr(e1.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            s2 = printExpMmaStr(e2.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            r#str = stringAppendList(list![s1, literal!("=="), s2]);
            r#str
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, (vars, knvars)) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printComponentRefMmaStr(cr.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            s2 = printExpMmaStr(e2.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            r#str = stringAppendList(list![s1, literal!("=="), s2]);
            r#str
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, (vars, knvars)) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpMmaStr(e1.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            s2 = printExpMmaStr(e2.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            r#str = stringAppendList(list![s1, literal!("=="), s2]);
            r#str
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, .. }, (vars, knvars)) => {
            let mut s1: ArcStr;
            s1 = printExpMmaStr(e1.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            r#str = stringAppendList(list![s1, literal!("== 0")]);
            r#str
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }, _) => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"Algorithm\",\"")); __mm_s.push_str(&*escapeMmaString(dumpSingleAlgorithmStr(metamodelica::AsArg::as_arg(&alg))?)?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) };
            r#str
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: whenEq, .. }, _) => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"When\",\"")); __mm_s.push_str(&*escapeMmaString(BackendDump::whenEquationString(metamodelica::AsArg::as_arg(&whenEq), true)?)?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) };
            r#str
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }, (vars, knvars)) => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = printExpMmaStr(e1.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            s2 = printExpMmaStr(e2.clone(), metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&knvars))?;
            r#str = stringAppendList(list![s1, literal!("=="), s2]);
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

/* Printing of equations and variables on Mathematica format*/
pub(crate) fn printExpMmaStr(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut vars: &BackendDAE::Variables,
    mut knvars: &BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = printExp2MmaStr(e, vars, knvars)?;
    Ok(s)
}

fn printExp2MmaStr(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut vars: &BackendDAE::Variables,
    mut knvars: &BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
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
                Deref @ DAE::Exp::RCONST { real: x } => {
                    let mut s: ArcStr;
                    let mut x2: metamodelica::Real;
                    x2 = intReal(((x.clone()).0.floor() as i32));
                    let true = (realEq(x2, x.clone())) else { return Err("pattern mismatch") };
                    s = intString(((x.clone()).0.floor() as i32));
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: x } => {
                    let mut s: ArcStr;
                    s = realString(x.clone());
                    s = stringAppendList(list![literal!("ToExpression[StringReplace[\""), s.clone(), literal!("\",\"e\"->\"*1.0*10^\"]]")]);
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s_1 = stringAppend(literal!("\""), s.clone());
                    s_2 = stringAppend(s_1.clone(), literal!("\""));
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: false } => {
                    Ok(literal!("False"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: true } => {
                    Ok(literal!("True"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut s: ArcStr;
                    s = printComponentRefMmaStr(cr.clone(), vars, knvars)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut sym: ArcStr;
                    let mut s2: ArcStr;
                    let mut p: i32;
                    let mut p1: i32;
                    let mut p2: i32;
                    let mut s1: ArcStr;
                    sym = ExpressionDump::binopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2MmaStr(e1.clone(), vars, knvars)?;
                    s2 = printExp2MmaStr(e2.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                    p2 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e2));
                    s1_1 = ExpressionDump::parenthesize(s1.clone(), p1, p, false);
                    s2_1 = ExpressionDump::parenthesize(s2.clone(), p2, p, true);
                    s = stringAppend(s1_1.clone(), sym.clone());
                    s_1 = stringAppend(s.clone(), s2_1.clone());
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sym: ArcStr;
                    let mut p: i32;
                    let mut p1: i32;
                    sym = ExpressionDump::unaryopSymbol(metamodelica::AsArg::as_arg(&op))?;
                    s = printExp2MmaStr(e1.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                    s_1 = ExpressionDump::parenthesize(s.clone(), p1, p, true);
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
                e @ Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut sym: ArcStr;
                    let mut s2: ArcStr;
                    let mut p: i32;
                    let mut p1: i32;
                    let mut p2: i32;
                    let mut s1: ArcStr;
                    sym = lbinopSymbolMma(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2MmaStr(e1.clone(), vars, knvars)?;
                    s2 = printExp2MmaStr(e2.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                    p2 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e2));
                    s1_1 = ExpressionDump::parenthesize(s1.clone(), p1, p, false);
                    s2_1 = ExpressionDump::parenthesize(s2.clone(), p2, p, true);
                    s = stringAppend(s1_1.clone(), sym.clone());
                    s_1 = stringAppend(s.clone(), s2_1.clone());
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sym: ArcStr;
                    let mut p: i32;
                    let mut p1: i32;
                    sym = lunaryopSymbolMma(metamodelica::AsArg::as_arg(&op))?;
                    s = printExp2MmaStr(e1.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                    s_1 = ExpressionDump::parenthesize(s.clone(), p1, p, true);
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
                e @ Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s1_1: ArcStr;
                    let mut s2_1: ArcStr;
                    let mut sym: ArcStr;
                    let mut s2: ArcStr;
                    let mut p: i32;
                    let mut p1: i32;
                    let mut s1: ArcStr;
                    sym = relopSymbolMma(metamodelica::AsArg::as_arg(&op))?;
                    s1 = printExp2MmaStr(e1.clone(), vars, knvars)?;
                    s2 = printExp2MmaStr(e2.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    p1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                    s1_1 = ExpressionDump::parenthesize(s1.clone(), p1, p, false);
                    s2_1 = ExpressionDump::parenthesize(s2.clone(), p1, p, true);
                    s = stringAppend(s1_1.clone(), sym.clone());
                    s_1 = stringAppend(s.clone(), s2_1.clone());
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: c, expThen: t, expElse: f } => {
                    let mut ifstr: ArcStr;
                    let mut thenstr: ArcStr;
                    let mut elsestr: ArcStr;
                    let mut res: ArcStr;
                    ifstr = printExp2MmaStr(c.clone(), vars, knvars)?;
                    thenstr = printExp2MmaStr(t.clone(), vars, knvars)?;
                    elsestr = printExp2MmaStr(f.clone(), vars, knvars)?;
                    res = stringAppendList(list![literal!("If[ "), ifstr.clone(), literal!(", "), thenstr.clone(), literal!(" ,"), elsestr.clone(), literal!("]")]);
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s_1 = printExpMmaStr(e.clone(), vars, knvars)?;
                    s_2 = stringAppendList(list![literal!("D["), s_1.clone(), literal!(",\\[FormalT]]")]);
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Modelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Math", path } }, expLst, attr: call_attr } => {
                    let mut s: ArcStr;
                    s = printExp2MmaStr(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: call_attr.clone() }), vars, knvars)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Modelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Math", path } } }, expLst, attr: call_attr } => {
                    let mut s: ArcStr;
                    s = printExp2MmaStr(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: call_attr.clone() }), vars, knvars)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: fname }, expLst, .. } => {
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut s1: ArcStr;
                    s1 = printBuiltinMmaFunc(metamodelica::AsArg::as_arg(&fname))?;
                    s_1 = stringDelimitList(List::map2(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s_2 = stringAppendList(list![s1.clone(), literal!("["), s_1.clone(), literal!("]")]);
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut s_11: ArcStr;
                    s_1 = printExpMmaStr(e1.clone(), vars, knvars)?;
                    s_11 = printExpMmaStr(e2.clone(), vars, knvars)?;
                    s_2 = stringAppendList(list![literal!("ArcTan["), s_1.clone(), literal!(","), s_11.clone(), literal!("]")]);
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s_1 = printExpMmaStr(e1.clone(), vars, knvars)?;
                    s_2 = stringAppendList(list![literal!("Log["), s_1.clone(), literal!(",10]")]);
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    fs = translateKnownMmaFuncs(&fs)?;
                    argstr = stringDelimitList(List::map2(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s = stringAppend(fs.clone(), literal!("["));
                    s_1 = stringAppend(s.clone(), argstr.clone());
                    s_2 = stringAppend(s_1.clone(), literal!("]"));
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
                    let mut s_2: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    argstr = stringDelimitList(List::map2(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s_2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FunctionCall[\"")); __mm_s.push_str(&*fs); __mm_s.push_str(&*literal!("\"][")); __mm_s.push_str(&*argstr); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { path: fcn, exps: args, .. } => {
                    let mut s_2: ArcStr;
                    let mut fs: ArcStr;
                    let mut argstr: ArcStr;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    argstr = stringDelimitList(List::map2(args.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s_2 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FunctionCall[\"")); __mm_s.push_str(&*fs); __mm_s.push_str(&*literal!("\"][")); __mm_s.push_str(&*argstr); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: es, .. } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = stringDelimitList(List::map2(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s_1 = stringAppend(literal!("{"), s.clone());
                    s_2 = stringAppend(s_1.clone(), literal!("}"));
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: es } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = stringDelimitList(List::map2(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExpMmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(","));
                    s_1 = stringAppend(literal!("{"), s.clone());
                    s_2 = stringAppend(s_1.clone(), literal!("}"));
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix, .. } => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = stringDelimitList(List::map2(matrix.clone(), &printRowMmaStr, vars.clone(), knvars.clone())?, literal!("},{"));
                    s_1 = stringAppend(literal!("{{"), s.clone());
                    s_2 = stringAppend(s_1.clone(), literal!("}}"));
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::RANGE { start, step: None, stop, .. } => {
                    let mut s1_1: ArcStr;
                    let mut s3: ArcStr;
                    let mut s3_1: ArcStr;
                    let mut s_3: ArcStr;
                    let mut p: i32;
                    let mut pstart: i32;
                    let mut pstop: i32;
                    let mut s1: ArcStr;
                    s1 = printExp2MmaStr(start.clone(), vars, knvars)?;
                    s3 = printExp2MmaStr(stop.clone(), vars, knvars)?;
                    p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                    pstart = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&start));
                    pstop = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&stop));
                    s1_1 = ExpressionDump::parenthesize(s1.clone(), pstart, p, false);
                    s3_1 = ExpressionDump::parenthesize(s3.clone(), pstop, p, false);
                    s_3 = stringAppendList(list![literal!("Range["), s1_1.clone(), literal!(","), s3_1.clone(), literal!("]")]);
                    Ok(s_3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { start, step: Some(step), stop, .. } => {
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s_5: ArcStr;
                    s2 = printExp2MmaStr(start.clone(), vars, knvars)?;
                    s3 = printExp2MmaStr(step.clone(), vars, knvars)?;
                    s4 = printExp2MmaStr(stop.clone(), vars, knvars)?;
                    s_5 = stringAppendList(list![literal!("Range["), s2.clone(), literal!(","), s4.clone(), literal!(","), s3.clone(), literal!("]")]);
                    Ok(s_5.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::ICONST { integer: ival } } => {
                    let mut res: ArcStr;
                    res = intString(ival.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::ICONST { integer: ival } } } => {
                    let mut res: ArcStr;
                    let mut res2: ArcStr;
                    res = intString(ival.clone());
                    res2 = stringAppend(literal!("-"), res.clone());
                    Ok(res2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e } => {
                    let mut s: ArcStr;
                    s = printExpMmaStr(e.clone(), vars, knvars)?;
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        e @ Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
                            let mut s1_1: ArcStr;
                            let mut s4: ArcStr;
                            let mut s_4: ArcStr;
                            let mut p: i32;
                            let mut pe1: i32;
                            let mut s1: ArcStr;
                            let mut ae1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            ae1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            p = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e));
                            pe1 = ExpressionDump::expPriority(metamodelica::AsArg::as_arg(&e1));
                            s1 = printExp2MmaStr(e1.clone(), vars, knvars)?;
                            s1_1 = ExpressionDump::parenthesize(s1.clone(), pe1, p, false);
                            s4 = stringDelimitList(List::map2(ae1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| printExp2MmaStr(__a0, &__a1, &__a2), vars.clone(), knvars.clone())?, literal!(", "));
                            s_4 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Index[")); __mm_s.push_str(&*s1_1); __mm_s.push_str(&*literal!(",{")); __mm_s.push_str(&*s4); __mm_s.push_str(&*literal!("}]")); ArcStr::from(__mm_s) };
                            Ok(s_4.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e, sz: Some(dim) } => {
                    let mut r#str: ArcStr;
                    let mut crstr: ArcStr;
                    let mut dimstr: ArcStr;
                    crstr = printExpMmaStr(e.clone(), vars, knvars)?;
                    dimstr = printExpMmaStr(dim.clone(), vars, knvars)?;
                    r#str = stringAppendList(list![literal!("Dimensions["), crstr.clone(), literal!("][["), dimstr.clone(), literal!("]]")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e, sz: None } => {
                    let mut r#str: ArcStr;
                    let mut crstr: ArcStr;
                    crstr = printExpMmaStr(e.clone(), vars, knvars)?;
                    r#str = stringAppendList(list![literal!("Dimensions["), crstr.clone(), literal!("]")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: fcn, .. }, expr: exp, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id, exp: iterexp, .. }, tail: _ } } => {
                    let mut fs: ArcStr;
                    let mut r#str: ArcStr;
                    let mut expstr: ArcStr;
                    let mut iterstr: ArcStr;
                    fs = AbsynUtil::pathString(fcn.clone(), literal!("."), true, false)?;
                    expstr = printExpMmaStr(exp.clone(), vars, knvars)?;
                    iterstr = printExpMmaStr(iterexp.clone(), vars, knvars)?;
                    r#str = stringAppendList(list![literal!("Table["), fs.clone(), literal!("["), expstr.clone(), literal!("],{"), id.clone(), literal!(", "), iterstr.clone(), literal!("}]")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: path, .. } => {
                    let mut r#str: ArcStr;
                    r#str = AbsynUtil::pathString(path.clone(), literal!("."), true, false)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"ModelicaName\",\"")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Missing[\"UnknownExpression\",\"")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\"]")); ArcStr::from(__mm_s) };
                    Ok(r#str.clone())
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

fn printComponentRefMmaStr(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
    mut knvars: &BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = 'mc: {
        let __mc_input = &*cr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", identType: _, subscriptLst: _ } => {
                    Ok(literal!("\\[FormalT]"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nameStr: ArcStr;
                    let mut res: ArcStr = res.clone();
                    BackendVariable::getVar(cr.clone(), vars)?;
                    nameStr = ComponentReferenceBasics::printComponentRefStr(&cr)?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$p"), literal!("."))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$lb"), literal!("["))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rb"), literal!("]"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$leftParentesis"), literal!("["))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rightParentesis"), literal!("]"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("("), literal!("["))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!(")"), literal!("]"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("_"), literal!("\\[UnderBracket]"))?;
                    nameStr = wrapInMember(nameStr.clone())?;
                    nameStr = addMissingForQuotedNames(nameStr.clone());
                    res = stringAppendList(list![nameStr.clone(), literal!("[\\[FormalT]]")]);
                    Ok((res.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nameStr: ArcStr;
                    let mut isInput: bool;
                    let mut isOutput: bool;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut res: ArcStr = res.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), knvars)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa0);
                    isInput = BackendVariable::isInput(&v);
                    isOutput = BackendVariable::isOutputVar(&v);
                    let true = (boolOr(isInput, isOutput)) else { return Err("pattern mismatch") };
                    nameStr = ComponentReferenceBasics::printComponentRefStr(&cr)?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$p"), literal!("."))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$lb"), literal!("["))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rb"), literal!("]"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$leftParentesis"), literal!("("))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rightParentesis"), literal!(")"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("_"), literal!("\\[UnderBracket]"))?;
                    nameStr = wrapInMember(nameStr.clone())?;
                    nameStr = addMissingForQuotedNames(nameStr.clone());
                    res = stringAppendList(list![nameStr.clone(), literal!("[\\[FormalT]]")]);
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
                    let mut nameStr: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    nameStr = ComponentReferenceBasics::printComponentRefStr(&cr)?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$p"), literal!("."))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$lb"), literal!("["))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rb"), literal!("]"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$leftParentesis"), literal!("("))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("$rightParentesis"), literal!(")"))?;
                    nameStr = System::stringReplace(nameStr.clone(), literal!("_"), literal!("\\[UnderBracket]"))?;
                    nameStr = wrapInMember(nameStr.clone())?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(res)
}

fn wrapInMember(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut s3: ArcStr;
    s3 = System::stringReplace(r#str, literal!("."), literal!("\\[UpPointer]"))?;
    outStr = s3;
    Ok(outStr)
}

fn addMissingForQuotedNames(mut name: ArcStr) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = 'mc: {
        let __mc_input = name.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut res: ArcStr = res.clone();
            let false = (-1 == System::stringFind(name.clone(), literal!("'"))?) else {
                return Err("pattern mismatch");
            };
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Missing[\"QuotedName\",\""));
                __mm_s.push_str(&*System::stringReplace(name.clone(), literal!("\\"), literal!("\\\\"))?);
                __mm_s.push_str(&*literal!("\"]"));
                ArcStr::from(__mm_s)
            };
            Ok((res.clone(), res.clone()))
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(name.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn lbinopSymbolMma(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::AND { ty: _ } => literal!(" && "),
        DAE::Operator::OR { ty: _ } => literal!(" || "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn lunaryopSymbolMma(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::NOT { ty: _ } => literal!(" ! "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn relopSymbolMma(mut inOperator: &DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inOperator.clone() {
        DAE::Operator::LESS { .. } => literal!(" < "),
        DAE::Operator::LESSEQ { .. } => literal!(" <= "),
        DAE::Operator::GREATER { .. } => literal!(" > "),
        DAE::Operator::GREATEREQ { .. } => literal!(" >= "),
        DAE::Operator::EQUAL { .. } => literal!(" == "),
        DAE::Operator::NEQUAL { .. } => literal!(" != "),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn printBuiltinMmaFunc(mut modelicaFuncName: &ArcStr) -> Result<ArcStr> {
    let mut mathematicaFuncName: ArcStr;
    mathematicaFuncName = (::match_deref::match_deref! { match &(modelicaFuncName.clone()) {
        Deref @ "sqrt" => literal!("Sqrt"),
        Deref @ "abs" => literal!("Abs"),
        Deref @ "sign" => literal!("Sign"),
        Deref @ "Integer" => literal!("IntegerPart"),
        Deref @ "div" => literal!("Rational"),
        Deref @ "max" => literal!("Max"),
        Deref @ "min" => literal!("Min"),
        Deref @ "mod" => literal!("Quotient"),
        Deref @ "rem" => literal!("Mod"),
        Deref @ "ceil" => literal!("Cieling"),
        Deref @ "floor" => literal!("Floor"),
        Deref @ "integer" => literal!("IntegerPart"),
        Deref @ "sin" => literal!("Sin"),
        Deref @ "cos" => literal!("Cos"),
        Deref @ "tan" => literal!("Tan"),
        Deref @ "asin" => literal!("ArcSin"),
        Deref @ "acos" => literal!("ArcCos"),
        Deref @ "atan" => literal!("ArcTan"),
        Deref @ "sinh" => literal!("Sinh"),
        Deref @ "cosh" => literal!("Cosh"),
        Deref @ "tanh" => literal!("Tanh"),
        Deref @ "exp" => literal!("Exp"),
        Deref @ "log" => literal!("Log"),
        _ => return Err("match: no arm matched"),
    } });
    Ok(mathematicaFuncName)
}

fn translateKnownMmaFuncs(mut func: &ArcStr) -> Result<ArcStr> {
    let mut mmaFunc: ArcStr;
    mmaFunc = (::match_deref::match_deref! { match &(func.clone()) {
        Deref @ "sin" => literal!("Sin"),
        Deref @ "Modelica.Math.sin" => literal!("Sin"),
        Deref @ "cos" => literal!("Cos"),
        Deref @ "Modelica.Math.cos" => literal!("Cos"),
        Deref @ "tan" => literal!("Tan"),
        Deref @ "Modelica.Math.tan" => literal!("Tan"),
        Deref @ "exp" => literal!("Exp"),
        Deref @ "Modelica.Math.exp" => literal!("Exp"),
        _ => return Err("match: no arm matched"),
    } });
    Ok(mmaFunc)
}

fn printRowMmaStr(
    mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut vars: BackendDAE::Variables,
    mut knvars: BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(
        List::map2(
            es,
            &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| {
                printExpMmaStr(__a0, &__a1, &__a2)
            },
            vars,
            knvars,
        )?,
        literal!(","),
    );
    Ok(s)
}

fn escapeMmaString(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = System::stringReplace(r#str, literal!("\""), literal!("\\\""))?;
    Ok(res)
}

fn dumpSingleAlgorithmStr(mut algs: &metamodelica::Ref<DAE::Algorithm>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**algs {
        DAE::Algorithm { statementLst: stmts } => {
            let mut r#str: ArcStr;
            let mut myStream: IOStream::IOStream;
            myStream = IOStream::create(literal!(""), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = DAEDump::dumpAlgorithmStream(
                &(metamodelica::Ref::new(DAE::Element::ALGORITHM {
                    algorithm_: metamodelica::Ref::new(DAE::Algorithm {
                        statementLst: stmts.clone(),
                    }),
                    source: DAE::emptyElementSource().clone(),
                })),
                myStream,
            );
            r#str = IOStream::string(&myStream)?;
            r#str
        }
    });
    Ok(outString)
}

pub(crate) fn printMmaVarsStr(
    mut vars: BackendDAE::Variables,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
)> {
    let mut states: metamodelica::List<ArcStr>;
    let mut algs: metamodelica::List<ArcStr>;
    let mut outputs: metamodelica::List<ArcStr>;
    let mut inputs: metamodelica::List<ArcStr>;
    (states, algs, outputs, inputs) = (match vars.clone() {
        _ => {
            let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            varLst = BackendVariable::varList(&vars)?;
            varLst = varLst.reverse();
            states = List::map2(
                varLst.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>,
                       __a1: bool,
                       __a2: BackendDAE::Variables|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(printMmaVarStr(&__a0, __a1, &__a2))
                },
                true,
                vars.clone(),
            )?;
            algs = List::map2(
                varLst.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>,
                       __a1: bool,
                       __a2: BackendDAE::Variables|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(printMmaVarStr(&__a0, __a1, &__a2))
                },
                false,
                vars,
            )?;
            outputs = List::map(
                varLst.clone(),
                &fnptr!(printMmaOutputStr, metamodelica::Ref<BackendDAE::Var>),
            )?;
            inputs = List::map(varLst, &fnptr!(printMmaInputStr, metamodelica::Ref<BackendDAE::Var>))?;
            (states, algs, outputs, inputs)
        }
    });
    Ok((states, algs, outputs, inputs))
}

pub(crate) fn printMmaVarStr(
    mut v: &metamodelica::Ref<BackendDAE::Var>,
    mut selectKind: bool,
    mut allVars: &BackendDAE::Variables,
) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = 'mc: {
        let __mc_input = (&**v, selectKind);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$dummy", identType: Deref @ DAE::Type::T_UNKNOWN { .. }, subscriptLst: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::STATE { .. }, .. }, true) => {
                    let mut nameStr: ArcStr;
                    nameStr = printComponentRefMmaStr(name.clone(), allVars, &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::VARIABLE { .. }, .. }, false) => {
                    let mut nameStr: ArcStr;
                    nameStr = printComponentRefMmaStr(name.clone(), allVars, &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::DUMMY_DER { .. }, .. }, false) => {
                    let mut nameStr: ArcStr;
                    nameStr = printComponentRefMmaStr(name.clone(), allVars, &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::DUMMY_STATE { .. }, .. }, false) => {
                    let mut nameStr: ArcStr;
                    nameStr = printComponentRefMmaStr(name.clone(), allVars, &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::DISCRETE { .. }, .. }, false) => {
                    let mut nameStr: ArcStr;
                    nameStr = printComponentRefMmaStr(name.clone(), allVars, &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(nameStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    r#str
}

fn printMmaOutputStr(mut param: metamodelica::Ref<BackendDAE::Var>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = param;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ Deref @ BackendDAE::Var { varName: name @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil }, varDirection: DAE::VarDirection::OUTPUT { .. }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let true = (BackendVariable::isVarOnTopLevelAndOutput(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    r#str = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
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
    r#str
}

fn printMmaInputStr(mut param: metamodelica::Ref<BackendDAE::Var>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = param;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                v @ Deref @ BackendDAE::Var { varName: name @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: Deref @ metamodelica::ListNode::Nil }, varDirection: DAE::VarDirection::INPUT { .. }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    let true = (BackendVariable::isVarOnTopLevelAndInput(metamodelica::AsArg::as_arg(&v))) else { return Err("pattern mismatch") };
                    r#str = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
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
    r#str
}

pub(crate) fn printMmaParamsStr(
    mut knvars: &BackendDAE::Variables,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut params: metamodelica::List<ArcStr>;
    let mut inputs: metamodelica::List<ArcStr>;
    (params, inputs) = (match knvars.clone() {
        _ => {
            let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            varLst = BackendVariable::varList(knvars)?;
            params = List::map(
                varLst.clone(),
                &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(printMmaParamStr(&__a0))
                },
            )?;
            inputs = List::map(varLst, &fnptr!(printMmaInputStr, metamodelica::Ref<BackendDAE::Var>))?;
            (params, inputs)
        }
    });
    Ok((params, inputs))
}

fn printMmaParamStr(mut param: &metamodelica::Ref<BackendDAE::Var>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = &**param;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(exp), .. } => {
                    let mut expStr: ArcStr;
                    let mut paramStr: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    expStr = printExpMmaStr(exp.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    paramStr = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    r#str = stringAppendList(list![paramStr.clone(), literal!("->"), expStr.clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: None, values: val, .. } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut expStr: ArcStr;
                    let mut paramStr: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(getStartAttribute(val.clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    expStr = printExpMmaStr(exp.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    paramStr = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    r#str = stringAppendList(list![paramStr.clone(), literal!("->"), expStr.clone()]);
                    Ok((r#str.clone(), r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            r#str = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: None, values: val, .. } => {
                    let mut expStr: ArcStr;
                    let mut paramStr: ArcStr;
                    let mut r#str: ArcStr = r#str.clone();
                    ::match_deref::match_deref! { match &(getStartAttribute(val.clone())) {
                        None => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    expStr = printExpMmaStr(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    paramStr = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    r#str = stringAppendList(list![paramStr.clone(), literal!("->"), expStr.clone()]);
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
                Deref @ BackendDAE::Var { varName: name, varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
                    let mut paramStr: ArcStr;
                    paramStr = printComponentRefMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
                    Ok(paramStr.clone())
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
    r#str
}

fn getStartAttribute(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut out: Option<metamodelica::Ref<DAE::Exp>>;
    out = (::match_deref::match_deref! { match &(inVariableAttributesOption) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { start: e, .. }) => {
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { start: e, .. }) => {
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: e, .. }) => {
            e.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: e, .. }) => {
            e.clone()
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}
