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
use openmodelica_frontend_dump::Dump;

pub fn rewriteBlockCall(mut inPg: Absyn::Program, mut inDefs: &Absyn::Program) -> Result<Absyn::Program> {
    let mut newOut: Absyn::Program;
    newOut = (match inDefs.clone() {
        _ => {
            let mut pg2: Absyn::Program;
            let mut res: ArcStr;
            pg2 = parseProgram(inPg, inDefs)?;
            res = Dump::unparseStr(pg2.clone(), false, Dump::defaultDumpOptions.clone())?;
            metamodelica::print(res);
            pg2
        }
    });
    Ok(newOut)
}

fn parseProgram(mut inPg: Absyn::Program, mut defs: &Absyn::Program) -> Result<Absyn::Program> {
    let mut outPg: Absyn::Program = inPg;
    outPg = (match outPg.clone() {
        Absyn::Program { .. } => {
            outPg.classes = parseClasses(&outPg.classes, defs)?;
            outPg
        }
    });
    Ok(outPg)
}

pub(crate) fn parseClasses(
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    mut defs: &Absyn::Program,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Class>>> {
    let mut out_classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    out_classes = (::match_deref::match_deref! { match classes {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
            let mut n_cls: metamodelica::Ref<Absyn::Class>;
            nr_classes = parseClasses(r_classes, defs)?;
            n_cls = parseClass(cls.clone(), defs)?;
            metamodelica::cons(n_cls, nr_classes)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_classes)
}

pub(crate) fn parseClass(
    mut in_class: metamodelica::Ref<Absyn::Class>,
    mut defs: &Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut out_class: metamodelica::Ref<Absyn::Class>;
    out_class = (::match_deref::match_deref! { match &(in_class) {
        __esc_out_class @ Deref @ Absyn::Class { body, .. } => {
            out_class = (*__esc_out_class).clone();
            assign_field!(out_class.body = parseClassDef(metamodelica::AsArg::as_arg(&body), defs)?);
            out_class.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_class)
}

fn parseClassDef(
    mut in_def: &metamodelica::Ref<Absyn::ClassDef>,
    mut defs: &Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut out_def: metamodelica::Ref<Absyn::ClassDef>;
    out_def = (match &**in_def {
        Absyn::ClassDef::PARTS {
            typeVars,
            classAttrs,
            classParts,
            ann,
            comment,
        } => {
            let mut nclsp: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (nclsp, eqs, elems, _) =
                parseClassParts(classParts, defs, &(metamodelica::nil()), &(metamodelica::nil()), 0)?;
            metamodelica::Ref::new(Absyn::ClassDef::PARTS {
                typeVars: typeVars.clone(),
                classAttrs: classAttrs.clone(),
                classParts: metamodelica::cons(
                    metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elems }),
                    metamodelica::cons(
                        metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: eqs }),
                        nclsp,
                    ),
                ),
                ann: ann.clone(),
                comment: comment.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(out_def)
}

fn parseClassParts(
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut defs: &Absyn::Program,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_classes, eqs, elems, newInstNo) = (::match_deref::match_deref! { match classes {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), oldEqs.clone(), oldElems.clone(), instNo)
        },
        Deref @ metamodelica::ListNode::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut n_cls: metamodelica::Ref<Absyn::ClassPart>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count1: i32;
            (n_cls, eqs2, elems2, count1) = parseClassPart(metamodelica::AsArg::as_arg(&cls), defs, oldEqs, oldElems, instNo)?;
            (nr_classes, eqs1, elems1, count) = parseClassParts(r_classes, defs, &eqs2, &elems2, count1)?;
            (metamodelica::cons(n_cls, nr_classes), eqs1, elems1, count)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_classes, eqs, elems, newInstNo))
}

fn parseClassPart(
    mut in_def: &metamodelica::Ref<Absyn::ClassPart>,
    mut defs: &Absyn::Program,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::Ref<Absyn::ClassPart>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_def: metamodelica::Ref<Absyn::ClassPart>;
    let mut reqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut relems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_def, reqs, relems, newInstNo) = (match &**in_def {
        Absyn::ClassPart::PUBLIC { contents: elems } => (
            metamodelica::Ref::new(Absyn::ClassPart::PUBLIC {
                contents: elems.clone(),
            }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
        Absyn::ClassPart::PROTECTED { contents: elems } => (
            metamodelica::Ref::new(Absyn::ClassPart::PROTECTED {
                contents: elems.clone(),
            }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
        Absyn::ClassPart::CONSTRAINTS { contents: exps } => (
            metamodelica::Ref::new(Absyn::ClassPart::CONSTRAINTS { contents: exps.clone() }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
        Absyn::ClassPart::EQUATIONS { contents: eqs } => {
            let mut neqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (neqs, eqs1, elems1, count) = parseEquations(eqs, defs, oldEqs, oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: neqs }),
                eqs1,
                elems1,
                count,
            )
        }
        Absyn::ClassPart::INITIALEQUATIONS { contents: eqs } => {
            let mut neqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (neqs, eqs1, elems1, count) = parseEquations(eqs, defs, oldEqs, oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS { contents: neqs }),
                eqs1,
                elems1,
                count,
            )
        }
        Absyn::ClassPart::ALGORITHMS { contents: algs } => (
            metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS { contents: algs.clone() }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
        Absyn::ClassPart::INITIALALGORITHMS { contents: algs } => (
            metamodelica::Ref::new(Absyn::ClassPart::INITIALALGORITHMS { contents: algs.clone() }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
        Absyn::ClassPart::EXTERNAL {
            externalDecl,
            annotation_,
        } => (
            metamodelica::Ref::new(Absyn::ClassPart::EXTERNAL {
                externalDecl: externalDecl.clone(),
                annotation_: annotation_.clone(),
            }),
            metamodelica::nil(),
            metamodelica::nil(),
            instNo,
        ),
    });
    Ok((out_def, reqs, relems, newInstNo))
}

fn parseEquations(
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut defs: &Absyn::Program,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_classes: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_classes, eqs, elems, newInstNo) = (::match_deref::match_deref! { match classes {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), oldEqs.clone(), oldElems.clone(), instNo)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, comment: cmt, info }, tail: r_classes } => {
            let mut neq: metamodelica::Ref<Absyn::Equation>;
            let mut nr_classes: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count1: i32;
            (neq, eqs2, elems2, count1) = parseEquation(metamodelica::AsArg::as_arg(&eq), defs, oldEqs.clone(), oldElems.clone(), instNo)?;
            (nr_classes, eqs1, elems1, count) = parseEquations(r_classes, defs, &eqs2, &elems2, count1)?;
            (metamodelica::cons(metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: neq, comment: cmt.clone(), info: info.clone() }), nr_classes), eqs1, elems1, count)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { comment }, tail: r_classes } => {
            let mut nr_classes: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (nr_classes, eqs1, elems1, count) = parseEquations(r_classes, defs, oldEqs, oldElems, instNo)?;
            (metamodelica::cons(metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEMCOMMENT { comment: comment.clone() }), nr_classes), eqs1, elems1, count)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((out_classes, eqs, elems, newInstNo))
}

fn parseEquation(
    mut in_eq: &metamodelica::Ref<Absyn::Equation>,
    mut defs: &Absyn::Program,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::Ref<Absyn::Equation>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_eq: metamodelica::Ref<Absyn::Equation>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_eq, eqs, elems, newInstNo) = (match &**in_eq {
        Absyn::Equation::EQ_IF {
            ifExp: exp1,
            equationTrueItems: leq1,
            elseIfBranches: tup1,
            equationElseItems: leq2,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nleq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut nleq2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs3: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems3: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count1: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1, defs, &oldEqs, &oldElems, instNo)?;
            (nleq1, eqs2, elems2, count1) = parseEquations(leq1, defs, &eqs1, &elems1, count)?;
            (nleq2, eqs3, elems3, count2) = parseEquations(leq2, defs, &eqs2, &elems2, count1)?;
            (
                metamodelica::Ref::new(Absyn::Equation::EQ_IF {
                    ifExp: nexp1,
                    equationTrueItems: nleq1,
                    elseIfBranches: tup1.clone(),
                    equationElseItems: nleq2,
                }),
                eqs3,
                elems3,
                count2,
            )
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: exp1,
            rightSide: exp2,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count1: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1, defs, &oldEqs, &oldElems, instNo)?;
            (nexp2, eqs2, elems2, count1) = parseExpression(exp2, defs, &eqs1, &elems1, count)?;
            (
                metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS {
                    leftSide: nexp1,
                    rightSide: nexp2,
                }),
                eqs2,
                elems2,
                count1,
            )
        }
        Absyn::Equation::EQ_PDE {
            leftSide: exp1,
            rightSide: exp2,
            domain,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count1: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1, defs, &oldEqs, &oldElems, instNo)?;
            (nexp2, eqs2, elems2, count1) = parseExpression(exp2, defs, &eqs1, &elems1, count)?;
            (
                metamodelica::Ref::new(Absyn::Equation::EQ_PDE {
                    leftSide: nexp1,
                    rightSide: nexp2,
                    domain: domain.clone(),
                }),
                eqs2,
                elems2,
                count1,
            )
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: cr1,
            connector2: cr2,
        } => (
            metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT {
                connector1: cr1.clone(),
                connector2: cr2.clone(),
            }),
            oldEqs,
            oldElems,
            instNo,
        ),
        Absyn::Equation::EQ_FOR {
            iterators: fi,
            forEquations: leq1,
        } => {
            let mut nleq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (nleq1, eqs2, elems2, count) = parseEquations(leq1, defs, &oldEqs, &oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::Equation::EQ_FOR {
                    iterators: fi.clone(),
                    forEquations: nleq1,
                }),
                eqs2,
                elems2,
                count,
            )
        }
        Absyn::Equation::EQ_WHEN_E {
            whenExp: exp1,
            whenEquations: leq1,
            elseWhenEquations: tup1,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nleq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            (nexp1, _, _, _) = parseExpression(exp1, defs, &oldEqs, &oldElems, instNo)?;
            (nleq1, _, _, _) = parseEquations(leq1, defs, &oldEqs, &oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::Equation::EQ_WHEN_E {
                    whenExp: nexp1,
                    whenEquations: nleq1,
                    elseWhenEquations: tup1.clone(),
                }),
                oldEqs,
                oldElems,
                instNo,
            )
        }
        Absyn::Equation::EQ_NORETCALL {
            functionName: cr1,
            functionArgs: farg,
        } => (
            metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL {
                functionName: cr1.clone(),
                functionArgs: farg.clone(),
            }),
            oldEqs,
            oldElems,
            instNo,
        ),
        Absyn::Equation::EQ_FAILURE { equ: eqi } => (
            metamodelica::Ref::new(Absyn::Equation::EQ_FAILURE { equ: eqi.clone() }),
            oldEqs,
            oldElems,
            instNo,
        ),
    });
    Ok((out_eq, eqs, elems, newInstNo))
}

