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
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVariable;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub(crate) const matlabStringDelim: &'static str = "'";

pub fn writeAdjacencyMatrix(
    mut dlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut fileNamePrefix: ArcStr,
    mut flatModelicaStr: ArcStr,
) -> Result<ArcStr> {
    let mut fileName: ArcStr;
    fileName = (match flatModelicaStr {
        mut flatStr => {
            let mut file: ArcStr;
            let mut strIMatrix: ArcStr;
            let mut strVariables: ArcStr;
            let mut strEquations: ArcStr;
            let mut m: metamodelica::Array<metamodelica::List<ArcStr>>;
            file = stringAppend(fileNamePrefix, literal!("_imatrix.m"));
            m = adjacencyMatrix(dlow)?;
            strIMatrix = getAdjacencyMatrix(m.clone());
            strVariables = getVariables(dlow)?;
            strEquations = getEquations(dlow)?;
            strIMatrix = stringAppendList(list![
                strIMatrix,
                literal!("\n"),
                strVariables,
                literal!("\n\n\n"),
                strEquations,
                literal!("\n\n\n"),
                flatStr
            ]);
            System::writeFile(file.clone(), strIMatrix)?;
            file
        }
    });
    Ok(fileName)
}

pub(crate) fn getEquations(mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strEqs: ArcStr;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut ls1: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &((*inBackendDAE)) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, shared: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    syst = metamodelica::Own::own(__pa0);
    ls1 = List::map(
        BackendEquation::equationList(syst.orderedEqs.clone())?,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>| equationStr(&__a0),
    )?;
    strEqs = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("EqStr = {"));
        __mm_s.push_str(&*stringDelimitList(ls1, literal!(",")));
        __mm_s.push_str(&*literal!("};"));
        ArcStr::from(__mm_s)
    };
    Ok(strEqs)
}

pub(crate) fn equationStr(mut inEquation: &metamodelica::Ref<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inEquation {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![literal!("'"), s1, literal!(" = "), s2, literal!(";'")]);
            res
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![literal!("'"), s1, literal!(" = "), s2, literal!(";'")]);
            res
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![literal!("'"), s1, literal!(" = "), s2, literal!(";'")]);
            res
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            res = stringAppendList(list![literal!("'"), s1, literal!(" = "), s2, literal!(";'")]);
            res
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition, whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e1, right: e2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e1.clone())?;
            s2 = ExpressionBasics::printExpStr(e2.clone())?;
            s3 = ExpressionBasics::printExpStr(condition.clone())?;
            res = stringAppendList(list![literal!("'when "), s3, literal!(" then "), s1, literal!(" = "), s2, literal!("; end when;'")]);
            res
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            res = stringAppendList(list![literal!("'"), s1, literal!("= 0"), literal!(";'")]);
            res
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            let mut res: ArcStr;
            res = stringAppendList(list![literal!("Algorithm\n")]);
            res
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

fn getAdjacencyMatrix(mut m: metamodelica::Array<metamodelica::List<ArcStr>>) -> ArcStr {
    let mut strIMatrix: ArcStr;
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut mstr: ArcStr;
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = intString(mlen);
    m_1 = m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>();
    mstr = getAdjacencyMatrix2(&m_1, 1);
    strIMatrix = stringAppendList(list![
        literal!("% Adjacency Matrix\n"),
        literal!("% ====================================\n"),
        literal!("% number of rows: "),
        mlen_str,
        literal!("\n"),
        literal!("IM={"),
        mstr,
        literal!("};")
    ]);
    strIMatrix
}