fn parseExpression(
    mut in_eq: &metamodelica::Ref<Absyn::Exp>,
    mut defs: &Absyn::Program,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_eq: metamodelica::Ref<Absyn::Exp>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_eq, eqs, elems, newInstNo) = (match &**in_eq {
        Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1, defs, oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count2) = parseExpression(exp2, defs, &eqs1, &elems1, count)?;
            (
                metamodelica::Ref::new(Absyn::Exp::BINARY {
                    exp1: nexp1,
                    op: op.clone(),
                    exp2: nexp2,
                }),
                eqs2,
                elems2,
                count2,
            )
        }
        Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count2: i32;
            (nexp1, eqs1, elems1, count) = parseExpression(exp1, defs, oldEqs, oldElems, instNo)?;
            (nexp2, eqs2, elems2, count2) = parseExpression(exp2, defs, &eqs1, &elems1, count)?;
            (
                metamodelica::Ref::new(Absyn::Exp::LBINARY {
                    exp1: nexp1,
                    op: op.clone(),
                    exp2: nexp2,
                }),
                eqs2,
                elems2,
                count2,
            )
        }
        Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (nexp2, eqs2, elems2, count) = parseExpression(exp2, defs, oldEqs, oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::Exp::UNARY {
                    op: op.clone(),
                    exp: nexp2,
                }),
                eqs2,
                elems2,
                count,
            )
        }
        Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (nexp2, eqs2, elems2, count) = parseExpression(exp2, defs, oldEqs, oldElems, instNo)?;
            (
                metamodelica::Ref::new(Absyn::Exp::LUNARY {
                    op: op.clone(),
                    exp: nexp2,
                }),
                eqs2,
                elems2,
                count,
            )
        }
        Absyn::Exp::IFEXP {
            ifExp: ife,
            trueBranch: exp1,
            elseBranch: exp2,
            elseIfBranch: elif,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut nife: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs3: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs4: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems3: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems4: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            let mut count2: i32;
            let mut count3: i32;
            let mut count4: i32;
            let mut nelif: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
            (nife, eqs1, elems1, count) = parseExpression(ife, defs, oldEqs, oldElems, instNo)?;
            (nexp1, eqs2, elems2, count2) = parseExpression(exp1, defs, &eqs1, &elems1, count)?;
            (nexp2, eqs3, elems3, count3) = parseExpression(exp2, defs, &eqs2, &elems2, count2)?;
            (nelif, eqs4, elems4, count4) = parseExpressionTuple(elif, defs, &eqs3, &elems3, count3)?;
            (
                metamodelica::Ref::new(Absyn::Exp::IFEXP {
                    ifExp: nife,
                    trueBranch: nexp1,
                    elseBranch: nexp2,
                    elseIfBranch: nelif,
                }),
                eqs4,
                elems4,
                count4,
            )
        }
        Absyn::Exp::CALL { .. } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count: i32;
            (nexp1, eqs1, elems1, count) = parseCall(in_eq.clone(), defs, instNo, oldEqs.clone(), oldElems.clone())?;
            (nexp1, eqs1, elems1, count)
        }
        _ => (in_eq.clone(), oldEqs.clone(), oldElems.clone(), instNo),
    });
    Ok((out_eq, eqs, elems, newInstNo))
}

fn parseExpressionTuple(
    mut tuple_list: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>,
    mut defs: &Absyn::Program,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut instNo: i32,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut out_tuple_list: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (out_tuple_list, eqs, elems, newInstNo) = (::match_deref::match_deref! { match tuple_list {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), oldEqs.clone(), oldElems.clone(), instNo)
        },
        Deref @ metamodelica::ListNode::Cons { head: (exp1, exp2), tail: r_tuple_list } => {
            let mut ntuples: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqs3: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut elems3: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            let mut count1: i32;
            let mut count3: i32;
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            (nexp1, eqs1, elems1, count1) = parseExpression(metamodelica::AsArg::as_arg(&exp1), defs, oldEqs, oldElems, instNo)?;
            (nexp2, _, _, _) = parseExpression(metamodelica::AsArg::as_arg(&exp2), defs, &eqs1, &elems1, count1)?;
            (ntuples, eqs3, elems3, count3) = parseExpressionTuple(r_tuple_list, defs, &eqs1, &elems1, count1)?;
            (metamodelica::cons((nexp1, nexp2), ntuples), eqs3, elems3, count3)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_tuple_list, eqs, elems, newInstNo))
}

/* *
When a function call is found, we check if it is in the block definitions, and if it is we replace it
*/
fn parseCall(
    mut in_eq: metamodelica::Ref<Absyn::Exp>,
    mut defs: &Absyn::Program,
    mut instNo: i32,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldElems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    i32,
)> {
    let mut res_expr: metamodelica::Ref<Absyn::Exp>;
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newElems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut newInstNo: i32;
    (res_expr, newEqs, newElems, newInstNo) = 'mc: {
        let __mc_input = &*in_eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: _ }, functionArgs: fargs, .. } => {
                    let mut elName: ArcStr;
                    let mut elem: metamodelica::Ref<Absyn::ElementItem>;
                    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut count: i32;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(getDefinition(metamodelica::AsArg::as_arg(&id), instNo, defs, metamodelica::AsArg::as_arg(&fargs), &oldEqs, &(metamodelica::nil()))) {
                        (__pa0, __pa1, true, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqs = metamodelica::Own::own(__pa0);
                    mods = metamodelica::Own::own(__pa1);
                    count = metamodelica::Own::own(__pa2);
                    elName = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) };
                    elem = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() }, typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }), arrayDim: None }), components: list![metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: elName.clone(), arrayDim: metamodelica::nil(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: mods.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })) }, condition: None, comment: None })] }), info: Absyn::dummyInfo.clone(), constrainClass: None }) });
                    Ok((metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: elName.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("out"), subscripts: metamodelica::nil() }) }) }), eqs.clone(), metamodelica::cons(elem.clone(), oldElems.clone()), count))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { .. } => {
                    Ok((in_eq.clone(), oldEqs.clone(), metamodelica::nil(), instNo))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((res_expr, newEqs, newElems, newInstNo))
}