fn getAdjacencyMatrix2(
    mut inStringLstLst: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut rowIndex: i32,
) -> ArcStr {
    let mut strIMatrix: ArcStr;
    strIMatrix = (::match_deref::match_deref! { match inStringLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: row, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut r#str: ArcStr;
            let mut str1: ArcStr;
            str1 = getAdjacencyRow(metamodelica::AsArg::as_arg(&row));
            r#str = stringAppendList(list![literal!("{"), str1, literal!("}")]);
            r#str
        },
        Deref @ metamodelica::ListNode::Cons { head: row, tail: rows } => {
            let mut r#str: ArcStr;
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = getAdjacencyRow(metamodelica::AsArg::as_arg(&row));
            str2 = getAdjacencyMatrix2(rows, rowIndex + 1);
            r#str = stringAppendList(list![literal!("{"), str1, literal!("},"), str2]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    strIMatrix
}

fn getAdjacencyRow(mut inStringLst: &metamodelica::List<ArcStr>) -> ArcStr {
    let mut strRow: ArcStr;
    strRow = (::match_deref::match_deref! { match inStringLst {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Nil } => {
            x.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut s: ArcStr;
            let mut s2: ArcStr;
            s2 = getAdjacencyRow(xs);
            s = stringAppendList(list![x.clone(), literal!(","), s2]);
            s
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    strRow
}

pub(crate) fn getVariables(mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = (::match_deref::match_deref! { match inBackendDAE {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut s: ArcStr;
            vars = BackendVariable::varList(metamodelica::AsArg::as_arg(&vars1))?;
            s = dumpVars(&vars)?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VL = {")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("};")); ArcStr::from(__mm_s) };
            s
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(strVars)
}

pub(crate) fn dumpVars(mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = dumpVars2(vars, 1)?;
    Ok(strVars)
}

fn dumpVars2(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inInteger: i32,
) -> Result<ArcStr> {
    let mut strVars: ArcStr;
    strVars = 'mc: {
        let __mc_input = (&**inVarLst, inInteger);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    str1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    r#str = stringAppendList(list![literal!("'"), str1.clone(), literal!("'")]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varName: cr, .. }, tail: xs }, varno) => {
                    let mut r#str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut varno_1: i32;
                    str1 = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    varno_1 = varno.clone() + 1;
                    str2 = dumpVars2(metamodelica::AsArg::as_arg(&xs), varno_1)?;
                    r#str = stringAppendList(list![literal!("'"), str1.clone(), literal!("',"), str2.clone()]);
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(strVars)
}

pub(crate) fn adjacencyMatrix(
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Array<metamodelica::List<ArcStr>>> {
    let mut outAdjacencyMatrix: metamodelica::Array<metamodelica::List<ArcStr>>;
    outAdjacencyMatrix = 'mc: {
        let __mc_input = &**inBackendDAE;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut eqnsl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut lstlst: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut arr: metamodelica::Array<metamodelica::List<ArcStr>>;
                    eqnsl = BackendEquation::equationList(eqns.clone())?;
                    lstlst = adjacencyMatrix2(vars.clone(), &eqnsl)?;
                    arr = metamodelica::arrayFromVec(lstlst.clone().into_iter().cloned().collect());
                    Ok(arr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("DAEQuery.adjacencyMatrix failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAdjacencyMatrix)
}

fn adjacencyMatrix2(
    mut inVariables: BackendDAE::Variables,
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut outStringLstLst: metamodelica::List<metamodelica::List<ArcStr>>;
    outStringLstLst = 'mc: {
        let __mc_input = (inVariables, &**inEquationLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ metamodelica::ListNode::Cons { head: e, tail: eqns }) => {
                    let mut lst: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut row: metamodelica::List<ArcStr>;
                    lst = adjacencyMatrix2(vars.clone(), metamodelica::AsArg::as_arg(&eqns))?;
                    row = adjacencyRow(vars.clone(), metamodelica::AsArg::as_arg(&e))?;
                    Ok(metamodelica::cons(row.clone(), lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    metamodelica::print(literal!("adjacency_matrix2 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLstLst)
}

fn adjacencyRow(
    mut inVariables: BackendDAE::Variables,
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outIntegerLst: metamodelica::List<ArcStr>;
    outIntegerLst = 'mc: {
        let __mc_input = (inVariables, &**inEquation);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    lst1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    lst2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    lst1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    lst2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    lst1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    lst2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    lst1 = adjacencyRowExp(&(Expression::crefExp(cr.clone())?), vars.clone());
                    lst2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    lst1 = adjacencyRowExp(&(Expression::crefExp(cr.clone())?), vars.clone());
                    lst2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }) => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: we, .. }) => {
                    let mut lst1: metamodelica::List<ArcStr>;
                    let mut lst2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    (cr, e2) = BackendEquation::getWhenEquationExpr(metamodelica::AsArg::as_arg(&we))?;
                    e1 = Expression::crefExp(cr.clone())?;
                    lst1 = adjacencyRowExp(&e1, vars.clone());
                    lst2 = adjacencyRowExp(&e2, vars.clone());
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vars, Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }) => {
                    let mut res_1: metamodelica::List<ArcStr>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut lstres: metamodelica::List<metamodelica::List<ArcStr>>;
                    expl = Algorithm::getAllExps(metamodelica::AsArg::as_arg(&alg))?;
                    lstres = List::map1(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> { ::std::result::Result::Ok(adjacencyRowExp(&__a0, __a1)) }, vars.clone())?;
                    res_1 = List::flatten(lstres.clone())?;
                    Ok(res_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    metamodelica::print(literal!("- DAEQuery.adjacencyRow failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIntegerLst)
}

// protected function adjacencyRowStmts "author: PA
//   Helper function to adjacencyRow, investigates statements for
//   variables, returning variable indexes."
//   input list<DAE.Statement> inAlgorithmStatementLst;
//   input BackendDAE.Variables inVariables;
//   output list<String> outStringLst;
// algorithm
//   outStringLst := matchcontinue (inAlgorithmStatementLst,inVariables)
//     local
//       list<String> lst1,lst2,lst3,res,lst3_1;
//       DAE.Type tp;
//       DAE.ComponentRef cr;
//       DAE.Exp e, e1;
//       list<DAE.Statement> rest,stmts;
//       BackendDAE.Variables vars;
//       list<DAE.Exp> expl;
//       DAE.Else else_;
//       list<list<String>> lstlst;
//
//     case ({},_) then {};
//
//     case ((DAE.STMT_ASSIGN(type_ = tp,exp1 = e1,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(e1, vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_TUPLE_ASSIGN(type_ = tp,expExpLst = expl,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lstlst = List.map1(expl, adjacencyRowExp, vars);
//         lst3_1 = List.flatten(lstlst);
//         res = List.flatten({lst1,lst2,lst3_1});
//       then
//         res;
//
//     case ((DAE.STMT_ASSIGN_ARR(type_ = tp,componentRef = cr,exp = e) :: rest),vars)
//       equation
//         lst1 = adjacencyRowStmts(rest, vars);
//         lst2 = adjacencyRowExp(e, vars);
//         lst3 = adjacencyRowExp(Expression.crefExp(cr), vars);
//         res = List.flatten({lst1,lst2,lst3});
//       then
//         res;
//
//     case ((DAE.STMT_IF(exp = e,statementLst = stmts,else_ = else_) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on IF not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_FOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on FOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_PARFOR(type_ = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on PARFOR not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHILE(exp = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHILE not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_WHEN(exp = e) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on WHEN not implemented\n");
//       then
//         {};
//
//     case ((DAE.STMT_ASSERT(cond = _) :: rest),vars)
//       equation
//         print("- DAEQuery.adjacencyRowStmts on ASSERT not implemented\n");
//       then
//         {};
//   end matchcontinue;
// end adjacencyRowStmts;
fn adjacencyRowExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inVariables: BackendDAE::Variables,
) -> metamodelica::List<ArcStr> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = (&**inExp, inVariables);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut p_1: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    p_1 = List::map1r(p.clone(), &fnptr!(intSub, i32, i32), 0)?;
                    pStr = List::map(p_1.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    pStr = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    pStr = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    pStr = listAppend(s1.clone(), s2.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::RELATION { operator: op1, exp2: ee2, .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    let mut s3: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut opStr: ArcStr;
                    opStr = ExpressionDump::relopSymbol(metamodelica::AsArg::as_arg(&op1))?;
                    s = printExpStr(ee2.clone());
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    ss1 = getAdjacencyRow(&s1);
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    ss2 = getAdjacencyRow(&s2);
                    s3 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e3), vars.clone());
                    ss3 = getAdjacencyRow(&s3);
                    ss = stringAppendList(list![literal!("{'if', "), s.clone(), literal!(",'"), opStr.clone(), literal!("' {"), ss1.clone(), literal!("}"), literal!(",{"), ss2.clone(), literal!("},"), ss3.clone(), literal!("}")]);
                    pStr = list![ss.clone()];
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::LBINARY { .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    let mut s3: metamodelica::List<ArcStr>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    printExpStr(e1.clone());
                    sb = stringAppendList(list![literal!("'true',"), literal!("'=='")]);
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    ss1 = getAdjacencyRow(&s1);
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    ss2 = getAdjacencyRow(&s2);
                    s3 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e3), vars.clone());
                    ss3 = getAdjacencyRow(&s3);
                    ss = stringAppendList(list![literal!("{'if', "), sb.clone(), literal!(","), literal!("{"), ss1.clone(), literal!("}"), literal!(",{"), ss2.clone(), literal!("},"), ss3.clone(), literal!("}")]);
                    pStr = list![ss.clone()];
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1 @ Deref @ DAE::Exp::CREF { .. }, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    let mut s3: metamodelica::List<ArcStr>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    sb = stringAppendList(list![literal!("'true',"), literal!("'=='")]);
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    ss1 = getAdjacencyRow(&s1);
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    ss2 = getAdjacencyRow(&s2);
                    s3 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e3), vars.clone());
                    ss3 = getAdjacencyRow(&s3);
                    ss = stringAppendList(list![literal!("{'if', "), sb.clone(), literal!(" {"), ss1.clone(), literal!("}"), literal!(",{"), ss2.clone(), literal!("},"), ss3.clone(), literal!("}")]);
                    pStr = list![ss.clone()];
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut s2: metamodelica::List<ArcStr>;
                    let mut s3: metamodelica::List<ArcStr>;
                    let mut ss: ArcStr;
                    let mut ss1: ArcStr;
                    let mut ss2: ArcStr;
                    let mut ss3: ArcStr;
                    let mut sb: ArcStr;
                    sb = printExpStr(e1.clone());
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    ss1 = getAdjacencyRow(&s1);
                    s2 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e2), vars.clone());
                    ss2 = getAdjacencyRow(&s2);
                    s3 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e3), vars.clone());
                    ss3 = getAdjacencyRow(&s3);
                    ss = stringAppendList(list![literal!("{'if', "), literal!("'"), sb.clone(), literal!("' {"), ss1.clone(), literal!("}"), literal!(",{"), ss2.clone(), literal!("},"), ss3.clone(), literal!("}")]);
                    pStr = list![ss.clone()];
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    (_, p) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    (_, p) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut p: metamodelica::List<i32>;
                    let mut pStr: metamodelica::List<ArcStr>;
                    (_, p) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    pStr = List::map(p.clone(), &fnptr!(intString, i32))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: expl, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut lst: metamodelica::List<metamodelica::List<ArcStr>>;
                    lst = List::map1(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> { ::std::result::Result::Ok(adjacencyRowExp(&__a0, __a1)) }, vars.clone())?;
                    pStr = List::flatten(lst.clone())?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut lst: metamodelica::List<metamodelica::List<ArcStr>>;
                    lst = List::map1(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> { ::std::result::Result::Ok(adjacencyRowExp(&__a0, __a1)) }, vars.clone())?;
                    pStr = List::flatten(lst.clone())?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explTpl, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    pStr = adjacencyRowMatrixExp(metamodelica::AsArg::as_arg(&explTpl), vars.clone())?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { .. }, _) => {
                    metamodelica::print(literal!("- DAEQuery.adjacency_row_exp TUPLE not impl. yet."));
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { exp: e, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    pStr = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    pStr = adjacencyRowExp(metamodelica::AsArg::as_arg(&e), vars.clone());
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { expr: e1, iterators: iters, .. }, vars) => {
                    let mut pStr: metamodelica::List<ArcStr>;
                    let mut s1: metamodelica::List<ArcStr>;
                    let mut lst: metamodelica::List<metamodelica::List<ArcStr>>;
                    s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
                    lst = List::map1(iters.clone(), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> { ::std::result::Result::Ok(adjacencyRowIter(&__a0, __a1)) }, vars.clone())?;
                    pStr = List::flatten(metamodelica::cons(s1.clone(), lst.clone()))?;
                    Ok(pStr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStringLst
}

fn adjacencyRowIter(
    mut iter: &metamodelica::Ref<DAE::ReductionIterator>,
    mut vars: BackendDAE::Variables,
) -> metamodelica::List<ArcStr> {
    let mut strs: metamodelica::List<ArcStr>;
    strs = (::match_deref::match_deref! { match iter {
        Deref @ DAE::ReductionIterator { guardExp: Some(e1), exp: e2, .. } => {
            let mut s1: metamodelica::List<ArcStr>;
            let mut s2: metamodelica::List<ArcStr>;
            s1 = adjacencyRowExp(metamodelica::AsArg::as_arg(&e1), vars.clone());
            s2 = adjacencyRowExp(e2, vars);
            listAppend(s1, s2)
        },
        Deref @ DAE::ReductionIterator { exp: e1, .. } => {
            adjacencyRowExp(e1, vars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    strs
}

fn adjacencyRowMatrixExp(
    mut inTplExpExpBooleanLstLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = (::match_deref::match_deref! { match inTplExpExpBooleanLstLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: expl_1, tail: es } => {
            let mut vars = inVariables;
            let mut res1: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut pStr: metamodelica::List<ArcStr>;
            let mut res1_1: metamodelica::List<ArcStr>;
            let mut res2: metamodelica::List<ArcStr>;
            res1 = List::map1(expl_1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: BackendDAE::Variables| -> metamodelica::Result<_> { ::std::result::Result::Ok(adjacencyRowExp(&__a0, __a1)) }, vars.clone())?;
            res2 = adjacencyRowMatrixExp(es, vars)?;
            res1_1 = List::flatten(res1)?;
            pStr = listAppend(res1_1, res2);
            pStr
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStringLst)
}

fn printExpStr(mut e: metamodelica::Ref<DAE::Exp>) -> ArcStr {
    let mut s: ArcStr;
    s = ExpressionDump::printExp2Str::<()>(e, &(literal!("'")), None, None);
    s
}