fn getDefinition(
    mut id: &ArcStr,
    mut instNo: i32,
    mut defs: &Absyn::Program,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    bool,
    i32,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut found: bool;
    let mut newInstNo: i32;
    (newEqs, newModif, found, newInstNo) = (match defs.clone() {
        Absyn::Program { .. } => parseClassesDefs(id, instNo, &defs.classes, fargs, oldEqs, oldModif),
    });
    (newEqs, newModif, found, newInstNo)
}

/* *
Get the block definitions, go through all packages
*/
fn parseClassesDefs(
    mut id: &ArcStr,
    mut instNo: i32,
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    bool,
    i32,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut found: bool;
    let mut newInstNo: i32;
    (newEqs, newModif, found, newInstNo) = 'mc: {
        let __mc_input = &**classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), false, instNo))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { name: _, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_PACKAGE { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. }, tail: _ } => {
                    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughClasses(id, instNo, fargs, metamodelica::AsArg::as_arg(&classParts), oldEqs, oldModif)) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqs = metamodelica::Own::own(__pa0);
                    mods = metamodelica::Own::own(__pa1);
                    Ok((eqs.clone(), mods.clone(), true, instNo + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { name: id2, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_BLOCK { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. }, tail: _ } => {
                    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let true = (metamodelica::stringEq(&id2, &id)) else { return Err("pattern mismatch") };
                    (eqs, mods) = parseArgs(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) }), classParts.clone(), fargs, oldEqs.clone(), oldModif.clone())?;
                    Ok((eqs.clone(), mods.clone(), true, instNo + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: r_classes } => {
                    Ok(parseClassesDefs(id, instNo, metamodelica::AsArg::as_arg(&r_classes), fargs, oldEqs, oldModif))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found, newInstNo)
}

fn lookThroughClasses(
    mut id: &ArcStr,
    mut instNo: i32,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    bool,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = &**classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: _ } => {
                    let mut eq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughElems(id, instNo, fargs, metamodelica::AsArg::as_arg(&elems1), oldEqs, oldModif)) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq1 = metamodelica::Own::own(__pa0);
                    modif = metamodelica::Own::own(__pa1);
                    Ok((eq1.clone(), modif.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: r_classes } => {
                    Ok(lookThroughClasses(id, instNo, fargs, metamodelica::AsArg::as_arg(&r_classes), oldEqs, oldModif))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn lookThroughElems(
    mut id: &ArcStr,
    mut instNo: i32,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut elems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    bool,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = &**elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: Deref @ Absyn::Class { name: id2, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_BLOCK { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. } }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let true = (metamodelica::stringEq(&id2, &id)) else { return Err("pattern mismatch") };
                    (eqs, mods) = parseArgs(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_autogen_")); __mm_s.push_str(&*id); __mm_s.push_str(&*intString(instNo)); ArcStr::from(__mm_s) }), classParts.clone(), fargs, oldEqs.clone(), oldModif.clone())?;
                    Ok((eqs.clone(), mods.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: Deref @ Absyn::Class { name: _, partialPrefix: _, finalPrefix: _, encapsulatedPrefix: _, restriction: Absyn::Restriction::R_PACKAGE { .. }, body: Deref @ Absyn::ClassDef::PARTS { typeVars: _, classAttrs: _, classParts, ann: _, comment: _ }, commentsBeforeClass: _, .. } }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookThroughClasses(id, instNo, fargs, metamodelica::AsArg::as_arg(&classParts), oldEqs, oldModif)) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqs = metamodelica::Own::own(__pa0);
                    mods = metamodelica::Own::own(__pa1);
                    Ok((eqs.clone(), mods.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: r_elems } => {
                    Ok(lookThroughElems(id, instNo, fargs, metamodelica::AsArg::as_arg(&r_elems), oldEqs, oldModif))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn parseArgs(
    mut elemId: &ArcStr,
    mut classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
)> {
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    (eqs, mods) = (match &**fargs {
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            let mut eqs1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut mods1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            (eqs1, mods1) = matchArgsClass(elemId, args.clone(), classes.clone(), oldEqs, oldModif)?;
            matchNamedArgsClass(elemId, argNames, &classes, eqs1, mods1)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((eqs, mods))
}

/* *
uniontype NamedArg "The NamedArg uniontype consist of an Identifier for the argument and an expression
  giving the value of the argument"
  record NAMEDARG
    Ident argName "argName" ;
    Exp argValue "argValue" ;
  end NAMEDARG;

end NamedArg;
*/
fn matchArgsClass<'__b>(
    mut elemId: &'__b ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((classes, args.clone())) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((oldEqs, oldModif))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((oldEqs, oldModif))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: r_classes }, _) => {
                let mut eq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut r_args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                (eq1, modif, r_args) = matchArgsElems(elemId, args, elems1.clone(), oldEqs, oldModif)?;
                { (elemId, args, classes, oldEqs, oldModif) = (elemId, r_args, r_classes.clone(), eq1, modif); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: r_classes }, _) => {
                { (elemId, args, classes, oldEqs, oldModif) = (elemId, args, r_classes.clone(), oldEqs, oldModif); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchArgsElems<'__b>(
    mut elemId: &'__b ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((args.clone(), elems)) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((oldEqs, oldModif, args))
            },
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((oldEqs, oldModif, args))
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: r_args }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::PARAM { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: r_elems }) => {
                let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                let mut r_args = (*r_args).clone();
                (modif, r_args) = matchParamArgs(&args, metamodelica::AsArg::as_arg(&comps), oldModif)?;
                { (elemId, args, elems, oldEqs, oldModif) = (elemId, r_args.clone(), r_elems.clone(), oldEqs, modif); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: r_args }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::VAR { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: r_elems }) => {
                let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut r_args = (*r_args).clone();
                (eqs, r_args) = matchVarArgs(elemId, &args, metamodelica::AsArg::as_arg(&comps), oldEqs)?;
                { (elemId, args, elems, oldEqs, oldModif) = (elemId, r_args.clone(), r_elems.clone(), eqs, oldModif); continue '__tco; }
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: r_elems }) => {
                { (elemId, args, elems, oldEqs, oldModif) = (elemId, args, r_elems.clone(), oldEqs, oldModif); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchParamArgs<'__b>(
    mut args: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut oldModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (comps, args) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((oldModif, args.clone()))
            },
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((oldModif, args.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: r_comps }, Deref @ metamodelica::ListNode::Cons { head: arg, tail: r_args }) => {
                let mut modif: metamodelica::Ref<Absyn::ElementArg>;
                modif = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: cName.clone() }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: arg.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                { (args, comps, oldModif) = (r_args, r_comps, metamodelica::cons(modif, oldModif)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchVarArgs<'__b>(
    mut elemId: &'__b ArcStr,
    mut args: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (comps, args) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((oldEqs, args.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: r_comps }, Deref @ metamodelica::ListNode::Cons { head: arg, tail: r_args }) => {
                let mut eq: metamodelica::Ref<Absyn::EquationItem>;
                eq = metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS { leftSide: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: elemId.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: cName.clone(), subscripts: metamodelica::nil() }) }) }), rightSide: arg.clone() }), comment: None, info: Absyn::dummyInfo.clone() });
                { (elemId, args, comps, oldEqs) = (elemId, r_args, r_comps, metamodelica::cons(eq, oldEqs)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchNamedArgsClass<'__b>(
    mut elemId: &'__b ArcStr,
    mut nargs: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut classes: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (classes, nargs) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((oldEqs, oldModif))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((oldEqs, oldModif))
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName, argValue }, tail: r_nargs }) => {
                let mut eq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                (eq1, modif) = matchNamedArgClass(elemId, metamodelica::AsArg::as_arg(&argName), metamodelica::AsArg::as_arg(&argValue), classes, &oldEqs, &oldModif);
                { (elemId, nargs, classes, oldEqs, oldModif) = (elemId, r_nargs, classes, eq1, modif); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn matchNamedArgClass(
    mut elemId: &ArcStr,
    mut argName: &ArcStr,
    mut argValue: &metamodelica::Ref<Absyn::Exp>,
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    (newEqs, newModif) = 'mc: {
        let __mc_input = &**classes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elems1 }, tail: _ } => {
                    let mut eq1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matchNamedArgElems(elemId, argName, argValue, metamodelica::AsArg::as_arg(&elems1), oldEqs, oldModif)) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq1 = metamodelica::Own::own(__pa0);
                    modif = metamodelica::Own::own(__pa1);
                    Ok((eq1.clone(), modif.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: r_classes } => {
                    Ok(matchNamedArgClass(elemId, argName, argValue, metamodelica::AsArg::as_arg(&r_classes), oldEqs, oldModif))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif)
}

fn matchNamedArgElems(
    mut elemId: &ArcStr,
    mut argName: &ArcStr,
    mut argValue: &metamodelica::Ref<Absyn::Exp>,
    mut elems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut oldEqs: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut oldModif: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    bool,
) {
    let mut newEqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut newModif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut found: bool;
    (newEqs, newModif, found) = 'mc: {
        let __mc_input = &**elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((oldEqs.clone(), oldModif.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::PARAM { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut modif: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let __pa0 = ::match_deref::match_deref! { match &(matchParamNamedArg(argName, argValue, metamodelica::AsArg::as_arg(&comps), oldModif)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    modif = metamodelica::Own::own(__pa0);
                    Ok((oldEqs.clone(), modif.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: _, streamPrefix: _, parallelism: _, variability: Absyn::Variability::VAR { .. }, direction: _, isField: _, .. }, typeSpec: _, components: comps }, info: _, constrainClass: _ } }, tail: _ } => {
                    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                    let __pa0 = ::match_deref::match_deref! { match &(matchVarNamedArg(elemId, argName, argValue, metamodelica::AsArg::as_arg(&comps), oldEqs)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqs = metamodelica::Own::own(__pa0);
                    Ok((eqs.clone(), oldModif.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: r_elems } => {
                    Ok(matchNamedArgElems(elemId, argName, argValue, metamodelica::AsArg::as_arg(&r_elems), oldEqs, oldModif))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (newEqs, newModif, found)
}

fn matchParamNamedArg<'__b>(
    mut argName: &'__b ArcStr,
    mut argValue: &'__b metamodelica::Ref<Absyn::Exp>,
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut oldModif: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> (metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match comps {
            Deref @ metamodelica::ListNode::Nil => {
                return (oldModif.clone(), false)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: _ } if (metamodelica::stringEq(&cName, &argName)) => {
                let mut modif: metamodelica::Ref<Absyn::ElementArg>;
                modif = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: cName.clone() }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: argValue.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                return (metamodelica::cons(modif, oldModif.clone()), true)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: r_comps } => {
                { (argName, argValue, comps, oldModif) = (argName, argValue, r_comps, oldModif); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn matchVarNamedArg<'__b>(
    mut elemId: &'__b ArcStr,
    mut argName: &'__b ArcStr,
    mut argValue: &'__b metamodelica::Ref<Absyn::Exp>,
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut oldEqs: &'__b metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> (metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match comps {
            Deref @ metamodelica::ListNode::Nil => {
                return (oldEqs.clone(), false)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: cName, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: _ } if (metamodelica::stringEq(&cName, &argName)) => {
                let mut eq: metamodelica::Ref<Absyn::EquationItem>;
                eq = metamodelica::Ref::new(Absyn::EquationItem::EQUATIONITEM { equation_: metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS { leftSide: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: elemId.clone(), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: cName.clone(), subscripts: metamodelica::nil() }) }) }), rightSide: argValue.clone() }), comment: None, info: Absyn::dummyInfo.clone() });
                return (metamodelica::cons(eq, oldEqs.clone()), true)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: r_comps } => {
                { (elemId, argName, argValue, comps, oldEqs) = (elemId, argName, argValue, r_comps, oldEqs); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}
